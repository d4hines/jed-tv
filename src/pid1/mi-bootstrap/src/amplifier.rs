//! Owned AD82111 board adapter. No dlopen, module INIT, implicit unmute or CLI writes.
//! Register data comes from the SHA-pinned offline extractor under tools/midaemon.
use std::{fs::File, io, time::Duration};

pub struct Plan {
    factory: Vec<[u8; 2]>,
    drc: Vec<[u8; 2]>,
    gain: Vec<[u8; 2]>,
}

impl Plan {
    /// Parse only the original HID185 route and exact table dimensions.
    /// The caller must verify the extractor's SHA-256 manifest when staging data.
    pub fn parse(data: &[u8]) -> io::Result<Self> {
        let bad = || {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "not an extracted HID185 AD82111 plan",
            )
        };
        if data.len() != 210 || data[..10] != [b'A', b'D', b'8', b'1', 1, 30, 0, 9, 0x60, 0] {
            return Err(bad());
        }
        if data[10..16] != [78, 0, 17, 0, 2, 0] {
            return Err(bad());
        }
        let pairs = |bytes: &[u8]| bytes.chunks_exact(2).map(|p| [p[0], p[1]]).collect();
        Ok(Self {
            factory: pairs(&data[16..172]),
            drc: pairs(&data[172..206]),
            gain: pairs(&data[206..210]),
        })
    }
}

pub trait Bus {
    fn set_pd(&mut self, high: bool) -> io::Result<()>;
    fn write(&mut self, register: u8, value: u8) -> io::Result<()>;
    fn wait(&mut self, duration: Duration);
}

pub struct Ad82111<B> {
    bus: B,
    attempted: bool,
    initialized: bool,
}
impl<B: Bus> Ad82111<B> {
    pub fn new(bus: B) -> Self {
        Self {
            bus,
            attempted: false,
            initialized: false,
        }
    }

    /// One reset/programming attempt. Audio must be quiescent and ownership exclusive.
    /// On error stop immediately; never proceed to automatic unmute.
    pub fn initialize_muted(&mut self, plan: &Plan) -> io::Result<()> {
        if self.attempted {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "amp initialization already attempted",
            ));
        }
        self.attempted = true;
        self.bus.set_pd(false)?;
        self.bus.wait(Duration::from_millis(6));
        self.bus.set_pd(true)?;
        self.bus.wait(Duration::from_millis(20));
        for (index, &[register, value]) in plan.factory.iter().enumerate() {
            self.bus.write(register, value)?;
            if index == 0 {
                self.bus.wait(Duration::from_millis(8));
            }
            if index == 1 {
                self.bus.wait(Duration::from_millis(20));
            }
        }
        for &[register, value] in plan.drc.iter().chain(&plan.gain) {
            self.bus.write(register, value)?;
        }
        self.mute()?;
        self.initialized = true;
        Ok(())
    }

    pub fn mute(&mut self) -> io::Result<()> {
        self.bus.write(0x02, 0x1e)?;
        self.bus.wait(Duration::from_millis(2));
        Ok(())
    }

    /// Caller must first ensure bounded quiet PCM gain. This method changes hardware mute.
    pub fn unmute(&mut self) -> io::Result<()> {
        if !self.initialized {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "amp initialization is incomplete",
            ));
        }
        self.bus.write(0x02, 0x10)?;
        self.bus.wait(Duration::from_millis(2));
        Ok(())
    }
}

#[derive(Debug)]
pub struct Snapshot {
    pub iic_handle: i32,
    pub pd_handle: i32,
}

fn request<T>(file: &File, number: u32, argument: &mut T) -> io::Result<()> {
    let result = super::call(
        file,
        number,
        argument as *mut T as u64,
        std::mem::size_of::<T>() as u16,
    )?;
    if result != 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("amplifier ioctl {number:#x}: vendor result {result}"),
        ));
    }
    Ok(())
}

fn lookup(iic: &File, gpio: &File) -> io::Result<Snapshot> {
    // Exact GetHandle implementations search existing tables; neither allocates.
    let mut port = [0u32, 9];
    request(iic, 0xc0081d04, &mut port)?;
    let mut pad = [0u32, 30];
    request(gpio, 0xc0082904, &mut pad)?;
    if port[0] == 0 || pad[0] == 0 {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "existing amplifier IIC/GPIO handles required",
        ));
    }
    Ok(Snapshot {
        iic_handle: port[0] as i32,
        pd_handle: pad[0] as i32,
    })
}

/// Read-only route lookup. Does not initialize GPIO/IIC or open logical handles.
pub fn snapshot() -> io::Result<Snapshot> {
    let _identity = super::Device::open()?;
    lookup(
        &super::open_node("iic", false)?,
        &super::open_node("gpio", false)?,
    )
}

#[derive(Debug)]
pub struct ControlStatus {
    pub handles: Snapshot,
    pub gpio_mi_level: u32,
    pub gpio_inverted: bool,
    pub gpio_physical_high: bool,
    pub control_register_02: u8,
}

/// One GPIO level query and one single-byte IIC read, with no reset, register
/// data write, INIT, speed change, or retry. Selecting register 0x02 is the
/// address phase of the IIC read, not a write of a new register value.
pub fn read_control_status() -> io::Result<ControlStatus> {
    let _identity = super::Device::open()?;
    let iic = super::open_node("iic", false)?;
    let gpio = super::open_node("gpio", false)?;
    let handles = lookup(&iic, &gpio)?;
    let mut level = [handles.pd_handle as u32, 0, 0];
    request(&gpio, 0xc00c2908, &mut level)?;
    if level[1] > 1 || level[2] > 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid GPIO level/inversion reply",
        ));
    }
    let register = 0x02u8;
    let mut value = 0u8;
    let mut read = control_read(handles.iic_handle, &register, &mut value);
    request(&iic, 0xc0281d05, &mut read)?;
    Ok(ControlStatus {
        handles,
        gpio_mi_level: level[1],
        gpio_inverted: level[2] != 0,
        gpio_physical_high: (level[1] == 0) ^ (level[2] != 0),
        control_register_02: value,
    })
}

pub struct MiBus {
    iic: File,
    gpio: File,
    handles: Snapshot,
    invert_configured: bool,
}

fn mi_gpio_level(high: bool) -> u32 {
    u32::from(!high)
}
impl MiBus {
    /// # Safety
    /// The caller must own the AD82111 and pad30 exclusively in PID 1, have
    /// stopped/drained audio, and have verified the HID185 configuration. No
    /// vendor amplifier/reset client may run concurrently. This adopts existing
    /// GPIO/IIC handles only; it does not infer safety from status bits.
    pub unsafe fn adopt_initialized_quiescent() -> io::Result<Self> {
        if super::getpid() != 1 || super::geteuid() != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "amp writes require owning root PID 1",
            ));
        }
        let _identity = super::Device::open()?;
        let iic = super::open_node("iic", true)?;
        let gpio = super::open_node("gpio", true)?;
        let handles = lookup(&iic, &gpio)?;
        Ok(Self {
            iic,
            gpio,
            handles,
            invert_configured: false,
        })
    }
}

#[repr(C)]
struct IicWrite {
    handle: i32,
    bus_slave: u16,
    address_count: u8,
    pad: u8,
    register_ptr: u64,
    count: u16,
    reserved: [u8; 6],
    data_ptr: u64,
}

#[repr(C)]
struct IicRead {
    transfer: IicWrite,
    mode: u32,
    padding: u32,
}

fn control_read(handle: i32, register: &u8, value: &mut u8) -> IicRead {
    IicRead {
        transfer: IicWrite {
            handle,
            bus_slave: 0x60,
            address_count: 1,
            pad: 0,
            register_ptr: register as *const u8 as u64,
            count: 1,
            reserved: [0; 6],
            data_ptr: value as *mut u8 as u64,
        },
        mode: 0,
        padding: 0,
    }
}

impl Bus for MiBus {
    fn set_pd(&mut self, high: bool) -> io::Result<()> {
        if !self.invert_configured {
            let mut invert = [self.handles.pd_handle as u32, 0];
            request(&self.gpio, 0xc0082905, &mut invert)?;
            self.invert_configured = true;
        }
        // MI_GPIO_LEVEL_ON=0 and OFF=1; FPP converts physical high to 0.
        let mut output = [self.handles.pd_handle as u32, mi_gpio_level(high)];
        request(&self.gpio, 0xc0082907, &mut output)
    }
    fn write(&mut self, register: u8, value: u8) -> io::Result<()> {
        let mut argument = IicWrite {
            handle: self.handles.iic_handle,
            bus_slave: 0x60,
            address_count: 1,
            pad: 0,
            register_ptr: &register as *const u8 as u64,
            count: 1,
            reserved: [0; 6],
            data_ptr: &value as *const u8 as u64,
        };
        assert_eq!(std::mem::size_of::<IicWrite>(), 32);
        request(&self.iic, 0xc0201d06, &mut argument)
    }
    fn wait(&mut self, duration: Duration) {
        std::thread::sleep(duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, PartialEq)]
    enum Event {
        Pd(bool),
        Write(u8, u8),
        Wait(u64),
    }
    #[derive(Default)]
    struct Fake {
        events: Vec<Event>,
        fail: bool,
    }
    impl Bus for Fake {
        fn set_pd(&mut self, high: bool) -> io::Result<()> {
            self.events.push(Event::Pd(high));
            Ok(())
        }
        fn write(&mut self, r: u8, v: u8) -> io::Result<()> {
            self.events.push(Event::Write(r, v));
            if self.fail {
                Err(io::Error::new(io::ErrorKind::Other, "IIC failed"))
            } else {
                Ok(())
            }
        }
        fn wait(&mut self, d: Duration) {
            self.events.push(Event::Wait(d.as_millis() as u64));
        }
    }
    fn small_plan() -> Plan {
        Plan {
            factory: vec![[2, 0], [2, 30]],
            drc: vec![[3, 0]],
            gain: vec![[4, 23]],
        }
    }
    #[test]
    fn programming_preserves_reset_delays_and_ends_muted() {
        let mut amp = Ad82111::new(Fake::default());
        amp.initialize_muted(&small_plan()).unwrap();
        assert_eq!(
            amp.bus.events,
            vec![
                Event::Pd(false),
                Event::Wait(6),
                Event::Pd(true),
                Event::Wait(20),
                Event::Write(2, 0),
                Event::Wait(8),
                Event::Write(2, 30),
                Event::Wait(20),
                Event::Write(3, 0),
                Event::Write(4, 23),
                Event::Write(2, 30),
                Event::Wait(2)
            ]
        );
    }
    #[test]
    fn failed_programming_cannot_unmute_or_retry() {
        let mut amp = Ad82111::new(Fake {
            fail: true,
            ..Fake::default()
        });
        assert!(amp.initialize_muted(&small_plan()).is_err());
        assert!(amp.unmute().is_err());
        assert!(amp.initialize_muted(&small_plan()).is_err());
        assert!(!amp.bus.events.contains(&Event::Write(2, 16)));
    }
    #[test]
    fn unrelated_board_plan_is_rejected() {
        let mut data = vec![0; 210];
        data[..10].copy_from_slice(&[b'A', b'D', b'8', b'1', 1, 26, 1, 9, 0x60, 0]);
        assert!(Plan::parse(&data).is_err());
    }
    #[test]
    fn physical_gpio_levels_match_vendor_on_off_encoding() {
        assert_eq!(mi_gpio_level(false), 1); // MI_GPIO_LEVEL_OFF -> physical low, invert=0.
        assert_eq!(mi_gpio_level(true), 0); // MI_GPIO_LEVEL_ON -> physical high, invert=0.
    }
    #[test]
    fn control_read_is_one_byte_default_mode_with_exact_wire_size() {
        let register = 2;
        let mut value = 0;
        let read = control_read(0x55000001, &register, &mut value);
        assert_eq!(std::mem::size_of::<IicRead>(), 40);
        assert_eq!(read.transfer.bus_slave, 0x60);
        assert_eq!(read.transfer.address_count, 1);
        assert_eq!(read.transfer.count, 1);
        assert_eq!(read.mode, 0);
        assert_eq!(read.transfer.register_ptr, &register as *const u8 as u64);
        assert_eq!(read.transfer.data_ptr, &mut value as *mut u8 as u64);
    }
}
