import Update.Receiver
import Update.Install

/- Complete datagram-to-install dispatcher. Network adapters only deliver
   owned bytes and send the resulting response; all update decisions are Lean. -/
namespace LeanBoot.Update.Server
open Memory
variable {m : Type → Type} [Monad m] [Reader m] [Writer m] [Crypto m] [Storage m]

@[inline] def separate (a n b k : UInt32) : Bool :=
  addressable a n && addressable b k && (a + n <= b || b + k <= a)

@[specialize] private partial def equal (a b n : UInt32) : m Bool := do
  if n == 0 then return true
  if (← Reader.byte a) != (← Reader.byte b) then return false
  equal (a + 1) (b + 1) (n - 1)

@[specialize] private def reply (out session op status acknowledged : UInt32) : m UInt32 := do
  let _ ← writeLE32 out 20 0 0x3152554c
  let _ ← writeLE32 out 20 4 session
  let _ ← writeLE32 out 20 8 op
  let _ ← writeLE32 out 20 12 status
  let _ ← writeLE32 out 20 16 acknowledged
  return 20

/-- State is 40 bytes: 16 receiver bytes, last completed Seal packet (20),
    and its status (4). Start with zeroes. Cached Seal replies make commit
    idempotent even when its first acknowledgement is lost. -/
@[inline] def dispatch (commit : UInt32 → UInt32 → m UInt32)
    (state buffer capacity packet size out scratch scratchSize : UInt32) : m UInt32 := do
  if size < 20 || size > 1044 ||
      !separate state 40 buffer capacity || !separate state 40 packet size ||
      !separate state 40 out 20 || !separate state 40 scratch scratchSize ||
      !separate buffer capacity packet size || !separate buffer capacity out 20 ||
      !separate buffer capacity scratch scratchSize || !separate packet size out 20 ||
      !separate packet size scratch scratchSize || !separate out 20 scratch scratchSize then return 0
  if (← readLE32 packet size 0) != 0x3152554c then return 0
  let op := (← readLE32 packet size 4).toUInt32
  let session := (← readLE32 packet size 8).toUInt32
  if op == 3 && size == 20 && (← readLE32 state 40 0) == 0 &&
      (← equal (state + 16) packet 20) then
    let status := (← readLE32 state 40 36).toUInt32
    let acknowledged := (← readLE32 packet size 12).toUInt32
    return ← reply out session op status acknowledged
  let result ← Receiver.receive state buffer capacity packet size
  let status := (result >>> 32).toUInt32
  let acknowledged := result.toUInt32
  if status != 0 then return ← reply out session op status acknowledged
  if op == 1 then
    let _ ← writeLE32 state 40 16 0
    return ← reply out session op 0 acknowledged
  if op != 3 then return ← reply out session op 0 acknowledged
  let installed ← commit buffer acknowledged
  let status := if installed == 0 then 0 else installed + 16
  let _ ← copy state 40 16 packet size 0 20
  let _ ← writeLE32 state 40 36 status
  let _ ← fill state 40 0 16 0
  reply out session op status acknowledged

@[inline] def respond (policy : Policy) (state buffer capacity packet size
    out scratch scratchSize : UInt32) : m UInt32 :=
  dispatch (fun source bytes => installNext policy source bytes scratch scratchSize)
    state buffer capacity packet size out scratch scratchSize

end LeanBoot.Update.Server
