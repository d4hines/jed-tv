//! doomtv: Doom as PID 1 on Linux (QEMU virt now, the TV later).
//!
//! The Rust main loop owns input and drives the C engine one tick at a time
//! (lib.rs). Two ways to run:
//! - As PID 1 (`qemu/run.py ... --bin doomtv`): mounts /dev, /proc, /sys. When
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
//! Sound and networking are absent from this build.
//! Other program arguments (and words after `--` on the kernel command line,
//! which the kernel hands to PID 1) go to Doom unchanged, e.g. `-warp 1 2`.

use doomtv::keys::ev;
use doomtv::{screen, wad, Doom, Exit, Stats};
use platform::{fb, input, sys};
mod tv;
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
fn doom_args(iwad: &str) -> Vec<String> {
    let mut a = vec!["-iwad".to_string(), iwad.to_string(), "-nomusic".into()];
    if !cfg!(feature = "audio") { a.push("-nosound".into()); }
    if let Some(extra) = sys::cmdline_arg("doom.args").or(Some("-warp,1,1".into())) {
        a.extend(extra.split(',').filter(|s| !s.is_empty()).map(str::to_owned));
    }
    a.extend(std::env::args().skip(1).filter(|s| !s.starts_with("doom.")));
    a
}

fn report(prev: &Stats, now: &Stats) {
    let wall = (now.at - prev.at).as_secs_f64();
    let frames = now.frames - prev.frames;
    let tics = now.tics - prev.tics;
    let busy = wall - (now.slept - prev.slept).as_secs_f64();
    let blit = (now.blit - prev.blit).as_secs_f64();
    let present = (now.present - prev.present).as_secs_f64();
    let per = |t: f64| 1e3 * t / frames.max(1) as f64;
    log(format!(
        "doom: {:.1} fps, {:.1} tics/s (35 = full speed), busy {:.0}%; per frame: scale {:.2} ms, \
         present {:.2} ms, engine+input {:.2} ms",
        frames as f64 / wall, tics as f64 / wall, 100.0 * busy / wall,
        per(blit), per(present), per(busy - blit - present)));
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

/// After Doom: PID 1 stays up. Enter/OK plays again, Escape/Back powers off.
fn after_doom(mut fb: fb::Framebuffer, inputs: &mut [input::Device], exit: Exit, iwad: &str) -> ! {
    let font = wad::Wad::open(iwad).ok().and_then(|mut w| wad::Font::load(&mut w));
    let status = match exit {
        Exit::Quit => "DOOM HAS QUIT".to_string(),
        Exit::Error(c) => format!("DOOM STOPPED: ERROR {c} (SEE CONSOLE)"),
    };
    message(&mut fb, font.as_ref(), &[&status, "", "PID 1 IS STILL RUNNING", "",
                                      "ENTER: PLAY AGAIN", "ESC: POWER OFF"]);
    log(format!("doom: engine ended ({exit:?}); PID {} still running, waiting for Enter (play \
                 again) or Escape (power off)", std::process::id()));
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
        tv::halt("panic; retaining PID 1");
    }));
    if sys::is_pid1() {
        sys::init_mounts();
        tv::prepare().unwrap_or_else(|e| tv::halt(&format!("early setup failed: {e}")));
    }
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    unsafe { libc::uname(&mut uts) };
    let cstr = |b: &[libc::c_char]| unsafe { std::ffi::CStr::from_ptr(b.as_ptr()) }
        .to_string_lossy().into_owned();
    log(format!("doomtv {} on Linux {} {} ({} CPUs), pid {}", env!("CARGO_PKG_VERSION"),
                cstr(&uts.release), cstr(&uts.machine),
                thread::available_parallelism().map_or(0, |n| n.get()), std::process::id()));

    let iwad: String = sys::cmdline_arg("doom.iwad").unwrap_or_else(|| {
        ["/doom/freedoom1.wad", "/freedoom1.wad"].into_iter()
            .find(|p| std::path::Path::new(p).exists()).unwrap_or("/doom/freedoom1.wad").to_string()
    });
    let onquit: String = arg("doom.onquit", if sys::is_pid1() { "menu" } else { "exit" }.to_string());
    let stats_every: f64 = arg("doom.stats", 5.0);
    let fb_path = arg("doom.fb", if std::path::Path::new("/dev/graphics/fb0").exists() { "/dev/graphics/fb0" } else { "/dev/fb0" }.to_string());
    let frame_file = sys::cmdline_arg("doom.frame-file");
    if frame_file.is_none() && !wait_for(&fb_path, 10000) {
        log("display: /dev/fb0 missing");
        tv::halt("framebuffer missing");
    }
    let fb = match frame_file.map_or_else(|| fb::Framebuffer::open(&fb_path),
        |path| fb::Framebuffer::shared_file(&path, 320, 240)) {
        Ok(fb) => fb,
        Err(e) => {
            log(format!("display: open failed: {e}"));
            tv::halt(&format!("framebuffer unavailable: {e}"));
        }
    };
    let v = fb.var;
    log(format!("display: {} {}x{} {}bpp R{} G{} B{} pages {} -> Doom {}x{} scaled to {:?}",
                fb.id(), v.xres, v.yres, v.bits_per_pixel, v.red.offset, v.green.offset,
                v.blue.offset, fb.pages, doomtv::RESX, doomtv::RESY,
                screen::fit_4_3(v.xres as usize, v.yres as usize)));
    wait_for("/dev/input/event0", 1000);
    let mut inputs = input::open_all();
    for d in &inputs {
        log(format!("input: {} \"{}\"", d.path, d.name));
    }

    let args = doom_args(&iwad);
    log(format!("doom: starting engine with {args:?}"));
    doomtv::attach_display(fb);
    let t0 = sys::monotonic();
    let exit = match Doom::start(&args) {
        Ok(mut doom) => {
            log(format!("doom: engine up in {:.2}s", (sys::monotonic() - t0).as_secs_f64()));
            #[cfg(feature = "audio")]
            if let Some(path) = sys::cmdline_arg("doom.audio-pipe") {
                start_audio_pipe(path);
            }
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

#[cfg(feature = "audio")]
fn start_audio_pipe(path: String) {
    std::thread::spawn(move || {
        use std::io::Write;
        unsafe extern "C" { fn doomtv_sound_mix(out: *mut i16, frames: i32, rate: i32); }
        let Ok(mut pipe) = std::fs::OpenOptions::new().write(true).open(path) else { return; };
        let mut pcm = [0i16; 960];
        let mut deadline = std::time::Instant::now();
        loop {
            unsafe { doomtv_sound_mix(pcm.as_mut_ptr(), 480, 48000); }
            let bytes = unsafe { std::slice::from_raw_parts(pcm.as_ptr() as *const u8, pcm.len() * 2) };
            if pipe.write_all(bytes).is_err() { break; }
            deadline += Duration::from_millis(10);
            if let Some(wait) = deadline.checked_duration_since(std::time::Instant::now()) { std::thread::sleep(wait); }
        }
    });
}
