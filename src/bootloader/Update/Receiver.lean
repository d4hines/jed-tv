import Memory
import Update.Receiver.Plan

/- Bounded stop-and-wait datagram payload receiver. Transport adapters supply
   owned stable packet bytes after validating Ethernet/IP/UDP. This module
   cannot read or write storage and cannot authorize an update. -/
namespace LeanBoot.Update.Receiver
open Memory
variable {m : Type → Type} [Monad m] [Reader m] [Writer m]

/-- Session state occupies 16 bytes: phase, id, total, received. Phase 0 is
    idle, 1 receiving, 2 sealed. Caller initializes it to zero once. -/
@[specialize] private partial def equalBytes (a b n : UInt32) : m Bool := do
  if n == 0 then return true
  if (← Reader.byte a) != (← Reader.byte b) then return false
  equalBytes (a + 1) (b + 1) (n - 1)

/-- Result packs status in the high word and acknowledged byte count in the
    low word. Status 0 ACK, 1 malformed, 2 busy/session, 3 offset, 4 incomplete,
    5 duplicate mismatch. Begin/Data/Seal/Abort use opcodes 1/2/3/4. -/
@[specialize] def receive (state buffer capacity packet size : UInt32) : m UInt64 := do
  if !addressable state 16 || !addressable buffer capacity ||
      !addressable packet size || capacity < headerSize || size < 20 || size > 1044 ||
      (state < buffer + capacity && buffer < state + 16) ||
      (packet < buffer + capacity && buffer < packet + size) ||
      (state < packet + size && packet < state + 16) then return 0x100000000
  if (← readLE32 packet size 0) != 0x3152554c then return 0x100000000
  let op := (← readLE32 packet size 4).toUInt32
  let id := (← readLE32 packet size 8).toUInt32
  let offset := (← readLE32 packet size 12).toUInt32
  let count := (← readLE32 packet size 16).toUInt32
  let phase := (← readLE32 state 16 0).toUInt32
  let session := (← readLE32 state 16 4).toUInt32
  let total := (← readLE32 state 16 8).toUInt32
  let received := (← readLE32 state 16 12).toUInt32
  if phase > 2 then return 0x100000000
  let decoded : State := {
    phase := if phase == 0 then .idle else if phase == 1 then .receiving else .sealed
    session := ⟨session⟩, total := ⟨total⟩, received := ⟨received⟩ }
  let packetFields : Packet := {
    opcode := Opcode.decode op, session := ⟨id⟩, offset := ⟨offset⟩,
    count := ⟨count⟩, size := ⟨size⟩ }
  let decision := step decoded packetFields ⟨capacity⟩
  match decision.action with
  | .begin =>
    let _ ← writeLE32 state 16 0 decision.next.phase.word
    let _ ← writeLE32 state 16 4 decision.next.session.value
    let _ ← writeLE32 state 16 8 decision.next.total.value
    let _ ← writeLE32 state 16 12 decision.next.received.value
  | .abort => let _ ← fill state 16 0 16 0; pure ()
  | .seal => let _ ← writeLE32 state 16 0 decision.next.phase.word; pure ()
  | .compare =>
    if !(← equalBytes (buffer + offset) (packet + 20) count) then
      return 0x500000000 ||| received.toUInt64
  | .append =>
    if (← copy buffer capacity offset packet size 20 count) != 0 then return 0x100000000
    let _ ← writeLE32 state 16 12 decision.next.received.value
  | .reply => pure ()
  return decision.status.code.toUInt64 <<< 32 ||| decision.acknowledged.value.toUInt64

end LeanBoot.Update.Receiver
