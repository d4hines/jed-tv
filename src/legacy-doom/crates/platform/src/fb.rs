//! Linux fbdev output: the display half of the platform API.
//! Same interface on QEMU (virtio-gpu fbdev emulation) and the TV (mstar_fbdev_mi).

use crate::sys::{ioc, ioctl, IOC_WRITE};
use libc::{c_ulong, c_void};
use std::io;
use std::os::fd::AsRawFd;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Bitfield {
    pub offset: u32,
    pub length: u32,
    pub msb_right: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct VarInfo {
    pub xres: u32,
    pub yres: u32,
    pub xres_virtual: u32,
    pub yres_virtual: u32,
    pub xoffset: u32,
    pub yoffset: u32,
    pub bits_per_pixel: u32,
    pub grayscale: u32,
    pub red: Bitfield,
    pub green: Bitfield,
    pub blue: Bitfield,
    pub transp: Bitfield,
    pub nonstd: u32,
    pub activate: u32,
    pub height: u32,
    pub width: u32,
    pub accel_flags: u32,
    pub pixclock: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub sync: u32,
    pub vmode: u32,
    pub rotate: u32,
    pub colorspace: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FixInfo {
    pub id: [u8; 16],
    pub smem_start: c_ulong,
    pub smem_len: u32,
    pub type_: u32,
    pub type_aux: u32,
    pub visual: u32,
    pub xpanstep: u16,
    pub ypanstep: u16,
    pub ywrapstep: u16,
    pub line_length: u32,
    pub mmio_start: c_ulong,
    pub mmio_len: u32,
    pub accel: u32,
    pub capabilities: u16,
    pub reserved: [u16; 2],
}

pub const FBIOGET_VSCREENINFO: u32 = 0x4600;
pub const FBIOGET_FSCREENINFO: u32 = 0x4602;
pub const FBIOPAN_DISPLAY: u32 = 0x4606;
pub const FBIO_WAITFORVSYNC: u32 = ioc(IOC_WRITE, b'F', 0x20, 4);

pub struct Framebuffer {
    file: std::fs::File,
    _memory_file: Option<std::fs::File>,
    pub var: VarInfo,
    pub fix: FixInfo,
    mem: *mut u8,
    pub pages: u32,
    back: u32,
    pub vsync_ok: bool,
    stream: bool,
}

impl Framebuffer {
    pub fn open(path: &str) -> io::Result<Self> {
        let file = std::fs::OpenOptions::new().read(true).write(true).open(path)?;
        Self::from_file(file, None)
    }

    /// Keep fbdev for mode/pan ioctls, optionally map its exported DMA buffer.
    pub fn from_file(file: std::fs::File, memory_file: Option<std::fs::File>) -> io::Result<Self> {
        let fd = file.as_raw_fd();
        let mut var = VarInfo::default();
        let mut fix: FixInfo = unsafe { std::mem::zeroed() };
        unsafe {
            ioctl(fd, FBIOGET_VSCREENINFO, &mut var)?;
            ioctl(fd, FBIOGET_FSCREENINFO, &mut fix)?;
        }
        if var.bits_per_pixel != 32 {
            return Err(io::Error::other(format!("unsupported bpp {}", var.bits_per_pixel)));
        }
        let frame = fix.line_length.checked_mul(var.yres)
            .ok_or_else(|| io::Error::other("framebuffer size overflow"))?;
        if var.xres == 0 || var.yres == 0 || var.xres > fix.line_length / 4
            || fix.line_length % 4 != 0 || frame == 0 || frame > fix.smem_len
            || var.yres_virtual < var.yres {
            return Err(io::Error::other("invalid framebuffer dimensions or backing memory"));
        }
        let mem = unsafe {
            libc::mmap(std::ptr::null_mut(), fix.smem_len as usize,
                       libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED,
                       memory_file.as_ref().map_or(fd, AsRawFd::as_raw_fd), 0)
        };
        if mem == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }
        let pages = (fix.smem_len / frame).min(var.yres_virtual / var.yres).max(1);
        let mut fb = Framebuffer { file, _memory_file: memory_file, var, fix, mem: mem as *mut u8, pages,
                                   back: if pages > 1 { 1 } else { 0 }, vsync_ok: false, stream: false };
        // Some drivers (virtio-gpu) accept FBIO_WAITFORVSYNC but return at once.
        // Trust it only if two back-to-back waits take a plausible frame time.
        let t0 = crate::sys::monotonic();
        let mut zero = 0u32;
        let ok = (0..2).all(|_| unsafe { ioctl(fd, FBIO_WAITFORVSYNC, &mut zero) }.is_ok());
        fb.vsync_ok = ok && (crate::sys::monotonic() - t0).as_micros() > 8_000;
        Ok(fb)
    }

    /// Two RGBA pages in a RAM file for a separate SurfaceFlinger presenter.
    pub fn shared_file(path: &str, width: u32, height: u32) -> io::Result<Self> {
        let bytes = width.checked_mul(height).and_then(|n| n.checked_mul(8))
            .and_then(|n| n.checked_add(4096)).filter(|_| width > 0 && height > 0)
            .ok_or_else(|| io::Error::other("invalid stream dimensions"))?;
        let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(path)?;
        file.set_len(bytes as u64)?;
        let mem = unsafe { libc::mmap(std::ptr::null_mut(), bytes as usize,
            libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, file.as_raw_fd(), 0) };
        if mem == libc::MAP_FAILED { return Err(io::Error::last_os_error()); }
        let mut var = VarInfo::default();
        var.xres = width; var.yres = height; var.xres_virtual = width;
        var.yres_virtual = height * 2; var.bits_per_pixel = 32;
        var.red = Bitfield { offset: 0, length: 8, msb_right: 0 };
        var.green = Bitfield { offset: 8, length: 8, msb_right: 0 };
        var.blue = Bitfield { offset: 16, length: 8, msb_right: 0 };
        var.transp = Bitfield { offset: 24, length: 8, msb_right: 0 };
        let mut fix: FixInfo = unsafe { std::mem::zeroed() };
        fix.id[..9].copy_from_slice(b"ramstream"); fix.line_length = width * 4; fix.smem_len = bytes;
        let header = mem as *mut u32;
        for (i, value) in [0x31465654, width, height, width * 4, 0, 0].iter().enumerate() {
            unsafe { header.add(i).write_volatile(*value); }
        }
        Ok(Self { file, _memory_file: None, var, fix, mem: mem as *mut u8,
            pages: 2, back: 0, vsync_ok: false, stream: true })
    }

    pub fn id(&self) -> String {
        String::from_utf8_lossy(&self.fix.id).trim_end_matches('\0').to_owned()
    }

    /// Pack 8-bit RGB into this framebuffer's pixel layout.
    #[inline]
    pub fn pixel(&self, r: u8, g: u8, b: u8) -> u32 {
        let v = &self.var;
        let put = |c: u8, f: &Bitfield| ((c as u32) >> (8 - f.length.min(8))) << f.offset;
        put(r, &v.red) | put(g, &v.green) | put(b, &v.blue)
            | if v.transp.length > 0 { ((1u32 << v.transp.length) - 1) << v.transp.offset } else { 0 }
    }

    /// The page not currently on screen (or the only page), as rows of u32.
    pub fn back_buffer(&mut self) -> BackBuffer<'_> {
        let stride = (self.fix.line_length / 4) as usize;
        let base = unsafe { self.mem.add((self.back * self.fix.line_length * self.var.yres) as usize + if self.stream { 4096 } else { 0 }) };
        BackBuffer {
            px: unsafe { std::slice::from_raw_parts_mut(base as *mut u32, stride * self.var.yres as usize) },
            stride,
            width: self.var.xres as usize,
            height: self.var.yres as usize,
        }
    }

    /// Show the back buffer (pan) and wait for vsync when the driver supports it.
    pub fn present(&mut self) -> io::Result<()> {
        if self.stream {
            let header = self.mem as *mut u32;
            unsafe {
                header.add(4).write_volatile(self.back);
                let sequence = &*(header.add(5) as *const std::sync::atomic::AtomicU32);
                sequence.fetch_add(1, std::sync::atomic::Ordering::Release);
            }
            self.back = (self.back + 1) % self.pages;
            return Ok(());
        }
        let fd = self.file.as_raw_fd();
        if self.pages > 1 {
            let mut v = self.var;
            v.yoffset = self.back * self.var.yres;
            unsafe { ioctl(fd, FBIOPAN_DISPLAY, &mut v)? };
            self.back = (self.back + 1) % self.pages;
        }
        if self.vsync_ok {
            let mut zero = 0u32;
            let _ = unsafe { ioctl(fd, FBIO_WAITFORVSYNC, &mut zero) };
        }
        Ok(())
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.mem as *mut c_void, self.fix.smem_len as usize) };
    }
}

pub struct BackBuffer<'a> {
    pub px: &'a mut [u32],
    pub stride: usize,
    pub width: usize,
    pub height: usize,
}

impl BackBuffer<'_> {
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for row in y..(y + h).min(self.height) {
            let start = row * self.stride + x.min(self.width);
            let end = row * self.stride + (x + w).min(self.width);
            self.px[start..end].fill(color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_frame_publishes_dimensions_pixels_and_page_rotation() {
        let path = format!("/tmp/doom-frame-test-{}", std::process::id());
        let mut fb = Framebuffer::shared_file(&path, 4, 3).unwrap();
        let word = |data: &[u8], at| u32::from_le_bytes(data[at..at + 4].try_into().unwrap());
        fb.back_buffer().px.fill(0xff332211);
        fb.present().unwrap();
        let first = std::fs::read(&path).unwrap();
        assert_eq!((word(&first, 4), word(&first, 8)), (4, 3));
        assert_eq!((word(&first, 16), word(&first, 20)), (0, 1));
        assert_eq!(word(&first, 4096), 0xff332211);
        fb.back_buffer().px.fill(0xff665544);
        fb.present().unwrap();
        let second = std::fs::read(&path).unwrap();
        assert_eq!((word(&second, 16), word(&second, 20)), (1, 2));
        assert_eq!(word(&second, 4096), 0xff332211);
        assert_eq!(word(&second, 4096 + 4 * 3 * 4), 0xff665544);
        drop(fb);
        std::fs::remove_file(path).unwrap();
    }
}
