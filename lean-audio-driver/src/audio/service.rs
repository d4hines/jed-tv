//! Linux and MI mechanisms for the Lean-owned direct audio driver, shared by
//! PID1 on the G06 and the standalone direct-audio process. Every decision is
//! Lean's (src/lean/TV/Audio); this file moves bytes, runs ioctls, keeps the
//! clock and the single-owner lease, and reports each outcome as one word:
//! bits 0..7 tag (0 ok, 1 MI status, 2 errno), bits 32..63 value. Byte arrays
//! and strings cross through src/lean/native/audio.c.
use crate::{aout_route, mi_pcm::{self, DeviceTransport, VendorError}};
use std::{
    cell::RefCell,
    fs,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{atomic::{AtomicU8, AtomicUsize, Ordering}, Arc, OnceLock},
    thread,
    time::{Duration, Instant},
};
use tvinit::lean_runtime::{self, Object};

pub type Log = Arc<dyn Fn(&str) + Send + Sync>;

/// 0 free, 1 owned, 2 poisoned: a cleanup failed and explicit recovery is required.
static OWNER: AtomicU8 = AtomicU8::new(0);
static START: OnceLock<Instant> = OnceLock::new();
const MAX_CLIENTS: usize = 4;
const IO_TIMEOUT: Duration = Duration::from_secs(3);

/// Processes that may own the speaker; each must be stopped (SIGSTOP or exited)
/// before the direct driver touches the hardware. Names are /proc/PID/comm.
pub const MEDIA_OWNERS: [&str; 9] = [
    "surfaceflinger",
    "composer@2.4-se",
    "audioserver",
    "mstar.hardware.",
    "midaemon",
    "app-rust-displa",
    "app-rust-audio-",
    "native-audio",
    "direct-audio",
];

/// The driver runs only on the TV whose serial it was built and checked for,
/// and only once every other media owner is stopped. Does not touch hardware.
pub fn verify_ownership(serial: &str) -> io::Result<()> {
    let expected = format!("androidboot.serialno={serial}");
    if !fs::read_to_string("/proc/cmdline")?.split_whitespace().any(|v| v == expected) {
        return Err(io::Error::other(format!("direct audio requires the TV with serial {serial}")));
    }
    let me = std::process::id().to_string();
    for entry in fs::read_dir("/proc")?.flatten() {
        let name = entry.file_name();
        let pid = name.to_string_lossy();
        if pid.parse::<u32>().is_err() || pid == me {
            continue;
        }
        let Ok(comm) = fs::read_to_string(entry.path().join("comm")) else { continue };
        if !MEDIA_OWNERS.contains(&comm.trim()) {
            continue;
        }
        let status = fs::read_to_string(entry.path().join("status"))?;
        let stopped = status
            .lines()
            .find_map(|s| s.strip_prefix("State:"))
            .is_some_and(|s| matches!(s.trim().chars().next(), Some('T' | 't')));
        if !stopped {
            return Err(io::Error::other(format!("media owner must be stopped first: {}", comm.trim())));
        }
    }
    Ok(())
}

struct Connection {
    socket: TcpStream,
    pcm: Option<DeviceTransport>,
    aout: Option<DeviceTransport>,
    log: Log,
}
thread_local! { static CONNECTION: RefCell<Option<Connection>> = const { RefCell::new(None) }; }
extern "C" {
    fn tv_audio_client() -> Object;
    fn tv_audio_io_u64(value: u64) -> Object;
    fn tv_audio_io_unit(unused: u32) -> Object;
}

fn with<R>(f: impl FnOnce(&mut Connection) -> R) -> R {
    CONNECTION.with(|slot| f(slot.borrow_mut().as_mut().expect("audio effect outside a connection")))
}
fn word(result: io::Result<u32>) -> u64 {
    match result {
        Ok(value) => (value as u64) << 32,
        Err(error) => match error.get_ref().and_then(|e| e.downcast_ref::<VendorError>()) {
            Some(vendor) => 1 | ((vendor.status as u32 as u64) << 32),
            None => 2 | ((error.raw_os_error().unwrap_or(5) as u32 as u64) << 32),
        },
    }
}
fn unit(result: io::Result<()>) -> u64 {
    word(result.map(|_| 0))
}
fn io_word(value: u64) -> Object {
    unsafe { tv_audio_io_u64(value) }
}
fn io_unit() -> Object {
    unsafe { tv_audio_io_unit(0) }
}
fn no_device() -> io::Error {
    io::Error::from_raw_os_error(9) // EBADF: Lean asked before opening
}

/// Accept app connections on `listener`; each runs the Lean driver on its own
/// thread. The Lean runtime and the driver's module must already be initialized
/// on the calling process (PID1 does so through TV.Pid1; direct-audio in main).
pub fn serve(listener: TcpListener, log: Log) {
    START.get_or_init(Instant::now);
    let active = Arc::new(AtomicUsize::new(0));
    let address = listener.local_addr().map(|a| a.to_string()).unwrap_or_default();
    log(&format!("LEAN_AUDIO ready {address} direct MI PCM, warm hardware, 48k stereo output"));
    for connection in listener.incoming() {
        let Ok(socket) = connection else { continue };
        let _ = socket.set_nodelay(true);
        if active.fetch_add(1, Ordering::Relaxed) >= MAX_CLIENTS {
            active.fetch_sub(1, Ordering::Relaxed);
            continue;
        }
        let (log, active) = (log.clone(), active.clone());
        thread::spawn(move || {
            let _thread = lean_runtime::ThreadGuard::attach();
            CONNECTION.with(|slot| {
                *slot.borrow_mut() = Some(Connection { socket, pcm: None, aout: None, log: log.clone() })
            });
            let result = unsafe { lean_runtime::consume_io_u64(tv_audio_client()) };
            if let Err(error) = result {
                log(&format!("LEAN_AUDIO client: Lean IO failed: {error}"));
            }
            CONNECTION.with(|slot| { slot.borrow_mut().take(); });
            active.fetch_sub(1, Ordering::Relaxed);
        });
    }
}

// Socket.
#[no_mangle]
extern "C" fn tv_audio_recv_command() -> Object {
    io_word(with(|c| {
        let mut byte = [0u8];
        let result = c.socket.set_read_timeout(None).and_then(|_| c.socket.read(&mut byte));
        match result {
            Ok(1) => (byte[0] as u64) << 32,
            Ok(_) => 2,
            Err(error) => word(Err(error)),
        }
    }))
}
#[no_mangle]
extern "C" fn tv_audio_recv_bytes(buffer: *mut u8, len: u32) -> u64 {
    with(|c| {
        let buffer = unsafe { std::slice::from_raw_parts_mut(buffer, len as usize) };
        unit(c.socket.set_read_timeout(Some(IO_TIMEOUT)).and_then(|_| c.socket.read_exact(buffer)))
    })
}
#[no_mangle]
extern "C" fn tv_audio_send_bytes(buffer: *const u8, len: u32) -> u64 {
    with(|c| {
        let buffer = unsafe { std::slice::from_raw_parts(buffer, len as usize) };
        unit(c.socket.set_write_timeout(Some(IO_TIMEOUT)).and_then(|_| c.socket.write_all(buffer)))
    })
}

// Host.
#[no_mangle]
extern "C" fn tv_audio_gain() -> Object {
    let directory = std::env::var("TV_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
    let gain = fs::read_to_string(std::path::Path::new(&directory).join("app-volume"))
        .ok()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .filter(|v| v.is_finite())
        .unwrap_or(0.0)
        .clamp(0.0, 1.0);
    io_word((gain * 1_000_000.0).round() as u64)
}
#[no_mangle]
extern "C" fn tv_audio_now() -> Object {
    io_word(START.get_or_init(Instant::now).elapsed().as_millis() as u64)
}
#[no_mangle]
extern "C" fn tv_audio_sleep(ms: u32) -> Object {
    thread::sleep(Duration::from_millis(ms as u64));
    io_unit()
}
#[no_mangle]
extern "C" fn tv_audio_log_bytes(text: *const u8, len: u32) {
    let text = unsafe { std::slice::from_raw_parts(text, len as usize) };
    with(|c| (c.log)(&String::from_utf8_lossy(text)));
}

// Lease.
#[no_mangle]
extern "C" fn tv_audio_acquire() -> Object {
    io_word(match OWNER.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => 0,
        Err(owner) => owner as u64,
    })
}
#[no_mangle]
extern "C" fn tv_audio_release() -> Object {
    let _ = OWNER.compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire);
    io_unit()
}
#[no_mangle]
extern "C" fn tv_audio_poison() -> Object {
    OWNER.store(2, Ordering::Release);
    io_unit()
}

// MI PCM.
#[no_mangle]
extern "C" fn tv_audio_pcm_open() -> Object {
    io_word(with(|c| {
        word(DeviceTransport::open_pcm().and_then(|device| {
            let handle = mi_pcm::open(&device)?;
            c.pcm = Some(device);
            Ok(handle)
        }))
    }))
}
#[no_mangle]
extern "C" fn tv_audio_pcm_start(handle: u32) -> Object {
    io_word(with(|c| unit(c.pcm.as_ref().ok_or_else(no_device).and_then(|d| mi_pcm::start(d, handle)))))
}
#[no_mangle]
extern "C" fn tv_audio_pcm_write_bytes(handle: u32, buffer: *const u8, len: u32) -> u64 {
    with(|c| {
        let pcm = unsafe { std::slice::from_raw_parts(buffer, len as usize) };
        word(c.pcm.as_ref().ok_or_else(no_device).and_then(|d| mi_pcm::write(d, handle, pcm)))
    })
}
#[no_mangle]
extern "C" fn tv_audio_pcm_queued(handle: u32) -> Object {
    io_word(with(|c| {
        word(c.pcm.as_ref().ok_or_else(no_device).and_then(|d| mi_pcm::buffer_info(d, handle)).map(|b| b.queued_bytes))
    }))
}
#[no_mangle]
extern "C" fn tv_audio_pcm_stop(handle: u32) -> Object {
    io_word(with(|c| unit(c.pcm.as_ref().ok_or_else(no_device).and_then(|d| mi_pcm::stop(d, handle)))))
}
/// Close releases the device file whatever the ioctl said; Lean reports the status.
#[no_mangle]
extern "C" fn tv_audio_pcm_close(handle: u32) -> Object {
    io_word(with(|c| unit(c.pcm.take().ok_or_else(no_device).and_then(|d| mi_pcm::close(&d, handle)))))
}
/// A known intentional leak: closing the device could free memory the speaker
/// still references. Process-level recovery is explicit.
#[no_mangle]
extern "C" fn tv_audio_pcm_retain(_handle: u32) -> Object {
    with(|c| {
        if let Some(device) = c.pcm.take() {
            std::mem::forget(device);
        }
    });
    io_unit()
}

// MI AOUT: the existing speaker output.
fn aout<R>(c: &Connection, f: impl FnOnce(&DeviceTransport) -> io::Result<R>) -> io::Result<R> {
    c.aout.as_ref().ok_or_else(no_device).and_then(f)
}
#[no_mangle]
extern "C" fn tv_audio_aout_speaker() -> Object {
    io_word(with(|c| {
        word(DeviceTransport::open_aout().and_then(|device| {
            let handle = aout_route::speaker(&device)?;
            c.aout = Some(device);
            Ok(handle)
        }))
    }))
}
#[no_mangle]
extern "C" fn tv_audio_aout_count(output: u32) -> Object {
    io_word(with(|c| word(aout(c, |d| aout_route::count(d, output)))))
}
#[no_mangle]
extern "C" fn tv_audio_aout_connected_into(output: u32, count: u32, handles: *mut u32) -> u64 {
    with(|c| {
        unit(aout(c, |d| aout_route::connected(d, output, count)).map(|list| {
            for (i, handle) in list.iter().enumerate().take(count as usize) {
                unsafe { handles.add(i).write_unaligned(*handle) }
            }
        }))
    })
}
#[no_mangle]
extern "C" fn tv_audio_aout_connect(output: u32, input: u32) -> Object {
    io_word(with(|c| unit(aout(c, |d| aout_route::connect(d, output, input)))))
}
#[no_mangle]
extern "C" fn tv_audio_aout_disconnect(output: u32, input: u32) -> Object {
    io_word(with(|c| unit(aout(c, |d| aout_route::disconnect(d, output, input)))))
}
#[no_mangle]
extern "C" fn tv_audio_aout_volume_into(output: u32, pair: *mut i32) -> u64 {
    with(|c| {
        unit(aout(c, |d| aout_route::volume_db(d, output)).map(|value| unsafe {
            pair.write_unaligned(value.integer);
            pair.add(1).write_unaligned(value.thousandths);
        }))
    })
}
#[no_mangle]
extern "C" fn tv_audio_aout_set_volume(output: u32, integer: u32, thousandths: u32) -> Object {
    let value = aout_route::VolumeDb { integer: integer as i32, thousandths: thousandths as i32 };
    io_word(with(|c| unit(aout(c, |d| aout_route::set_volume_db(d, output, value)))))
}
