import DoomTest.Net.Rng
/-!
# net-sim: deterministic network simulation of Doom.Net sessions

One host and 1–3 joiners exchange datagrams through a seeded simulated
network (loss, duplication, reordering, latency and jitter) on a 1 ms clock.
Each peer runs frames at its own period with a skewed local clock and drives
the session exactly as the game loop would. Checks per scenario:

* every peer delivers byte-identical tic sets for every tic it ran;
* each player's command in every set equals the command that player
  generated for that tic (clamped), so nothing is lost or misattributed;
* masks follow the drop table; a killed joiner is dropped at one tic that
  every survivor reports; survivors keep running;
* a killed host ends every joiner with `hostLost` within the timeout;
* an injected bad checksum is reported at exactly that tic and slot;
* progress (tics run versus wall-clock tics) and packet sizes.

Exit code 0 iff every scenario passed.
-/

namespace Doom.Net.Test.Sim

open Doom.Net
open Doom.Net.Test
open Doom.Playsim.Player (TicCmd)

structure NetParams where
  lossPm : Nat := 0
  dupPm : Nat := 0
  reorderPm : Nat := 0
  latMs : Nat := 1
  jitterMs : Nat := 0
  deriving Repr

structure Scenario where
  name : String
  players : Nat
  net : NetParams
  seed : UInt64
  durationMs : Nat := 20000
  /-- Peer index and simulated time at which it stops dead. -/
  kill : Option (Nat × Nat) := none
  /-- Peer index and simulated time at which it calls `leave`, flushes, and exits. -/
  quit : Option (Nat × Nat) := none
  /-- Peer index and tic offset at which it reports a wrong checksum. -/
  badSum : Option (Nat × Nat) := none
  /-- Every peer freezes this long right after the game starts (level load). -/
  loadMs : Nat := 0
  dropTimeoutMs : UInt64 := 3000
  inputDelay : Nat := 2
  maxLead : Nat := 6
  startTic : UInt32 := 0
  /-- Peer index forced to this frame period (ms), e.g. a slow renderer. -/
  slowPeer : Option (Nat × Nat) := none

structure Packet where
  due : Nat
  seq : Nat
  src : Addr
  dst : Nat
  data : ByteArray

structure SimPeer where
  s : Session
  addr : Addr
  alive : Bool := true
  period : Nat
  nextFrame : Nat
  /-- Clock skew in parts per 10000. -/
  skew : Int
  offset : Nat
  inbox : Array (Addr × ByteArray) := #[]
  pauseUntil : Nat := 0
  wasRunning : Bool := false
  /-- Clamped local commands by tic offset (prefill zeros included). -/
  gen : Array TicCmd := #[]
  sets : Array (UInt8 × Array TicCmd) := #[]
  hash : UInt64 := 0xcbf29ce484222325
  endedAt : Option Nat := none
  dropSeenAt : Option Nat := none
  sessionNs : Nat := 0
  maxFrameNs : Nat := 0
  leadSum : Nat := 0
  runFrames : Nat := 0

instance : Inhabited SimPeer := ⟨{ s := default, addr := default, period := 1, nextFrame := 0, skew := 0, offset := 0 }⟩

structure Report where
  name : String
  ok : Bool
  fails : Array String
  startMs : Nat
  minRan : Nat
  expected : Nat
  packets : Nat
  bytes : Nat
  maxPacket : Nat
  avgLead : Float
  nsPerTic : Float
  maxFrameUs : Float
  note : String

def baseIp : UInt32 := 0x0A000001
def addrOf (i : Nat) : Addr := ⟨baseIp + i.toUInt32, PORT⟩
def digest : ByteArray := ⟨#[0xF1, 0x2E, 0x3D, 0x4C, 0x5B, 0x6A]⟩
def MS_PER_TIC : Float := 1000.0 / 35.0

def localNow (p : SimPeer) (t : Nat) : UInt64 :=
  let scaled : Int := (Int.ofNat t * (10000 + p.skew)) / 10000
  (Int.toNat (Int.ofNat p.offset + scaled)).toUInt64

def fnvByte (h : UInt64) (b : UInt8) : UInt64 := (h ^^^ b.toUInt64) * 0x100000001b3

def hashSet (h0 : UInt64) (m : UInt8) (cmds : Array TicCmd) : UInt64 := Id.run do
  let mut h := fnvByte h0 m
  for c in cmds do
    for b in (packCmd c).toList do h := fnvByte h b
  return h

def sendAll (net : NetParams) (rng0 : Rng) (inflight0 : Array Packet) (seq0 : Nat) (src : Addr)
    (t : Nat) (out : Array (Addr × ByteArray)) : Rng × Array Packet × Nat := Id.run do
  let mut rng := rng0
  let mut inflight := inflight0
  let mut seq := seq0
  for (dst, b) in out do
    let di := (dst.ip - baseIp).toNat
    let step : R (Array Nat) := do
      if ← chance net.lossPm then return #[]
      let mut ds := #[]
      let copies := if ← chance net.dupPm then 2 else 1
      for _ in [0:copies] do
        let mut d := net.latMs + (← below (net.jitterMs + 1))
        if ← chance net.reorderPm then d := d + (← below (3 * net.jitterMs + 20))
        ds := ds.push d
      return ds
    let (ds, r) := step.run rng
    rng := r
    for d in ds do
      inflight := inflight.push { due := t + d, seq, src, dst := di, data := b }
      seq := seq + 1
  return (rng, inflight, seq)

def runScenario (sc : Scenario) : IO Report := do
  let mut rng := Rng.ofSeed sc.seed
  let mut peers : Array SimPeer := #[]
  for i in [0:sc.players] do
    let cfg : Config := { name := s!"p{i}", iwadDigest := digest
                          nonce := (sc.seed.toUInt32 * 7919 + i.toUInt32 * 104729 + 1)
                          inputDelay := sc.inputDelay, maxLead := sc.maxLead
                          dropTimeoutMs := sc.dropTimeoutMs }
    let s := if i == 0 then Session.host cfg else Session.join cfg (addrOf 0)
    let gen : R (Nat × Int × Nat × Nat) := do
      return (← range 10 20, ← randInt (-50) 50, ← below 100000, ← below 30)
    let ((period0, skew, offset, first), r) := gen.run rng
    rng := r
    let period := match sc.slowPeer with
      | some (si, sp) => if si == i then sp else period0
      | none => period0
    peers := peers.push { s, addr := addrOf i, period, nextFrame := first + i * 5, skew, offset }
  let mut inflight : Array Packet := #[]
  let mut seq := 0
  let mut startMs : Option Nat := none
  let settings : GameSettings := { map := 1, skill := 3, deathmatch := 1, startTic := sc.startTic }
  for t in [0:sc.durationMs] do
    if let some (k, tk) := sc.kill then
      if tk == t then peers := peers.modify k fun p => { p with alive := false }
    if let some (k, tk) := sc.quit then
      if tk == t then
        let p := peers.getD k default
        let (s, out) := p.s.leave.poll (localNow p t)
        peers := peers.setIfInBounds k { p with s, alive := false }
        let (r2, inf2, seq2) := sendAll sc.net rng inflight seq p.addr t out
        rng := r2
        inflight := inf2
        seq := seq2
    -- deliver due datagrams into socket buffers
    if inflight.any (·.due ≤ t) then
      let due := (inflight.filter (·.due ≤ t)).qsort fun a b => a.due < b.due || (a.due == b.due && a.seq < b.seq)
      inflight := inflight.filter (·.due > t)
      for pk in due do
        peers := peers.modify pk.dst fun p =>
          if p.alive && p.inbox.size < 512 then { p with inbox := p.inbox.push (pk.src, pk.data) } else p
    -- frames
    for i in [0:peers.size] do
      let p0 := peers.getD i (peers.getD 0 default)
      if !p0.alive || p0.nextFrame > t || p0.pauseUntil > t then continue
      let now := localNow p0 t
      -- take the peer out of the array and the session out of the peer: an
      -- unshared session keeps every ring update in place (as in the game loop)
      peers := peers.setIfInBounds i default
      let inbox := p0.inbox
      let mut s := p0.s
      let mut p := { p0 with s := default, inbox := #[] }
      let t0 ← IO.monoNanosNow
      for (src, b) in inbox do s := s.receive src b now
      if s.role == .host && s.phase == .lobby && s.playerCount == sc.players then
        match s.start settings now with
        | .ok s' =>
          s := s'
          startMs := some t
        | .error _ => pure ()
      let justStarted := s.phase == .running && !p.wasRunning
      if justStarted then
        let st := (s.info.map (·.settings.startTic)).getD 0
        p := { p with wasRunning := true, gen := Array.replicate (s.gen - st).toNat TicCmd.zero }
      if justStarted && sc.loadMs > 0 then
        -- integration rule: poll once, then block for the level load
        let (s', out) := s.poll now
        let t1 ← IO.monoNanosNow
        let p' := { p with
          s := s', pauseUntil := t + sc.loadMs, nextFrame := t + sc.loadMs
          sessionNs := p.sessionNs + (t1 - t0) }
        peers := peers.setIfInBounds i p'
        let (r2, inf2, seq2) := sendAll sc.net rng inflight seq p.addr t out
        rng := r2
        inflight := inf2
        seq := seq2
        continue
      -- local commands
      let mut k := 0
      let mut cmdsMade : Array TicCmd := #[]
      while k < 8 && s.localDue now do
        let (c, r) := randCmdWide.run rng
        rng := r
        cmdsMade := cmdsMade.push (clampCmd c)
        s := s.submitLocal c now
        k := k + 1
      let (s', out) := s.poll now
      s := s'
      let mut ready : Array ReadyTic := #[]
      let mut n := 0
      while n < 16 do
        match s.nextTic? with
        | some rt =>
          ready := ready.push rt
          s := s.advance
          n := n + 1
        | none => break
      let t1 ← IO.monoNanosNow
      -- game-side bookkeeping (untimed)
      p := { p with gen := p.gen ++ cmdsMade }
      let mut sums : Array (UInt32 × UInt32) := #[]
      for rt in ready do
        let h := hashSet p.hash rt.ingame rt.cmds
        p := { p with hash := h, sets := p.sets.push (rt.ingame, rt.cmds) }
        let st := (s.info.map (·.settings.startTic)).getD 0
        let bad := match sc.badSum with
          | some (bi, bt) => bi == i && rt.tic == st + bt.toUInt32
          | none => false
        sums := sums.push (rt.tic, if bad then h.toUInt32 ^^^ 0xDEADBEEF else h.toUInt32)
      let t2 ← IO.monoNanosNow
      for (tic, sum) in sums do s := s.submitChecksum tic sum
      let t3 ← IO.monoNanosNow
      let dt := (t1 - t0) + (t3 - t2)
      if s.phase == .running then
        p := { p with leadSum := p.leadSum + s.pacing.localLead, runFrames := p.runFrames + 1 }
      if s.phase == .ended && p.endedAt.isNone then p := { p with endedAt := some t }
      if s.drops.size > 0 && p.dropSeenAt.isNone then p := { p with dropSeenAt := some t }
      let (jit, r) := (below 4).run rng
      rng := r
      p := { p with s, sessionNs := p.sessionNs + dt, maxFrameNs := max p.maxFrameNs dt
                    nextFrame := t + p.period + jit }
      peers := peers.setIfInBounds i p
      let (r2, inf2, seq2) := sendAll sc.net rng inflight seq p.addr t out
      rng := r2
      inflight := inf2
      seq := seq2
  -- ---------------------------------------------------------------- checks
  let mut fails : Array String := #[]
  let st := startMs.getD sc.durationMs
  if startMs.isNone then fails := fails.push "game never started"
  let host := peers.getD 0 default
  -- reference log: the longest
  let mut ref : Array (UInt8 × Array TicCmd) := #[]
  for p in peers do if p.sets.size > ref.size then ref := p.sets
  for i in [0:peers.size] do
    let p := peers.getD i default
    let mut bad := 0
    for k in [0:p.sets.size] do
      if p.sets[k]? != ref[k]? then bad := bad + 1
    if bad > 0 then fails := fails.push s!"peer {i}: {bad} tic sets differ from the reference"
  -- owner of each player number
  let mut owner : Array Nat := #[0, 0, 0, 0]
  for i in [0:peers.size] do
    let p := peers.getD i default
    if p.wasRunning then owner := owner.setIfInBounds p.s.consoleplayer i
  let startMask := (host.s.info.map (·.playerMask)).getD 0
  let drops := if host.alive then host.s.drops else
    ((peers.toList.drop 1).find? (·.alive) |>.map (·.s.drops)).getD #[]
  let mut cmdBad := 0
  let mut maskBad := 0
  for k in [0:ref.size] do
    let (m, cmds) := ref.getD k (0, #[])
    let mut want := startMask
    for d in drops do
      if d.tic.toNat ≤ sc.startTic.toNat + k then want := want &&& ~~~((1 : UInt8) <<< d.slot)
    if m != want then maskBad := maskBad + 1
    for pl in [0:4] do
      let c := cmds.getD pl TicCmd.zero
      if bit m pl then
        let o := peers.getD (owner.getD pl 0) default
        if o.gen[k]? != some c then cmdBad := cmdBad + 1
      else if c != TicCmd.zero then cmdBad := cmdBad + 1
  if cmdBad > 0 then fails := fails.push s!"{cmdBad} commands differ from what their player generated"
  if maskBad > 0 then fails := fails.push s!"{maskBad} tic masks disagree with the drop table"
  -- progress
  let expected := ((sc.durationMs - min st sc.durationMs).toFloat / MS_PER_TIC).toUInt64.toNat
  let mut minRan := ref.size
  for p in peers do
    if p.alive && p.s.phase != .ended then minRan := min minRan p.sets.size
  let mut note := ""
  -- scenario-specific
  let departure : Option (Nat × Nat × Bool) := match sc.kill, sc.quit with
    | some (k, t), _ => some (k, t, false)
    | none, some (k, t) => some (k, t, true)
    | none, none => none
  match departure with
  | some (0, tk, graceful) =>
    let want := if graceful then EndReason.hostQuit else .hostLost
    let bound := if graceful then 300 else sc.dropTimeoutMs.toNat + 300
    for i in [1:peers.size] do
      let p := peers.getD i default
      match p.s.endReason, p.endedAt with
      | some r, some te =>
        if r != want then fails := fails.push s!"peer {i}: expected {repr want}, got {repr r}"
        if te < tk || te - tk > bound then
          fails := fails.push s!"peer {i}: host departure detected after {te - tk} ms"
      | r, _ => fails := fails.push s!"peer {i}: expected {repr want}, got {repr r}"
    let lat := (peers.toList.drop 1).map fun p => (p.endedAt.getD 0) - tk
    note := s!"host {if graceful then "quit" else "killed"} at {tk} ms; joiners ended {repr want} after {lat} ms"
  | some (k, tk, graceful) =>
    let victim := (peers.getD k default).s.consoleplayer
    let wantReason := if graceful then DropReason.quit else .timeout
    match host.s.drops.toList with
    | [d] =>
      if d.slot.toNat != victim || d.reason != wantReason then
        fails := fails.push s!"host drop entry wrong: {repr d}"
      for i in [1:peers.size] do
        let p := peers.getD i default
        if p.alive && !(p.s.drops.any fun e => e.slot == d.slot && e.tic == d.tic) then
          fails := fails.push s!"peer {i} did not report drop of {d.slot} at {d.tic}"
      let T := (d.tic - sc.startTic).toNat
      let before := (ref.getD (T - 1) (0, #[])).1
      let after := (ref.getD T (0, #[])).1
      if T == 0 || !bit before victim || bit after victim then
        fails := fails.push s!"mask did not change exactly at drop tic {d.tic}"
      let seen := host.dropSeenAt.getD 0
      if graceful then
        if seen < tk || seen - tk > 300 then
          fails := fails.push s!"graceful leave noticed {seen - tk} ms after quitting"
      -- local clocks are skewed by up to 0.5%
      else if seen + sc.dropTimeoutMs.toNat / 100 < tk + sc.dropTimeoutMs.toNat then
        fails := fails.push s!"dropped {seen - tk} ms after the kill, before the timeout"
      let afterExpected := ((sc.durationMs - seen).toFloat / MS_PER_TIC).toUInt64.toNat
      if minRan < T + afterExpected / 2 then
        fails := fails.push s!"survivors stalled after the drop (ran {minRan}, drop tic {T})"
      note := s!"peer {k} {if graceful then "quit" else "killed"} at {tk} ms; dropped slot {d.slot} ({repr d.reason}) effective tic {d.tic} (host noticed at {seen} ms); all survivors agree"
    | ds => fails := fails.push s!"host drop table: {repr ds}"
  | none => pure ()
  match sc.badSum with
  | some (bi, bt) =>
    let slot := (peers.getD bi default).s.consoleplayer
    let want : Option Desync := some { tic := sc.startTic + bt.toUInt32, slots := (1 : UInt8) <<< slot.toUInt8 }
    for i in [0:peers.size] do
      let p := peers.getD i default
      if p.s.desync != want then fails := fails.push s!"peer {i}: desync {repr p.s.desync}, want {repr want}"
    note := s!"wrong checksum injected by peer {bi} at tic {bt}: all peers report desync at tic {bt}, slot mask {(1 : Nat) <<< slot}"
  | none =>
    for i in [0:peers.size] do
      if (peers.getD i default).s.desync.isSome then fails := fails.push s!"peer {i}: spurious desync"
  if departure.isNone then
    for i in [0:peers.size] do
      let p := peers.getD i default
      if p.s.phase != .running then fails := fails.push s!"peer {i} not running at end: {repr p.s.endReason}"
  if departure.map (·.1) != some 0 && minRan * 10 < expected * 6 then
    fails := fails.push s!"too little progress: {minRan} of {expected} tics"
  let mut packets := 0
  let mut bytes := 0
  let mut maxPacket := 0
  let mut ns := 0
  let mut tics := 0
  let mut leadSum := 0
  let mut runFrames := 0
  let mut maxFrame := 0
  for p in peers do
    packets := packets + p.s.stats.txPackets
    bytes := bytes + p.s.stats.txBytes
    maxPacket := max maxPacket p.s.stats.maxPacket
    ns := ns + p.sessionNs
    tics := tics + p.sets.size
    leadSum := leadSum + p.leadSum
    runFrames := runFrames + p.runFrames
    maxFrame := max maxFrame p.maxFrameNs
  if maxPacket > MAX_PACKET then fails := fails.push s!"packet of {maxPacket} bytes"
  return { name := sc.name, ok := fails.isEmpty, fails, startMs := st, minRan, expected, packets, bytes
           maxPacket, avgLead := leadSum.toFloat / (max runFrames 1).toFloat
           nsPerTic := ns.toFloat / (max tics 1).toFloat, maxFrameUs := maxFrame.toFloat / 1000.0, note }

def lan : NetParams := { lossPm := 0, dupPm := 0, reorderPm := 0, latMs := 1, jitterMs := 1 }
def lossy (loss : Nat) : NetParams := { lossPm := loss, dupPm := 50, reorderPm := 100, latMs := 2, jitterMs := 12 }
def wan (loss : Nat) : NetParams := { lossPm := loss, dupPm := 50, reorderPm := 100, latMs := 40, jitterMs := 30 }

def scenarios : Array Scenario := Id.run do
  let mut out := #[]
  for n in [2, 3, 4] do
    for seed in [1, 2, 3] do
      out := out.push { name := s!"{n}p lan s{seed}", players := n, net := lan, seed := seed.toUInt64 }
      for loss in [50, 100, 200] do
        out := out.push { name := s!"{n}p loss{loss / 10}% dup5% reord10% s{seed}", players := n
                          net := lossy loss, seed := (seed * 100 + loss).toUInt64 }
      out := out.push { name := s!"{n}p wan40±30ms loss10% s{seed}", players := n, net := wan 100
                        seed := (seed * 1000 + 7).toUInt64, inputDelay := 3 }
      out := out.push { name := s!"{n}p wan40±30ms loss10% maxLead12 s{seed}", players := n
                        net := wan 100, seed := (seed * 1000 + 7).toUInt64, inputDelay := 4
                        maxLead := 12 }
    out := out.push { name := s!"{n}p kill joiner loss10%", players := n, net := lossy 100
                      seed := (n * 31).toUInt64, kill := some (n - 1, 8000) }
    out := out.push { name := s!"{n}p kill host loss10%", players := n, net := lossy 100
                      seed := (n * 37).toUInt64, kill := some (0, 8000), durationMs := 14000 }
    out := out.push { name := s!"{n}p joiner quits loss5%", players := n, net := lossy 50
                      seed := (n * 43).toUInt64, quit := some (n - 1, 7000) }
    out := out.push { name := s!"{n}p host quits loss5%", players := n, net := lossy 50
                      seed := (n * 47).toUInt64, quit := some (0, 7000), durationMs := 9000 }
    out := out.push { name := s!"{n}p desync", players := n, net := lossy 50, seed := (n * 41).toUInt64
                      badSum := some (n - 1, 300) }
  out := out.push { name := "4p kill joiner 1 of 3, loss20%", players := 4, net := lossy 200, seed := 99
                    kill := some (1, 9000), durationMs := 22000 }
  out := out.push { name := "4p level load 2s", players := 4, net := lossy 50, seed := 77, loadMs := 2000 }
  out := out.push { name := "3p level load 6s > 3s drop timeout, loss10%", players := 3, net := lossy 100
                    seed := 78, loadMs := 6000, durationMs := 24000 }
  out := out.push { name := "3p default 8s timeout kill", players := 3, net := lossy 50, seed := 55
                    kill := some (2, 5000), dropTimeoutMs := 8000, durationMs := 25000 }
  out := out.push { name := "2p startTic 1000 delay 0", players := 2, net := lossy 100, seed := 66
                    startTic := 1000, inputDelay := 0 }
  out := out.push { name := "3p host renders at 22 fps", players := 3, net := lossy 50, seed := 88
                    slowPeer := some (0, 45) }
  out := out.push { name := "4p joiner renders at 25 fps, loss10%", players := 4, net := lossy 100
                    seed := 89, slowPeer := some (2, 40) }
  out := out.push { name := "4p long run 3 min loss10%", players := 4, net := lossy 100, seed := 90
                    durationMs := 180000 }
  return out

def fmt (x : Float) (d : Nat := 1) : String :=
  let p := (10 : Float) ^ d.toFloat
  toString ((x * p).round / p)

def main (args : List String) : IO UInt32 := do
  let filter := args.head?
  let mut failed := 0
  let mut count := 0
  let mut nsTotal := 0.0
  let mut maxNs := 0.0
  for sc in scenarios do
    if let some f := filter then
      if !(sc.name.splitOn f).length > 1 then continue
    let r ← runScenario sc
    count := count + 1
    nsTotal := nsTotal + r.nsPerTic
    maxNs := max maxNs r.nsPerTic
    let status := if r.ok then "PASS" else "FAIL"
    IO.println s!"{status} {r.name}: start {r.startMs} ms, ran ≥{r.minRan}/{r.expected} tics ({fmt (100 * r.minRan.toFloat / (max r.expected 1).toFloat)}%), {r.packets} pkts {r.bytes / 1024} KiB, max {r.maxPacket} B, lead {fmt r.avgLead 2}, {fmt (r.nsPerTic / 1000) 2} µs/tic, worst frame {fmt r.maxFrameUs 0} µs"
    if r.note != "" then IO.println s!"     {r.note}"
    for f in r.fails do IO.println s!"     ! {f}"
    unless r.ok do failed := failed + 1
  IO.println s!"net-sim: {count - failed}/{count} scenarios passed; mean session cost {fmt (nsTotal / (max count 1).toFloat / 1000) 2} µs per peer-tic (worst scenario {fmt (maxNs / 1000) 2})"
  return if failed == 0 then 0 else 1

end Doom.Net.Test.Sim

def main (args : List String) : IO UInt32 := Doom.Net.Test.Sim.main args
