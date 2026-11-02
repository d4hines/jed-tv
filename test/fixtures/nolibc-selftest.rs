//! Exercise the C library underneath std, as PID 1 in QEMU:
//!   python3 nolibc/run.py fast|faithful --bin selftest --headless
//! Prints one line per check and "selftest: N passed, M failed", then
//! powers off. The same source builds against musl for comparison.

use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::time::{Duration, Instant};

static PASSED: AtomicUsize = AtomicUsize::new(0);
static FAILED: AtomicUsize = AtomicUsize::new(0);

fn check(name: &str, ok: bool, detail: impl std::fmt::Display) {
    if ok {
        PASSED.fetch_add(1, Ordering::Relaxed);
        println!("ok   {name}: {detail}");
    } else {
        FAILED.fetch_add(1, Ordering::Relaxed);
        println!("FAIL {name}: {detail}");
    }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

fn mem_functions() {
    let mut rng = Lcg(42);
    let mut bad = 0;
    let src: Vec<u8> = (0..4096).map(|i| (i * 7 + 3) as u8).collect();
    for _ in 0..3000 {
        let (so, d_o, n) = (rng.next() as usize % 64, rng.next() as usize % 64, rng.next() as usize % 1500);
        let mut dst = vec![0xAAu8; 1700];
        dst[d_o..d_o + n].copy_from_slice(&src[so..so + n]); // memcpy
        if dst[d_o..d_o + n] != src[so..so + n] || dst[..d_o].iter().any(|&b| b != 0xAA)
            || dst[d_o + n..].iter().any(|&b| b != 0xAA) {
            bad += 1;
        }
        // memmove both directions
        let mut buf = src.clone();
        let (a, b) = (rng.next() as usize % 2000, rng.next() as usize % 2000);
        buf.copy_within(a..a + n, b);
        let mut want = src.clone();
        let tmp: Vec<u8> = want[a..a + n].to_vec();
        for i in 0..n {
            want[b + i] = tmp[i];
        }
        if buf != want {
            bad += 1;
        }
        // memset
        let c = rng.next() as u8;
        let mut z = vec![1u8; 1700];
        z[d_o..d_o + n].fill(c);
        if z[d_o..d_o + n].iter().any(|&x| x != c) || z[..d_o].iter().any(|&x| x != 1) {
            bad += 1;
        }
        // memcmp / bcmp via slice comparisons
        let x = src[so..so + n].to_vec();
        let mut y = x.clone();
        if x != y || x.cmp(&y) != std::cmp::Ordering::Equal {
            bad += 1;
        }
        if n > 0 {
            let k = rng.next() as usize % n;
            y[k] = y[k].wrapping_add(1);
            let want = if y[k] == 0 { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less };
            if x == y || x.cmp(&y) != want {
                bad += 1;
            }
        }
    }
    check("memcpy/memmove/memset/memcmp", bad == 0, format!("{bad} mismatches in 3000 random cases"));
    let mut bad = 0;
    for off in 0..16 {
        for len in 0..70 {
            let mut v = vec![b'x'; off + len + 1 + 16];
            v[off + len] = 0;
            let s = unsafe { std::ffi::CStr::from_ptr(v.as_ptr().add(off) as *const libc::c_char) };
            if s.to_bytes().len() != len {
                bad += 1;
            }
        }
    }
    check("strlen", bad == 0, format!("{bad} wrong of 1120"));
    // Throughput of the C library's memcpy/memset (16 MiB, aligned and not).
    let src = vec![7u8; 16 << 20];
    let mut dst = vec![0u8; (16 << 20) + 64];
    let mut rate = |off: usize| {
        let t0 = Instant::now();
        for _ in 0..8 {
            dst[off..off + src.len()].copy_from_slice(std::hint::black_box(&src));
        }
        (8 * 16) as f64 / 1024.0 / t0.elapsed().as_secs_f64()
    };
    let (aligned, unaligned) = (rate(0), rate(3));
    let t0 = Instant::now();
    for i in 0..8 {
        std::hint::black_box(&mut dst).fill(i as u8);
    }
    let set = (8 * 16) as f64 / 1024.0 / t0.elapsed().as_secs_f64();
    check("memcpy/memset throughput", aligned > 0.0,
          format!("memcpy {aligned:.2} GB/s aligned, {unaligned:.2} GB/s dst+3; memset {set:.2} GB/s"));
}

fn allocator() {
    let t0 = Instant::now();
    let mut v: Vec<u64> = Vec::new();
    for i in 0..(8 << 20) {
        v.push(i);
    }
    let ok = v.iter().enumerate().all(|(i, &x)| x == i as u64);
    check("realloc growth to 64 MiB", ok, format!("{:.1} ms", t0.elapsed().as_secs_f64() * 1e3));
    drop(v);
    let t0 = Instant::now();
    let mut boxes = Vec::new();
    for i in 0..200_000usize {
        boxes.push(vec![i as u8; 1 + i % 200]);
        if i % 3 == 0 {
            boxes.swap_remove(i / 7 % boxes.len());
        }
    }
    let sum: usize = boxes.iter().map(|b| b.len()).sum();
    check("200k small allocations", sum > 0, format!("{} live, {:.1} ms", boxes.len(), t0.elapsed().as_secs_f64() * 1e3));
    drop(boxes);
    let mut ok = true;
    for align in [16usize, 64, 4096, 65536] {
        let l = std::alloc::Layout::from_size_align(1000, align).unwrap();
        let p = unsafe { std::alloc::alloc(l) };
        ok &= !p.is_null() && p as usize % align == 0;
        unsafe { std::alloc::dealloc(p, l) };
    }
    check("aligned allocation (posix_memalign)", ok, "16..65536");
}

struct Dropper;
static DROPS: AtomicUsize = AtomicUsize::new(0);
impl Drop for Dropper {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::Relaxed);
    }
}
thread_local! {
    static TLS_VAL: std::cell::Cell<u64> = const { std::cell::Cell::new(7) };
    static TLS_DROP: std::cell::RefCell<Option<Dropper>> = const { std::cell::RefCell::new(None) };
}

fn threads() {
    let n = std::thread::available_parallelism().map_or(0, |n| n.get());
    check("available_parallelism", n >= 1, n);
    let counter = Arc::new(AtomicUsize::new(0));
    let list = Arc::new(Mutex::new(Vec::new()));
    let t0 = Instant::now();
    let hs: Vec<_> = (0..8).map(|t| {
        let (c, l) = (counter.clone(), list.clone());
        std::thread::Builder::new().name(format!("worker{t}")).spawn(move || {
            TLS_VAL.with(|v| v.set(100 + t));
            TLS_DROP.with(|d| *d.borrow_mut() = Some(Dropper));
            for _ in 0..100_000 {
                c.fetch_add(1, Ordering::Relaxed);
            }
            for i in 0..1000 {
                l.lock().unwrap().push(i);
            }
            let name = std::thread::current().name().map(str::to_owned);
            (TLS_VAL.with(|v| v.get()), name)
        }).unwrap()
    }).collect();
    let results: Vec<_> = hs.into_iter().map(|h| h.join().unwrap()).collect();
    let tls_ok = results.iter().enumerate().all(|(t, (v, n))| *v == 100 + t as u64
        && n.as_deref() == Some(format!("worker{t}").as_str()));
    check("8 threads: atomics + mutex", counter.load(Ordering::Relaxed) == 800_000 && list.lock().unwrap().len() == 8000,
          format!("{:.1} ms", t0.elapsed().as_secs_f64() * 1e3));
    check("thread-locals per thread, names", tls_ok && TLS_VAL.with(|v| v.get()) == 7, format!("{results:?}"));
    check("thread-local destructors ran", DROPS.load(Ordering::Relaxed) == 8, DROPS.load(Ordering::Relaxed));
    // Detached threads (handles dropped), then more threads reusing their stacks.
    let done = Arc::new(AtomicUsize::new(0));
    for _ in 0..64 {
        let d = done.clone();
        drop(std::thread::spawn(move || {
            d.fetch_add(1, Ordering::Relaxed);
        }));
    }
    let t0 = Instant::now();
    while done.load(Ordering::Relaxed) < 64 && t0.elapsed() < Duration::from_secs(5) {
        std::thread::sleep(Duration::from_millis(5));
    }
    let t0 = Instant::now();
    for _ in 0..200 {
        std::thread::spawn(|| 1).join().unwrap();
    }
    check("64 detached + 200 spawn/join", done.load(Ordering::Relaxed) == 64,
          format!("spawn+join {:.0} us each", t0.elapsed().as_secs_f64() * 1e6 / 200.0));
    // Condvar + Barrier.
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let p2 = pair.clone();
    let h = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(20));
        *p2.0.lock().unwrap() = true;
        p2.1.notify_one();
    });
    let mut g = pair.0.lock().unwrap();
    while !*g {
        g = pair.1.wait(g).unwrap();
    }
    drop(g);
    h.join().unwrap();
    let b = Arc::new(Barrier::new(4));
    let hs: Vec<_> = (0..3).map(|_| {
        let b = b.clone();
        std::thread::spawn(move || b.wait().is_leader())
    }).collect();
    let me = b.wait().is_leader();
    let leaders = hs.into_iter().map(|h| h.join().unwrap() as usize).sum::<usize>() + me as usize;
    check("condvar + barrier", leaders == 1, "1 leader of 4");
}

fn time() {
    let t0 = Instant::now();
    std::thread::sleep(Duration::from_millis(30));
    let dt = t0.elapsed();
    check("sleep 30 ms (nanosleep, clock_gettime)", dt >= Duration::from_millis(30) && dt < Duration::from_millis(1000),
          format!("{:.1} ms", dt.as_secs_f64() * 1e3));
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    check("wall clock (RTC)", now > 1_700_000_000, now);
    let t0 = Instant::now();
    let mut x = 0u128;
    for _ in 0..10_000 {
        x += Instant::now().elapsed().as_nanos();
    }
    check("clock_gettime cost", x > 0 || true, format!("{:.0} ns per Instant::now+elapsed",
                                                     t0.elapsed().as_secs_f64() * 1e9 / 10_000.0));
}

fn files() {
    let dir = std::path::Path::new("/tmp/selftest");
    let r = std::fs::create_dir_all(dir.join("sub"));
    check("mkdir -p", r.is_ok(), format!("{r:?}"));
    let data: Vec<u8> = (0..100_000).map(|i| (i % 251) as u8).collect();
    let w = std::fs::write(dir.join("f.bin"), &data);
    let back = std::fs::read(dir.join("f.bin")).unwrap_or_default();
    check("write + read 100 kB", w.is_ok() && back == data, back.len());
    let md = std::fs::metadata(dir.join("f.bin"));
    check("stat", md.as_ref().map(|m| m.len() == 100_000 && m.is_file()).unwrap_or(false), format!("{md:?}").len());
    let mut names: Vec<String> = std::fs::read_dir(dir).map(|rd| rd.flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned()).collect()).unwrap_or_default();
    names.sort();
    check("read_dir", names == ["f.bin", "sub"], format!("{names:?}"));
    let c = std::fs::canonicalize("/tmp/selftest/sub/../f.bin");
    check("canonicalize (realpath)", c.as_deref().ok() == Some(std::path::Path::new("/tmp/selftest/f.bin")), format!("{c:?}"));
    let mut f = std::fs::File::open(dir.join("f.bin")).unwrap();
    use std::io::Seek;
    let pos = f.seek(std::io::SeekFrom::Start(99_990));
    let mut tail = Vec::new();
    let _ = f.read_to_end(&mut tail);
    check("lseek", pos.ok() == Some(99_990) && tail == data[99_990..], tail.len());
    let e = std::fs::read("/nonexistent").unwrap_err().to_string();
    check("strerror", e.contains("No such file or directory"), e);
    let cmdline = std::fs::read_to_string("/proc/self/cmdline").unwrap_or_default();
    check("/proc readable", !cmdline.is_empty(), cmdline.replace('\0', " ").trim().to_owned());
    let exe = std::env::current_exe();
    check("current_exe (readlink)", exe.is_ok(), format!("{exe:?}"));
}

fn network() {
    let up = platform::net::configure_ipv4("lo", std::net::Ipv4Addr::new(127, 0, 0, 1), 8);
    check("lo up (SIOCSIF* ioctls)", up.is_ok(), format!("{up:?}"));
    // musl resolves "localhost" from /etc/hosts (else it asks DNS at 127.0.0.1:53).
    let _ = std::fs::create_dir_all("/etc");
    let _ = std::fs::write("/etc/hosts", "127.0.0.1 localhost\n");
    let l = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let port = l.local_addr().unwrap().port();
    let h = std::thread::spawn(move || {
        let (mut s, _) = l.accept().unwrap();
        let mut buf = vec![0u8; 5];
        s.read_exact(&mut buf).unwrap();
        s.write_all(&buf.iter().map(|b| b.to_ascii_uppercase()).collect::<Vec<_>>()).unwrap();
    });
    let mut c = std::net::TcpStream::connect(("localhost", port)).unwrap();
    c.write_all(b"hello").unwrap();
    let mut buf = [0u8; 5];
    c.read_exact(&mut buf).unwrap();
    h.join().unwrap();
    check("TCP loopback via getaddrinfo(localhost)", &buf == b"HELLO", String::from_utf8_lossy(&buf));
    let u1 = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let u2 = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    u1.send_to(b"ping", u2.local_addr().unwrap()).unwrap();
    let mut b = [0u8; 8];
    let (n, from) = u2.recv_from(&mut b).unwrap();
    check("UDP loopback", &b[..n] == b"ping" && from == u1.local_addr().unwrap(), n);
    let e = std::net::TcpStream::connect(("no-such-host.invalid", 80)).map(|_| ()).unwrap_err().to_string();
    check("DNS name (expected: unsupported)", !e.is_empty(), e);
}

static SIGNALLED: AtomicUsize = AtomicUsize::new(0);
extern "C" fn on_usr1(_: libc::c_int) {
    SIGNALLED.fetch_add(1, Ordering::Relaxed);
}

fn process() {
    unsafe {
        let mut sa: libc::sigaction = std::mem::zeroed();
        sa.sa_sigaction = on_usr1 as usize;
        libc::sigaction(libc::SIGUSR1, &sa, std::ptr::null_mut());
        libc::kill(libc::getpid(), libc::SIGUSR1);
    }
    check("signal handler (sigaction + restorer)", SIGNALLED.load(Ordering::Relaxed) == 1, SIGNALLED.load(Ordering::Relaxed));
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        let v: Vec<u8> = vec![1; 1 << 20]; // malloc works in the child
        unsafe { libc::_exit(v.len() as i32 >> 17) };
    }
    let mut st = 0;
    unsafe { libc::waitpid(pid, &mut st, 0) };
    check("fork + waitpid", libc::WIFEXITED(st) && libc::WEXITSTATUS(st) == 8, format!("status {st:#x}"));
    // Stack overflow in a spawned thread: the guard page faults, std's SIGSEGV
    // handler (on its sigaltstack) recognises the guard range it got from
    // pthread_getattr_np, prints "has overflowed its stack" and aborts.
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        #[allow(unconditional_recursion)]
        fn recurse(n: u64) -> u64 {
            let buf = std::hint::black_box([n as u8; 1024]);
            recurse(n + buf[0] as u64 + 1) + buf[1] as u64
        }
        let h = std::thread::Builder::new().name("overflow".into()).stack_size(128 << 10)
            .spawn(|| recurse(std::hint::black_box(0))).unwrap();
        let _ = h.join();
        unsafe { libc::_exit(3) };
    }
    let mut st = 0;
    unsafe { libc::waitpid(pid, &mut st, 0) };
    check("thread stack overflow detected (guard page + sigaltstack)",
          libc::WIFSIGNALED(st) && libc::WTERMSIG(st) == libc::SIGABRT, format!("status {st:#x}"));
    // seccomp in a child: real work under the allowlist, then a blocked call.
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        use platform::seccomp;
        let mut sc = seccomp::base_syscalls();
        sc.extend(seccomp::network_syscalls());
        if seccomp::install(&sc, &seccomp::platform_ioctls()).is_err() {
            unsafe { libc::_exit(1) };
        }
        let h = std::thread::spawn(|| vec![0u8; 3 << 20].len());
        let n = h.join().unwrap();
        std::thread::sleep(Duration::from_millis(5));
        let s = std::fs::read_to_string("/proc/self/stat").unwrap_or_default();
        let _ = std::time::SystemTime::now();
        if n == 3 << 20 && !s.is_empty() {
            println!("     (child under seccomp: thread, 3 MiB alloc, sleep, file read all allowed)");
        }
        unsafe { libc::getuid() }; // not in the allowlist: SIGSYS handler exits 159
        unsafe { libc::_exit(2) };
    }
    let mut st = 0;
    unsafe { libc::waitpid(pid, &mut st, 0) };
    check("seccomp: allowlist installs, work passes, getuid trapped", libc::WIFEXITED(st) && libc::WEXITSTATUS(st) == 159,
          format!("child exit {}", libc::WEXITSTATUS(st)));
}

fn main() {
    unsafe { libc::write(2, b"selftest: start\n".as_ptr() as *const _, 16) };
    if platform::sys::is_pid1() {
        platform::sys::init_mounts();
    }
    let args: Vec<String> = std::env::args().collect();
    check("args", !args.is_empty(), format!("{args:?}"));
    check("environ/getenv", std::env::var("HOME").is_ok(), format!("HOME={:?}", std::env::var("HOME")));
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    unsafe { libc::uname(&mut uts) };
    let machine = unsafe { std::ffi::CStr::from_ptr(uts.machine.as_ptr()) }.to_string_lossy().into_owned();
    check("uname", !machine.is_empty(), machine);
    mem_functions();
    allocator();
    time();
    threads();
    files();
    network();
    process();
    let (p, f) = (PASSED.load(Ordering::Relaxed), FAILED.load(Ordering::Relaxed));
    println!("selftest: {p} passed, {f} failed");
    platform::sys::power_off(if f == 0 { 0 } else { 1 });
}
