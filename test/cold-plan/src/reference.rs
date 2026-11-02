//! Frozen f698010 pre-port Rust oracle; never linked into TV code.
use std::io;
#[allow(dead_code)]
enum Module {
    Display,
    DisplayOutput,
    Osd,
    Audio,
    AudioOutput,
    Pcm,
}
pub mod cold_start {
    // Offline plan validation for the first owned V109 bootstrap.
    // This module does not load a module, stage a file, or authorize hardware I/O.
    use super::Module;
    use std::{collections::BTreeMap, io};

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

    pub const FLAGS: &[&str] = &[
        "MI_SYS_INIT",
        "MI_UART_INIT",
        "MI_WDT_INIT",
        "MI_IIC_INIT",
        "MI_GPIO_INIT",
        "MI_FLASH_INIT",
        "MI_DISP_INIT",
        "MI_DISPOUT_INIT",
        "MI_EXTIN_INIT",
        "MI_TUNER_INIT",
        "MI_DMX_INIT",
        "MI_TSIO_INIT",
        "MI_VIDEO_INIT",
        "MI_VMON_INIT",
        "MI_AUDIO_INIT",
        "MI_AOUT_INIT",
        "MI_PCM_INIT",
        "MI_SYNC_INIT",
        "MI_CAP_INIT",
        "MI_VENC_INIT",
        "MI_OSD_INIT",
        "MI_IMGDEC_INIT",
        "MI_INJECT_INIT",
        "MI_CEC_INIT",
        "MI_SAR_INIT",
        "MI_PWM_INIT",
        "MI_SMC_INIT",
        "MI_DSC_INIT",
        "MI_KL_INIT",
        "MI_CIPHER_INIT",
        "MI_ADVCA_INIT",
        "MI_IR_INIT",
        "MI_PCMCIA_INIT",
        "MI_TSR_INIT",
        "MI_TSENSOR_INIT",
    ];
    pub const PARAMETERS: &[(&str, &str)] = &[
        ("MI_DISP_OUTPUT_TIMING", "6"),
        ("MI_DISP_TV_SYSTEM", "6"),
        ("MI_DISP_DE_FLICKER", "0"),
        ("MI_OSD_WAIT_SYNC", "0"),
        ("MI_OSD_AUTO_FLIP", "0"),
        ("MI_OSD_WINDOW_CANVAS_MODE", "0"),
        ("MI_OSD_SUPPORT_SURFACE_COUNT", "0"),
        ("MI_OSD_WAIT_IDLE", "0"),
    ];

    /// Evidence that the provided INI agrees with the explicit-init recipe. It is
    /// not evidence that these bytes were loaded by the kernel, that configuration
    /// dependencies are complete, or that this is a cold boot.
    #[derive(Debug)]
    pub struct ValidatedIni(());

    /// Require exactly SYS/FLASH auto-init, with every other known INIT disabled.
    /// In particular, reject the earlier DISP/OSD auto-init candidate. Call before
    /// staging/loading mik; later edits or another mounted INI invalidate it.
    pub fn validate_explicit_ini(text: &str) -> io::Result<ValidatedIni> {
        let fail = |message| io::Error::new(io::ErrorKind::InvalidData, message);
        let mut values = BTreeMap::new();
        for line in text.lines() {
            let line = line.split(['#', ';']).next().unwrap().trim();
            if line.is_empty() || line.starts_with('[') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let (key, value) = (key.trim(), value.trim());
            if values.insert(key, value).is_some() {
                return Err(fail(format!("duplicate INI key {key}")));
            }
        }
        for &key in FLAGS {
            let expected = if matches!(key, "MI_SYS_INIT" | "MI_FLASH_INIT") {
                "1"
            } else {
                "0"
            };
            if values.get(key).copied() != Some(expected) {
                return Err(fail(format!(
                    "explicit-init recipe requires {key}={expected}"
                )));
            }
        }
        for (&key, &value) in &values {
            if key.ends_with("_INIT") && !FLAGS.contains(&key) && value != "0" {
                return Err(fail(format!("unknown enabled INIT key {key}")));
            }
        }
        for &(key, expected) in PARAMETERS {
            if values.get(key).copied() != Some(expected) {
                return Err(fail(format!(
                    "exact V109 payload requires {key}={expected}"
                )));
            }
        }
        Ok(ValidatedIni(()))
    }
}
fn fail(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}
pub fn explicit_ini(stock: &str) -> io::Result<String> {
    let mut result = String::with_capacity(stock.len());
    let mut changed = 0;
    for line in stock.split_inclusive('\n') {
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == "MI_TUNER_INIT" {
                let value = value.split([';', '#']).next().unwrap().trim();
                if value != "1" {
                    return Err(fail("expected stock MI_TUNER_INIT=1"));
                }
                let equal = line.find('=').unwrap();
                let index = equal + 1 + line[equal + 1..].find('1').unwrap();
                result.push_str(&line[..index]);
                result.push('0');
                result.push_str(&line[index + 1..]);
                changed += 1;
                continue;
            }
        }
        result.push_str(line);
    }
    if changed != 1 {
        return Err(fail("expected one stock tuner INIT key"));
    }
    cold_start::validate_explicit_ini(&result)?;
    Ok(result)
}
