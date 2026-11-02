//! Audio services for the owning PID1; apps use the shared protocol instead.
#[path = "mi-pcm.rs"]
pub mod mi_pcm;
#[path = "device-resources.rs"]
pub mod device_resources;
#[cfg(test)]
pub mod protocol;
pub use mi_pcm::{BufferInfo, DeviceTransport, Pcm, Transport, VendorError};

#[path = "aout-route.rs"]
pub mod aout_route;
