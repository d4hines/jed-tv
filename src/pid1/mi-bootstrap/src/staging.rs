//! Fixed G06/V109 filesystem and module stages. No MI ioctl or vendor process.
use crate::{cold_start, SERIAL};
use std::{
    collections::BTreeSet,
    ffi::CString,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    os::{
        fd::AsRawFd,
        unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt},
    },
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

const VENDOR_OFFSET: u64 = 734_003_200;
const LOOP_SET_FD: u32 = 0x4c00;
const LOOP_SET_STATUS64: u32 = 0x4c04;
const LOOP_GET_STATUS64: u32 = 0x4c05;
const MS_RDONLY: usize = 1;
const MS_REMOUNT: usize = 32;
const MS_BIND: usize = 4096;
const RAM_DIR: &str = "/tmp/mi-bootstrap-stage";
const INI: &str = "/vendor/tvconfig/config/mik_init.ini";
const WATCHDOG_MODULES: &[&str] = &["iniparser", "kdrv_xc", "utpa2k"];
static CLAIMED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub action: &'static str,
    pub detail: String,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    New,
    Mounting,
    Mounted,
    Staging,
    Configured,
    Loading,
    Loaded,
    Failed,
}

/// Records survive a failed stage so the caller can report partial progress.
pub struct ColdPlatform {
    events: Vec<Event>,
    phase: Phase,
    inherited: BTreeSet<String>,
}
pub struct StagedPlatform {
    pub events: Vec<Event>,
    pub ini_ram_path: &'static str,
}

impl ColdPlatform {
    /// Reserve the one staging attempt in this PID 1.
    ///
    /// # Safety
    /// Must be called in a fresh native boot before any user MI initializer or
    /// client has run. Caller serializes all mounting/module work throughout.
    /// Neither an empty process list nor module metadata establishes this history.
    /// Existing validated read-only mounts and exactly the watchdog prerequisites
    /// iniparser/kdrv_xc/utpa2k are allowed. Caller attests those modules came from
    /// the current stock watchdog startup, and keeps that watchdog running.
    /// No other MI preload member or framebuffer module may already exist.
    pub unsafe fn begin() -> io::Result<Self> {
        guard()?;
        let mounts = read_mounts()?;
        if !covering_mount(&mounts, "/")
            .is_some_and(|m| ["rootfs", "ramfs", "tmpfs"].contains(&m.fs.as_str()))
        {
            return Err(fail("cold staging requires a native RAM root"));
        }
        let inherited = inherited_watchdog_modules()?;
        CLAIMED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| fail("cold platform staging already attempted in this PID 1"))?;
        Ok(Self {
            events: vec![],
            phase: Phase::New,
            inherited,
        })
    }
    pub fn events(&self) -> &[Event] {
        &self.events
    }
    fn event(
        &mut self,
        observer: &mut impl FnMut(&Event),
        action: &'static str,
        detail: impl Into<String>,
    ) {
        let event = Event {
            action,
            detail: detail.into(),
        };
        observer(&event);
        self.events.push(event);
    }
    fn enter(&mut self, expected: Phase, running: Phase) -> io::Result<()> {
        if self.phase != expected {
            return Err(fail("wrong staging phase; failed stages cannot be retried"));
        }
        self.phase = running;
        if let Err(error) = guard() {
            self.phase = Phase::Failed;
            return Err(error);
        }
        Ok(())
    }
    pub fn mount_stock(&mut self, observer: &mut impl FnMut(&Event)) -> io::Result<()> {
        self.enter(Phase::New, Phase::Mounting)?;
        let result = (|| {
            self.event(
                observer,
                "mounts",
                "validating fixed stock partition geometry",
            );
            for (name, sectors, start, partname) in [
                ("mmcblk0p4", 16_384, 22_528, "tvcertificate"),
                ("mmcblk0p7", 65_536, 51_200, "tvconfig"),
                ("mmcblk0p27", 245_760, 1_708_032, "tclconfig"),
                ("mmcblk0p28", 7_065_600, 1_953_792, "super"),
            ] {
                verify_partition(name, sectors, start, partname)?;
            }
            ensure_loop()?;
            self.event(
                observer,
                "loop",
                "loop1: read-only super vendor slice, offset734003200",
            );
            ensure_ext4("/dev/loop1", "/stock-vendor", "/")?;
            ensure_bind("/stock-vendor/lib", "/vendor/lib", "/dev/loop1", "/lib")?;
            ensure_ext4("/dev/mmcblk0p7", "/vendor/tvconfig", "/")?;
            ensure_ext4("/dev/mmcblk0p27", "/tclconfig", "/")?;
            // The original filesystem needs journal recovery; expose it only
            // with the same ro,noload policy as the other stock partitions.
            ensure_ext4("/dev/mmcblk0p4", "/vendor/tvcertificate", "/")?;
            ensure_alias("/vendor/tvconfig/config", "/config")?;
            ensure_alias("/tclconfig", "/vendor/tclconfig")?;
            // Preserve the stock flash library's well-known aliases without
            // modifying an existing alias or opening a block device for writing.
            for entry in fs::read_dir("/sys/class/block")? {
                let entry = entry?;
                let name = entry.file_name().to_string_lossy().into_owned();
                if !name.starts_with("mmcblk0p") {
                    continue;
                }
                let uevent = fs::read_to_string(entry.path().join("uevent"))?;
                if let Some(part) = uevent.lines().find_map(|l| l.strip_prefix("PARTNAME=")) {
                    if part.is_empty()
                        || !part
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
                    {
                        return Err(fail("unexpected partition name"));
                    }
                    for prefix in [
                        "/dev/block/by-name",
                        "/dev/block/platform/mstar_mci.0/by-name",
                    ] {
                        fs::create_dir_all(prefix)?;
                        ensure_alias(&format!("/dev/{name}"), &format!("{prefix}/{part}"))?;
                    }
                }
            }
            self.event(
                observer,
                "mounts",
                "stock vendor modules and tvconfig/tclconfig available read-only",
            );
            Ok(())
        })();
        self.phase = if result.is_ok() {
            Phase::Mounted
        } else {
            Phase::Failed
        };
        result
    }
    pub fn stage_configuration(
        &mut self,
        policy: &impl cold_start::IniPolicy,
        observer: &mut impl FnMut(&Event),
    ) -> io::Result<()> {
        self.enter(Phase::Mounted, Phase::Staging)?;
        let result = (|| {
            let mounts = read_mounts()?;
            let parent =
                covering_mount(&mounts, "/tmp").ok_or_else(|| fail("no mount record for /tmp"))?;
            if !["rootfs", "ramfs", "tmpfs"].contains(&parent.fs.as_str()) {
                return Err(fail("INI staging parent must be RAM-backed"));
            }
            let original = read_bounded(INI, 64 * 1024)?;
            let original =
                std::str::from_utf8(&original).map_err(|_| fail("stock INI is not UTF-8"))?;
            let staged = policy.prepare(original)?;
            fs::create_dir(RAM_DIR)?; // Refuse stale output from a previous attempt.
            let path = format!("{RAM_DIR}/mik_init.ini");
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o400)
                .open(&path)?;
            file.write_all(staged.as_bytes())?;
            file.sync_all()?;
            if fs::read(&path)? != staged.as_bytes() {
                return Err(fail("RAM INI readback mismatch"));
            }
            mount_call(Some(&path), INI, None, MS_BIND, None)?;
            mount_call(None, INI, None, MS_BIND | MS_REMOUNT | MS_RDONLY, None)?;
            if fs::read(INI)? != staged.as_bytes() {
                return Err(fail("bound INI readback mismatch"));
            }
            policy.validate(&fs::read_to_string(INI)?)?;
            self.event(observer, "ini", "RAM copy bound read-only; only MI_TUNER_INIT changed1→0; all35 INIT flags validated");
            Ok(())
        })();
        self.phase = if result.is_ok() {
            Phase::Configured
        } else {
            Phase::Failed
        };
        result
    }
    pub fn load_modules(
        &mut self,
        policy: &impl cold_start::IniPolicy,
        observer: &mut impl FnMut(&Event),
    ) -> io::Result<()> {
        self.enter(Phase::Configured, Phase::Loading)?;
        let result = (|| {
            if inherited_watchdog_modules()? != self.inherited {
                return Err(fail("watchdog module set changed during staging"));
            }
            policy.validate(&fs::read_to_string(INI)?)?;
            for &name in cold_start::PRE_DISPLAY_MODULES {
                let path = format!("/vendor/lib/modules/{name}.ko");
                let bytes = read_bounded(&path, 64 * 1024 * 1024)?;
                validate_module(&bytes)?;
                if self.inherited.contains(name) {
                    self.event(
                        observer,
                        "module-adopted",
                        format!("{name}: existing stock watchdog prerequisite; no reinsertion"),
                    );
                    continue;
                }
                self.event(observer, "module-loading", name);
                insert_module(&bytes)
                    .map_err(|e| io::Error::new(e.kind(), format!("init_module {name}: {e}")))?;
                if !loaded_modules()?.contains(name) {
                    return Err(fail(&format!(
                        "module {name} absent after successful init_module"
                    )));
                }
                self.event(observer, "module-loaded", name);
            }
            Ok(())
        })();
        self.phase = if result.is_ok() {
            Phase::Loaded
        } else {
            Phase::Failed
        };
        result
    }
    pub fn finish(self) -> io::Result<StagedPlatform> {
        if self.phase != Phase::Loaded {
            return Err(fail("platform stages incomplete"));
        }
        Ok(StagedPlatform {
            events: self.events,
            ini_ram_path: "/tmp/mi-bootstrap-stage/mik_init.ini",
        })
    }
}

/// Convenience composition. Failed partial progress is delivered to `observer`.
///
/// # Safety
/// Same exclusive fresh-boot history contract as `ColdPlatform::begin`.
pub unsafe fn prepare_cold_platform(
    policy: &impl cold_start::IniPolicy,
    mut observer: impl FnMut(&Event),
) -> io::Result<StagedPlatform> {
    let mut staging = unsafe { ColdPlatform::begin()? };
    staging.mount_stock(&mut observer)?;
    staging.stage_configuration(policy, &mut observer)?;
    staging.load_modules(policy, &mut observer)?;
    staging.finish()
}

fn fail(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}
fn guard() -> io::Result<()> {
    if unsafe { crate::getpid() } != 1 || unsafe { crate::geteuid() } != 0 {
        return Err(fail("cold staging requires root PID 1"));
    }
    if !fs::read_to_string("/proc/cmdline")?
        .split_whitespace()
        .any(|s| s == SERIAL)
        || fs::read_to_string("/proc/sys/kernel/osrelease")?.trim() != "4.19.116+"
    {
        return Err(fail("wrong G06 identity or kernel"));
    }
    let mut count = 0;
    for entry in fs::read_dir("/sys/class/block")? {
        let entry = entry?;
        if entry.file_name().to_string_lossy().starts_with("mmcblk0") {
            count += 1;
            if fs::read_to_string(entry.path().join("ro"))?.trim() != "1" {
                return Err(fail("all visible eMMC nodes must already be read-only"));
            }
        }
    }
    if count == 0 {
        return Err(fail("no protected eMMC nodes"));
    }
    Ok(())
}
fn verify_partition(name: &str, sectors: u64, start: u64, part: &str) -> io::Result<()> {
    let base = format!("/sys/class/block/{name}");
    for (field, expected) in [("size", sectors), ("start", start)] {
        if fs::read_to_string(format!("{base}/{field}"))?.trim() != expected.to_string() {
            return Err(fail(&format!("unexpected {name} {field}")));
        }
    }
    if !fs::read_to_string(format!("{base}/uevent"))?
        .lines()
        .any(|l| l == format!("PARTNAME={part}"))
        || !fs::metadata(format!("/dev/{name}"))?
            .file_type()
            .is_block_device()
    {
        return Err(fail("partition identity/node mismatch"));
    }
    Ok(())
}

#[repr(C)]
struct LoopInfo {
    device: u64,
    inode: u64,
    rdevice: u64,
    offset: u64,
    size_limit: u64,
    number: u32,
    encrypt_type: u32,
    encrypt_key_size: u32,
    flags: u32,
    file_name: [u8; 64],
    crypt_name: [u8; 64],
    encrypt_key: [u8; 32],
    init: [u64; 2],
}
const _: () = {
    assert!(std::mem::size_of::<LoopInfo>() == 232);
    assert!(std::mem::offset_of!(LoopInfo, offset) == 24);
    assert!(std::mem::offset_of!(LoopInfo, flags) == 52);
    assert!(std::mem::offset_of!(LoopInfo, file_name) == 56);
};
impl Default for LoopInfo {
    fn default() -> Self {
        Self {
            device: 0,
            inode: 0,
            rdevice: 0,
            offset: 0,
            size_limit: 0,
            number: 0,
            encrypt_type: 0,
            encrypt_key_size: 0,
            flags: 0,
            file_name: [0; 64],
            crypt_name: [0; 64],
            encrypt_key: [0; 32],
            init: [0; 2],
        }
    }
}
fn loop_matches(info: &LoopInfo, device: u64, inode: u64, rdevice: u64) -> bool {
    info.device == device
        && info.inode == inode
        && info.rdevice == rdevice
        && info.offset == VENDOR_OFFSET
        && info.size_limit == 0
        && info.flags & 1 == 1
        && info.encrypt_type == 0
        && info.encrypt_key_size == 0
}
fn ensure_loop() -> io::Result<()> {
    let backing = File::open("/dev/mmcblk0p28")?;
    let metadata = backing.metadata()?;
    let device = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/loop1")?;
    let mut info = LoopInfo::default();
    let result = unsafe { crate::ioctl(device.as_raw_fd(), LOOP_GET_STATUS64 as _, &mut info) };
    if result == 0 {
        if loop_matches(&info, metadata.dev(), metadata.ino(), metadata.rdev()) {
            return Ok(());
        }
        return Err(fail(
            "loop1 already has a different or writable mapping; left untouched",
        ));
    }
    if io::Error::last_os_error().raw_os_error() != Some(6) {
        return Err(io::Error::last_os_error());
    } // ENXIO
    if unsafe { crate::ioctl(device.as_raw_fd(), LOOP_SET_FD as _, backing.as_raw_fd()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    info.offset = VENDOR_OFFSET;
    info.flags = 1;
    if unsafe { crate::ioctl(device.as_raw_fd(), LOOP_SET_STATUS64 as _, &info) } != 0 {
        return Err(io::Error::last_os_error());
    }
    info = LoopInfo::default();
    if unsafe { crate::ioctl(device.as_raw_fd(), LOOP_GET_STATUS64 as _, &mut info) } != 0 {
        return Err(io::Error::last_os_error());
    }
    if !loop_matches(&info, metadata.dev(), metadata.ino(), metadata.rdev()) {
        return Err(fail("loop1 readback mismatch"));
    }
    Ok(()) // Never detach or auto-clear an adopted or newly mounted loop device.
}

#[derive(Debug, Eq, PartialEq)]
struct Mount {
    point: String,
    root: String,
    device: String,
    fs: String,
    readonly: bool,
}
fn unescape(text: &str) -> io::Result<String> {
    let mut result = Vec::new();
    let mut input = text.as_bytes().iter().copied();
    while let Some(c) = input.next() {
        if c != b'\\' {
            result.push(c);
            continue;
        }
        let code = [input.next(), input.next(), input.next()];
        result.push(match code {
            [Some(b'0'), Some(b'4'), Some(b'0')] => b' ',
            [Some(b'0'), Some(b'1'), Some(b'1')] => b'\t',
            [Some(b'0'), Some(b'1'), Some(b'2')] => b'\n',
            [Some(b'1'), Some(b'3'), Some(b'4')] => b'\\',
            _ => return Err(fail("invalid mountinfo escape")),
        });
    }
    String::from_utf8(result).map_err(|_| fail("invalid mountinfo text"))
}
fn mounts(text: &str) -> io::Result<Vec<Mount>> {
    text.lines()
        .map(|line| {
            let (left, right) = line
                .split_once(" - ")
                .ok_or_else(|| fail("invalid mountinfo separator"))?;
            let l: Vec<_> = left.split_whitespace().collect();
            let r: Vec<_> = right.split_whitespace().collect();
            if l.len() < 6 || r.len() < 3 {
                return Err(fail("short mountinfo record"));
            }
            Ok(Mount {
                point: unescape(l[4])?,
                root: unescape(l[3])?,
                device: l[2].into(),
                fs: r[0].into(),
                readonly: l[5].split(',').chain(r[2].split(',')).any(|v| v == "ro"),
            })
        })
        .collect()
}
fn read_mounts() -> io::Result<Vec<Mount>> {
    mounts(&fs::read_to_string("/proc/self/mountinfo")?)
}
fn covering_mount<'a>(mounts: &'a [Mount], path: &str) -> Option<&'a Mount> {
    mounts
        .iter()
        .filter(|m| {
            m.point == "/"
                || path == m.point
                || path
                    .strip_prefix(&m.point)
                    .is_some_and(|r| r.starts_with('/'))
        })
        .max_by_key(|m| m.point.len())
}
fn validate_mount(mount: &Mount, device: &str, root: &str) -> io::Result<()> {
    if mount.device != device || mount.root != root || mount.fs != "ext4" || !mount.readonly {
        return Err(fail(
            "existing mount does not match expected read-only stock source",
        ));
    }
    Ok(())
}
fn verify_mount(target: &str, source: &str, root: &str) -> io::Result<bool> {
    let mounts = read_mounts()?;
    if mounts.iter().any(|m| {
        m.point
            .strip_prefix(target)
            .is_some_and(|r| r.starts_with('/'))
    }) {
        return Err(fail("unexpected nested stock mount"));
    }
    let name = Path::new(source)
        .file_name()
        .ok_or_else(|| fail("invalid block source"))?
        .to_string_lossy();
    let device = fs::read_to_string(format!("/sys/class/block/{name}/dev"))?;
    let found: Vec<_> = mounts.iter().filter(|m| m.point == target).collect();
    if found.len() > 1 {
        return Err(fail("stacked mounts are not supported"));
    }
    if let Some(m) = found.first() {
        validate_mount(m, device.trim(), root)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
fn ensure_ext4(source: &str, target: &str, root: &str) -> io::Result<()> {
    if verify_mount(target, source, root)? {
        return Ok(());
    }
    fs::create_dir_all(target)?;
    if fs::symlink_metadata(target)?.file_type().is_symlink() {
        return Err(fail("mount point is a symlink"));
    }
    mount_call(
        Some(source),
        target,
        Some("ext4"),
        MS_RDONLY,
        Some("noload"),
    )?;
    if !verify_mount(target, source, root)? {
        return Err(fail("new ext4 mount missing"));
    }
    Ok(())
}
fn ensure_bind(source: &str, target: &str, device: &str, root: &str) -> io::Result<()> {
    if verify_mount(target, device, root)? {
        return Ok(());
    }
    fs::create_dir_all(target)?;
    if fs::symlink_metadata(target)?.file_type().is_symlink() {
        return Err(fail("mount point is a symlink"));
    }
    mount_call(Some(source), target, None, MS_BIND, None)?;
    mount_call(None, target, None, MS_BIND | MS_REMOUNT | MS_RDONLY, None)?;
    if !verify_mount(target, device, root)? {
        return Err(fail("new read-only bind missing"));
    }
    Ok(())
}
fn ensure_alias(source: &str, target: &str) -> io::Result<()> {
    match fs::symlink_metadata(target) {
        Ok(_) if fs::canonicalize(source)? == fs::canonicalize(target)? => Ok(()),
        Ok(_) => Err(fail("existing alias points to a different object")),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            if let Some(parent) = Path::new(target).parent() {
                fs::create_dir_all(parent)?;
            }
            std::os::unix::fs::symlink(source, target)
        }
        Err(e) => Err(e),
    }
}
fn read_bounded(path: &str, max: usize) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() > max as u64 {
        return Err(fail("staging source too large"));
    }
    let mut bytes = vec![];
    file.take(max as u64 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > max {
        return Err(fail("staging source grew beyond bound"));
    }
    Ok(bytes)
}
fn validate_module(bytes: &[u8]) -> io::Result<()> {
    if bytes.len() < 52
        || bytes[..7] != *b"\x7fELF\x01\x01\x01"
        || bytes[16..20] != [1, 0, 40, 0]
        || !bytes
            .windows(b"vermagic=4.19.116+ ".len())
            .any(|w| w == b"vermagic=4.19.116+ ")
    {
        return Err(fail("expected stock ARM32 relocatable 4.19.116+ module"));
    }
    Ok(())
}
fn loaded_modules() -> io::Result<BTreeSet<String>> {
    Ok(fs::read_to_string("/proc/modules")?
        .lines()
        .filter_map(|l| l.split_whitespace().next().map(String::from))
        .collect())
}
fn validate_inherited_modules(loaded: &BTreeSet<String>) -> io::Result<BTreeSet<String>> {
    let mut inherited = BTreeSet::new();
    for name in cold_start::PRE_DISPLAY_MODULES
        .iter()
        .chain(std::iter::once(&cold_start::FRAMEBUFFER_MODULE))
    {
        if loaded.contains(*name) {
            if !WATCHDOG_MODULES.contains(name) {
                return Err(fail(
                    "non-watchdog MI module already loaded; require a fresh staging attempt",
                ));
            }
            inherited.insert((*name).to_owned());
        }
    }
    Ok(inherited)
}
fn inherited_watchdog_modules() -> io::Result<BTreeSet<String>> {
    validate_inherited_modules(&loaded_modules()?)
}

fn mount_call(
    source: Option<&str>,
    target: &str,
    kind: Option<&str>,
    flags: usize,
    data: Option<&str>,
) -> io::Result<()> {
    let string = |s: &str| CString::new(s).map_err(|_| fail("NUL in mount argument"));
    let source = source.map(string).transpose()?;
    let target = string(target)?;
    let kind = kind.map(string).transpose()?;
    let data = data.map(string).transpose()?;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        extern "C" {
            fn mount(
                source: *const std::os::raw::c_char,
                target: *const std::os::raw::c_char,
                kind: *const std::os::raw::c_char,
                flags: usize,
                data: *const std::ffi::c_void,
            ) -> i32;
        }
        let ptr = |s: &Option<CString>| s.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());
        if unsafe {
            mount(
                ptr(&source),
                target.as_ptr(),
                ptr(&kind),
                flags,
                ptr(&data).cast(),
            )
        } != 0
        {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    {
        let _ = (source, target, kind, flags, data);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Linux mount required",
        ))
    }
}
fn insert_module(bytes: &[u8]) -> io::Result<()> {
    #[cfg(all(target_arch = "arm", any(target_os = "linux", target_os = "android")))]
    {
        extern "C" {
            fn syscall(number: i32, ...) -> isize;
        }
        let params = b"\0"; // Preserve stock module parameters, including HDCP defaults.
        if unsafe { syscall(128, bytes.as_ptr(), bytes.len(), params.as_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    #[cfg(not(all(target_arch = "arm", any(target_os = "linux", target_os = "android"))))]
    {
        let _ = bytes;
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "ARM Linux init_module required",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loop_wire_layout_and_identity_checks() {
        assert_eq!(std::mem::size_of::<LoopInfo>(), 232);
        assert_eq!(std::mem::offset_of!(LoopInfo, offset), 24);
        assert_eq!(std::mem::offset_of!(LoopInfo, flags), 52);
        assert_eq!(std::mem::offset_of!(LoopInfo, file_name), 56);
        let mut i = LoopInfo {
            device: 1,
            inode: 2,
            rdevice: 3,
            offset: VENDOR_OFFSET,
            flags: 1,
            ..Default::default()
        };
        assert!(loop_matches(&i, 1, 2, 3));
        i.offset += 512;
        assert!(!loop_matches(&i, 1, 2, 3));
        i.offset = VENDOR_OFFSET;
        i.flags = 0;
        assert!(!loop_matches(&i, 1, 2, 3));
    }
    #[test]
    fn mounts_reject_writable_and_wrong_sources() {
        let parsed = mounts("1 0 7:1 /lib /vendor/lib rw - ext4 /dev/loop1 ro,noload\n").unwrap();
        validate_mount(&parsed[0], "7:1", "/lib").unwrap();
        assert!(validate_mount(&parsed[0], "7:2", "/lib").is_err());
        let wrong = mounts("1 0 7:1 /lib /vendor/lib rw - ext4 /dev/loop1 rw\n").unwrap();
        assert!(validate_mount(&wrong[0], "7:1", "/lib").is_err());
        assert!(mounts("invalid").is_err());
        assert!(unescape("/bad\\777").is_err());
    }
    #[test]
    fn mount_coverage_respects_path_boundaries() {
        let parsed =
            mounts("1 0 0:1 / / rw - rootfs rootfs rw\n2 1 0:2 / /tmp rw - tmpfs tmpfs rw\n")
                .unwrap();
        assert_eq!(covering_mount(&parsed, "/tmp/ini").unwrap().fs, "tmpfs");
        assert_eq!(covering_mount(&parsed, "/tmp-other").unwrap().fs, "rootfs");
    }
    #[test]
    fn ordinary_process_cannot_stage_or_load() {
        assert!(unsafe { ColdPlatform::begin() }.is_err());
    }
    #[test]
    fn out_of_order_and_failed_stages_are_not_retryable() {
        let mut stage = ColdPlatform {
            events: vec![],
            phase: Phase::Failed,
            inherited: BTreeSet::new(),
        };
        assert!(stage.enter(Phase::New, Phase::Mounting).is_err());
        assert!(stage.finish().is_err());
    }
    #[test]
    fn only_the_three_existing_watchdog_modules_can_be_adopted() {
        let mut loaded: BTreeSet<String> =
            WATCHDOG_MODULES.iter().map(|s| (*s).to_owned()).collect();
        loaded.insert("irrelevant_network_driver".into());
        let adopted = validate_inherited_modules(&loaded).unwrap();
        assert_eq!(adopted.len(), 3);
        for name in cold_start::PRE_DISPLAY_MODULES
            .iter()
            .chain(std::iter::once(&cold_start::FRAMEBUFFER_MODULE))
        {
            if WATCHDOG_MODULES.contains(name) {
                continue;
            }
            loaded.insert((*name).into());
            assert!(validate_inherited_modules(&loaded).is_err(), "{name}");
            loaded.remove(*name);
        }
    }
}
