//! Bring-up helpers for running as PID 1 on a kernel with no IP autoconfig
//! (the TV's 4.19 has CONFIG_IP_PNP=n): static IPv4 via the classic ioctls,
//! a UDP log sink, kernel-log (/dev/kmsg) reading, and module loading.

use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::os::fd::AsRawFd;
use std::sync::{Mutex, OnceLock};

const SIOCGIFFLAGS: u32 = 0x8913;
const SIOCSIFFLAGS: u32 = 0x8914;
const SIOCSIFADDR: u32 = 0x8916;
const SIOCSIFNETMASK: u32 = 0x891c;
const IFF_UP: i16 = 0x1;
const IFF_RUNNING: i16 = 0x40;

/// struct ifreq: 16-byte name + a union sized like struct ifmap
/// (24 bytes on 64-bit, 16 on 32-bit).
#[repr(C)]
struct IfReq {
    name: [u8; 16],
    u: IfReqUnion,
}

#[repr(C)]
#[derive(Clone, Copy)]
union IfReqUnion {
    addr: libc::sockaddr_in,
    flags: i16,
    _map: [libc::c_ulong; 2],
    _pad: [u8; 16],
    _pad64: IfMapPad,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IfMapPad {
    _a: libc::c_ulong,
    _b: libc::c_ulong,
    _c: [u8; 8],
}

fn ifreq(iface: &str) -> io::Result<IfReq> {
    let mut name = [0u8; 16];
    let b = iface.as_bytes();
    if b.len() >= name.len() {
        return Err(io::Error::other("interface name too long"));
    }
    name[..b.len()].copy_from_slice(b);
    Ok(IfReq { name, u: IfReqUnion { _pad64: IfMapPad { _a: 0, _b: 0, _c: [0; 8] } } })
}

fn sin(a: Ipv4Addr) -> libc::sockaddr_in {
    let mut s: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    s.sin_family = libc::AF_INET as libc::sa_family_t;
    s.sin_addr = libc::in_addr { s_addr: u32::from_ne_bytes(a.octets()) };
    s
}

/// Configure `iface` with `addr/prefix` and bring it up.
pub fn configure_ipv4(iface: &str, addr: Ipv4Addr, prefix: u8) -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    let fd = sock.as_raw_fd();
    let mask = if prefix == 0 { 0 } else { u32::MAX << (32 - prefix.min(32)) };
    let ioctl = |req: u32, r: &mut IfReq| -> io::Result<()> {
        if unsafe { libc::ioctl(fd, req as _, r as *mut IfReq) } < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    };
    let mut r = ifreq(iface)?;
    r.u.addr = sin(addr);
    ioctl(SIOCSIFADDR, &mut r)?;
    let mut r = ifreq(iface)?;
    r.u.addr = sin(Ipv4Addr::from(mask));
    ioctl(SIOCSIFNETMASK, &mut r)?;
    let mut r = ifreq(iface)?;
    ioctl(SIOCGIFFLAGS, &mut r)?;
    unsafe { r.u.flags |= IFF_UP | IFF_RUNNING };
    ioctl(SIOCSIFFLAGS, &mut r)?;
    Ok(())
}

/// Parse "a.b.c.d/nn".
pub fn parse_cidr(s: &str) -> Option<(Ipv4Addr, u8)> {
    let (a, p) = s.split_once('/').unwrap_or((s, "24"));
    Some((a.parse().ok()?, p.parse().ok()?))
}

/// Log lines go to the console, to an optional UDP sink, and to an in-memory
/// ring (so a late listener can still fetch the boot log over TCP).
pub struct Logger {
    udp: Option<(UdpSocket, SocketAddrV4)>,
    ring: Vec<String>,
}

fn logger() -> &'static Mutex<Logger> {
    static L: OnceLock<Mutex<Logger>> = OnceLock::new();
    L.get_or_init(|| Mutex::new(Logger { udp: None, ring: Vec::new() }))
}

/// Send future log lines to `dest` over UDP as well.
pub fn log_to_udp(dest: SocketAddrV4) -> io::Result<()> {
    let s = UdpSocket::bind("0.0.0.0:0")?;
    let mut l = logger().lock().unwrap();
    // Replay what was logged before the network came up.
    for line in &l.ring {
        let _ = s.send_to(format!("{line}\n").as_bytes(), dest);
    }
    l.udp = Some((s, dest));
    Ok(())
}

/// Log one line (adds a newline for UDP).
pub fn log_line(line: &str) {
    println!("{line}");
    let mut l = logger().lock().unwrap();
    if let Some((s, d)) = &l.udp {
        let _ = s.send_to(format!("{line}\n").as_bytes(), *d);
    }
    if l.ring.len() >= 4000 {
        l.ring.drain(..1000);
    }
    l.ring.push(line.to_owned());
}

/// Everything logged so far, one line per entry.
pub fn log_snapshot() -> Vec<String> {
    logger().lock().unwrap().ring.clone()
}

/// Forward kernel log records from /dev/kmsg into the log, forever
/// (run on its own thread). Each read returns one record:
/// "prio,seq,usec,flags;message".
pub fn forward_kmsg() {
    use std::io::Read;
    let Ok(mut f) = std::fs::File::open("/dev/kmsg") else { return };
    let mut buf = vec![0u8; 8192];
    loop {
        match f.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let rec = String::from_utf8_lossy(&buf[..n]);
                let (head, msg) = rec.split_once(';').unwrap_or(("", &rec));
                let usec: u64 = head.split(',').nth(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                let msg = msg.lines().next().unwrap_or("");
                log_line(&format!("kmsg [{:>5}.{:06}] {msg}", usec / 1_000_000, usec % 1_000_000));
            }
            Err(e) if e.raw_os_error() == Some(libc::EPIPE) => continue, // records overwritten
            Err(_) => break,
        }
    }
}

/// Load a kernel module file with finit_module(2). `params` like "path=/x".
pub fn load_module(path: &str, params: &str) -> io::Result<()> {
    let f = std::fs::File::open(path)?;
    let p = std::ffi::CString::new(params).map_err(io::Error::other)?;
    let r = unsafe { libc::syscall(libc::SYS_finit_module, f.as_raw_fd(), p.as_ptr(), 0) };
    if r < 0 {
        let e = io::Error::last_os_error();
        if e.raw_os_error() == Some(libc::EEXIST) {
            return Ok(()); // already loaded (or built in)
        }
        return Err(e);
    }
    Ok(())
}
