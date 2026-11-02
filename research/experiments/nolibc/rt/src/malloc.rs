//! malloc/free/calloc/realloc/posix_memalign over dlmalloc (Rust port), one
//! global futex lock (like musl's allocators), pages from mmap/mremap/munmap.

use crate::sync::{Lock, Locked};
use core::ptr::null_mut;
use libc::{c_int, c_void, size_t};

struct Pages;

unsafe impl dlmalloc::Allocator for Pages {
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        let r = unsafe {
            crate::io::mmap_raw(0, size, libc::PROT_READ | libc::PROT_WRITE,
                                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, -1, 0)
        };
        if crate::sys::is_err(r) { (null_mut(), 0, 0) } else { (r as *mut u8, size, 0) }
    }

    fn remap(&self, ptr: *mut u8, old: usize, new: usize, can_move: bool) -> *mut u8 {
        let flags = if can_move { libc::MREMAP_MAYMOVE } else { 0 };
        let r = unsafe { sc!(libc::SYS_mremap, ptr, old, new, flags) };
        if crate::sys::is_err(r) { null_mut() } else { r as *mut u8 }
    }

    fn free_part(&self, ptr: *mut u8, old: usize, new: usize) -> bool {
        unsafe {
            if !crate::sys::is_err(sc!(libc::SYS_mremap, ptr, old, new, 0)) {
                return true;
            }
            sc!(libc::SYS_munmap, ptr.add(new), old - new) == 0
        }
    }

    fn free(&self, ptr: *mut u8, size: usize) -> bool {
        unsafe { sc!(libc::SYS_munmap, ptr, size) == 0 }
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        true
    }

    fn allocates_zeros(&self) -> bool {
        true
    }

    fn page_size(&self) -> usize {
        4096
    }
}

static HEAP: Locked<dlmalloc::Dlmalloc<Pages>> = Locked::new(dlmalloc::Dlmalloc::new_with_allocator(Pages));

/// The allocator's lock, for fork().
pub fn lock() -> &'static Lock {
    HEAP.raw_lock()
}

#[no_mangle]
pub unsafe extern "C" fn malloc(n: size_t) -> *mut c_void {
    let p = HEAP.with(|h| h.c_malloc(n)) as *mut c_void;
    if p.is_null() {
        crate::sys::set_errno(libc::ENOMEM);
    }
    p
}

#[no_mangle]
pub unsafe extern "C" fn calloc(n: size_t, size: size_t) -> *mut c_void {
    let Some(total) = n.checked_mul(size) else {
        crate::sys::set_errno(libc::ENOMEM);
        return null_mut();
    };
    let p = HEAP.with(|h| h.calloc(total, 1)) as *mut c_void;
    if p.is_null() {
        crate::sys::set_errno(libc::ENOMEM);
    }
    p
}

#[no_mangle]
pub unsafe extern "C" fn realloc(p: *mut c_void, n: size_t) -> *mut c_void {
    let q = HEAP.with(|h| h.c_realloc(p as *mut u8, n)) as *mut c_void;
    if q.is_null() {
        crate::sys::set_errno(libc::ENOMEM);
    }
    q
}

#[no_mangle]
pub unsafe extern "C" fn free(p: *mut c_void) {
    if !p.is_null() {
        HEAP.with(|h| h.c_free(p as *mut u8));
    }
}

#[no_mangle]
pub unsafe extern "C" fn posix_memalign(out: *mut *mut c_void, align: size_t, n: size_t) -> c_int {
    if !align.is_power_of_two() || align % core::mem::size_of::<usize>() != 0 {
        return libc::EINVAL;
    }
    let p = HEAP.with(|h| h.c_memalign(align, n)) as *mut c_void;
    if p.is_null() {
        return libc::ENOMEM;
    }
    *out = p;
    0
}

#[no_mangle]
pub unsafe extern "C" fn aligned_alloc(align: size_t, n: size_t) -> *mut c_void {
    let mut p = null_mut();
    match posix_memalign(&mut p, align.max(core::mem::size_of::<usize>()), n) {
        0 => p,
        e => {
            crate::sys::set_errno(e);
            null_mut()
        }
    }
}
