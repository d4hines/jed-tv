//! Exact V109 MI wire ABI; configuration and module payload live only in RAM.
use super::State;
use std::fs::{self, OpenOptions};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd};
use std::path::Path;

const PACK: &[u8] = include_bytes!(env!("HELLO_MI_PACK"));

#[repr(C)]
struct ModuleStatus { selector: u32, ready: u8, padding: [u8; 3] }
#[repr(C)]
struct AttrWire { attr: u64, address: u64 }
#[repr(C)]
struct IoctlWire { bytes: u16, reserved: [u8; 6], address: u64 }
const _: () = assert!(std::mem::size_of::<ModuleStatus>() == 8);
const _: () = assert!(std::mem::offset_of!(ModuleStatus, ready) == 4);
const _: () = assert!(std::mem::size_of::<AttrWire>() == 16);
const _: () = assert!(std::mem::offset_of!(AttrWire, address) == 8);
const _: () = assert!(std::mem::size_of::<IoctlWire>() == 16);
const _: () = assert!(std::mem::offset_of!(IoctlWire, address) == 8);

fn attr_ioctl(fd: i32, request: u32, attr: &mut AttrWire) -> io::Result<()> {
    // libmi3 ioctl_wrapper (0x5090c) adds this outer envelope to all calls.
    let mut wire = IoctlWire { bytes: 16, reserved: [0; 6], address: (attr as *mut AttrWire) as usize as u64 };
    let result = unsafe { libc::ioctl(fd, request as _, &mut wire) };
    if result == 0 { return Ok(()); }
    if result < 0 { return Err(io::Error::last_os_error()); }
    Err(io::Error::other(format!("MI ioctl 0x{request:08x}: vendor return={result}")))
}

pub(super) fn stage(state: &State) -> io::Result<()> {
    // This kernel has no Android policy loaded yet. Refuse MI initialization
    // unless all visible eMMC block devices are read-only for this boot.
    let mut protected = 0;
    fs::create_dir_all("/dev/block")?;
    let block_entries = match fs::read_dir("/sys/class/block") {
        Ok(entries) => Some(entries),
        Err(e) if e.kind() == io::ErrorKind::NotFound && super::arg("hello.modules", "1") == "0" => None,
        Err(e) => return Err(e),
    };
    for entry in block_entries.into_iter().flatten() {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if !name.starts_with("mmcblk0") { continue; }
        let path = format!("/dev/{name}");
        let node = OpenOptions::new().read(true).open(&path)
            .map_err(|error| io::Error::other(format!("protect {path}: {error}")))?;
        let mut readonly: libc::c_int = 1;
        if unsafe { libc::ioctl(node.as_raw_fd(), 0x125du32 as _, &mut readonly) } != 0 {
            return Err(io::Error::other(format!("BLKROSET {path}: {}", io::Error::last_os_error())));
        }
        readonly = 0;
        if unsafe { libc::ioctl(node.as_raw_fd(), 0x125eu32 as _, &mut readonly) } != 0 || readonly != 1 {
            return Err(io::Error::other(format!("eMMC read-only verification failed: {name}")));
        }
        let alias = format!("/dev/block/{name}");
        match fs::symlink_metadata(&alias) {
            Ok(_) if fs::read_link(&alias)? == Path::new(&format!("../{name}")) => (),
            Ok(_) => return Err(io::Error::other(format!("unexpected existing block alias: {alias}"))),
            Err(e) if e.kind() == io::ErrorKind::NotFound => std::os::unix::fs::symlink(format!("../{name}"), alias)?,
            Err(e) => return Err(e),
        }
        protected += 1;
    }
    if protected == 0 && super::arg("hello.modules", "1") != "0" {
        return Err(io::Error::other("no eMMC block devices found for read-only protection"));
    }
    state.event(format!("{protected} eMMC block devices verified read-only for this boot"));
    let invalid = || io::Error::other("invalid embedded V109 configuration pack");
    let mut cursor = 0;
    let mut count = 0;
    while cursor < PACK.len() {
        let header = PACK.get(cursor..cursor + 8).ok_or_else(invalid)?;
        let name_len = u32::from_le_bytes(header[..4].try_into().unwrap()) as usize;
        let data_len = u32::from_le_bytes(header[4..].try_into().unwrap()) as usize;
        cursor += 8;
        if name_len > 512 || data_len > 8 * 1024 * 1024 { return Err(invalid()); }
        let name = std::str::from_utf8(PACK.get(cursor..cursor + name_len).ok_or_else(invalid)?)
            .map_err(|_| invalid())?;
        cursor += name_len;
        let content = PACK.get(cursor..cursor + data_len).ok_or_else(invalid)?;
        cursor += data_len;
        if !(name.starts_with("/vendor/tvconfig/config/") || name.starts_with("/tclconfig/hardware/")
            || name.starts_with("/modules/")) || name.split('/').any(|part| part == "..") {
            return Err(invalid());
        }
        let path = Path::new(name);
        fs::create_dir_all(path.parent().ok_or_else(invalid)?)?;
        fs::write(path, content)?;
        count += 1;
    }
    match fs::symlink_metadata("/config") {
        Ok(_) if fs::read_link("/config")? == Path::new("vendor/tvconfig/config") => (),
        Ok(_) => return Err(io::Error::other("unexpected /config alias")),
        Err(e) if e.kind() == io::ErrorKind::NotFound => std::os::unix::fs::symlink("vendor/tvconfig/config", "/config")?,
        Err(e) => return Err(e),
    }
    fs::write("/modules/order", "iniparser.ko\nkdrv_platform.ko\nkdrv_xc.ko\nkdrv_alg.ko\nlibdmdbraslia.ko\nutpa2k.ko\nhdcp1x.ko\nhdcp2x.ko\nxcker.ko\nmwgifker.ko\nmisck.ko\nmik.ko\nmstar_fbdev_mi.ko\n")?;
    state.event(format!("staged {count} V109 config/module files in RAM; DISP_INIT=1 OSD_INIT=1 TUNER_INIT=0"));
    Ok(())
}

fn get_status(fd: i32, selector: u32) -> io::Result<ModuleStatus> {
    let mut status = ModuleStatus { selector, ready: 0, padding: [0; 3] };
    let mut wire = AttrWire { attr: 5, address: (&mut status as *mut ModuleStatus) as usize as u64 };
    attr_ioctl(fd, 0xc0101805, &mut wire)
        .map_err(|error| io::Error::other(format!("MI GET selector={selector}: {error}")))?;
    if status.selector != selector { return Err(io::Error::other("MI GET changed selector")); }
    Ok(status)
}

pub(super) fn ready(state: &State) -> io::Result<()> {
    let path = ["/dev/mik/sys", "/dev/mik!sys"].into_iter().find(|path| Path::new(path).exists())
        .ok_or_else(|| io::Error::other(format!("MI sys device absent; class device={:?}; native mik entries={:?}",
            fs::read_to_string("/sys/class/mik/mik!sys/dev"),
            fs::read_dir("/dev/mik").map(|entries| entries.filter_map(Result::ok).map(|e| e.file_name()).collect::<Vec<_>>()))))?;
    state.event(format!("opening MI status device {path}"));
    let device = OpenOptions::new().read(true).write(true).open(path)?;
    let fd = device.as_raw_fd();
    for selector in [40, 6, 9, 1] {
        let status = get_status(fd, selector)?;
        state.event(format!("MI status selector={selector} ready={}", status.ready));
        if selector != 1 && status.ready != 1 {
            return Err(io::Error::other(format!("MI initialization incomplete: selector={selector}")));
        }
    }
    // Only reach this after successful module initialization and DISP/OSD status.
    let mut status = ModuleStatus { selector: 1, ready: 1, padding: [0; 3] };
    let mut wire = AttrWire { attr: 5, address: (&mut status as *mut ModuleStatus) as usize as u64 };
    attr_ioctl(fd, 0xc0101806, &mut wire)?;
    if get_status(fd, 1)?.ready != 1 { return Err(io::Error::other("MI readiness readback failed")); }
    state.event("MI DISP/OSD initialized; published selector 1 and verified readback");
    Ok(())
}

pub(super) fn framebuffer_nodes() -> io::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir("/sys/class/graphics")? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if name.starts_with("fb") && name.len() > 2 && name[2..].bytes().all(|b| b.is_ascii_digit()) {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}

pub(super) fn select_framebuffer(state: &State, before: &[String]) -> io::Result<String> {
    let after = framebuffer_nodes()?;
    state.event(format!("framebuffers before MI fb module={before:?} after={after:?}; proc/fb={:?}", fs::read_to_string("/proc/fb")));
    let created = after.iter().filter(|name| !before.contains(name)).collect::<Vec<_>>();
    let name = if created.len() == 1 {
        created[0].clone()
    } else if created.is_empty() {
        // An uploaded program may inherit the already-loaded module. Reuse
        // only the node recorded by our first successful module load in RAM.
        let saved = fs::read_to_string("/tmp/.hello-mi-fb")?;
        if !after.contains(&saved) { return Err(io::Error::other("saved MI framebuffer no longer exists")); }
        saved
    } else {
        return Err(io::Error::other("multiple new framebuffers; refusing to guess MI display"));
    };
    let path = [format!("/dev/{name}"), format!("/dev/graphics/{name}")].into_iter()
        .find(|path| Path::new(path).exists()).ok_or_else(|| io::Error::other("MI framebuffer node absent"))?;
    fs::write("/tmp/.hello-mi-fb", name)?;
    state.event(format!("selected MI framebuffer {path}"));
    Ok(path)
}

pub(super) fn prepare_framebuffer(state: &State, path: &str) -> io::Result<std::fs::File> {
    let file = OpenOptions::new().read(true).write(true).open(path)?;
    let fd = file.as_raw_fd();
    let mut var = super::fb::VarInfo::default();
    let mut fix: super::fb::FixInfo = unsafe { std::mem::zeroed() };
    let call = |request: u32, argument: *mut libc::c_void| -> io::Result<()> {
        let result = unsafe { libc::ioctl(fd, request as _, argument) };
        if result == 0 { Ok(()) } else { Err(io::Error::other(format!("MI framebuffer ioctl 0x{request:08x}: result={result} errno={}", io::Error::last_os_error()))) }
    };
    call(super::fb::FBIOGET_VSCREENINFO, (&mut var as *mut super::fb::VarInfo).cast())?;
    call(super::fb::FBIOGET_FSCREENINFO, (&mut fix as *mut super::fb::FixInfo).cast())?;
    state.event(format!("MI framebuffer initial var={var:?} fix={fix:?}"));
    if fix.smem_len == 0 {
        if var.xres != 1280 || var.yres != 720 || var.bits_per_pixel != 32 {
            return Err(io::Error::other("unexpected unallocated MI framebuffer geometry"));
        }
        // Exact V109 mstar_fb_ioctl: 0x4696 allocates IOMMU bytes, 0x4675
        // accepts width/height/pitch/MI-format, 0x4681 creates the OSD layer.
        let pitch = var.xres * 4;
        let mut bytes = pitch * var.yres * 2;
        call(0x40044696, (&mut bytes as *mut u32).cast())?;
        let mut osd = [var.xres, var.yres, pitch, 8u32];
        call(0x40104675, osd.as_mut_ptr().cast())?;
        state.event(format!("requested {bytes} bytes of MI IOMMU memory"));
    }
    call(0x4681, std::ptr::null_mut())?;
    let mut show = 1u8;
    call(0x40014663, (&mut show as *mut u8).cast())?;
    call(super::fb::FBIOGET_VSCREENINFO, (&mut var as *mut super::fb::VarInfo).cast())?;
    call(super::fb::FBIOGET_FSCREENINFO, (&mut fix as *mut super::fb::FixInfo).cast())?;
    state.event(format!("MI framebuffer prepared var={var:?} fix={fix:?}"));
    Ok(file)
}

pub(super) fn map_framebuffer(state: &State, file: std::fs::File) -> io::Result<super::fb::Framebuffer> {
    // fbdev's legacy mmap truncates this board's 64-bit IOVA. Export the
    // existing allocation through the same MMA API used by stock libutopia.
    let mut info = [0u32; 4];
    if unsafe { libc::ioctl(file.as_raw_fd(), 0x80104672u32 as _, info.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    let iova = (info[0] as u64) | ((info[1] as u64) << 32);
    let bytes = info[2];
    state.event(format!("MI allocation IOVA=0x{iova:x} bytes={bytes}"));
    if iova < 0x200000000 || bytes != 1280 * 720 * 4 * 2 {
        return Err(io::Error::other("unexpected MI IOMMU allocation"));
    }
    let mma = OpenOptions::new().read(true).write(true).open("/dev/mma")?;
    let call = |request: u32, wire: &mut [u32; 10]| -> io::Result<()> {
        let result = unsafe { libc::ioctl(mma.as_raw_fd(), request as _, wire.as_mut_ptr()) };
        if result < 0 { return Err(io::Error::other(format!("MMA ioctl 0x{request:08x}: result={result} errno={}", io::Error::last_os_error()))); }
        Ok(())
    };
    let mut wire = [0u32; 10];
    wire[8] = info[0]; wire[9] = info[1];
    call(0xc0284d12, &mut wire)?;
    let global_name = wire[1];
    let base = wire[8] as u64 | ((wire[9] as u64) << 32);
    state.event(format!("MMA global name={global_name} base=0x{base:x}"));
    if global_name == 0 || base != iova { return Err(io::Error::other("MMA allocation identity mismatch")); }
    wire = [0; 10]; wire[1] = global_name;
    call(0xc0284d0a, &mut wire)?;
    let fd = wire[0] as i32;
    if fd < 0 { return Err(io::Error::other("MMA returned invalid DMA buffer fd")); }
    let memory = unsafe { std::fs::File::from_raw_fd(fd) };
    wire = [0; 10]; wire[0] = fd as u32; wire[6] = 0;
    call(0x40284d06, &mut wire)?;
    state.event(format!("mapping exported MMA DMA buffer fd={fd}, uncached"));
    super::fb::Framebuffer::from_file(file, Some(memory))
}

pub(super) fn reset_osd(state: &State) -> io::Result<()> {
    let file = OpenOptions::new().read(true).write(true).open("/dev/mik/osd")?;
    let fd = file.as_raw_fd();
    // Exact libmi3 MI_OSD_DeInit followed by MI_OSD_Init with the zero-valued
    // settings from this board's mik_init.ini. Only use between draw children.
    if unsafe { libc::ioctl(fd, 0x1401u32 as _, std::ptr::null_mut::<u8>()) } != 0 {
        return Err(io::Error::other(format!("OSD deinit: {}", io::Error::last_os_error())));
    }
    let mut init = [0u32; 3];
    let mut wire = IoctlWire { bytes: 12, reserved: [0; 6], address: init.as_mut_ptr() as usize as u64 };
    let result = unsafe { libc::ioctl(fd, 0xc00c1400u32 as _, &mut wire) };
    if result != 0 { return Err(io::Error::other(format!("OSD reinit: {result}"))); }
    state.event("reset stock OSD bookkeeping between draw attempts");
    Ok(())
}

fn words_ioctl(file: &std::fs::File, request: u32, words: &mut [u32]) -> io::Result<()> {
    let mut wire = IoctlWire { bytes: (words.len() * 4) as u16, reserved: [0; 6], address: words.as_mut_ptr() as usize as u64 };
    let result = unsafe { libc::ioctl(file.as_raw_fd(), request as _, &mut wire) };
    if result != 0 { return Err(io::Error::other(format!("MI ioctl 0x{request:08x} returned {result}: {}", io::Error::last_os_error()))); }
    Ok(())
}

pub(super) fn report_output(state: &State) -> io::Result<()> {
    let disp = OpenOptions::new().read(true).write(true).open("/dev/mik/disp")?;
    let mut controller = [0u32, 0];
    words_ioctl(&disp, 0xc0081206, &mut controller)?;
    state.event(format!("display controller=0x{:x}", controller[0]));
    let mut mute = [controller[0], u32::MAX, 0];
    match words_ioctl(&disp, 0xc00c1228, &mut mute) {
        Ok(()) => state.event(format!("display muted={} (all mute reasons)", mute[2])),
        Err(error) => state.event(format!("mute query not supported for controller: {error}")),
    }
    let osd = OpenOptions::new().read(true).write(true).open("/dev/mik/osd")?;
    let mut layer = [0u32, 1];
    words_ioctl(&osd, 0xc0081456, &mut layer)?;
    state.event(format!("OSD layer 1 handle=0x{:x}", layer[0]));
    if std::env::args().any(|arg| arg == "--show-layer") {
        let mut show = [layer[0]];
        words_ioctl(&osd, 0xc0041405, &mut show)?;
        state.event("explicit MI_OSD_LayerShow completed");
    }
    let mut enabled = [layer[0], 0];
    words_ioctl(&osd, 0xc008145d, &mut enabled)?;
    state.event(format!("OSD layer enabled={}", enabled[1]));
    Ok(())
}

pub(super) fn init_display(state: &State) -> io::Result<()> {
    for (path, request, mut args, name) in [
        ("/dev/mik/disp", 0xc00c1200, [6u32, 6, 0], "MI_DISP_Init"),
        ("/dev/mik/osd", 0xc00c1400, [0u32; 3], "MI_OSD_Init"),
    ] {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        words_ioctl(&file, request, &mut args)?;
        state.event(format!("{name} succeeded"));
    }
    Ok(())
}
