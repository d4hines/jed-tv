//! Bounded, transport-independent subset of the exact V109 supplicant contract.
//! No receive loop, device discovery, policy downgrade, storage or RPMB handler.
use std::{
    collections::BTreeMap,
    fmt, io,
    sync::atomic::{fence, Ordering},
};
#[cfg(target_os = "linux")]
pub mod linux;
mod vendor;

pub(crate) const SHM_ALLOC_IOCTL: u32 = 0xc010a401;
#[repr(C)]
pub(crate) struct ShmAllocRequest {
    pub size: u64,
    pub flags: u32,
    pub id: i32,
}

pub const LOAD_TA: u32 = 0;
pub const SHM_ALLOC: u32 = 6;
pub const SHM_FREE: u32 = 7;
pub const LOAD_OBJECT: u32 = 100;
pub const OBJECT_SIZE: u32 = 101;
pub const SUCCESS: u32 = 0;
pub const GENERIC: u32 = 0xffff0000;
pub const BAD_PARAMETERS: u32 = 0xffff0006;
pub const ITEM_NOT_FOUND: u32 = 0xffff0008;
pub const NOT_SUPPORTED: u32 = 0xffff000a;
pub const OUT_OF_MEMORY: u32 = 0xffff000c;
pub const SHORT_BUFFER: u32 = 0xffff0010;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Param {
    pub attr: u64,
    pub a: u64,
    pub b: u64,
    pub c: u64,
}
fn value(p: &Param) -> bool {
    (1..=3).contains(&p.attr)
}
fn memref(p: &Param) -> bool {
    (5..=7).contains(&p.attr)
}

/// Canonical network-order UUID octets, independent of native C UUID layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Uuid(pub [u8; 16]);
impl Uuid {
    pub fn from_value(p: &Param) -> Self {
        let mut bytes = [0; 16];
        bytes[..8].copy_from_slice(&p.a.to_le_bytes());
        bytes[8..].copy_from_slice(&p.b.to_le_bytes());
        Self(bytes)
    }
    /// Stock mi_an naming omits the hyphen between UUID groups four and five.
    pub fn vendor_filename(&self) -> String {
        let b = &self.0;
        format!("{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}.ta",b[0],b[1],b[2],b[3],b[4],b[5],b[6],b[7],b[8],b[9],b[10],b[11],b[12],b[13],b[14],b[15])
    }
}
impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.vendor_filename();
        write!(f, "{}-{}", &s[..23], &s[23..s.len() - 3])
    }
}
pub const AUDIO_TA: Uuid = Uuid([
    0x77, 0x8e, 0xe6, 0x8e, 0x6e, 0xe0, 0x11, 0xe8, 0xad, 0xc0, 0xfa, 0x7a, 0xe0, 0x1b, 0xbe, 0xbc,
]);

pub trait Region {
    fn id(&self) -> i32;
    fn len(&self) -> usize;
    /// Copy into owned memory only; must fail without writing beyond the region.
    /// mmap-backed implementations should not expose normal Rust references to
    /// memory that the secure world can asynchronously modify.
    fn write(&mut self, offset: usize, data: &[u8]) -> io::Result<()>;
    /// Snapshot owned shared memory without exposing references to its contents.
    fn read(&self, offset: usize, data: &mut [u8]) -> io::Result<()>;
    fn clear(&mut self) -> io::Result<()>;
}
pub trait Allocator {
    type Region: Region;
    /// Return an owned mapping whose Drop unmaps and closes its kernel SHM fd.
    fn allocate(&mut self, bytes: usize) -> io::Result<Self::Region>;
}
#[derive(Debug)]
pub enum LoadError {
    NotFound,
    Io(io::Error),
    TooLarge,
    Changed,
}
pub trait TaStore {
    /// Read at most max_bytes, returning an opaque signed TA container.
    fn load(&mut self, uuid: Uuid, max_bytes: usize) -> Result<Vec<u8>, LoadError>;
    /// Read an allowlisted vendor object, independently of the TA namespace.
    fn load_object(&mut self, _uuid: Uuid, _max_bytes: usize) -> Result<Vec<u8>, LoadError> {
        Err(LoadError::NotFound)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_regions: usize,
    pub region_bytes: usize,
    pub total_bytes: usize,
    pub ta_bytes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_regions: 64,
            region_bytes: 8 * 1024 * 1024,
            total_bytes: 32 * 1024 * 1024,
            ta_bytes: 8 * 1024 * 1024,
        }
    }
}
struct Owned<R> {
    region: R,
    requested: usize,
}

pub struct Provider<A: Allocator, S: TaStore> {
    // Rust drops fields in declaration order: mappings before the device.
    regions: BTreeMap<i32, Owned<A::Region>>,
    allocator: A,
    store: S,
    used: usize,
    limits: Limits,
}
impl<A: Allocator, S: TaStore> Provider<A, S> {
    pub fn new(allocator: A, store: S, limits: Limits) -> Result<Self, &'static str> {
        if limits.max_regions == 0
            || limits.region_bytes == 0
            || limits.total_bytes < limits.region_bytes
            || limits.ta_bytes == 0
        {
            return Err("invalid TEE provider limits");
        }
        Ok(Self {
            allocator,
            store,
            regions: BTreeMap::new(),
            used: 0,
            limits,
        })
    }
    pub fn owns_region(&self, id: u64) -> bool {
        i32::try_from(id)
            .ok()
            .is_some_and(|id| self.regions.contains_key(&id))
    }
    pub fn owned_regions(&self) -> usize {
        self.regions.len()
    }
    pub fn owned_bytes(&self) -> usize {
        self.used
    }
    /// Caller must have removed the leading transport META params. These
    /// handlers reject META flags, extra arguments, and unknown operations.
    pub fn handle(&mut self, function: u32, params: &mut [Param]) -> u32 {
        match function {
            LOAD_TA => self.load_ta(params),
            SHM_ALLOC => self.alloc(params),
            SHM_FREE => self.free(params),
            LOAD_OBJECT | OBJECT_SIZE => self.vendor_object(function, params),
            _ => NOT_SUPPORTED,
        }
    }
    fn alloc(&mut self, p: &mut [Param]) -> u32 {
        if p.len() != 1 || !value(&p[0]) {
            return BAD_PARAMETERS;
        }
        let Ok(bytes) = usize::try_from(p[0].b) else {
            return OUT_OF_MEMORY;
        };
        if bytes == 0 {
            return BAD_PARAMETERS;
        }
        if bytes > self.limits.region_bytes
            || self.regions.len() >= self.limits.max_regions
            || bytes > self.limits.total_bytes.saturating_sub(self.used)
        {
            return OUT_OF_MEMORY;
        }
        let Ok(mut region) = self.allocator.allocate(bytes) else {
            return OUT_OF_MEMORY;
        };
        let actual = region.len();
        let id = region.id();
        if actual < bytes
            || actual > self.limits.region_bytes
            || actual > self.limits.total_bytes.saturating_sub(self.used)
        {
            return OUT_OF_MEMORY;
        }
        if id < 0 || self.regions.contains_key(&id) {
            return GENERIC;
        }
        if region.clear().is_err() {
            return GENERIC;
        }
        // Finish all mapped-memory clears before returning an ID to secure world.
        fence(Ordering::Release);
        self.regions.insert(
            id,
            Owned {
                region,
                requested: bytes,
            },
        );
        self.used += actual;
        p[0].c = id as u64;
        SUCCESS
    }
    fn free(&mut self, p: &mut [Param]) -> u32 {
        if p.len() != 1 || !value(&p[0]) {
            return BAD_PARAMETERS;
        }
        let Ok(id) = i32::try_from(p[0].b) else {
            return BAD_PARAMETERS;
        };
        let Some(region) = self.regions.remove(&id) else {
            return BAD_PARAMETERS;
        };
        self.used -= region.region.len();
        drop(region);
        SUCCESS
    }
    fn load_ta(&mut self, p: &mut [Param]) -> u32 {
        if p.len() != 2 || !value(&p[0]) || !memref(&p[1]) {
            return BAD_PARAMETERS;
        }
        let id = i32::try_from(p[1].c).ok();
        let found = id.and_then(|id| self.regions.get(&id));
        let destination = if let Some(region) = found {
            let (Ok(offset), Ok(bytes)) = (usize::try_from(p[1].a), usize::try_from(p[1].b)) else {
                return BAD_PARAMETERS;
            };
            if offset
                .checked_add(bytes)
                .filter(|&end| end <= region.requested)
                .is_none()
            {
                return BAD_PARAMETERS;
            }
            Some((id.unwrap(), offset, bytes))
        } else {
            // Exact stock get_memref permits absent output/inout SHM as a
            // NULL size query, but rejects absent INPUT references.
            if p[1].attr == 5 {
                return BAD_PARAMETERS;
            }
            None
        };
        let data = match self
            .store
            .load(Uuid::from_value(&p[0]), self.limits.ta_bytes)
        {
            Ok(data) if data.len() <= self.limits.ta_bytes => data,
            Ok(_) | Err(LoadError::TooLarge) => return OUT_OF_MEMORY,
            Err(LoadError::NotFound) => return ITEM_NOT_FOUND,
            Err(LoadError::Io(_) | LoadError::Changed) => return GENERIC,
        };
        p[1].b = data.len() as u64;
        if let Some((id, offset, capacity)) = destination {
            if data.len() > capacity {
                return SHORT_BUFFER;
            }
            if self
                .regions
                .get_mut(&id)
                .unwrap()
                .region
                .write(offset, &data)
                .is_err()
            {
                return GENERIC;
            }
        }
        // The owner may now serialize/send the reply; copied TA bytes must be
        // globally ordered before that handoff, including on ARM.
        fence(Ordering::Release);
        SUCCESS
    }
}

/// File-only allowlisted TA store. No caller-controlled pathname is accepted.
/// The trusted manifest supplies expected lengths; secure world still performs
/// signature/authentication. A length check is not a replacement for that check.
pub struct DirectoryStore {
    root: std::path::PathBuf,
    manifest: BTreeMap<Uuid, usize>,
}
impl DirectoryStore {
    pub fn new(root: std::path::PathBuf, manifest: BTreeMap<Uuid, usize>) -> Self {
        Self { root, manifest }
    }
    pub fn audio(root: std::path::PathBuf) -> Self {
        Self::new(root, BTreeMap::from([(AUDIO_TA, 145_328)]))
    }
}
impl TaStore for DirectoryStore {
    fn load(&mut self, uuid: Uuid, max_bytes: usize) -> Result<Vec<u8>, LoadError> {
        use std::io::Read;
        let Some(&expected) = self.manifest.get(&uuid) else {
            return Err(LoadError::NotFound);
        };
        if expected > max_bytes {
            return Err(LoadError::TooLarge);
        }
        let path = self.root.join(uuid.vendor_filename());
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            let flag = match std::env::consts::ARCH {
                "arm" => 0x8000,
                "aarch64" | "x86" | "x86_64" => 0x20000,
                _ => {
                    return Err(LoadError::Io(io::Error::new(
                        io::ErrorKind::Unsupported,
                        "unverified Linux O_NOFOLLOW ABI",
                    )))
                }
            };
            options.custom_flags(flag);
        } //O_NOFOLLOW
        let file = options.open(path).map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                LoadError::NotFound
            } else {
                LoadError::Io(e)
            }
        })?;
        let meta = file.metadata().map_err(LoadError::Io)?;
        if !meta.is_file() || meta.len() != expected as u64 {
            return Err(LoadError::Changed);
        }
        let limit = (expected as u64)
            .checked_add(1)
            .ok_or(LoadError::TooLarge)?;
        let mut data = Vec::new();
        data.try_reserve_exact(expected)
            .map_err(|_| LoadError::TooLarge)?;
        file.take(limit)
            .read_to_end(&mut data)
            .map_err(LoadError::Io)?;
        if data.len() != expected {
            return Err(LoadError::Changed);
        }
        Ok(data)
    }
}
#[cfg(test)]
#[path = "../../test/unit/tee-rpc/tests.rs"]
mod tests;
