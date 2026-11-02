//! Compile-only probe: origin's program start and threads in a staticlib.
#![no_std]
extern crate alloc;
extern crate origin;

use core::sync::atomic::{AtomicUsize, Ordering};

struct Bump;
static NEXT: AtomicUsize = AtomicUsize::new(0);
static mut HEAP: [u8; 1 << 16] = [0; 1 << 16];
unsafe impl core::alloc::GlobalAlloc for Bump {
    unsafe fn alloc(&self, l: core::alloc::Layout) -> *mut u8 {
        let off = NEXT.fetch_add(l.size() + l.align(), Ordering::Relaxed);
        let p = (&raw mut HEAP as *mut u8).add(off);
        p.add(p.align_offset(l.align()))
    }
    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {}
}
#[global_allocator]
static A: Bump = Bump;

#[no_mangle]
unsafe fn origin_main(_argc: usize, _argv: *mut *mut u8, _envp: *mut *mut u8) -> i32 {
    let t = origin::thread::create(|_| None, &[], origin::thread::default_stack_size(),
                                   origin::thread::default_guard_size()).unwrap();
    origin::thread::join(t);
    0
}
