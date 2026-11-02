import Axiom.World

namespace Axiom

/-- Samples per second of the TV's sound output. Upstream synthesized WAV files
at 44.1 kHz for Raylib; the TV plays raw signed 16-bit mono at this rate. -/
def audioRate : Nat := 22050

@[inline] def audioNoise (kind i : Nat) : Float32 :=
  let raw := (hash2 (0xa710d10 + UInt64.ofNat kind) i (i * 17 + kind * 31) % 2001).toNat
  raw.toFloat32 / 1000 - 1

/-- The four sounds: 0 UI click, 1 break, 2 place, 3 jump. Unchanged from upstream. -/
def synthSample (kind : Nat) (t duration : Float32) : Float32 :=
  let phase (frequency : Float32) := (6.2831853 * frequency * t).sin
  let life := max 0 (1 - t / duration)
  let value :=
    match kind with
    | 0 => (phase 620 * 0.55 + phase 930 * 0.2) * life * life
    | 1 => (audioNoise kind (t * 44100).toUInt32.toNat * 0.72 + phase 105 * 0.28) * life
    | 2 => (phase 132 * 0.72 + phase 66 * 0.28) * life * life
    | _ => phase (300 + 420 * (t / duration)) * life * 0.72
  max (-1) (min 1 value)

/-- Little-endian signed 16-bit mono samples for `duration` seconds at `volume` (0..1). -/
def synthPcm (kind : Nat) (duration volume : Float32) : ByteArray := Id.run do
  let frameCount := (duration * audioRate.toFloat32).toUInt32.toNat
  let mut out := ByteArray.emptyWithCapacity (frameCount * 2)
  for i in [0:frameCount] do
    let t := i.toFloat32 / audioRate.toFloat32
    let signed := (synthSample kind t duration * volume * 30000).toInt32.toInt
    let raw : Nat := if signed < 0 then (signed + 65536).toNat else signed.toNat
    out := out.push (UInt16.ofNat raw).toUInt8 |>.push ((UInt16.ofNat raw) >>> 8).toUInt8
  return out

structure AudioBank where
  ui : ByteArray := ByteArray.empty
  breakBlock : ByteArray := ByteArray.empty
  placeBlock : ByteArray := ByteArray.empty
  jump : ByteArray := ByteArray.empty

def loadAudioBank : AudioBank := {
  ui := synthPcm 0 0.075 0.25
  breakBlock := synthPcm 1 0.13 0.34
  placeBlock := synthPcm 2 0.11 0.38
  jump := synthPcm 3 0.15 0.22
}

end Axiom
