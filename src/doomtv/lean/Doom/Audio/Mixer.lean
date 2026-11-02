import Doom.Audio.Sfx

/-!
# Doom.Audio.Mixer

Sound-effect mixer modelled on Chocolate Doom's SDL backend
(`i_sdlsound.c` + SDL_mixer):

* `N` channels (vanilla `snd_channels = 8`); a channel plays one sfx once.
* Per-channel gains follow `I_SDL_UpdateSoundParams`:
  `left = (254 - sep) * vol / 127`, `right = sep * vol / 127`, clamped to
  0..255 (`vol` 0..127, `sep` 0..255, 128 = centre); SDL_mixer's panning
  effect scales samples by `gain / 255`.
* Samples are expanded from unsigned 8-bit as `d * 257 - 32768`
  (Chocolate's `data[src] | (data[src] << 8)` minus 32768).
* Rate conversion is a 16.16 fixed-point step (nearest sample) followed by
  Chocolate's one-pole low-pass filter at half the sfx rate
  (`alpha = dt / (rc + dt)`, `rc = 1 / (3.14 * sfx_rate)`), which is what
  Chocolate does for non-power-of-two ratios such as 11025 -> 48000. The
  filter is applied whenever the sfx rate is below the output rate.
* Optional pitch shifting (`snd_pitchshift`) scales the step by
  `128 / (256 - pitch)`, matching the duration change of Chocolate's
  `PitchShift`.
* Mixing order is SDL_mixer's: channels are added in index order into a
  silent buffer with signed 16-bit saturation after every addition, then the
  music stream is added the same way.

Output is interleaved signed 16-bit little-endian stereo. The inner loop
carries all state in scalar parameters and writes the uniquely referenced
output buffer in place.
-/

namespace Doom.Audio.Mixer

open Doom.Audio.Sfx

structure Chan where
  data : ByteArray := .empty
  offset : UInt32 := 0
  length : UInt32 := 0
  pos : UInt32 := 0
  frac : UInt32 := 0
  stepInt : UInt32 := 0
  stepFrac : UInt32 := 0
  gainL : Int32 := 0
  gainR : Int32 := 0
  /-- Low-pass coefficient, Q15 (32768 = pass-through). -/
  alpha : Int32 := 0
  lp : Int32 := 0
  active : Bool := false
  /-- SDL_mixer panning values (0..255), for inspection. -/
  left : UInt32 := 0
  right : UInt32 := 0
deriving Inhabited

structure Mixer where
  rate : Nat
  chans : Array Chan
  pitchShift : Bool := false
deriving Inhabited

@[inline] def i32 (u : UInt32) : Int32 := u.toInt32
@[inline] def sext16 (x : UInt32) : Int32 := (((x &&& 0xffff) ^^^ 0x8000) - 0x8000).toInt32
@[inline] def clip16 (x : Int32) : Int32 :=
  if x > i32 32767 then i32 32767 else if x < i32 0xffff8000 then i32 0xffff8000 else x

/-- A mixer with `numChannels` idle channels at output rate `rate`. -/
def Mixer.init (numChannels : Nat := 8) (rate : Nat := 48000) (pitchShift : Bool := false) : Mixer :=
  { rate, chans := Array.replicate numChannels {}, pitchShift }

/-- `I_SDL_UpdateSoundParams` panning: `(left, right)` in 0..255. -/
def panning (vol sep : Int) : Nat × Nat :=
  let clamp (x : Int) : Nat := if x < 0 then 0 else if x > 255 then 255 else x.toNat
  (clamp (((254 - sep) * vol).tdiv 127), clamp ((sep * vol).tdiv 127))

/-- Q16 gain for an SDL_mixer panning value (`g / 255`). -/
def gainQ16 (g : Nat) : Int32 := (g * 65536 / 255).toUInt32.toInt32

/-- Q15 low-pass coefficient for upsampling `sfxRate` to `outRate`. -/
def lowPassAlpha (sfxRate outRate : Nat) : Int32 :=
  if sfxRate >= outRate then i32 32768
  else
    -- alpha = 3.14 r / (3.14 r + R)
    let num := 314 * sfxRate
    ((32768 * num) / (num + 100 * outRate)).toUInt32.toInt32

def Mixer.numChannels (m : Mixer) : Nat := m.chans.size

@[inline] def Mixer.modChan (m : Mixer) (i : Nat) (f : Chan → Chan) : Mixer :=
  let cs := m.chans
  let m := { m with chans := #[] }
  { m with chans := cs.modify i f }

/-- Set a channel's volume (0..127) and separation (0..255, 128 = centre). -/
def Mixer.update (m : Mixer) (ch : Nat) (vol sep : Int) : Mixer :=
  let (l, r) := panning vol sep
  m.modChan ch fun c => { c with gainL := gainQ16 l, gainR := gainQ16 r, left := l.toUInt32, right := r.toUInt32 }

/--
Start `sfx` on channel `ch` (replacing whatever played there) at volume
`vol` and separation `sep`; `pitch` (128 = normal) is used only when the
mixer was created with `pitchShift`.
-/
def Mixer.start (m : Mixer) (ch : Nat) (sfx : Sfx) (vol sep : Int) (pitch : Nat := normPitch) : Mixer :=
  let p := if m.pitchShift then min pitch 255 else normPitch
  let step : Nat := (sfx.rate * 65536 * 128) / (m.rate * (256 - p))
  let alpha := lowPassAlpha (sfx.rate * 128 / (256 - p)) m.rate
  -- the filter starts settled on the first sample (Chocolate leaves sample 0 unfiltered)
  let first : Int32 := ((sfx.data.get! sfx.offset).toUInt32 * 257).toInt32 - i32 32768
  let m := m.modChan ch fun _ =>
    { data := sfx.data, offset := sfx.offset.toUInt32, length := sfx.length.toUInt32,
      stepInt := (step >>> 16).toUInt32, stepFrac := (step &&& 0xffff).toUInt32,
      alpha := alpha, lp := first, active := true }
  m.update ch vol sep

def Mixer.stop (m : Mixer) (ch : Nat) : Mixer :=
  m.modChan ch fun c => { c with active := false, data := .empty }

def Mixer.isPlaying (m : Mixer) (ch : Nat) : Bool :=
  match m.chans[ch]? with
  | some c => c.active
  | none => false

/-- Saturating add of `v` to the s16le sample at byte offset `o`. -/
@[inline] def addS16 (b : ByteArray) (o : Nat) (v : Int32) : ByteArray :=
  let cur := sext16 ((b.get! o).toUInt32 ||| ((b.get! (o + 1)).toUInt32 <<< 8))
  let u := (clip16 (cur + v)).toUInt32
  (b.set! o u.toUInt8).set! (o + 1) (u >>> 8).toUInt8

structure LoopOut where
  buf : ByteArray
  pos : UInt32
  frac : UInt32
  lp : Int32

/-- Mix one channel into frames `[i, n)` of `buf`. -/
def mixLoop (data : @& ByteArray) (buf : ByteArray) (i n : Nat) (off len pos frac si sf : UInt32)
    (gl gr alpha lp : Int32) : LoopOut :=
  if i < n then
    if pos < len then
      let d := (data.get! (off + pos).toNat).toUInt32
      let s : Int32 := (d * 257).toInt32 - i32 32768
      let lp := lp + (((s - lp) * alpha) >>> i32 15)
      let o := 4 * i
      let buf := addS16 buf o ((lp * gl) >>> i32 16)
      let buf := addS16 buf (o + 2) ((lp * gr) >>> i32 16)
      let frac := frac + sf
      mixLoop data buf (i + 1) n off len (pos + si + (frac >>> 16)) (frac &&& 0xffff) si sf gl gr alpha lp
    else { buf, pos, frac, lp }
  else { buf, pos, frac, lp }
termination_by n - i

/-- `n` zero bytes. -/
def zeros (n : Nat) : ByteArray := go (ByteArray.emptyWithCapacity n) n
where
  go (b : ByteArray) : Nat → ByteArray
    | 0 => b
    | k + 1 => go (b.push 0) k

/-- Saturating add of a whole s16le buffer (`src`) onto `dst`. -/
def addBuffer (dst : ByteArray) (src : @& ByteArray) (i n : Nat) : ByteArray :=
  if i < n then
    let o := 2 * i
    let v := sext16 ((src.get! o).toUInt32 ||| ((src.get! (o + 1)).toUInt32 <<< 8))
    addBuffer (addS16 dst o v) src (i + 1) n
  else dst
termination_by n - i

def mixChannels (chans : Array Chan) (buf : ByteArray) (frames : Nat) (i : Nat) : Array Chan × ByteArray :=
  if h : i < chans.size then
    let c := chans[i]
    if c.active then
      let r := mixLoop c.data buf 0 frames c.offset c.length c.pos c.frac c.stepInt c.stepFrac
        c.gainL c.gainR c.alpha c.lp
      let ended := r.pos >= c.length
      let chans := chans.modify i fun c =>
        { c with pos := r.pos, frac := r.frac, lp := r.lp, active := !ended,
                 data := if ended then .empty else c.data }
      mixChannels chans r.buf frames (i + 1)
    else mixChannels chans buf frames (i + 1)
  else (chans, buf)
termination_by chans.size - i
decreasing_by all_goals (simp_all; omega)

/--
Mix `frames` frames: all active channels, then `music` (s16le stereo,
`frames * 4` bytes, or empty for none). Returns the new mixer and the
interleaved s16le stereo output.
-/
def Mixer.mix (m : Mixer) (frames : Nat) (music : ByteArray := .empty) : Mixer × ByteArray :=
  let hasMusic := music.size == frames * 4
  if !(m.chans.any (·.active)) then
    (m, if hasMusic then music else zeros (frames * 4))
  else
    let cs := m.chans
    let m := { m with chans := #[] }
    let (cs, buf) := mixChannels cs (zeros (frames * 4)) frames 0
    let buf := if hasMusic then addBuffer buf music 0 (frames * 2) else buf
    ({ m with chans := cs }, buf)

end Doom.Audio.Mixer
