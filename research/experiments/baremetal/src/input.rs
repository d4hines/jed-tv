//! Keys from virtio-input, in the same shape as `platform::input` (Linux
//! evdev keycodes), so the platform's `remote.rs` compiles here unchanged.

use crate::hal::HalImpl;
use alloc::string::String;
use alloc::vec::Vec;
use virtio_drivers::device::input::VirtIOInput;
use virtio_drivers::transport::mmio::MmioTransport;

const EV_KEY: u16 = 1;

/// A key press (value 1), repeat (2) or release (0), with the Linux keycode.
#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub code: u16,
    pub value: i32,
}

pub struct Device {
    pub path: String,
    pub name: String,
    dev: VirtIOInput<HalImpl, MmioTransport<'static>>,
}

impl Device {
    pub fn new(path: String, transport: MmioTransport<'static>) -> Result<Self, virtio_drivers::Error> {
        let mut dev = VirtIOInput::new(transport)?;
        let name = dev.name().map_or_else(|_| String::from("?"), |n| n.trim_end_matches('\0').into());
        Ok(Device { path, name, dev })
    }

    /// Drain pending key events without blocking.
    pub fn poll_keys(&mut self) -> Result<Vec<Key>, virtio_drivers::Error> {
        self.dev.ack_interrupt();
        let mut keys = Vec::new();
        while let Some(e) = self.dev.pop_pending_event() {
            if e.event_type == EV_KEY {
                keys.push(Key { code: e.code, value: e.value as i32 });
            }
        }
        Ok(keys)
    }
}
