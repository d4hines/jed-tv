//! Linux-enforced boundary around the retained G06 TEE helper.
use std::{fs, io, os::unix::fs::{chown, FileTypeExt, MetadataExt, PermissionsExt}, path::Path};

// These are allocation-free scalar Lean functions. The hardware worker never
// enters the boxed Lean runtime; it only transports bytes and kernel metadata.
extern "C" {
    fn tv_tee_device_step(state: u32, major: u64, minor: u64, byte: u32) -> u32;
    fn tv_tee_device_major(state: u32, major: u64, byte: u32) -> u64;
    fn tv_tee_device_minor(state: u32, minor: u64, byte: u32) -> u64;
    fn tv_tee_device_finish(state: u32) -> u8;
    fn tv_tee_device_matches(major: u64, minor: u64, raw: u64, character: u8) -> u8;
    fn tv_tee_receive_step(state: u32, fd: u64, request: u64, byte: u32) -> u32;
    fn tv_tee_receive_fd(state: u32, fd: u64, byte: u32) -> u64;
    fn tv_tee_receive_request(state: u32, request: u64, byte: u32) -> u64;
    fn tv_tee_receive_finish(state: u32, fd: u64, request: u64) -> u64;
}

fn device_identity(bytes: &[u8]) -> Option<(u64, u64)> {
    let (mut state, mut major, mut minor) = (0, 0, 0);
    for &byte in bytes {
        let next = unsafe { tv_tee_device_step(state, major, minor, byte.into()) };
        major = unsafe { tv_tee_device_major(state, major, byte.into()) };
        minor = unsafe { tv_tee_device_minor(state, minor, byte.into()) };
        state = next;
    }
    (unsafe { tv_tee_device_finish(state) } != 0).then_some((major, minor))
}

fn bad(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::PermissionDenied, message)
}

pub fn restrict() -> io::Result<()> {
    for name in ["tee0", "teepriv0"] {
        let path = format!("/dev/{name}");
        let meta = fs::symlink_metadata(&path)?;
        let device = fs::read(format!("/sys/class/tee/{name}/dev"))?;
        let (major, minor) = device_identity(&device).ok_or_else(|| bad("invalid TEE device identity"))?;
        if unsafe { tv_tee_device_matches(major, minor, meta.rdev(), meta.file_type().is_char_device().into()) } == 0 {
            return Err(bad("TEE device does not match sysfs"));
        }
        chown(&path, Some(0), Some(0))?;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
        let meta = fs::symlink_metadata(&path)?;
        if meta.uid() != 0 || meta.gid() != 0 || meta.mode() & 0o777 != 0o600 {
            return Err(bad("TEE device permissions did not stick"));
        }
    }
    for path in ["/data/tee", "/data/vendor", "/mnt/vendor/tmp"] {
        fs::create_dir_all(path)?;
        let meta = fs::symlink_metadata(path)?;
        if !meta.is_dir() || meta.file_type().is_symlink() {
            return Err(bad("TEE storage is not a private directory"));
        }
        chown(path, Some(0), Some(0))?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

fn receive_fd(bytes: &[u8]) -> Option<u32> {
    let (mut state, mut fd, mut request) = (0, 0, 0);
    for &byte in bytes {
        let next = unsafe { tv_tee_receive_step(state, fd, request, byte.into()) };
        fd = unsafe { tv_tee_receive_fd(state, fd, byte.into()) };
        request = unsafe { tv_tee_receive_request(state, request, byte.into()) };
        state = next;
    }
    let result = unsafe { tv_tee_receive_finish(state, fd, request) };
    if result == 0 { None } else { Some((result - 1) as u32) }
}

pub fn waiting(pid: u32) -> bool {
    let Ok(tasks) = fs::read_dir(format!("/proc/{pid}/task")) else { return false; };
    tasks.flatten().any(|task| {
        let Ok(call) = fs::read(task.path().join("syscall")) else { return false; };
        let Some(fd) = receive_fd(&call) else { return false; };
        fs::read_link(format!("/proc/{pid}/fd/{fd}"))
            .is_ok_and(|path| path == Path::new("/dev/teepriv0"))
    })
}

#[cfg(test)]
#[path = "../../../test/unit/pid1/tee-boundary.rs"]
mod tests;
