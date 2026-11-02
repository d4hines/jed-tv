//! Start a native Android client from adb root without passing su-owned FDs
//! into Binder. The child retains uid 0, but uses stock shell SELinux access.
use std::{ffi::CString, fs::OpenOptions, os::fd::AsRawFd, os::unix::process::CommandExt};
extern "C" {
    fn close(fd: i32) -> i32;
    fn dup2(old: i32, new: i32) -> i32;
    fn fcntl(fd: i32, command: i32, ...) -> i32;
    fn getuid() -> u32;
    fn umask(mask: u32) -> u32;
}
fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() < 2 || unsafe { getuid() } != 0 {
        return Err(std::io::Error::other("usage (root): android-client LOG PROGRAM [ARGS...]"));
    }
    let context = std::fs::read_to_string("/proc/self/attr/current")?;
    if context.trim_end_matches(['\0', '\n']) != "u:r:su:s0" {
        return Err(std::io::Error::other("expected adb-root su domain"));
    }
    let fds: Vec<i32> = std::fs::read_dir("/proc/self/fd")?
        .filter_map(Result::ok).filter_map(|e| e.file_name().to_str()?.parse().ok()).collect();
    // Change the current domain before exec opens its executable. setexeccon
    // alone leaves that descriptor owned by su, which stock shell cannot use.
    let domain = CString::new("u:r:shell:s0").unwrap();
    std::fs::write("/proc/self/attr/current", domain.as_bytes_with_nul())?;
    for fd in fds { unsafe { close(fd); } }
    unsafe { umask(0o022); }
    let input = OpenOptions::new().read(true).open("/dev/null")?;
    let output = OpenOptions::new().create(true).append(true).open(&args[0])?;
    unsafe { dup2(input.as_raw_fd(), 0); dup2(output.as_raw_fd(), 1); dup2(output.as_raw_fd(), 2); }
    // OpenOptions uses CLOEXEC, and dup2(fd, fd) does not clear that flag.
    for fd in 0..=2 { unsafe { fcntl(fd, 2, 0); } }
    // Keep the duplicated standard descriptors when these File values drop.
    std::mem::forget(input);
    std::mem::forget(output);
    Err(std::process::Command::new(&args[1]).args(&args[2..]).exec())
}
