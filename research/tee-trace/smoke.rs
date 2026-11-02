//! Harmless ptrace workload: fake TEE ioctl numbers on /dev/null, never /dev/tee*.
use std::{
    ffi::{c_char, CString},
    fs::File,
    os::fd::AsRawFd,
    sync::atomic::{AtomicUsize, Ordering},
};
extern "C" {
    fn ioctl(fd: i32, request: usize, ...) -> i32;
    fn signal(number: i32, handler: usize) -> usize;
    fn raise(number: i32) -> i32;
    fn execv(path: *const c_char, argv: *const *const c_char) -> i32;
}
static SIGNALS: AtomicUsize = AtomicUsize::new(0);
extern "C" fn caught(_: i32) {
    SIGNALS.fetch_add(1, Ordering::SeqCst);
}
fn requests() {
    let file = File::open("/dev/null").unwrap();
    let mut payload = [42 | (1u64 << 32), 1, 0xfeed, 123, 456];
    #[repr(C)]
    struct Envelope {
        address: u64,
        length: u64,
    }
    let envelope = Envelope {
        address: payload.as_mut_ptr() as u64,
        length: 40,
    };
    for request in [0x8010a406usize, 0x8010a407] {
        let ret = unsafe { ioctl(file.as_raw_fd(), request, &envelope as *const _) };
        assert_eq!(ret, -1);
        assert_eq!(std::io::Error::last_os_error().raw_os_error(), Some(25));
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.iter().any(|a| a == "--exec-leaf") {
        requests();
        return;
    }
    if args.iter().any(|a| a == "--thread-exec") {
        requests();
        std::thread::spawn(|| {
            use std::os::unix::ffi::OsStrExt;
            let path =
                CString::new(std::env::current_exe().unwrap().as_os_str().as_bytes()).unwrap();
            let leaf = CString::new("--exec-leaf").unwrap();
            let argv = [path.as_ptr(), leaf.as_ptr(), std::ptr::null()];
            unsafe {
                execv(path.as_ptr(), argv.as_ptr());
            }
            panic!("nonleader exec failed: {}", std::io::Error::last_os_error());
        })
        .join()
        .unwrap();
        panic!("nonleader exec unexpectedly returned");
    }
    assert_ne!(
        unsafe { signal(10, caught as *const () as usize) },
        usize::MAX
    );
    assert_eq!(unsafe { raise(10) }, 0);
    assert_eq!(
        SIGNALS.load(Ordering::SeqCst),
        1,
        "SIGUSR1 was not delivered"
    );
    let workers: Vec<_> = (0..3).map(|_| std::thread::spawn(requests)).collect();
    for worker in workers {
        worker.join().unwrap()
    }
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--thread-exec")
        .status()
        .unwrap();
    assert!(status.success());
    println!("tee-trace harmless smoke PASS: 10 failed /dev/null ioctls, caught SIGUSR1,3 threads, child and nonleader exec");
}
