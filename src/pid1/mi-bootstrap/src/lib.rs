//! Exact G06/V109 MI initialization adapters for the owning PID 1.
//!
//! This module does not initialize SYS from guessed security/memory-map data,
//! load modules, set readiness bits, start services, or configure panel power.
//! It requires the kernel's SYS/FLASH initialization to have succeeded first.
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io,
    os::{
        fd::AsRawFd,
        raw::{c_int, c_ulong},
    },
    sync::atomic::{AtomicBool, Ordering},
};

pub mod amplifier;
pub mod speaker;
pub mod amp_handles;
pub mod cold_start;
pub mod owned;
pub mod staging;
mod display_boot;
pub use display_boot::DisplayHandoff;

pub const SERIAL: &str = "androidboot.serialno=4C0490290FA05A0C0";

// MI_SYS_Init overwrites the status table after FLASH was initialized.
// Selector3 is informational, not a cold-boot readiness prerequisite.
pub(crate) const KERNEL_PREREQUISITES: &[u32] = &[40, 0];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Module {
    Display,
    DisplayOutput,
    Osd,
    Audio,
    AudioOutput,
    Pcm,
}

impl Module {
    pub fn name(self) -> &'static str {
        match self {
            Self::Display => "disp",
            Self::DisplayOutput => "dispout",
            Self::Osd => "osd",
            Self::Audio => "audio",
            Self::AudioOutput => "aout",
            Self::Pcm => "pcm",
        }
    }
    pub fn status_selector(self) -> u32 {
        match self {
            Self::Display => 6,
            Self::DisplayOutput => 7,
            Self::Osd => 9,
            Self::Audio => 17,
            Self::AudioOutput => 18,
            Self::Pcm => 19,
        }
    }
    pub fn init_request(self) -> u32 {
        match self {
            Self::Display => 0xc00c1200,
            Self::DisplayOutput => 0xc0013200,
            Self::Osd => 0xc00c1400,
            Self::Audio => 0xc0011000,
            Self::AudioOutput => 0x40011100,
            Self::Pcm => 0xc0013400,
        }
    }
    fn payload(self) -> Vec<u8> {
        match self {
            Self::Display => [6u32.to_le_bytes(), 6u32.to_le_bytes(), 0u32.to_le_bytes()].concat(),
            Self::Osd => vec![0; 12],
            _ => vec![0],
        }
    }
}

/// Implemented by the real ioctl transport and by offline contract tests.
pub trait Transport {
    /// Must fail unless the caller owns an exclusive fresh-boot session.
    fn ensure_exclusive_cold_boot(&self) -> io::Result<()>;
    fn status(&mut self, selector: u32) -> io::Result<bool>;
    fn initialize(&mut self, module: Module, payload: &mut [u8]) -> io::Result<i32>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Observation {
    pub module: Module,
    pub previously_ready: bool,
    /// None means the existing ready state caused initialization to be skipped.
    pub init_result: Option<i32>,
    /// Metadata from SYS; a direct INIT need not publish this vendor status bit.
    pub ready_after: bool,
}

/// Initialize an explicit subset in caller-specified order, stopping at failure.
/// A successful INIT result is recorded; it is never converted into a fake
/// SYS readiness write. Boot-logo, fbdev, output routes and amplifier are separate.
pub fn initialize<T: Transport>(io: &mut T, modules: &[Module]) -> io::Result<Vec<Observation>> {
    io.ensure_exclusive_cold_boot()?;
    if io.status(1)? {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "SYS_USER is already ready; cold bootstrap refused",
        ));
    }
    for &selector in KERNEL_PREREQUISITES {
        if !io.status(selector)? {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("prerequisite MI selector {selector} is not ready; no INIT issued"),
            ));
        }
    }
    let mut result = Vec::new();
    for &module in modules {
        let before = io.status(module.status_selector())?;
        if before {
            result.push(Observation {
                module,
                previously_ready: true,
                init_result: None,
                ready_after: true,
            });
            continue;
        }
        let code = io.initialize(module, &mut module.payload())?;
        // These are exactly the two values accepted by the stock MI wrappers.
        if code != 0 && code != 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "{} INIT returned {code}; subsequent INITs skipped",
                    module.name()
                ),
            ));
        }
        result.push(Observation {
            module,
            previously_ready: false,
            init_result: Some(code),
            ready_after: io.status(module.status_selector())?,
        });
    }
    Ok(result)
}

#[repr(C)]
struct Envelope {
    bytes: u16,
    reserved: [u8; 6],
    address: u64,
}
#[repr(C)]
struct Attribute {
    kind: u64,
    address: u64,
}
#[repr(C)]
struct Status {
    selector: u32,
    ready: u8,
    reserved: [u8; 3],
}
extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn geteuid() -> u32;
    fn getpid() -> c_int;
}

static BOOT_OWNERSHIP_CLAIMED: AtomicBool = AtomicBool::new(false);

/// Non-cloneable permission from the audited PID-1 cold-start phase.
/// Kernel status bits, absence of FDs, and process names cannot establish it.
pub struct ColdBootProvenance {
    owner_pid: c_int,
}

impl ColdBootProvenance {
    /// Claim the one initialization session before starting any vendor services.
    ///
    /// # Safety
    /// The caller must be the freshly booted owning PID 1. No userspace MI
    /// initialization or client access may have happened since kernel boot;
    /// kernel INI auto-initialization is allowed and is skipped using status.
    /// The caller must have staged the verified MI_FLASH_INIT=1 profile before
    /// loading mik and retained it through kernel initialization. SYS init
    /// clears earlier FLASH status metadata, so selector3 cannot verify this.
    /// The caller must serialize all hardware access for this session. Do not
    /// call this from a REPL, a service restart, or after vendor services ran,
    /// even if all relevant metadata bits are false. Runtime checks cannot
    /// reconstruct that historical fact.
    pub unsafe fn claim_exclusive_boot_phase() -> io::Result<Self> {
        let owner_pid = getpid();
        if owner_pid != 1 || geteuid() != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "cold initialization ownership can only be claimed by root PID 1",
            ));
        }
        BOOT_OWNERSHIP_CLAIMED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "cold initialization session was already claimed",
                )
            })?;
        Ok(Self { owner_pid })
    }
}

/// Owns MI descriptors. Embed this in PID 1; do not expose it directly to apps.
pub struct Device {
    sys: File,
    modules: BTreeMap<Module, File>,
    cold_boot: Option<ColdBootProvenance>,
    attempted: BTreeSet<Module>,
    completed: BTreeSet<Module>,
    display_handoff_attempted: bool,
}

impl Device {
    /// Finish the cold display init without publishing SYS_USER or opening fbdev.
    pub fn finish_display_bootstrap(&mut self) -> io::Result<DisplayHandoff> {
        display_boot::run(self)
    }

    /// Only opens SYS. This path permits read-only status queries.
    pub fn open() -> io::Result<Self> {
        if !fs::read_to_string("/proc/cmdline")?
            .split_whitespace()
            .any(|x| x == SERIAL)
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "wrong TV serial",
            ));
        }
        assert_eq!(std::mem::size_of::<Envelope>(), 16);
        assert_eq!(std::mem::size_of::<Attribute>(), 16);
        assert_eq!(std::mem::size_of::<Status>(), 8);
        Ok(Self {
            sys: open_node("sys", false)?,
            modules: BTreeMap::new(),
            cold_boot: None,
            attempted: BTreeSet::new(),
            completed: BTreeSet::new(),
            display_handoff_attempted: false,
        })
    }

    /// Consume cold-start provenance and check existing eMMC protection.
    /// This never creates provenance from false metadata or absent handles.
    pub fn permit_initialization(&mut self, provenance: ColdBootProvenance) -> io::Result<()> {
        if provenance.owner_pid != unsafe { getpid() } || unsafe { geteuid() } != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "cold-start provenance belongs to a different process",
            ));
        }
        let mut count = 0;
        for entry in fs::read_dir("/sys/class/block")? {
            let entry = entry?;
            if entry.file_name().to_string_lossy().starts_with("mmcblk0") {
                count += 1;
                if fs::read_to_string(entry.path().join("ro"))?.trim() != "1" {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "eMMC is not read-only",
                    ));
                }
            }
        }
        if count == 0 {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no protected eMMC devices found",
            ));
        }
        if self.status(1)? {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "SYS_USER already ready; cold bootstrap refused",
            ));
        }
        self.cold_boot = Some(provenance);
        Ok(())
    }
}

fn open_node(name: &str, writable: bool) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(writable)
        .open(format!("/dev/mik/{name}"))
        .or_else(|_| {
            OpenOptions::new()
                .read(true)
                .write(writable)
                .open(format!("/dev/mik!{name}"))
        })
}

fn call(fd: &File, request: u32, address: u64, bytes: u16) -> io::Result<i32> {
    let mut wire = Envelope {
        bytes,
        reserved: [0; 6],
        address,
    };
    // The kernel synchronously copies the inner and nested arguments. All
    // caller-owned buffers remain alive until this syscall returns.
    let code = unsafe {
        ioctl(
            fd.as_raw_fd(),
            request as c_ulong,
            &mut wire as *mut Envelope,
        )
    };
    if code < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(code)
    }
}

impl Transport for Device {
    fn ensure_exclusive_cold_boot(&self) -> io::Result<()> {
        if self.cold_boot.as_ref().map(|p| p.owner_pid) != Some(unsafe { getpid() }) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "INIT requires exclusive cold-boot provenance from PID 1",
            ));
        }
        Ok(())
    }
    fn status(&mut self, selector: u32) -> io::Result<bool> {
        let mut status = Status {
            selector,
            ready: 0,
            reserved: [0; 3],
        };
        let mut attr = Attribute {
            kind: 5,
            address: (&mut status as *mut Status) as u64,
        };
        let code = call(
            &self.sys,
            0xc0101805,
            (&mut attr as *mut Attribute) as u64,
            16,
        )?;
        if code != 0 || status.selector != selector {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("SYS GET selector {selector}: vendor result {code}"),
            ));
        }
        Ok(status.ready == 1)
    }
    fn initialize(&mut self, module: Module, payload: &mut [u8]) -> io::Result<i32> {
        self.ensure_exclusive_cold_boot()?;
        if payload != module.payload() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unexpected V109 init payload",
            ));
        }
        if !self.attempted.insert(module) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "INIT was already attempted for this module; no retry in the same boot",
            ));
        }
        if !self.modules.contains_key(&module) {
            self.modules.insert(module, open_node(module.name(), true)?);
        }
        let result = call(
            &self.modules[&module],
            module.init_request(),
            payload.as_mut_ptr() as u64,
            payload.len() as u16,
        )?;
        if result == 0 || result == 2 {
            self.completed.insert(module);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct Fake {
        cold: bool,
        user_ready: bool,
        missing: Option<u32>,
        fail: Option<Module>,
        called: Vec<Module>,
    }
    impl Fake {
        fn cold() -> Self {
            Self {
                cold: true,
                ..Self::default()
            }
        }
    }
    impl Transport for Fake {
        fn ensure_exclusive_cold_boot(&self) -> io::Result<()> {
            if self.cold {
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "no cold provenance",
                ))
            }
        }
        fn status(&mut self, s: u32) -> io::Result<bool> {
            Ok((s == 1 && self.user_ready) || ([40, 0, 3].contains(&s) && self.missing != Some(s)))
        }
        fn initialize(&mut self, m: Module, _: &mut [u8]) -> io::Result<i32> {
            self.called.push(m);
            Ok(if self.fail == Some(m) { 7 } else { 0 })
        }
    }
    #[test]
    fn absent_sys_kernel_prevents_every_init() {
        let mut io = Fake {
            missing: Some(0),
            ..Fake::cold()
        };
        assert!(initialize(&mut io, &[Module::Display, Module::Osd]).is_err());
        assert!(io.called.is_empty());
    }
    #[test]
    fn missing_module_registration_prevents_every_init() {
        let mut io = Fake { missing: Some(40), ..Fake::cold() };
        assert!(initialize(&mut io, &[Module::Display]).is_err());
        assert!(io.called.is_empty());
    }
    #[test]
    fn erased_flash_metadata_does_not_reinitialize_flash_or_block_display() {
        let mut io = Fake { missing: Some(3), ..Fake::cold() };
        let observed = initialize(&mut io, &[Module::Display]).unwrap();
        assert_eq!(io.called, vec![Module::Display]);
        assert_eq!(observed[0].init_result, Some(0));
        assert!(!io.status(3).unwrap());
    }
    #[test]
    fn display_failure_does_not_initialize_following_modules() {
        let mut io = Fake {
            fail: Some(Module::Display),
            ..Fake::cold()
        };
        assert!(initialize(&mut io, &[Module::Display, Module::Osd]).is_err());
        assert_eq!(io.called, vec![Module::Display]);
    }
    #[test]
    fn success_does_not_claim_metadata_ready() {
        let mut io = Fake::cold();
        let observed = initialize(&mut io, &[Module::Display, Module::Osd]).unwrap();
        assert!(observed
            .iter()
            .all(|o| o.init_result == Some(0) && !o.ready_after));
    }
    #[test]
    fn false_audio_metadata_without_cold_provenance_never_initializes() {
        // Mirrors live AOUT/PCM: metadata can be false with open, audible streams.
        let mut io = Fake::default();
        assert!(initialize(&mut io, &[Module::AudioOutput, Module::Pcm]).is_err());
        assert!(io.called.is_empty());
    }
    #[test]
    fn user_ready_rejects_even_claimed_cold_session() {
        let mut io = Fake {
            user_ready: true,
            ..Fake::cold()
        };
        assert!(initialize(&mut io, &[Module::AudioOutput, Module::Pcm]).is_err());
        assert!(io.called.is_empty());
    }
    #[test]
    fn ordinary_process_cannot_claim_pid1_cold_provenance() {
        assert!(unsafe { ColdBootProvenance::claim_exclusive_boot_phase() }.is_err());
    }
}
