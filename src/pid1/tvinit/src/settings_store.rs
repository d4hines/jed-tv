//! Settings filesystem effects. Lean decides identity acceptance and commit order.
use std::{
    fs::{self, DirBuilder, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    os::{
        fd::AsRawFd,
        unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt},
    },
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        OnceLock,
    },
};

use super::lean_runtime::{self, Object};

extern "C" {
    fn initialize_tv_TV_SettingsFile(builtin: u8) -> Object;
    fn tv_settings_file_parse(text: Object) -> u64;
    fn tv_settings_file_default(unused: u32) -> u64;
    fn tv_settings_file_format(pin: u32, mask: u32, grants: u32, offline: u8) -> Object;
    fn lean_mk_string_from_bytes(data: *const u8, length: usize) -> Object;
    fn tv_storage_directory_accepted(mode: u32, uid: u32, owner: u32) -> u32;
    fn tv_storage_file_accepted(mode: u32, uid: u32, owner: u32, links: u64, bytes: u64) -> u32;
    fn tv_storage_block_accepted(mode: u32, actual: u64, expected: u64) -> u32;
    fn tv_storage_cache_geometry_accepted(start: u64, sectors: u64) -> u32;
    fn tv_storage_filesystem_accepted(magic: u32, blocks: u32, shift: u32) -> u32;
    fn tv_storage_mount_decision(actual: u64, expected: u64, owner: u32) -> u32;
    fn tv_storage_commit_start(unused: u32) -> u32;
    fn tv_storage_commit_next(stage: u32, ok: u32) -> u32;
}

static CODEC: OnceLock<Result<(), String>> = OnceLock::new();
fn codec() -> io::Result<()> {
    // This module is called synchronously by Core on the single Lean executor.
    match CODEC.get_or_init(|| unsafe {
        lean_runtime::consume_io_unit(initialize_tv_TV_SettingsFile(1))
            .map_err(|error| error.to_string())
    }) {
        Ok(()) => Ok(()),
        Err(message) => Err(error(message)),
    }
}

#[repr(C)]
struct LeanString {
    header: [u8; 8],
    size: usize,
    capacity: usize,
    length: usize,
}

fn parse_policy(text: Option<&str>) -> io::Result<u64> {
    let _executor = lean_runtime::ThreadGuard::attach();
    codec()?;
    let word = unsafe {
        match text {
            Some(text) => {
                tv_settings_file_parse(lean_mk_string_from_bytes(text.as_ptr(), text.len()))
            }
            None => tv_settings_file_default(0),
        }
    };
    if word & (1 << 32) != 0 {
        return Err(error("Invalid parental settings format"));
    }
    Ok(word)
}

fn format_policy(pin: u32, mask: u32, grants: u32, offline: bool) -> io::Result<Vec<u8>> {
    let _executor = lean_runtime::ThreadGuard::attach();
    codec()?;
    unsafe {
        let object = tv_settings_file_format(pin, mask, grants, offline.into());
        let string = &*object.cast::<LeanString>();
        debug_assert_eq!(string.header[7], 249);
        let bytes = std::slice::from_raw_parts(
            object.cast::<u8>().add(std::mem::size_of::<LeanString>()),
            string.size - 1,
        )
        .to_vec();
        lean_runtime::release_object(object);
        if bytes.is_empty() {
            Err(error("Invalid parental settings values"))
        } else {
            Ok(bytes)
        }
    }
}

pub const APPS: [(&str, u32); 7] = [
    ("YouTube", 1),
    ("Doom", 2),
    ("Slides", 3),
    ("Infinite RPG", 5),
    ("City flight", 6),
    ("Axiom", 7),
    ("Snake", 8),
];
pub fn app_id(path: &str) -> u32 {
    match path {
        "/bin/ytplay" | "/bin/lean/ytplay" => 1,
        "/bin/doomtv" | "/bin/lean/doomtv" => 2,
        "/bin/slides" => 3,
        "/bin/rpgtv" => 5,
        "/bin/flighttv" => 6,
        "/bin/axiomtv" => 7,
        "/bin/snaketv" => 8,
        _ => 0,
    }
}
/// The Core effect owns this storage handle. Menus receive only the enabled
/// mask from Core; they never receive this handle or the packed startup policy.
/// A qualified private directory without loading parental policy.
pub(super) fn storage_root() -> io::Result<PathBuf> {
    directory()
}

pub(super) struct CoreStore {
    root: PathBuf,
}

impl CoreStore {
    /// Return Core's startup word (TV.Core.decodePolicy): PIN, enabled apps,
    /// offline and grants.
    /// An error is not a default policy: the native boundary must report Core's
    /// load-failed bit and retain no writable handle for that failed load.
    pub(super) fn open() -> io::Result<(Self, u64)> {
        Self::read_from(directory()?)
    }

    pub(super) fn read_from(root: PathBuf) -> io::Result<(Self, u64)> {
        let text = read_policy(&root)?;
        let policy = parse_policy(text.as_deref())?;
        Ok((Self { root }, policy))
    }

    /// Persist the policy supplied by Core. Callers must propagate failure;
    /// reporting success after a failed save would desynchronize Core/storage.
    pub(super) fn save(&self, pin: u32, enabled: u32, grants: u32, offline: bool) -> io::Result<()> {
        let bytes = format_policy(pin, enabled, grants, offline)?;
        atomic_write(&self.root.join("parental"), &bytes)
    }
}

fn error(s: &str) -> io::Error {
    io::Error::other(s)
}
fn number(path: &str) -> io::Result<u64> {
    super::storage_records::number(&fs::read(path)?)
        .ok_or_else(|| error("Invalid sysfs decimal number"))
}

fn private_directory(root: &Path) -> io::Result<()> {
    match fs::symlink_metadata(root) {
        Ok(meta) => {
            if unsafe { tv_storage_directory_accepted(meta.mode(), meta.uid(), libc::geteuid()) }
                == 0
            {
                return Err(error(
                    "Settings directory must be owner-only and owned by the supervisor",
                ));
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            DirBuilder::new().mode(0o700).create(root)?;
        }
        Err(e) => return Err(e),
    }
    Ok(())
}

fn cache_mount_required(current_device: u64, owner: u32, expected_device: u64) -> io::Result<bool> {
    match unsafe { tv_storage_mount_decision(current_device, expected_device, owner) } {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(error("Invalid settings mount point owner")),
    }
}

fn directory() -> io::Result<PathBuf> {
    if unsafe { libc::geteuid() } != 0 {
        return Err(error("Settings require PID 1 privileges"));
    }
    let command = fs::read_to_string("/proc/cmdline")?;
    // Android 14 moves androidboot properties out of the kernel command line.
    // Match the exact G13 serial in the kernel's flattened bootconfig view.
    let g13 = command
        .split_whitespace()
        .any(|s| s == "androidboot.serialno=18ACC22D1C47")
        || fs::read_to_string("/proc/bootconfig").is_ok_and(|config| {
            config.lines().any(|line| {
                line.split_once('=').is_some_and(|(key, value)| {
                    key.trim() == "androidboot.serialno" && value.trim() == "\"18ACC22D1C47\""
                })
            })
        });
    let root = if command
        .split_whitespace()
        .any(|s| s == "androidboot.serialno=4C0490290FA05A0C0")
    {
        // Only this G06 cache partition becomes writable. Other eMMC devices
        // retain the native read-only policy. Never format or clear cache.
        let base = "/sys/class/block/mmcblk0p29";
        if unsafe {
            tv_storage_cache_geometry_accepted(
                number(&format!("{base}/start"))?,
                number(&format!("{base}/size"))?,
            )
        } == 0
        {
            return Err(error("Unexpected G06 cache geometry"));
        }
        match fs::symlink_metadata("/settings") {
            Ok(meta) if !meta.is_dir() => return Err(error("Invalid settings mount point")),
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => fs::create_dir("/settings")?,
            Err(e) => return Err(e),
        }
        let device = Path::new("/dev/mmcblk0p29");
        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(device)?;
        let metadata = file.metadata()?;
        let pair = fs::read(format!("{base}/dev"))?;
        let (major, minor) = super::storage_records::device(&pair)
            .ok_or_else(|| error("Invalid cache device identity"))?;
        let expected = libc::makedev(major, minor);
        if unsafe { tv_storage_block_accepted(metadata.mode(), metadata.rdev(), expected) } == 0 {
            return Err(error("Cache node identity mismatch"));
        }
        file.seek(SeekFrom::Start(1024))?;
        let mut superblock = [0; 1024];
        file.read_exact(&mut superblock)?;
        let blocks = u32::from_le_bytes(superblock[4..8].try_into().unwrap());
        let shift = u32::from_le_bytes(superblock[24..28].try_into().unwrap());
        let magic = u16::from_le_bytes(superblock[56..58].try_into().unwrap()) as u32;
        if unsafe { tv_storage_filesystem_accepted(magic, blocks, shift) } == 0 {
            return Err(error("Cache filesystem geometry mismatch"));
        }
        let mountpoint = fs::symlink_metadata("/settings")?;
        if cache_mount_required(mountpoint.dev(), mountpoint.uid(), metadata.rdev())? {
            let mut read_only: libc::c_int = 0;
            if unsafe { libc::ioctl(file.as_raw_fd(), 0x125du32 as _, &mut read_only) } != 0 {
                return Err(io::Error::last_os_error());
            }
            let result = unsafe {
                libc::mount(
                    c"/dev/mmcblk0p29".as_ptr(),
                    c"/settings".as_ptr(),
                    c"ext4".as_ptr(),
                    libc::MS_NOSUID | libc::MS_NODEV | libc::MS_NOEXEC,
                    std::ptr::null(),
                )
            };
            if result != 0 {
                let failure = io::Error::last_os_error();
                read_only = 1;
                unsafe {
                    libc::ioctl(file.as_raw_fd(), 0x125du32 as _, &mut read_only);
                }
                return Err(failure);
            }
        }
        PathBuf::from("/settings/dande-tv")
    } else if command
        .split_whitespace()
        .any(|s| s == "hello.qemu=1" || s.starts_with("tvinit.iface="))
    {
        PathBuf::from("/tmp/dande-tv-settings")
    } else if g13 && Path::new("/settings/dande-tv").is_dir() {
        // The G13 supervisor binds its qualified /metadata directory here.
        PathBuf::from("/settings/dande-tv")
    } else if g13 && Path::new("/metadata/dande-tv").is_dir() {
        PathBuf::from("/metadata/dande-tv/settings")
    } else {
        return Err(error("No qualified settings storage on this system"));
    };
    private_directory(&root)?;
    Ok(root)
}

static TEMPORARY: AtomicU64 = AtomicU64::new(0);
/// An exclusive fresh temporary file avoids following or truncating an existing
/// link. Sync both file and containing directory before reporting success.
pub fn atomic_write(path: &Path, data: &[u8]) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| error("Settings path has no parent"))?;
    let name = path
        .file_name()
        .ok_or_else(|| error("Settings path has no filename"))?
        .to_string_lossy();
    let temporary = parent.join(format!(
        ".{name}.{}.{}.new",
        std::process::id(),
        TEMPORARY.fetch_add(1, Ordering::Relaxed)
    ));
    commit(|action| match action {
        CommitAction::Create(file) => {
            *file = Some(
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
                    .open(&temporary)?,
            );
            Ok(())
        }
        CommitAction::Write(file) => file.write_all(data),
        CommitAction::SyncFile(file) => file.sync_all(),
        CommitAction::Rename => fs::rename(&temporary, path),
        CommitAction::SyncDirectory => File::open(parent)?.sync_all(),
        CommitAction::Cleanup => fs::remove_file(&temporary),
    })
}

// Every branch performs one requested native operation. The allocation-free
// Lean state machine alone chooses what follows, including failed cleanup.
pub(super) enum CommitAction<'a> {
    Create(&'a mut Option<File>),
    Write(&'a mut File),
    SyncFile(&'a File),
    Rename,
    SyncDirectory,
    Cleanup,
}

pub(super) fn commit(
    mut perform: impl FnMut(CommitAction<'_>) -> io::Result<()>,
) -> io::Result<()> {
    let mut file = None;
    let mut failure = None;
    let mut stage = unsafe { tv_storage_commit_start(0) };
    loop {
        let result = match stage {
            0 => perform(CommitAction::Create(&mut file)),
            1 => perform(CommitAction::Write(
                file.as_mut().expect("Lean commit created file"),
            )),
            2 => perform(CommitAction::SyncFile(
                file.as_ref().expect("Lean commit created file"),
            )),
            3 => perform(CommitAction::Rename),
            4 => perform(CommitAction::SyncDirectory),
            5 => {
                // Preserve the previous adapter's close-before-cleanup order.
                drop(file.take());
                perform(CommitAction::Cleanup)
            }
            6 => return Ok(()),
            7 => return Err(failure.unwrap_or_else(|| error("Settings commit failed"))),
            _ => return Err(error("Invalid Lean settings commit action")),
        };
        let ok = result.is_ok();
        if failure.is_none() {
            failure = result.err();
        }
        stage = unsafe { tv_storage_commit_next(stage, u32::from(ok)) };
    }
}

fn read_policy(root: &Path) -> io::Result<Option<String>> {
    private_directory(root)?;
    let path = root.join("parental");
    match OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK)
        .open(&path)
    {
        Ok(file) => {
            let meta = file.metadata()?;
            if unsafe {
                tv_storage_file_accepted(
                    meta.mode(),
                    meta.uid(),
                    libc::geteuid(),
                    meta.nlink(),
                    meta.len(),
                )
            } == 0
            {
                return Err(error("Invalid parental settings file"));
            }
            let mut text = String::new();
            file.take(65).read_to_string(&mut text)?;
            Ok(Some(text))
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::{symlink, PermissionsExt};

    fn test_root() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "dande-settings-test-{}-{}",
            std::process::id(),
            TEMPORARY.fetch_add(1, Ordering::Relaxed)
        ));
        private_directory(&root).unwrap();
        root
    }
    #[test]
    fn verified_cache_root_owner_does_not_prevent_reopening_settings() {
        assert!(!cache_mount_required(29, 1000, 29).unwrap());
        assert!(!cache_mount_required(29, 0, 29).unwrap());
        assert!(cache_mount_required(1, 0, 29).unwrap());
        assert!(cache_mount_required(1, 1000, 29).is_err());
    }
    #[test]
    fn file_identity_and_length_are_checked_before_policy_parsing() {
        let root = test_root();
        assert!(read_policy(&root).unwrap().is_none());
        atomic_write(&root.join("parental"), b"not a policy").unwrap();
        assert_eq!(read_policy(&root).unwrap().as_deref(), Some("not a policy"));
        atomic_write(&root.join("target"), b"DANDE1 0000 15\n").unwrap();
        fs::remove_file(root.join("parental")).unwrap();
        symlink(root.join("target"), root.join("parental")).unwrap();
        assert!(read_policy(&root).is_err());
        fs::remove_file(root.join("parental")).unwrap();
        fs::hard_link(root.join("target"), root.join("parental")).unwrap();
        assert!(read_policy(&root).is_err());
        fs::remove_file(root.join("parental")).unwrap();
        fs::rename(root.join("target"), root.join("parental")).unwrap();
        fs::set_permissions(root.join("parental"), fs::Permissions::from_mode(0o644)).unwrap();
        assert!(read_policy(&root).is_err());
        atomic_write(&root.join("parental"), &[b' '; 65]).unwrap();
        assert!(read_policy(&root).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn atomic_replace_never_follows_a_destination_symlink() {
        let root = test_root();
        fs::write(root.join("outside"), b"keep").unwrap();
        symlink(root.join("outside"), root.join("parental")).unwrap();
        atomic_write(&root.join("parental"), b"DANDE1 0000 15\n").unwrap();
        assert_eq!(fs::read(root.join("outside")).unwrap(), b"keep");
        let metadata = fs::symlink_metadata(root.join("parental")).unwrap();
        assert!(!metadata.file_type().is_symlink());
        assert_eq!(metadata.mode() & 0o777, 0o600);
        assert_eq!(metadata.nlink(), 1);
        fs::remove_dir_all(root).unwrap();
    }
}
