//! PID 1 duties: mount the pseudo filesystems, read the kernel command line,
//! and power off instead of exiting (init exiting panics the kernel).

use std::ffi::CString;
use std::io;

fn mount(src: &str, target: &str, fstype: &str) -> io::Result<()> {
    let _ = std::fs::create_dir_all(target);
    let (s, t, f) = (CString::new(src)?, CString::new(target)?, CString::new(fstype)?);
    let r = unsafe { libc::mount(s.as_ptr(), t.as_ptr(), f.as_ptr(), 0, std::ptr::null()) };
    if r == 0 || io::Error::last_os_error().raw_os_error() == Some(libc::EBUSY) {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// Mount devtmpfs, proc and sysfs. Only meaningful when running as PID 1.
pub fn init_mounts() {
    for (src, target, fs) in [("devtmpfs", "/dev", "devtmpfs"), ("proc", "/proc", "proc"),
                              ("sysfs", "/sys", "sysfs")] {
        if let Err(e) = mount(src, target, fs) {
            eprintln!("ytplay: mount {target}: {e}");
        }
    }
}

/// Value of `key=value` from /proc/cmdline (we use the `ytplay.` prefix).
pub fn cmdline_arg(key: &str) -> Option<String> {
    // Program arguments first (when launched by hand, e.g. from adb shell),
    // then the kernel command line (when running as PID 1).
    let pick = |kv: &str| kv.strip_prefix(key)?.strip_prefix('=').map(str::to_owned);
    if let Some(v) = std::env::args().skip(1).find_map(|a| pick(&a)) {
        return Some(v);
    }
    // Apps whose main is not Rust's (C, Lean) get no std::env::args on musl;
    // the launcher mirrors `tvapp.remote=N` as TVAPP_REMOTE=N.
    if let Ok(v) = std::env::var(key.replace('.', "_").to_ascii_uppercase()) {
        return Some(v);
    }
    let line = std::fs::read_to_string("/proc/cmdline").ok()?;
    line.split_whitespace().find_map(pick)
}

pub fn is_pid1() -> bool {
    std::process::id() == 1
}

/// Power the machine off (QEMU exits). Falls back to exiting when not PID 1.
pub fn power_off(code: i32) -> ! {
    if is_pid1() {
        unsafe {
            libc::sync();
            libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF);
        }
        loop {
            std::thread::park();
        }
    }
    std::process::exit(code)
}

pub fn monotonic() -> std::time::Duration {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
    std::time::Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32)
}

/// ioctl request number, as the kernel's _IOC macro builds it (ARM/generic layout).
pub const fn ioc(dir: u32, ty: u8, nr: u8, size: usize) -> u32 {
    (dir << 30) | ((size as u32) << 16) | ((ty as u32) << 8) | nr as u32
}
pub const IOC_NONE: u32 = 0;
pub const IOC_WRITE: u32 = 1;
pub const IOC_READ: u32 = 2;

/// ioctl with a typed argument; returns the OS error on failure.
///
/// # Safety
/// `arg` must be the struct type the kernel expects for `req`.
pub unsafe fn ioctl<T>(fd: i32, req: u32, arg: *mut T) -> io::Result<i32> {
    let r = libc::ioctl(fd, req as _, arg);
    if r < 0 { Err(io::Error::last_os_error()) } else { Ok(r) }
}
