import Types
import Memory
import Update.Policy

/- Pure receiver state machine. Packet fields have already been decoded from
   a bounded input span. No storage, crypto, register or execution capability. -/
namespace LeanBoot.Update.Receiver

structure SessionId where
  value : UInt32
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq

structure ByteOffset where
  value : UInt32
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq

inductive Phase | idle | receiving | sealed
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq
inductive Opcode | begin | data | seal | abort | invalid
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq
inductive Action | reply | begin | append | compare | seal | abort
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq

structure State where
  phase : Phase
  session : SessionId
  total : ByteCount
  received : ByteCount
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq

structure Packet where
  opcode : Opcode
  session : SessionId
  offset : ByteOffset
  count : ByteCount
  size : ByteCount
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq

structure Decision where
  action : Action
  status : Status
  acknowledged : ByteCount
  next : State
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq

@[inline] def Phase.word : Phase → UInt32
  | .idle => 0 | .receiving => 1 | .sealed => 2
@[inline] def Opcode.decode : UInt32 → Opcode
  | 1 => .begin | 2 => .data | 3 => .seal | 4 => .abort | _ => .invalid

@[inline] def chunkFits (total received offset count : UInt32) : Bool :=
  received <= total && offset <= received && count > 0 && count <= 1024 &&
  Memory.contains total offset count && (offset == received || Memory.contains received offset count)

@[inline] def reply (s : State) (code : UInt32) : Decision :=
  ⟨.reply, ⟨code⟩, s.received, s⟩
@[inline] def malformed (s : State) : Decision :=
  ⟨.reply, ⟨1⟩, ⟨0⟩, s⟩

/-- A successful append alone advances the high-water mark. Retransmissions
    request comparison, never a write. Seal is admitted only at exact length. -/
@[inline] def step (s : State) (p : Packet) (capacity : ByteCount) : Decision :=
  if s.received.value > s.total.value || s.total.value > capacity.value then malformed s
  else if p.opcode == .begin then
    if p.size.value != 20 || p.offset.value != 0 || p.session.value == 0 ||
        p.count.value < headerSize || p.count.value > capacity.value ||
        p.count.value > maxPayload + headerSize then malformed s
    else if s.phase != .idle then
      if s.session == p.session && s.total == p.count then reply s 0 else reply s 2
    else ⟨.begin, .ok, ⟨0⟩, ⟨.receiving, p.session, p.count, ⟨0⟩⟩⟩
  else if s.phase == .idle || p.session != s.session then reply s 2
  else if p.opcode == .abort then
    if p.size.value != 20 || p.offset.value != 0 || p.count.value != 0 then malformed s
    else ⟨.abort, .ok, ⟨0⟩, ⟨.idle, ⟨0⟩, ⟨0⟩, ⟨0⟩⟩⟩
  else if p.opcode == .seal then
    if p.size.value != 20 || p.offset.value != s.total.value || p.count.value != 0 then malformed s
    else if s.received != s.total then reply s 4
    else ⟨.seal, .ok, s.received, { s with phase := .sealed }⟩
  else if p.opcode != .data || p.size.value - 20 != p.count.value then malformed s
  else if !chunkFits s.total.value s.received.value p.offset.value p.count.value then reply s 3
  else if p.offset.value < s.received.value then ⟨.compare, .ok, s.received, s⟩
  else if s.phase != .receiving then reply s 2
  else
    let next : ByteCount := ⟨s.received.value + p.count.value⟩
    ⟨.append, .ok, next, { s with received := next }⟩

end LeanBoot.Update.Receiver
