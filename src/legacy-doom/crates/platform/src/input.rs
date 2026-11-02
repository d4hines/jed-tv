//! evdev key input: the remote-control half of the platform API.
//! On QEMU this is virtio-keyboard; on the TV, "MStar Smart TV IR Receiver".

use crate::sys::{ioc, ioctl, IOC_READ};
use libc::c_ulong;
use std::io::{self, Read};
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct RawEvent {
    sec: c_ulong,
    usec: c_ulong,
    type_: u16,
    code: u16,
    value: i32,
}

const EV_KEY: u16 = 1;
/// EVIOCGNAME with the 128-byte buffer `device_name` uses.
pub const EVIOCGNAME_128: u32 = ioc(IOC_READ, b'E', 0x06, 128);

/// A key press (value 1), repeat (2) or release (0), with the Linux keycode.
#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub code: u16,
    pub value: i32,
}

pub struct Device {
    pub path: String,
    pub name: String,
    file: std::fs::File,
}

fn device_name(f: &std::fs::File) -> String {
    let mut buf = [0u8; 128];
    let req = EVIOCGNAME_128;
    match unsafe { ioctl(f.as_raw_fd(), req, buf.as_mut_ptr()) } {
        Ok(n) => String::from_utf8_lossy(&buf[..(n as usize).saturating_sub(1)]).into_owned(),
        Err(_) => String::from("?"),
    }
}

/// Open every /dev/input/event* non-blocking.
pub fn open_all() -> Vec<Device> {
    let mut out = Vec::new();
    let Ok(dir) = std::fs::read_dir("/dev/input") else { return out };
    let mut paths: Vec<_> = dir.flatten().map(|e| e.path())
        .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("event")))
        .collect();
    paths.sort();
    for p in paths {
        if let Ok(file) = std::fs::OpenOptions::new().read(true)
            .custom_flags(libc::O_NONBLOCK).open(&p)
        {
            let name = device_name(&file);
            out.push(Device { path: p.display().to_string(), name, file });
        }
    }
    out
}

impl Device {
    /// Drain pending key events without blocking.
    pub fn poll_keys(&mut self) -> io::Result<Vec<Key>> {
        let mut keys = Vec::new();
        let sz = std::mem::size_of::<RawEvent>();
        let mut buf = vec![0u8; sz * 32];
        loop {
            match self.file.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    for chunk in buf[..n].chunks_exact(sz) {
                        let ev: RawEvent = unsafe { std::ptr::read_unaligned(chunk.as_ptr() as *const _) };
                        if ev.type_ == EV_KEY {
                            keys.push(Key { code: ev.code, value: ev.value });
                        }
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
        Ok(keys)
    }
}
