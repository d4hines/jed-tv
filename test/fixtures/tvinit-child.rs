//! Hostile launch-boundary probe, then a deliberately uncooperative app.
extern "C" {
    fn signal(number: i32, handler: extern "C" fn(i32)) -> usize;
    fn fork() -> i32;
    fn vfork() -> i32;
    fn getuid() -> u32;
    fn geteuid() -> u32;
    fn execve(path: *const core::ffi::c_char, argv: *const *const core::ffi::c_char, env: *const *const core::ffi::c_char) -> i32;

    fn fcntl(fd: i32, command: i32, ...) -> i32;
    fn syscall(number: i32, ...) -> isize;
    fn write(fd: i32, data: *const u8, count: usize) -> isize;
}
unsafe fn execveat(fd: i32, path: *const core::ffi::c_char, argv: *const *const core::ffi::c_char, env: *const *const core::ffi::c_char, flags: i32) -> i32 {
    #[cfg(target_arch="arm")] let nr = 387;
    #[cfg(target_arch="aarch64")] let nr = 281;
    unsafe { syscall(nr, fd, path, argv, env, flags) as i32 }
}
extern "C" fn ignored(_: i32) {
    let message = b"fixture: ignored SIGTERM\n";
    unsafe { write(1, message.as_ptr(), message.len()); }
}
fn rejected(result: i32, expected_errno: i32) {
    let actual_errno = std::io::Error::last_os_error().raw_os_error();
    assert_eq!(result, -1);
    assert_eq!(actual_errno, Some(expected_errno));
}
fn main() {
    unsafe {
        assert_eq!(getuid(), 10001);
        assert_eq!(geteuid(), 10001);
        rejected(fork(), 1);
        rejected(vfork(), 1);
        let argv = [c"/bin/slides".as_ptr(), std::ptr::null()];
        let env = [std::ptr::null()];
        rejected(execve(argv[0], argv.as_ptr(), env.as_ptr()), 1);
        rejected(execveat(3, c"".as_ptr(), argv.as_ptr(), env.as_ptr(), 0x1000), 1);
        // Reserved launch descriptor is closed and impossible to recreate.
        rejected(fcntl(1023, 1), 9);
        rejected(fcntl(3, 1030, 1023), 22);
        rejected(execveat(1023, c"".as_ptr(), argv.as_ptr(), env.as_ptr(), 0x1000), 9);
        // Absolute paths ignore dirfd: noexec mounts must block this bypass.
        rejected(execveat(1023, c"/bin/slides".as_ptr(), argv.as_ptr(), env.as_ptr(), 0), 13);
        rejected(execveat(1023, c"/proc/self/exe".as_ptr(), argv.as_ptr(), env.as_ptr(), 0), 2);
        #[cfg(target_arch="arm")] { rejected(syscall(120, 17usize, 0usize) as i32, 1); }
        #[cfg(target_arch="aarch64")] { rejected(syscall(220, 17usize, 0usize) as i32, 1); }
        rejected(syscall(435, 0usize, 0usize) as i32, 38);
        signal(15, ignored);
    }
    assert_eq!(std::fs::File::open("/dev/input/event0").unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    if let Ok(fd) = std::env::var("TEE_INHERITED_FD") {
        let fd: i32 = fd.parse().unwrap();
        rejected(unsafe { fcntl(fd, 1) }, 9);
        for path in ["/dev/tee0", "/dev/teepriv0"] {
            assert_eq!(std::fs::File::open(path).unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
        }
        for path in ["/data/tee", "/data/vendor", "/mnt/vendor/tmp"] {
            assert_eq!(std::fs::read_dir(path).unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
        }
        println!("fixture: TEE boundary PASS (nodes, private storage, inherited descriptor)");
    }
    assert!(!std::path::Path::new("/proc/self/exe").exists());
    assert_eq!(std::fs::File::open("/data/apps/10003/private").unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    std::thread::spawn(|| println!("fixture: thread creation allowed")).join().unwrap();
    println!("fixture: confinement PASS (uid, fork, vfork, clone, clone3, exec, reserved fd, raw input)");
    println!("fixture: uncooperative app running as PID {}", std::process::id());
    loop { std::thread::sleep(std::time::Duration::from_millis(50)); }
}
