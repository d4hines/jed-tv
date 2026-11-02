import Doom.Wad
/-!
# AudioCheck.Util (host test support)

WAD lump loading, WAV writing and simple signal statistics for the audio
checks.
-/

namespace AudioCheck.Util

open Doom.Wad

def loadLump (wad : WadDirectory) (name : String) : IO ByteArray := do
  match checkNumForName wad name with
  | none => throw (IO.userError s!"lump {name} not found")
  | some i =>
    match lumpData wad i with
    | .ok b => pure b
    | .error e => throw (IO.userError e)

def lumpOpt (wad : WadDirectory) (name : String) : Option ByteArray :=
  match checkNumForName wad name with
  | none => none
  | some i => (lumpData wad i).toOption

/-- Names of all lumps starting with `pfx` (directory order, unique). -/
def lumpNames (wad : WadDirectory) (pfx : String) : Array String := Id.run do
  let mut out : Array String := #[]
  for e in wad.entries do
    let n := String.ofList ((e.name.toList.takeWhile (· != 0)).map fun c => Char.ofNat c.toNat)
    if n.startsWith pfx && !out.contains n then out := out.push n
  return out

private def le32 (b : ByteArray) (v : Nat) : ByteArray :=
  b.push (v % 256).toUInt8 |>.push ((v >>> 8) % 256).toUInt8 |>.push ((v >>> 16) % 256).toUInt8
    |>.push ((v >>> 24) % 256).toUInt8
private def le16 (b : ByteArray) (v : Nat) : ByteArray :=
  b.push (v % 256).toUInt8 |>.push ((v >>> 8) % 256).toUInt8

/-- A 16-bit stereo PCM WAV file around raw s16le interleaved data. -/
def wav (rate : Nat) (pcm : ByteArray) : ByteArray :=
  let h := "RIFF".toUTF8
  let h := le32 h (36 + pcm.size)
  let h := h ++ "WAVEfmt ".toUTF8
  let h := le32 h 16
  let h := le16 h 1
  let h := le16 h 2
  let h := le32 h rate
  let h := le32 h (rate * 4)
  let h := le16 h 4
  let h := le16 h 16
  let h := h ++ "data".toUTF8
  let h := le32 h pcm.size
  h ++ pcm

def sample (b : ByteArray) (i : Nat) : Int :=
  let u := (b.get! (2 * i)).toNat ||| ((b.get! (2 * i + 1)).toNat <<< 8)
  if u >= 0x8000 then (u : Int) - 0x10000 else u

structure Stats where
  frames : Nat
  rmsL : Float
  rmsR : Float
  peak : Nat
  /-- Samples at full scale (32767 or -32768). -/
  clipped : Nat
  /-- Fraction of 100 ms windows (left+right) with RMS below 30 (about -60 dBFS). -/
  silentWindows : Float
  /-- Zero-crossing rate per second (left channel), a rough brightness measure. -/
  zcr : Float
  /-- RMS of the first difference over RMS (spectral tilt: higher = more high-frequency energy). -/
  hfRatio : Float

def stats (pcm : ByteArray) (rate : Nat) : Stats := Id.run do
  let frames := pcm.size / 4
  let mut sl : Float := 0
  let mut sr : Float := 0
  let mut sd : Float := 0
  let mut peak : Nat := 0
  let mut clipped := 0
  let mut zc := 0
  let mut prevL : Int := 0
  let win := rate / 10
  let mut wsum : Float := 0
  let mut wcount := 0
  let mut silent := 0
  let mut windows := 0
  for i in [0:frames] do
    let l := sample pcm (2 * i)
    let r := sample pcm (2 * i + 1)
    let lf := Float.ofInt l
    let rf := Float.ofInt r
    sl := sl + lf * lf
    sr := sr + rf * rf
    let d := Float.ofInt (l - prevL)
    sd := sd + d * d
    if (l >= 0) != (prevL >= 0) then zc := zc + 1
    prevL := l
    let a := max l.natAbs r.natAbs
    if a > peak then peak := a
    if l >= 32767 || l <= -32768 then clipped := clipped + 1
    if r >= 32767 || r <= -32768 then clipped := clipped + 1
    wsum := wsum + (lf * lf + rf * rf) / 2
    wcount := wcount + 1
    if wcount == win then
      windows := windows + 1
      if (wsum / win.toFloat).sqrt < 30 then silent := silent + 1
      wsum := 0
      wcount := 0
  let n := (max frames 1).toFloat
  let rmsL := (sl / n).sqrt
  return { frames, rmsL, rmsR := (sr / n).sqrt, peak, clipped,
           silentWindows := if windows == 0 then 1 else silent.toFloat / windows.toFloat,
           zcr := zc.toFloat * rate.toFloat / n,
           hfRatio := if rmsL == 0 then 0 else (sd / n).sqrt / rmsL }

def Stats.summary (s : Stats) (rate : Nat) : String :=
  let r (x : Float) : String := toString ((x * 10).round / 10)
  s!"{s.frames} frames ({r (s.frames.toFloat / rate.toFloat)} s), rms L/R {r s.rmsL}/{r s.rmsR}, " ++
  s!"peak {s.peak}, clipped {s.clipped}, silent windows {r (s.silentWindows * 100)}%, " ++
  s!"zcr {r s.zcr}/s, hf ratio {r (s.hfRatio * 100)}%"

end AudioCheck.Util
