use crate::{validate_identity, Packet, Transport, PACKET_BYTES};
use std::{
    fs::{File, OpenOptions},
    io,
    os::{
        fd::AsRawFd,
        unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt},
    },
    sync::atomic::{fence, Ordering},
};
// This module is ARM-only; catch accidental use of generic-x86 flag values.
const _: () = assert!(libc::O_NOFOLLOW == 0x8000);
const DEVICE: &str = "/dev/teepriv0";
const LOCK: &str = "/tmp/g06-tee-provider.lock";
const VERSION: u32 = 0x800ca400;
const RECV: u32 = 0x8010a406;
const SEND: u32 = 0x8010a407;
#[repr(C)]
struct Envelope {
    pointer: u64,
    length: u64,
}
#[repr(C)]
#[derive(Default, Debug)]
pub struct Version {
    pub implementation: u32,
    pub implementation_caps: u32,
    pub generic_caps: u32,
}
extern "C" {
    fn geteuid() -> u32;
    fn flock(fd: i32, operation: i32) -> i32;
    fn ioctl(fd: i32, request: usize, ...) -> i32;
}
fn error(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::PermissionDenied, s)
}
fn major(d: u64) -> u64 {
    ((d >> 8) & 0xfff) | ((d >> 32) & 0xfffff000)
}
fn minor(d: u64) -> u64 {
    (d & 0xff) | ((d >> 12) & 0xffffff00)
}
pub fn check_identity() -> io::Result<()> {
    validate_identity(
        unsafe { geteuid() },
        &std::fs::read_to_string("/proc/cmdline")?,
        &std::fs::read_to_string("/proc/sys/kernel/osrelease")?,
    )
}

pub struct Device {
    file: File,
    _lock: File,
    pub version: Version,
}
impl Device {
    pub fn open() -> io::Result<Self> {
        check_identity()?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(LOCK)?;
        let meta = lock.metadata()?;
        if !meta.is_file() || meta.uid() != 0 || meta.mode() & 0o022 != 0 {
            return Err(error("unsafe provider lock file"));
        }
        if unsafe { flock(lock.as_raw_fd(), 2 | 4) } != 0 {
            return Err(io::Error::last_os_error());
        }
        // One open only: optee_open atomically rejects an existing supplicant
        // context with EBUSY. Duplicates below refer to this same context.
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(DEVICE)?;
        let meta = file.metadata()?;
        let expected = std::fs::read_to_string("/sys/class/tee/teepriv0/dev")?;
        if !meta.file_type().is_char_device()
            || expected.trim() != format!("{}:{}", major(meta.rdev()), minor(meta.rdev()))
        {
            return Err(error("TEE node does not match teepriv0 sysfs identity"));
        }
        let mut version = Version::default();
        let rc = unsafe {
            ioctl(
                file.as_raw_fd(),
                VERSION as usize,
                &mut version as *mut Version,
            )
        };
        if rc < 0 {
            return Err(io::Error::last_os_error());
        }
        if rc != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected positive version result",
            ));
        }
        if version.implementation != 1
            || version.implementation_caps & 1 == 0
            || version.generic_caps & 3 != 3
        {
            return Err(error("requires privileged GlobalPlatform OP-TEE device"));
        }
        Ok(Self {
            file,
            _lock: lock,
            version,
        })
    }
    pub fn allocator_file(&self) -> io::Result<File> {
        self.file.try_clone()
    }
    fn call(&self, request: u32, packet: *mut Packet, bytes: usize) -> io::Result<()> {
        let mut arg = Envelope {
            pointer: packet as u64,
            length: bytes as u64,
        };
        let rc = unsafe {
            ioctl(
                self.file.as_raw_fd(),
                request as usize,
                &mut arg as *mut Envelope,
            )
        };
        if rc < 0 {
            return Err(io::Error::last_os_error());
        }
        if rc != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "TEE ioctl returned unexpected positive result",
            ));
        }
        Ok(())
    }
}
impl Transport for Device {
    fn receive(&mut self, packet: &mut Packet) -> io::Result<()> {
        self.call(RECV, packet, PACKET_BYTES)?;
        fence(Ordering::Acquire);
        Ok(())
    }
    fn send(&mut self, packet: &Packet) -> io::Result<()> {
        let size = packet.byte_len()?;
        // Use a private mutable copy because the syscall takes a writable void*;
        // never cast an immutable live Rust packet into mutable aliased memory.
        let mut reply = packet.clone();
        fence(Ordering::Release);
        self.call(SEND, &mut reply, size)
    }
}
