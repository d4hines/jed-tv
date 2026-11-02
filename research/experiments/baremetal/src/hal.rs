//! Heap and the virtio-drivers HAL.
//!
//! RAM is identity mapped (VA == PA) and QEMU's virtio devices are cache
//! coherent, so DMA memory is just zeroed, page-aligned heap memory and
//! share/unshare are no-ops. On a non-coherent SoC these two hooks are where
//! cache clean (before device reads) and invalidate (after device writes)
//! by virtual address would go.

use alloc::alloc::{alloc_zeroed, dealloc, Layout};
use core::ptr::NonNull;
use linked_list_allocator::LockedHeap;
use virtio_drivers::{BufferDirection, Hal, PhysAddr, PAGE_SIZE};

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

unsafe extern "C" {
    static __heap_start: u8;
    static __image_start: u8;
}

/// Heap from the end of the image to `ram_end` (from the DTB memory node).
pub fn init_heap(ram_end: usize) -> (usize, usize) {
    let start = &raw const __heap_start as usize;
    let size = ram_end - start;
    unsafe { HEAP.lock().init(start as *mut u8, size) };
    (start, size)
}

pub fn image_start() -> usize {
    &raw const __image_start as usize
}

pub fn heap_used() -> usize {
    HEAP.lock().used()
}

pub struct HalImpl;

fn dma_layout(pages: usize) -> Layout {
    Layout::from_size_align(pages * PAGE_SIZE, PAGE_SIZE).unwrap()
}

unsafe impl Hal for HalImpl {
    fn dma_alloc(pages: usize, _dir: BufferDirection) -> (PhysAddr, NonNull<u8>) {
        let p = unsafe { alloc_zeroed(dma_layout(pages)) };
        let p = NonNull::new(p).expect("DMA allocation failed");
        (p.as_ptr() as usize as PhysAddr, p)
    }

    unsafe fn dma_dealloc(_paddr: PhysAddr, vaddr: NonNull<u8>, pages: usize) -> i32 {
        unsafe { dealloc(vaddr.as_ptr(), dma_layout(pages)) };
        0
    }

    unsafe fn mmio_phys_to_virt(paddr: PhysAddr, _size: usize) -> NonNull<u8> {
        NonNull::new(paddr as usize as *mut u8).unwrap()
    }

    unsafe fn share(buffer: NonNull<[u8]>, _dir: BufferDirection) -> PhysAddr {
        buffer.as_ptr() as *mut u8 as usize as PhysAddr
    }

    unsafe fn unshare(_paddr: PhysAddr, _buffer: NonNull<[u8]>, _dir: BufferDirection) {}
}
