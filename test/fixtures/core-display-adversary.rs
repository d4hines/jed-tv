//! Uncooperative confined app for the offline Core display-ownership check.
use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom, Write},
    os::fd::AsRawFd,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
extern "C" {
    fn getuid() -> u32;
    fn getgid() -> u32;
    fn signal(number: i32, handler: extern "C" fn(i32)) -> usize;
    fn fcntl(fd: i32, command: i32, ...) -> i32;
    fn write(fd: i32, bytes: *const u8, count: usize) -> isize;
}
static TERM: AtomicBool = AtomicBool::new(false);
extern "C" fn ignore_term(_: i32) {
    TERM.store(true, Ordering::Relaxed);
    let message = b"hostile: IGNORED_TERM\n";
    unsafe {
        write(1, message.as_ptr(), message.len());
    }
}
fn denied(path: &str, write: bool) {
    let error = OpenOptions::new()
        .read(!write)
        .write(write)
        .open(path)
        .unwrap_err();
    assert_eq!(
        error.kind(),
        std::io::ErrorKind::PermissionDenied,
        "{path}: {error}"
    );
}
fn header(file: &mut File, words: [u32; 6]) {
    file.seek(SeekFrom::Start(0)).unwrap();
    for word in words {
        file.write_all(&word.to_le_bytes()).unwrap();
    }
}
fn main() {
    assert_eq!(unsafe { getuid() }, 10001);
    assert_eq!(unsafe { getgid() }, 4000);
    for path in ["/run/tv-core/menu.frames", "/dev/fb0"] {
        denied(path, false);
        denied(path, true);
    }
    for fd in 4..256 {
        assert_eq!(unsafe { fcntl(fd, 1) }, -1, "inherited fd {fd}");
    }
    println!("hostile: PRIVATE_MENU_AND_RAW_FB_DENIED uid=10001 gid=4000 inherited-fds=0");
    unsafe {
        signal(15, ignore_term);
    }
    let path = std::env::var("TV_FB_PATH").expect("Core must provide app frame path");
    assert_eq!(path, "/tmp/app.frames");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();
    let size = 4096 + 1280 * 720 * 4 * 2;
    file.set_len(size).unwrap();
    let pixel = 0xffe010f0u32.to_le_bytes();
    let pixels = pixel.repeat(1280 * 720);
    file.seek(SeekFrom::Start(4096)).unwrap();
    file.write_all(&pixels).unwrap();
    header(&mut file, [0x32465654, 1280, 720, 5120, 0, 1]);
    println!("hostile: VALID_MAGENTA_FRAME fd={}", file.as_raw_fd());
    std::thread::sleep(Duration::from_secs(5));
    println!("hostile: CORRUPTION_STARTED");
    let started = Instant::now();
    let mut sequence = 2u32;
    let mut reported_term_write = false;
    loop {
        file.set_len(0).unwrap();
        std::thread::sleep(Duration::from_millis(1));
        file.set_len(size).unwrap();
        header(&mut file, [0x32465654, u32::MAX, 0, u32::MAX, 3, sequence]);
        std::thread::sleep(Duration::from_millis(1));
        header(&mut file, [0x32465654, 1280, 720, 5120, 0, sequence]);
        sequence = sequence.wrapping_add(1).max(2);
        if TERM.load(Ordering::Relaxed) && !reported_term_write {
            println!("hostile: WRITE_AFTER_TERM seq={sequence}");
            reported_term_write = true;
        }
        if sequence % 128 == 0 {
            println!(
                "hostile: MUTATING seq={sequence} elapsed-ms={}",
                started.elapsed().as_millis()
            );
        }
    }
}
