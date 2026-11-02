//! ytplay: minimal YouTube player. Runs as PID 1 on Linux (QEMU virt now, the
//! TV later).
//!
//! Kernel command line options:
//!   ytplay.mode=app|play|browse|framemd5|selftest|bench  default play
//!   ytplay.query=TEXT                   browse: search for TEXT right away
//!   ytplay.file=/path/136.mp4           bench input (local file, no network)
//!   ytplay.server=http://10.0.2.2:8090  base URL (replay server on the host)
//!   ytplay.video=PVGeM40dABA            video ID
//!   ytplay.frames=N                     stop after N video frames
//!   ytplay.trace=1                      print a syscall/ioctl census (ptrace)

mod browse;
mod decode;
mod http;
mod hub;
mod player;
mod pool;
mod selftest;
mod trace;
mod yt;
mod yuv;

pub use platform::{alsa, fb, input, sys};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn log(msg: impl AsRef<str>) {
    let t = sys::monotonic();
    println!("[{:>4}.{:03}] {}", t.as_secs(), t.subsec_millis(), msg.as_ref());
}

/// Wait up to `ms` for a device node; drivers may still be probing.
pub fn wait_for(path: &str, ms: u64) -> bool {
    for _ in 0..ms / 50 {
        if std::path::Path::new(path).exists() {
            return true;
        }
        thread::sleep(Duration::from_millis(50));
    }
    false
}

fn arg<T: std::str::FromStr>(key: &str, default: T) -> T {
    sys::cmdline_arg(key).and_then(|s| s.parse().ok()).unwrap_or(default)
}

fn selftest() {
    let seconds = 4.0;
    let stop = Arc::new(AtomicBool::new(false));
    let inp = { let s = stop.clone(); thread::spawn(move || selftest::input_watch(s)) };
    let disp = { let s = stop.clone(); thread::spawn(move || selftest::display_test(seconds, s)) };
    let aud = thread::spawn(move || selftest::audio_test(seconds.min(3.0)));
    match sys::cmdline_arg("ytplay.server") {
        Some(server) => selftest::network_test(&server, &arg("ytplay.video", "PVGeM40dABA".to_string())),
        None => log("net: no ytplay.server= on the kernel command line; skipping"),
    }
    let _ = aud.join();
    let _ = disp.join();
    stop.store(true, Ordering::Relaxed);
    let _ = inp.join();
}

fn run_player(mode: &str) -> std::io::Result<()> {
    if mode == "bench" {
        return player::bench(&arg("ytplay.file", String::new()), arg("ytplay.frames", usize::MAX), "1,2,4");
    }
    let server = sys::cmdline_arg("ytplay.server")
        .ok_or_else(|| std::io::Error::other("need ytplay.server="))?;
    let limit: usize = arg("ytplay.frames", usize::MAX);
    let threads = 0; // decoder picks its thread count
    if mode == "app" {
        return hub::run(&server, limit, threads);
    }
    if mode == "browse" {
        // Search screen; OK plays a result and Back comes back here.
        let mut ui = browse::Browse::new(&server, &arg("ytplay.query", String::new()));
        while let Some(item) = ui.interact()? {
            log(format!("browse: playing {} \"{}\"", item.id, item.title));
            let result = play_video(&server, &item.id, "play", limit, threads);
            if let Err(e) = &result {
                log(format!("browse: {} failed: {e}", item.id));
            }
            ui.played(&result);
        }
        return Ok(());
    }
    play_video(&server, &arg("ytplay.video", "PVGeM40dABA".to_string()), mode, limit, threads)
}

/// Resolve a video ID and play it (or check it, in framemd5 mode).
fn play_video(server: &str, video_id: &str, mode: &str, limit: usize, threads: u32) -> std::io::Result<()> {
    let v = yt::resolve(server, video_id)?;
    let vf = yt::pick_video(&v.formats, 720) // the team's cap: the panel is 1280x720
        .ok_or_else(|| std::io::Error::other("no H.264 format"))?;
    let af = yt::pick_audio(&v.formats).ok_or_else(|| std::io::Error::other("no AAC format"))?;
    log(format!("\"{}\": video itag {} {}x{}@{}, audio itag {}", v.title, vf.itag, vf.width,
                vf.height, vf.fps, af.itag));
    let video = player::Track::open(vf)?;
    log(format!("video: {} segments, timescale {}, edit offset {}", video.index.segments.len(),
                video.init.timescale, video.init.edit_offset));
    if mode == "framemd5" {
        return player::framemd5(&video, limit, threads);
    }
    let audio = player::Track::open(af)?;
    let duration = if v.length_s > 0 { v.length_s as f64 } else { 0.0 };
    player::play(video, audio, &v.title, duration, limit, threads)
}

/// Restrict this process to the measured platform API (see platform::seccomp).
fn sandbox() {
    let mut syscalls = platform::seccomp::base_syscalls();
    syscalls.extend(platform::seccomp::brokered_syscalls());
    let ioctls = platform::seccomp::platform_ioctls();
    match platform::seccomp::install(&syscalls, &ioctls) {
        Ok(()) => log(format!("seccomp: allowlist of {} syscalls, {} ioctl requests", syscalls.len(), ioctls.len())),
        Err(e) => log(format!("seccomp: not installed: {e}")),
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("ytplay: PANIC: {info}");
        sys::power_off(101);
    }));
    if sys::is_pid1() {
        sys::init_mounts();
        // Kernel DHCP (ip=dhcp) writes the nameserver to /proc/net/pnp in
        // resolv.conf format; musl's resolver reads /etc/resolv.conf.
        if let Ok(pnp) = std::fs::read_to_string("/proc/net/pnp") {
            let _ = std::fs::create_dir_all("/etc");
            let _ = std::fs::write("/etc/resolv.conf", pnp);
        }
    }
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    unsafe { libc::uname(&mut uts) };
    let cstr = |b: &[libc::c_char]| unsafe { std::ffi::CStr::from_ptr(b.as_ptr()) }
        .to_string_lossy().into_owned();
    log(format!("ytplay {} on Linux {} {} ({} CPUs), pid {}", env!("CARGO_PKG_VERSION"),
                cstr(&uts.release), cstr(&uts.machine),
                thread::available_parallelism().map_or(0, |n| n.get()), std::process::id()));
    let mode: String = arg("ytplay.mode", "play".to_string());
    let dispatch = |mode: &str| {
        sandbox();
        match mode {
            "selftest" => selftest(),
            // Negative test: a syscall outside the allowlist must be trapped.
            "seccomp-test" => { log(format!("getuid() returned {}", unsafe { libc::getuid() })); }
            m => if let Err(e) = run_player(m) { log(format!("{m}: error: {e}")) },
        }
    };
    if arg("ytplay.trace", 0u32) == 1 {
        // Syscall census: run the mode in a ptrace-traced child (see trace.rs).
        let census = trace::run_traced(|| dispatch(&mode));
        census.print();
    } else {
        dispatch(&mode);
    }
    log("done; powering off");
    sys::power_off(0);
}
