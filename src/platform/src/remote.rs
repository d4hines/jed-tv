//! Remote-control keys as actions.
//!
//! The TV's IR receiver is the kernel's built-in MStar driver
//! (CONFIG_MSTAR_IR_REFACTOR + RC_CORE, `/dev/input/event2`, "MStar Smart TV
//! IR Receiver"), so it needs no vendor module or Android service. It emits
//! Linux keycodes. Which code each button sends comes from two files in the
//! V109 firmware backup (TVBACKUP/TCL32Q3K-V109): the IR table
//! `/vendor/tvconfig/config/ir_config.ini` (`[KIR_ChinaMobile_Keymap]`, RCA
//! command byte -> `KEY_*` name, in tvconfig.img) and TCL's key layout
//! `/vendor/usr/keylayout/Vendor_3697_Product_0001.kl` (Linux keycode ->
//! Android key, in the vendor partition of super.img). tv-emu/docs/ir-codes.md
//! describes how they were read.
//!
//! The handset shipped with the 32Q3K (TCL G0401-000303 family) has: Power,
//! Mute, a D-pad with OK, Back, Home, Volume +/-, Channel +/-, and NETFLIX,
//! prime video and YouTube buttons. It has no number pad, no colour keys and
//! no transport keys: no play/pause, no fast-forward, no rewind. Its Input,
//! Settings and TCLtv+ buttons are not modelled yet because their IR codes
//! have not been confirmed on the device (GitHub issue #7).
//!
//! In QEMU the same actions come from a keyboard, or from the on-screen
//! remote (demo/remote.py), which sends these exact keycodes to PID 1's
//! virtual IR receiver (`input::open_all`). Apps should match on `Action`,
//! not on raw codes.
//!
//! PID 1 keeps Power, Home, Volume +/- and Mute for itself. An app gets the
//! other eleven buttons: the D-pad, OK, Back, Ch +/-, NETFLIX, prime video
//! and YouTube. Everything an app needs must be reachable from those; the
//! three app buttons work as its function keys, and each app's help or
//! pause screen says what they do.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    // Buttons on the TCL remote. The comment is the Linux keycode the IR
    // receiver emits for it.
    /// 116
    Power,
    /// 113
    Mute,
    /// 103
    Up,
    /// 108
    Down,
    /// 105
    Left,
    /// 106
    Right,
    /// 28 (KEY_ENTER; the key layout calls it DPAD_CENTER)
    Ok,
    /// 158
    Back,
    /// 102 (KEY_HOME, not KEY_HOMEPAGE)
    Home,
    /// 115
    VolumeUp,
    /// 114
    VolumeDown,
    /// 402
    ChannelUp,
    /// 403
    ChannelDown,
    /// 566 (TCL_NETFLIX)
    Netflix,
    /// 752 (TCL_PRIMEVIDEO)
    PrimeVideo,
    /// 569 (TCL_YOUTUBE)
    YouTube,

    // No button on this remote. A keyboard in QEMU (or another TCL handset
    // that uses the same IR table) can still send them, so apps may offer
    // extras on them, but nothing the TV needs may depend on them.
    /// 139
    Menu,
    /// 174 (TCL_EXIT)
    Exit,
    /// 195
    Info,
    /// 164
    PlayPause,
    /// 2..=11 (KEY_1..KEY_9, KEY_0)
    Digit(u8),
    /// 398
    Red,
    /// 399
    Green,
    /// 400
    Yellow,
    /// 401
    Blue,
}

impl Action {
    /// True for a button that is on the TCL remote.
    pub fn on_remote(self) -> bool {
        use Action::*;
        matches!(self, Power | Mute | Up | Down | Left | Right | Ok | Back | Home
            | VolumeUp | VolumeDown | ChannelUp | ChannelDown | Netflix | PrimeVideo | YouTube)
    }
}

/// Map a Linux keycode (EV_KEY code) to an action.
pub fn action(code: u16) -> Option<Action> {
    use Action::*;
    Some(match code {
        // The TCL remote (ir_config.ini -> Vendor_3697_Product_0001.kl).
        116 => Power,
        113 => Mute,
        103 => Up,
        108 => Down,
        105 => Left,
        106 => Right,
        28 => Ok,
        158 => Back,
        102 => Home,
        115 => VolumeUp,
        114 => VolumeDown,
        402 => ChannelUp,
        403 => ChannelDown,
        566 => Netflix,
        752 => PrimeVideo,
        569 => YouTube,
        // Keyboard equivalents in QEMU (virtio-keyboard).
        96 | 352 | 353 => Ok,      // KPENTER, KEY_OK, KEY_SELECT
        1 | 14 => Back,            // Esc, Backspace
        172 => Home,               // KEY_HOMEPAGE
        104 => ChannelUp,          // Page Up
        109 => ChannelDown,        // Page Down
        // Keys with no button on the TCL remote.
        139 => Menu,
        174 => Exit,
        195 | 358 => Info,
        164 | 57 => PlayPause,     // KEY_PLAYPAUSE; keyboard Space
        2..=10 => Digit((code - 1) as u8), // KEY_1..KEY_9
        11 => Digit(0),
        398 => Red,
        399 => Green,
        400 => Yellow,
        401 => Blue,
        _ => return None,
    })
}

/// Actions from pending key presses (value 1) and autorepeats (value 2) on all devices.
pub fn poll(devs: &mut [crate::input::Device]) -> Vec<(Action, bool)> {
    let mut out = Vec::new();
    for d in devs.iter_mut() {
        for k in d.poll_keys().unwrap_or_default() {
            if k.value != 0 {
                if let Some(a) = action(k.code) {
                    out.push((a, k.value == 2));
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every button on the TCL remote, with the keycode from the firmware's IR table.
    const TCL_REMOTE: [(u16, Action); 16] = [
        (116, Action::Power), (113, Action::Mute),
        (103, Action::Up), (108, Action::Down), (105, Action::Left), (106, Action::Right),
        (28, Action::Ok), (158, Action::Back), (102, Action::Home),
        (115, Action::VolumeUp), (114, Action::VolumeDown),
        (402, Action::ChannelUp), (403, Action::ChannelDown),
        (566, Action::Netflix), (752, Action::PrimeVideo), (569, Action::YouTube),
    ];

    #[test]
    fn tcl_remote_codes() {
        for (code, expected) in TCL_REMOTE {
            assert_eq!(action(code), Some(expected), "keycode {code}");
            assert!(expected.on_remote(), "{expected:?}");
        }
    }

    #[test]
    fn no_transport_keys() {
        // KEY_FASTFORWARD, KEY_REWIND, KEY_PLAY, KEY_PAUSE, KEY_STOP, KEY_NEXTSONG,
        // KEY_PREVIOUSSONG and the second set TCL's IR table also knows.
        for code in [159, 168, 207, 119, 201, 128, 163, 165, 575, 577, 578, 579] {
            assert_eq!(action(code), None, "keycode {code}");
        }
    }

    #[test]
    fn keyboard_equivalents() {
        assert_eq!(action(96), Some(Action::Ok));
        assert_eq!(action(1), Some(Action::Back));
        assert_eq!(action(172), Some(Action::Home));
        assert_eq!(action(104), Some(Action::ChannelUp));
        assert_eq!(action(109), Some(Action::ChannelDown));
        assert_eq!(action(2), Some(Action::Digit(1)));
        assert_eq!(action(11), Some(Action::Digit(0)));
        assert!(!Action::Digit(1).on_remote());
        assert!(!Action::PlayPause.on_remote());
        assert_eq!(action(9999), None);
    }
}
