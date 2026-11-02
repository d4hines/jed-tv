//! Fixed V109 module plan and the owning executor's configuration policy.
//! This module does not load a module, stage a file, or authorize hardware I/O.
use crate::Module;
use std::io;

/// Link closure plus the three stock runtime-registration modules. Framebuffer
/// is deliberately separate: its readiness gate needs independent proof.
pub const PRE_DISPLAY_MODULES: &[&str] = &[
    "iniparser",
    "kdrv_platform",
    "kdrv_xc",
    "kdrv_alg",
    "libdmdbraslia",
    "utpa2k",
    "hdcp1x",
    "hdcp2x",
    "xcker",
    "mwgifker",
    "misck",
    "mik",
];
pub const FRAMEBUFFER_MODULE: &str = "mstar_fbdev_mi";
pub const DISPLAY_SEQUENCE: &[Module] = &[Module::Display, Module::DisplayOutput, Module::Osd];
pub const AUDIO_SEQUENCE: &[Module] = &[Module::Audio, Module::AudioOutput, Module::Pcm];

/// Pure configuration policy supplied by the owning PID1 executor.
/// Implementations exchange owned text only; filesystem and hardware effects
/// stay with ColdPlatform. Failure must abort the current staging attempt.
pub trait IniPolicy {
    fn prepare(&self, stock: &str) -> io::Result<String>;
    fn validate(&self, staged: &str) -> io::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn framebuffer_is_after_bootstrap_not_part_of_preload() {
        assert_eq!(PRE_DISPLAY_MODULES.last(), Some(&"mik"));
        assert!(!PRE_DISPLAY_MODULES.contains(&FRAMEBUFFER_MODULE));
    }
}
