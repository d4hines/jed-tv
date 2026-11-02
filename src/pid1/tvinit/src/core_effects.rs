//! The native effect interpreter for TV.Core. All launcher policy is in Lean.
use super::{broker, core_menu::CoreMenu, input, launch, lean_runtime, log, native_volume,
    prepare_command, remote, settings_store::CoreStore, sys, Children, Control, Key};
use std::{cell::Cell, collections::VecDeque, ffi::c_void, io,
    os::fd::{AsRawFd, OwnedFd}, time::{Duration, Instant}};
use lean_runtime::Object;

extern "C" {
    fn tv_core_main() -> Object;
    fn tv_core_io_word(word: u64) -> Object;
    fn tv_core_io_unit(unused: u32) -> Object;
    fn tv_core_io_error(errno: u32) -> Object;
    fn tv_app_network(app: u32, grants: u32) -> u32;
}
/// One launch of an app. `pending` holds the raw bytes of the request Core is
/// deciding on; Lean parses them (TV.Core.parseRequest), this file never does.
struct Child { app: u32, launch: u32, pid: i32, remote: OwnedFd, requests: OwnedFd, pending: Option<Vec<u8>>, name: String, started: Instant }
struct Context<'a> {
    control: &'a Control,
    epoch: u64,
    children: &'a mut dyn Children,
    child: Option<Child>,
    menu: CoreMenu,
    devices: Vec<input::Device>,
    keys: VecDeque<input::Key>,
    choices: VecDeque<u64>,
    last_tick: Instant,
    store: Option<CoreStore>,
    app_visible: bool,
    save_failed: bool,
    /// Reports on network changes, delivered to Core by `next` (tag 6).
    net_reports: VecDeque<u64>,
    /// When `next` last handed Core an app request: at most one per tick, and
    /// only when no key is waiting, so no app can delay Home or Power.
    last_request: Instant,
}
thread_local! { static CURRENT: Cell<*mut c_void> = const { Cell::new(std::ptr::null_mut()) }; }
struct Scope;
impl Drop for Scope { fn drop(&mut self) { CURRENT.with(|p| p.set(std::ptr::null_mut())); } }
fn with<R>(action: impl FnOnce(&mut Context<'_>) -> R) -> R {
    CURRENT.with(|p| {
        assert!(!p.get().is_null(), "Core effect outside interpreter");
        // This synchronous interpreter owns its context on this thread. The
        // scope clears the pointer before the stack-owned context is dropped.
        unsafe { action(&mut *p.get().cast::<Context<'_>>()) }
    })
}
fn word(value: u64) -> Object { unsafe { tv_core_io_word(value) } }
fn unit() -> Object { unsafe { tv_core_io_unit(0) } }

pub(super) fn run(control: &Control, epoch: u64, children: &mut dyn Children) -> io::Result<()> {
    let mut context = Context {
        control, epoch, children, child: None, menu: CoreMenu::new()?,
        devices: input::open_all(), keys: VecDeque::new(), choices: VecDeque::new(),
        last_tick: Instant::now(), store: None, app_visible: false, save_failed: false,
        net_reports: VecDeque::new(), last_request: Instant::now(),
    };
    CURRENT.with(|p| { assert!(p.get().is_null()); p.set((&mut context as *mut Context<'_>).cast()); });
    let scope = Scope;
    let result = unsafe { lean_runtime::consume_io_unit(tv_core_main()) };
    drop(scope);
    // Development stop and native IO failures terminate this run, not PID1.
    // Restore the trusted screen before signalling the remaining child.
    if context.save_failed {
        context.menu.set_enabled(0); context.menu.set_storage_available(false);
    }
    context.menu.show_home();
    control.present_app(false);
    if let Some(child) = context.child.take() {
        let _ = context.children.signal(child.pid, libc::SIGTERM);
        let deadline = Instant::now() + Duration::from_millis(1500);
        let mut killed = false;
        loop {
            context.children.tick();
            if let Some(raw) = context.children.poll(child.pid)? {
                log(format!("{} {} after {:.0}s", child.name, super::supervisor::status(raw), child.started.elapsed().as_secs_f64()));
                break;
            }
            if !killed && Instant::now() >= deadline {
                let _ = context.children.signal(child.pid, libc::SIGKILL);
                killed = true;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    result
}

#[no_mangle]
extern "C" fn tv_core_next() -> Object {
    word(with(|ctx| {
        ctx.children.tick();
        if !ctx.control.current(ctx.epoch) { return 255; }
        if let Some(child) = &ctx.child {
            match ctx.children.poll(child.pid) {
                Ok(Some(raw)) => {
                    let (app, launch) = (child.app, child.launch);
                    log(format!("{} {} after {:.0}s", child.name, super::supervisor::status(raw), child.started.elapsed().as_secs_f64()));
                    ctx.child = None;
                    return exit_word(app, launch);
                }
                Err(error) => {
                    log(format!("Core wait failed: {error}"));
                    // Do not invent an exit: stop safely if the child registry
                    // cannot attest to its process identity.
                    return 255;
                }
                Ok(None) => (),
            }
        }
        // Ticks cannot starve behind a held key or busy menu queue.
        if ctx.last_tick.elapsed() >= Duration::from_millis(20) {
            ctx.last_tick = Instant::now(); return 0;
        }
        if let Some(choice) = ctx.choices.pop_front() { return choice; }
        if let Some(report) = ctx.net_reports.pop_front() { return report; }
        ctx.menu.poll();
        if ctx.keys.is_empty() {
            for device in &mut ctx.devices { ctx.keys.extend(device.poll_keys().unwrap_or_default()); }
        }
        while let Some(key) = ctx.keys.pop_front() {
            let action = remote::action(key.code);
            if matches!(action, Some(Key::VolumeUp | Key::VolumeDown | Key::Mute)) {
                if key.value != 0 { native_volume(action.unwrap(), key.value == 2); }
                continue;
            }
            if key.value == 0 {
                if matches!(action, Some(Key::Home | Key::Power)) { continue; }
                return 4 | (key.code as u64) << 32;
            }
            let kind = match action { Some(Key::Home) => 0, Some(Key::Power) => 1, _ => 2 };
            return 1 | kind << 8 | ((key.value == 2) as u64) << 16 | (key.code as u64) << 32;
        }
        // Only now, with no key waiting: one request from the app per tick.
        // Core learns the sender from the socket it arrived on.
        if ctx.last_request.elapsed() >= Duration::from_millis(20) {
            if let Some(child) = ctx.child.as_mut().filter(|c| c.pending.is_none()) {
                if let Some(bytes) = broker::receive(&child.requests) {
                    ctx.last_request = Instant::now();
                    child.pending = Some(bytes);
                    return 5 | (child.app as u64) << 8 | ((child.launch & 0xff_ffff) as u64) << 40;
                }
            }
        }
        if ctx.devices.is_empty() { ctx.devices = input::open_all(); }
        std::thread::sleep(Duration::from_millis(2));
        ctx.last_tick = Instant::now();
        0
    }))
}
#[no_mangle]
extern "C" fn tv_core_clock() -> Object { word(sys::monotonic().as_nanos() as u64) }
#[no_mangle]
extern "C" fn tv_core_load() -> Object {
    word(with(|ctx| match CoreStore::open() {
        Ok((store, policy)) => { ctx.store = Some(store); ctx.menu.set_storage_available(true); policy }
        Err(error) => {
            log(format!("Core settings unavailable: {error}"));
            ctx.menu.set_storage_available(false); 1u64 << 32
        }
    }))
}
#[no_mangle]
extern "C" fn tv_core_save(pin: u32, enabled: u32, grants: u32, offline: u8) -> Object {
    let result = with(|ctx| {
        let result = ctx.store.as_ref().ok_or_else(|| io::Error::other("settings unavailable"))
            .and_then(|store| store.save(pin, enabled, grants, offline != 0));
        if let Err(error) = &result {
            ctx.save_failed = true;
            log(format!("Core save failed; closing controller: {error}"));
        }
        result
    });
    match result { Ok(()) => unit(), Err(error) => unsafe { tv_core_io_error(error.raw_os_error().unwrap_or(libc::EIO) as u32) } }
}
#[no_mangle]
extern "C" fn tv_core_show_home() -> Object {
    with(|ctx| {
        ctx.app_visible = false;
        ctx.menu.show_home();
        ctx.control.present_app(false);
        log("Core: home owns display and input");
    }); unit()
}
#[no_mangle]
extern "C" fn tv_core_show_app(id: u32) -> Object {
    with(|ctx| {
        if ctx.child.as_ref().is_some_and(|c| c.app == id) {
            ctx.menu.show_app(); ctx.control.present_app(true); ctx.app_visible = true;
        }
    }); unit()
}
#[no_mangle]
extern "C" fn tv_core_menu_key(kind: u8, code: u32, held: u8) -> Object {
    with(|ctx| {
        if let Some(choice) = ctx.menu.handle_key(kind, code, if held != 0 { 2 } else { 1 }) {
            ctx.choices.push_back(choice);
        }
    }); unit()
}
#[no_mangle]
extern "C" fn tv_core_menu_policy(enabled: u32, grants: u32, offline: u8) -> Object {
    with(|ctx| {
        log(format!("Core policy for menus: enabled={enabled:#x} grants={grants:#x} offline={}", offline != 0));
        ctx.menu.set_enabled(enabled);
    }); unit()
}
#[no_mangle]
extern "C" fn tv_core_tell(kind: u8, id: u32) -> Object {
    with(|ctx| { log(format!("Core notice kind={kind} app={id}")); ctx.menu.tell(kind, id); }); unit()
}
#[no_mangle]
extern "C" fn tv_core_start(id: u32, launch: u32, grants: u32) -> Object {
    with(|ctx| {
        let start = (|| -> io::Result<Child> {
            if ctx.child.is_some() { return Err(io::Error::other("child slot occupied")); }
            let (path, args) = ctx.menu.take_launch(id).ok_or_else(|| io::Error::other("no matching menu launch"))?;
            if super::settings_store::app_id(path) != id { return Err(io::Error::other("manifest mismatch")); }
            let name = if id == 1 && args.iter().any(|s| s.starts_with("ytplay.video=")) { "Demo video" }
                else { match id { 1 => "YouTube", 2 => "Doom", 3 => "Slides", 5 => "Infinite RPG", 6 => "City flight", 7 => "Axiom", 8 => "Snake", _ => "App" } }.to_owned();
            log(format!("starting {name} ({path} {})", args.join(" ")));
            let (path, args, mut env) = prepare_command(path, args);
            // An app with connections from PID 1 (network bit 2 in TV/Apps.lean)
            // asks for them on descriptor 4 (platform::broker).
            if unsafe { tv_app_network(id, grants) } & 2 != 0 { env.push(c"TVAPP_REQUESTS=4".to_owned()); }
            let mut argv: Vec<_> = args.iter().map(|s| s.as_ptr()).collect(); argv.push(std::ptr::null());
            let mut envp: Vec<_> = env.iter().map(|s| s.as_ptr()).collect(); envp.push(std::ptr::null());
            let directory = launch::prepare(id)?;
            let (remote, input) = launch::socket_pair()?;
            let (requests, app_requests) = launch::socket_pair()?;
            let pid = ctx.children.spawn(&mut || launch::start(&path, &argv, &envp, id, grants, &input, &app_requests, &directory))?;
            log(format!("Core app {id} launch {launch}: confined pid {pid}, grants {grants:#x}"));
            Ok(Child { app: id, launch, pid, remote, requests, pending: None, name, started: Instant::now() })
        })();
        match start {
            Ok(child) => ctx.child = Some(child),
            Err(error) => {
                log(format!("Core app {id} failed to start: {error}"));
                ctx.choices.push_back(exit_word(id, launch));
            }
        }
    }); unit()
}
#[no_mangle]
extern "C" fn tv_core_signal(id: u32, kill: u8) -> Object {
    with(|ctx| if let Some(child) = &ctx.child {
        if child.app == id {
            if kill == 0 { log("home button: stopping the app"); }
            let _ = ctx.children.signal(child.pid, if kill != 0 { libc::SIGKILL } else { libc::SIGTERM });
        }
    }); unit()
}
#[no_mangle]
extern "C" fn tv_core_forward(id: u32, code: u32, value: u8) -> Object {
    with(|ctx| {
        if !ctx.app_visible || value > 2 || code > u16::MAX as u32 { return; }
        if let Some(child) = &ctx.child {
            if child.app == id {
                let _ = input::forward_key(child.remote.as_raw_fd(), input::Key { code: code as u16, value: value as i32 });
            }
        }
    }); unit()
}
#[no_mangle]
extern "C" fn tv_core_power_off() -> Object { sys::power_off(0) }
#[no_mangle]
extern "C" fn tv_core_request_byte(index: u32) -> Object {
    word(with(|ctx| ctx.child.as_ref().and_then(|c| c.pending.as_ref())
        .and_then(|bytes| bytes.get(index as usize)).map_or(256, |&b| b as u64)))
}
#[no_mangle]
extern "C" fn tv_core_reply(id: u32, launch: u32, granted: u8, host_start: u32, host_length: u32, port: u32) -> Object {
    with(|ctx| if let Some(child) = ctx.child.as_mut().filter(|c| c.app == id && c.launch & 0xff_ffff == launch & 0xff_ffff) {
        let Some(bytes) = child.pending.take() else { return };
        // The reply begins with the request's first four bytes, copied unread.
        let number: [u8; 4] = bytes.get(..4).and_then(|b| b.try_into().ok()).unwrap_or([0; 4]);
        let host = bytes.get(host_start as usize..(host_start + host_length) as usize);
        match (granted != 0, host) {
            (true, Some(host)) => {
                log(format!("broker: granted {} launch {launch} {}:{port}", child.name, String::from_utf8_lossy(host)));
                if let Err(error) = broker::grant(&child.requests, number, host.to_vec(), port as u16, |m| log(m)) {
                    log(format!("broker: could not start: {error}"));
                    broker::refuse(&child.requests, number);
                }
            }
            _ => {
                log(format!("broker: refused {} launch {launch}", child.name));
                broker::refuse(&child.requests, number);
            }
        }
    }); unit()
}
#[no_mangle]
extern "C" fn tv_core_network(op: u32, up: u8) -> Object {
    with(|ctx| {
        let up = up != 0;
        let ok = network::switch(up);
        log(format!("network change {op}: {} {}", if up { "up" } else { "down" }, if ok { "confirmed" } else { "NOT confirmed" }));
        ctx.net_reports.push_back(6 | (up as u64) << 8 | (ok as u64) << 9 | (op as u64) << 32);
    }); unit()
}
#[no_mangle]
extern "C" fn tv_core_daemon(daemon: u8, start: u8) -> Object {
    // The Wi-Fi daemons become Core outputs in lane F; until then the menus'
    // existing Wi-Fi flow still runs them.
    log(format!("Core daemon {} {}", if daemon == 0 { "wpa_supplicant" } else { "udhcpc" }, if start != 0 { "start" } else { "stop" }));
    unit()
}
#[no_mangle]
extern "C" fn tv_core_save_wifi() -> Object {
    log("Core: Wi-Fi network saved with the PIN");
    unit()
}
#[no_mangle]
extern "C" fn tv_core_reboot() -> Object {
    log("Core: restarting the TV at the user's request");
    if std::process::id() == 1 {
        unsafe { libc::sync(); libc::reboot(libc::LINUX_REBOOT_CMD_RESTART); }
    }
    unit()
}

/// `next`'s word for the end of a launch (tag 3).
fn exit_word(app: u32, launch: u32) -> u64 {
    3 | (app as u64) << 8 | ((launch & 0xff_ffff) as u64) << 40
}

/// The interface layer of the offline switch: every interface except the
/// loopback goes down (or up), and is read back. `tvinit.keep=IFACE` keeps a
/// development debug link up; production builds pass no such argument.
mod network {
    use std::{ffi::CString, fs};
    const SIOCGIFFLAGS: libc::c_ulong = 0x8913;
    const SIOCSIFFLAGS: libc::c_ulong = 0x8914;
    #[repr(C)] struct IfReq { name: [libc::c_char; 16], flags: libc::c_short, pad: [u8; 22] }

    fn flags(socket: i32, name: &str, set: Option<bool>) -> Option<bool> {
        let mut request = IfReq { name: [0; 16], flags: 0, pad: [0; 22] };
        let bytes = CString::new(name).ok()?;
        for (slot, b) in request.name.iter_mut().zip(bytes.as_bytes()) { *slot = *b as libc::c_char; }
        if unsafe { libc::ioctl(socket, SIOCGIFFLAGS as _, &mut request) } != 0 { return None; }
        if let Some(up) = set {
            request.flags = if up { request.flags | libc::IFF_UP as libc::c_short } else { request.flags & !(libc::IFF_UP as libc::c_short) };
            if unsafe { libc::ioctl(socket, SIOCSIFFLAGS as _, &request) } != 0 { return None; }
            if unsafe { libc::ioctl(socket, SIOCGIFFLAGS as _, &mut request) } != 0 { return None; }
        }
        Some(request.flags & libc::IFF_UP as libc::c_short != 0)
    }

    pub fn switch(up: bool) -> bool {
        let keep = super::super::arg("tvinit.keep");
        let socket = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM | libc::SOCK_CLOEXEC, 0) };
        if socket < 0 { return false; }
        let mut ok = true;
        for entry in fs::read_dir("/sys/class/net").into_iter().flatten().flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name == "lo" || keep.as_deref() == Some(name.as_str()) { continue; }
            ok &= flags(socket, &name, Some(up)) == Some(up);
        }
        unsafe { libc::close(socket); }
        ok
    }
}
