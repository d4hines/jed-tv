//! Coordinator-only live diagnostic. Creates no PCM, tone, module init or route.
use mi_bootstrap::speaker::{Io, Output, OutputPath};
use std::{
    fs::OpenOptions,
    io,
    os::fd::AsRawFd,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
#[repr(C)]
struct Envelope {
    size: u16,
    pad: [u8; 6],
    address: u64,
}
extern "C" {
    fn ioctl(fd: i32, request: usize, ...) -> i32;
    fn geteuid() -> u32;
    fn signal(sig: i32, handler: usize) -> usize;
}
static STOP: AtomicBool = AtomicBool::new(false);
extern "C" fn stop(_: i32) {
    STOP.store(true, Ordering::Relaxed);
}
struct ProbeIo(std::fs::File);
impl Io for ProbeIo {
    fn authorize(&mut self) -> io::Result<()> {
        if unsafe { geteuid() } != 0
            || !std::fs::read_to_string("/proc/cmdline")?
                .split_whitespace()
                .any(|s| s == mi_bootstrap::SERIAL)
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "requires root on original G06",
            ));
        }
        Ok(())
    }
    fn call(&mut self, r: u32, b: &mut [u8]) -> io::Result<i32> {
        self.authorize()?;
        let mut e = Envelope {
            size: b.len() as u16,
            pad: [0; 6],
            address: b.as_mut_ptr() as u64,
        };
        let n = unsafe { ioctl(self.0.as_raw_fd(), r as usize, &mut e as *mut Envelope) };
        if n < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(n)
        }
    }
}
fn run() -> io::Result<()> {
    if unsafe { geteuid() } != 0 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "root required",
        ));
    }
    let cmd = std::fs::read_to_string("/proc/cmdline")?;
    if !cmd.split_whitespace().any(|s| s == mi_bootstrap::SERIAL) {
        return Err(io::Error::other("wrong TV"));
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/mik/aout")
        .or_else(|_| {
            OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/mik!aout")
        })?;
    unsafe {
        if signal(15, stop as *const () as usize) == usize::MAX
            || signal(2, stop as *const () as usize) == usize::MAX
        {
            return Err(io::Error::last_os_error());
        }
    }
    let mut owner = Output::create(ProbeIo(file), OutputPath::MainSpeaker)?;
    println!("READY speaker handle={:#010x}; holding owner; SIGTERM requests close only after all routes detach",owner.handle());
    loop {
        std::thread::sleep(Duration::from_secs(1));
        if STOP.load(Ordering::Relaxed) {
            match owner.close() {
                Ok(()) => {
                    println!("speaker closed");
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("speaker retained: {e}");
                    STOP.store(false, Ordering::Relaxed);
                }
            }
        }
    }
}
fn main() {
    if let Err(e) = run() {
        eprintln!("speaker-owner-probe: {e}");
        std::process::exit(1)
    }
}
