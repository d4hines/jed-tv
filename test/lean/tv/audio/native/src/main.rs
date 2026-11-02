//! Actual production socket/FFI adapter with only the MI device replaced.
extern crate self as g06_audio;
extern crate self as tvinit;
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
#[derive(Default)]
pub struct State;
impl State {
    fn service(&self, _: &str, _: &str) {}
    fn event(&self, s: impl AsRef<str>) {
        if s.as_ref().starts_with("LEAN_AUDIO") {
            REPORTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        println!("{}", s.as_ref());
    }
}
#[path = "../../../../../../src/pid1/tvinit/src/lean_runtime.rs"]
pub mod lean_runtime;

#[path = "../../../../../../src/pid1/audio/aout-route.rs"]
pub mod aout_route;
#[path = "../../../../../../src/pid1/audio/mi-pcm.rs"]
pub mod mi_pcm;
#[path = "../../../../../../src/pid1/audio/device-resources.rs"]
pub mod real_resources;
static FAIL_NEXT_REQUEST: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static REQUESTS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
static WRITES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static REPORTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static PCM: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static OWNED: Mutex<bool> = Mutex::new(false);
static INPUTS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
static GAIN: Mutex<aout_route::VolumeDb> = Mutex::new(aout_route::VolumeDb {
    integer: -47,
    thousandths: 0,
});
#[derive(Clone)]
pub struct Mock;
fn word(bytes: &[u8], at: usize) -> u32 {
    u32::from_ne_bytes(bytes[at..at + 4].try_into().unwrap())
}
impl mi_pcm::Transport for Mock {
    fn transact(&self, request: u32, bytes: &mut [u8]) -> std::io::Result<()> {
        REQUESTS.lock().unwrap().push(request);
        if FAIL_NEXT_REQUEST
            .compare_exchange(
                request,
                0,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            )
            .is_ok()
        {
            return Err(std::io::Error::other("injected MI failure"));
        }
        match request {
            0xc0803402 => {
                let mut owned = OWNED.lock().unwrap();
                if *owned {
                    return Err(std::io::Error::from_raw_os_error(libc::EBUSY));
                }
                *owned = true;
                bytes[..4].copy_from_slice(&0x81000000u32.to_ne_bytes());
            }
            0xc0081116 => bytes[4..8].copy_from_slice(&0x17000001u32.to_ne_bytes()),
            0xc010111e => {
                bytes[12..16].copy_from_slice(&(INPUTS.lock().unwrap().len() as u32).to_ne_bytes())
            }
            0xc018111f => {
                let dest =
                    u64::from_ne_bytes(bytes[16..24].try_into().unwrap()) as usize as *mut u32;
                for (i, input) in INPUTS
                    .lock()
                    .unwrap()
                    .iter()
                    .take(word(bytes, 12) as usize)
                    .enumerate()
                {
                    unsafe {
                        dest.add(i).write(*input);
                    }
                }
            }
            0xc00c111c => INPUTS.lock().unwrap().push(word(bytes, 4)),
            0xc008111d => INPUTS
                .lock()
                .unwrap()
                .retain(|input| *input != word(bytes, 4)),
            0xc010110a => {
                let gain = GAIN.lock().unwrap();
                bytes[8..12].copy_from_slice(&gain.integer.to_ne_bytes());
                bytes[12..16].copy_from_slice(&gain.thousandths.to_ne_bytes());
            }
            0x40101109 => {
                *GAIN.lock().unwrap() = aout_route::VolumeDb {
                    integer: word(bytes, 8) as i32,
                    thousandths: word(bytes, 12) as i32,
                }
            }
            0xc0083404 | 0xc0043405 => (),
            0xc0043403 => {
                assert!(INPUTS.lock().unwrap().is_empty());
                *OWNED.lock().unwrap() = false;
            }
            0xc018340b => {
                let dest = u64::from_ne_bytes(bytes[16..24].try_into().unwrap()) as usize
                    as *mut mi_pcm::BufferInfo;
                unsafe {
                    *dest = mi_pcm::BufferInfo {
                        capacity_bytes: 19200,
                        queued_bytes: 0,
                        free_bytes: 19200,
                    };
                }
            }
            0xc0383408 => {
                let n = if WRITES.fetch_add(1, std::sync::atomic::Ordering::SeqCst) % 11 == 10 {
                    0
                } else {
                    (word(bytes, 16) as usize).min(640)
                };
                let source =
                    u64::from_ne_bytes(bytes[8..16].try_into().unwrap()) as usize as *const u8;
                PCM.lock()
                    .unwrap()
                    .extend_from_slice(unsafe { std::slice::from_raw_parts(source, n) });
                bytes[48..52].copy_from_slice(&(n as u32).to_ne_bytes());
            }
            _ => panic!("unexpected MI request {request:x}"),
        }
        Ok(())
    }
}
pub mod device_resources {
    pub type Resources = super::real_resources::Resources<super::Mock, super::Mock>;
}
impl Default for device_resources::Resources {
    fn default() -> Self {
        Self::with_factories(
            || mi_pcm::Pcm::open_with(Mock),
            || aout_route::Aout::existing_speaker_with(Mock),
        )
    }
}
#[path = "../../../../../../src/pid1/src/audio_service.rs"]
mod audio_service;
#[allow(dead_code)]
#[path = "../../../../../../research/probes/audio/pcm-converter.rs"]
mod reference_converter;
fn status(stream: &mut TcpStream) {
    let mut b = [0; 4];
    stream.read_exact(&mut b).unwrap();
    assert_eq!(i32::from_le_bytes(b), 0);
}
fn client() {
    let mut s = TcpStream::connect("127.0.0.1:8877").unwrap();
    s.set_nodelay(true).unwrap();
    s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    let mut h = b"O".to_vec();
    for n in [44100u32, 1, 2] {
        h.extend(n.to_le_bytes());
    }
    for part in h.chunks(3) {
        s.write_all(part).unwrap();
    }
    status(&mut s);
    let input = [10000i16, -10000];
    s.write_all(b"W").unwrap();
    s.write_all(&4u32.to_le_bytes()).unwrap();
    for n in input {
        s.write_all(&n.to_le_bytes()).unwrap();
    }
    status(&mut s);
    s.write_all(b"D").unwrap();
    let mut counts = [0; 16];
    s.read_exact(&mut counts).unwrap();
    assert_eq!(u64::from_le_bytes(counts[..8].try_into().unwrap()), 2);
    assert_eq!(u64::from_le_bytes(counts[8..].try_into().unwrap()), 2);
    s.write_all(b"R").unwrap();
    status(&mut s);
    s.write_all(b"W").unwrap();
    s.write_all(&2u32.to_le_bytes()).unwrap();
    s.write_all(&32767i16.to_le_bytes()).unwrap();
    status(&mut s);
    s.write_all(b"F").unwrap();
    status(&mut s);
    s.write_all(b"R").unwrap();
    status(&mut s);
    s.write_all(b"W").unwrap();
    s.write_all(&1048576u32.to_le_bytes()).unwrap();
    s.write_all(&vec![0; 1048576]).unwrap();
    status(&mut s);
    s.write_all(b"D").unwrap();
    let mut counts = [0; 16];
    s.read_exact(&mut counts).unwrap();
    assert_eq!(u64::from_le_bytes(counts[..8].try_into().unwrap()), 524288);
    assert_eq!(u64::from_le_bytes(counts[8..].try_into().unwrap()), 524288);
    s.write_all(b"?").unwrap();
    let mut failure = [0; 4];
    s.read_exact(&mut failure).unwrap();
    assert_eq!(i32::from_le_bytes(failure), -22);
    let mut tail = [0; 1];
    assert_eq!(s.read(&mut tail).unwrap(), 0);
    let mut invalid = TcpStream::connect("127.0.0.1:8877").unwrap();
    invalid
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    invalid.write_all(b"D").unwrap();
    invalid.read_exact(&mut failure).unwrap();
    assert_eq!(i32::from_le_bytes(failure), -22);
}
fn exhaustive() {
    let gain_file =
        std::path::Path::new(&std::env::var("TV_RUNTIME_DIR").unwrap()).join("app-volume");
    for boost in 1..=4u32 {
        let mut s = TcpStream::connect("127.0.0.1:8877").unwrap();
        s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        s.set_nodelay(true).unwrap();
        s.write_all(b"O").unwrap();
        for n in [48000u32, 1, boost] {
            s.write_all(&n.to_le_bytes()).unwrap();
        }
        status(&mut s);
        for gain in [0.0f32, 0.0025, 0.015, 0.075, 0.15, 0.25, 0.3] {
            std::fs::write(&gain_file, format!("{gain}\n")).unwrap();
            thread::sleep(Duration::from_millis(60));
            PCM.lock().unwrap().clear();
            let data: Vec<_> = (0..=65535u32)
                .flat_map(|n| (n as u16).to_le_bytes())
                .collect();
            s.write_all(b"W").unwrap();
            s.write_all(&(data.len() as u32).to_le_bytes()).unwrap();
            s.write_all(&data).unwrap();
            status(&mut s);
            let expected: Vec<_> = (0..=65535u32)
                .flat_map(|n| {
                    let sample = n as u16 as i16;
                    let value = (((sample as f32 * boost as f32).clamp(-32767.0, 32767.0) * gain)
                        .round() as i16)
                        .to_le_bytes();
                    [value[0], value[1], value[0], value[1]]
                })
                .collect();
            let output = PCM.lock().unwrap();
            if *output != expected {
                let at = output
                    .iter()
                    .zip(&expected)
                    .position(|(a, b)| a != b)
                    .unwrap_or(output.len().min(expected.len()));
                panic!("mix mismatch boost={boost} gain={gain} at byte {at}; actual={:?} expected={:?}",&output[at..(at+8).min(output.len())],&expected[at..(at+8).min(expected.len())]);
            }
            drop(output);
            s.write_all(b"R").unwrap();
            status(&mut s);
        }
        s.write_all(b"?").unwrap();
        let mut failure = [0; 4];
        s.read_exact(&mut failure).unwrap();
        assert_eq!(i32::from_le_bytes(failure), -22);
        let mut eof = [0];
        assert_eq!(s.read(&mut eof).unwrap(), 0);
    }
}

fn converter_oracle() {
    let gain_file =
        std::path::Path::new(&std::env::var("TV_RUNTIME_DIR").unwrap()).join("app-volume");
    for rate in [44100u32, 48000] {
        for channels in [1u32, 2] {
            for split in [false, true] {
                std::fs::write(&gain_file, "0.25\n").unwrap();
                thread::sleep(Duration::from_millis(60));
                PCM.lock().unwrap().clear();
                let mut s = TcpStream::connect("127.0.0.1:8877").unwrap();
                s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
                s.set_nodelay(true).unwrap();
                s.write_all(b"O").unwrap();
                for n in [rate, channels, 1] {
                    s.write_all(&n.to_le_bytes()).unwrap();
                }
                status(&mut s);
                let input: Vec<_> = (0..=65535u32).map(|n| n as u16 as i16).collect();
                let mut converter = reference_converter::Converter::new(rate, channels).unwrap();
                let mut expected = Vec::new();
                let mut offset = 0;
                let mut turn = 0;
                while offset < input.len() {
                    let count = if split {
                        ((turn * 17) % 373 + 1) * channels as usize
                    } else {
                        input.len()
                    };
                    let end = (offset + count).min(input.len());
                    let part = &input[offset..end];
                    let data: Vec<_> = part.iter().flat_map(|v| v.to_le_bytes()).collect();
                    s.write_all(b"W").unwrap();
                    s.write_all(&(data.len() as u32).to_le_bytes()).unwrap();
                    s.write_all(&data).unwrap();
                    status(&mut s);
                    let mixed: Vec<_> = part
                        .iter()
                        .map(|&v| ((v as f32).clamp(-32767.0, 32767.0) * 0.25).round() as i16)
                        .collect();
                    expected.extend(converter.convert(&mixed).unwrap());
                    offset = end;
                    turn += 1;
                }
                let output = PCM.lock().unwrap();
                if *output != expected {
                    let at = output
                        .iter()
                        .zip(&expected)
                        .position(|(a, b)| a != b)
                        .unwrap_or(output.len().min(expected.len()));
                    panic!(
                        "converter mismatch rate={rate} channels={channels} split={split} at {at}"
                    );
                }
                drop(output);
                s.write_all(b"D").unwrap();
                let mut counts = [0; 16];
                s.read_exact(&mut counts).unwrap();
                assert_eq!(
                    u64::from_le_bytes(counts[..8].try_into().unwrap()),
                    input.len() as u64 / channels as u64
                );
                assert_eq!(
                    u64::from_le_bytes(counts[8..].try_into().unwrap()),
                    input.len() as u64 / channels as u64
                );
                s.write_all(b"?").unwrap();
                let mut code = [0; 4];
                s.read_exact(&mut code).unwrap();
                assert_eq!(i32::from_le_bytes(code), -22);
                let mut tail = [0];
                assert_eq!(s.read(&mut tail).unwrap(), 0);
            }
        }
    }
}

fn device_failures() {
    use std::sync::atomic::Ordering::SeqCst;
    fn open() -> TcpStream {
        let mut s = TcpStream::connect("127.0.0.1:8877").unwrap();
        s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        s.write_all(b"O").unwrap();
        for n in [48000u32, 2, 1] {
            s.write_all(&n.to_le_bytes()).unwrap();
        }
        s
    }
    fn rejected(s: &mut TcpStream) {
        let mut code = [0; 4];
        s.read_exact(&mut code).unwrap();
        assert_eq!(i32::from_le_bytes(code), -1);
        let mut tail = [0];
        assert_eq!(s.read(&mut tail).unwrap(), 0);
    }
    FAIL_NEXT_REQUEST.store(0xc0083404, SeqCst);
    rejected(&mut open());
    assert!(!*OWNED.lock().unwrap());
    assert!(INPUTS.lock().unwrap().is_empty());
    let mut s = open();
    status(&mut s);
    FAIL_NEXT_REQUEST.store(0xc008111d, SeqCst);
    drop(s);
    let deadline = Instant::now() + Duration::from_secs(10);
    while FAIL_NEXT_REQUEST.load(SeqCst) != 0 {
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
    thread::sleep(Duration::from_millis(5));
    assert!(*OWNED.lock().unwrap(), "uncertain route released PCM");
    assert_eq!(*INPUTS.lock().unwrap(), [0x81000000]);
    let count = REQUESTS.lock().unwrap().len();
    rejected(&mut open());
    assert_eq!(
        REQUESTS.lock().unwrap().len(),
        count,
        "poisoned device retried MI operations"
    );
}

#[cfg(target_os = "linux")]
fn loopback() {
    unsafe {
        let fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
        assert!(fd >= 0);
        let mut req: libc::ifreq = std::mem::zeroed();
        req.ifr_name[0] = b'l' as _;
        req.ifr_name[1] = b'o' as _;
        req.ifr_ifru.ifru_flags = (libc::IFF_UP | libc::IFF_RUNNING) as _;
        assert_eq!(libc::ioctl(fd, libc::SIOCSIFFLAGS as _, &req), 0);
        libc::close(fd);
    }
}
fn main() {
    #[cfg(target_os = "linux")]
    if std::process::id() == 1 {
        loopback();
    }
    let dir = std::env::temp_dir().join(format!("audio-cooperative-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::env::set_var("TV_RUNTIME_DIR", &dir);
    std::fs::write(dir.join("app-volume"), "0.0025\n").unwrap();
    audio_service::prepare(Arc::new(State)).unwrap();
    let app = thread::spawn(client);
    let deadline = Instant::now() + Duration::from_secs(10);
    while !app.is_finished() {
        audio_service::tick();
        thread::sleep(Duration::from_millis(1));
        assert!(Instant::now() < deadline);
    }
    app.join().unwrap();
    for _ in 0..4 {
        audio_service::tick();
    }
    let mut expected: Vec<_> = [50i16, 50, -33, -33, 82, 82]
        .into_iter()
        .flat_map(i16::to_le_bytes)
        .collect();
    expected.resize(
        expected.len() + ((524288u64 * 48000 / 44100) as usize) * 4,
        0,
    );
    assert_eq!(*PCM.lock().unwrap(), expected);
    assert!(!*OWNED.lock().unwrap());
    assert_eq!(REPORTS.load(std::sync::atomic::Ordering::SeqCst), 3);
    println!("PASS: production cooperative adapter, real TCP, fragmented open, PCM, counters, reset, flush, cleanup");
    let app = thread::spawn(exhaustive);
    let deadline = Instant::now() + Duration::from_secs(120);
    while !app.is_finished() {
        audio_service::tick();
        thread::sleep(Duration::from_millis(1));
        assert!(Instant::now() < deadline);
    }
    app.join().unwrap();
    for _ in 0..4 {
        audio_service::tick();
    }
    assert!(!*OWNED.lock().unwrap());
    println!(
        "PASS: 1835008 exact PCM sample conversions through production TCP/Lean/device adapter"
    );
    let app = thread::spawn(converter_oracle);
    let deadline = Instant::now() + Duration::from_secs(120);
    while !app.is_finished() {
        audio_service::tick();
        thread::sleep(Duration::from_millis(1));
        assert!(Instant::now() < deadline);
    }
    app.join().unwrap();
    for _ in 0..4 {
        audio_service::tick();
    }
    assert!(!*OWNED.lock().unwrap());
    println!("PASS: 8 streaming converter oracle cases, every input value, both rates/channels/chunkings");
    let app = thread::spawn(device_failures);
    let deadline = Instant::now() + Duration::from_secs(30);
    while !app.is_finished() {
        audio_service::tick();
        thread::sleep(Duration::from_millis(1));
        assert!(Instant::now() < deadline);
    }
    app.join().unwrap();
    println!("PASS: production Lean device rollback, uncertain route retention, and poisoned reopen refusal");
}
