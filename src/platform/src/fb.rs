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

impl VarInfo {
    /// Halve colour channels while keeping the display layer opaque.
    #[inline]
    pub fn dim(&self, pixel: u32) -> u32 {
        let alpha = self.pixel(0, 0, 0);
        ((pixel >> 1) & (self.pixel(127, 127, 127) & !alpha)) | alpha
    }

    /// Pack 8-bit RGB into this framebuffer's pixel layout.
    #[inline]
    pub fn pixel(&self, r: u8, g: u8, b: u8) -> u32 {
        let v = self;
        let put = |c: u8, f: &Bitfield| ((c as u32) >> (8 - f.length.min(8))) << f.offset;
        put(r, &v.red) | put(g, &v.green) | put(b, &v.blue)
            | if v.transp.length > 0 { ((1u32 << v.transp.length) - 1) << v.transp.offset } else { 0 }
    }

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
    held_page: Option<u32>,
}

impl Framebuffer {
    pub fn open_default() -> io::Result<Self> { Self::open("/dev/fb0") }
    pub fn open(path: &str) -> io::Result<Self> {
        if let Ok(shared) = std::env::var("TV_FB_PATH") {
            return Self::shared_file(&shared, 1280, 720);
        }
        Self::device(path)
    }

    /// Open a physical/surrogate framebuffer, bypassing the app transport.
    pub fn device(path: &str) -> io::Result<Self> {
        let file = std::fs::OpenOptions::new().read(true).write(true).open(path)?;
        Self::from_file(file, None)
    }

    /// Keep fbdev for mode/pan ioctls, mapping an exported DMA-BUF when supplied.
    /// This explicit constructor does not use the TV_FB_PATH app transport override.
    pub fn from_file(file: std::fs::File, memory_file: Option<std::fs::File>) -> io::Result<Self> {
        let fd = file.as_raw_fd();
        let mut var = VarInfo::default();
        let mut fix: FixInfo = unsafe { std::mem::zeroed() };
        unsafe {
            ioctl(fd, FBIOGET_VSCREENINFO, &mut var)?;
            ioctl(fd, FBIOGET_FSCREENINFO, &mut fix)?;
        }
        Self::from_layout(file, memory_file, var, fix)
    }

    fn from_layout(file: std::fs::File, memory_file: Option<std::fs::File>, var: VarInfo, fix: FixInfo) -> io::Result<Self> {
        let pages = mapping_pages(&var, &fix)?;
        let fd = file.as_raw_fd();
        let map_fd = memory_file.as_ref().map_or(fd, AsRawFd::as_raw_fd);
        let mem = unsafe {
            libc::mmap(std::ptr::null_mut(), fix.smem_len as usize,
                       libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, map_fd, 0)
        };
        if mem == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }
        let mut fb = Framebuffer { file, _memory_file: memory_file, var, fix, mem: mem as *mut u8, pages,
                                   back: if pages > 1 { 1 } else { 0 }, vsync_ok: false, stream: false, held_page: None };
        // Some drivers (virtio-gpu) accept FBIO_WAITFORVSYNC but return at once.
        // Trust it only if two back-to-back waits take a plausible frame time.
        let t0 = crate::sys::monotonic();
        let mut zero = 0u32;
        let ok = (0..2).all(|_| unsafe { ioctl(fd, FBIO_WAITFORVSYNC, &mut zero) }.is_ok());
        fb.vsync_ok = ok && (crate::sys::monotonic() - t0).as_micros() > 8_000;
        Ok(fb)
    }

    pub fn shared_file(path: &str, width: u32, height: u32) -> io::Result<Self> {
        let bytes = width.checked_mul(height).and_then(|n| n.checked_mul(8))
            .and_then(|n| n.checked_add(4096)).filter(|_| width > 0 && height > 0)
            .ok_or_else(|| io::Error::other("invalid stream dimensions"))?;
        let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(path)?;
        if file.metadata()?.len() != bytes as u64 { file.set_len(bytes as u64)?; }
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
        let valid=unsafe { header.read_volatile()==0x32465654
            && header.add(1).read_volatile()==width && header.add(2).read_volatile()==height
            && header.add(3).read_volatile()==width*4 && header.add(4).read_volatile()<2 };
        if !valid {
            for (i, value) in [0x32465654, width, height, width * 4, 0, 0].iter().enumerate() {
                unsafe { header.add(i).write_volatile(*value); }
            }
        }
        let back=unsafe { (header.add(4).read_volatile()+1)%2 };
        Ok(Self { file, _memory_file: None, var, fix, mem: mem as *mut u8,
            pages: 2, back, vsync_ok: false, stream: true, held_page: None })
    }

    fn region_lock(&self, start: u64, len: u64, kind: i16) -> io::Result<()> {
        let mut lock: libc::flock = unsafe { std::mem::zeroed() };
        lock.l_type = kind;
        lock.l_whence = libc::SEEK_SET as i16;
        lock.l_start = start as _;
        lock.l_len = len as _;
        let until = std::time::Instant::now() + std::time::Duration::from_secs(2);
        loop {
            if unsafe { libc::fcntl(self.file.as_raw_fd(), libc::F_SETLK, &lock) } == 0 { return Ok(()); }
            let error = io::Error::last_os_error();
            if !matches!(error.raw_os_error(), Some(libc::EAGAIN) | Some(libc::EACCES) | Some(libc::EINTR)) {
                return Err(error);
            }
            if std::time::Instant::now() >= until { return Err(io::Error::new(io::ErrorKind::TimedOut, "display buffer still held by consumer")); }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    fn hold_page(&mut self, page: u32) -> io::Result<()> {
        if !self.stream || self.held_page == Some(page) { return Ok(()); }
        if self.held_page.is_some() { return Err(io::Error::other("display already has an acquired frame")); }
        let bytes = (self.fix.line_length * self.var.yres) as u64;
        self.region_lock(4096 + page as u64 * bytes, bytes, libc::F_WRLCK as i16)?;
        self.held_page = Some(page);
        Ok(())
    }

    pub fn cancel_frame(&mut self) {
        if let Some(page) = self.held_page.take() {
            let bytes = (self.fix.line_length * self.var.yres) as u64;
            let _ = self.region_lock(4096 + page as u64 * bytes, bytes, libc::F_UNLCK as i16);
        }
    }

    /// Acquire a frame for exclusive writing. Dropping it cancels publication.
    pub fn acquire(&mut self) -> io::Result<FrameLease<'_>> {
        self.hold_page(self.back)?;
        Ok(FrameLease { display: self })
    }

    /// Consumer-side read lease. The producer cannot reuse this page until drop.
    pub fn read_presented(&mut self) -> io::Result<PresentedFrame<'_>> {
        let bytes = (self.fix.line_length * self.var.yres) as u64;
        for _ in 0..100 {
            self.region_lock(0, 4096, libc::F_RDLCK as i16)?;
            let header = self.mem as *const u32;
            let (page, sequence) = unsafe { (header.add(4).read_volatile(), header.add(5).read_volatile()) };
            self.region_lock(0, 4096, libc::F_UNLCK as i16)?;
            if page >= self.pages { return Err(io::Error::other("invalid published page")); }
            self.region_lock(4096 + page as u64 * bytes, bytes, libc::F_RDLCK as i16)?;
            self.region_lock(0, 4096, libc::F_RDLCK as i16)?;
            let unchanged = unsafe { header.add(4).read_volatile() == page && header.add(5).read_volatile() == sequence };
            self.region_lock(0, 4096, libc::F_UNLCK as i16)?;
            if unchanged { return Ok(PresentedFrame { display: self, page, sequence }); }
            self.region_lock(4096 + page as u64 * bytes, bytes, libc::F_UNLCK as i16)?;
        }
        Err(io::Error::new(io::ErrorKind::WouldBlock, "display changed during read acquisition"))
    }

    pub fn id(&self) -> String {
        String::from_utf8_lossy(&self.fix.id).trim_end_matches('\0').to_owned()
    }

    /// Pack RGB using the driver's reported bitfields.
    #[inline]
    pub fn pixel(&self, r: u8, g: u8, b: u8) -> u32 {
        self.var.pixel(r, g, b)
    }

    /// The page not currently on screen (or the only page), as rows of u32.
    pub fn back_buffer(&mut self) -> BackBuffer<'_> {
        self.hold_page(self.back).expect("acquire display buffer");
        let stride = (self.fix.line_length / 4) as usize;
        let base = unsafe { self.mem.add((self.back * self.fix.line_length * self.var.yres) as usize + if self.stream { 4096 } else { 0 }) };
        BackBuffer {
            px: unsafe { std::slice::from_raw_parts_mut(base as *mut u32, stride * self.var.yres as usize) },
            stride,
            width: self.var.xres as usize,
            height: self.var.yres as usize,
        }
    }

    /// The page currently on screen (the last one presented). Drawing here shows
    /// immediately; use it for small static updates such as a pause overlay.
    pub fn front_buffer(&mut self) -> BackBuffer<'_> {
        let front = (self.back + self.pages - 1) % self.pages;
        self.hold_page(front).expect("acquire visible display buffer");
        let stride = (self.fix.line_length / 4) as usize;
        let base = unsafe { self.mem.add((front * self.fix.line_length * self.var.yres) as usize + if self.stream { 4096 } else { 0 }) };
        BackBuffer {
            px: unsafe { std::slice::from_raw_parts_mut(base as *mut u32, stride * self.var.yres as usize) },
            stride,
            width: self.var.xres as usize,
            height: self.var.yres as usize,
        }
    }

    /// Show the back buffer (pan) and wait for vsync when the driver supports it.
    pub fn refresh_front(&mut self) {
        if self.stream {
            self.region_lock(0, 4096, libc::F_WRLCK as i16).expect("display header lock");
            let header = self.mem as *mut u32;
            unsafe {
                header.add(4).write_volatile((self.back + self.pages - 1) % self.pages);
                (&*(header.add(5) as *const std::sync::atomic::AtomicU32))
                    .fetch_add(1, std::sync::atomic::Ordering::Release);
            }
            self.region_lock(0, 4096, libc::F_UNLCK as i16).expect("display header unlock");
            self.cancel_frame();
        }
    }

    pub fn present(&mut self) -> io::Result<()> {
        if self.stream {
            self.region_lock(0, 4096, libc::F_WRLCK as i16)?;
            let header = self.mem as *mut u32;
            unsafe {
                header.add(4).write_volatile(self.back);
                (&*(header.add(5) as *const std::sync::atomic::AtomicU32))
                    .fetch_add(1, std::sync::atomic::Ordering::Release);
            }
            self.region_lock(0, 4096, libc::F_UNLCK as i16)?;
            self.cancel_frame();
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

fn mapping_pages(var: &VarInfo, fix: &FixInfo) -> io::Result<u32> {
    if var.bits_per_pixel != 32 {
        return Err(io::Error::other(format!("unsupported bpp {}", var.bits_per_pixel)));
    }
    let frame = fix.line_length.checked_mul(var.yres)
        .ok_or_else(|| io::Error::other("framebuffer size overflow"))?;
    if var.xres == 0 || var.yres == 0 || var.xres > fix.line_length / 4
        || fix.line_length % 4 != 0 || frame == 0 || frame > fix.smem_len
        || var.yres_virtual < var.yres || fix.smem_len as u64 > isize::MAX as u64 {
        return Err(io::Error::other("invalid framebuffer dimensions or backing memory"));
    }
    Ok((fix.smem_len / frame).min(var.yres_virtual / var.yres).max(1))
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        self.cancel_frame();
        unsafe { libc::munmap(self.mem as *mut c_void, self.fix.smem_len as usize) };
    }
}

pub struct FrameLease<'a> {
    display: &'a mut Framebuffer,
}

impl FrameLease<'_> {
    pub fn pixels(&mut self) -> BackBuffer<'_> { self.display.back_buffer() }
    pub fn present(self) -> io::Result<()> { self.display.present() }
}

impl std::ops::Deref for FrameLease<'_> {
    type Target = Framebuffer;
    fn deref(&self) -> &Self::Target { self.display }
}
impl std::ops::DerefMut for FrameLease<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.display }
}

impl Drop for FrameLease<'_> {
    fn drop(&mut self) { self.display.cancel_frame(); }
}

pub struct PresentedFrame<'a> {
    display: &'a mut Framebuffer,
    page: u32,
    pub sequence: u32,
}

impl PresentedFrame<'_> {
    pub fn pixels(&self) -> &[u32] {
        let count = (self.display.fix.line_length * self.display.var.yres / 4) as usize;
        unsafe { std::slice::from_raw_parts(self.display.mem.add(4096 + self.page as usize * count * 4).cast(), count) }
    }
}

impl Drop for PresentedFrame<'_> {
    fn drop(&mut self) {
        let bytes = (self.display.fix.line_length * self.display.var.yres) as u64;
        let _ = self.display.region_lock(4096 + self.page as u64 * bytes, bytes, libc::F_UNLCK as i16);
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
mod mapping_tests {
    use super::*;
    use std::io::{Seek, SeekFrom, Read};
    fn layout() -> (VarInfo, FixInfo) {
        let var=VarInfo{xres:4,yres:3,xres_virtual:4,yres_virtual:6,bits_per_pixel:32,..VarInfo::default()};
        let mut fix:FixInfo=unsafe{std::mem::zeroed()};fix.line_length=16;fix.smem_len=96;(var,fix)
    }
    #[test]
    fn rejects_geometry_that_would_escape_or_divide_by_zero() {
        let(var,fix)=layout();assert_eq!(mapping_pages(&var,&fix).unwrap(),2);
        for bad in [VarInfo{xres:0,..var},VarInfo{yres:0,..var},VarInfo{xres:5,..var},VarInfo{yres_virtual:2,..var},VarInfo{bits_per_pixel:24,..var}] {
            assert!(mapping_pages(&bad,&fix).is_err());
        }
        let mut short=fix;short.smem_len=47;assert!(mapping_pages(&var,&short).is_err());
        let mut overflow=fix;overflow.line_length=u32::MAX-3;assert!(mapping_pages(&var,&overflow).is_err());
    }
    #[test]
    fn tvf2_lease_publication_and_cancellation_stay_compatible() {
        let name=std::env::temp_dir().join(format!("platform-lease-{}",std::process::id()));
        let mut fb=Framebuffer::shared_file(name.to_str().unwrap(),4,3).unwrap();
        let mut frame=fb.acquire().unwrap();frame.pixels().px.fill(0xff332211);frame.present().unwrap();
        let bytes=std::fs::read(&name).unwrap();assert_eq!(u32::from_le_bytes(bytes[..4].try_into().unwrap()),0x32465654);
        {let mut canceled=fb.acquire().unwrap();canceled.pixels().px.fill(0xff665544);}
        {let presented=fb.read_presented().unwrap();assert_eq!(presented.sequence,1);assert!(presented.pixels().iter().all(|&p|p==0xff332211));}
        drop(fb);std::fs::remove_file(name).unwrap();
    }
    #[test]
    fn exported_dma_file_is_mapped_and_retained_separately_from_fb_control() {
        let name=std::env::temp_dir().join(format!("platform-map-{}",std::process::id()));
        let data=std::fs::OpenOptions::new().read(true).write(true).create_new(true).open(&name).unwrap();
        data.set_len(96).unwrap();let mut observer=data.try_clone().unwrap();
        let control=std::fs::File::open("/dev/null").unwrap();
        let(var,fix)=layout();let mut fb=Framebuffer::from_layout(control,Some(data),var,fix).unwrap();
        fb.back_buffer().px.fill(0x44332211);drop(fb);
        observer.seek(SeekFrom::Start(48)).unwrap();let mut word=[0;4];observer.read_exact(&mut word).unwrap();
        assert_eq!(u32::from_ne_bytes(word),0x44332211);std::fs::remove_file(name).unwrap();
    }
}
