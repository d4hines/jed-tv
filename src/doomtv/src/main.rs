//! doomtv: Doom as PID 1 on Linux (QEMU virt now, the TV later).
//!
//! The Rust main loop owns input and drives the C engine one tick at a time
//! (lib.rs). Two ways to run:
//! - As PID 1 (`test/run.py ... --bin doomtv`): mounts /dev, /proc, /sys. When
//!   Doom quits, PID 1 keeps running and shows a screen offering to play again
//!   (re-exec, so the engine starts from clean globals) or to power off.
//! - As a child of the launcher (tvinit runs /bin/doomtv): no mounts; when Doom
//!   quits the process just exits (0 = quit from the menu, 1 = engine error).
//!
//! Options, from program arguments or the kernel command line:
//!   doom.iwad=PATH       game data; default /doom/freedoom1.wad, else /freedoom1.wad
//!   doom.args=-warp,1,1  extra Doom arguments, comma separated
//!   doom.onquit=menu|poweroff|restart|exit   after Doom quits; default menu as
//!                        PID 1, exit otherwise
//!   doom.stats=5         seconds between frame/tic rate reports (0 = off)
//!   doom.sound=1         sound effects on /dev/snd/pcmC0D0p (0 = off)
//! Multiplayer: /etc/doom-network contains "host" or "join IPV4". Two players
//! auto-start private-LAN deathmatch; use the same FreeDM IWAD on both peers.
//! No file means ordinary solo play.
//! Other program arguments (and words after `--` on the kernel command line,
//! which the kernel hands to PID 1) go to Doom unchanged, e.g. `-warp 1 2`.

use doomtv::keys::ev;
use doomtv::{screen, wad, Doom, Exit, Stats};
use platform::{alsa, fb, input, sys};
use std::thread;
use std::time::Duration;

fn log(msg: impl AsRef<str>) {
    let t = sys::monotonic();
    println!("[{:>4}.{:03}] {}", t.as_secs(), t.subsec_millis(), msg.as_ref());
}

fn arg<T: std::str::FromStr>(key: &str, default: T) -> T {
    sys::cmdline_arg(key).and_then(|s| s.parse().ok()).unwrap_or(default)
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

/// Doom's argv (without argv[0]).
fn doom_args(iwad: &str) -> std::io::Result<Vec<String>> {
    let mut a = vec!["-iwad".to_string(), iwad.to_string()];
    if let Some(extra) = sys::cmdline_arg("doom.args") {
        a.extend(extra.split(',').filter(|s| !s.is_empty()).map(str::to_owned));
    }
    a.extend(std::env::args().skip(1).filter(|s| !s.starts_with("doom.") && !s.starts_with("tvapp.")));
    match std::fs::read_to_string("/etc/doom-network") {
        Ok(config) => {
            let words: Vec<_> = config.split_whitespace().collect();
            match words.as_slice() {
                ["host"] => a.push("-privateserver".into()),
                ["join", address] if address.parse::<std::net::Ipv4Addr>().is_ok() => {
                    a.extend(["-connect".into(), address.to_string()]);
                }
                _ => return Err(std::io::Error::other("/etc/doom-network: expected host or join IPV4")),
            }
            a.extend(["-deathmatch".into(), "-nomonsters".into()]);
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {},
        Err(e) => return Err(e),
    }
    Ok(a)
}

fn report(prev: &Stats, now: &Stats) {
    let wall = (now.at - prev.at).as_secs_f64();
    let frames = now.frames - prev.frames;
    let tics = now.tics - prev.tics;
    let busy = wall - (now.slept - prev.slept).as_secs_f64();
    let xruns = now.underruns - prev.underruns;
    let blit = (now.blit - prev.blit).as_secs_f64();
    let present = (now.present - prev.present).as_secs_f64();
    let per = |t: f64| 1e3 * t / frames.max(1) as f64;
    log(format!(
        "doom: {:.1} fps, {:.1} tics/s (35 = full speed), busy {:.0}%; per frame: scale {:.2} ms, \
         present {:.2} ms, engine+input+audio {:.2} ms; audio underruns {}",
        frames as f64 / wall, tics as f64 / wall, 100.0 * busy / wall,
        per(blit), per(present), per(busy - blit - present), xruns));
}

/// The main loop: route keys to Doom, tick it, report rates. Returns on quit.
fn run(doom: &mut Doom, inputs: &mut [input::Device], stats_every: f64) -> Exit {
    let mut last = doom.stats();
    loop {
        for d in inputs.iter_mut() {
            if let Ok(keys) = d.poll_keys() {
                for k in keys {
                    doom.key(k.code, k.value);
                }
            }
        }
        if let Some(e) = doom.tick() {
            return e;
        }
        let now = doom.stats();
        if stats_every > 0.0 && (now.at - last.at).as_secs_f64() >= stats_every {
            report(&last, &now);
            last = now;
        }
    }
}

/// Draw centered lines of Doom-font text on black (or a plain colour band if
/// the WAD's font is unavailable) into both pages.
fn message(fb: &mut fb::Framebuffer, font: Option<&wad::Font>, lines: &[&str]) {
    let fmt = screen::PixelFormat::of(fb);
    for _ in 0..fb.pages {
        let mut bb = fb.back_buffer();
        let (w, h, stride) = (bb.width, bb.height, bb.stride);
        bb.px.fill(fmt.pack(0, 0, 0));
        let scale = 3;
        match font {
            Some(font) => {
                let line_h = 12 * scale;
                let mut y = (h - lines.len() * line_h) / 2;
                for l in lines {
                    let x = w.saturating_sub(font.width(l) * scale) / 2;
                    font.draw(l, x, y, scale, &mut |x, y, [r, g, b]| {
                        if x < w && y < h {
                            bb.px[y * stride + x] = fmt.pack(r, g, b);
                        }
                    });
                    y += line_h;
                }
            }
            None => bb.fill_rect(0, h / 2 - 20, w, 40, fmt.pack(160, 0, 0)),
        }
        let _ = fb.present();
    }
}

fn reexec() -> ! {
    use std::os::unix::process::CommandExt;
    let args: Vec<String> = std::env::args().collect();
    log("doom: restarting (re-exec; PID stays the same)");
    let err = std::process::Command::new("/proc/self/exe")
        .arg0(args.first().map_or("/init", |s| s.as_str())).args(args.iter().skip(1)).exec();
    log(format!("doom: re-exec failed: {err}"));
    sys::power_off(1)
}

/// After Doom: PID 1 stays up. OK/Enter plays again, Back/Escape powers off.
fn after_doom(mut fb: fb::Framebuffer, inputs: &mut [input::Device], exit: Exit, iwad: &str) -> ! {
    let font = wad::Wad::open(iwad).ok().and_then(|mut w| wad::Font::load(&mut w));
    let status = match exit {
        Exit::Quit => "DOOM HAS QUIT".to_string(),
        Exit::Error(c) => format!("DOOM STOPPED: ERROR {c} (SEE CONSOLE)"),
    };
    message(&mut fb, font.as_ref(), &[&status, "", "PID 1 IS STILL RUNNING", "",
                                      "OK: PLAY AGAIN", "BACK: POWER OFF"]);
    log(format!("doom: engine ended ({exit:?}); PID {} still running, waiting for OK (play \
                 again) or Back (power off)", std::process::id()));
    loop {
        for d in inputs.iter_mut() {
            for k in d.poll_keys().unwrap_or_default() {
                if k.value != 1 {
                    continue;
                }
                match k.code {
                    ev::ENTER | ev::KPENTER | ev::OK | ev::SELECT => {
                        drop(fb);
                        reexec();
                    }
                    ev::ESC | ev::BACK | ev::EXIT => {
                        log("doom: powering off");
                        sys::power_off(0);
                    }
                    _ => {}
                }
            }
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("doomtv: PANIC: {info}");
        sys::power_off(101);
    }));
    if sys::is_pid1() {
        sys::init_mounts();
    }
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    unsafe { libc::uname(&mut uts) };
    let cstr = |b: &[libc::c_char]| unsafe { std::ffi::CStr::from_ptr(b.as_ptr()) }
        .to_string_lossy().into_owned();
    log(format!("doomtv {} on Linux {} {} ({} CPUs), pid {}", env!("CARGO_PKG_VERSION"),
                cstr(&uts.release), cstr(&uts.machine),
                thread::available_parallelism().map_or(0, |n| n.get()), std::process::id()));

    let iwad: String = sys::cmdline_arg("doom.iwad").unwrap_or_else(|| {
        let paths = if std::path::Path::new("/etc/doom-network").exists() {
            ["/doom/freedm.wad", "/doom/freedoom1.wad", "/freedoom1.wad"]
        } else { ["/doom/freedoom1.wad", "/freedoom1.wad", "/doom/freedm.wad"] };
        paths.into_iter()
            .find(|p| std::path::Path::new(p).exists()).unwrap_or("/doom/freedoom1.wad").to_string()
    });
    let onquit: String = arg("doom.onquit", if sys::is_pid1() { "menu" } else { "exit" }.to_string());
    let stats_every: f64 = arg("doom.stats", 5.0);
    if std::env::var_os("TV_FB_PATH").is_none() && !wait_for("/dev/fb0", 3000) {
        log("display: /dev/fb0 missing");
        sys::power_off(1);
    }
    let fb = match platform::display::Display::open_default() {
        Ok(fb) => fb,
        Err(e) => {
            log(format!("display: open failed: {e}"));
            sys::power_off(1);
        }
    };
    let v = fb.var;
    log(format!("display: {} {}x{} {}bpp R{} G{} B{} pages {} -> Doom {}x{} scaled to {:?}",
                fb.id(), v.xres, v.yres, v.bits_per_pixel, v.red.offset, v.green.offset,
                v.blue.offset, fb.pages, doomtv::RESX, doomtv::RESY,
                screen::fit_4_3(v.xres as usize, v.yres as usize)));
    let pcm_path = "/dev/snd/pcmC0D0p";
    if arg("doom.sound", 1) != 0 && (std::env::var_os("TV_AUDIO_ADDR").is_some() || wait_for(pcm_path, 1000)) {
        match alsa::Pcm::open(pcm_path, 48000, 2).or_else(|_| alsa::Pcm::open(pcm_path, 44100, 2)) {
            Ok(p) => {
                log(format!("audio: {pcm_path} S16_LE {} Hz x{} period {} buffer {} frames",
                            p.rate, p.channels, p.period_frames, p.buffer_frames));
                doomtv::attach_audio(p);
            }
            Err(e) => log(format!("audio: {pcm_path}: {e}; no sound")),
        }
    }
    if sys::cmdline_arg("tvapp.remote").is_none() { wait_for("/dev/input/event0", 1000); }
    let mut inputs = input::open_all();
    for d in &inputs {
        log(format!("input: {} \"{}\"", d.path, d.name));
    }

    let args = doom_args(&iwad).unwrap_or_else(|e| {
        log(format!("doom: network configuration: {e}"));
        sys::power_off(1)
    });
    log(format!("doom: starting engine with {args:?}"));
    doomtv::attach_display(fb);
    let t0 = sys::monotonic();
    let exit = match Doom::start(&args) {
        Ok(mut doom) => {
            log(format!("doom: engine up in {:.2}s", (sys::monotonic() - t0).as_secs_f64()));
            run(&mut doom, &mut inputs, stats_every)
        }
        Err(e) => e,
    };
    log(format!("doom: engine exited: {exit:?}"));
    let fb = doomtv::detach_display().expect("display");
    match onquit.as_str() {
        "exit" => std::process::exit(if exit == Exit::Quit { 0 } else { 1 }),
        "poweroff" => {
            log("doom: powering off");
            sys::power_off(0)
        }
        "restart" => {
            drop(fb);
            reexec()
        }
        _ => after_doom(fb, &mut inputs, exit, &iwad),
    }
}
