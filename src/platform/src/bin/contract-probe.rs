//! Linux-only ABI and cross-process display ownership check.
use platform::{app_api::*, fb};
use std::{io::BufRead, os::fd::AsRawFd, process::{Command, Stdio}, time::{Duration, Instant}};
fn main() {
    if std::env::args().any(|a| a == "reader") { reader_holds_slot(); return; }
    if std::env::args().any(|a| a == "audio") {
        unsafe {
            let mut audio = std::ptr::null_mut();
            assert_eq!(tv_audio_open(48000, 2, &mut audio), 0);
            assert_eq!(tv_audio_write(audio, vec![0i16; 9600].as_ptr(), 9600), 0);
            let mut queued = 0;
            assert_eq!(tv_audio_queued(audio, &mut queued), 0);
            assert!(queued >= 0);
            assert_eq!(tv_audio_reset(audio), 0);
            tv_audio_close(audio);
            let mut input = std::ptr::null_mut();
            assert_eq!(tv_input_open(&mut input), 0);
            tv_input_close(input);
        }
        println!("APP-API AUDIO PASS: silent PCM, queue, reset, input open/close");
        return;
    }
    let pid1 = platform::sys::is_pid1();
    if pid1 { platform::sys::init_mounts(); }
    std::fs::create_dir_all("/tmp").unwrap();
    display_ownership_and_c_abi();
    println!("APP-API PASS: C ABI, cancellation and cross-process buffer ownership");
    if pid1 { loop { std::thread::sleep(Duration::from_secs(60)); } }
}
    fn reader_holds_slot() {
        let Ok(path) = std::env::var("TV_API_LOCK_TEST") else { return };
        let file = std::fs::File::open(path).unwrap();
        let mut lock: libc::flock = unsafe { std::mem::zeroed() };
        lock.l_type = libc::F_RDLCK as _;
        lock.l_whence = libc::SEEK_SET as _;
        lock.l_start = 4096 + 1280 * 720 * 4;
        lock.l_len = 1280 * 720 * 4;
        assert_eq!(unsafe { libc::fcntl(file.as_raw_fd(), libc::F_SETLK, &lock) }, 0);
        println!("READER_LOCKED");
        std::thread::sleep(Duration::from_millis(250));
    }

    fn display_ownership_and_c_abi() {
        let path = std::env::temp_dir().join(format!("tv-api-{}", std::process::id()));
        std::env::set_var("TV_FB_PATH", &path);
        let mut display = std::ptr::null_mut();
        unsafe { assert_eq!(tv_display_open(&mut display), 0); }
        let mut child = Command::new(std::env::current_exe().unwrap())
            .arg("reader")
            .env("TV_API_LOCK_TEST", &path).stdout(Stdio::piped()).spawn().unwrap();
        let mut lines = std::io::BufReader::new(child.stdout.take().unwrap()).lines();
        assert!(lines.any(|l| l.unwrap().contains("READER_LOCKED")));
        let mut frame: Frame = unsafe { std::mem::zeroed() };
        let start = Instant::now();
        unsafe {
            assert_eq!(tv_app_api_version(), 0x10000);
            assert_eq!(tv_display_acquire(display, &mut frame), 0);
            assert!(start.elapsed() >= Duration::from_millis(100), "producer reused a reader-owned slot");
            assert_eq!(tv_display_acquire(display, &mut frame), -libc::EBUSY);
            assert_eq!((frame.width, frame.height, frame.stride_bytes), (1280,720,5120));
            std::slice::from_raw_parts_mut(frame.pixels, 1280*720).fill(0xff443322);
            assert_eq!(tv_display_present(display), 0);
            assert_eq!(tv_display_present(display), -libc::EINVAL);
            assert_eq!(tv_display_acquire(display, &mut frame), 0);
            frame.pixels.write(0xff000000);
            tv_display_cancel(display);
            let mut reader = fb::Framebuffer::shared_file(path.to_str().unwrap(),1280,720).unwrap();
            let visible = reader.read_presented().unwrap();
            assert_eq!(visible.sequence, 1);
            assert_eq!(visible.pixels()[0], 0xff443322, "cancel published an unfinished frame");
            drop(visible);
            tv_display_close(display);
        }
        assert!(child.wait().unwrap().success());
        std::fs::remove_file(path).unwrap();
        std::env::remove_var("TV_FB_PATH");
    }
