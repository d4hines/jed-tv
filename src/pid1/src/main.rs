use platform::{fb, sys};
use std::collections::{BTreeMap, VecDeque};
use std::ffi::CString;
use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::os::fd::AsRawFd;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(feature = "mi-display")]
mod mi;
#[cfg(feature = "repl")]
mod repl;
#[cfg(feature = "repl")]
mod volume_control;
#[cfg(feature = "repl")]
mod direct_services;
#[cfg(feature = "repl")]
mod audio_service;
#[cfg(feature = "repl")]
mod hardware;
#[cfg(feature = "repl")]
mod cold_plan;
#[cfg(feature = "repl")]
mod launcher;
#[cfg(feature = "repl")]
mod lean_entry;
#[cfg(feature = "repl")]
#[path="../../shared/volume.rs"]
mod volume;

const BUILD: &str = match option_env!("HELLO_BUILD_ID") { Some(s) => s, None => "development" };

struct State {
    start: Instant,
    phase: Mutex<String>,
    events: Mutex<VecDeque<String>>,
    services: Mutex<BTreeMap<&'static str, &'static str>>,
    media_owned: Mutex<bool>,
    child_wait_lock: Mutex<()>,
    foreground_child: Mutex<Option<(i32, Option<i32>)>>,
    launcher: tvinit::Control,
}

impl State {
    fn new(phase: &str) -> Self {
        Self { start: Instant::now(), phase: Mutex::new(phase.into()), events: Mutex::new(VecDeque::new()), services: Mutex::new(BTreeMap::new()), media_owned: Mutex::new(false), child_wait_lock: Mutex::new(()), foreground_child: Mutex::new(None), launcher: tvinit::Control::default() }
    }

    #[cfg(feature = "repl")]
    fn service(&self, name: &'static str, status: &'static str) {
        self.services.lock().unwrap().insert(name, status);
        self.event(format!("service={name} status={status}"));
    }

    fn event(&self, message: impl AsRef<str>) {
        let text = format!("[{:.3}] {}", self.start.elapsed().as_secs_f64(), message.as_ref());
        eprintln!("{text}");
        if let Ok(mut kernel) = std::fs::OpenOptions::new().write(true).open("/dev/kmsg") {
            let _ = writeln!(kernel, "<6>hello-pid1: {text}");
        }
        let mut events = self.events.lock().unwrap();
        if events.len() >= 256 { events.pop_front(); }
        events.push_back(text);
    }

    fn phase(&self, phase: &str) {
        *self.phase.lock().unwrap() = phase.to_owned();
        self.event(format!("phase={phase}"));
    }

    fn status(&self) -> String {
        let services = self.services.lock().unwrap().iter()
            .map(|(name, status)| format!("\"{name}\":\"{status}\""))
            .collect::<Vec<_>>().join(",");
        format!("{{\"app\":\"hello-pid1\",\"build\":\"{}\",\"pid\":{},\"phase\":\"{}\",\"uptime_ms\":{},\"filesystem_mounts\":\"RAM/pseudo filesystems; stock mounts read-only\",\"display_source\":\"{}\",\"services\":{{{services}}}}}\n",
                BUILD, std::process::id(), self.phase.lock().unwrap(), self.start.elapsed().as_millis(), self.launcher.display_source())
    }
}

fn arg(key: &str, fallback: &str) -> String {
    sys::cmdline_arg(key).unwrap_or_else(|| fallback.to_owned())
}

fn ipv4_address(ip: Ipv4Addr) -> libc::sockaddr {
    let address = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16, sin_port: 0,
        sin_addr: libc::in_addr { s_addr: u32::from_ne_bytes(ip.octets()) }, sin_zero: [0; 8],
    };
    unsafe { std::ptr::read((&address as *const libc::sockaddr_in).cast::<libc::sockaddr>()) }
}

fn network(interface: &str, ip: Ipv4Addr, mask: Ipv4Addr) -> io::Result<()> {
    if interface.len() >= libc::IFNAMSIZ { return Err(io::Error::other("interface name too long")); }
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 { return Err(io::Error::last_os_error()); }
    use std::os::fd::FromRawFd;
    let socket = unsafe { std::os::fd::OwnedFd::from_raw_fd(fd) };
    let mut req: libc::ifreq = unsafe { std::mem::zeroed() };
    for (dst, src) in req.ifr_name.iter_mut().zip(interface.bytes()) { *dst = src as libc::c_char; }
    req.ifr_ifru.ifru_addr = ipv4_address(ip);
    let call = |request: u32, req: &mut libc::ifreq| {
        if unsafe { libc::ioctl(socket.as_raw_fd(), request as _, req) } < 0 {
            Err(io::Error::last_os_error())
        } else { Ok(()) }
    };
    call(libc::SIOCSIFADDR, &mut req)?;
    req.ifr_ifru.ifru_addr = ipv4_address(mask);
    call(libc::SIOCSIFNETMASK, &mut req)?;
    call(libc::SIOCGIFFLAGS, &mut req)?;
    unsafe { req.ifr_ifru.ifru_flags |= (libc::IFF_UP | libc::IFF_RUNNING) as i16; }
    call(libc::SIOCSIFFLAGS, &mut req)
}

fn reply(stream: &mut TcpStream, status: &str, body: &str) -> io::Result<()> {
    write!(stream, "HTTP/1.0 {status}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len())
}

fn kernel_log() -> io::Result<String> {
    // SYSLOG_ACTION_READ_ALL copies the ring without consuming or clearing it.
    let mut bytes = vec![0u8; 256 * 1024];
    let count = unsafe { libc::klogctl(3, bytes.as_mut_ptr().cast(), bytes.len() as _) };
    if count < 0 { return Err(io::Error::last_os_error()); }
    bytes.truncate(count as usize);
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn read_request(stream: &mut TcpStream) -> io::Result<(String, String, Vec<u8>)> {
    let invalid = || io::Error::other("expected bounded HTTP headers and Content-Length body");
    let mut bytes = Vec::new();
    let header_end = loop {
        if let Some(offset) = bytes.windows(4).position(|s| s == b"\r\n\r\n") { break offset + 4; }
        if bytes.len() >= 4096 { return Err(invalid()); }
        let mut chunk = [0u8; 1024];
        let count = stream.read(&mut chunk)?;
        if count == 0 { return Err(invalid()); }
        bytes.extend_from_slice(&chunk[..count]);
    };
    if header_end > 4096 { return Err(invalid()); }
    let header = std::str::from_utf8(&bytes[..header_end]).map_err(|_| invalid())?;
    let mut lines = header.lines();
    let first = lines.next().ok_or_else(invalid)?.split_whitespace().collect::<Vec<_>>();
    if first.len() != 3 { return Err(invalid()); }
    let (method, path) = (first[0].to_owned(), first[1].to_owned());
    let mut length = None;
    for line in lines.filter(|line| !line.is_empty()) {
        let (key, value) = line.split_once(':').ok_or_else(invalid)?;
        if key.eq_ignore_ascii_case("transfer-encoding") { return Err(invalid()); }
        if key.eq_ignore_ascii_case("content-length") {
            if length.is_some() { return Err(invalid()); }
            length = Some(value.trim().parse::<usize>().map_err(|_| invalid())?);
        }
    }
    let length = length.unwrap_or(0);
    if length > 8 * 1024 * 1024 { return Err(invalid()); }
    let mut body = bytes.split_off(header_end);
    if body.len() > length { return Err(invalid()); }
    let mut chunk = [0u8; 16384];
    while body.len() < length {
        let remaining = (length - body.len()).min(chunk.len());
        let count = stream.read(&mut chunk[..remaining])?;
        if count == 0 { return Err(invalid()); }
        body.extend_from_slice(&chunk[..count]);
    }
    Ok((method, path, body))
}

fn stage_program(bytes: &[u8]) -> io::Result<()> {
    let invalid = || io::Error::other("expected a static little-endian ARM32 executable, at most 8 MiB");
    if bytes.len() < 52 || bytes.len() > 8 * 1024 * 1024 || &bytes[..7] != b"\x7fELF\x01\x01\x01"
        || bytes[16..20] != [2, 0, 40, 0] { return Err(invalid()); }
    let word = |offset: usize| u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
    let half = |offset: usize| u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as usize;
    let (offset, size, count, entry) = (word(28), half(42), half(44), word(24) & !1);
    if size != 32 || count == 0 || count > 128
        || offset.checked_add(size * count).is_none_or(|end| end > bytes.len()) { return Err(invalid()); }
    let mut executable_entry = false;
    for index in 0..count {
        let ph = offset + index * size;
        let kind = word(ph);
        if kind == 3 { return Err(invalid()); } // PT_INTERP requires a dynamic linker.
        if kind == 1 {
            let (file, address, file_size, memory_size, flags) =
                (word(ph + 4), word(ph + 8), word(ph + 16), word(ph + 20), word(ph + 24));
            if file_size > memory_size || file.checked_add(file_size).is_none_or(|end| end > bytes.len()) {
                return Err(invalid());
            }
            if flags & 1 != 0 && entry >= address && entry - address < memory_size { executable_entry = true; }
        }
    }
    if !executable_entry { return Err(invalid()); }
    let path = "/tmp/.hello-next";
    match std::fs::remove_file(path) { Ok(()) => (), Err(e) if e.kind() == io::ErrorKind::NotFound => (), Err(e) => return Err(e) }
    let mut file = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o700).open(path)?;
    file.write_all(bytes)?;
    file.set_permissions(std::fs::Permissions::from_mode(0o700))?;
    Ok(())
}

fn reboot(state: &State, why: &str) {
    let mode = arg("hello.return", "stock");
    state.event(format!("reboot requested: {why}; return={mode}; this program never writes firmware"));
    unsafe { libc::sync(); }
    if mode == "bootloader" {
        // The TV's handling of this restart command still needs a hardware test.
        let command = CString::new("bootloader").unwrap();
        unsafe { libc::syscall(libc::SYS_reboot, 0xfee1deadu32, 0x28121969u32,
                              0xa1b2c3d4u32, command.as_ptr()); }
    } else {
        unsafe { libc::reboot(libc::RB_AUTOBOOT); }
    }
    state.event(format!("reboot returned: {}", io::Error::last_os_error()));
}

fn server(state: Arc<State>, listener: TcpListener, peer: Ipv4Addr) {
    for connection in listener.incoming() {
        let mut stream = match connection { Ok(s) => s, Err(e) => { state.event(format!("accept: {e}")); continue; } };
        if stream.peer_addr().ok().map(|a| a.ip()) != Some(std::net::IpAddr::V4(peer)) {
            let _ = reply(&mut stream, "403 Forbidden", "This diagnostic channel is restricted to the directly connected host.\n");
            continue;
        }
        let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(2)));
        let (method, path, body) = match read_request(&mut stream) {
            Ok(request) => request,
            Err(e) => { let _ = reply(&mut stream, "400 Bad Request", &format!("{e}\n")); continue; }
        };
        match (method.as_str(), path.as_str()) {
            #[cfg(feature = "repl")]
            ("POST", "/launcher/reload") => {
                state.launcher.reload();
                let _ = reply(&mut stream, "200 OK", "Launcher reload requested.\n");
            }
            #[cfg(feature = "repl")]
            ("POST", "/launcher/stop") => {
                state.launcher.stop();
                let deadline = Instant::now() + Duration::from_secs(5);
                while state.launcher.active() && Instant::now() < deadline {
                    thread::sleep(Duration::from_millis(20));
                }
                let (code, message) = if state.launcher.active() {
                    ("503 Service Unavailable", "Launcher has not stopped; files must not be replaced.\n")
                } else { ("200 OK", "Launcher stopped.\n") };
                let _ = reply(&mut stream, code, message);
            }
            ("GET", "/status") => { let _ = reply(&mut stream, "200 OK", &state.status()); }
            #[cfg(feature = "repl")]
            ("GET", "/volume") => {
                let gain=volume_control::read_gain();
                let body=format!("{{\"owner_pid\":1,\"gain\":{gain},\"percent\":{},\"ready\":{}}}\n",
                    volume::level::percent(gain),Path::new("/tmp/pid1-volume.ready").exists());
                let _=reply(&mut stream,"200 OK",&body);
            }
            ("GET", "/kernel-log") => {
                match kernel_log() {
                    Ok(log) => { let _ = reply(&mut stream, "200 OK", &log); }
                    Err(error) => { let _ = reply(&mut stream, "500 Internal Server Error", &format!("{error}\n")); }
                }
            }
            #[cfg(feature = "repl")]
            ("PUT", path) if path.starts_with("/files/") => {
                let name = &path[7..];
                if name.is_empty() || name.len() > 100 || !name.bytes().all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c)) || name == "." || name == ".." {
                    let _ = reply(&mut stream, "400 Bad Request", "Expected a simple filename\n");
                    continue;
                }
                let save = || -> io::Result<()> {
                    let temporary = format!("/tmp/.upload-{name}");
                    match std::fs::remove_file(&temporary) {
                        Ok(()) => (), Err(error) if error.kind() == io::ErrorKind::NotFound => (), Err(error) => return Err(error),
                    }
                    let mut file = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o700).open(&temporary)?;
                    file.write_all(&body)?;
                    std::fs::rename(temporary, format!("/tmp/{name}"))
                };
                match save() {
                    Ok(()) => { let _ = reply(&mut stream, "200 OK", &format!("Saved {} bytes to /tmp/{name}\n", body.len())); }
                    Err(error) => { let _ = reply(&mut stream, "500 Internal Server Error", &format!("{error}\n")); }
                }
            }
            ("GET", "/logs") => {
                let text = state.events.lock().unwrap().iter().cloned().collect::<Vec<_>>().join("\n") + "\n";
                let _ = reply(&mut stream, "200 OK", &text);
            }
            ("POST", "/reboot") => {
                let _ = reply(&mut stream, "200 OK", "Requesting reboot.\n");
                drop(stream);
                thread::sleep(Duration::from_millis(200));
                reboot(&state, "debug host");
            }
            ("POST", "/exec") => {
                // Keep the claim locked through exec, so startup cannot acquire
                // DMA resources while the old supervisor is being replaced.
                let ownership = state.media_owned.lock().unwrap();
                if *ownership {
                    let _ = reply(&mut stream, "409 Conflict", "PID1 owns media resources. Restart applications, or reboot before replacing the supervisor.\n");
                    continue;
                }
                if let Err(e) = stage_program(&body) {
                    let _ = reply(&mut stream, "400 Bad Request", &format!("{e}\n")); continue;
                }
                state.event(format!("replacing PID 1 with uploaded static ARM executable ({} bytes)", body.len()));
                let _ = reply(&mut stream, "200 OK", "Executable staged in RAM; replacing PID 1.\n");
                drop(stream);
                thread::sleep(Duration::from_millis(100));
                let error = std::process::Command::new("/tmp/.hello-next").exec();
                state.event(format!("exec failed; current PID 1 retained: {error}"));
                drop(ownership);
            }
            _ => { let _ = reply(&mut stream, "404 Not Found", "GET /status, GET /logs, GET /kernel-log, POST /reboot, POST /exec\n"); }
        }
    }
}

fn display(state: &State) -> io::Result<()> {
    #[cfg(feature = "mi-display")]
    let mut mi_framebuffer = None;
    #[cfg(feature = "mi-display")]
    mi::stage(state)?;
    if arg("hello.modules", "1") != "0" {
        for name in std::fs::read_to_string("/modules/order")?.lines() {
            if name.is_empty() || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.') {
                return Err(io::Error::other("invalid module name"));
            }
            #[cfg(feature = "mi-display")]
            let before = if name == "mstar_fbdev_mi.ko" {
                mi::ready(state)?;
                Some(mi::framebuffer_nodes()?)
            } else { None };
            state.event(format!("loading {name}"));
            let data = std::fs::read(format!("/modules/{name}"))?;
            let params = CString::new("").unwrap();
            if unsafe { libc::syscall(libc::SYS_init_module, data.as_ptr(), data.len(), params.as_ptr()) } != 0 {
                let error = io::Error::last_os_error();
                if error.raw_os_error() != Some(libc::EEXIST) {
                    state.event(format!("module {name} failed: {error}"));
                    return Err(error);
                }
                state.event(format!("{name} is already loaded; retaining it"));
            }
            #[cfg(feature = "mi-display")]
            if let Some(before) = before { mi_framebuffer = Some(mi::select_framebuffer(state, &before)?); }
        }
    }
    let default_path = if Path::new("/dev/graphics/fb0").exists() { "/dev/graphics/fb0" } else { "/dev/fb0" };
    #[cfg(feature = "mi-display")]
    let path = mi_framebuffer.as_deref().unwrap_or(default_path);
    #[cfg(not(feature = "mi-display"))]
    let path = default_path;
    #[cfg(feature = "mi-display")]
    let prepared = if mi_framebuffer.is_some() { Some(mi::prepare_framebuffer(state, path)?) } else { None };
    state.event(format!("opening {path}"));
    #[cfg(feature = "mi-display")]
    let mut fb = if let Some(file) = prepared { mi::map_framebuffer(state, file)? } else { fb::Framebuffer::open(path)? };
    #[cfg(not(feature = "mi-display"))]
    let mut fb = fb::Framebuffer::open(path)?;
    state.event(format!("fb id={} {}x{} stride={} bpp={} pages={}", fb.id(), fb.var.xres, fb.var.yres,
                        fb.fix.line_length, fb.var.bits_per_pixel, fb.pages));
    let glyphs = [
        [17,17,17,31,17,17,17], [0,0,14,17,31,16,14], [12,4,4,4,4,4,14], [12,4,4,4,4,4,14],
        [0,0,14,17,17,17,14], [0,0,0,0,0,0,0], [0,0,17,17,21,21,10], [0,0,14,17,17,17,14],
        [0,0,22,25,16,16,16], [12,4,4,4,4,4,14], [1,1,15,17,17,17,15],
    ];
    let bg = fb.pixel(16, 32, 48);
    let fg = fb.pixel(244, 246, 250);
    for _ in 0..fb.pages.min(2) {
        let mut buffer = fb.back_buffer();
        buffer.px.fill(bg);
        let scale = (buffer.width / 70).min(buffer.height / 12).max(1);
        let left = buffer.width.saturating_sub((glyphs.len() * 6 - 1) * scale) / 2;
        let top = buffer.height.saturating_sub(7 * scale) / 2;
        for (i, glyph) in glyphs.iter().enumerate() {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4-col)) != 0 { buffer.fill_rect(left + (i*6+col)*scale, top+row*scale, scale, scale, fg); }
                }
            }
        }
        fb.present()?;
    }
    // Retain the mapping and file for the duration of this boot.
    std::mem::forget(fb);
    Ok(())
}

fn main() {
    #[cfg(feature = "mi-display")]
    if std::env::args().nth(1).as_deref() == Some("--init-display") {
        let state = State::new("display-init");
        if let Err(error) = mi::init_display(&state) { state.event(error.to_string()); std::process::exit(1); }
        return;
    }
    #[cfg(feature = "mi-display")]
    if std::env::args().nth(1).as_deref() == Some("--display-status") {
        let state = State::new("display-status");
        if let Err(error) = mi::report_output(&state) { state.event(error.to_string()); std::process::exit(1); }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--draw") {
        let state = State::new("display-setup");
        #[cfg(feature = "mi-display")]
        if std::env::args().any(|arg| arg == "--reset-osd") {
            if let Err(error) = mi::reset_osd(&state) { state.event(error.to_string()); std::process::exit(1); }
        }
        match display(&state) {
            Ok(()) => {
                state.phase("hello-visible");
                #[cfg(feature = "mi-display")]
                if let Err(error) = mi::report_output(&state) { state.event(format!("display status: {error}")); }
                thread::sleep(Duration::from_secs(arg("hello.hold", "300").parse::<u64>().unwrap_or(300).min(3600)));
            }
            Err(error) => { state.event(error.to_string()); std::process::exit(1); }
        }
        return;
    }
    if std::process::id() != 1 { eprintln!("Run this program as PID 1 inside its initramfs."); std::process::exit(2); }
    #[cfg(feature = "repl")]
    { lean_entry::run(); }
    #[cfg(not(feature = "repl"))]
    {
        let Ok((state, _, _, _)) = bootstrap() else { loop { thread::park(); } };
        if cfg!(feature = "diagnostics-only") {
            state.phase("diagnostics-only");
            state.event("display initialization skipped; existing kernel/module state preserved");
            loop { thread::park(); }
        }
        let display_delay = arg("hello.display_delay", "0").parse::<u64>().unwrap_or(0).min(10);
        if display_delay > 0 {
            state.phase("display-delay");
            state.event(format!("keeping diagnostics available for {display_delay}s before display setup"));
            thread::sleep(Duration::from_secs(display_delay));
        }
        state.phase("display-setup");
        match display(&state) {
            Ok(()) => state.phase("hello-visible"),
            Err(e) => { state.phase("display-failed"); state.event(e.to_string()); }
        }
        loop { thread::sleep(Duration::from_millis(250)); }
    }
}

fn bootstrap() -> Result<(Arc<State>, Ipv4Addr, Ipv4Addr, bool), ()> {
    let live_handoff=Path::new("/tmp/pid1-live-handoff").exists();
    if live_handoff { let _=std::fs::remove_file("/tmp/pid1-live-handoff"); }
    else { sys::init_mounts(); }
    // Set immutable process environment before any service starts threads.
    std::env::set_var("TV_FB_PATH", "/tmp/app.frames");
    if arg("hello.qemu", "0") != "1" {
        std::env::set_var("TV_AUDIO_ADDR", "127.0.0.1:8877");
        std::env::set_var("TV_HTTP_PROXY", "192.168.2.1:8899");
    }
    let state = Arc::new(State::new("early"));
    state.event(format!("hello-pid1 build={BUILD} pid={}", std::process::id()));
    let default_runtime = if cfg!(feature = "repl") { "0" } else if cfg!(feature = "mi-display") { "40" } else { "90" };
    let timeout_key = if cfg!(feature = "repl") { "hello.repl_timeout" } else { "hello.timeout" };
    let requested = arg(timeout_key, default_runtime).parse::<u64>().unwrap_or(40);
    let runtime = if cfg!(feature = "repl") && requested == 0 { 0 } else { requested.clamp(15, 300) };
    let watchdog = state.clone();
    if runtime > 0 {
        thread::spawn(move || { thread::sleep(Duration::from_secs(runtime)); reboot(&watchdog, "bounded test deadline"); });
    }
    let ip = arg("hello.ip", "192.168.77.2").parse::<Ipv4Addr>().expect("valid hello.ip");
    let peer = arg("hello.peer", "192.168.77.1").parse::<Ipv4Addr>().expect("valid hello.peer");
    let interface = arg("hello.iface", "eth0");
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        match network(&interface, ip, Ipv4Addr::new(255, 255, 255, 0)) {
            Ok(()) => break,
            Err(e) if Instant::now() < deadline => { state.event(format!("network waiting: {e}")); thread::sleep(Duration::from_millis(250)); }
            Err(e) => { state.phase("network-failed"); state.event(e.to_string()); return Err(()) }
        }
    }
    if let Err(error) = network("lo", Ipv4Addr::LOCALHOST, Ipv4Addr::new(255, 0, 0, 0)) {
        state.event(format!("loopback setup failed: {error}"));
    }
    let listener = match TcpListener::bind((ip, 8765)) {
        Ok(listener) => listener,
        Err(e) => { state.phase("debug-bind-failed"); state.event(e.to_string()); return Err(()) }
    };
    state.phase("debug-ready");
    let deadline_description = if runtime == 0 { "no automatic deadline".to_owned() } else { format!("automatic reboot in {runtime}s") };
    state.event(format!("diagnostics http://{ip}:8765; only peer {peer}; {deadline_description}"));
    let diagnostics = state.clone();
    thread::spawn(move || server(diagnostics, listener, peer));
    Ok((state, ip, peer, live_handoff))
}
