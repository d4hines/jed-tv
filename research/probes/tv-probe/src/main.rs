//! tv-probe: read-only check of the "platform API" a static Rust binary gets
//! when launched from `adb shell` on the TCL G06 TV (Android 11, ARM32).
//!
//! It never writes to device nodes, never grabs input, and only opens audio /
//! framebuffer nodes read-only (expected to be refused by DAC/SELinux).
//!
//! Usage on the TV:  /data/local/tmp/tv-probe [listen_port]
//! Then from the laptop:  head -c 50000000 /dev/zero | nc 192.168.77.2 <port>

use std::ffi::CString;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::{Duration, Instant};

/// Minimal musl/Linux ARM32 FFI so the crate has no build scripts (the host
/// toolchain here cannot link build scripts against the macOS 27 SDK).
#[allow(non_camel_case_types, non_upper_case_globals)]
mod libc {
    pub type c_int = i32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_char = u8;
    #[repr(C)]
    pub struct utsname {
        pub sysname: [c_char; 65],
        pub nodename: [c_char; 65],
        pub release: [c_char; 65],
        pub version: [c_char; 65],
        pub machine: [c_char; 65],
        pub domainname: [c_char; 65],
    }
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: i64, // musl >= 1.2 uses 64-bit time_t on arm
        pub tv_nsec: c_long,
        pub _pad: c_long,
    }
    pub const AT_HWCAP: c_ulong = 16;
    pub const AT_HWCAP2: c_ulong = 26;
    pub const CLOCK_REALTIME: c_int = 0;
    pub const CLOCK_MONOTONIC: c_int = 1;
    pub const CLOCK_MONOTONIC_RAW: c_int = 4;
    pub const CLOCK_BOOTTIME: c_int = 7;
    pub const SYS_getrandom: c_long = 384;
    pub const O_RDONLY: c_int = 0;
    pub const O_RDWR: c_int = 2;
    pub const O_NONBLOCK: c_int = 0o4000;
    pub const O_CLOEXEC: c_int = 0o2000000;
    extern "C" {
        pub fn uname(buf: *mut utsname) -> c_int;
        pub fn getuid() -> u32;
        pub fn geteuid() -> u32;
        pub fn getgid() -> u32;
        pub fn getauxval(t: c_ulong) -> c_ulong;
        #[link_name = "__clock_getres_time64"]
        pub fn clock_getres(id: c_int, ts: *mut timespec) -> c_int;
        #[link_name = "__clock_gettime64"]
        pub fn clock_gettime(id: c_int, ts: *mut timespec) -> c_int;
        pub fn syscall(n: c_long, ...) -> c_long;
        pub fn open(path: *const c_char, flags: c_int, ...) -> c_int;
        pub fn close(fd: c_int) -> c_int;
        pub fn ioctl(fd: c_int, req: c_int, ...) -> c_int;
    }
}

fn section(name: &str) {
    println!("\n== {name}");
}

fn errno_str() -> String {
    std::io::Error::last_os_error().to_string()
}

fn read_file(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s.trim_end_matches(['\0', '\n']).to_string(),
        Err(e) => format!("<{e}>"),
    }
}

fn identity() {
    section("identity");
    let mut u: libc::utsname = unsafe { std::mem::zeroed() };
    unsafe { libc::uname(&mut u) };
    let f = |b: &[libc::c_char]| unsafe { std::ffi::CStr::from_ptr(b.as_ptr() as *const std::ffi::c_char) }
        .to_string_lossy()
        .into_owned();
    println!("uname: {} {} {} {}", f(&u.sysname), f(&u.release), f(&u.version), f(&u.machine));
    println!("uid={} euid={} gid={}", unsafe { libc::getuid() }, unsafe { libc::geteuid() }, unsafe { libc::getgid() });
    println!("selinux context: {}", read_file("/proc/self/attr/current"));
    for line in read_file("/proc/self/status").lines() {
        if ["Seccomp", "NoNewPrivs", "CapEff", "CapBnd", "Groups"].iter().any(|k| line.starts_with(k)) {
            println!("{line}");
        }
    }
    println!("/etc/resolv.conf: {}", if std::path::Path::new("/etc/resolv.conf").exists() { "present" } else { "ABSENT" });
    println!("available_parallelism: {:?}", std::thread::available_parallelism());
}

fn hwcap() {
    section("hwcap");
    let hw = unsafe { libc::getauxval(libc::AT_HWCAP) };
    let hw2 = unsafe { libc::getauxval(libc::AT_HWCAP2) };
    let names1 = [
        (0, "swp"), (1, "half"), (2, "thumb"), (3, "26bit"), (4, "fastmult"), (5, "fpa"),
        (6, "vfp"), (7, "edsp"), (8, "java"), (9, "iwmmxt"), (10, "crunch"), (11, "thumbee"),
        (12, "neon"), (13, "vfpv3"), (14, "vfpv3d16"), (15, "tls"), (16, "vfpv4"),
        (17, "idiva"), (18, "idivt"), (19, "vfpd32"), (20, "lpae"), (21, "evtstrm"),
    ];
    let names2 = [(0, "aes"), (1, "pmull"), (2, "sha1"), (3, "sha2"), (4, "crc32")];
    let decode = |v: u64, n: &[(u32, &str)]| {
        n.iter().filter(|(b, _)| v & (1u64 << b) != 0).map(|(_, s)| *s).collect::<Vec<_>>().join(" ")
    };
    println!("AT_HWCAP=0x{hw:x}: {}", decode(hw as u64, &names1));
    println!("AT_HWCAP2=0x{hw2:x}: {}", decode(hw2 as u64, &names2));
}

fn clocks() {
    section("clock");
    for (name, id) in [
        ("REALTIME", libc::CLOCK_REALTIME),
        ("MONOTONIC", libc::CLOCK_MONOTONIC),
        ("MONOTONIC_RAW", libc::CLOCK_MONOTONIC_RAW),
        ("BOOTTIME", libc::CLOCK_BOOTTIME),
    ] {
        let mut res = libc::timespec { tv_sec: 0, tv_nsec: 0, _pad: 0 };
        let mut now = libc::timespec { tv_sec: 0, tv_nsec: 0, _pad: 0 };
        let r1 = unsafe { libc::clock_getres(id, &mut res) };
        let r2 = unsafe { libc::clock_gettime(id, &mut now) };
        println!("{name}: res={}ns now={}.{:09} (rc {r1},{r2})", res.tv_nsec, now.tv_sec, now.tv_nsec);
    }
    // Cost of clock_gettime (vDSO vs syscall) and sleep jitter, both matter for A/V sync.
    let n = 200_000;
    let t = Instant::now();
    for _ in 0..n {
        std::hint::black_box(Instant::now());
    }
    println!("Instant::now cost: {:.1} ns/call", t.elapsed().as_nanos() as f64 / n as f64);
    let mut worst = Duration::ZERO;
    let mut total = Duration::ZERO;
    for _ in 0..200 {
        let t = Instant::now();
        std::thread::sleep(Duration::from_millis(1));
        let over = t.elapsed().saturating_sub(Duration::from_millis(1));
        total += over;
        worst = worst.max(over);
    }
    println!("sleep(1ms) oversleep: mean {:?} worst {:?}", total / 200, worst);
}

fn random() {
    section("random");
    let mut buf = [0u8; 32];
    let r = unsafe { libc::syscall(libc::SYS_getrandom, buf.as_mut_ptr(), buf.len() as u32, 0u32) };
    println!("getrandom(32,0) = {r} {}", if r == 32 { "ok".into() } else { errno_str() });
    let mut f = [0u8; 32];
    match std::fs::File::open("/dev/urandom").and_then(|mut h| h.read_exact(&mut f)) {
        Ok(()) => println!("/dev/urandom: ok"),
        Err(e) => println!("/dev/urandom: {e}"),
    }
}

fn try_open(path: &str, flags: libc::c_int) -> Option<libc::c_int> {
    let c = CString::new(path).unwrap();
    let fd = unsafe { libc::open(c.as_ptr() as *const u8, flags | libc::O_CLOEXEC | libc::O_NONBLOCK) };
    if fd < 0 {
        println!("  {path:<28} {}: {}", if flags & libc::O_RDWR != 0 { "rw" } else { "ro" }, errno_str());
        None
    } else {
        println!("  {path:<28} {}: OPEN", if flags & libc::O_RDWR != 0 { "rw" } else { "ro" });
        Some(fd)
    }
}

fn devices() {
    section("device access (ro unless noted)");
    for p in [
        "/dev/graphics/fb0", "/dev/graphics/fb1", "/dev/snd/pcmC0D0p", "/dev/snd/controlC0",
        "/dev/hw_random", "/dev/kmsg", "/dev/uinput", "/dev/tee0", "/proc/cmdline",
    ] {
        if let Some(fd) = try_open(p, libc::O_RDONLY) {
            unsafe { libc::close(fd) };
        }
    }
    // These are world-rw on this firmware; opening only creates/destroys a context.
    for p in ["/dev/ion", "/dev/mali0"] {
        if let Some(fd) = try_open(p, libc::O_RDWR) {
            unsafe { libc::close(fd) };
        }
    }
}

fn input() {
    section("input (EVIOCGNAME only, no grab, no writes)");
    for i in 0..8 {
        let path = format!("/dev/input/event{i}");
        if !std::path::Path::new(&path).exists() {
            continue;
        }
        if let Some(fd) = try_open(&path, libc::O_RDONLY) {
            let mut name = [0u8; 128];
            // EVIOCGNAME(len) = _IOC(_IOC_READ, 'E', 0x06, len)
            let req = (2u32 << 30) | ((name.len() as u32) << 16) | (0x45 << 8) | 0x06;
            let r = unsafe { libc::ioctl(fd, req as i32, name.as_mut_ptr()) };
            let n = String::from_utf8_lossy(&name[..name.iter().position(|&b| b == 0).unwrap_or(0)]).into_owned();
            println!("    name={n:?} (ioctl rc {r})");
            unsafe { libc::close(fd) };
        }
    }
}

fn compute() {
    section("compute");
    // Memory bandwidth: 64 MiB copy, best of 5.
    let size = 64 << 20;
    let src = vec![1u8; size];
    let mut dst = vec![0u8; size];
    let mut best = f64::MAX;
    for _ in 0..5 {
        let t = Instant::now();
        dst.copy_from_slice(&src);
        std::hint::black_box(&dst);
        best = best.min(t.elapsed().as_secs_f64());
    }
    println!("memcpy 64MiB: {:.0} MB/s", size as f64 / best / 1e6);
    // Single-core integer throughput: FNV-1a over 64 MiB.
    let t = Instant::now();
    let mut h: u32 = 0x811c9dc5;
    for &b in &src {
        h = (h ^ b as u32).wrapping_mul(0x01000193);
    }
    std::hint::black_box(h);
    println!("fnv1a 64MiB single-core: {:.0} MB/s", size as f64 / t.elapsed().as_secs_f64() / 1e6);
    // Same work across all cores, to see whether 4 cores scale.
    let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let t = Instant::now();
    std::thread::scope(|s| {
        for chunk in src.chunks(size / cores) {
            s.spawn(move || {
                let mut h: u32 = 0x811c9dc5;
                for &b in chunk {
                    h = (h ^ b as u32).wrapping_mul(0x01000193);
                }
                std::hint::black_box(h);
            });
        }
    });
    println!("fnv1a 64MiB on {cores} threads: {:.0} MB/s", size as f64 / t.elapsed().as_secs_f64() / 1e6);
}

fn net(port: u16) {
    section("network");
    match std::net::ToSocketAddrs::to_socket_addrs("www.youtube.com:443") {
        Ok(mut a) => println!("DNS www.youtube.com (musl, /etc/resolv.conf): {:?}", a.next()),
        Err(e) => println!("DNS www.youtube.com (musl, /etc/resolv.conf): {e}"),
    }
    let l = match TcpListener::bind(("0.0.0.0", port)) {
        Ok(l) => l,
        Err(e) => {
            println!("bind 0.0.0.0:{port}: {e}");
            return;
        }
    };
    println!("listening on 0.0.0.0:{port}; waiting up to 60s for one sender");
    l.set_nonblocking(true).ok();
    let deadline = Instant::now() + Duration::from_secs(60);
    let mut s = loop {
        match l.accept() {
            Ok((s, peer)) => {
                println!("accepted from {peer}");
                break s;
            }
            Err(_) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(50)),
            Err(e) => {
                println!("no sender: {e}");
                return;
            }
        }
    };
    s.set_nonblocking(false).ok();
    s.set_read_timeout(Some(Duration::from_secs(20))).ok();
    let mut buf = vec![0u8; 256 << 10];
    let mut total = 0usize;
    let t = Instant::now();
    loop {
        match s.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => total += n,
            Err(e) => {
                println!("read error: {e}");
                break;
            }
        }
    }
    let secs = t.elapsed().as_secs_f64();
    let msg = format!("received {total} bytes in {secs:.2}s = {:.1} Mbit/s", total as f64 * 8.0 / secs / 1e6);
    println!("{msg}");
    let _ = s.write_all(msg.as_bytes());
}

fn main() {
    let port: u16 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    identity();
    hwcap();
    clocks();
    random();
    devices();
    input();
    compute();
    if port != 0 {
        net(port);
    }
    println!("\n== done");
}
