//! G06/V109 cold-only allocation of the amplifier's GPIO30 and IIC port9.
//! No pin level, inversion, bus speed, register, or amplifier power is changed.
//! INIT can configure controller hardware; it requires the same exclusive boot
//! permit as all other MI initialization. Keep this session after errors; never
//! construct a new session to retry a partially completed bootstrap.
use std::{fs::File, io};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Domain {
    Gpio,
    Iic,
}
impl Domain {
    fn name(self) -> &'static str {
        match self {
            Self::Gpio => "gpio",
            Self::Iic => "iic",
        }
    }
    fn init(self) -> u32 {
        match self {
            Self::Gpio => 0xc0012900,
            Self::Iic => 0xc0011d00,
        }
    }
    fn open(self) -> u32 {
        self.init() + 0x70002
    }
    fn lookup(self) -> u32 {
        self.init() + 0x70004
    }
    fn valid(self, h: u32) -> bool {
        match self {
            Self::Gpio => h == 0x4b00001e,
            Self::Iic => h & 0xfffffff0 == 0x55000000,
        }
    }
}

/// Injected transport also makes the potentially mutating decision tree testable.
pub trait HandleIo {
    fn authorize(&mut self) -> io::Result<()>;
    fn call(&mut self, domain: Domain, request: u32, data: &mut [u8]) -> io::Result<i32>;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Endpoint {
    pub handle: u32,
    pub initialized: bool,
    pub created: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Handles {
    pub gpio: Endpoint,
    pub iic: Endpoint,
}

pub struct Session<T> {
    io: T,
    attempted: bool,
}
impl<T: HandleIo> Session<T> {
    pub fn new(io: T) -> Self {
        Self {
            io,
            attempted: false,
        }
    }
    /// One attempt, including authorization failure. No rollback: GPIO/IIC may
    /// be shared by later cold initialization; never close an adopted handle.
    pub fn prepare(&mut self) -> io::Result<Handles> {
        if self.attempted {
            return Err(error("amplifier handle bootstrap already attempted"));
        }
        self.attempted = true;
        self.io.authorize()?;
        Ok(Handles {
            gpio: self.endpoint(Domain::Gpio, 30)?,
            iic: self.endpoint(Domain::Iic, 9)?,
        })
    }
    fn endpoint(&mut self, domain: Domain, id: u32) -> io::Result<Endpoint> {
        // GPIO's ID is u16+zero padding; for GPIO30 these bytes also equal u32.
        let payload = || [0u32.to_le_bytes(), id.to_le_bytes()].concat();
        let mut data = payload();
        let mut code = self.io.call(domain, domain.lookup(), &mut data)?;
        let mut initialized = false;
        if code == 4 {
            // MI_RET_NOT_INIT, established by exact kernel branches.
            let init = self.io.call(domain, domain.init(), &mut [0])?;
            if init != 0 && init != 2 {
                return Err(status(domain, "INIT", init));
            }
            initialized = true;
            data = payload();
            code = self.io.call(domain, domain.lookup(), &mut data)?;
        }
        let mut created = false;
        if code == 9 {
            // Lookup found no active slot; unlike PCM this allocates nothing.
            data = payload();
            let opened = self.io.call(domain, domain.open(), &mut data)?;
            if opened != 0 {
                return Err(status(domain, "Open", opened));
            }
            created = true;
        } else if code != 0 {
            return Err(status(domain, "GetHandle", code));
        }
        let handle = u32::from_le_bytes(data[..4].try_into().unwrap());
        if !domain.valid(handle) {
            return Err(error("unexpected amplifier handle returned"));
        }
        if created {
            let mut check = payload();
            let verified = self.io.call(domain, domain.lookup(), &mut check)?;
            if verified != 0 {
                return Err(status(domain, "GetHandle readback", verified));
            }
            if check[..4] != data[..4] {
                return Err(error("amplifier handle readback mismatch"));
            }
        }
        Ok(Endpoint {
            handle,
            initialized,
            created,
        })
    }
}
fn error(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}
fn status(d: Domain, operation: &str, code: i32) -> io::Error {
    error(&format!(
        "{} {operation} returned vendor status {code}; bootstrap stopped",
        d.name()
    ))
}

/// The borrowed Device retains its unforgeable cold-boot permission. This adapter
/// is deliberately not exposed as a warm probe or standalone initialization CLI.
pub struct DeviceIo<'a> {
    device: &'a mut super::Device,
    gpio: Option<File>,
    iic: Option<File>,
}
impl<'a> DeviceIo<'a> {
    pub fn new(device: &'a mut super::Device) -> Self {
        Self {
            device,
            gpio: None,
            iic: None,
        }
    }
}
impl HandleIo for DeviceIo<'_> {
    fn authorize(&mut self) -> io::Result<()> {
        use super::Transport;
        self.device.ensure_exclusive_cold_boot()?;
        if self.device.status(1)? {
            return Err(error("SYS_USER already ready"));
        }
        for &selector in super::KERNEL_PREREQUISITES {
            if !self.device.status(selector)? {
                return Err(error("SYS/module prerequisite not ready"));
            }
        }
        Ok(())
    }
    fn call(&mut self, domain: Domain, request: u32, data: &mut [u8]) -> io::Result<i32> {
        // Enforce the permit even if a caller invokes this trait method directly.
        self.authorize()?;
        let slot = match domain {
            Domain::Gpio => &mut self.gpio,
            Domain::Iic => &mut self.iic,
        };
        if slot.is_none() {
            *slot = Some(super::open_node(domain.name(), true)?);
        }
        super::call(
            slot.as_ref().unwrap(),
            request,
            data.as_mut_ptr() as u64,
            data.len() as u16,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    struct Fake {
        allowed: bool,
        steps: VecDeque<(Domain, u32, i32, u32)>,
        calls: usize,
    }
    impl HandleIo for Fake {
        fn authorize(&mut self) -> io::Result<()> {
            if self.allowed {
                Ok(())
            } else {
                Err(error("denied"))
            }
        }
        fn call(&mut self, d: Domain, r: u32, p: &mut [u8]) -> io::Result<i32> {
            self.calls += 1;
            let (ed, er, status, h) = self.steps.pop_front().expect("unexpected operation");
            assert_eq!((d, r), (ed, er));
            if r == d.init() {
                assert_eq!(p, &[0]);
            } else {
                assert_eq!(p.len(), 8);
                assert_eq!(&p[..4], &[0; 4]);
                assert_eq!(
                    u32::from_le_bytes(p[4..8].try_into().unwrap()),
                    if d == Domain::Gpio { 30 } else { 9 }
                );
                p[..4].copy_from_slice(&h.to_le_bytes());
            }
            Ok(status)
        }
    }
    fn s(d: Domain, op: u32, status: i32, h: u32) -> (Domain, u32, i32, u32) {
        (d, op, status, h)
    }
    fn fake(steps: Vec<(Domain, u32, i32, u32)>) -> Session<Fake> {
        Session::new(Fake {
            allowed: true,
            steps: steps.into(),
            calls: 0,
        })
    }
    #[test]
    fn exact_requests() {
        assert_eq!(Domain::Gpio.open(), 0xc0082902);
        assert_eq!(Domain::Iic.lookup(), 0xc0081d04);
    }
    #[test]
    fn authorization_precedes_io() {
        let mut x = fake(vec![]);
        x.io.allowed = false;
        assert!(x.prepare().is_err());
        assert_eq!(x.io.calls, 0);
        assert!(x.prepare().is_err());
    }
    #[test]
    fn existing_handles_are_adopted() {
        let mut x = fake(vec![
            s(Domain::Gpio, Domain::Gpio.lookup(), 0, 0x4b00001e),
            s(Domain::Iic, Domain::Iic.lookup(), 0, 0x55000003),
        ]);
        let h = x.prepare().unwrap();
        assert!(!h.gpio.created && !h.iic.initialized);
        assert_eq!(x.io.calls, 2);
        assert!(x.prepare().is_err());
    }
    #[test]
    fn cold_init_open_and_verify() {
        for init_status in [0, 2] {
            let mut steps = vec![];
            for (d, h) in [(Domain::Gpio, 0x4b00001e), (Domain::Iic, 0x55000000)] {
                steps.extend([
                    s(d, d.lookup(), 4, 0),
                    s(d, d.init(), init_status, 0),
                    s(d, d.lookup(), 9, 0),
                    s(d, d.open(), 0, h),
                    s(d, d.lookup(), 0, h),
                ]);
            }
            let mut x = fake(steps);
            let h = x.prepare().unwrap();
            assert!(h.gpio.created && h.iic.initialized);
            assert!(x.io.steps.is_empty());
        }
    }
    #[test]
    fn missing_slot_does_not_reinitialize() {
        let d = Domain::Gpio;
        let h = 0x4b00001e;
        let mut x = fake(vec![
            s(d, d.lookup(), 9, 0),
            s(d, d.open(), 0, h),
            s(d, d.lookup(), 0, h),
            s(Domain::Iic, Domain::Iic.lookup(), 0, 0x55000000),
        ]);
        assert!(!x.prepare().unwrap().gpio.initialized);
    }
    #[test]
    fn failures_stop_and_cannot_retry() {
        let d = Domain::Gpio;
        for steps in [
            vec![s(d, d.lookup(), 8, 0)],
            vec![s(d, d.lookup(), 4, 0), s(d, d.init(), 3, 0)],
            vec![s(d, d.lookup(), 9, 0), s(d, d.open(), 9, 0)],
            vec![s(d, d.lookup(), 0, 0xdeadbeef)],
            vec![
                s(d, d.lookup(), 9, 0),
                s(d, d.open(), 0, 0x4b00001e),
                s(d, d.lookup(), 0, 0x4b00001f),
            ],
        ] {
            let mut x = fake(steps);
            assert!(x.prepare().is_err());
            assert!(x.io.steps.is_empty());
            let n = x.io.calls;
            assert!(x.prepare().is_err());
            assert_eq!(x.io.calls, n);
        }
    }
}
