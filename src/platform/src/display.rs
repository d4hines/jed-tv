//! App-facing display API v1. Backend selection is outside application code.
//!
//! `Display::open_default()` -> `acquire()` -> draw through `pixels()` ->
//! `FrameLease::present()`. A dropped lease cancels without publishing.
//! Pixel layout is described by `var`; callers must not assume byte order.
pub use crate::fb::{BackBuffer as Pixels, FrameLease, Framebuffer as Display};
