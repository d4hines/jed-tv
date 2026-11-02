//! Threads: thread-local storage (TLS variant I, as on ARM and AArch64),
//! errno, and the pthread subset std uses (create/join/detach/self, attrs,
//! keys with destructors, getattr_np, setname_np).
//!
//! Memory of a thread, one mmap: [guard page][stack ...][Thread][TCB|TLS]
//!                                                         thread pointer ^
//! The thread pointer (TP) points at the 2-word ABI TCB; the program's TLS
//! block follows at TP + align_up(TCB_SIZE, p_align); our `Thread` record sits
//! just below TP, so `__errno_location` is one subtraction away.
//!
//! Finished threads' mappings are kept on a free list and reused by the next
//! pthread_create (a detached thread cannot unmap the stack it is running on,
//! and reuse also avoids mmap/munmap churn). A mapping is only reused after
//! the kernel has cleared its tid (CLONE_CHILD_CLEARTID), i.e. after the
//! thread is completely gone.

use crate::arch::{self, TCB_SIZE};
use crate::start::{phdrs, AUX};
use crate::sync::{futex_wait, Locked, Racy};
use core::mem::size_of;
use core::ptr::{self, null_mut};
use core::sync::atomic::{AtomicI32, AtomicU32, AtomicUsize, Ordering};
use libc::{c_char, c_int, c_uint, c_void, pthread_attr_t, pthread_t, size_t};

const KEYS_MAX: usize = 64;
const DEFAULT_STACK: usize = 256 * 1024;
const MAX_CACHED: usize = 16;

const JOINABLE: u32 = 0;
const DETACHED: u32 = 1;
const EXITED: u32 = 2;

#[repr(C, align(16))]
pub struct Thread {
    pub errno: c_int,
    /// Kernel tid; set by CLONE_PARENT_SETTID, zeroed + futex-woken by the
    /// kernel when the thread is gone (CLONE_CHILD_CLEARTID).
    tid: AtomicI32,
    state: AtomicU32,
    map_base: usize,
    map_size: usize,
    stack_lo: usize,
    stack_size: usize,
    guard_size: usize,
    start: usize,
    arg: usize,
    result: usize,
    next: *mut Thread,
    keys: [usize; KEYS_MAX],
}

#[derive(Clone, Copy)]
struct TlsImage {
    image: usize,
    filesz: usize,
    memsz: usize,
    align: usize,
}

static TLS: Racy<TlsImage> = Racy::new(TlsImage { image: 0, filesz: 0, memsz: 0, align: 1 });
static FREE: Locked<*mut Thread> = Locked::new(null_mut());
static KEY_DTORS: [AtomicUsize; KEYS_MAX] = [const { AtomicUsize::new(0) }; KEYS_MAX];
static KEY_USED: [AtomicU32; KEYS_MAX] = [const { AtomicU32::new(0) }; KEYS_MAX];

#[inline(always)]
pub fn self_thread() -> *mut Thread {
    (arch::tp() - size_of::<Thread>()) as *mut Thread
}

#[no_mangle]
pub extern "C" fn __errno_location() -> *mut c_int {
    unsafe { ptr::addr_of_mut!((*self_thread()).errno) }
}

fn align_up(v: usize, a: usize) -> usize {
    (v + a - 1) & !(a - 1)
}

/// (offset of TP from the start of the area, total bytes, required alignment)
fn area_layout() -> (usize, usize, usize) {
    let t = TLS.get();
    let align = t.align.max(16);
    let tp_off = align_up(size_of::<Thread>(), align);
    let size = tp_off + align_up(TCB_SIZE, t.align) + t.memsz;
    (tp_off, size, align)
}

/// Lay out a fresh Thread + TLS block at `base` (aligned as area_layout says);
/// returns the thread pointer value.
unsafe fn init_area(base: usize) -> usize {
    let t = TLS.get();
    let (tp_off, _, _) = area_layout();
    let tp = base + tp_off;
    let th = (tp - size_of::<Thread>()) as *mut Thread;
    ptr::write_bytes(th as *mut u8, 0, size_of::<Thread>() + TCB_SIZE);
    let block = (tp + align_up(TCB_SIZE, t.align)) as *mut u8;
    ptr::copy_nonoverlapping(t.image as *const u8, block, t.filesz);
    ptr::write_bytes(block.add(t.filesz), 0, t.memsz - t.filesz);
    tp
}

unsafe fn mmap_rw(len: usize) -> usize {
    let r = crate::io::mmap_raw(0, len, libc::PROT_READ | libc::PROT_WRITE,
                                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, -1, 0);
    if crate::sys::is_err(r) { 0 } else { r as usize }
}

/// Startup: find PT_TLS, give the main thread its TLS block and thread pointer.
pub unsafe fn init_main_thread() {
    for p in phdrs() {
        if p.p_type == libc::PT_TLS {
            TLS.set(TlsImage {
                image: p.p_vaddr as usize + crate::start::load_bias(),
                filesz: p.p_filesz as usize,
                memsz: p.p_memsz as usize,
                align: (p.p_align as usize).max(1),
            });
        }
    }
    let (_, size, align) = area_layout();
    let base = mmap_rw(size + align);
    if base == 0 {
        sc!(libc::SYS_exit_group, 120);
    }
    let tp = init_area(align_up(base, align));
    if arch::set_tp(tp) < 0 {
        sc!(libc::SYS_exit_group, 121);
    }
    let th = self_thread();
    (*th).tid.store(sc!(libc::SYS_gettid) as i32, Ordering::Relaxed);
    // The main thread's stack belongs to the kernel; report the usual 8 MiB
    // below the top of the initial stack.
    let a = AUX.get();
    let hi = align_up(a.execfn.max(a.initial_sp), a.pagesz);
    (*th).stack_size = 8 << 20;
    (*th).stack_lo = hi - (*th).stack_size;
    (*th).guard_size = a.pagesz;
}

/// Called after fork() in the child.
pub unsafe fn after_fork_child() {
    (*self_thread()).tid.store(sc!(libc::SYS_gettid) as i32, Ordering::Relaxed);
}

// ---------- attributes: [stacksize, guardsize, stackaddr, flags] ----------

fn attr_words(a: *const pthread_attr_t) -> *mut usize {
    a as *mut usize
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_init(a: *mut pthread_attr_t) -> c_int {
    ptr::write_bytes(a as *mut u8, 0, size_of::<pthread_attr_t>());
    let w = attr_words(a);
    *w = DEFAULT_STACK;
    *w.add(1) = AUX.get().pagesz;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_destroy(_a: *mut pthread_attr_t) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_setstacksize(a: *mut pthread_attr_t, size: size_t) -> c_int {
    if size < libc::PTHREAD_STACK_MIN || size > usize::MAX / 4 {
        return libc::EINVAL;
    }
    *attr_words(a) = size;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getstack(a: *const pthread_attr_t, addr: *mut *mut c_void,
                                               size: *mut size_t) -> c_int {
    let w = attr_words(a);
    *addr = *w.add(2) as *mut c_void;
    *size = *w;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getguardsize(a: *const pthread_attr_t, g: *mut size_t) -> c_int {
    *g = *attr_words(a).add(1);
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_getattr_np(t: pthread_t, a: *mut pthread_attr_t) -> c_int {
    let th = t as *const Thread;
    ptr::write_bytes(a as *mut u8, 0, size_of::<pthread_attr_t>());
    let w = attr_words(a);
    *w = (*th).stack_size;
    *w.add(1) = (*th).guard_size;
    *w.add(2) = (*th).stack_lo;
    0
}

// ---------- create / exit / join / detach ----------

#[no_mangle]
pub extern "C" fn pthread_self() -> pthread_t {
    self_thread() as pthread_t
}

/// A finished thread's mapping of exactly `need` bytes, if one is free.
/// Also unmaps surplus finished mappings.
unsafe fn take_free(need: usize, guard: usize) -> usize {
    let mut found = 0;
    let mut surplus: [usize; 4] = [0; 4];
    let mut surplus_len: [usize; 4] = [0; 4];
    FREE.with(|head| {
        let mut prev: *mut *mut Thread = head;
        let mut n = 0;
        let mut cur = *head;
        let mut s = 0;
        while !cur.is_null() {
            let next = (*cur).next;
            let gone = (*cur).tid.load(Ordering::Acquire) == 0;
            if gone && found == 0 && (*cur).map_size == need && (*cur).guard_size == guard {
                found = (*cur).map_base;
                *prev = next;
            } else if gone && n >= MAX_CACHED && s < surplus.len() {
                surplus[s] = (*cur).map_base;
                surplus_len[s] = (*cur).map_size;
                s += 1;
                *prev = next;
            } else {
                n += 1;
                prev = ptr::addr_of_mut!((*cur).next);
            }
            cur = next;
        }
    });
    for i in 0..surplus.len() {
        if surplus[i] != 0 {
            sc!(libc::SYS_munmap, surplus[i], surplus_len[i]);
        }
    }
    found
}

unsafe fn put_free(th: *mut Thread) {
    FREE.with(|head| {
        (*th).next = *head;
        *head = th;
    });
}

extern "C" fn thread_entry(th: usize) -> usize {
    unsafe {
        let th = th as *mut Thread;
        let f: extern "C" fn(*mut c_void) -> *mut c_void = core::mem::transmute((*th).start);
        let r = f((*th).arg as *mut c_void);
        exit_thread(th, r as usize)
    }
}

unsafe fn run_key_dtors(th: *mut Thread) {
    for _ in 0..4 {
        let mut ran = false;
        for k in 0..KEYS_MAX {
            let v = (*th).keys[k];
            if v != 0 {
                (*th).keys[k] = 0;
                let d = KEY_DTORS[k].load(Ordering::Acquire);
                if d != 0 {
                    let d: extern "C" fn(*mut c_void) = core::mem::transmute(d);
                    d(v as *mut c_void);
                    ran = true;
                }
            }
        }
        if !ran {
            break;
        }
    }
}

unsafe fn exit_thread(th: *mut Thread, result: usize) -> ! {
    run_key_dtors(th);
    (*th).result = result;
    if (*th).state.compare_exchange(JOINABLE, EXITED, Ordering::AcqRel, Ordering::Acquire).is_err() {
        // Detached: nobody will join, so hand the mapping to the free list
        // now; it is only reused once the kernel has cleared our tid.
        put_free(th);
    }
    loop {
        sc!(libc::SYS_exit, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pthread_create(
    out: *mut pthread_t,
    attr: *const pthread_attr_t,
    start: extern "C" fn(*mut c_void) -> *mut c_void,
    arg: *mut c_void,
) -> c_int {
    let page = AUX.get().pagesz;
    let (mut stack, mut guard) = (DEFAULT_STACK, page);
    if !attr.is_null() {
        let w = attr_words(attr);
        stack = *w;
        guard = *w.add(1);
    }
    let stack = align_up(stack.max(libc::PTHREAD_STACK_MIN), page);
    let guard = align_up(guard, page);
    let (_, area, align) = area_layout();
    let need = guard + stack + align_up(area + align, page);
    let mut map = take_free(need, guard);
    if map == 0 {
        map = mmap_rw(need);
        if map == 0 {
            return libc::EAGAIN;
        }
        if guard > 0 {
            sc!(libc::SYS_mprotect, map, guard, libc::PROT_NONE);
        }
    }
    let stack_top = map + guard + stack;
    let tp = init_area(align_up(stack_top, align));
    let th = (tp - size_of::<Thread>()) as *mut Thread;
    (*th).map_base = map;
    (*th).map_size = need;
    (*th).stack_lo = map + guard;
    (*th).stack_size = stack;
    (*th).guard_size = guard;
    (*th).start = start as usize;
    (*th).arg = arg as usize;
    let flags = (libc::CLONE_VM | libc::CLONE_FS | libc::CLONE_FILES | libc::CLONE_SIGHAND
        | libc::CLONE_THREAD | libc::CLONE_SYSVSEM | libc::CLONE_SETTLS
        | libc::CLONE_PARENT_SETTID | libc::CLONE_CHILD_CLEARTID) as usize;
    let tid = (*th).tid.as_ptr();
    let r = arch::__nolibc_clone(flags, stack_top, tid, tp, tid, thread_entry, th as usize);
    if r < 0 {
        (*th).tid.store(0, Ordering::Release);
        put_free(th);
        return libc::EAGAIN;
    }
    *out = th as pthread_t;
    0
}

/// Wait until the kernel has cleared the thread's tid (it is fully gone).
unsafe fn wait_gone(th: *mut Thread) {
    loop {
        let tid = (*th).tid.load(Ordering::Acquire);
        if tid == 0 {
            return;
        }
        // The kernel's CLONE_CHILD_CLEARTID wake is a shared (non-private) futex.
        futex_wait(&*((*th).tid.as_ptr() as *const AtomicU32), tid as u32, false);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pthread_join(t: pthread_t, ret: *mut *mut c_void) -> c_int {
    let th = t as *mut Thread;
    if th == self_thread() {
        return libc::EDEADLK;
    }
    wait_gone(th);
    if !ret.is_null() {
        *ret = (*th).result as *mut c_void;
    }
    put_free(th);
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_detach(t: pthread_t) -> c_int {
    let th = t as *mut Thread;
    match (*th).state.compare_exchange(JOINABLE, DETACHED, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => 0,
        Err(EXITED) => {
            // Already finished and waiting to be joined: reclaim it now.
            wait_gone(th);
            put_free(th);
            0
        }
        Err(_) => libc::EINVAL,
    }
}

#[no_mangle]
pub unsafe extern "C" fn pthread_setname_np(t: pthread_t, name: *const c_char) -> c_int {
    if t as *mut Thread != self_thread() {
        return libc::ENOSYS;
    }
    crate::sys::ret_pthread(sc!(libc::SYS_prctl, libc::PR_SET_NAME, name, 0, 0, 0))
}

// ---------- keys ----------

#[no_mangle]
pub unsafe extern "C" fn pthread_key_create(key: *mut c_uint, dtor: Option<unsafe extern "C" fn(*mut c_void)>)
    -> c_int {
    for k in 0..KEYS_MAX {
        if KEY_USED[k].compare_exchange(0, 1, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            KEY_DTORS[k].store(dtor.map_or(0, |d| d as usize), Ordering::Release);
            *key = k as c_uint;
            return 0;
        }
    }
    libc::EAGAIN
}

#[no_mangle]
pub unsafe extern "C" fn pthread_key_delete(key: c_uint) -> c_int {
    if key as usize >= KEYS_MAX {
        return libc::EINVAL;
    }
    KEY_DTORS[key as usize].store(0, Ordering::Release);
    KEY_USED[key as usize].store(0, Ordering::Release);
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_setspecific(key: c_uint, v: *const c_void) -> c_int {
    if key as usize >= KEYS_MAX {
        return libc::EINVAL;
    }
    (*self_thread()).keys[key as usize] = v as usize;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_getspecific(key: c_uint) -> *mut c_void {
    if key as usize >= KEYS_MAX {
        return null_mut();
    }
    (*self_thread()).keys[key as usize] as *mut c_void
}
