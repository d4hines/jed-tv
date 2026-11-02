//! The platform API: the only services the apps (YouTube player, Doom) use
//! from below: fbdev display, ALSA PCM audio, evdev input, and PID 1 duties.
//! QEMU validates these Linux APIs. The TV additionally needs MediaTek MI
//! initialization; bare framebuffer and speaker operation remain unverified.
//! See research/platform-api.md.

pub mod alsa;
pub mod broker;
pub mod fb;
pub mod display;
pub mod app_api;
pub mod input;
pub mod net;
mod native_audio;
pub mod remote;
pub mod seccomp;
pub mod sys;
