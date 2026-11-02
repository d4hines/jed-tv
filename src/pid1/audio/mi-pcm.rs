//! Exact V109 MI PCM wire adapter. All ownership belongs to its caller (PID1).
use std::{
    fmt,
    fs::{File, OpenOptions},
    io,
    os::fd::AsRawFd,
    thread,
    time::{Duration, Instant},
};

#[repr(C)]
struct Envelope {
    size: u16,
    pad: [u8; 6],
    pointer: u64,
}
#[repr(C)]
struct OpenWire {
    handle: u32,
    name: [u8; 64],
    blocking: u8,
    mixing: u8,
    pad: [u8; 2],
    rate: u32,
    channels: u8,
    bits: u8,
    big_endian: u8,
    format_pad: u8,
    channel_map: [u32; 10],
    duration_control: u8,
    duration_pad: [u8; 3],
    duration_ms: u32,
}
#[repr(C)]
struct WriteWire {
    handle: u32,
    pad0: u32,
    buffer: u64,
    bytes: u32,
    pad1: u32,
    pts: u64,
    stream_flags: u64,
    metadata_index: u64,
    written: u32,
    pad2: u32,
}
#[repr(C)]
struct GetAttr {
    handle: u32,
    attr: u32,
    input: u64,
    output: u64,
}
const _: () = {
    assert!(std::mem::size_of::<Envelope>() == 16);
    assert!(std::mem::offset_of!(Envelope, pointer) == 8);
    assert!(std::mem::size_of::<OpenWire>() == 128);
    assert!(std::mem::offset_of!(OpenWire, blocking) == 68);
    assert!(std::mem::offset_of!(OpenWire, rate) == 72);
    assert!(std::mem::offset_of!(OpenWire, channel_map) == 80);
    assert!(std::mem::offset_of!(OpenWire, duration_ms) == 124);
    assert!(std::mem::size_of::<WriteWire>() == 56);
    assert!(std::mem::offset_of!(WriteWire, buffer) == 8);
    assert!(std::mem::offset_of!(WriteWire, pts) == 24);
    assert!(std::mem::offset_of!(WriteWire, written) == 48);
    assert!(std::mem::size_of::<GetAttr>() == 24);
};
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct BufferInfo {
    pub capacity_bytes: u32,
    pub queued_bytes: u32,
    pub free_bytes: u32,
}
unsafe extern "C" {
    fn ioctl(fd: i32, request: usize, ...) -> i32;
}

/// A transport must complete synchronously; it must not retain the wire slice
/// or embedded buffer pointers after returning. This seam also supports offline tests.
pub trait Transport {
    fn transact(&self, request: u32, wire: &mut [u8]) -> io::Result<()>;
}
impl<T: Transport> Transport for std::sync::Arc<T> {
    fn transact(&self, request: u32, wire: &mut [u8]) -> io::Result<()> {
        (**self).transact(request, wire)
    }
}

pub struct DeviceTransport(File);
impl DeviceTransport {
    /// Reuse the exact MI envelope on a device file opened by the owning PID1.
    pub fn from_file(file: File) -> Self {
        Self(file)
    }
}
#[derive(Debug)]
pub struct VendorError {
    pub request: u32,
    pub status: i32,
}
impl fmt::Display for VendorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MI request {:#x}: status {}", self.request, self.status)
    }
}
impl std::error::Error for VendorError {}
fn vendor_status(request: u32, status: i32) -> io::Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(io::Error::other(VendorError { request, status }))
    }
}
impl Transport for DeviceTransport {
    fn transact(&self, request: u32, wire: &mut [u8]) -> io::Result<()> {
        let mut envelope = Envelope {
            size: wire.len() as u16,
            pad: [0; 6],
            pointer: wire.as_mut_ptr() as usize as u64,
        };
        let status = unsafe { ioctl(self.0.as_raw_fd(), request as usize, &mut envelope) };
        if status < 0 {
            Err(io::Error::last_os_error())
        } else {
            vendor_status(request, status)
        }
    }
}
fn call<T>(transport: &impl Transport, request: u32, arg: &mut T) -> io::Result<()> {
    // All wire types used here have explicit padding initialized by their constructors.
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(arg as *mut T as *mut u8, std::mem::size_of::<T>())
    };
    transport.transact(request, bytes)
}

#[derive(Debug)]
pub struct CleanupError {
    pub stop: io::Error,
    pub close: io::Error,
}
impl fmt::Display for CleanupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PCM stop failed: {}; close also failed: {}",
            self.stop, self.close
        )
    }
}
impl std::error::Error for CleanupError {}

pub struct Pcm<T: Transport = DeviceTransport> {
    transport: T,
    handle: u32,
    started: bool,
}
impl Pcm<DeviceTransport> {
    /// Opens only the normal stock HAL endpoint, with its exact 48k stereo config.
    /// Call only after the coordinator has quiesced other audio owners.
    pub fn open() -> io::Result<Self> {
        assert_eq!(std::mem::size_of::<Envelope>(), 16);
        assert_eq!(std::mem::size_of::<OpenWire>(), 128);
        assert_eq!(std::mem::size_of::<WriteWire>(), 56);
        assert_eq!(std::mem::size_of::<GetAttr>(), 24);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mik/pcm")
            .or_else(|_| {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/mik!pcm")
            })?;
        Self::open_with(DeviceTransport(file))
    }
}
impl<T: Transport> Pcm<T> {
    pub fn open_with(transport: T) -> io::Result<Self> {
        let mut arg = OpenWire {
            handle: 0,
            name: [0; 64],
            blocking: 0,
            mixing: 0,
            pad: [0; 2],
            rate: 48000,
            channels: 2,
            bits: 16,
            big_endian: 0,
            format_pad: 0,
            channel_map: [0; 10],
            duration_control: 1,
            duration_pad: [0; 3],
            duration_ms: 100,
        };
        arg.name[..14].copy_from_slice(b"HW DMA Reader1");
        call(&transport, 0xc0803402, &mut arg)?;
        // Successful Open owns this handle; failed Open must never close an existing one.
        if arg.handle == 0 {
            return Err(io::Error::other("MI PCM Open returned a zero handle"));
        }
        Ok(Self {
            transport,
            handle: arg.handle,
            started: false,
        })
    }
    pub fn handle(&self) -> u32 {
        self.handle
    }
    pub fn start(&mut self) -> io::Result<()> {
        if self.started {
            return Ok(());
        }
        call(&self.transport, 0xc0083404, &mut [self.handle, 0u32])?;
        self.started = true;
        Ok(())
    }
    pub fn buffer_info(&self) -> io::Result<BufferInfo> {
        let mut buffer = BufferInfo::default();
        let mut arg = GetAttr {
            handle: self.handle,
            attr: 0,
            input: 0,
            output: &mut buffer as *mut BufferInfo as usize as u64,
        };
        call(&self.transport, 0xc018340b, &mut arg)?;
        Ok(buffer)
    }
    pub fn write(&mut self, pcm: &[u8]) -> io::Result<usize> {
        if !self.started {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "PCM stream has not started",
            ));
        }
        if pcm.is_empty() {
            return Ok(0);
        }
        if pcm.len() % 4 != 0 || pcm.len() > u32::MAX as usize {
            return Err(io::Error::other("unaligned/oversized PCM"));
        }
        let mut arg = WriteWire {
            handle: self.handle,
            pad0: 0,
            buffer: pcm.as_ptr() as usize as u64,
            bytes: pcm.len() as u32,
            pad1: 0,
            pts: 0,
            stream_flags: 0,
            metadata_index: 0,
            written: 0,
            pad2: 0,
        };
        call(&self.transport, 0xc0383408, &mut arg)?;
        if arg.written > arg.bytes || arg.written % 4 != 0 {
            return Err(io::Error::other("invalid MI write count"));
        }
        Ok(arg.written as usize)
    }
    /// Writes the whole frame-aligned buffer with one shared deadline, including
    /// positive short writes. Timeout errors may follow a partially accepted buffer.
    pub fn write_all_until(&mut self, pcm: &[u8], deadline: Instant) -> io::Result<()> {
        self.write_all_with(
            pcm,
            || {
                if Instant::now() >= deadline {
                    Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "PCM write deadline",
                    ))
                } else {
                    Ok(())
                }
            },
            || thread::sleep(Duration::from_millis(5)),
        )
    }
    fn write_all_with(
        &mut self,
        mut pcm: &[u8],
        mut before: impl FnMut() -> io::Result<()>,
        mut stall: impl FnMut(),
    ) -> io::Result<()> {
        if pcm.len() % 4 != 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "unaligned PCM"));
        }
        while !pcm.is_empty() {
            before()?;
            let n = self.write(pcm)?;
            pcm = &pcm[n..];
            if n == 0 {
                stall()
            }
        }
        Ok(())
    }
    pub fn finish(mut self) -> io::Result<()> {
        self.cleanup()
    }
    fn cleanup(&mut self) -> io::Result<()> {
        if self.handle == 0 {
            return Ok(());
        }
        // Keep our handle immutable across in/out ioctls. A driver may modify
        // its wire argument even on error; Close still needs our original handle.
        let handle = self.handle;
        self.handle = 0;
        let stop = if self.started {
            self.started = false;
            let mut wire_handle = handle;
            call(&self.transport, 0xc0043405, &mut wire_handle)
        } else {
            Ok(())
        };
        // Attempt Close even if Stop fails, exactly once; a failed Close leaves
        // kernel ownership uncertain and is reported, never blindly retried by Drop.
        let mut wire_handle = handle;
        let close = call(&self.transport, 0xc0043403, &mut wire_handle);
        match (stop, close) {
            (Ok(()), Ok(())) => Ok(()),
            (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
            (Err(stop), Err(close)) => Err(io::Error::other(CleanupError { stop, close })),
        }
    }
}
impl<T: Transport> Drop for Pcm<T> {
    fn drop(&mut self) {
        if let Err(error) = self.cleanup() {
            eprintln!("MI PCM cleanup failed: {error}")
        }
    }
}

#[cfg(test)]
#[path = "../../../test/unit/audio/mi-pcm-tests.rs"]
mod tests;
