//! V109 direct scanout. Intended for PID1 ownership; caller serializes hardware.
//! Opening fb1 has driver-side effects, even before our explicit ioctls.
use std::{
    ffi::c_void,
    fs::{File, OpenOptions},
    io::{self, Write},
    os::fd::{AsRawFd, FromRawFd},
    ptr::NonNull,
};
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const PITCH: u32 = WIDTH * 4;
const ALLOCATION_BYTES: u32 = PITCH * HEIGHT * 2;
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
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
struct Bitfield {
    offset: u32,
    length: u32,
    msb_right: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
struct Var {
    x: u32,
    y: u32,
    xvirtual: u32,
    yvirtual: u32,
    xoffset: u32,
    yoffset: u32,
    bpp: u32,
    grayscale: u32,
    red: Bitfield,
    green: Bitfield,
    blue: Bitfield,
    alpha: Bitfield,
    rest: [u32; 20],
}
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
struct Fix {
    id: [u8; 16],
    start: u32,
    bytes: u32,
    kind: u32,
    aux: u32,
    visual: u32,
    xpan: u16,
    ypan: u16,
    ywrap: u16,
    pitch: u32,
    mmio: u32,
    mmio_len: u32,
    accel: u32,
    capabilities: u16,
    reserved: [u16; 2],
}
const _: () = assert!(std::mem::size_of::<Var>() == 160);
const _: () = assert!(std::mem::size_of::<Fix>() == 68);
#[repr(C)]
struct Wire {
    bytes: u16,
    pad: [u8; 6],
    pointer: u64,
}
const _: () = assert!(std::mem::size_of::<Wire>() == 16);
fn call<T>(file: &File, request: u32, value: &mut T) -> io::Result<()> {
    let r = unsafe { ioctl(file.as_raw_fd(), request, (value as *mut T).cast()) };
    if r == 0 {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "ioctl {request:#x}: result={r} errno={}",
            io::Error::last_os_error()
        )))
    }
}
fn mma(file: &File, request: u32, value: &mut [u32; 10]) -> io::Result<()> {
    let r = unsafe { ioctl(file.as_raw_fd(), request, value.as_mut_ptr().cast()) };
    if r >= 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
// A surviving window is a DMA consumer even after its layer is hidden.
// V109's WindowDestroy is broken, and fbdev FREE still frees the allocation
// after that error. Retain memory unless the MI getters prove no such window.
fn no_window_references(file: &File) -> io::Result<bool> {
    fn words(file: &File, request: u32, values: &mut [u32; 2]) -> i32 {
        let mut wire = Wire {
            bytes: 8,
            pad: [0; 6],
            pointer: values.as_mut_ptr() as usize as u64,
        };
        unsafe { ioctl(file.as_raw_fd(), request, (&mut wire as *mut Wire).cast()) }
    }
    let mut layer = [0u32, 1];
    let status = words(file, 0xc0081456, &mut layer);
    if status == 3 && layer[0] == 0 {
        return Ok(true);
    }
    if status != 0 || layer[0] != 0x20001 {
        return Err(io::Error::other(
            "cannot establish layer1 identity for cleanup",
        ));
    }
    let mut window = [layer[0], 0];
    let status = words(file, 0xc0081457, &mut window);
    Ok(window_is_absent(status, window[1]))
}
fn window_is_absent(status: i32, handle: u32) -> bool {
    status == 3 && handle == 0
}
fn release_allocation(file: &File) {
    let may_free = File::open("/dev/mik/osd").or_else(|_| File::open("/dev/mik!osd")).and_then(|osd| no_window_references(&osd));
    if !matches!(may_free, Ok(true)) {
        let _=writeln!(std::io::stdout(),"DIRECT allocation retained: window still exists or cleanup identity is uncertain; do not recreate layer1");
        return;
    }
    let r = unsafe { ioctl(file.as_raw_fd(), 0x4697, std::ptr::null_mut()) };
    let _ = writeln!(std::io::stdout(), "DIRECT allocation released result={r}");
}

struct Allocation {
    file: File,
    allocated: bool,
    release: fn(&File),
}
impl std::ops::Deref for Allocation {
    type Target = File;
    fn deref(&self) -> &File {
        &self.file
    }
}
impl Drop for Allocation {
    fn drop(&mut self) {
        if self.allocated {
            (self.release)(&self.file);
        }
    }
}
pub struct Scanout {
    framebuffer: Allocation,
    memory: Option<File>,
    pointer: NonNull<u32>,
    bytes: usize,
    var: Var,
    fix: Fix,
    back: u32,
    pages: u32,
    shown: bool,
}
impl Scanout {
    /// Explicitly creates a new layer through fb1. Never calls global MI init/reset.
    pub fn create() -> io::Result<Self> {
        if std::fs::read_to_string("/sys/class/graphics/fb1/name")?.trim() != "MStar FB0" {
            return Err(io::Error::other("fb1 is not V109 MI framebuffer"));
        }
        let osd = File::open("/dev/mik/osd").or_else(|_| File::open("/dev/mik!osd"))?;
        let mut layer = [0u32, 1];
        let mut wire = Wire {
            bytes: 8,
            pad: [0; 6],
            pointer: layer.as_mut_ptr() as usize as u64,
        };
        let status = unsafe { ioctl(osd.as_raw_fd(), 0xc0081456, (&mut wire as *mut Wire).cast()) };
        if status != 3 || layer[0] != 0 {
            return Err(io::Error::other(format!(
                "layer1 must be absent before takeover: status={status} handle={:#x}",
                layer[0]
            )));
        }
        let path = if std::path::Path::new("/dev/fb1").exists() {
            "/dev/fb1"
        } else {
            "/dev/graphics/fb1"
        };
        let mut framebuffer = Allocation {
            file: OpenOptions::new().read(true).write(true).open(path)?,
            allocated: false,
            release: release_allocation,
        };
        // Queued two-page flips stall on this firmware without the vendor presenter.
        // Immediate ExternSurface presentation is the verified standalone path.
        call(&framebuffer, 0x40044690, &mut 1u32)?;
        let mut var = Var::default();
        let mut fix = Fix::default();
        call(&framebuffer, 0x4600, &mut var)?;
        call(&framebuffer, 0x4602, &mut fix)?;
        let _ = writeln!(std::io::stdout(), "DIRECT initial var={var:?} fix={fix:?}");
        if var.x != 1280 || var.y != 720 || var.bpp != 32 || fix.bytes != 0 {
            return Err(io::Error::other(
                "expected unallocated 1280x720 MI framebuffer",
            ));
        }
        let mut bytes = ALLOCATION_BYTES;
        call(&framebuffer, 0x40044696, &mut bytes)?;
        framebuffer.allocated = true;
        let mut config = [WIDTH, HEIGHT, PITCH, 8];
        call(&framebuffer, 0x40104675, &mut config)?;
        let r = unsafe { ioctl(framebuffer.as_raw_fd(), 0x4681, std::ptr::null_mut()) };
        if r != 0 {
            return Err(io::Error::other(format!(
                "create framebuffer OSD layer: {r}"
            )));
        }
        call(&framebuffer, 0x4600, &mut var)?;
        call(&framebuffer, 0x4602, &mut fix)?;
        let _ = writeln!(
            std::io::stdout(),
            "DIRECT allocated var={var:?} fix={fix:?}"
        );
        let pages = validate_layout(&var, &fix)?;
        let mut info = [0u32; 4];
        call(&framebuffer, 0x80104672, &mut info)?;
        let iova = allocation_address(info)?;
        let device = OpenOptions::new().read(true).write(true).open("/dev/mma")?;
        let mut wire = [0; 10];
        wire[8] = info[0];
        wire[9] = info[1];
        mma(&device, 0xc0284d12, &mut wire)?;
        let name = wire[1];
        if name == 0 || (u64::from(wire[8]) | (u64::from(wire[9]) << 32)) != iova {
            return Err(io::Error::other("MMA allocation identity mismatch"));
        }
        wire = [0; 10];
        wire[0] = u32::MAX;
        wire[1] = name;
        mma(&device, 0xc0284d0a, &mut wire)?;
        if wire[0] > i32::MAX as u32 {
            return Err(io::Error::other("invalid DMA-BUF fd"));
        }
        // Successful MMA_IMPORT transfers this newly created descriptor to us.
        // Declared after framebuffer, so every subsequent error closes this
        // reference before Allocation frees the kernel allocation.
        let memory = unsafe { File::from_raw_fd(wire[0] as i32) };
        if memory.metadata()?.len() != ALLOCATION_BYTES as u64 {
            return Err(io::Error::other("exported DMA-BUF length mismatch"));
        }
        wire = [0; 10];
        wire[0] = memory.as_raw_fd() as u32;
        mma(&device, 0x40284d06, &mut wire)?;
        let pointer = unsafe {
            mmap(
                std::ptr::null_mut(),
                ALLOCATION_BYTES as usize,
                3,
                1,
                memory.as_raw_fd(),
                0,
            )
        };
        if pointer as isize == -1 {
            return Err(io::Error::last_os_error());
        }
        let Some(pointer) = NonNull::new(pointer.cast::<u32>()) else {
            unsafe {
                munmap(pointer, ALLOCATION_BYTES as usize);
            }
            return Err(io::Error::other("mmap returned null address"));
        };
        let _ = writeln!(
            std::io::stdout(),
            "DIRECT mapped IOVA={iova:#x} bytes={bytes} pages={pages} DMA-BUF={} layer=1",
            memory.as_raw_fd()
        );
        Ok(Self {
            framebuffer,
            memory: Some(memory),
            pointer,
            bytes: ALLOCATION_BYTES as usize,
            var,
            fix,
            back: 0,
            pages,
            shown: false,
        })
    }
    pub fn color_bars(&mut self) -> io::Result<()> {
        let rgba = [
            0xff0000ffu32,
            0xff00ff00,
            0xffff0000,
            0xffffffff,
            0xff808080,
            0xff000000,
        ];
        let packed = rgba.map(|p| self.pack(p));
        let pitch = self.fix.pitch as usize / 4;
        let pixels = unsafe {
            std::slice::from_raw_parts_mut(
                self.pointer.as_ptr().add(self.back as usize * pitch * 720),
                pitch * 720,
            )
        };
        for y in 0..720 {
            for x in 0..1280 {
                pixels[y * pitch + x] = packed[x * packed.len() / 1280];
            }
        }
        self.present()
    }
    /// Input is the APIv1 RGBA word encoding, independently of hardware bitfields.
    pub fn draw_rgba(&mut self, pixels: &[u32]) -> io::Result<()> {
        if pixels.len() != 1280 * 720 {
            return Err(io::Error::other("expected composed 1280x720 pixels"));
        }
        let pitch = self.fix.pitch as usize / 4;
        let base = self.back as usize * pitch * 720;
        for (y, row) in pixels.chunks_exact(1280).enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                unsafe {
                    self.pointer
                        .as_ptr()
                        .add(base + y * pitch + x)
                        .write_volatile(self.pack(pixel));
                }
            }
        }
        self.present()
    }
    /// Hide this layer while retaining its allocation/window for later frames.
    pub fn hide(&mut self) -> io::Result<()> {
        if self.shown {
            call(&self.framebuffer, 0x40014663, &mut 0u8)?;
            self.shown = false;
        }
        Ok(())
    }
    fn pack(&self, p: u32) -> u32 {
        pack_pixel(p, &self.var)
    }
    fn present(&mut self) -> io::Result<()> {
        let mut var = self.var;
        var.yoffset = self.back * 720;
        call(&self.framebuffer, 0x4606, &mut var)?;
        if !self.shown {
            call(&self.framebuffer, 0x40014663, &mut 1u8)?;
            self.shown = true;
        }
        self.back = (self.back + 1) % self.pages.max(1);
        Ok(())
    }
}
impl Drop for Scanout {
    fn drop(&mut self) {
        // Stop scanout, unmap CPU memory, and close the export before the
        // Allocation field's destructor tears down windows and frees memory.
        if self.shown {
            let _ = call(&self.framebuffer, 0x40014663, &mut 0u8);
        }
        unsafe {
            munmap(self.pointer.as_ptr().cast(), self.bytes);
        }
        drop(self.memory.take());
    }
}

fn validate_layout(var: &Var, fix: &Fix) -> io::Result<u32> {
    if var.x != WIDTH
        || var.y != HEIGHT
        || var.xvirtual < WIDTH
        || var.bpp != 32
        || fix.pitch != PITCH
        || fix.bytes != ALLOCATION_BYTES
        || var.yvirtual < HEIGHT
    {
        return Err(io::Error::other(
            "unexpected allocated framebuffer geometry",
        ));
    }
    // Four distinct byte channels form the complete 32-bit pixel. Rejecting
    // arbitrary shifts keeps packing bounded and prevents overlapping colors.
    let mut offsets = 0u32;
    for c in [var.red, var.green, var.blue, var.alpha] {
        if c.length != 8
            || c.offset > 24
            || c.offset % 8 != 0
            || c.msb_right != 0
            || offsets & (1 << c.offset) != 0
        {
            return Err(io::Error::other(
                "unsupported or overlapping framebuffer bitfields",
            ));
        }
        offsets |= 1 << c.offset;
    }
    Ok((var.yvirtual / HEIGHT).min(ALLOCATION_BYTES / (PITCH * HEIGHT)))
}
fn allocation_address(info: [u32; 4]) -> io::Result<u64> {
    let iova = u64::from(info[0]) | (u64::from(info[1]) << 32);
    if iova < 0x200000000
        || info[2] != ALLOCATION_BYTES
        || iova.checked_add(ALLOCATION_BYTES as u64).is_none()
    {
        return Err(io::Error::other("invalid full-width IOVA/length"));
    }
    Ok(iova)
}
fn pack_pixel(p: u32, var: &Var) -> u32 {
    ((p & 255) << var.red.offset)
        | (((p >> 8) & 255) << var.green.offset)
        | (((p >> 16) & 255) << var.blue.offset)
        | (((p >> 24) & 255) << var.alpha.offset)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn layout() -> (Var, Fix) {
        let mut var = Var::default();
        var.x = WIDTH;
        var.y = HEIGHT;
        var.xvirtual = WIDTH;
        var.yvirtual = HEIGHT * 2;
        var.bpp = 32;
        var.red = Bitfield {
            offset: 16,
            length: 8,
            msb_right: 0,
        };
        var.green = Bitfield {
            offset: 8,
            length: 8,
            msb_right: 0,
        };
        var.blue = Bitfield {
            offset: 0,
            length: 8,
            msb_right: 0,
        };
        var.alpha = Bitfield {
            offset: 24,
            length: 8,
            msb_right: 0,
        };
        let mut fix = Fix::default();
        fix.pitch = PITCH;
        fix.bytes = ALLOCATION_BYTES;
        (var, fix)
    }
    #[test]
    fn free_requires_proven_absent_window() {
        assert!(window_is_absent(3, 0));
        for (status, handle) in [(0, 0x40001), (0, 0), (3, 0x40001), (8, 0), (-1, 0)] {
            assert!(!window_is_absent(status, handle));
        }
    }
    #[test]
    fn full_width_iova_and_allocation_bounds() {
        assert_eq!(
            allocation_address([0x12345000, 2, ALLOCATION_BYTES, 0]).unwrap(),
            0x212345000
        );
        assert!(allocation_address([0x12345000, 0, ALLOCATION_BYTES, 0]).is_err());
        assert!(allocation_address([0x12345000, 2, ALLOCATION_BYTES - 4, 0]).is_err());
        assert!(allocation_address([u32::MAX, u32::MAX, ALLOCATION_BYTES, 0]).is_err());
    }
    #[test]
    fn layout_rejects_truncated_memory_and_overlapping_channels() {
        let (var, mut fix) = layout();
        assert_eq!(validate_layout(&var, &fix).unwrap(), 2);
        fix.bytes -= 4;
        assert!(validate_layout(&var, &fix).is_err());
        let (mut var, fix) = layout();
        var.blue = var.red;
        assert!(validate_layout(&var, &fix).is_err());
        let (mut var, fix) = layout();
        var.yvirtual = 0;
        assert!(validate_layout(&var, &fix).is_err());
    }
    #[test]
    fn converts_rgba_words_to_hardware_argb() {
        let (var, _) = layout();
        assert_eq!(pack_pixel(0x44332211, &var), 0x44112233);
    }
    thread_local! {static RELEASES:std::cell::RefCell<Vec<i32>>=const{std::cell::RefCell::new(Vec::new())};}
    fn record_release(file: &File) {
        assert!(file.metadata().is_ok());
        RELEASES.with(|r| r.borrow_mut().push(file.as_raw_fd()));
    }
    #[test]
    fn successful_allocation_is_released_on_later_failure_before_fd_closes() {
        RELEASES.with(|r| r.borrow_mut().clear());
        let result = (|| -> io::Result<()> {
            let file = File::open("/dev/null")?;
            let mut guard = Allocation {
                file,
                allocated: false,
                release: record_release,
            };
            guard.allocated = true;
            Err(io::Error::other("later stage failed"))
        })();
        assert!(result.is_err());
        RELEASES.with(|r| assert_eq!(r.borrow().len(), 1));
    }
    #[test]
    fn failed_allocation_is_not_freed() {
        RELEASES.with(|r| r.borrow_mut().clear());
        drop(Allocation {
            file: File::open("/dev/null").unwrap(),
            allocated: false,
            release: record_release,
        });
        RELEASES.with(|r| assert!(r.borrow().is_empty()));
    }
}
