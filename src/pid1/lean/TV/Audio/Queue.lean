prelude
import TV.Audio.Converter

/-! The direct driver's bounded PCM queue submission and consumption accounting.
No ioctl, ownership, or transport error is hidden inside this state machine. -/
namespace TV.Audio

structure QueueEffects (m : Type → Type) where
  /-- One nonblocking driver write of at most `count` bytes from `offset`.
  The result is the actual accepted byte count, including zero/backpressure. -/
  write : ByteArray → UInt32 → UInt32 → m UInt32
  /-- Record output frames immediately, even if a later write fails. -/
  accepted : UInt32 → m Unit
  clockNs : m Word64
  sleepMs : UInt32 → m Unit

private partial def submit [Monad m] [MonadExceptOf Failure m] (e : QueueEffects m)
    (data : ByteArray) (offset : Nat) (deadline : Nat) : m Unit := do
  if offset == data.size then return
  if (← e.clockNs).toUInt64.toNat ≥ deadline then throw (.native "direct PCM write deadline" 110)
  let count := min 1920 (data.size - offset)
  let written ← e.write data offset.toUInt32 count.toUInt32
  if written.toNat > count || written % 4 != 0 then
    throw (.protocol "invalid MI write count")
  e.accepted (written / 4)
  if written == 0 then e.sleepMs 2
  submit e data (offset + written.toNat) deadline

/-- At most 10 ms of 48 kHz stereo s16 PCM per nonblocking ioctl. Positive
short writes and full-queue stalls share the same five-second deadline. -/
def submitPcm [Monad m] [MonadExceptOf Failure m] (e : QueueEffects m)
    (data : ByteArray) : m Unit := do
  if data.size % 4 != 0 || data.size > 0xffffffff then
    throw (.protocol "unaligned/oversized PCM")
  let deadline := (← e.clockNs).toUInt64.toNat + 5000000000
  submit e data 0 deadline

/-- Driver queue observations must be aligned and cannot exceed the total
output accepted. Return complete consumed input frames, capped by input
acknowledgements, so a partially submitted input block is never overcounted. -/
def playedInputFrames (queuedBytes : UInt32) (outputWritten inputWritten : UInt64)
    (rate : UInt32) : Except Failure UInt64 := do
  if queuedBytes % 4 != 0 || queuedBytes.toUInt64 / 4 > outputWritten then
    throw (.protocol "inconsistent hardware PCM queue")
  return consumedInputFrames (outputWritten - queuedBytes.toUInt64 / 4) rate inputWritten

end TV.Audio
