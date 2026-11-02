import DoomTest.Net.Rng
/-!
# net-fuzz: decoder and session fuzzing

1. `decode` on random datagrams and on mutations (bit flips, byte overwrites,
   truncation, extension, slice duplication, splicing) of valid messages and
   of real captured traffic. Every result must be `invalid`, `wrongVersion` on
   a foreign HELLO, or `ok` with bounded fields whose re-encoding is
   byte-identical (strict, canonical decoding).
2. Sessions in every phase (host lobby/running, joiner joining/lobby/running)
   fed with random and mutated datagrams from legitimate and stranger
   addresses, interleaved with polls, local commands, tic runs and checksums.
   Invariants (buffer sizes, tic ordering, bounded queues, datagram sizes)
   are checked after every step.

Run with `LEAN_ABORT_ON_PANIC=1` so any panic aborts. Exit 0 iff all passed.
Usage: `net-fuzz [iterations-scale]` (default 1).
-/

namespace Doom.Net.Test.Fuzz

open Doom.Net
open Doom.Net.Test
open Doom.Playsim.Player (TicCmd)

def addrOf (i : Nat) : Addr := ⟨0x0A000001 + i.toUInt32, PORT⟩
def digest : ByteArray := ⟨#[7, 7, 7]⟩

/-! ## Mutations -/

def mutate (b0 : ByteArray) (other : ByteArray) : R ByteArray := do
  let mut b := b0
  let rounds ← range 1 4
  for _ in [0:rounds] do
    match ← below 8 with
    | 0 => -- bit flips
      for _ in [0:(← range 1 8)] do
        if b.size > 0 then
          let i ← below b.size
          b := put8 b i (rd8 b i ^^^ ((1 : UInt8) <<< (← below 8).toUInt8))
    | 1 => -- byte overwrite with interesting values
      if b.size > 0 then
        let i ← below b.size
        let v := match ← below 6 with
          | 0 => 0 | 1 => 0xff | 2 => 0x7f | 3 => 0x80 | 4 => 1 | _ => 0
        let rv ← randU8
        b := put8 b i (if (← chance 300) then rv else v)
    | 2 => b := b.extract 0 (← below (b.size + 1))                 -- truncate
    | 3 => b := b.append (← randBytes (← range 1 16))              -- extend
    | 4 => -- duplicate a slice
      if b.size > 1 then
        let i ← below b.size
        let n ← range 1 (min 64 (b.size - i))
        b := (b.extract 0 (i + n)).append (b.extract i b.size)
    | 5 => -- splice with another datagram
      let i ← below (b.size + 1)
      let j ← below (other.size + 1)
      b := (b.extract 0 i).append (other.extract j other.size)
    | 6 => -- overwrite a count/length-like byte past the header
      if b.size > 8 then
        let i ← range 8 (b.size - 1)
        b := put8 b i (← randU8)
    | _ => -- 32-bit field overwrite
      if b.size ≥ 12 then
        let i ← range 4 (b.size - 4)
        let v := match ← below 4 with | 0 => 0 | 1 => 0xFFFFFFFF | 2 => 0x7FFFFFFF | _ => 0
        let rv ← randU32
        b := put32 b i (if (← chance 500) then rv else v)
  return b

def randomDatagram : R ByteArray := do
  let n ← below 1501
  let mut b ← randBytes n
  if (← chance 400) && b.size ≥ 4 then
    b := put8 (put8 b 0 MAGIC0) 1 MAGIC1
    if ← chance 800 then b := put8 b 2 PROTOCOL_VERSION
    if ← chance 800 then b := put8 b 3 (← range 1 7).toUInt8
  return b

/-! ## Decoded-message bounds -/

def boundedMsg : Msg → Bool
  | .hello _ d n => d.size ≤ DIGEST_MAX && n.utf8ByteSize ≤ NAME_MAX
  | .lobby _ you m names => you < 4 && m ≤ 15 && names.size == 4
      && names.all (·.utf8ByteSize ≤ NAME_MAX)
  | .reject .. => true
  | .start _ i => i.names.size == 4 && i.names.all (·.utf8ByteSize ≤ NAME_MAX)
      && i.playerMask ≤ 15 && i.consoleplayer < 4 && i.settings.valid
  | .cmds c => c.count ≤ MAX_CMDS_PER_PACKET && c.cmds.size == c.count * CMD_BYTES
      && c.sumCount ≤ MAX_SUMS_PER_PACKET && c.sums.size == 4 * c.sumCount && c.slot < 4
  | .sets s => s.count ≤ MAX_SETS_PER_PACKET && s.body.size ≤ MAX_SETS_PER_PACKET * SET_STRIDE
      && s.drops.size ≤ MAX_PLAYERS && s.desyncMask ≤ 15
  | .leave slot _ => slot < 4

structure Tally where
  total : Nat := 0
  ok : Nat := 0
  wrongVersion : Nat := 0
  invalid : Nat := 0
  perType : Array Nat := #[0, 0, 0, 0, 0, 0, 0]
  bad : Nat := 0
  deriving Inhabited

def classify (t : Tally) (b : ByteArray) : Tally :=
  let t := { t with total := t.total + 1 }
  match decode b with
  | .invalid => { t with invalid := t.invalid + 1 }
  | .wrongVersion v _ =>
    let good := v != PROTOCOL_VERSION && rd8 b 3 == T_HELLO
    { t with wrongVersion := t.wrongVersion + 1, bad := t.bad + (if good then 0 else 1) }
  | .ok sess m =>
    let canonical := rd8 b 2 != PROTOCOL_VERSION || encode sess m == b
    let good := canonical && boundedMsg m && b.size ≤ MAX_PACKET
    let ty := (rd8 b 3).toNat
    { t with ok := t.ok + 1, perType := t.perType.modify (ty - 1) (· + 1)
             bad := t.bad + (if good then 0 else 1) }

/-! ## Captured traffic and phase snapshots -/

structure Capture where
  corpus : Array (Addr × Addr × ByteArray) := #[]
  hostLobby : Session := default
  hostRunning : Session := default
  joinJoining : Session := default
  joinLobby : Session := default
  joinRunning : Session := default

/-- Lossless 4-peer exchange; records every datagram and snapshots of each phase. -/
def capture : Capture := Id.run do
  let mut ss : Array Session := #[Session.host { name := "h", iwadDigest := digest, nonce := 0x5151 }]
  for i in [1:4] do
    ss := ss.push (Session.join { name := s!"j{i}", iwadDigest := digest, nonce := 900 + i.toUInt32 } (addrOf 0))
  let mut cap : Capture := { joinJoining := ss.getD 1 default }
  let mut inbox : Array (Array (Addr × ByteArray)) := #[#[], #[], #[], #[]]
  let mut hashes : Array UInt64 := #[0, 0, 0, 0]
  for r in [0:240] do
    let now := (r * 10).toUInt64
    for i in [0:4] do
      let mut s := ss.getD i default
      for (src, b) in inbox.getD i #[] do s := s.receive src b now
      inbox := inbox.setIfInBounds i #[]
      if i == 0 && s.phase == .lobby && s.playerCount == 4 && r ≥ 40 then
        cap := { cap with hostLobby := s }
        match s.start { deathmatch := 2, skill := 4 } now with
        | .ok s' => s := s'
        | .error _ => pure ()
      if i == 1 && s.phase == .lobby then cap := { cap with joinLobby := s }
      let mut k := 0
      while k < 4 && s.localDue now do
        s := s.submitLocal { forwardmove := (r % 50).toInt32, sidemove := 3, angleturn := 640, buttons := 1 } now
        k := k + 1
      let (s', out) := s.poll now
      s := s'
      for (dst, b) in out do
        cap := { cap with corpus := cap.corpus.push (addrOf i, dst, b) }
        let di := (dst.ip - 0x0A000001).toNat
        inbox := inbox.modify di (·.push (addrOf i, b))
      let mut n := 0
      while n < 8 do
        match s.nextTic? with
        | some rt =>
          s := s.advance
          let h := (hashes.getD i 0) * 31 + rt.tic.toUInt64
          hashes := hashes.setIfInBounds i h
          s := s.submitChecksum rt.tic h.toUInt32
          n := n + 1
        | none => break
      ss := ss.setIfInBounds i s
    if r == 160 then
      cap := { cap with hostRunning := ss.getD 0 default, joinRunning := ss.getD 2 default }
  return cap

/-! ## Session invariants -/

def invariants (s : Session) : Option String :=
  if s.peers.size != 4 then some "peers size"
  else if s.setRing.size != RING * SET_STRIDE then some "set ring size"
  else if s.cmdRing.size != MAX_PLAYERS * RING * CMD_BYTES then some "cmd ring size"
  else if s.sumRing.size != MAX_PLAYERS * RING * 4 then some "sum ring size"
  else if s.runNext > s.setNext then some "runNext > setNext"
  else if s.setNext - s.runNext > RING32 then some "set window > RING"
  else if s.phase == .running && s.setNext > s.gen then some "set beyond local commands"
  else if s.outbox.size > OUTBOX_MAX then some "outbox unbounded"
  else if s.drops.size > MAX_PLAYERS then some "drop table unbounded"
  else if s.phase == .running && s.gen - s.runNext > s.maxLead then some "lead beyond window"
  else if s.role == .joiner && s.phase == .running && s.gen - s.cmdAck ≥ RING32 then some "unacked > RING"
  else none

structure SessTally where
  steps : Nat := 0
  polls : Nat := 0
  datagramsOut : Nat := 0
  maxOut : Nat := 0
  maxBatch : Nat := 0
  ticsRun : Nat := 0
  violations : Array String := #[]

def fuzzSession (label : String) (s0 : Session) (corpus : Array (Addr × Addr × ByteArray))
    (steps : Nat) (seed : UInt64) : SessTally := Id.run do
  let mut rng := Rng.ofSeed seed
  let mut s := s0
  let mut now : UInt64 := 2000
  let mut t : SessTally := {}
  for i in [0:steps] do
    if i % 400 == 0 then
      s := s0
      now := 2000
    let step : R (Addr × ByteArray × UInt64) := do
      let (src, _, base) := corpus.getD (← below corpus.size) (default, default, .empty)
      let (_, _, other) := corpus.getD (← below corpus.size) (default, default, .empty)
      let pkt ← match ← below 10 with
        | 0 | 1 => randomDatagram
        | 2 => pure base
        | _ => mutate base other
      let from_ ← if ← chance 750 then pure src else pure (addrOf (← below 6))
      let dt := (← below 6).toUInt64
      return (from_, pkt, dt)
    let ((src, pkt, dt), r) := step.run rng
    rng := r
    now := now + dt
    s := s.receive src pkt now
    t := { t with steps := t.steps + 1 }
    if let some v := invariants s then
      if t.violations.size < 5 then t := { t with violations := t.violations.push s!"{label} step {i} after receive: {v}" }
    if i % 4 == 0 then
      let (c, r) := randCmdWide.run rng
      rng := r
      s := s.submitLocal c now
      let (s', out) := s.poll now
      s := s'
      t := { t with polls := t.polls + 1, datagramsOut := t.datagramsOut + out.size
                    maxBatch := max t.maxBatch out.size }
      for (_, b) in out do
        t := { t with maxOut := max t.maxOut b.size }
        if b.size > MAX_PACKET then t := { t with violations := t.violations.push s!"{label}: {b.size} B datagram" }
        if decode b matches .invalid then
          t := { t with violations := t.violations.push s!"{label}: emitted undecodable datagram" }
      match s.nextTic? with
      | some rt =>
        s := s.advance.submitChecksum rt.tic (rt.tic * 2654435761)
        t := { t with ticsRun := t.ticsRun + 1 }
      | none => pure ()
      if let some v := invariants s then
        if t.violations.size < 5 then t := { t with violations := t.violations.push s!"{label} step {i} after poll: {v}" }
  return t

def main (args : List String) : IO UInt32 := do
  let scale := (args.head? >>= String.toNat?).getD 1
  let mut failed := false
  -- 1. decoders
  let mut rng := Rng.ofSeed 2026
  let mut tr : Tally := {}
  for _ in [0:300000 * scale] do
    let (b, r) := randomDatagram.run rng
    rng := r
    tr := classify tr b
  IO.println s!"random datagrams: {tr.total}, accepted {tr.ok} {tr.perType}, wrongVersion {tr.wrongVersion}, rejected {tr.invalid}, violations {tr.bad}"
  if tr.bad > 0 then failed := true
  let mut tm : Tally := {}
  for _ in [0:300000 * scale] do
    let step : R ByteArray := do
      let m ← randMsg
      let o ← randMsg
      mutate (encode (← randU32) m) (encode (← randU32) o)
    let (b, r) := step.run rng
    rng := r
    tm := classify tm b
  IO.println s!"mutated messages: {tm.total}, accepted {tm.ok} {tm.perType}, wrongVersion {tm.wrongVersion}, rejected {tm.invalid}, violations {tm.bad}"
  if tm.bad > 0 then failed := true
  let cap := capture
  let mut tc : Tally := {}
  for _ in [0:200000 * scale] do
    let step : R ByteArray := do
      let (_, _, b) := cap.corpus.getD (← below cap.corpus.size) (default, default, .empty)
      let (_, _, o) := cap.corpus.getD (← below cap.corpus.size) (default, default, .empty)
      mutate b o
    let (b, r) := step.run rng
    rng := r
    tc := classify tc b
  IO.println s!"mutated captured traffic ({cap.corpus.size} datagrams): {tc.total}, accepted {tc.ok} {tc.perType}, rejected {tc.invalid}, violations {tc.bad}"
  if tc.bad > 0 then failed := true
  -- 2. sessions
  let phases : List (String × Session) := [
    ("host lobby", cap.hostLobby), ("host running", cap.hostRunning),
    ("joiner joining", cap.joinJoining), ("joiner lobby", cap.joinLobby),
    ("joiner running", cap.joinRunning)]
  for (label, s) in phases do
    let wantRole := if label.startsWith "host" then Role.host else Role.joiner
    if s.phase == .ended || s.role != wantRole then
      IO.println s!"{label}: snapshot missing"
      failed := true
    let t := fuzzSession label s cap.corpus (100000 * scale) (label.length.toUInt64 * 77)
    IO.println s!"session {label} ({repr s.phase}): {t.steps} datagrams in, {t.polls} polls, {t.datagramsOut} out (≤{t.maxBatch}/poll, ≤{t.maxOut} B), {t.ticsRun} tics run, violations {t.violations.size}"
    for v in t.violations do IO.println s!"   ! {v}"
    if !t.violations.isEmpty then failed := true
  IO.println s!"net-fuzz: {if failed then "FAILED" else "all checks passed"}"
  return if failed then 1 else 0

end Doom.Net.Test.Fuzz

def main (args : List String) : IO UInt32 := Doom.Net.Test.Fuzz.main args
