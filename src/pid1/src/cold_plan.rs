//! Owned text requests from the cold hardware worker to PID1's Lean executor.
//! No Lean object or closure crosses a channel or survives a native call.
use mi_bootstrap::cold_start::IniPolicy;
use std::{
    cell::Cell,
    io, ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, SyncSender, TryRecvError},
        Mutex, OnceLock,
    },
    time::Duration,
};
use tvinit::lean_runtime::{self, Object};

const MAX_INI_BYTES: usize = 64 * 1024;
const REPLY_TIMEOUT: Duration = Duration::from_secs(10);
#[derive(Clone, Copy)]
pub(crate) enum Operation {
    Prepare,
    Validate,
}
struct Request {
    operation: Operation,
    text: String,
    reply: SyncSender<io::Result<String>>,
}
pub(crate) struct Client {
    sender: SyncSender<Request>,
}
static CLAIMED: AtomicBool = AtomicBool::new(false);
static PENDING: Mutex<Option<Receiver<Request>>> = Mutex::new(None);
thread_local! {
    static REQUESTS: std::cell::RefCell<Option<Receiver<Request>>> = const { std::cell::RefCell::new(None) };
    static CURRENT: Cell<*mut Option<io::Result<String>>> = const { Cell::new(ptr::null_mut()) };
}
static INITIALIZED: OnceLock<Result<(), String>> = OnceLock::new();
extern "C" {
    fn initialize_tv_TV_Hardware_ColdPlan(builtin: u8) -> Object;
    fn tv_cold_plan_prepare(text: Object) -> Object;
    fn tv_cold_plan_validate(text: Object) -> Object;
    fn tv_cold_plan_io_unit(unused: u32) -> Object;
    fn lean_mk_string_from_bytes(data: *const u8, size: usize) -> Object;
}

/// Set up one finite request stream; the worker never enters Lean itself.
pub(crate) fn start() -> io::Result<Client> {
    lean_runtime::initialize_runtime()?;
    CLAIMED
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .map_err(|_| io::Error::other("cold INI policy already claimed"))?;
    let (sender, receiver) = mpsc::sync_channel(1);
    *PENDING
        .lock()
        .map_err(|_| io::Error::other("cold INI channel poisoned"))? = Some(receiver);
    Ok(Client { sender })
}
impl Client {
    fn request(&self, operation: Operation, text: &str) -> io::Result<String> {
        if text.len() > MAX_INI_BYTES {
            return Err(io::Error::other("cold INI exceeds bound"));
        }
        let (reply, result) = mpsc::sync_channel(1);
        self.sender
            .try_send(Request {
                operation,
                text: text.to_owned(),
                reply,
            })
            .map_err(|_| io::Error::other("cold INI executor unavailable or busy"))?;
        result
            .recv_timeout(REPLY_TIMEOUT)
            .map_err(|_| io::Error::other("cold INI executor did not reply"))?
    }
}
impl IniPolicy for Client {
    fn prepare(&self, stock: &str) -> io::Result<String> {
        self.request(Operation::Prepare, stock)
    }
    fn validate(&self, staged: &str) -> io::Result<()> {
        if self.request(Operation::Validate, staged)?.is_empty() {
            Ok(())
        } else {
            Err(io::Error::other(
                "cold INI validator returned unexpected data",
            ))
        }
    }
}

/// One bounded policy request per main-executor poll. Errors return to the
/// worker, which already treats every staging failure as terminal.
pub(crate) fn tick() {
    if lean_runtime::initialize_runtime().is_err() {
        return;
    }
    REQUESTS.with(|cell| {
        let mut requests = cell.borrow_mut();
        if requests.is_none() {
            *requests = PENDING.lock().expect("cold INI channel poisoned").take();
        }
        let Some(receiver) = requests.as_ref() else {
            return;
        };
        match receiver.try_recv() {
            Ok(request) => {
                let _ = request
                    .reply
                    .send(evaluate(request.operation, &request.text));
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                *requests = None;
            }
        }
    });
}
struct Scope;
impl Drop for Scope {
    fn drop(&mut self) {
        CURRENT.with(|p| p.set(ptr::null_mut()));
    }
}

/// The only entry to the compiled policy. initialize_runtime checks thread
/// ownership before any input allocation, including on failed worker misuse.
pub(crate) fn evaluate(operation: Operation, text: &str) -> io::Result<String> {
    lean_runtime::initialize_runtime()?;
    let _executor = lean_runtime::ThreadGuard::attach();
    match INITIALIZED.get_or_init(|| unsafe {
        lean_runtime::consume_io_unit(initialize_tv_TV_Hardware_ColdPlan(1))
            .map_err(|e| e.to_string())
    }) {
        Ok(()) => (),
        Err(message) => return Err(io::Error::other(message.clone())),
    }
    if text.len() > MAX_INI_BYTES {
        return Err(io::Error::other("cold INI exceeds bound"));
    }
    let mut result = None;
    CURRENT.with(|p| {
        assert!(p.get().is_null(), "recursive cold INI policy entry");
        p.set(&mut result);
    });
    let scope = Scope;
    let status = unsafe {
        let input = lean_mk_string_from_bytes(text.as_ptr(), text.len());
        lean_runtime::consume_io_unit(match operation {
            Operation::Prepare => tv_cold_plan_prepare(input),
            Operation::Validate => tv_cold_plan_validate(input),
        })
    };
    drop(scope);
    status?;
    result.ok_or_else(|| io::Error::other("cold INI policy did not reply"))?
}

#[repr(C)]
struct LeanString {
    header: [u8; 8],
    size: usize,
    capacity: usize,
    length: usize,
}
unsafe fn copied_string(value: Object) -> io::Result<String> {
    if value.is_null() || value as usize & 1 != 0 {
        return Err(io::Error::other("invalid cold INI result"));
    }
    let object = &*value.cast::<LeanString>();
    if object.header[7] != 249
        || object.size == 0
        || object.size > MAX_INI_BYTES + 256
        || object.size > object.capacity
    {
        return Err(io::Error::other("invalid cold INI result bounds"));
    }
    let bytes = std::slice::from_raw_parts(
        value.cast::<u8>().add(std::mem::size_of::<LeanString>()),
        object.size - 1,
    );
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| io::Error::other("invalid cold INI result UTF-8"))
}
#[no_mangle]
unsafe extern "C" fn tv_cold_plan_result(success: u32, kind: u32, text: Object) -> Object {
    CURRENT.with(|p| {
        assert!(
            !p.get().is_null(),
            "cold INI callback outside executor request"
        );
        let output = &mut *p.get();
        if output.is_some() {
            *output = Some(Err(io::Error::other("cold INI policy replied twice")));
        } else {
            *output = Some(copied_string(text).and_then(|text| match (success, kind) {
                (1, 0) => Ok(text),
                (0, 1) => Err(io::Error::other(text)),
                (0, 2) => Err(io::Error::new(io::ErrorKind::InvalidData, text)),
                _ => Err(io::Error::other("invalid cold INI policy result tag")),
            }));
        }
    });
    tv_cold_plan_io_unit(0)
}
