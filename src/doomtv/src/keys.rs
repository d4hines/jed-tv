//! Linux evdev keycodes (input-event-codes.h) -> Doom keys (doomkeys.h).
//!
//! doomgeneric's TranslateKey() passes DG_GetKey's value straight through, so
//! these are the engine's own key values. Default bindings (m_controls.c):
//! fire = KEY_FIRE, use = KEY_USE, run = KEY_RSHIFT, strafe = KEY_RALT,
//! strafe left/right = KEY_STRAFE_L/R, menus = arrows/Enter/Escape/Y/N.
//! `Doom::start` rebinds three engine variables so the remote can do what
//! the keyboard does: previous/next weapon = Page Up/Down, menu confirm = Enter.
//!
//! The TCL remote (platform::remote; PID 1 keeps Power, Home, Volume and Mute):
//! arrows move and turn, OK fires and selects (and answers "yes" to prompts),
//! Back is Escape, Ch+ uses (and zooms the automap in), Ch- runs (and zooms
//! out), NETFLIX / prime video pick the previous / next weapon, YouTube
//! toggles the automap.

pub const KEY_RIGHTARROW: u8 = 0xae;
pub const KEY_LEFTARROW: u8 = 0xac;
pub const KEY_UPARROW: u8 = 0xad;
pub const KEY_DOWNARROW: u8 = 0xaf;
pub const KEY_STRAFE_L: u8 = 0xa0;
pub const KEY_STRAFE_R: u8 = 0xa1;
pub const KEY_USE: u8 = 0xa2;
pub const KEY_FIRE: u8 = 0xa3;
pub const KEY_ESCAPE: u8 = 27;
pub const KEY_ENTER: u8 = 13;
pub const KEY_TAB: u8 = 9;
pub const KEY_BACKSPACE: u8 = 0x7f;
pub const KEY_PAUSE: u8 = 0xff;
pub const KEY_RSHIFT: u8 = 0x80 + 0x36;
pub const KEY_RALT: u8 = 0x80 + 0x38;
pub const KEY_CAPSLOCK: u8 = 0x80 + 0x3a;
pub const KEY_HOME: u8 = 0x80 + 0x47;
pub const KEY_END: u8 = 0x80 + 0x4f;
pub const KEY_PGUP: u8 = 0x80 + 0x49;
pub const KEY_PGDN: u8 = 0x80 + 0x51;
pub const KEY_INS: u8 = 0x80 + 0x52;
pub const KEY_DEL: u8 = 0x80 + 0x53;

/// Linux keycodes the apps care about outside Doom too (launcher, quit screen).
pub mod ev {
    pub const ESC: u16 = 1;
    pub const ENTER: u16 = 28;
    pub const KPENTER: u16 = 96;
    pub const BACK: u16 = 158;
    pub const HOMEPAGE: u16 = 172;
    pub const EXIT: u16 = 174;
    pub const OK: u16 = 0x160;
    pub const SELECT: u16 = 0x161;
}

/// Doom keys for one evdev keycode (most map to one; a remote's OK maps to two).
pub fn doom_keys(code: u16) -> &'static [u8] {
    const DIGITS: &[u8] = b"1234567890";
    // US QWERTY rows as Linux numbers them: KEY_Q=16..KEY_RIGHTBRACE=27,
    // KEY_A=30..KEY_GRAVE=41, KEY_BACKSLASH=43..KEY_SLASH=53.
    const ROW_Q: &[u8] = b"qwertyuiop[]";
    const ROW_A: &[u8] = b"asdfghjkl;'`";
    const ROW_Z: &[u8] = b"\\zxcvbnm";
    match code {
        1 => &[KEY_ESCAPE],
        2..=11 => std::slice::from_ref(&DIGITS[(code - 2) as usize]),
        12 => b"-",
        13 => b"=",
        14 => &[KEY_BACKSPACE],
        15 => &[KEY_TAB],
        16..=27 => std::slice::from_ref(&ROW_Q[(code - 16) as usize]),
        28 | 96 => &[KEY_ENTER, KEY_FIRE],          // TCL OK=28; Enter in menus, fire in game
        29 | 97 => &[KEY_FIRE],                     // left/right Ctrl
        30..=41 => std::slice::from_ref(&ROW_A[(code - 30) as usize]),
        42 | 54 => &[KEY_RSHIFT],                   // Shift: run
        43..=50 => std::slice::from_ref(&ROW_Z[(code - 43) as usize]),
        51 => &[KEY_STRAFE_L],                      // ,
        52 => &[KEY_STRAFE_R],                      // .
        53 => b"/",
        56 | 100 => &[KEY_RALT],                    // Alt: strafe
        57 => &[KEY_USE],                           // Space
        58 => &[KEY_CAPSLOCK],
        59..=68 => std::slice::from_ref(&F1_F10[(code - 59) as usize]),
        87 => &[0x80 + 0x57],                       // F11
        88 => &[0x80 + 0x58],                       // F12
        102 => &[KEY_HOME],
        103 => &[KEY_UPARROW],
        104 => &[KEY_PGUP],
        105 => &[KEY_LEFTARROW],
        106 => &[KEY_RIGHTARROW],
        107 => &[KEY_END],
        108 => &[KEY_DOWNARROW],
        109 => &[KEY_PGDN],
        110 => &[KEY_INS],
        111 => &[KEY_DEL],
        119 => &[KEY_PAUSE],
        // The TCL remote (codes from the firmware's IR table, see
        // platform::remote). Physical repeat/release behavior is unmeasured.
        // Standard KEY_OK/SELECT aliases are kept for other input devices.
        ev::OK | ev::SELECT => &[KEY_ENTER, KEY_FIRE],
        ev::BACK | ev::EXIT | 139 => &[KEY_ESCAPE], // 139 = KEY_MENU
        402 => &[KEY_USE, b'='],                    // Ch+: use; automap zoom in
        403 => &[KEY_RSHIFT, b'-'],                 // Ch-: run; automap zoom out
        566 => &[KEY_PGUP],                         // NETFLIX: previous weapon (Doom::start)
        752 => &[KEY_PGDN],                         // prime video: next weapon
        569 => &[KEY_TAB],                          // YouTube: automap
        // Keys other TCL handsets send (same IR table); not on this remote.
        164 => &[KEY_USE],                          // KEY_PLAYPAUSE
        0x18e => &[KEY_FIRE],                       // KEY_RED
        0x18f => &[KEY_USE],                        // KEY_GREEN
        0x190 => &[KEY_RSHIFT],                     // KEY_YELLOW
        0x191 => &[KEY_TAB],                        // KEY_BLUE: automap
        0x200..=0x209 => std::slice::from_ref(&NUMERIC[(code - 0x200) as usize]), // KEY_NUMERIC_0..9
        _ => &[],
    }
}

const F1_F10: [u8; 10] = [0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4];
const NUMERIC: [u8; 10] = *b"0123456789";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tcl_remote_gameplay_controls() {
        // Vendor_3697_Product_0001.kl uses 28, not KEY_OK=352, for DPAD_CENTER.
        assert_eq!(doom_keys(28), &[KEY_ENTER, KEY_FIRE]);
        assert_eq!(doom_keys(103), &[KEY_UPARROW]);
        assert_eq!(doom_keys(108), &[KEY_DOWNARROW]);
        assert_eq!(doom_keys(105), &[KEY_LEFTARROW]);
        assert_eq!(doom_keys(106), &[KEY_RIGHTARROW]);
        assert_eq!(doom_keys(158), &[KEY_ESCAPE]);
        // Every button PID 1 forwards does something in the game.
        assert!(doom_keys(402).contains(&KEY_USE));     // Ch+
        assert!(doom_keys(403).contains(&KEY_RSHIFT));  // Ch-
        assert_eq!(doom_keys(566), &[KEY_PGUP]);        // NETFLIX
        assert_eq!(doom_keys(752), &[KEY_PGDN]);        // prime video
        assert_eq!(doom_keys(569), &[KEY_TAB]);         // YouTube
        // Automap zoom rides on the Ch rocker.
        assert!(doom_keys(402).contains(&b'='));
        assert!(doom_keys(403).contains(&b'-'));
        for alias in [96, ev::OK, ev::SELECT] {
            assert_eq!(doom_keys(alias), doom_keys(28));
        }
    }

    #[test]
    fn other_tcl_handsets() {
        assert_eq!(doom_keys(164), &[KEY_USE]);
        assert_eq!(doom_keys(399), &[KEY_USE]);
        assert_eq!(doom_keys(400), &[KEY_RSHIFT]);
        assert_eq!(doom_keys(401), &[KEY_TAB]);
    }

    #[test]
    fn keyboard() {
        assert_eq!(doom_keys(1), &[KEY_ESCAPE]);
        assert_eq!(doom_keys(2), b"1");
        assert_eq!(doom_keys(11), b"0");
        assert_eq!(doom_keys(16), b"q");
        assert_eq!(doom_keys(21), b"y");
        assert_eq!(doom_keys(49), b"n");
        assert_eq!(doom_keys(44), b"z");
        assert_eq!(doom_keys(41), b"`");
        assert_eq!(doom_keys(43), b"\\");
        assert_eq!(doom_keys(28), &[KEY_ENTER, KEY_FIRE]);
        assert_eq!(doom_keys(29), &[KEY_FIRE]);
        assert_eq!(doom_keys(57), &[KEY_USE]);
        assert_eq!(doom_keys(59), &[0x80 + 0x3b]);
        assert_eq!(doom_keys(68), &[0x80 + 0x44]);
        assert_eq!(doom_keys(103), &[KEY_UPARROW]);
        assert_eq!(doom_keys(108), &[KEY_DOWNARROW]);
        assert!(doom_keys(0).is_empty());
    }
}
