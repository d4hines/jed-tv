prelude
import TV.Audio.Protocol
import Init.Data.Int.DivMod.Basic

/-! Input-rate PCM to the driver's fixed 48 kHz stereo format. Keeping phase
and the previous input frame in the value makes chunk boundaries irrelevant. -/
namespace TV.Audio

structure Stereo where
  left : Int16
  right : Int16
  deriving BEq, Repr

structure Converter where
  rate : UInt32
  channels : UInt32
  phase : UInt32 := 0
  previous : Option Stereo := none
  deriving BEq, Repr

def Converter.create (rate channels : UInt32) : Except Failure Converter :=
  if (rate == 44100 || rate == 48000) && (channels == 1 || channels == 2) then
    .ok ⟨rate, channels, 0, none⟩
  else .error (.protocol "direct audio supports 44.1/48k mono/stereo")

private def sample (bytes : ByteArray) (offset : Nat) : Int16 :=
  (bytes[offset]!.toUInt16 ||| (bytes[offset + 1]!.toUInt16 <<< 8)).toInt16

private def emit (bytes : ByteArray) (value : Int) : ByteArray :=
  let bits := value.toInt16.toUInt16
  bytes.push bits.toUInt8 |>.push (bits >>> 8).toUInt8

/-- Signed division truncates toward zero, exactly as in the original Rust
converter. Euclidean division would introduce a one-LSB negative-sample bias. -/
private def interpolate (previous current : Int16) (fraction : UInt32) : Int :=
  (previous.toInt * (48000 - fraction.toNat) + current.toInt * fraction.toNat).tdiv 48000

def Converter.convert (state : Converter) (input : ByteArray) : Except Failure (Converter × ByteArray) := do
  if !(state.rate == 44100 || state.rate == 48000) ||
      !(state.channels == 1 || state.channels == 2) || state.phase ≥ state.rate then
    throw (.protocol "invalid converter state")
  if input.size % (state.channels.toNat * 2) != 0 then
    throw (.protocol "incomplete PCM frame")
  let mut state := state
  let mut out := ByteArray.empty
  for frame in [0:input.size / (state.channels.toNat * 2)] do
    let offset := frame * state.channels.toNat * 2
    let current : Stereo := ⟨sample input offset, sample input (offset + (state.channels.toNat - 1) * 2)⟩
    let previous := state.previous.getD current
    state := { state with phase := state.phase + 48000 }
    for _ in [0:2] do
      if state.phase ≥ state.rate then
        state := { state with phase := state.phase - state.rate }
        let fraction := 48000 - state.phase
        out := emit out (interpolate previous.left current.left fraction)
        out := emit out (interpolate previous.right current.right fraction)
    state := { state with previous := some current }
  return (state, out)

/-- Invert floor(input * 48000 / rate), counting only input frames whose
complete output has played; then bound by the acknowledged input count. -/
def consumedInputFrames (output : UInt64) (rate : UInt32) (written : UInt64) : UInt64 :=
  min (((output.toNat + 1) * rate.toNat - 1) / 48000).toUInt64 written

end TV.Audio
