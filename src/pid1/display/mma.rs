//! Owned uncached DMA-BUF for direct OSD. Allocation alone never publishes pixels.
use crate::osd_abi;
use std::{
    ffi::c_void,
    fs::{File, OpenOptions},
    io,
    os::fd::{AsRawFd, FromRawFd},
    ptr::NonNull,
};

#[cfg(target_os = "android")]
type MmapOffset = i32;
#[cfg(not(target_os = "android"))]
type MmapOffset = i64;
extern "C" {
    fn ioctl(fd: i32, request: u32, argument: *mut c_void) -> i32;
    fn mmap(
        address: *mut c_void,
        length: usize,
        prot: i32,
        flags: i32,
        fd: i32,
        offset: MmapOffset,
    ) -> *mut c_void;
    fn munmap(address: *mut c_void, length: usize) -> i32;
}

/// This allocation has no MI surface/window consumer. Keep it unpublished so
/// dropping the last descriptor cannot invalidate hardware scanout.
pub struct UnpublishedBuffer {
    memory: File,
    pointer: NonNull<u32>,
    address: u64,
}
impl UnpublishedBuffer {
    pub fn allocate() -> io::Result<Self> {
        let device = OpenOptions::new().read(true).write(true).open("/dev/mma")?;
        let mut wire = osd_abi::allocation();
        if unsafe {
            ioctl(
                device.as_raw_fd(),
                osd_abi::MMA_ALLOC,
                wire.as_mut_ptr().cast(),
            )
        } != 0
        {
            return Err(io::Error::last_os_error());
        }
        // Adopt the returned descriptor before validating the remaining reply.
        // Successful allocation transfers this reference to the caller.
        if wire[9] > i32::MAX as u32 {
            return Err(io::Error::other(
                "MMA did not return a valid DMA-BUF descriptor",
            ));
        }
        let memory = unsafe { File::from_raw_fd(wire[9] as i32) };
        let (address, _) = osd_abi::allocation_result(&wire).map_err(io::Error::other)?;
        if memory.metadata()?.len() != osd_abi::ALLOCATION_BYTES as u64 {
            return Err(io::Error::other("MMA DMA-BUF length mismatch"));
        }
        let mut cache = [0u32; 10];
        cache[0] = memory.as_raw_fd() as u32;
        if unsafe { ioctl(device.as_raw_fd(), 0x40284d06, cache.as_mut_ptr().cast()) } < 0 {
            return Err(io::Error::last_os_error());
        }
        let length = osd_abi::ALLOCATION_BYTES as usize;
        let mapped = unsafe { mmap(std::ptr::null_mut(), length, 3, 1, memory.as_raw_fd(), 0) };
        if mapped as isize == -1 {
            return Err(io::Error::last_os_error());
        }
        let Some(pointer) = NonNull::new(mapped.cast::<u32>()) else {
            unsafe {
                munmap(mapped, length);
            }
            return Err(io::Error::other("MMA mapping returned null"));
        };
        Ok(Self {
            memory,
            pointer,
            address,
        })
    }

    pub fn address(&self) -> u64 {
        self.address
    }
    pub fn descriptor(&self) -> i32 {
        self.memory.as_raw_fd()
    }

    pub(crate) fn write_rgba_page(&mut self, page: u32, pixels: &[u32]) -> io::Result<()> {
        let words = osd_abi::PAGE_BYTES as usize / 4;
        if page >= 2 || pixels.len() != words {
            return Err(io::Error::other("invalid MMA page or pixel count"));
        }
        for (index, &rgba) in pixels.iter().enumerate() {
            let argb = (rgba & 0xff00ff00) | ((rgba & 0xff) << 16) | ((rgba >> 16) & 0xff);
            unsafe {
                self.pointer
                    .as_ptr()
                    .add(page as usize * words + index)
                    .write_volatile(argb);
            }
        }
        std::sync::atomic::fence(std::sync::atomic::Ordering::Release);
        Ok(())
    }

    /// Verify both page ends in this private allocation; no OSD handles exist.
    pub fn check_pages(&mut self) -> io::Result<()> {
        let words = osd_abi::PAGE_BYTES as usize / 4;
        for index in [0, words - 1, words, words * 2 - 1] {
            let value = 0xa5a50000 ^ index as u32;
            unsafe {
                let slot = self.pointer.as_ptr().add(index);
                slot.write_volatile(value);
                if slot.read_volatile() != value {
                    return Err(io::Error::other("MMA private page readback mismatch"));
                }
                slot.write_volatile(0);
            }
        }
        Ok(())
    }
}
impl Drop for UnpublishedBuffer {
    fn drop(&mut self) {
        unsafe {
            munmap(
                self.pointer.as_ptr().cast(),
                osd_abi::ALLOCATION_BYTES as usize,
            );
        }
    }
}
