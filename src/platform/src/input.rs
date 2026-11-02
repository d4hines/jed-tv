//! evdev key input: the remote-control half of the platform API.
//! On the TV this is "MStar Smart TV IR Receiver"; on QEMU, virtio-keyboard
//! plus the virtual IR receiver (`remote.udp=PORT`) fed by the on-screen remote.

use crate::sys::{ioc, ioctl, IOC_READ};
use libc::c_ulong;
use std::io::{self, Read};
use std::os::fd::{AsRawFd, FromRawFd};
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

/// How a device's bytes are laid out.
#[derive(Clone, Copy)]
enum Wire {
    /// Linux `struct input_event`s: evdev nodes and PID 1's forwarding socket.
    Evdev,
    /// The virtual IR receiver: 8-byte little-endian records
    /// `u16 type, u16 code, i32 value`, any number per datagram.
    Datagram,
}

pub struct Device {
    pub path: String,
    pub name: String,
    file: std::fs::File,
    wire: Wire,
}

/// Guest UDP port of the virtual IR receiver, from `remote.udp=PORT`.
static VIRTUAL_IR: std::sync::OnceLock<std::net::UdpSocket> = std::sync::OnceLock::new();

/// QEMU stand-in for the TV's IR receiver: the on-screen remote
/// (demo/remote.py) sends the TCL keycodes to this UDP port. Bound once per
/// process; every caller of `open_all` gets a handle to the same socket.
fn virtual_ir(port: u16) -> io::Result<Device> {
    let sock = match VIRTUAL_IR.get() {
        Some(sock) => sock,
        None => {
            let sock = std::net::UdpSocket::bind(("0.0.0.0", port))?;
            sock.set_nonblocking(true)?;
            VIRTUAL_IR.get_or_init(|| sock)
        }
    };
    let file = std::fs::File::from(std::os::fd::OwnedFd::from(sock.try_clone()?));
    Ok(Device { path: format!("udp:{port}"), name: "virtual IR receiver".into(), file, wire: Wire::Datagram })
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
    // PID 1 supplies a launch-bound Unix datagram socket. Never fall back to
    // opening physical input devices if that connection is unavailable.
    if let Some(fd) = crate::sys::cmdline_arg("tvapp.remote").and_then(|s| s.parse::<i32>().ok()) {
        let copy = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 4) };
        if copy >= 0 {
            out.push(Device { path: "pid1:remote".into(), name: "PID1 remote service".into(),
                file: unsafe { std::fs::File::from_raw_fd(copy) }, wire: Wire::Evdev });
        }
        return out;
    }
    // PID 1 in QEMU: the on-screen remote's keys arrive here. Apps never
    // reach this branch; they get the keys forwarded like any other.
    if let Some(port) = crate::sys::cmdline_arg("remote.udp").and_then(|s| s.parse::<u16>().ok()) {
        match virtual_ir(port) {
            Ok(d) => out.push(d),
            Err(e) => eprintln!("input: virtual IR receiver on udp:{port}: {e}"),
        }
    }
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
            // A launcher hosted under Android must keep the stock UI from
            // acting on the same remote presses. The grab follows this fd's
            // lifetime; forwarded app input above never opens/grabs hardware.
            if std::env::var_os("TV_INPUT_EXCLUSIVE").is_some() {
                let result = unsafe { libc::ioctl(file.as_raw_fd(), crate::sys::ioc(crate::sys::IOC_WRITE, b'E', 0x90, 4) as _, 1) };
                if result < 0 {
                    eprintln!("input: cannot grab {}: {}", p.display(), io::Error::last_os_error());
                    continue;
                }
            }
            out.push(Device { path: p.display().to_string(), name, file, wire: Wire::Evdev });
        }
    }
    out
}

impl Device {
    /// Drain pending key events without blocking.
    pub fn poll_keys(&mut self) -> io::Result<Vec<Key>> {
        let mut keys = Vec::new();
        let sz = match self.wire { Wire::Evdev => std::mem::size_of::<RawEvent>(), Wire::Datagram => 8 };
        let mut buf = vec![0u8; sz * 32];
        loop {
            match self.file.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    for chunk in buf[..n].chunks_exact(sz) {
                        let (type_, code, value) = match self.wire {
                            Wire::Evdev => {
                                let ev: RawEvent = unsafe { std::ptr::read_unaligned(chunk.as_ptr() as *const _) };
                                (ev.type_, ev.code, ev.value)
                            }
                            Wire::Datagram => (
                                u16::from_le_bytes([chunk[0], chunk[1]]),
                                u16::from_le_bytes([chunk[2], chunk[3]]),
                                i32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]),
                            ),
                        };
                        if type_ == EV_KEY {
                            keys.push(Key { code, value });
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

/// Serialize one evdev key onto a launch-bound Unix datagram socket.
/// Called only by PID 1; Home/Power are consumed by the supervisor first.
pub fn forward_key(fd: i32, key: Key) -> io::Result<()> {
    let event = RawEvent { type_: EV_KEY, code: key.code, value: key.value, ..Default::default() };
    let n = unsafe { libc::send(fd, &event as *const _ as *const _, std::mem::size_of::<RawEvent>(),
        libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL) };
    if n < 0 { Err(io::Error::last_os_error()) } else { Ok(()) }
}
