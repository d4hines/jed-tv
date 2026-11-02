use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::MetadataExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

const SERIAL: &str = "4C0490290FA05A0C0";
const BUILD: &str = env!("TV_LAUNCHER_ID");
#[cfg(not(feature = "qemu-test"))]
const PROFILE: &str = "TV-LAUNCHER-PROFILE:G06-V109";
#[cfg(feature = "qemu-test")]
const PROFILE: &str = "TV-LAUNCHER-PROFILE:QEMU-TEST-ONLY";
const CACHE_BYTES: u64 = 104857600;
const CACHE_MOUNT: &str = "/launcher-cache";
const REQUEST_NAME: &[u8] = b"request\0";
#[cfg(not(feature = "qemu-test"))]
const DEVICE: &str = "mmcblk0p29";
#[cfg(not(feature = "qemu-test"))]
const START_SECTOR: u64 = 9019392;
#[cfg(feature = "qemu-test")]
const DEVICE: &str = "vda";
#[cfg(feature = "qemu-test")]
const START_SECTOR: u64 = 0;

fn event(message: impl AsRef<str>) {
    let message = message.as_ref();
    eprintln!("tv-launcher: {message}");
    if let Ok(mut file) = OpenOptions::new().write(true).open("/dev/kmsg") {
        let _ = writeln!(file, "<6>tv-launcher: {message}");
    }
}

fn cstring(text: &str) -> CString { CString::new(text).expect("fixed internal path") }

fn mount(source: &str, target: &str, fs: &str, flags: libc::c_ulong) -> io::Result<()> {
    let (source, target, fs) = (cstring(source), cstring(target), cstring(fs));
    if unsafe { libc::mount(source.as_ptr(), target.as_ptr(), fs.as_ptr(), flags, std::ptr::null()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn unmount(target: &str) -> io::Result<()> {
    if unsafe { libc::umount2(cstring(target).as_ptr(), 0) } != 0 { return Err(io::Error::last_os_error()); }
    Ok(())
}

struct Mounts(Vec<&'static str>);

impl Mounts {
    fn add(&mut self, source: &str, target: &'static str, fs: &str, flags: libc::c_ulong) -> io::Result<()> {
        mount(source, target, fs, flags)?;
        self.0.push(target);
        Ok(())
    }

    fn remove(&mut self, target: &'static str) -> io::Result<()> {
        unmount(target)?;
        self.0.retain(|entry| *entry != target);
        Ok(())
    }

    fn cleanup(&mut self) -> bool {
        let mut clean = true;
        while let Some(target) = self.0.pop() {
            if let Err(error) = unmount(target) {
                event(format!("unmount {target} failed: {error}"));
                // A private mount must not leak into Android startup. A detached
                // mount has no remaining launcher FDs and cannot be reached.
                if unsafe { libc::umount2(cstring(target).as_ptr(), libc::MNT_DETACH) } != 0 {
                    event(format!("detach {target} failed: {}", io::Error::last_os_error()));
                }
                clean = false;
            }
        }
        clean
    }
}

fn read_number(path: impl AsRef<Path>) -> io::Result<u64> {
    std::fs::read_to_string(path)?.trim().parse().map_err(io::Error::other)
}

fn request() -> Vec<u8> {
    format!("TV-LAUNCHER/1\nG06\nV109\n{SERIAL}\nhello\n{BUILD}\n").into_bytes()
}

fn device_ready() -> io::Result<String> {
    let sys = format!("/sys/class/block/{DEVICE}");
    let deadline = Instant::now() + Duration::from_secs(2);
    while !Path::new(&sys).exists() {
        if Instant::now() >= deadline { return Err(io::Error::other("cache device did not enumerate within two seconds")); }
        thread::sleep(Duration::from_millis(25));
    }
    let start = if START_SECTOR == 0 { 0 } else { read_number(format!("{sys}/start"))? };
    if start != START_SECTOR || read_number(format!("{sys}/size"))? * 512 != CACHE_BYTES {
        return Err(io::Error::other("cache partition geometry mismatch"));
    }
    let device = if Path::new(&format!("/dev/{DEVICE}")).exists() {
        format!("/dev/{DEVICE}")
    } else {
        format!("/dev/block/{DEVICE}")
    };
    let metadata = std::fs::metadata(&device)?;
    let pair = std::fs::read_to_string(format!("{sys}/dev"))?;
    let (major, minor) = pair.trim().split_once(':').ok_or_else(|| io::Error::other("bad block device ID"))?;
    let (major, minor) = (major.parse::<u32>().map_err(io::Error::other)?, minor.parse::<u32>().map_err(io::Error::other)?);
    if metadata.mode() & libc::S_IFMT != libc::S_IFBLK || metadata.rdev() != libc::makedev(major, minor) {
        return Err(io::Error::other("cache device node identity mismatch"));
    }
    let mut file = File::open(&device)?;
    file.seek(SeekFrom::Start(1024))?;
    let mut sb = [0u8; 1024];
    file.read_exact(&mut sb)?;
    let blocks = u32::from_le_bytes(sb[4..8].try_into().unwrap()) as u64;
    let shift = u32::from_le_bytes(sb[24..28].try_into().unwrap());
    if sb[56..58] != [0x53, 0xef] || shift > 6 || (blocks << (10 + shift)) != CACHE_BYTES {
        return Err(io::Error::other("cache ext4 superblock geometry mismatch"));
    }
    Ok(device)
}

fn request_directory(path: &str) -> io::Result<File> {
    let fd = unsafe { libc::open(cstring(path).as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC) };
    if fd < 0 { return Err(io::Error::last_os_error()); }
    let file = unsafe { File::from_raw_fd(fd) };
    let meta = file.metadata()?;
    if meta.uid() != 0 || meta.gid() != 0 || meta.mode() & 0o777 != 0o700 {
        return Err(io::Error::other("request directory must be root:root mode 0700"));
    }
    Ok(file)
}

fn valid_request(directory: &File) -> io::Result<bool> {
    let fd = unsafe { libc::openat(directory.as_raw_fd(), REQUEST_NAME.as_ptr().cast(), libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK) };
    if fd < 0 {
        let error = io::Error::last_os_error();
        if error.kind() == io::ErrorKind::NotFound { return Ok(false); }
        return Err(error);
    }
    let file = unsafe { File::from_raw_fd(fd) };
    let meta = file.metadata()?;
    if meta.mode() & libc::S_IFMT != libc::S_IFREG || meta.uid() != 0 || meta.gid() != 0
        || meta.mode() & 0o777 != 0o600 || meta.nlink() != 1 || meta.len() > 256 {
        return Err(io::Error::other("request must be a small root:root mode 0600 regular file with one link"));
    }
    let mut content = Vec::new();
    file.take(257).read_to_end(&mut content)?;
    Ok(content == request())
}

fn consume(mounts: &mut Mounts, device: &str) -> io::Result<bool> {
    let flags = libc::MS_NOSUID | libc::MS_NODEV | libc::MS_NOEXEC;
    // Read-only ext4 mount may replay a dirty journal. No formatting or raw
    // block writes are performed. The same stock kernel will mount this FS.
    mounts.add(device, CACHE_MOUNT, "ext4", flags | libc::MS_RDONLY)?;
    let request_dir = format!("{CACHE_MOUNT}/tv-launcher");
    let valid = match request_directory(&request_dir) {
        Ok(directory) => valid_request(&directory)?,
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(error) => return Err(error),
    };
    mounts.remove(CACHE_MOUNT)?;
    if !valid { return Ok(false); }
    mounts.add(device, CACHE_MOUNT, "ext4", flags)?;
    {
        let directory = request_directory(&request_dir)?;
        if !valid_request(&directory)? { return Err(io::Error::other("request changed while remounting")); }
        if unsafe { libc::unlinkat(directory.as_raw_fd(), REQUEST_NAME.as_ptr().cast(), 0) } != 0 {
            return Err(io::Error::last_os_error());
        }
        directory.sync_all()?;
        if unsafe { libc::syncfs(directory.as_raw_fd()) } != 0 { return Err(io::Error::last_os_error()); }
    }
    #[cfg(feature = "qemu-test")]
    if std::fs::read_to_string("/proc/cmdline")?.split_whitespace().any(|word| word == "launcher.test=busy-unmount") {
        std::env::set_current_dir(CACHE_MOUNT)?;
    }
    mounts.remove(CACHE_MOUNT)?;
    // Re-read after a clean unmount to verify that the request is absent.
    mounts.add(device, CACHE_MOUNT, "ext4", flags | libc::MS_RDONLY)?;
    {
        let directory = request_directory(&request_dir)?;
        let result = unsafe { libc::openat(directory.as_raw_fd(), REQUEST_NAME.as_ptr().cast(), libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK) };
        let absent = result < 0 && io::Error::last_os_error().kind() == io::ErrorKind::NotFound;
        if result >= 0 { unsafe { libc::close(result); } }
        if !absent {
            return Err(io::Error::other("consumed request is still present or could not be verified absent"));
        }
    }
    mounts.remove(CACHE_MOUNT)?;
    event("request durably consumed and verified absent");
    Ok(true)
}

fn decide(mounts: &mut Mounts) -> io::Result<bool> {
    mounts.add("devtmpfs", "/dev", "devtmpfs", libc::MS_NOSUID)?;
    mounts.add("proc", "/proc", "proc", 0)?;
    mounts.add("sysfs", "/sys", "sysfs", 0)?;
    let cmdline = std::fs::read_to_string("/proc/cmdline")?;
    let expected = format!("androidboot.serialno={SERIAL}");
    if cmdline.split_whitespace().filter(|word| word.starts_with("androidboot.serialno=")).collect::<Vec<_>>() != [expected.as_str()] {
        return Err(io::Error::other("G06 serial identity missing/mismatched; experiment refused"));
    }
    let device = device_ready()?;
    consume(mounts, &device)
}

fn android() -> ! {
    let _ = std::env::set_current_dir("/");
    event("select=android restoring Magiskinit to /init");
    let _ = std::fs::remove_file("/launcher-self");
    if let Err(error) = std::fs::rename("/init", "/launcher-self") { event(format!("save launcher: {error}")); }
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    match std::fs::rename("/launcher-magiskinit", "/init") {
        Ok(()) => {
            let error = Command::new("/init").arg0("/init").args(&args).exec();
            event(format!("Magiskinit exec failed: {error}; falling back to original stock init"));
        }
        Err(error) => event(format!("restore Magiskinit failed: {error}; using original stock init")),
    }
    let _ = std::fs::remove_file("/init");
    if let Err(error) = std::fs::rename("/launcher-stock-init", "/init") { event(format!("restore stock init: {error}")); }
    let error = Command::new("/init").arg0("/init").args(&args).exec();
    event(format!("fatal: original stock init exec failed: {error}"));
    loop { thread::sleep(Duration::from_secs(60)); }
}

fn control(action: &str) -> io::Result<()> {
    if unsafe { libc::geteuid() } != 0 { return Err(io::Error::other("launcherctl requires root (use Magisk su)")); }
    let cmdline = std::fs::read_to_string("/proc/cmdline")?;
    for expected in [format!("androidboot.serialno={SERIAL}"), format!("androidboot.tv_launcher={BUILD}")] {
        let key = expected.split_once('=').unwrap().0.to_owned() + "=";
        if cmdline.split_whitespace().filter(|word| word.starts_with(key.as_str())).collect::<Vec<_>>() != [expected.as_str()] {
            return Err(io::Error::other("running boot does not match this G06 launcher build"));
        }
    }
    let device = device_ready()?;
    let cache = File::open("/cache")?;
    if cache.metadata()?.dev() != std::fs::metadata(&device)?.rdev() {
        return Err(io::Error::other("/cache is not mounted from the expected physical cache partition"));
    }
    let path = "/cache/tv-launcher";
    if action == "--arm" {
        if unsafe { libc::mkdir(cstring(path).as_ptr(), 0o700) } != 0 && io::Error::last_os_error().kind() != io::ErrorKind::AlreadyExists {
            return Err(io::Error::last_os_error());
        }
    }
    let directory = match request_directory(path) {
        Ok(directory) => directory,
        Err(error) if error.kind() == io::ErrorKind::NotFound && action != "--arm" => {
            println!("No experiment request is pending"); return Ok(());
        }
        Err(error) => return Err(error),
    };
    if action == "--inspect" {
        println!("Matching experiment request pending: {}", valid_request(&directory)?);
        return Ok(());
    }
    if action == "--cancel" {
        if !valid_request(&directory)? { return Err(io::Error::other("no matching request; refusing to remove an unrelated file")); }
        if unsafe { libc::unlinkat(directory.as_raw_fd(), REQUEST_NAME.as_ptr().cast(), 0) } != 0 { return Err(io::Error::last_os_error()); }
    } else {
        // Never overwrite a pending file. Cancel it explicitly before rearming.
        let fd = unsafe { libc::openat(directory.as_raw_fd(), REQUEST_NAME.as_ptr().cast(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC, 0o600) };
        if fd < 0 { return Err(io::Error::last_os_error()); }
        let mut file = unsafe { File::from_raw_fd(fd) };
        file.write_all(&request())?;
        file.sync_all()?;
    }
    directory.sync_all()?;
    if unsafe { libc::syncfs(directory.as_raw_fd()) } != 0 { return Err(io::Error::last_os_error()); }
    println!("{} durable for launcher build {BUILD}", if action == "--arm" { "One-shot Hello request armed" } else { "Request cancelled" });
    Ok(())
}

fn main() {
    if std::process::id() != 1 {
        let args = std::env::args().skip(1).collect::<Vec<_>>();
        if args.len() != 1 || !matches!(args[0].as_str(), "--arm" | "--cancel" | "--inspect") {
            eprintln!("usage: launcherctl --inspect|--arm|--cancel (root, matching G06 launcher boot required)");
            std::process::exit(2);
        }
        if let Err(error) = control(&args[0]) { eprintln!("launcherctl: {error}"); std::process::exit(1); }
        return;
    }
    event(format!("build={BUILD} pid=1 {PROFILE}"));
    let mut mounts = Mounts(Vec::new());
    let experiment = match decide(&mut mounts) {
        Ok(selected) => selected,
        Err(error) => { event(format!("experiment refused: {error}")); false }
    };
    let clean = mounts.cleanup();
    if experiment && clean {
        event("select=hello request consumed; exec retains PID 1; normal reset returns to default Android");
        let error = Command::new("/launcher-hello").args([
            "hello.timeout=90", "hello.return=stock", "hello.display_delay=5",
        ]).exec();
        event(format!("Hello exec failed: {error}; using Android"));
    }
    android();
}
