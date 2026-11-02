//! The pinned Lean 4.35 IO ABI, shared by the launcher and native PID1.
use std::{ffi::c_void, io, sync::OnceLock, thread::ThreadId};

pub type Object = *mut c_void;

extern "C" {
    fn initialize_tv_TV_Core(builtin: u8) -> Object;
    fn lean_io_mark_end_initialization();
    fn lean_dec_ref_cold(result: Object);
}

#[repr(C)]
struct Header { rc: i32, cs_sz: u16, other: u8, tag: u8 }
const _: [(); 8] = [(); std::mem::size_of::<Header>()];

/// Releases an owned reference using the pinned lean.h algorithm.
unsafe fn release(object: Object) {
    if object as usize & 1 != 0 { return; }
    let header = object.cast::<Header>();
    if (*header).rc > 1 { (*header).rc -= 1; }
    else if (*header).rc != 0 { lean_dec_ref_cold(object); }
}

unsafe fn check(result: Object) -> io::Result<()> {
    claim_owner()?;
    if result.is_null() || result as usize & 1 != 0 {
        return Err(io::Error::other("invalid Lean IO result"));
    }
    if (*result.cast::<Header>()).tag != 0 {
        eprintln!("Lean IO action returned an error");
        release(result);
        Err(io::Error::other("Lean IO failed"))
    } else {
        Ok(())
    }
}

/// Consumes an owned compiled-IO result, discarding its value. The erased
/// world token is not an ordinary EStateM.Result field.
/// # Safety
/// `result` must be a live, owned result of a compiled Lean IO action.
pub unsafe fn discard_io_result(result: Object) -> io::Result<()> {
    check(result)?;
    release(result);
    Ok(())
}

/// # Safety
/// `result` must be a live, owned result of a compiled Lean `IO Unit` action.
pub unsafe fn consume_io_unit(result: Object) -> io::Result<()> {
    discard_io_result(result)
}

/// Keep an owned reference using the same scalar/RC representation as lean.h.
unsafe fn retain(object: Object) {
    if object as usize & 1 != 0 { return; }
    let count = &mut (*object.cast::<Header>()).rc;
    if *count > 0 {
        *count = count.checked_add(1).unwrap_or(i32::MIN + 0x10000);
    } else if (*count as u32) > 0x90000000 {
        *count = count.wrapping_sub(1);
    }
}

/// Transfer a value out of an owned compiled IO success result. Retaining the
/// value before releasing the wrapper also supports a shared result wrapper.
/// # Safety
/// `result` must be a live, owned result of a compiled Lean IO action.
pub unsafe fn consume_io_value(result: Object) -> io::Result<Object> {
    check(result)?;
    let value = result.cast::<u8>().add(8).cast::<Object>().read();
    retain(value);
    release(result);
    Ok(value)
}

/// # Safety
/// `object` must be a live owned Lean reference, released on the executor.
pub unsafe fn release_object(object: Object) {
    claim_owner().expect("Lean reference released outside its executor");
    release(object);
}

/// # Safety
/// `result` must be a live, owned result of a compiled Lean `IO UInt64` action.
pub unsafe fn consume_io_u64(result: Object) -> io::Result<u64> {
    let value = consume_io_value(result)?;
    let word = value.cast::<u8>().add(8).cast::<u64>().read_unaligned();
    release(value);
    Ok(word)
}

static INITIALIZED: OnceLock<Result<(), String>> = OnceLock::new();
static OWNER: OnceLock<ThreadId> = OnceLock::new();

fn claim_owner() -> io::Result<()> {
    let current = std::thread::current().id();
    if *OWNER.get_or_init(|| current) == current { Ok(()) }
    else { Err(io::Error::other("Lean runtime entered from a second thread")) }
}

/// Record that the embedding PID1 initialized the shared Lean modules here.
/// # Safety
/// The caller must have successfully completed Lean runtime initialization on
/// this thread before starting any other thread that enters Lean.
pub unsafe fn adopt_initialized_runtime() {
    claim_owner().expect("Lean runtime adopted outside its executor");
    INITIALIZED.get_or_init(|| Ok(()));
}

/// Call once on the initial thread, before creating launcher threads. The
/// generated initializer initializes the actual Lean allocator and IO runtime.
pub fn initialize_runtime() -> io::Result<()> {
    claim_owner()?;
    match INITIALIZED.get_or_init(|| unsafe {
        consume_io_unit(initialize_tv_TV_Core(1)).map_err(|e| e.to_string())?;
        lean_io_mark_end_initialization();
        Ok(())
    }) {
        Ok(()) => Ok(()),
        Err(message) => Err(io::Error::other(message.clone())),
    }
}

/// Scope every entry to the single Lean executor. The marker makes this guard
/// non-Send; native worker threads may never enter the allocating runtime.
pub struct ThreadGuard { _same_thread: std::marker::PhantomData<*mut ()> }
impl ThreadGuard {
    pub fn attach() -> Self {
        claim_owner().expect("Lean runtime entered outside its executor");
        Self { _same_thread: std::marker::PhantomData }
    }
}
