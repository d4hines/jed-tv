//! Native PID 1 supervisor: static BusyBox, Ethernet PTY console, watchdog.
use super::{arg, State};
use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, Shutdown, TcpListener, TcpStream};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::process::Command;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const BUSYBOX: &[u8] = include_bytes!(env!("HELLO_BUSYBOX"));

fn protect_disks(state: &State) -> io::Result<()> {
    let entries = match fs::read_dir("/sys/class/block") {
        Ok(entries) => Some(entries),
        Err(_) if arg("hello.qemu", "0") == "1" => None,
        Err(error) => return Err(error),
    };
    let mut count = 0;
    for entry in entries.into_iter().flatten() {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if !name.starts_with("mmcblk0") { continue; }
        let file = File::open(format!("/dev/{name}"))?;
        let mut value: libc::c_int = 1;
        if unsafe { libc::ioctl(file.as_raw_fd(), 0x125du32 as _, &mut value) } != 0 { return Err(io::Error::last_os_error()); }
        value = 0;
        if unsafe { libc::ioctl(file.as_raw_fd(), 0x125eu32 as _, &mut value) } != 0 || value != 1 {
            return Err(io::Error::other("eMMC read-only verification failed"));
        }
        count += 1;
    }
    if count == 0 && arg("hello.qemu", "0") != "1" { return Err(io::Error::other("eMMC devices absent")); }
    state.event(format!("REPL: {count} eMMC block devices verified read-only"));
    Ok(())
}

fn watchdog_ioctl(file: &File, command: u32, argument: u32) -> io::Result<()> {
    let args = [command, argument];
    let result = unsafe { libc::ioctl(file.as_raw_fd(), 0xc0085503u32 as _, args.as_ptr()) };
    if result < 0 { return Err(io::Error::last_os_error()); }
    if result != 0 { return Err(io::Error::other(format!("WDT command {command} returned {result}"))); }
    Ok(())
}

fn watchdog(state: &State) -> io::Result<Option<File>> {
    if arg("hello.qemu", "0") == "1" {
        state.event("REPL: QEMU mode, hardware watchdog unavailable");
        return Ok(None);
    }
    let command = fs::read_to_string("/proc/cmdline")?;
    if !command.split_whitespace().any(|v| v == "androidboot.serialno=4C0490290FA05A0C0") {
        return Err(io::Error::other("refusing watchdog access on an unidentified board"));
    }
    for name in ["iniparser.ko", "kdrv_xc.ko", "utpa2k.ko"] {
        let module = fs::read(format!("/modules/{name}"))?;
        let params = CString::new("").unwrap();
        if unsafe { libc::syscall(libc::SYS_init_module, module.as_ptr(), module.len(), params.as_ptr()) } != 0
            && io::Error::last_os_error().raw_os_error() != Some(libc::EEXIST) {
            return Err(io::Error::last_os_error());
        }
        state.event(format!("REPL: watchdog prerequisite {name} ready"));
    }
    let file = OpenOptions::new().read(true).write(true).open("/proc/utopia")?;
    // Exact ARM32 V109 libutopia ABI: module ID, version, attribute pointer.
    let open = [64u32, 0, 0];
    let result = unsafe { libc::ioctl(file.as_raw_fd(), 0xc00c5501u32 as _, open.as_ptr()) };
    if result < 0 { return Err(io::Error::last_os_error()); }
    if result != 0 { return Err(io::Error::other(format!("WDT open returned {result}"))); }
    let debug_level = 0u32;
    // INIT establishes the kernel RIU mapping; HAL_WDT_Start is a no-op.
    // CLEAR feeds the existing watchdog without changing its timeout.
    watchdog_ioctl(&file, 0, &debug_level as *const u32 as usize as u32)?;
    watchdog_ioctl(&file, 2, 0)?;
    state.event("REPL: stock Utopia WDT initialized and fed; existing timeout retained");
    Ok(Some(file))
}

fn console(mut socket: TcpStream, state: &State) -> io::Result<()> {
    socket.set_nodelay(true)?;
    let fd = unsafe { libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY | libc::O_CLOEXEC) };
    if fd < 0 { return Err(io::Error::last_os_error()); }
    let mut master = unsafe { File::from_raw_fd(fd) };
    if unsafe { libc::grantpt(fd) } != 0 || unsafe { libc::unlockpt(fd) } != 0 { return Err(io::Error::last_os_error()); }
    let mut path = [0 as libc::c_char; 128];
    let error = unsafe { libc::ptsname_r(fd, path.as_mut_ptr(), path.len()) };
    if error != 0 { return Err(io::Error::from_raw_os_error(error)); }
    let path = unsafe { std::ffi::CStr::from_ptr(path.as_ptr()) }.to_str().map_err(io::Error::other)?;
    let slave = OpenOptions::new().read(true).write(true).custom_flags(libc::O_NOCTTY).open(path)?;
    let size = libc::winsize { ws_row: 30, ws_col: 100, ws_xpixel: 0, ws_ypixel: 0 };
    unsafe { libc::ioctl(slave.as_raw_fd(), libc::TIOCSWINSZ, &size); }
    let executable = CString::new("/bin/sh").unwrap();
    let flag = CString::new("-i").unwrap();
    let directory = CString::new("/root").unwrap();
    let environment = ["PATH=/bin:/sbin", "HOME=/root", "TERM=xterm-256color", "PS1=tv-repl# "]
        .map(|value| CString::new(value).unwrap());
    let argv = [executable.as_ptr(), flag.as_ptr(), std::ptr::null()];
    let envp = [environment[0].as_ptr(), environment[1].as_ptr(), environment[2].as_ptr(), environment[3].as_ptr(), std::ptr::null()];
    socket.write_all(b"Native TV console: our init is PID 1; Android is not running.\r\nCtrl-] disconnects; files remain in RAM.\r\n")?;
    let slave_fd = slave.as_raw_fd();
    // Prepare every allocation before fork. The child performs only
    // async-signal-safe libc calls until execve, avoiding Rust's extra
    // AF_UNIX exec-status socket requirement on the minimal QEMU kernel.
    let pid = unsafe { libc::fork() };
    if pid < 0 { return Err(io::Error::last_os_error()); }
    if pid == 0 {
        unsafe {
            if libc::setsid() < 0 || libc::ioctl(slave_fd, libc::TIOCSCTTY, 0) < 0 { libc::_exit(126); }
            for fd in 0..3 { if libc::dup2(slave_fd, fd) < 0 { libc::_exit(126); } }
            if libc::chdir(directory.as_ptr()) < 0 { libc::_exit(126); }
            libc::execve(executable.as_ptr(), argv.as_ptr(), envp.as_ptr());
            libc::_exit(127);
        }
    }
    drop(slave);
    state.event(format!("REPL: shell pid={pid} connected"));
    let mut input = socket.try_clone()?;
    let mut terminal = master.try_clone()?;
    let copy_in = thread::spawn(move || {
        let _ = io::copy(&mut input, &mut terminal);
        // Interrupt a shell blocked in terminal input on network disconnect.
        unsafe { libc::kill(-pid, libc::SIGHUP); }
    });
    let mut buffer = [0u8; 4096];
    loop {
        match master.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => if socket.write_all(&buffer[..n]).is_err() { break; },
            Err(error) if error.raw_os_error() == Some(libc::EIO) => break,
            Err(error) => { state.event(format!("REPL terminal: {error}")); break; }
        }
    }
    let _ = socket.shutdown(Shutdown::Both);
    unsafe { libc::kill(-pid, libc::SIGHUP); }
    drop(master);
    let _ = copy_in.join();
    state.event(format!("REPL: shell pid={pid} disconnected"));
    Ok(())
}

pub(super) fn start(state: Arc<State>, ip: Ipv4Addr, peer: Ipv4Addr, live_handoff: bool) -> io::Result<()> {
    state.phase("shell-setup");
    if !live_handoff { thread::sleep(Duration::from_secs(2)); }
    protect_disks(&state)?;
    for path in ["/bin", "/root", "/dev/pts", "/tmp"] { fs::create_dir_all(path)?; }
    if !live_handoff {
    fs::write("/bin/busybox", BUSYBOX)?;
    fs::set_permissions("/bin/busybox", fs::Permissions::from_mode(0o755))?;
    if !Command::new("/bin/busybox").args(["--install", "-s", "/bin"]).status()?.success() {
        return Err(io::Error::other("BusyBox applet installation failed"));
    }
    let source = CString::new("devpts").unwrap();
    let target = CString::new("/dev/pts").unwrap();
    let options = CString::new("mode=620,ptmxmode=666").unwrap();
    if unsafe { libc::mount(source.as_ptr(), target.as_ptr(), source.as_ptr(), 0, options.as_ptr().cast()) } != 0
        && io::Error::last_os_error().raw_os_error() != Some(libc::EBUSY) { return Err(io::Error::last_os_error()); }
    } else if !Path::new("/bin/sh").exists() {
        return Err(io::Error::other("live handoff requires existing shell"));
    }
    let feed = watchdog(&state)?;
    state.phase("watchdog-ready");
    if !live_handoff { thread::sleep(Duration::from_secs(2)); }
    let listener = TcpListener::bind((ip, 8766))?;
    let watchdog_log = state.clone();
    thread::spawn(move || loop {
        if let Some(ref file) = feed {
            if let Err(error) = watchdog_ioctl(file, 2, 0) { watchdog_log.event(format!("REPL watchdog feed failed: {error}")); }
        }
        thread::sleep(Duration::from_secs(2));
    });
    if live_handoff {
        state.service("direct", "refused");
        state.event("cold hardware startup refused after live PID1 replacement");
    } else if let Err(error) = super::hardware::start(state.clone()) {
        state.service("direct", "refused");
        state.event(format!("direct services refused: {error}"));
    }
    state.phase("shell-ready");
    state.event(format!("native PTY console on {ip}:8766, restricted to {peer}; PID 1 stays running"));
    thread::spawn(move || {
        for connection in listener.incoming() {
            match connection {
                Ok(socket) if socket.peer_addr().ok().map(|a| a.ip()) == Some(peer.into()) => {
                    let state = state.clone();
                    thread::spawn(move || {
                        if let Err(error) = console(socket, &state) { state.event(format!("REPL session: {error}")); }
                    });
                }
                Ok(_) => state.event("REPL: rejected non-host peer"),
                Err(error) => state.event(format!("REPL accept: {error}")),
            }
        }
    });
    Ok(())
}
