//! ALSA PCM playback through raw ioctls (no alsa-lib): the audio half of the
//! platform API. Struct layouts follow include/uapi/sound/asound.h.

use crate::sys::{ioc, ioctl, IOC_NONE, IOC_READ, IOC_WRITE};
use libc::{c_long, c_ulong, c_void};
use std::io;
use std::os::fd::AsRawFd;

#[repr(C)]
#[derive(Clone, Copy)]
struct Mask {
    bits: [u32; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Interval {
    min: u32,
    max: u32,
    flags: u32, // bit0 openmin, bit1 openmax, bit2 integer, bit3 empty
}

#[repr(C)]
struct HwParams {
    flags: u32,
    masks: [Mask; 3],
    mres: [Mask; 5],
    intervals: [Interval; 12],
    ires: [Interval; 9],
    rmask: u32,
    cmask: u32,
    info: u32,
    msbits: u32,
    rate_num: u32,
    rate_den: u32,
    fifo_size: c_ulong,
    reserved: [u8; 64],
}

#[repr(C)]
struct XferI {
    result: c_long,
    buf: *mut c_void,
    frames: c_ulong,
}

// Parameter indices (SNDRV_PCM_HW_PARAM_*).
const ACCESS: usize = 0;
const FORMAT: usize = 1;
const SUBFORMAT: usize = 2;
const FIRST_INTERVAL: usize = 8;
const CHANNELS: usize = 10;
const RATE: usize = 11;
const PERIOD_SIZE: usize = 13;
const PERIODS: usize = 15;
const BUFFER_SIZE: usize = 17;

const ACCESS_RW_INTERLEAVED: u32 = 3;
const FORMAT_S16_LE: u32 = 2;
const SUBFORMAT_STD: u32 = 0;

pub const HW_PARAMS: u32 = ioc(IOC_READ | IOC_WRITE, b'A', 0x11, std::mem::size_of::<HwParams>());
pub const PREPARE: u32 = ioc(IOC_NONE, b'A', 0x40, 0);
pub const DRAIN: u32 = ioc(IOC_NONE, b'A', 0x44, 0);
pub const DROP: u32 = ioc(IOC_NONE, b'A', 0x43, 0);
pub const DELAY: u32 = ioc(IOC_READ, b'A', 0x21, std::mem::size_of::<c_long>());
pub const WRITEI_FRAMES: u32 = ioc(IOC_WRITE, b'A', 0x50, std::mem::size_of::<XferI>());

pub struct Pcm {
    native: Option<crate::native_audio::NativePcm>,
    file: std::fs::File,
    path: String,
    pub rate: u32,
    pub channels: u32,
    pub period_frames: u32,
    pub buffer_frames: u32,
    pub underruns: u32,
}

impl HwParams {
    fn any() -> Self {
        let mut p: HwParams = unsafe { std::mem::zeroed() };
        for m in &mut p.masks {
            m.bits = [!0; 8];
        }
        for i in &mut p.intervals {
            *i = Interval { min: 0, max: u32::MAX, flags: 0 };
        }
        p.rmask = !0;
        p.info = !0;
        p
    }
    fn set_mask(&mut self, idx: usize, val: u32) {
        let m = &mut self.masks[idx];
        m.bits = [0; 8];
        m.bits[(val / 32) as usize] = 1 << (val % 32);
    }
    fn set_range(&mut self, idx: usize, min: u32, max: u32) {
        self.intervals[idx - FIRST_INTERVAL] = Interval { min, max, flags: 0 };
    }
    fn get(&self, idx: usize) -> u32 {
        self.intervals[idx - FIRST_INTERVAL].min
    }
}

impl Pcm {
    /// Open an interleaved S16_LE playback stream at (or near) `rate`.
    pub fn open(path: &str, rate: u32, channels: u32) -> io::Result<Self> {
        if let Ok(address) = std::env::var("TV_AUDIO_ADDR") {
            let native = crate::native_audio::NativePcm::open(&address,rate,channels)?;
            return Ok(Self { native: Some(native), file: std::fs::File::open("/dev/null")?,
                path: path.into(), rate, channels, period_frames: rate/50,
                buffer_frames: rate/10, underruns: 0 });
        }
        let file = std::fs::OpenOptions::new().read(true).write(true).open(path)?;
        let fd = file.as_raw_fd();
        let mut p = HwParams::any();
        p.set_mask(ACCESS, ACCESS_RW_INTERLEAVED);
        p.set_mask(FORMAT, FORMAT_S16_LE);
        p.set_mask(SUBFORMAT, SUBFORMAT_STD);
        p.set_range(CHANNELS, channels, channels);
        p.set_range(RATE, rate, rate);
        // ~40-100 ms periods and a 200-500 ms buffer. With 10 ms periods, QEMU's
        // virtio-snd fed to macOS CoreAudio drained audio at ~1/3 real time (it
        // appears to pass one period per host callback), so video, which follows
        // the audio clock, crawled. Larger periods also mean fewer wakeups on the
        // TV. A/V sync is unaffected: the player measures the queue with DELAY.
        p.set_range(PERIOD_SIZE, rate / 25, rate / 10);
        p.set_range(PERIODS, 2, 16);
        p.set_range(BUFFER_SIZE, rate / 5, rate / 2);
        unsafe { ioctl(fd, HW_PARAMS, &mut p)? };
        unsafe { ioctl::<c_void>(fd, PREPARE, std::ptr::null_mut())? };
        Ok(Pcm {
            native: None,
            file,
            path: path.to_owned(),
            rate: p.get(RATE),
            channels: p.get(CHANNELS),
            period_frames: p.get(PERIOD_SIZE),
            buffer_frames: p.get(BUFFER_SIZE),
            underruns: 0,
        })
    }

    /// Blocking write of interleaved frames; recovers from underruns.
    pub fn write(&mut self, samples: &[i16]) -> io::Result<()> {
        if let Some(native) = &self.native { return native.write(samples); }
        let fd = self.file.as_raw_fd();
        let ch = self.channels as usize;
        let mut done = 0usize;
        while done < samples.len() / ch {
            let rest = &samples[done * ch..];
            let mut x = XferI { result: 0, buf: rest.as_ptr() as *mut c_void,
                                frames: (rest.len() / ch) as c_ulong };
            match unsafe { ioctl(fd, WRITEI_FRAMES, &mut x) } {
                Ok(_) => done += x.result as usize,
                Err(e) if e.raw_os_error() == Some(libc::EPIPE) => {
                    self.underruns += 1;
                    unsafe { ioctl::<c_void>(fd, PREPARE, std::ptr::null_mut())? };
                }
                Err(e) if e.raw_os_error() == Some(libc::EINTR) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Frames queued but not yet audible: the audio clock for A/V sync.
    pub fn delay(&self) -> io::Result<i64> {
        if let Some(native) = &self.native { return native.delay(); }
        let mut d: c_long = 0;
        unsafe { ioctl(self.file.as_raw_fd(), DELAY, &mut d)? };
        Ok(d as i64)
    }

    pub fn drain(&self) -> io::Result<()> {
        if let Some(native) = &self.native { return native.command(b'F'); }
        unsafe { ioctl::<c_void>(self.file.as_raw_fd(), DRAIN, std::ptr::null_mut())? };
        Ok(())
    }

    /// Close and reopen the device with the same settings: the most portable
    /// way to stop now and throw away queued audio (seek/pause, or recovery
    /// from a stuck stream). QEMU's virtio-snd stalled after DROP+PREPARE on a
    /// running stream; a fresh open always starts clean.
    pub fn reset(&mut self) -> io::Result<()> {
        if let Some(native) = &self.native { return native.command(b'R'); }
        let underruns = self.underruns;
        // Close first: exclusive hardware devices refuse a second open.
        drop(std::mem::replace(&mut self.file, std::fs::File::open("/dev/null")?));
        let path = self.path.clone();
        *self = Pcm::open(&path, self.rate, self.channels)?;
        self.underruns = underruns;
        Ok(())
    }

    /// Stop now and discard queued frames, then get ready to play again:
    /// DROP followed by PREPARE.
    pub fn discard(&self) -> io::Result<()> {
        if let Some(native) = &self.native { return native.command(b'R'); }
        let fd = self.file.as_raw_fd();
        unsafe {
            ioctl::<c_void>(fd, DROP, std::ptr::null_mut())?;
            ioctl::<c_void>(fd, PREPARE, std::ptr::null_mut())?;
        }
        Ok(())
    }
}
