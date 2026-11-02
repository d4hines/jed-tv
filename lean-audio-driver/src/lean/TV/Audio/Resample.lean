prelude
import Init.Data.UInt
import Init.Data.SInt.Basic
import Init.Data.ByteArray.Basic
import Init.Data.List.Basic
import Init.Data.Option.Basic
import TV.Audio.Gain

/- The hardware takes 48 kHz stereo only. Apps send 44.1 or 48 kHz, mono or
   stereo. Conversion is linear interpolation with an exact rational phase:
   every input frame advances the phase by 48000, and one output frame is
   produced each time it passes `rate`, so n input frames give exactly
   n * 48000 / rate output frames. The last input frame is kept, so splitting
   the input into chunks never changes the output. Phase is always < rate. -/
namespace TV.Audio.Resample

def outputRate : UInt32 := 48000

/-- A frame as two 16-bit samples; mono input is duplicated into both. -/
abbrev Frame := Int16 × Int16

structure Converter where
  rate : UInt32
  channels : UInt32
  phase : UInt32
  previous : Option Frame
  deriving Repr

/-- Only 44.1 or 48 kHz, mono or stereo. -/
@[inline] def Converter.new (rate channels : UInt32) : Option Converter :=
  if (rate == 44100 || rate == 48000) && (channels == 1 || channels == 2)
  then some { rate, channels, phase := 0, previous := none } else none

/-- One input frame moves the phase by 48000: the number of output frames to
emit (0, 1 or 2, since 48000 < 2 * rate) and the phase that remains. -/
@[inline] def step (rate phase : UInt32) : UInt32 × UInt32 :=
  let p := phase + outputRate
  if p >= rate then
    if p - rate >= rate then (2, p - rate - rate) else (1, p - rate)
  else (0, p)

/-- `(previous * phase + current * (48000 - phase)) / 48000` for one channel,
truncating toward zero as the Rust driver did. Both products fit in 32 bits. -/
@[inline] def mix (previous current : Int16) (phase : UInt32) : Int16 :=
  let p := previous.toInt32 * phase.toInt32
  let c := current.toInt32 * (outputRate - phase).toInt32
  ((p + c) / outputRate.toInt32).toInt16

/-- Append one 48 kHz stereo frame, interpolated at `phase`. -/
@[inline] def emitFrame (previous current : Frame) (phase : UInt32) (out : ByteArray) : ByteArray :=
  Gain.pushInt16 (Gain.pushInt16 out (mix previous.1 current.1 phase)) (mix previous.2 current.2 phase)

/-- Feed one input frame: emit what `step` says (the first output frame at the
phase after one subtraction, the second after two), then remember the frame. -/
def feed (c : Converter) (current : Frame) (out : ByteArray) : Converter × ByteArray :=
  let previous := c.previous.getD current
  let (count, phase) := step c.rate c.phase
  let out :=
    if count == 0 then out
    else if count == 1 then emitFrame previous current phase out
    else emitFrame previous current phase (emitFrame previous current (phase + c.rate) out)
  ({ c with phase, previous := some current }, out)

/-- The whole conversion is a fold of `feed` over the input frames. -/
def convertFrames (c : Converter) (frames : List Frame) (out : ByteArray) : Converter × ByteArray :=
  frames.foldl (fun (c, out) frame => feed c frame out) (c, out)

/-- Interleaved little-endian 16-bit input, `channels` samples per frame. -/
def decodeFrames (channels : UInt32) (input : ByteArray) : List Frame :=
  let frameBytes := channels.toNat * 2
  (List.range (input.size / frameBytes)).map fun i =>
    (Gain.readInt16 input (i * frameBytes), Gain.readInt16 input (i * frameBytes + frameBytes - 2))

/-- Interleaved little-endian 16-bit input; 48 kHz stereo output in the same
encoding. `none` when the input does not hold whole frames. -/
def Converter.convert (c : Converter) (input : ByteArray) : Option (Converter × ByteArray) :=
  let frameBytes := c.channels.toNat * 2
  if input.size % frameBytes != 0 then none
  else some (convertFrames c (decodeFrames c.channels input)
    (ByteArray.emptyWithCapacity ((input.size / frameBytes + 1) * 8)))

/-- Number of whole input frames whose output has all been consumed: the
inverse of `floor(input * 48000 / rate)`, saturated at the accepted input. -/
def consumedNat (output rate written : Nat) : Nat :=
  min (((output + 1) * rate - 1) / 48000) written

def consumedInputFrames (output : UInt64) (rate : UInt32) (written : UInt64) : UInt64 :=
  (consumedNat output.toNat rate.toNat written.toNat).toUInt64

end TV.Audio.Resample
