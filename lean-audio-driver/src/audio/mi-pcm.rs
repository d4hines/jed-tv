//! Exact V109 MI PCM wire adapter: the ioctl envelopes and one function per
//! request. Nothing here owns a handle or decides anything; the Lean driver
//! (src/lean/TV/Audio) does, and reads every raw value these return.
use std::{
    fmt,
    fs::{File, OpenOptions},
    io,
    os::fd::AsRawFd,
};

pub const OPEN: u32 = 0xc0803402;
pub const START: u32 = 0xc0083404;
pub const GET_ATTR: u32 = 0xc018340b;
pub const WRITE: u32 = 0xc0383408;
pub const STOP: u32 = 0xc0043405;
pub const CLOSE: u32 = 0xc0043403;

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
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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

pub struct DeviceTransport(File);
impl DeviceTransport {
    /// The stock HAL's PCM device node.
    pub fn open_pcm() -> io::Result<Self> {
        Self::open_node("/dev/mik/pcm", "/dev/mik!pcm")
    }
    /// The stock HAL's AOUT device node; the speaker output already exists on it.
    pub fn open_aout() -> io::Result<Self> {
        Self::open_node("/dev/mik/aout", "/dev/mik!aout")
    }
    fn open_node(first: &str, second: &str) -> io::Result<Self> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(first)
            .or_else(|_| OpenOptions::new().read(true).write(true).open(second))
            .map(Self)
    }
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
        } else if status == 0 {
            Ok(())
        } else {
            Err(io::Error::other(VendorError { request, status }))
        }
    }
}
pub(crate) fn call<T>(transport: &impl Transport, request: u32, arg: &mut T) -> io::Result<()> {
    // All wire types used here have explicit padding initialized by their constructors.
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(arg as *mut T as *mut u8, std::mem::size_of::<T>())
    };
    transport.transact(request, bytes)
}

/// MI_PCM_Open on the normal stock HAL endpoint, 48 kHz stereo 16-bit; returns
/// the raw handle (a zero handle is for the caller to reject).
pub fn open(transport: &impl Transport) -> io::Result<u32> {
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
    call(transport, OPEN, &mut arg)?;
    Ok(arg.handle)
}
pub fn start(transport: &impl Transport, handle: u32) -> io::Result<()> {
    call(transport, START, &mut [handle, 0u32])
}
/// Returns the raw accepted byte count; the caller checks it against `pcm.len()`.
pub fn write(transport: &impl Transport, handle: u32, pcm: &[u8]) -> io::Result<u32> {
    let mut arg = WriteWire {
        handle,
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
    call(transport, WRITE, &mut arg)?;
    Ok(arg.written)
}
pub fn buffer_info(transport: &impl Transport, handle: u32) -> io::Result<BufferInfo> {
    let mut buffer = BufferInfo::default();
    let mut arg = GetAttr {
        handle,
        attr: 0,
        input: 0,
        output: &mut buffer as *mut BufferInfo as usize as u64,
    };
    call(transport, GET_ATTR, &mut arg)?;
    Ok(buffer)
}
pub fn stop(transport: &impl Transport, handle: u32) -> io::Result<()> {
    // The driver may modify its wire argument even on error; a copy keeps our handle.
    let mut wire = handle;
    call(transport, STOP, &mut wire)
}
pub fn close(transport: &impl Transport, handle: u32) -> io::Result<()> {
    let mut wire = handle;
    call(transport, CLOSE, &mut wire)
}

#[cfg(test)]
#[path = "../../test/unit/audio/mi-pcm-tests.rs"]
mod tests;
