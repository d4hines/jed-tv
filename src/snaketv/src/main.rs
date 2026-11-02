//! snaketv: snake on a torus on the TV, steered with the remote.
//!
//! The game is prototype 10's (`main.S`, see the crate docs in lib.rs). What
//! changes is the hardware: the round panel and bit-banged SPI become the
//! platform framebuffer, the knob becomes the remote, and SYSTIMER's
//! periodic alarm becomes the monotonic clock. The frame schedule is the
//! same: step k happens at T_0 + k * PERIOD, so lateness never builds up.
//!
//! Remote:
//!   Left / Right   turn (one press is one detent of the knob; the latest
//!                  press before a step wins, as the knob's did)
//!   OK             after a crash: a new game (the knob's push)
//!   Back / Exit    leave (Home is the launcher's)
//!
//! Under the launcher Back exits 0 and the home screen returns; as PID 1
//! (`test/run.py ... --bin snaketv`) it mounts /dev, /proc, /sys first and
//! powers off instead.
//!
//! On a non-Linux host, `cargo run -p snaketv -- out.ppm [steps]` plays a
//! scripted game for some steps and writes one 1280x720 frame (a layout
//! preview without QEMU).

#[cfg(target_os = "linux")]
fn main() {
    tv::main()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    use snaketv::render::{draw, Colors, Screen};
    use snaketv::{Game, Heading, Turn, SECTORS, START_RING};
    use std::io::Write;
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).cloned().unwrap_or_else(|| "snaketv.ppm".into());
    let steps: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(60);
    let (w, h) = (1280usize, 720usize);
    let t0 = std::time::Instant::now();
    let screen = Screen::new(w, h);
    println!("map built in {:.1} ms", t0.elapsed().as_secs_f64() * 1e3);
    // A longer snake than the start, so the picture shows body, head and apples.
    let r = START_RING * SECTORS;
    let mut cells: Vec<usize> = (0..12).map(|s| r + s).collect();
    cells.extend([r + 11 + SECTORS, r + 11 + 2 * SECTORS, r + 10 + 2 * SECTORS, r + 9 + 2 * SECTORS]);
    let mut game = Game::from_cells(&cells, Heading::Anticlockwise);
    for i in 0..steps {
        if i % 9 == 4 {
            game.turn(if i % 2 == 0 { Turn::Right } else { Turn::Left });
        }
        if game.step().is_none() {
            break;
        }
    }
    let colors = Colors::new(|r, g, b| (r as u32) << 16 | (g as u32) << 8 | b as u32);
    let mut px = vec![0u32; w * h];
    let t1 = std::time::Instant::now();
    draw(&screen, &game, &colors, &mut px, w);
    println!("frame drawn in {:.1} ms: length {}, step {}, {}", t1.elapsed().as_secs_f64() * 1e3,
             game.length(), game.steps, if game.over { "over" } else { "playing" });
    let mut f = std::io::BufWriter::new(std::fs::File::create(&out).expect("create"));
    write!(f, "P6\n{w} {h}\n255\n").unwrap();
    for p in &px {
        f.write_all(&[(p >> 16) as u8, (p >> 8) as u8, *p as u8]).unwrap();
    }
    println!("{out}");
}

#[cfg(target_os = "linux")]
mod tv {
    use platform::remote::{self, Action};
    use platform::{fb, input, seccomp, sys};
    use snaketv::render::{draw, Colors, Screen};
    use snaketv::{Game, Outcome, Turn, PERIOD_MS, RINGS, SECTORS};
    use std::thread;
    use std::time::Duration;

    const PERIOD: Duration = Duration::from_millis(PERIOD_MS);

    fn log(msg: impl AsRef<str>) {
        let t = sys::monotonic();
        println!("[{:>4}.{:03}] snaketv: {}", t.as_secs(), t.subsec_millis(), msg.as_ref());
    }

    fn quit(code: i32) -> ! {
        if sys::is_pid1() {
            log("powering off");
        }
        sys::power_off(code)
    }

    fn wait_for(path: &str, ms: u64) -> bool {
        for _ in 0..ms / 20 {
            if std::path::Path::new(path).exists() {
                return true;
            }
            thread::sleep(Duration::from_millis(20));
        }
        false
    }

    /// Paint the whole frame into the back page and show it.
    fn present(fb: &mut fb::Framebuffer, screen: &Screen, game: &Game, colors: &Colors) {
        {
            let bb = fb.back_buffer();
            let stride = bb.stride;
            draw(screen, game, colors, bb.px, stride);
        }
        if let Err(e) = fb.present() {
            log(format!("present failed: {e}"));
        }
    }

    pub fn main() {
        std::panic::set_hook(Box::new(|info| {
            eprintln!("snaketv: PANIC: {info}");
            sys::power_off(101);
        }));
        if sys::is_pid1() {
            sys::init_mounts();
        }
        // No threads exist yet, so the filter covers the whole process.
        match seccomp::install(&seccomp::base_syscalls(), &seccomp::platform_ioctls()) {
            Ok(()) => log("seccomp allowlist installed (base syscalls + platform ioctls, no network)"),
            Err(e) => log(format!("seccomp: {e}")),
        }
        log(format!("world: {SECTORS} sectors x {RINGS} rings, a step every {PERIOD_MS} ms"));
        if std::env::var_os("TV_FB_PATH").is_none() && !wait_for("/dev/fb0", 3000) {
            log("display: /dev/fb0 missing");
            quit(1);
        }
        let mut fb = match fb::Framebuffer::open("/dev/fb0") {
            Ok(fb) => fb,
            Err(e) => {
                log(format!("display: open failed: {e}"));
                quit(1);
            }
        };
        let v = fb.var;
        log(format!("display: {} {}x{} {}bpp R{} G{} B{} pages {}", fb.id(), v.xres, v.yres, v.bits_per_pixel,
                    v.red.offset, v.green.offset, v.blue.offset, fb.pages));
        if sys::cmdline_arg("tvapp.remote").is_none() {
            wait_for("/dev/input/event0", 1000);
        }
        let mut devs = input::open_all();
        for d in &devs {
            log(format!("input: {} \"{}\"", d.path, d.name));
        }

        let t0 = sys::monotonic();
        let screen = Screen::new(v.xres as usize, v.yres as usize);
        log(format!("map: {}x{} pixels round ({:.0}, {:.0}), disc {:.0}, rings {:.0} to {:.0}, built in {} ms",
                    screen.box_w, screen.box_h, screen.cx, screen.cy, screen.inner, screen.inner, screen.outer,
                    (sys::monotonic() - t0).as_millis()));
        let colors = Colors::new(|r, g, b| v.pixel(r, g, b));

        let mut game = Game::new();
        log(format!("new game: length {} at cell {}, heading {}", game.length(), game.head_cell, game.heading.name()));
        present(&mut fb, &screen, &game, &colors);
        // The frame timer: step k is due at T_0 + k * PERIOD.
        let mut due = sys::monotonic() + PERIOD;
        loop {
            for (action, repeat) in remote::poll(&mut devs) {
                // An IR remote repeats while held; the knob gave one turn per detent.
                if repeat {
                    continue;
                }
                match action {
                    Action::Left => game.turn(Turn::Left),
                    Action::Right => game.turn(Turn::Right),
                    Action::Ok if game.over => {
                        game = Game::new();
                        log(format!("new game: length {} at cell {}, heading {}", game.length(), game.head_cell, game.heading.name()));
                        present(&mut fb, &screen, &game, &colors);
                        due = sys::monotonic() + PERIOD;
                    }
                    Action::Back | Action::Exit => {
                        log("quit");
                        quit(0);
                    }
                    _ => {}
                }
            }
            let now = sys::monotonic();
            if game.over || now < due {
                thread::sleep(if game.over { Duration::from_millis(20) } else { (due - now).min(Duration::from_millis(5)) });
                continue;
            }
            let turned = game.turn;
            let t = sys::monotonic();
            let outcome = game.step();
            if let Some(turn) = turned {
                log(format!("turn {}: heading {}", if turn == Turn::Left { "left" } else { "right" }, game.heading.name()));
            }
            match outcome {
                Some(Outcome::Ate) => log(format!("ate the apple at cell {}: length {}", game.head_cell, game.length())),
                Some(Outcome::Crashed) => log(format!("crashed into itself after {} steps, length {}; OK plays again",
                                                      game.steps, game.length())),
                _ if game.steps.is_multiple_of(80) => log(format!("step {}: length {}, cell {}, heading {}", game.steps,
                                                        game.length(), game.head_cell, game.heading.name())),
                _ => {}
            }
            present(&mut fb, &screen, &game, &colors);
            if game.steps == 1 {
                log(format!("first frame: step and paint took {:.1} ms", (sys::monotonic() - t).as_secs_f64() * 1e3));
            }
            // The next frame is due a period later; if we fell more than a
            // period behind (the process was stopped), start the schedule
            // again rather than run a burst of steps.
            due += PERIOD;
            if due <= sys::monotonic() {
                due = sys::monotonic() + PERIOD;
            }
        }
    }
}
