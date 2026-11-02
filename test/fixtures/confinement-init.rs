//! Offline QEMU-only PID1 harness for the same native launch gate as tvinit.
//! No display, sound, network or physical TV access is needed.
use std::{ffi::{c_char, c_void}, os::{fd::AsRawFd, unix::fs::PermissionsExt}, time::Duration};
extern "C" {
    fn mount(source: *const c_char, target: *const c_char, kind: *const c_char, flags: usize, data: *const c_void) -> i32;
    fn mknod(path: *const c_char, mode: u32, device: u64) -> i32;
    fn chown(path: *const c_char, uid: u32, gid: u32) -> i32;
    fn socketpair(domain: i32, kind: i32, protocol: i32, pair: *mut i32) -> i32;
    fn close(fd: i32) -> i32;
    fn fcntl(fd: i32, command: i32, ...) -> i32;
    fn kill(pid: i32, signal: i32) -> i32;
    fn waitpid(pid: i32, status: *mut i32, flags: i32) -> i32;
    fn tv_linux_launch(path: *const c_char, argv: *const *const c_char, env: *const *const c_char,
        app: u32, grants: u32, input: i32, requests: i32, workdir: *const c_char) -> i32;
}
fn main() {
    assert_eq!(std::process::id(),1);
    unsafe {
        mount(c"devtmpfs".as_ptr(),c"/dev".as_ptr(),c"devtmpfs".as_ptr(),0,std::ptr::null());
        assert_eq!(mount(c"proc".as_ptr(),c"/proc".as_ptr(),c"proc".as_ptr(),0,std::ptr::null()),0);
    }
    std::fs::create_dir_all("/dev/input").unwrap();
    std::fs::File::create("/dev/input/event0").unwrap();
    std::fs::set_permissions("/dev/input/event0",std::fs::Permissions::from_mode(0o600)).unwrap();
    // Fake TEE character nodes exercise Linux access control, not TEE hardware.
    for name in [c"/dev/tee0", c"/dev/teepriv0"] {
        assert_eq!(unsafe { mknod(name.as_ptr(), 0o020600, 0x103) }, 0);
    }
    for path in ["/data/tee", "/data/vendor", "/mnt/vendor/tmp"] {
        std::fs::create_dir_all(path).unwrap();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let tee = std::fs::File::open("/dev/tee0").unwrap();
    let tee_fd = unsafe { fcntl(tee.as_raw_fd(), 0, 100) }; // F_DUPFD deliberately clears CLOEXEC.
    assert!(tee_fd >= 100);
    let tee_env = std::ffi::CString::new(format!("TEE_INHERITED_FD={tee_fd}")).unwrap();
    for uid in [10001,10003] {
        let path=format!("/data/apps/{uid}");std::fs::create_dir_all(&path).unwrap();
        std::fs::set_permissions(&path,std::fs::Permissions::from_mode(0o700)).unwrap();
        let path=std::ffi::CString::new(path).unwrap();
        assert_eq!(unsafe{chown(path.as_ptr(),uid,uid)},0);
    }
    let mut pair=[-1;2];
    assert_eq!(unsafe{socketpair(1,2|0x80000|0x800,0,pair.as_mut_ptr())},0);
    let mut requests=[-1;2];
    assert_eq!(unsafe{socketpair(1,2|0x80000|0x800,0,requests.as_mut_ptr())},0);
    let argv=[c"/bin/ytplay".as_ptr(),c"tvapp.remote=3".as_ptr(),std::ptr::null()];
    let env=[c"PATH=/bin".as_ptr(),tee_env.as_ptr(),std::ptr::null()];
    let pid=unsafe{tv_linux_launch(argv[0],argv.as_ptr(),env.as_ptr(),1,1,pair[1],requests[1],c"/data/apps/10001".as_ptr())};
    assert!(pid>0,"launch returned {pid}");
    unsafe{close(pair[1]);close(requests[1]);close(tee_fd);}
    std::thread::sleep(Duration::from_secs(2));
    let mut status=0;
    assert_eq!(unsafe{waitpid(pid,&mut status,1)},0,"probe exited early: {status}");
    assert_eq!(unsafe{kill(pid,15)},0);
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(unsafe{waitpid(pid,&mut status,1)},0,"probe did not ignore TERM");
    assert_eq!(unsafe{kill(pid,9)},0);
    assert_eq!(unsafe{waitpid(pid,&mut status,0)},pid);
    assert_eq!(status&127,9);
    println!("G06-CONFINEMENT PASS: native launch, UID isolation, seccomp and TERM/KILL on this kernel");
    loop { std::thread::park(); }
}
