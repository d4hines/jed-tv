//! Exact TEE_IOC_SHM_ALLOC backend. No TEE device is opened by this library.
use super::{Allocator, Region, ShmAllocRequest, SHM_ALLOC_IOCTL};
use std::{
    fs::File,
    io,
    os::fd::{AsRawFd, FromRawFd},
    os::raw::{c_int, c_ulong, c_void},
    ptr::NonNull,
};
extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap(
        address: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    fn munmap(address: *mut c_void, length: usize) -> c_int;
}
pub struct LinuxAllocator {
    device: File,
    max_bytes: usize,
}
impl LinuxAllocator {
    /// Caller supplies the already authorized TEE device FD from PID1. Passing
    /// a File does not change OP-TEE mode or initialize secure-world services.
    pub fn new(device: File, max_bytes: usize) -> Self {
        Self { device, max_bytes }
    }
}
pub struct Mapping {
    fd: File,
    address: NonNull<u8>,
    bytes: usize,
    id: i32,
}
impl Region for Mapping {
    fn id(&self) -> i32 {
        self.id
    }
    fn len(&self) -> usize {
        self.bytes
    }
    fn read(&self, offset: usize, data: &mut [u8]) -> io::Result<()> {
        if offset
            .checked_add(data.len())
            .filter(|&n| n <= self.bytes)
            .is_none()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "SHM read outside mapping",
            ));
        }
        for (i, byte) in data.iter_mut().enumerate() {
            *byte = unsafe { self.address.as_ptr().add(offset + i).read_volatile() };
        }
        Ok(())
    }
    fn write(&mut self, offset: usize, data: &[u8]) -> io::Result<()> {
        if offset
            .checked_add(data.len())
            .filter(|&n| n <= self.bytes)
            .is_none()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "SHM write outside mapping",
            ));
        }
        for (i, &byte) in data.iter().enumerate() {
            unsafe { self.address.as_ptr().add(offset + i).write_volatile(byte) }
        }
        Ok(())
    }
    fn clear(&mut self) -> io::Result<()> {
        for i in 0..self.bytes {
            unsafe { self.address.as_ptr().add(i).write_volatile(0) }
        }
        Ok(())
    }
}
impl Drop for Mapping {
    fn drop(&mut self) {
        unsafe {
            munmap(self.address.as_ptr().cast(), self.bytes);
        } // fd closes after unmap.
        let _ = &self.fd;
    }
}
impl Allocator for LinuxAllocator {
    type Region = Mapping;
    fn allocate(&mut self, bytes: usize) -> io::Result<Mapping> {
        if bytes == 0 || bytes > self.max_bytes {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "SHM allocation bound",
            ));
        }
        let mut req = ShmAllocRequest {
            size: bytes as u64,
            flags: 0,
            id: 0,
        };
        let raw = unsafe {
            ioctl(
                self.device.as_raw_fd(),
                SHM_ALLOC_IOCTL as c_ulong,
                &mut req as *mut ShmAllocRequest,
            )
        };
        if raw < 0 {
            return Err(io::Error::last_os_error());
        }
        let fd = unsafe { File::from_raw_fd(raw) };
        let size = usize::try_from(req.size)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "SHM size overflows usize"))?;
        if size < bytes || size > self.max_bytes || req.id < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid SHM allocation result",
            ));
        }
        let address = unsafe { mmap(std::ptr::null_mut(), size, 3, 1, fd.as_raw_fd(), 0) };
        if address as isize == -1 {
            return Err(io::Error::last_os_error());
        }
        let Some(address) = NonNull::new(address.cast()) else {
            unsafe {
                munmap(address, size);
            }
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected null SHM mapping",
            ));
        };
        Ok(Mapping {
            fd,
            address,
            bytes: size,
            id: req.id,
        })
    }
}
