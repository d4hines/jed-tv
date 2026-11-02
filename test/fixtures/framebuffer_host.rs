//! Host harness for the real framebuffer module, without Linux-only services.
#[path = "../../src/platform/src/fb.rs"]
pub mod fb;

mod sys {
    pub const IOC_WRITE: u32 = 1;
    pub const fn ioc(direction: u32, kind: u8, number: u8, size: usize) -> u32 {
        (direction << 30) | ((size as u32) << 16) | ((kind as u32) << 8) | number as u32
    }
    pub unsafe fn ioctl<T>(fd: i32, request: u32, arg: *mut T) -> std::io::Result<i32> {
        let result = libc::ioctl(fd, request as _, arg);
        if result < 0 { Err(std::io::Error::last_os_error()) } else { Ok(result) }
    }
    pub fn monotonic() -> std::time::Duration {
        let mut time = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut time); }
        std::time::Duration::new(time.tv_sec as u64, time.tv_nsec as u32)
    }
}
