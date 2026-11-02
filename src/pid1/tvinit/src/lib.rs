//! Resident home screen and app supervision, embedded by the TV PID1.
//! The standalone entry point is also used by the small QEMU app harness.
//!
//! Mounts the pseudo filesystems, then loops: draw a smart-TV style home
//! screen on /dev/fb0 (rows of 16:9 tiles), read remote keys from evdev, run
//! the chosen app (`/bin/ytplay`, `/bin/doomtv`) as a separate process, and
//! come back when it exits. Apps never run as PID 1, so an app calling exit()
//! (Doom does) just returns here, and each app is isolated in its own process.
//!
//! Tile art is raw RGB24 at 288x162 in /share/tiles/<name>.rgb (made on the
//! host by tools/make_tiles.py), so the TV needs no image decoder.
//!
//! Kernel command line options:
//!   tvinit.utc_offset=-420 minutes east of UTC for the clock (default 0)
//!   tvinit.ip=A.B.C.D/N    static IPv4 on tvinit.iface (default eth0)
//!   tvinit.log=IP:PORT     also send every log line (incl. kernel log) over UDP
//!   tvinit.status=PORT     serve the boot log to anyone connecting over TCP
//!   tvinit.modules=FILE    kernel modules to load, in order (default /etc/modules.list)
//!   tvinit.hold=1          bring-up only: no home screen, just stay alive
#[path = "../../../shared/audio_level.rs"]
mod audio_level;

mod supervisor;
mod core_effects;
mod core_menu;
mod presentation;
pub mod lean_runtime;
mod launch;
mod broker;
mod settings_store;
mod storage_records;
mod settings_wifi;
mod wifi_codec;

use platform::remote::{self, Action as Key};
use platform::{fb, input, net, sys};
use std::ffi::CString;
use std::time::Duration;

const FOCUS_W: usize = 288; // focused tiles grow to the art's native size
const FOCUS_H: usize = 162;
fn log(msg: impl AsRef<str>) {
    let t = sys::monotonic();
    net::log_line(&format!("[{:>4}.{:03}] tvinit: {}", t.as_secs(), t.subsec_millis(), msg.as_ref()));
}

/// Bring-up for a kernel with nothing else in userspace (the TV): forward the
/// kernel log, configure a static IP, stream logs over UDP, serve the boot
/// log over TCP, and load kernel modules from a list. All driven by
///   tvinit.ip=192.168.77.2/24  tvinit.iface=eth0  tvinit.log=192.168.77.1:5140
///   tvinit.status=7777  tvinit.modules=/etc/modules.list
fn bring_up() {
    std::thread::spawn(net::forward_kmsg);
    if let Some((addr, prefix)) = arg("tvinit.ip").as_deref().and_then(net::parse_cidr) {
        let iface = arg("tvinit.iface").unwrap_or_else(|| "eth0".into());
        // The NIC driver may still be probing; retry for a few seconds.
        let mut result = Err(std::io::Error::other("not tried"));
        for _ in 0..50 {
            result = net::configure_ipv4(&iface, addr, prefix);
            if result.is_ok() {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        match result {
            Ok(()) => log(format!("network: {iface} = {addr}/{prefix}")),
            Err(e) => log(format!("network: configuring {iface} failed: {e}")),
        }
    }
    if let Some(dest) = arg("tvinit.log").and_then(|s| s.parse::<std::net::SocketAddrV4>().ok()) {
        match net::log_to_udp(dest) {
            Ok(()) => log(format!("logging to udp://{dest}")),
            Err(e) => log(format!("udp log to {dest} failed: {e}")),
        }
    }
    if let Some(port) = arg("tvinit.status").and_then(|s| s.parse::<u16>().ok()) {
        std::thread::spawn(move || status_server(port));
    }
    let list = arg("tvinit.modules").unwrap_or_else(|| "/etc/modules.list".into());
    if let Ok(text) = std::fs::read_to_string(&list) {
        load_modules(&text);
    }
}

/// Each line: "<path.ko> [params...]"; '#' starts a comment.
fn load_modules(list: &str) {
    let (mut ok, mut failed) = (0, 0);
    for line in list.lines().map(|l| l.split('#').next().unwrap_or("").trim()).filter(|l| !l.is_empty()) {
        let (path, params) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
        let t0 = sys::monotonic();
        match net::load_module(path, params.trim()) {
            Ok(()) => {
                ok += 1;
                log(format!("module {path} loaded in {} ms", (sys::monotonic() - t0).as_millis()));
            }
            Err(e) => {
                failed += 1;
                log(format!("module {path} FAILED: {e}"));
            }
        }
    }
    log(format!("modules: {ok} loaded, {failed} failed"));
}

/// Anyone connecting to this TCP port gets the boot log so far.
fn status_server(port: u16) {
    use std::io::Write;
    let Ok(listener) = std::net::TcpListener::bind(("0.0.0.0", port)) else {
        return log(format!("status: cannot listen on {port}"));
    };
    log(format!("status: boot log on tcp port {port}"));
    for mut conn in listener.incoming().flatten() {
        let text = net::log_snapshot().join("\n") + "\n";
        let _ = conn.write_all(text.as_bytes());
    }
}

/// Settings come from the kernel command line, or else from /etc/tvinit.conf
/// in the ramdisk (`key=value` per line), for bootloaders that rewrite the
/// command line.
fn arg(key: &str) -> Option<String> {
    sys::cmdline_arg(key).or_else(|| {
        let conf = std::fs::read_to_string("/etc/tvinit.conf").ok()?;
        conf.lines().map(str::trim).filter(|l| !l.starts_with('#'))
            .find_map(|l| l.strip_prefix(key)?.strip_prefix('=').map(str::to_owned))
    })
}

// ---------- drawing ----------

struct Painter<'a> {
    bb: fb::BackBuffer<'a>,
    format: fb::VarInfo,
}

impl Painter<'_> {
    fn rect(&mut self, x: isize, y: isize, w: usize, h: usize, c: u32) {
        let (x0, y0) = (x.max(0) as usize, y.max(0) as usize);
        let x1 = (x + w as isize).clamp(0, self.bb.width as isize) as usize;
        let y1 = (y + h as isize).clamp(0, self.bb.height as isize) as usize;
        if x1 > x0 && y1 > y0 {
            self.bb.fill_rect(x0, y0, x1 - x0, y1 - y0, c);
        }
    }

    fn frame(&mut self, x: isize, y: isize, w: usize, h: usize, t: usize, c: u32) {
        self.rect(x - t as isize, y - t as isize, w + 2 * t, t, c);
        self.rect(x - t as isize, y + h as isize, w + 2 * t, t, c);
        self.rect(x - t as isize, y, t, h, c);
        self.rect(x + w as isize, y, t, h, c);
    }

    fn gradient(&mut self, top: (u8, u8, u8), bottom: (u8, u8, u8)) {
        let h = self.bb.height;
        for y in 0..h {
            let f = |a: u8, b: u8| (a as isize + (b as isize - a as isize) * y as isize / h as isize) as u8;
            let c = self.format.pixel(f(top.0, bottom.0), f(top.1, bottom.1), f(top.2, bottom.2));
            let w = self.bb.width;
            self.bb.fill_rect(0, y, w, 1, c);
        }
    }

    /// 8x8 bitmap font at integer `scale`; returns the drawn width.
    fn text(&mut self, x: isize, y: isize, scale: usize, c: u32, s: &str) -> usize {
        use font8x8::legacy::BASIC_LEGACY;
        for (i, ch) in s.chars().enumerate() {
            let glyph = BASIC_LEGACY.get(ch as usize).copied().unwrap_or([0; 8]);
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << col) != 0 {
                        self.rect(x + ((i * 8 + col) * scale) as isize, y + (row * scale) as isize, scale, scale, c);
                    }
                }
            }
        }
        s.chars().count() * 8 * scale
    }

    /// Nearest-neighbour blit of RGB24 art (aw x ah) into a w x h rectangle.
    fn art(&mut self, x: isize, y: isize, w: usize, h: usize, art: &[u8], aw: usize, ah: usize) {
        for dy in 0..h {
            let sy = y + dy as isize;
            if sy < 0 || sy as usize >= self.bb.height {
                continue;
            }
            let row = (dy * ah / h) * aw * 3;
            for dx in 0..w {
                let sx = x + dx as isize;
                if sx < 0 || sx as usize >= self.bb.width {
                    continue;
                }
                let p = row + (dx * aw / w) * 3;
                let c = self.format.pixel(art[p], art[p + 1], art[p + 2]);
                self.bb.px[sy as usize * self.bb.stride + sx as usize] = c;
            }
        }
    }
}

fn about_text() -> Vec<String> {
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    unsafe { libc::uname(&mut uts) };
    let cstr = |b: &[libc::c_char]| unsafe { std::ffi::CStr::from_ptr(b.as_ptr()) }.to_string_lossy().into_owned();
    vec![
        format!("Linux {} {}, {} CPUs; native TV launcher", cstr(&uts.release), cstr(&uts.machine),
                std::thread::available_parallelism().map_or(0, |n| n.get())),
        if std::env::var_os("TV_FB_PATH").is_some() { "Display and audio: shared frames and PCM streams".into() } else { "Display and audio: framebuffer and PCM devices".into() },
        "YouTube: 2 JSON POSTs + ranged GETs; no JS engine, no tokens".into(),
        "  H.264 720p (FFmpeg decoder) + AAC (symphonia), rustls TLS".into(),
        "  sandboxed by seccomp: 46 syscalls, 11 ioctls".into(),
        "Doom: the GPL engine ported to Lean + Freedoom data".into(),
        "Each app runs as its own process; Back returns here".into(),
    ]
}

/// Build the fixed executable arguments and environment for a Core-authorized app.
fn prepare_command(path: &str, args: &[&str]) -> (CString, Vec<CString>, Vec<CString>) {
    let cpath = CString::new(path).unwrap();
    let mut argv: Vec<CString> = vec![cpath.clone()];
    argv.extend(args.iter().map(|a| CString::new(*a).unwrap()));
    // Pass the YouTube server on (the bootloader may rewrite the kernel command
    // line, so it can come from /etc/tvinit.conf); a tile's own setting wins.
    if !args.iter().any(|a| a.starts_with("ytplay.server=")) {
        if let Some(server) = arg("ytplay.server") {
            argv.push(CString::new(format!("ytplay.server={server}")).unwrap());
        }
    }
    // Doom settings (e.g. doom.net=host for a LAN game) come from the kernel
    // command line or /etc/tvinit.conf, like ytplay.server above.
    if path.ends_with("doomtv") {
        let mut words: Vec<String> = std::fs::read_to_string("/proc/cmdline").unwrap_or_default()
            .split_whitespace().filter(|w| w.starts_with("doom.")).map(str::to_owned).collect();
        if let Ok(conf) = std::fs::read_to_string("/etc/tvinit.conf") {
            words.extend(conf.lines().map(str::trim).filter(|l| l.starts_with("doom.")).map(str::to_owned));
        }
        argv.extend(words.into_iter().filter_map(|w| CString::new(w).ok()));
    }
    argv.push(c"tvapp.remote=3".to_owned());
    let mut environment = vec![CString::new("PATH=/bin").unwrap()];
    for key in ["TV_FB_PATH", "TV_AUDIO_ADDR", "TV_HTTP_PROXY"] {
        if let Ok(value) = std::env::var(key) {
            environment.push(CString::new(format!("{key}={value}")).unwrap());
        }
    }
    environment.push(CString::new(format!("TV_AUDIO_BOOST={}", if path.ends_with("doomtv") { 4 } else { 1 })).unwrap());
    // Also in the environment: a Rust staticlib under a C or Lean main cannot
    // read program arguments on musl (std::env::args is empty there).
    environment.push(CString::new("TVAPP_REMOTE=3").unwrap());
    (cpath, argv, environment)
}

fn load_art(name: &str) -> Option<Vec<u8>> {
    let a = std::fs::read(format!("/share/tiles/{name}.rgb")).ok()?;
    (a.len() == FOCUS_W * FOCUS_H * 3).then_some(a)
}

pub fn run_surrogate_display(control: &Control) -> ! { presentation::run(control, None) }

pub fn standalone() {
    // An external Android/bridge renderer keeps its original output file;
    // apps receive a separate input buffer that the Core can revoke.
    let external_output = std::env::var("TV_FB_PATH").ok();
    std::env::set_var("TV_FB_PATH", if external_output.is_some() { "/tmp/core-app.frames" } else { "/tmp/app.frames" });
    std::panic::set_hook(Box::new(|info| {
        eprintln!("tvinit: PANIC: {info}");
        sys::power_off(101);
    }));
    if let Err(error) = lean_runtime::initialize_runtime() {
        panic!("Lean foreground runtime initialization failed: {error}");
    }
    if sys::is_pid1() {
        sys::init_mounts();
        if let Ok(pnp) = std::fs::read_to_string("/proc/net/pnp") {
            let _ = std::fs::create_dir_all("/etc");
            let _ = std::fs::write("/etc/resolv.conf", pnp);
        }
        bring_up();
    }
    if arg("tvinit.hold").as_deref() == Some("1") {
        // Bring-up only (e.g. the first boot on the TV): stay alive, serve logs.
        log("hold: bring-up only, no home screen");
        loop {
            std::thread::park();
        }
    }
    // The surrogate uses exactly the same two-buffer ownership boundary as
    // the TV. Apps never receive the physical framebuffer or menu buffer.
    let control = std::sync::Arc::new(Control::default());
    presentation::start(control.clone(), external_output);
    control.reload();
    serve(&control, &mut LocalChildren);
}

/// Lifecycle shared by the resident launcher and the deployment endpoint.
pub struct Control {
    epoch: std::sync::atomic::AtomicU64,
    enabled: std::sync::atomic::AtomicBool,
    active: std::sync::atomic::AtomicBool,
    presentation: std::sync::Mutex<(bool, u64)>,
    app_frames: String,
}
impl Default for Control {
    fn default() -> Self {
        Self { epoch: Default::default(), enabled: Default::default(), active: Default::default(),
            presentation: Default::default(),
            app_frames: std::env::var("TV_FB_PATH").unwrap_or_else(|_| "/tmp/app.frames".into()) }
    }
}
impl Control {
    /// The display owner holds this lock through submission. Switching to Home
    /// therefore waits for any previous app submission to finish.
    pub fn with_presentation<R>(&self, action: impl FnOnce(&str, u64) -> R) -> R {
        let state = self.presentation.lock().unwrap();
        action(if state.0 { &self.app_frames } else { "/run/tv-core/menu.frames" }, state.1)
    }
    pub fn display_source(&self) -> String {
        if self.presentation.lock().unwrap().0 { self.app_frames.clone() } else { "/run/tv-core/menu.frames".into() }
    }
    pub fn present_app(&self, app: bool) {
        let mut state = self.presentation.lock().unwrap();
        state.0 = app;
        state.1 = state.1.wrapping_add(1);
    }

    pub fn reload(&self) {
        self.epoch.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.enabled.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn stop(&self) {
        self.enabled.store(false, std::sync::atomic::Ordering::SeqCst);
        self.epoch.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn active(&self) -> bool { self.active.load(std::sync::atomic::Ordering::SeqCst) }
    fn current(&self, epoch: u64) -> bool {
        self.enabled.load(std::sync::atomic::Ordering::SeqCst)
            && self.epoch.load(std::sync::atomic::Ordering::SeqCst) == epoch
    }
}

/// The process-wide child reaper supplies exit status to the UI.
pub trait Children {
    fn tick(&mut self) {}
    fn spawn(&mut self, start: &mut dyn FnMut() -> std::io::Result<i32>) -> std::io::Result<i32>;
    fn poll(&mut self, pid: i32) -> std::io::Result<Option<i32>>;
    fn signal(&mut self, pid: i32, signal: i32) -> std::io::Result<bool>;
}
struct LocalChildren;
impl Children for LocalChildren {
    fn signal(&mut self, pid: i32, signal: i32) -> std::io::Result<bool> {
        if unsafe { libc::kill(pid, signal) } == 0 { Ok(true) }
        else if std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH) { Ok(false) }
        else { Err(std::io::Error::last_os_error()) }
    }
    fn spawn(&mut self, start: &mut dyn FnMut() -> std::io::Result<i32>) -> std::io::Result<i32> { start() }
    fn poll(&mut self, pid: i32) -> std::io::Result<Option<i32>> {
        let mut status = 0;
        match unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) } {
            0 => Ok(None), n if n == pid => Ok(Some(status)),
            _ => Err(std::io::Error::last_os_error()),
        }
    }
}

/// Run inside PID1; no mounts, process replacement or global panic hook.
pub fn serve(control: &Control, children: &mut dyn Children) -> ! {
    if let Err(error) = lean_runtime::initialize_runtime() {
        panic!("Lean foreground runtime initialization failed: {error}");
    }
    let _lean_thread = lean_runtime::ThreadGuard::attach();
    loop {
        children.tick();
        let epoch = control.epoch.load(std::sync::atomic::Ordering::SeqCst);
        if !control.current(epoch) {
            std::thread::sleep(Duration::from_millis(20));
            continue;
        }
        control.active.store(true, std::sync::atomic::Ordering::SeqCst);
        if let Err(error) = core_effects::run(control, epoch, children) {
            log(format!("Lean core stopped: {error}"));
            std::thread::sleep(Duration::from_millis(100));
        }
        control.active.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

/// Run the home screen until a tile is chosen; returns (row, column).
fn native_volume(key: Key, repeat: bool) -> bool {
    if std::path::Path::new("/tmp/pid1-volume.ready").exists() {
        return matches!(key,Key::VolumeUp|Key::VolumeDown|Key::Mute);
    }
    if std::env::var_os("TV_AUDIO_ADDR").is_none() { return false; }
    static SAVED: std::sync::Mutex<f32> = std::sync::Mutex::new(audio_level::DEFAULT_GAIN);
    let current=std::fs::read_to_string("/tmp/app-volume").ok()
        .and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(audio_level::DEFAULT_GAIN);
    let next=match key {
        Key::VolumeUp => (current+audio_level::STEP).min(audio_level::MAX_GAIN),
        Key::VolumeDown => (current-audio_level::STEP).max(0.0),
        Key::Mute if !repeat => {
            if current>0.0 { *SAVED.lock().unwrap()=current; 0.0 }
            else { *SAVED.lock().unwrap() }
        },
        Key::Mute => return true,
        _ => return false,
    };
    if let Err(e)=std::fs::write("/tmp/app-volume",format!("{next}\n")) { log(format!("volume: {e}")); }
    else { log(format!("audio output ceiling {:.2}%",next*100.0)); }
    true
}
