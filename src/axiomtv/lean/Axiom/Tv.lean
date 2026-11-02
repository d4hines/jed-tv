import Axiom.World
import Axiom.Vec

/-!
The TV platform layer, in place of Raylib: a 320x180 frame drawn by the
software renderer in ../gfx (Rust) and shown on the TV display, key events
from the remote (via PID 1) or a keyboard, and signed 16-bit mono sound.
The C side is ../native/lean_glue.c. Coordinates are frame pixels; the TV
shows the frame scaled to 1280x720.
-/

namespace Axiom

/-- Frame size in pixels. -/
def screenW : Int32 := 320
def screenH : Int32 := 180

@[extern "atv_open"] opaque atvOpen (remoteFd rate : UInt32) : IO UInt32
/-- `0x80000000 ||| value <<< 16 ||| keycode`, or 0 when no key event is pending.
Value 0 = release, 1 = press, 2 = repeat. -/
@[extern "atv_poll_key"] opaque atvPollKey : IO UInt32
@[extern "atv_now_ms"] opaque atvNowMs : IO UInt32
@[extern "atv_begin_frame"] opaque atvBeginFrame : IO Unit
@[extern "atv_rect"] opaque atvRect (x y w h : Int32) (rgba : UInt32) : IO Unit
@[extern "atv_gradient_v"] opaque atvGradientV (x y w h : Int32) (top bottom : UInt32) : IO Unit
@[extern "atv_gradient_h"] opaque atvGradientH (x y w h : Int32) (left right : UInt32) : IO Unit
@[extern "atv_circle"] opaque atvCircle (x y : Int32) (r : Float32) (rgba : UInt32) : IO Unit
@[extern "atv_line"] opaque atvLine (x0 y0 x1 y1 : Int32) (rgba : UInt32) : IO Unit
/-- 8x8 font at an integer scale. -/
@[extern "atv_text"] opaque atvText (x y : Int32) (scale rgba : UInt32) (s : @& String) : IO Unit
@[extern "atv_begin_3d"] opaque atvBegin3d (px py pz tx ty tz fovy : Float32) : IO Unit
@[extern "atv_section"] opaque atvSection (ox oy oz size : UInt32) (faces : @& ByteArray) : IO Unit
@[extern "atv_cube"] opaque atvCube (cx cy cz sx sy sz : Float32) (rgba : UInt32) : IO Unit
@[extern "atv_cube_wires"] opaque atvCubeWires (cx cy cz size : Float32) (rgba : UInt32) : IO Unit
@[extern "atv_plane"] opaque atvPlane (cx cy cz w d : Float32) (rgba : UInt32) : IO Unit
@[extern "atv_present"] opaque atvPresent : IO UInt32
@[extern "atv_audio_write"] opaque atvAudioWrite (pcm : @& ByteArray) : IO UInt32

/-- Colour as the renderer takes it: r g b a packed big to small. -/
def rgba (r g b a : UInt8) : UInt32 :=
  r.toUInt32 <<< 24 ||| g.toUInt32 <<< 16 ||| b.toUInt32 <<< 8 ||| a.toUInt32

def Rgba.pack (c : Rgba) : UInt32 := rgba c.r c.g c.b c.a

def textWidth (scale : UInt32) (s : String) : Int32 :=
  Int32.ofNat (s.length * 8 * scale.toNat)

def playSound (pcm : ByteArray) : IO Unit := do
  if pcm.size > 0 then
    let _ ← atvAudioWrite pcm

/-- Keys held down, from press and release events. The remote has no mouse, so
Left/Right turn and Ch+/Ch- look up and down; NETFLIX held rises and YouTube
held sinks while flying. A keyboard in the emulator uses the upstream WASD keys. -/
structure Held where
  forward : Bool := false
  backward : Bool := false
  turnLeft : Bool := false
  turnRight : Bool := false
  strafeLeft : Bool := false
  strafeRight : Bool := false
  lookUp : Bool := false
  lookDown : Bool := false
  jump : Bool := false
  descend : Bool := false
  sprint : Bool := false
deriving Inhabited

/-- Linux keycodes: the TCL remote (codes from the firmware's IR table, see
src/platform/src/remote.rs; PID 1 keeps Power, Home, Volume and Mute) and
keyboard equivalents. -/
def Held.update (held : Held) (code : UInt32) (down : Bool) : Held :=
  match code with
  | 103 | 17 => { held with forward := down }          -- Up, W
  | 108 | 31 => { held with backward := down }         -- Down, S
  | 105 => { held with turnLeft := down }              -- Left
  | 106 => { held with turnRight := down }             -- Right
  | 30 => { held with strafeLeft := down }             -- A
  | 32 => { held with strafeRight := down }            -- D
  | 402 | 104 | 16 => { held with lookUp := down }     -- Ch+, Page Up, Q
  | 403 | 109 | 18 => { held with lookDown := down }   -- Ch-, Page Down, E
  | 566 | 139 | 57 => { held with jump := down }       -- NETFLIX, Menu, Space (held: rise in flight)
  | 569 | 11 | 29 => { held with descend := down }     -- YouTube, 0, Left Ctrl (held: sink in flight)
  | 42 => { held with sprint := down }                 -- Left Shift
  | _ => held

/-- What a key press (not a repeat) means. -/
inductive Press where
  | ok          -- OK/Enter: mine, begin, resume
  | place       -- prime video, Play/Pause, X
  | back        -- Back, Esc, Backspace, Exit
  | jump        -- NETFLIX, Menu, Space; on the pause screen: respawn
  | nextBlock   -- YouTube: the next block of the palette
  | fly         -- F; prime video on the pause screen
  | debug       -- F3
  | digit (n : Nat)
  | other
deriving BEq, Inhabited

def pressOf (code : UInt32) : Press :=
  match code with
  | 28 | 96 | 352 | 353 => .ok
  | 752 | 164 | 45 => .place
  | 158 | 1 | 14 | 174 => .back
  | 566 | 139 | 57 => .jump
  | 569 => .nextBlock
  | 33 => .fly
  | 61 => .debug
  | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => .digit (code.toNat - 1)
  | 11 => .digit 0
  | _ => .other

/-- Drain pending key events: the new held state and the presses, in order. -/
partial def pollKeys (held : Held) (presses : Array Press := #[]) : IO (Held × Array Press) := do
  let ev ← atvPollKey
  if ev == 0 then return (held, presses)
  let code := ev &&& 0xffff
  let value := (ev >>> 16) &&& 0xff
  let held := held.update code (value != 0)
  let presses := if value == 1 then presses.push (pressOf code) else presses
  pollKeys held presses

end Axiom
