//! flighttv: the city flight demo on the TV, flown with the remote.
//!
//! Two threads: the simulation (Lean JSBSim plant plus the gate filter's
//! exact rollouts, slow on the TV) ticks on its own, paced to real time when
//! it can keep up and in slow motion otherwise; the main thread reads the
//! remote and draws the latest snapshot, so the picture stays responsive.
//!
//! Options (program arguments or kernel command line):
//!   flight.escapes=N   escapes the gate filter uses (default 6, all of them, as the project does)
//!   flight.scenario=N  first scenario, 1-5
//!
//! On a non-Linux host, `cargo run -p flighttv -- out.ppm [ticks] [nohud]` flies
//! the point-mass model for some ticks and writes one frame (a layout preview;
//! demo/flight-tile.png is one without the HUD).

#[cfg(target_os = "linux")]
fn main() {
    tv::main()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    use flighttv::render::{Raster, OUT_H, OUT_W};
    use std::io::Write;
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).cloned().unwrap_or_else(|| "flighttv.ppm".into());
    let ticks: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);
    let t0 = std::time::Instant::now();
    let world: &'static cityflight::world::World = Box::leak(Box::new(cityflight::world::World::new(1337)));
    println!("world built in {:.1} s, {} boxes", t0.elapsed().as_secs_f64(), world.boxes.len());
    let mut flight = flighttv::Flight::new(world, 4);
    for _ in 0..ticks {
        flight.steer(0.0, 0.3, flighttv::DT_F);
        flight.tick();
    }
    let mut r = Raster::new();
    let t1 = std::time::Instant::now();
    let snap = flight.snap(0.0);
    r.frame(world, &snap.s, 0, &snap.path, snap.accepted);
    let mut frame = vec![0u32; OUT_W * OUT_H];
    r.upscale(&mut frame);
    if args.get(3).map_or(true, |a| a != "nohud") {
        flighttv::hud(&mut frame, &snap, &flighttv::Overlay { fps: 0.0, speed: 0.0, show_help: true, status: "preview" });
    }
    println!("frame: {} triangles in {:.1} ms", r.tris, t1.elapsed().as_secs_f64() * 1e3);
    let mut f = std::io::BufWriter::new(std::fs::File::create(&out).expect("create"));
    write!(f, "P6\n{OUT_W} {OUT_H}\n255\n").unwrap();
    for p in &frame {
        f.write_all(&[(p >> 16) as u8, (p >> 8) as u8, *p as u8]).unwrap();
    }
    println!("{out}");
}

#[cfg(target_os = "linux")]
mod tv {
    use cityflight::sim::SCENARIOS;
    use cityflight::world::World;
    use flighttv::render::{Raster, OUT_H, OUT_W};
    use flighttv::{hud, pace, state, Command, Flight, Overlay, Snap, DT_F};
    use platform::remote::{self, Action};
    use platform::{fb, input, seccomp, sys};
    use std::sync::atomic::Ordering;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    /// How long the help panel stays up after the flight starts.
    const HELP_FOR: Duration = Duration::from_secs(20);

    fn log(msg: impl AsRef<str>) {
        let t = sys::monotonic();
        println!("[{:>4}.{:03}] flighttv: {}", t.as_secs(), t.subsec_millis(), msg.as_ref());
    }

    /// What the remote asks of the simulation.
    #[derive(Default)]
    struct Controls {
        pitch: f64,
        roll: f64,
        commands: Vec<Command>,
    }

    /// What the simulation thread reports.
    #[derive(Default)]
    struct Report {
        snap: Option<Snap>,
        /// Progress text until the first snapshot.
        stage: String,
        /// Simulated seconds per real second.
        speed: f64,
    }

    /// Keys the remote holds down. An IR remote repeats while held and may
    /// not report a release, so a key counts as held for a short time after
    /// its last press or repeat.
    #[derive(Default)]
    struct Held {
        up: Duration, down: Duration, left: Duration, right: Duration,
    }

    impl Held {
        fn is(&self, since: Duration, now: Duration) -> bool {
            !since.is_zero() && now - since < Duration::from_millis(220)
        }
    }

    fn simulate(world: &'static World, controls: Arc<Mutex<Controls>>, report: Arc<Mutex<Report>>, escapes: u8, scenario: usize) {
        let t0 = sys::monotonic();
        let mut flight = Flight::new(world, scenario);
        log(format!("JSBSim plant and gate filter ready in {:.2} s ({} escapes); scenario {}", (sys::monotonic() - t0).as_secs_f64(), escapes, SCENARIOS[scenario]));
        let mut tick_ms = 0.0;
        let mut sim_secs = 0.0;
        let mut window = sys::monotonic();
        let mut speed = 0.0;
        loop {
            let started = sys::monotonic();
            let (pitch, roll, commands) = {
                let mut c = controls.lock().unwrap();
                (c.pitch, c.roll, std::mem::take(&mut c.commands))
            };
            for c in commands {
                flight.apply(c);
                match c {
                    Command::ToggleFilter => log(format!("filter {}", if flight.sim.cfg.filter_on { "on" } else { "off" })),
                    Command::Reset(n) => log(format!("scenario {}", SCENARIOS[n])),
                    Command::ToggleKamikaze => log(format!("kamikaze {}", if flight.kamikaze.is_some() { "on" } else { "off" })),
                    Command::Throttle(_) => (),
                }
            }
            let crashed = flight.sim.crashed;
            flight.steer(pitch, roll, DT_F);
            flight.tick();
            if flight.sim.crashed && !crashed {
                log(format!("crashed at t={:.1} s", flight.sim.time()));
            }
            let cost = sys::monotonic() - started;
            tick_ms = 0.7 * tick_ms + 0.3 * cost.as_secs_f64() * 1e3;
            if !flight.sim.crashed {
                sim_secs += DT_F;
            }
            let span = sys::monotonic() - window;
            if span >= Duration::from_secs(3) {
                speed = sim_secs / span.as_secs_f64();
                log(format!("sim {:.0}% of real time, tick {:.0} ms", speed * 100.0, tick_ms));
                sim_secs = 0.0;
                window = sys::monotonic();
            }
            {
                let mut r = report.lock().unwrap();
                r.snap = Some(flight.snap(tick_ms));
                r.speed = speed;
            }
            std::thread::sleep(pace(started, sys::monotonic()).max(Duration::from_millis(1)));
        }
    }

    pub fn main() {
        std::panic::set_hook(Box::new(|info| {
            eprintln!("flighttv: PANIC: {info}");
            sys::power_off(101);
        }));
        // The Lean runtime's start-up: signal handlers for stack overflow, stack
        // limits, thread bookkeeping. All of these are within PID 1's Lean policy.
        let mut syscalls = seccomp::base_syscalls();
        syscalls.extend([libc::SYS_rt_sigaction, libc::SYS_prlimit64, libc::SYS_uname, libc::SYS_set_tid_address,
                         libc::SYS_set_robust_list, libc::SYS_membarrier, libc::SYS_getuid, libc::SYS_pread64].map(|n| n as i64));
        match seccomp::install(&syscalls, &seccomp::platform_ioctls()) {
            Ok(()) => log("seccomp allowlist installed (base syscalls + Lean runtime start-up + platform ioctls, no network)"),
            Err(e) => log(format!("seccomp: {e}")),
        }
        // The Lean JSBSim plant and the exact-rollout gate filter, as `--env gate` on the desktop.
        cityflight::jsb::JSB_PLANT.store(true, Ordering::Relaxed);
        cityflight::envfilter::ENV_FILTER.store(cityflight::envfilter::parse("gate"), Ordering::Relaxed);
        // No wall-clock cap: the game runs in slow motion instead of cutting checks short.
        cityflight::envfilter::TIME_CAP_MS.store(0, Ordering::Relaxed);
        let escapes = sys::cmdline_arg("flight.escapes").and_then(|s| s.parse::<u8>().ok()).unwrap_or(6).clamp(1, 6);
        cityflight::envfilter::N_USED.store(escapes, Ordering::Relaxed);
        let scenario = sys::cmdline_arg("flight.scenario").and_then(|s| s.parse::<usize>().ok()).map_or(4, |n| n.clamp(1, 5) - 1);

        // A GPU renderer (flightgpu, another process drawing with the Mali) may
        // have announced itself. Then this process publishes the state to draw
        // and presents the frames the renderer sends back; the display stays here.
        let mut gpu: Option<(&'static mut state::Shared, &'static mut state::Frame)> = state::announced().and_then(|path| {
            match (state::open(&path, false), state::open_frame(&path, false)) {
                (Ok(shared), Ok(frame)) => {
                    log(format!("GPU renderer announced; publishing state to {path}, presenting its frames"));
                    Some((shared, frame))
                }
                (Err(e), _) | (_, Err(e)) => {
                    log(format!("GPU renderer announced {path} but it cannot be mapped ({e}); drawing on the CPU"));
                    None
                }
            }
        });
        let mut screen = match fb::Framebuffer::open("/dev/fb0") {
            Ok(s) => s,
            Err(e) => {
                log(format!("display: {e}"));
                sys::power_off(1);
            }
        };
        let var = screen.var;
        let native = var.red.offset == 16 && var.green.offset == 8 && var.blue.offset == 0;
        let mut devs = input::open_all();
        let mut frame = vec![0u32; OUT_W * OUT_H];
        let mut raster = Raster::new();
        let present = |screen: &mut fb::Framebuffer, frame: &[u32]| {
            let bb = screen.back_buffer();
            let (w, h) = (bb.width.min(OUT_W), bb.height.min(OUT_H));
            let (ox, oy) = ((bb.width - w) / 2, (bb.height - h) / 2);
            for y in 0..h {
                let src = &frame[y * OUT_W..y * OUT_W + w];
                let dst = &mut bb.px[(y + oy) * bb.stride + ox..(y + oy) * bb.stride + ox + w];
                if native {
                    dst.copy_from_slice(src);
                } else {
                    for (d, &p) in dst.iter_mut().zip(src) {
                        *d = var.pixel((p >> 16) as u8, (p >> 8) as u8, p as u8);
                    }
                }
            }
            let _ = screen.present();
        };

        let splash = |screen: &mut fb::Framebuffer, frame: &mut Vec<u32>, stage: &str| {
            frame.fill(0x101420);
            flighttv::render::text(frame, 120, 330, 3, 0xf4f4f8, stage);
            flighttv::render::text(frame, 120, 380, 2, 0xb0b4c0, "City flight: Cessna 172, Lean JSBSim plant, exact-rollout safety filter, all on this TV");
            present(screen, frame);
        };
        splash(&mut screen, &mut frame, "Building the city...");
        let t0 = sys::monotonic();
        let world: &'static World = Box::leak(Box::new(World::new(1337)));
        log(format!("world built in {:.1} s: {} buildings", (sys::monotonic() - t0).as_secs_f64(), world.boxes.len()));
        let controls = Arc::new(Mutex::new(Controls::default()));
        let report = Arc::new(Mutex::new(Report {
            stage: "Starting the Lean JSBSim plant and the safety filter (about a minute)...".into(), ..Default::default()
        }));
        {
            let (controls, report) = (controls.clone(), report.clone());
            std::thread::Builder::new().stack_size(16 << 20).spawn(move || simulate(world, controls, report, escapes, scenario)).expect("simulation thread");
        }

        let mut held = Held::default();
        let mut last_gpu_frame = 0u32;
        let mut cam_mode = 0u8;
        // The help panel shows itself for a while, then gets out of the way;
        // a keyboard's Info brings it back. (The remote has no spare button.)
        let mut show_help = true;
        let mut help_until = Some(sys::monotonic() + HELP_FOR);
        // What the last frame showed: the scenario and whether it has crashed,
        // so OK can mean "fly again" after a crash.
        let (mut scenario_now, mut crashed) = (scenario, false);
        let mut frames = 0u32;
        let mut fps = 0.0;
        let mut window = sys::monotonic();
        loop {
            let now = sys::monotonic();
            if help_until.is_some_and(|t| now >= t) {
                help_until = None;
                show_help = false;
            }
            let mut commands = vec![];
            for d in &mut devs {
                for k in d.poll_keys().unwrap_or_default() {
                    let Some(a) = remote::action(k.code) else { continue };
                    let pressed = k.value != 0;
                    match a {
                        Action::Up => held.up = if pressed { now } else { Duration::ZERO },
                        Action::Down => held.down = if pressed { now } else { Duration::ZERO },
                        Action::Left => held.left = if pressed { now } else { Duration::ZERO },
                        Action::Right => held.right = if pressed { now } else { Duration::ZERO },
                        _ if !pressed || k.value == 2 => {}
                        // The TCL remote: OK, Ch rocker, NETFLIX, prime video, YouTube.
                        Action::Ok if crashed => commands.push(Command::Reset(scenario_now)),
                        Action::Ok => commands.push(Command::ToggleFilter),
                        Action::ChannelUp => commands.push(Command::Throttle(0.1)),
                        Action::ChannelDown => commands.push(Command::Throttle(-0.1)),
                        Action::Netflix | Action::PlayPause => commands.push(Command::ToggleKamikaze),
                        Action::PrimeVideo | Action::Menu => cam_mode = (cam_mode + 1) % 2,
                        Action::YouTube => commands.push(Command::Reset((scenario_now + 1) % SCENARIOS.len())),
                        // Keyboard only.
                        Action::Digit(d) if (1..=SCENARIOS.len() as u8).contains(&d) => commands.push(Command::Reset(d as usize - 1)),
                        Action::Info => { show_help = !show_help; help_until = None; }
                        Action::Back | Action::Exit => {
                            log("quit");
                            sys::power_off(0);
                        }
                        _ => {}
                    }
                }
            }
            {
                let mut c = controls.lock().unwrap();
                c.pitch = (held.is(held.up, now) as i32 - held.is(held.down, now) as i32) as f64;
                c.roll = (held.is(held.right, now) as i32 - held.is(held.left, now) as i32) as f64;
                c.commands.extend(commands);
            }
            let (snap, stage, speed) = {
                let r = report.lock().unwrap();
                (r.snap.clone(), r.stage.clone(), r.speed)
            };
            match snap {
                None => {
                    splash(&mut screen, &mut frame, &stage);
                    std::thread::sleep(Duration::from_millis(100));
                    continue;
                }
                Some(snap) => {
                    (scenario_now, crashed) = (snap.scenario, snap.crashed);
                    let status = if snap.crashed { format!("crashed at t={:.1} s", snap.time) } else { String::new() };
                    let overlay = Overlay { fps, speed, show_help, status: &status };
                    if let Some((shared, gpu_frame)) = gpu.as_mut() {
                        shared.publish(&snap, &overlay, cam_mode);
                        // The renderer's picture, when it has a new one; otherwise wait a little.
                        match gpu_frame.read(last_gpu_frame, &mut frame) {
                            Some(seq) => {
                                last_gpu_frame = seq;
                                present(&mut screen, &frame);
                            }
                            None => std::thread::sleep(Duration::from_millis(8)),
                        }
                    } else {
                        raster.frame(world, &snap.s, cam_mode, &snap.path, snap.accepted);
                        raster.upscale(&mut frame);
                        hud(&mut frame, &snap, &overlay);
                        present(&mut screen, &frame);
                    }
                }
            }
            frames += 1;
            let span = sys::monotonic() - window;
            if span >= Duration::from_secs(2) {
                fps = frames as f64 / span.as_secs_f64();
                if gpu.is_some() {
                    log(format!("{fps:.1} loops/s with the GPU renderer, last GPU frame {last_gpu_frame}"));
                } else {
                    log(format!("{fps:.1} fps, {} triangles", raster.tris));
                }
                frames = 0;
                window = sys::monotonic();
            }
        }
    }
}
