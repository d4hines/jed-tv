/-!
# Doom.TV.Keys

Linux input keycodes (keyboard and the TCL remote) to Doom key values
(`doomkeys.h`) as bound by the engine (`Doom.Playsim.Input`). The remote's OK
is both Enter (menus, and "yes" to prompts) and fire (game); Back opens the
menu; Ch+ uses and zooms the automap in; Ch- runs and zooms out; YouTube
toggles the automap. The remote has no digits, so NETFLIX and prime video
are reserved for previous/next weapon once `Doom.Playsim.Input` supports
weapon cycling (the Rust/C build already does); until then they do nothing.
Keyboard: Page Up/Down zoom like '=' and '-'.
-/

namespace Doom.TV.Keys

def KEY_RIGHTARROW : UInt32 := 0xae
def KEY_LEFTARROW : UInt32 := 0xac
def KEY_UPARROW : UInt32 := 0xad
def KEY_DOWNARROW : UInt32 := 0xaf
-- The engine keeps Chocolate Doom's default bindings (`Doom.Playsim.Input`):
-- fire = right Ctrl, use = space, strafe left/right = ',' / '.'.
def KEY_STRAFE_L : UInt32 := 44
def KEY_STRAFE_R : UInt32 := 46
def KEY_USE : UInt32 := 32
def KEY_FIRE : UInt32 := 0x80 + 0x1d
def KEY_ESCAPE : UInt32 := 27
def KEY_ENTER : UInt32 := 13
def KEY_TAB : UInt32 := 9
def KEY_BACKSPACE : UInt32 := 0x7f
def KEY_PAUSE : UInt32 := 0xff
def KEY_RSHIFT : UInt32 := 0x80 + 0x36
def KEY_RALT : UInt32 := 0x80 + 0x38

private def ascii (s : String) (i : Nat) : List UInt32 :=
  match s.toList[i]? with
  | some c => [c.toNat.toUInt32]
  | none => []

/-- Doom keys for one Linux keycode (usually one; OK gives Enter and fire). -/
def doomKeys (code : UInt32) : List UInt32 :=
  let c := code.toNat
  if c == 1 then [KEY_ESCAPE]
  else if c >= 2 && c <= 11 then ascii "1234567890" (c - 2)
  else if c == 12 then ascii "-" 0
  else if c == 13 then ascii "=" 0
  else if c == 14 then [KEY_BACKSPACE]
  else if c == 15 then [KEY_TAB]
  else if c >= 16 && c <= 27 then ascii "qwertyuiop[]" (c - 16)
  else if c == 28 || c == 96 || c == 0x160 || c == 0x161 then [KEY_ENTER, KEY_FIRE]
  else if c == 29 || c == 97 || c == 0x18e then [KEY_FIRE]
  else if c >= 30 && c <= 41 then ascii "asdfghjkl;'`" (c - 30)
  else if c == 42 || c == 54 || c == 0x190 then [KEY_RSHIFT]
  else if c >= 43 && c <= 50 then ascii "\\zxcvbnm" (c - 43)
  else if c == 51 then [KEY_STRAFE_L]
  else if c == 52 then [KEY_STRAFE_R]
  else if c == 53 then ascii "/" 0
  else if c == 56 || c == 100 then [KEY_RALT]
  else if c == 57 || c == 164 || c == 0x18f then [KEY_USE]
  else if c >= 59 && c <= 68 then [(0xbb + (c - 59)).toUInt32]
  else if c == 87 then [0x80 + 0x57]
  else if c == 88 then [0x80 + 0x58]
  else if c == 103 then [KEY_UPARROW]
  else if c == 104 then ascii "=" 0  -- Page Up: automap zoom in
  else if c == 109 then ascii "-" 0  -- Page Down: zoom out
  else if c == 105 then [KEY_LEFTARROW]
  else if c == 106 then [KEY_RIGHTARROW]
  else if c == 108 then [KEY_DOWNARROW]
  else if c == 119 then [KEY_PAUSE]
  else if c == 158 || c == 174 || c == 139 then [KEY_ESCAPE]
  -- The TCL remote (codes from the firmware's IR table; see platform::remote).
  else if c == 402 then [KEY_USE] ++ ascii "=" 0     -- Ch+: use; automap zoom in
  else if c == 403 then [KEY_RSHIFT] ++ ascii "-" 0  -- Ch-: run; automap zoom out
  else if c == 569 || c == 0x191 then [KEY_TAB]      -- YouTube (and Blue on other handsets)
  else if c >= 0x200 && c <= 0x209 then ascii "0123456789" (c - 0x200)
  else []

end Doom.TV.Keys
