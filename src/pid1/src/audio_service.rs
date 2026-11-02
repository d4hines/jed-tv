//! Nonblocking native operations for the cooperative Lean audio executor.
//! Every Lean allocation/call happens on the PID1/Core thread in tick().
use super::State;
use g06_audio::device_resources::Resources;
use std::{
    cell::{Cell, RefCell},
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    ptr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tvinit::lean_runtime::{consume_io_value, discard_io_result, Object};

const FAILED: u32 = 0x8000_0000;
const SLOTS: usize = 4;
struct Prepared {
    listener: TcpListener,
    state: Arc<State>,
}
static PREPARED: Mutex<Option<Prepared>> = Mutex::new(None);
struct Client {
    socket: TcpStream,
}
struct Runtime {
    listener: TcpListener,
    clients: [Option<Client>; SLOTS],
    device: Resources,
    events: Arc<State>,
    epoch: Instant,
    gain_time: Instant,
    gain: u32,
    scratch: Box<[u8; 65536]>,
    lean: Object,
}
thread_local! {
    static SERVICE: RefCell<Option<Box<Runtime>>> = const { RefCell::new(None) };
    static CURRENT: Cell<*mut Runtime> = const { Cell::new(ptr::null_mut()) };
}
extern "C" {
    fn initialize_tv_TV_Audio_Cooperative(builtin: u8) -> Object;
    fn tv_audio_initial(ignored: u32) -> Object;
    fn tv_audio_tick(state: Object) -> Object;
    fn tv_audio_io_u32(value: u32) -> Object;
    fn tv_audio_io_unit(ignored: u32) -> Object;
    fn tv_audio_io_now(low: u32, high: u32) -> Object;
    fn tv_audio_io_read(bytes: Object, status: u32) -> Object;
    fn tv_audio_alloc_bytes(size: u32) -> Object;
}
#[repr(C)]
struct Header {
    rc: i32,
    size: u16,
    other: u8,
    tag: u8,
}
#[repr(C)]
struct ScalarArray {
    header: Header,
    size: usize,
    capacity: usize,
}

fn error(error: io::Error) -> u32 {
    FAILED | error.raw_os_error().unwrap_or(libc::EIO) as u32
}
fn with<R>(f: impl FnOnce(&mut Runtime) -> R) -> R {
    CURRENT.with(|p| {
        assert!(!p.get().is_null(), "audio effect outside main executor");
        unsafe { f(&mut *p.get()) }
    })
}
unsafe fn bytes<'a>(value: Object, offset: u32, count: u32) -> io::Result<&'a [u8]> {
    if value.is_null() || value as usize & 1 != 0 {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }
    let h = &*value.cast::<ScalarArray>();
    let end = (offset as usize)
        .checked_add(count as usize)
        .ok_or_else(|| io::Error::from_raw_os_error(libc::EINVAL))?;
    if h.header.tag != 248 || h.header.other & 0x7f != 1 || h.size > h.capacity || end > h.size {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }
    Ok(std::slice::from_raw_parts(
        value
            .cast::<u8>()
            .add(std::mem::size_of::<ScalarArray>() + offset as usize),
        count as usize,
    ))
}
fn number(value: u32) -> Object {
    unsafe { tv_audio_io_u32(value) }
}
fn unit() -> Object {
    unsafe { tv_audio_io_unit(0) }
}

/// Safe to call during native startup: this stages Rust resources only.
pub(super) fn prepare(state: Arc<State>) -> io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 8877))?;
    listener.set_nonblocking(true)?;
    let mut pending = PREPARED.lock().unwrap();
    if pending.is_some() {
        return Err(io::Error::other("audio already prepared"));
    }
    state.service("audio", "listening");
    state.event("PID1 cooperative Lean audio: localhost:8877");
    *pending = Some(Prepared { listener, state });
    Ok(())
}
/// Call from Children::tick / the Core executor, never from a worker thread.
pub(super) fn tick() {
    SERVICE.with(|cell| {
        let mut service = cell.borrow_mut();
        if service.is_none() {
            let Some(prepared) = PREPARED.lock().unwrap().take() else {
                return;
            };
            let init = unsafe { discard_io_result(initialize_tv_TV_Audio_Cooperative(1)) };
            if let Err(error) = init {
                prepared
                    .state
                    .event(format!("audio Lean init failed: {error}"));
                return;
            }
            let now = Instant::now();
            *service = Some(Box::new(Runtime {
                listener: prepared.listener,
                clients: std::array::from_fn(|_| None),
                device: Resources::default(),
                events: prepared.state,
                epoch: now,
                gain_time: now - Duration::from_secs(1),
                gain: 0,
                scratch: Box::new([0; 65536]),
                lean: unsafe { tv_audio_initial(0) },
            }));
        }
        let runtime = service.as_mut().unwrap();
        if runtime.lean.is_null() {
            return;
        }
        CURRENT.with(|p| {
            assert!(p.get().is_null());
            p.set(&mut **runtime);
        });
        let old = std::mem::replace(&mut runtime.lean, ptr::null_mut());
        let result = unsafe { consume_io_value(tv_audio_tick(old)) };
        CURRENT.with(|p| p.set(ptr::null_mut()));
        match result {
            Ok(next) => runtime.lean = next,
            Err(error) => {
                for slot in 0..SLOTS {
                    runtime.close(slot);
                }
                runtime.device.abandon();
                runtime.events.service("audio", "failed");
                runtime.events.event(format!(
                    "audio executor stopped; device ownership retained until recovery: {error}"
                ));
            }
        }
    });
}
impl Runtime {
    fn close(&mut self, slot: usize) {
        if slot < SLOTS {
            self.clients[slot] = None;
        }
    }
}

#[no_mangle]
extern "C" fn tv_audio_server_accept(slot: u32) -> Object {
    number(with(|rt| {
        if slot as usize >= SLOTS || rt.clients[slot as usize].is_some() {
            return FAILED | libc::EINVAL as u32;
        }
        match rt.listener.accept() {
            Ok((socket, _)) => {
                if let Err(e) = socket
                    .set_nonblocking(true)
                    .and_then(|_| socket.set_nodelay(true))
                {
                    return error(e);
                }
                rt.clients[slot as usize] = Some(Client { socket });
                1
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => 0,
            Err(e) => error(e),
        }
    }))
}
#[no_mangle]
extern "C" fn tv_audio_server_read(slot: u32, count: u32) -> Object {
    with(|rt| {
        let result = match rt.clients.get_mut(slot as usize).and_then(Option::as_mut) {
            Some(client) => client
                .socket
                .read(&mut rt.scratch[..(count as usize).min(65536)]),
            None => Err(io::Error::from_raw_os_error(libc::EBADF)),
        };
        let (size, status) = match result {
            Ok(0) => (0, 2),
            Ok(n) => (n, 0),
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                ) =>
            {
                (0, 1)
            }
            Err(e) => (0, error(e)),
        };
        unsafe {
            let value = tv_audio_alloc_bytes(size as u32);
            if size > 0 {
                ptr::copy_nonoverlapping(
                    rt.scratch.as_ptr(),
                    value.cast::<u8>().add(std::mem::size_of::<ScalarArray>()),
                    size,
                );
            }
            tv_audio_io_read(value, status)
        }
    })
}
#[no_mangle]
extern "C" fn tv_audio_server_write(slot: u32, data: Object, offset: u32, count: u32) -> Object {
    number(with(|rt| {
        let data = match unsafe { bytes(data, offset, count) } {
            Ok(v) => v,
            Err(e) => return error(e),
        };
        let result = rt
            .clients
            .get_mut(slot as usize)
            .and_then(Option::as_mut)
            .ok_or_else(|| io::Error::from_raw_os_error(libc::EBADF))
            .and_then(|c| c.socket.write(data));
        match result {
            Ok(n) => n as u32,
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                ) =>
            {
                0
            }
            Err(e) => error(e),
        }
    }))
}
#[no_mangle]
extern "C" fn tv_audio_server_close(slot: u32) -> Object {
    with(|rt| rt.close(slot as usize));
    unit()
}
#[no_mangle]
extern "C" fn tv_audio_device_step(operation: u32) -> Object {
    number(with(|rt| match rt.device.step(operation) {
        Ok(()) => 0,
        Err(failure) => {
            rt.events.event(format!(
                "audio device operation {operation}: {}",
                failure.error
            ));
            if failure.uncertain {
                2
            } else {
                1
            }
        }
    }))
}
#[no_mangle]
extern "C" fn tv_audio_server_pcm(slot: u32, data: Object, offset: u32, count: u32) -> Object {
    number(with(|rt| {
        let data = match unsafe { bytes(data, offset, count) } {
            Ok(v) => v,
            Err(e) => return error(e),
        };
        let result = if rt
            .clients
            .get(slot as usize)
            .and_then(Option::as_ref)
            .is_some()
        {
            rt.device.write(data)
        } else {
            Err(io::Error::from_raw_os_error(libc::EBADF))
        };
        match result {
            Ok(n) => n as u32,
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                ) =>
            {
                0
            }
            Err(e) => error(e),
        }
    }))
}
#[no_mangle]
extern "C" fn tv_audio_server_queued(slot: u32) -> Object {
    number(with(|rt| {
        if rt
            .clients
            .get(slot as usize)
            .and_then(Option::as_ref)
            .is_none()
        {
            return FAILED | libc::EBADF as u32;
        }
        rt.device.queued_bytes().unwrap_or_else(error)
    }))
}

#[no_mangle]
extern "C" fn tv_audio_server_gain() -> Object {
    number(with(|rt| {
        if rt.gain_time.elapsed() >= Duration::from_millis(50) {
            rt.gain_time = Instant::now();
            let value = std::fs::read_to_string(
                std::path::Path::new(
                    &std::env::var("TV_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into()),
                )
                .join("app-volume"),
            )
            .ok()
            .and_then(|s| s.trim().parse::<f32>().ok())
            .filter(|v| v.is_finite())
            .unwrap_or(0.0)
            .clamp(0.0, 0.30);
            rt.gain = value.to_bits();
        }
        rt.gain
    }))
}
#[no_mangle]
extern "C" fn tv_audio_server_now() -> Object {
    let ms = with(|rt| rt.epoch.elapsed().as_millis() as u64);
    unsafe { tv_audio_io_now(ms as u32, (ms >> 32) as u32) }
}
#[no_mangle]
extern "C" fn tv_audio_server_report(
    slot: u32,
    low: u32,
    high: u32,
    played_low: u32,
    played_high: u32,
    source: u32,
    output: u32,
    gain: u32,
) -> Object {
    with(|rt| {
        rt.events.event(format!("LEAN_AUDIO slot={slot} written={} played={} source_peak={source} output_peak={output} gain={}",
        low as u64|((high as u64)<<32),played_low as u64|((played_high as u64)<<32),f32::from_bits(gain)))
    });
    unit()
}
