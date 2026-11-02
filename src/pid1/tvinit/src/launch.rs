//! Transitional Rust adapter for the Lean table and single native mechanism.
use std::{ffi::CString, io, os::fd::{AsRawFd, FromRawFd, OwnedFd}, os::unix::fs::PermissionsExt};
extern "C" {
    fn tv_app_uid(app: u32) -> u32;
    fn tv_app_gid(app: u32) -> u32;
    fn tv_linux_launch(path: *const libc::c_char, argv: *const *const libc::c_char,
        env: *const *const libc::c_char, app: u32, grants: u32, input: i32, requests: i32, workdir: *const libc::c_char) -> i32;
}
fn access(path: &str, uid: u32, gid: u32, mode: u32) -> io::Result<()> {
    let name = CString::new(path)?;
    if unsafe { libc::chown(name.as_ptr(), uid, gid) } != 0 { return Err(io::Error::last_os_error()); }
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
}
pub fn prepare(app: u32) -> io::Result<CString> {
    let (uid, gid) = unsafe { (tv_app_uid(app), tv_app_gid(app)) };
    if uid == 0 { return Err(io::Error::other("unknown application")); }
    let workdir = format!("/data/apps/{uid}");
    for id in [1, 2, 3, 5, 6, 7, 8] {
        let owner = unsafe { tv_app_uid(id) };
        let private = format!("/data/apps/{owner}");
        std::fs::create_dir_all(&private)?;
        access(&private, owner, owner, 0o700)?;
    }
    // Shared frames are the TV's transitional display backend. Only the
    // launcher grants write access; volume/settings remain root-owned.
    if let Ok(path) = std::env::var("TV_FB_PATH") {
        let _ = std::fs::OpenOptions::new().create(true).truncate(false).write(true).open(&path)?;
        access(&path, 0, gid, 0o660)?;
    }
    // Direct display/audio access is an explicit transitional capability.
    let shared = std::env::var_os("TV_FB_PATH").is_some();
    let (device_group, device_mode) = if shared { (0, 0o600) } else { (gid, 0o660) };
    if std::path::Path::new("/dev/fb0").exists() { access("/dev/fb0", 0, device_group, device_mode)?; }
    if let Ok(devices) = std::fs::read_dir("/dev/snd") {
        for device in devices.flatten() {
            access(&device.path().to_string_lossy(), 0, device_group, device_mode)?;
        }
    }
    // Remote input is exclusively PID1-owned, including on kernels whose
    // default devtmpfs input-node permissions might otherwise be permissive.
    if let Ok(devices) = std::fs::read_dir("/dev/input") {
        for device in devices.flatten() {
            access(&device.path().to_string_lossy(), 0, 0, 0o600)?;
        }
    }
    CString::new(workdir).map_err(Into::into)
}
pub fn socket_pair() -> io::Result<(OwnedFd, OwnedFd)> {
    let mut fds = [-1; 2];
    if unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_DGRAM | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC, 0, fds.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])) })
}
pub fn start(path: &CString, argv: &[*const libc::c_char], env: &[*const libc::c_char], app: u32, grants: u32, input: &OwnedFd, requests: &OwnedFd, workdir: &CString) -> io::Result<i32> {
    let pid = unsafe { tv_linux_launch(path.as_ptr(), argv.as_ptr(), env.as_ptr(), app, grants, input.as_raw_fd(), requests.as_raw_fd(), workdir.as_ptr()) };
    if pid < 0 { Err(io::Error::from_raw_os_error(-pid)) } else { Ok(pid) }
}
