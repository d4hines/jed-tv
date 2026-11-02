//! Opt-in routing through an existing V109 speaker output. Never opens/closes
//! an output stream or initializes hardware. Only OwnedVolume explicitly changes gain.
//! The caller must serialize all route changes. Snapshots cover the MI graph,
//! not hidden DSP/firmware state; cold initialization is a separate concern.
use crate::mi_pcm::{DeviceTransport, Transport};
use std::{fmt, fs::OpenOptions, io};

const GET_HANDLE: u32 = 0xc0081116;
const GET_COUNT: u32 = 0xc010111e;
const GET_CONNECTED: u32 = 0xc018111f;
const CONNECT: u32 = 0xc00c111c;
const DISCONNECT: u32 = 0xc008111d;
const MAX_CONNECTIONS: u32 = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputHandle(u32);
impl InputHandle {
    pub fn from_raw(value: u32) -> io::Result<Self> {
        if value == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "zero input handle",
            ));
        }
        Ok(Self(value))
    }
    pub fn raw(self) -> u32 {
        self.0
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutputHandle(u32);
impl OutputHandle {
    pub fn raw(self) -> u32 {
        self.0
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct VolumeDb {
    pub integer: i32,
    pub thousandths: i32,
}
#[repr(C)]
struct VolumeDbQuery {
    output: u32,
    path: u32,
    value: VolumeDb,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    pub output: OutputHandle,
    pub inputs: Vec<InputHandle>,
}
#[derive(Clone, Copy, Debug, Default)]
pub enum RoutePolicy {
    /// Borrow an existing identical route, or attach only if graph inputs are empty.
    #[default]
    RequireEmpty,
    /// Explicitly permit adding a route; all original graph inputs must survive.
    PreserveExisting,
}
#[repr(C)]
struct Connections {
    output: u32,
    is_input: u32,
    module: u32,
    count: u32,
    array: u64,
}
#[repr(C)]
struct Connect {
    output: u32,
    input: u32,
    reserved: u8,
    pad: [u8; 3],
}
#[repr(C)]
struct BoolQuery {
    output: u32,
    selector: u32,
    value: u8,
    pad: [u8; 3],
}
#[repr(C)]
struct AttrQuery {
    output: u32,
    attr: u32,
    input: u64,
    result: u64,
    extra: [u8; 48],
}
const _: () = {
    assert!(std::mem::size_of::<Connections>() == 24);
    assert!(std::mem::offset_of!(Connections, array) == 16);
    assert!(std::mem::size_of::<Connect>() == 12);
    assert!(std::mem::size_of::<BoolQuery>() == 12);
    assert!(std::mem::size_of::<AttrQuery>() == 72);
    assert!(std::mem::offset_of!(AttrQuery, result) == 16);
};
fn call<T>(transport: &impl Transport, request: u32, arg: &mut T) -> io::Result<()> {
    // These wire structures have explicit padding and every byte is initialized.
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(arg as *mut T as *mut u8, std::mem::size_of::<T>())
    };
    transport.transact(request, bytes)
}
#[derive(Debug)]
pub struct RestoreError {
    pub failures: Vec<(&'static str, io::Error)>,
}
impl fmt::Display for RestoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AOUT restore incomplete")?;
        for (stage, error) in &self.failures {
            write!(f, "; {stage}: {error}")?;
        }
        Ok(())
    }
}
impl std::error::Error for RestoreError {}
#[derive(Debug)]
pub struct AttachError {
    pub original: Snapshot,
    pub cause: io::Error,
    pub rollback: Option<io::Error>,
}
impl fmt::Display for AttachError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AOUT attach failed: {}", self.cause)?;
        if let Some(error) = &self.rollback {
            write!(f, "; rollback failed: {error}")?;
        }
        Ok(())
    }
}
impl std::error::Error for AttachError {}

pub struct Aout<T: Transport = DeviceTransport> {
    transport: T,
    output: OutputHandle,
}
impl Aout<DeviceTransport> {
    /// Opens a device descriptor, then queries the already-existing speaker.
    /// This is not MI_AOUT_Open: ownership of the output stays with its creator.
    pub fn existing_speaker() -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mik/aout")
            .or_else(|_| {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/mik!aout")
            })?;
        Self::existing_speaker_with(DeviceTransport::from_file(file))
    }
}
impl<T: Transport> Aout<T> {
    pub fn shared(self) -> Aout<std::sync::Arc<T>> {
        Aout {
            transport: std::sync::Arc::new(self.transport),
            output: self.output,
        }
    }
    pub fn duplicate(&self) -> Self
    where
        T: Clone,
    {
        Self {
            transport: self.transport.clone(),
            output: self.output,
        }
    }
    pub fn existing_speaker_with(transport: T) -> io::Result<Self> {
        let mut wire = [4u32, 0];
        call(&transport, GET_HANDLE, &mut wire)?;
        if wire[1] >> 24 != 0x17 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid existing AOUT handle",
            ));
        }
        Ok(Self {
            transport,
            output: OutputHandle(wire[1]),
        })
    }
    pub fn output(&self) -> OutputHandle {
        self.output
    }
    /// Aggregate MI mute state; stock getter ignores the caller's mute-type field.
    pub fn muted(&self) -> io::Result<bool> {
        let mut wire = BoolQuery {
            output: self.output.0,
            selector: 0,
            value: 0xff,
            pad: [0; 3],
        };
        call(&self.transport, 0xc00c110e, &mut wire)?;
        Self::boolean(wire.value)
    }
    /// Raw stock 8-bit speaker volume index; no conversion to percent or dB.
    pub fn volume_index(&self) -> io::Result<u8> {
        let mut wire = BoolQuery {
            output: self.output.0,
            selector: 4,
            value: 0,
            pad: [0; 3],
        };
        call(&self.transport, 0xc00c1108, &mut wire)?;
        Ok(wire.value)
    }
    pub fn volume_db(&self) -> io::Result<VolumeDb> {
        let mut wire = VolumeDbQuery {
            output: self.output.0,
            path: 4,
            value: VolumeDb::default(),
        };
        call(&self.transport, 0xc010110a, &mut wire)?;
        Ok(wire.value)
    }
    pub fn amplifier_muted(&self) -> io::Result<bool> {
        self.bool_attr(0)
    }
    pub fn output_enabled(&self) -> io::Result<bool> {
        self.bool_attr(6)
    }
    fn boolean(value: u8) -> io::Result<bool> {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid vendor boolean",
            )),
        }
    }
    fn bool_attr(&self, attr: u32) -> io::Result<bool> {
        // Exact GetAttrParamSize table: attributes0 and6 have zero input and
        // one-byte output. V109 GetAttr wire is72 bytes, not PCM's24 bytes.
        let mut value = 0xffu8;
        let mut wire = AttrQuery {
            output: self.output.0,
            attr,
            input: 0,
            result: &mut value as *mut u8 as usize as u64,
            extra: [0; 48],
        };
        call(&self.transport, 0xc0481115, &mut wire)?;
        Self::boolean(value)
    }
    fn count(&self) -> io::Result<u32> {
        let mut wire = [self.output.0, 1u32, 0u32, 0u32];
        call(&self.transport, GET_COUNT, &mut wire)?;
        if wire[3] > MAX_CONNECTIONS {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "AOUT connection count exceeds bound",
            ));
        }
        Ok(wire[3])
    }
    pub fn snapshot(&self) -> io::Result<Snapshot> {
        for _ in 0..3 {
            let count = self.count()?;
            let mut handles = vec![0u32; count as usize];
            if count > 0 {
                let mut wire = Connections {
                    output: self.output.0,
                    is_input: 1,
                    module: 0,
                    count,
                    array: handles.as_mut_ptr() as usize as u64,
                };
                call(&self.transport, GET_CONNECTED, &mut wire)?;
            }
            if self.count()? != count {
                continue;
            }
            let mut inputs = Vec::with_capacity(handles.len());
            for handle in handles {
                let handle = InputHandle::from_raw(handle).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "AOUT snapshot contains zero handle",
                    )
                })?;
                if inputs.contains(&handle) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "duplicate AOUT input",
                    ));
                }
                inputs.push(handle);
            }
            return Ok(Snapshot {
                output: self.output,
                inputs,
            });
        }
        Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "AOUT graph changed while taking snapshot",
        ))
    }
    fn connect(&self, input: InputHandle) -> io::Result<()> {
        call(
            &self.transport,
            CONNECT,
            &mut Connect {
                output: self.output.0,
                input: input.0,
                reserved: 0,
                pad: [0; 3],
            },
        )
    }
    fn disconnect(&self, input: InputHandle) -> io::Result<()> {
        call(&self.transport, DISCONNECT, &mut [self.output.0, input.0])
    }
    /// Owns the AOUT descriptor and restoration plan without borrowing caller state.
    /// Suitable for embedding alongside an independently owned PCM stream.
    pub fn route_owned(
        mut self,
        input: InputHandle,
        policy: RoutePolicy,
    ) -> io::Result<OwnedRoute<T>> {
        let (before, input, restore_needed) = {
            let mut route = self.route(input, policy)?;
            let plan = (route.before.clone(), route.input, route.restore_needed);
            route.restore_needed = false;
            plan
        };
        Ok(OwnedRoute {
            aout: self,
            before,
            input,
            restore_needed,
        })
    }
    /// Only the returned guard owns the added connection. Existing routes are borrowed.
    pub fn route(&mut self, input: InputHandle, policy: RoutePolicy) -> io::Result<Route<'_, T>> {
        let before = self.snapshot()?;
        let borrowed = before.inputs.contains(&input);
        if !borrowed && !before.inputs.is_empty() && matches!(policy, RoutePolicy::RequireEmpty) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "speaker already has inputs; explicit preservation policy required",
            ));
        }
        let mut route = Route {
            aout: self,
            before,
            input,
            restore_needed: !borrowed,
        };
        if !borrowed {
            let result = route.aout.connect(input).and_then(|_| {
                let after = route.aout.snapshot()?;
                if !after.inputs.contains(&input)
                    || !route.before.inputs.iter().all(|h| after.inputs.contains(h))
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "connect did not preserve requested graph",
                    ));
                }
                Ok(())
            });
            if let Err(cause) = result {
                let rollback = route.restore_once().err();
                return Err(io::Error::other(AttachError {
                    original: route.before.clone(),
                    cause,
                    rollback,
                }));
            }
        }
        Ok(route)
    }
}

pub struct Route<'a, T: Transport> {
    aout: &'a mut Aout<T>,
    before: Snapshot,
    input: InputHandle,
    restore_needed: bool,
}
impl<T: Transport> Route<'_, T> {
    pub fn before(&self) -> &Snapshot {
        &self.before
    }
    pub fn snapshot(&self) -> io::Result<Snapshot> {
        self.aout.snapshot()
    }
    pub fn restore(mut self) -> io::Result<()> {
        self.restore_once()
    }
    fn restore_once(&mut self) -> io::Result<()> {
        if !self.restore_needed {
            return Ok(());
        }
        // No blind retries in Drop after an explicit failure: caller must inspect.
        self.restore_needed = false;
        restore_graph(self.aout, &self.before, self.input)
    }
}
impl<T: Transport> Drop for Route<'_, T> {
    fn drop(&mut self) {
        if let Err(error) = self.restore_once() {
            eprintln!("AOUT route cleanup: {error}")
        }
    }
}

/// Owned restoration guard; its fields contain no references to itself or PCM.
pub struct OwnedRoute<T: Transport> {
    aout: Aout<T>,
    before: Snapshot,
    input: InputHandle,
    restore_needed: bool,
}
impl<T: Transport> OwnedRoute<T> {
    pub fn before(&self) -> &Snapshot {
        &self.before
    }
    pub fn snapshot(&self) -> io::Result<Snapshot> {
        self.aout.snapshot()
    }
    pub fn restore(mut self) -> io::Result<()> {
        self.restore_once()
    }
    fn restore_once(&mut self) -> io::Result<()> {
        if !self.restore_needed {
            return Ok(());
        }
        self.restore_needed = false;
        restore_graph(&self.aout, &self.before, self.input)
    }
}
impl<T: Transport> Drop for OwnedRoute<T> {
    fn drop(&mut self) {
        if let Err(error) = self.restore_once() {
            eprintln!("AOUT owned-route cleanup: {error}")
        }
    }
}
fn restore_graph<T: Transport>(
    aout: &Aout<T>,
    before: &Snapshot,
    input: InputHandle,
) -> io::Result<()> {
    let current = aout.snapshot()?;
    let mut failures = vec![];
    if current.inputs.contains(&input) {
        if let Err(error) = aout.disconnect(input) {
            failures.push(("disconnect owned input", error));
        }
    }
    match aout.snapshot() {
        Ok(current) => {
            for input in &before.inputs {
                if !current.inputs.contains(input) {
                    if let Err(error) = aout.connect(*input) {
                        failures.push(("restore original input", error));
                    }
                }
            }
        }
        Err(error) => failures.push(("read graph during restore", error)),
    }
    match aout.snapshot() {
        Ok(after) => {
            if after.inputs.contains(&input)
                || !before.inputs.iter().all(|h| after.inputs.contains(h))
            {
                failures.push((
                    "verify restore",
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "graph restoration did not verify",
                    ),
                ));
            }
        }
        Err(error) => failures.push(("verify restore snapshot", error)),
    }
    // Inputs added by another actor are never disconnected by this guard.
    if failures.is_empty() {
        Ok(())
    } else {
        Err(io::Error::other(RestoreError { failures }))
    }
}

#[cfg(test)]
#[path = "../../../test/unit/audio/aout-route-tests.rs"]
mod tests;

/// Scoped hardware attenuation. Caller must serialize all speaker gain changes.
/// It never changes mute state, routing, or the app's global software gain.
pub struct OwnedVolume<T: Transport> {
    aout: Aout<T>,
    original: VolumeDb,
    target: VolumeDb,
    restore_needed: bool,
}
#[derive(Debug)]
pub struct VolumeTakeoverError {
    pub original: VolumeDb,
    pub cause: io::Error,
    pub rollback: Option<io::Error>,
}
impl fmt::Display for VolumeTakeoverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "speaker volume takeover failed: {}", self.cause)?;
        if let Some(error) = &self.rollback {
            write!(f, "; restore {:?} failed: {error}", self.original)?;
        }
        Ok(())
    }
}
impl std::error::Error for VolumeTakeoverError {}
impl<T: Transport> Aout<T> {
    fn set_volume_db(&self, value: VolumeDb) -> io::Result<()> {
        let mut wire = VolumeDbQuery {
            output: self.output.0,
            path: 4,
            value,
        };
        call(&self.transport, 0x40101109, &mut wire)
    }
    /// Use only with an explicitly approved hardware target and bounded playback.
    pub fn volume_owned(self, target: VolumeDb) -> io::Result<OwnedVolume<T>> {
        let total = target.integer as i64 * 1000 + target.thousandths as i64;
        if !(-999..=999).contains(&target.thousandths) || !(-96000..=0).contains(&total) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "hardware attenuation must be -96..0 dB",
            ));
        }
        let original = self.volume_db()?;
        let mut lease = OwnedVolume {
            aout: self,
            original,
            target,
            restore_needed: original != target,
        };
        if lease.restore_needed {
            let result = lease
                .aout
                .set_volume_db(target)
                .and_then(|_| lease.verify(target));
            if let Err(cause) = result {
                let rollback = lease.restore_once().err();
                return Err(io::Error::other(VolumeTakeoverError {
                    original,
                    cause,
                    rollback,
                }));
            }
        }
        Ok(lease)
    }
}
impl<T: Transport> OwnedVolume<T> {
    pub fn original(&self) -> VolumeDb {
        self.original
    }
    pub fn target(&self) -> VolumeDb {
        self.target
    }
    pub fn current(&self) -> io::Result<VolumeDb> {
        self.aout.volume_db()
    }
    pub fn restore(mut self) -> io::Result<()> {
        self.restore_once()
    }
    fn verify(&self, expected: VolumeDb) -> io::Result<()> {
        let actual = self.aout.volume_db()?;
        if actual != expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("speaker gain readback {actual:?}, expected {expected:?}"),
            ));
        }
        Ok(())
    }
    fn restore_once(&mut self) -> io::Result<()> {
        if !self.restore_needed {
            return Ok(());
        }
        self.restore_needed = false;
        self.aout.set_volume_db(self.original)?;
        self.verify(self.original)
    }
}
impl<T: Transport> Drop for OwnedVolume<T> {
    fn drop(&mut self) {
        if let Err(error) = self.restore_once() {
            eprintln!("speaker gain restoration FAILED: {error}")
        }
    }
}
