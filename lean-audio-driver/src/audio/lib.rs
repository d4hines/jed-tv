//! MI wire adapters and mechanisms for the Lean audio driver (src/lean/TV/Audio),
//! used by PID1 on the G06 and by the standalone direct-audio process. Apps use
//! the shared socket protocol instead; the G13 Android AAudio bridge
//! (bridge.rs, protocol.rs) is built separately against Android libraries.
#[path = "mi-pcm.rs"]
pub mod mi_pcm;
#[path = "aout-route.rs"]
pub mod aout_route;
#[cfg(feature = "lean")]
pub mod service;
pub use mi_pcm::{BufferInfo, DeviceTransport, Transport, VendorError};
