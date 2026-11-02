import Doom.Net.Wire

/-!
# Doom.Net.Session

Pure lockstep session for a star of one host and up to three joiners. The
platform layer owns the UDP socket and the clock: it feeds datagrams to
`receive`, sends whatever `poll` returns, and passes milliseconds explicitly.

## Model

* **Lobby.** Joiners repeat HELLO (`helloMs`) until START; the host answers
  each with a per-recipient LOBBY (their slot and the roster) or a REJECT
  (version / IWAD / full / started). Lobby members silent for `joinTimeoutMs`
  are removed. `start` compacts the occupied slots to players `0..n-1` (host is
  player 0) and repeats START to each joiner until its first CMDS arrives.
* **Commands.** Each peer owns a 35 Hz tic clock (`localDue`). Local command
  `g` is accepted only while `g < runNext + maxLead`; the first `inputDelay`
  commands are zero and queued automatically at start, so in steady state
  the command made while running tic `t` is for tic `t + inputDelay`. A
  joiner sends every not-yet-acknowledged command in each CMDS packet.
* **Sets.** The host produces the merged set for tic `t` as soon as it holds
  tic `t` from every player in the game at `t`, then sends each joiner all
  sets from that joiner's acknowledgement onwards. A peer may run `t` only
  once it holds set `t`; the host runs from the same ring, so every peer runs
  byte-identical sets.
* **Pacing.** Each SETS tells the recipient how many tics its command stream
  is ahead of the slowest player's (as seen by the host). When that is ≥ 2,
  the local tic clock is delayed by 2 ms per excess tic per new SETS; the
  slowest player is never delayed, so the group runs at the slowest clock
  with minimal lead. Window stalls drop missed clock slots instead of bursting.
* **Drops.** A player silent for `dropTimeoutMs` (or sending LEAVE) is removed
  effective at the host's next unproduced tic `T`: sets `< T` contain it,
  sets `≥ T` do not, and every set carries the in-game mask, so all peers
  clear `playeringame` at exactly `T`. The drop table (slot, `T`, reason) is
  repeated in every SETS. Joiners treat host silence, host LEAVE or their own
  absence from a set as the end of the game.
* **Checksums.** Peers submit a 32-bit state checksum per tic in order; joiners
  send unacknowledged ones redundantly. The host compares tic `c` once every
  in-game player reported it and records the first differing tic and slots.
  Either every peer submits checksums or none does.
* **Level loads.** Until the first tic with real (non-prefill) commands from
  everyone, silence is judged against `loadTimeoutMs` instead of
  `dropTimeoutMs`; the first START to each joiner is sent twice.

Every buffer is a fixed `RING`-tic ring (`ByteArray`); nothing grows with time.

## Integration (once per frame, `now` = monotonic ms)

1. `s := s.receive src bytes now` for every datagram read from UDP port 2342.
2. Host lobby: `s.start settings now` when the user chooses. Any peer whose
   `phase` just became `running`: read `s.info` (settings, `playerMask`,
   `consoleplayer`), call `poll` once and send, then load the level.
3. `while s.localDue now do s := s.submitLocal (buildTiccmd input) now`.
4. `let (s', out) := s.poll now`; send each `(addr, bytes)`.
5. `while let some rt := s.nextTic? do` set `playeringame[p] := bit rt.ingame p`
   (a player leaving the mask has left the game at `rt.tic`), copy
   `rt.cmds[p]` into each player, run one game tic, then
   `s := s.advance.submitChecksum rt.tic (stateChecksum)`. Cap the tics run
   per frame if needed; leftovers stay ready.
6. Render. When no tic was ready, keep showing the last frame (do not block).
   `phase = ended` ends the game with `endReason`; `drops`/`desync` are
   sticky event records.

Thread the session linearly (never keep an older copy alive), so ring updates
stay in place instead of copying the buffers.
-/

namespace Doom.Net

open Doom.Playsim.Player (TicCmd)

inductive Role where
  | host
  | joiner
  deriving BEq, DecidableEq, Repr, Inhabited

inductive Phase where
  /-- Joiner: waiting for the host's first LOBBY. -/
  | joining
  | lobby
  | running
  | ended
  deriving BEq, DecidableEq, Repr, Inhabited

def ST_EMPTY : UInt8 := 0
def ST_LOBBY : UInt8 := 1
def ST_GAME : UInt8 := 2
def ST_DROPPED : UInt8 := 3

def NEVER : UInt64 := 0xFFFFFFFFFFFFFFFF

/-- `interval` has elapsed since `last` (or nothing was ever sent). -/
@[inline] def isDue (now last interval : UInt64) : Bool := last == NEVER || now ≥ last + interval

/-- Silent for longer than `timeout` since `heard`. -/
@[inline] def isSilent (now heard timeout : UInt64) : Bool := heard != NEVER && now > heard + timeout

/-- `a - b`, or 0 when `b > a`. -/
@[inline] def subSat (a b : UInt32) : UInt32 := if a ≥ b then a - b else 0

@[inline] def ringIdx (t : UInt32) : Nat := (t &&& RING_MASK).toNat
/-- One stored set: mask byte, then 4 packed commands. -/
def SET_STRIDE : Nat := 1 + MAX_PLAYERS * CMD_BYTES
@[inline] def setOff (t : UInt32) : Nat := ringIdx t * SET_STRIDE
@[inline] def cmdOff (p : Nat) (t : UInt32) : Nat := (p * RING + ringIdx t) * CMD_BYTES
@[inline] def sumOff (p : Nat) (t : UInt32) : Nat := (p * RING + ringIdx t) * 4

/-- Per-slot record. Host: the connection to that player. Joiner: roster data only. -/
structure Peer where
  state : UInt8 := ST_EMPTY
  addr : Addr := default
  nonce : UInt32 := 0
  name : String := ""
  lastHeard : UInt64 := NEVER
  lastSent : UInt64 := NEVER
  startAcked : Bool := false
  /-- Host holds this player's commands for every tic below. -/
  cmdNext : UInt32 := 0
  /-- This player holds every set below. -/
  setAck : UInt32 := 0
  /-- Host holds this player's checksums for every tic below. -/
  sumNext : UInt32 := 0
  sentSetEnd : UInt32 := 0
  sentCmdAck : UInt32 := 0
  sentSumAck : UInt32 := 0
  /-- Send to this peer at the next poll. -/
  dirty : Bool := false
  deriving Inhabited

structure Stats where
  rxPackets : Nat := 0
  rxInvalid : Nat := 0
  txPackets : Nat := 0
  txBytes : Nat := 0
  maxPacket : Nat := 0
  rejectsQueued : Nat := 0
  deriving Repr, Inhabited

structure Session where
  role : Role
  cfg : Config
  phase : Phase
  endReason : Option EndReason := none
  /-- Host-chosen id carried by every post-lobby datagram (joiner: 0 until learned). -/
  session : UInt32
  /-- Joiner: the host's address. -/
  hostAddr : Addr
  /-- Local slot: lobby slot, then consoleplayer once running. -/
  slot : UInt8 := 0
  peers : Array Peer
  info : Option StartInfo := none
  startMask : UInt8 := 0
  inputDelay : UInt32
  maxLead : UInt32
  /-- Next local tic to generate a command for. -/
  gen : UInt32 := 0
  /-- When the next local command is due, in units of 1/35 ms. -/
  nextDue35 : UInt64 := 0
  /-- Sets below this tic are known (host: produced). -/
  setNext : UInt32 := 0
  /-- Sets below this tic were handed to the game. -/
  runNext : UInt32 := 0
  setRing : ByteArray
  /-- Host: incoming commands per player. Joiner: unacknowledged local commands (region 0). -/
  cmdRing : ByteArray
  /-- Joiner: the host holds our commands below this tic. -/
  cmdAck : UInt32 := 0
  /-- Host: checksums per player. Joiner: local checksums (region 0). -/
  sumRing : ByteArray
  /-- Local checksums below this tic were submitted. -/
  sumNext : UInt32 := 0
  /-- Joiner: the host holds our checksums below this tic. -/
  sumAck : UInt32 := 0
  /-- Host: checksums for every tic below were compared. -/
  sumCompared : UInt32 := 0
  desync : Option Desync := none
  drops : Array DropInfo := #[]
  /-- Our command stream minus the slowest player's, per the host. -/
  ahead : Int32 := 0
  lastHeard : UInt64 := NEVER
  lastSent : UInt64 := NEVER
  firstPoll : UInt64 := NEVER
  sentGen : UInt32 := 0
  sentSetNext : UInt32 := 0
  outbox : Array (Addr × ByteArray) := #[]
  stats : Stats := {}

/-- One tic ready to run: the in-game mask and every player's command (zero if absent). -/
structure ReadyTic where
  tic : UInt32
  ingame : UInt8
  cmds : Array TicCmd

structure LobbyEntry where
  slot : Nat
  name : String
  isLocal : Bool
  isHost : Bool
  deriving Repr, BEq

structure Pacing where
  /-- Sets held but not yet run; > 0 means this peer is behind and should run them now. -/
  readyTics : Nat
  /-- Local commands generated but not yet run. -/
  localLead : Nat
  /-- This peer's command stream minus the slowest player's, per the host. -/
  aheadTics : Int
  /-- The lead window has room for another local command. -/
  windowOpen : Bool
  deriving Repr

namespace Session

private def normalize (cfg : Config) : Config :=
  let delay := min cfg.inputDelay MAX_INPUT_DELAY
  { cfg with
    name := sanitizeName cfg.name
    iwadDigest := cfg.iwadDigest.extract 0 DIGEST_MAX
    inputDelay := delay
    maxLead := max (delay + 2) (min cfg.maxLead MAX_LEAD) }

private def blank (role : Role) (cfg0 : Config) (phase : Phase) (host : Addr) : Session :=
  let cfg := normalize cfg0
  { role, cfg, phase, session := 0, hostAddr := host
    peers := #[{}, {}, {}, {}]
    inputDelay := cfg.inputDelay.toUInt32, maxLead := cfg.maxLead.toUInt32
    setRing := zeroBytes (RING * SET_STRIDE)
    cmdRing := zeroBytes (MAX_PLAYERS * RING * CMD_BYTES)
    sumRing := zeroBytes (MAX_PLAYERS * RING * 4) }

/-- A host lobby with the local player in slot 0. `cfg.nonce` is the session id. -/
def host (cfg : Config) : Session :=
  let s := blank .host cfg .lobby default
  { s with
    session := if cfg.nonce == 0 then 1 else cfg.nonce
    peers := s.peers.setIfInBounds 0 { state := ST_LOBBY, name := s.cfg.name, nonce := cfg.nonce } }

/-- A joiner that will HELLO `hostAddr` from the first `poll`. -/
def join (cfg : Config) (hostAddr : Addr) : Session :=
  blank .joiner cfg .joining hostAddr

@[inline] def peer (s : Session) (p : Nat) : Peer := s.peers.getD p default

@[inline] def updPeer (s : Session) (p : Nat) (f : Peer → Peer) : Session :=
  { s with peers := s.peers.modify p f }

private def markAllDirty (s0 : Session) : Session := Id.run do
  let mut s := s0
  for p in [1:MAX_PLAYERS] do
    s := s.updPeer p fun q => { q with dirty := true }
  return s

private def queue (s : Session) (to : Addr) (b : ByteArray) : Session :=
  if s.outbox.size < OUTBOX_MAX then { s with outbox := s.outbox.push (to, b) } else s

private def finish (s : Session) (r : EndReason) : Session :=
  { s with phase := .ended, endReason := some r }

/-- In-game mask at tic `t`: the start mask minus players dropped at or before `t`. -/
def maskAt (s : Session) (t : UInt32) : UInt8 := Id.run do
  let mut m := s.startMask
  for d in s.drops do
    if d.tic ≤ t then m := m &&& ~~~((1 : UInt8) <<< d.slot)
  return m

/-- Occupied lobby slots and their names (host lobby). -/
private def roster (s : Session) (st : UInt8) : UInt8 × Array String := Id.run do
  let mut mask : UInt8 := 0
  let mut names : Array String := #["", "", "", ""]
  for p in [0:MAX_PLAYERS] do
    let q := s.peer p
    if q.state == st then
      mask := mask ||| ((1 : UInt8) <<< p.toUInt8)
      names := names.setIfInBounds p q.name
  return (mask, names)

/-! ## Local commands -/

/-- Room in the lead window (and in the local command ring) for another command. -/
def windowOpen (s : Session) : Bool :=
  s.phase == .running && subSat s.gen s.runNext < s.maxLead &&
    (match s.role with
     | .host => subSat s.gen s.setNext < RING32
     | .joiner => subSat s.gen s.cmdAck < RING32)

/-- A local command is due now and would be accepted by `submitLocal`. -/
def localDue (s : Session) (nowMs : UInt64) : Bool :=
  s.windowOpen && nowMs * 35 ≥ s.nextDue35

/-- Tic number the next accepted local command will be scheduled for. -/
def nextLocalTic (s : Session) : UInt32 := s.gen

/-! ## Host: producing sets and comparing checksums -/

/-- Host: produce every set whose commands are all present. -/
def produce (s0 : Session) : Session := Id.run do
  if s0.role != .host || s0.phase != .running then return s0
  let mut s := s0
  let mut ring := s.setRing
  s := { s with setRing := .empty }
  let mut made := 0
  for _ in [0:RING] do
    let t := s.setNext
    let m := s.maskAt t
    let mut retain := s.runNext
    let mut ready := true
    for p in [0:MAX_PLAYERS] do
      let q := s.peer p
      if p != 0 && q.state == ST_GAME && q.setAck < retain then retain := q.setAck
      if bit m p && q.cmdNext ≤ t then ready := false
    if !ready || subSat t retain ≥ RING32 then break
    let base := setOff t
    ring := put8 ring base m
    for p in [0:MAX_PLAYERS] do
      if bit m p then ring := copyBytes ring (base + 1 + p * CMD_BYTES) s.cmdRing (cmdOff p t) CMD_BYTES
      else ring := zeroRange ring (base + 1 + p * CMD_BYTES) CMD_BYTES
    s := { s with setNext := t + 1 }
    made := made + 1
  s := { s with setRing := ring }
  -- pacing hint for the host's own clock
  let mut minNext := s.gen
  for p in [1:MAX_PLAYERS] do
    let q := s.peer p
    if q.state == ST_GAME && q.cmdNext < minNext then minNext := q.cmdNext
  let ah := subSat s.gen minNext
  s := { s with ahead := (min ah 127).toInt32 }
  if made > 0 && ah ≥ 2 then
    s := { s with nextDue35 := s.nextDue35 + 70 * ((min ah 8) - 1).toUInt64 }
  return s

/-- Host: compare checksums for every tic all in-game players have reported. -/
def compareSums (s0 : Session) : Session := Id.run do
  if s0.role != .host || s0.phase != .running then return s0
  let mut s := s0
  for _ in [0:RING] do
    let c := s.sumCompared
    if c ≥ s.sumNext then break
    let m := s.maskAt c
    let hostSum := rd32 s.sumRing (sumOff 0 c)
    let mut wait := false
    let mut diff : UInt8 := 0
    for p in [1:MAX_PLAYERS] do
      if bit m p then
        let q := s.peer p
        if q.sumNext > c then
          if rd32 s.sumRing (sumOff p c) != hostSum then diff := diff ||| ((1 : UInt8) <<< p.toUInt8)
        else if q.state == ST_GAME then wait := true
    if wait then break
    if diff != 0 && s.desync.isNone then
      s := markAllDirty { s with desync := some { tic := c, slots := diff } }
    s := { s with sumCompared := c + 1 }
  return s

/-- Host: remove player `p` from the game, effective at the next unproduced tic. -/
def dropPlayer (s : Session) (p : Nat) (r : DropReason) : Session :=
  if s.role != .host || s.phase != .running || p == 0 || (s.peer p).state != ST_GAME then s
  else
    let s := s.updPeer p fun q => { q with state := ST_DROPPED }
    let s := { s with drops := s.drops.push { slot := p.toUInt8, tic := s.setNext, reason := r } }
    (markAllDirty s).produce.compareSums

/-- Queue `inputDelay` zero commands so the first real command lands `inputDelay` tics out. -/
private def prefill (s0 : Session) : Session := Id.run do
  let mut s := s0
  let mut ring := s.cmdRing
  s := { s with cmdRing := .empty }
  for _ in [0:s.inputDelay.toNat] do
    ring := putCmd ring (cmdOff 0 s.gen) TicCmd.zero
    s := { s with gen := s.gen + 1 }
  s := { s with cmdRing := ring }
  if s.role == .host then
    let g := s.gen
    s := s.updPeer 0 fun q => { q with cmdNext := g }
  return s

/--
Accept a local command for tic `nextLocalTic` if `localDue nowMs`; otherwise
return the session unchanged. Commands are clamped to the wire ranges.
-/
def submitLocal (s0 : Session) (cmd : TicCmd) (nowMs : UInt64) : Session :=
  if !s0.localDue nowMs then s0
  else
    let now35 := nowMs * 35
    -- After a stall, catch up at most a window's worth of missed tics (the
    -- window caps outstanding commands anyway); older slots are dropped.
    let catchUp := (s0.cfg.maxLead.max 4 - 1).toUInt64 * 1000
    let base := if now35 > s0.nextDue35 + catchUp then now35 - catchUp else s0.nextDue35
    let ring := s0.cmdRing
    let s := { s0 with cmdRing := .empty }
    let off := cmdOff 0 s.gen
    let g := s.gen + 1
    let s := { s with cmdRing := putCmd ring off cmd, gen := g, nextDue35 := base + 1000 }
    match s.role with
    | .host => (s.updPeer 0 fun q => { q with cmdNext := g }).produce
    | .joiner => s

/-! ## Running tics -/

/-- The next tic set to run, if every command for it is known. -/
def nextTic? (s : Session) : Option ReadyTic :=
  if s.phase != .running || s.runNext ≥ s.setNext then none
  else
    let b := setOff s.runNext
    some { tic := s.runNext, ingame := rd8 s.setRing b
           cmds := #[readCmd s.setRing (b + 1), readCmd s.setRing (b + 6),
                     readCmd s.setRing (b + 11), readCmd s.setRing (b + 16)] }

/-- Mark the tic returned by `nextTic?` as run. -/
def advance (s : Session) : Session :=
  if s.phase != .running || s.runNext ≥ s.setNext then s
  else
    let s := { s with runNext := s.runNext + 1 }
    if s.role == .host then s.produce else s

/--
Report the state checksum after running `tic`. Must be called in tic order,
after `advance`; out-of-order or early reports are ignored.
-/
def submitChecksum (s0 : Session) (tic : UInt32) (sum : UInt32) : Session :=
  if s0.phase != .running || tic != s0.sumNext || tic ≥ s0.runNext then s0
  else
    let limit := if s0.role == .host then s0.sumCompared else s0.sumAck
    if subSat tic limit ≥ RING32 then s0
    else
      let ring := s0.sumRing
      let s := { s0 with sumRing := .empty }
      let s := { s with sumRing := put32 ring (sumOff 0 tic) sum, sumNext := tic + 1 }
      if s.role == .host then (s.updPeer 0 fun q => { q with sumNext := tic + 1 }).compareSums
      else s

/-! ## Host: lobby and start -/

private def findAddr (s : Session) (a : Addr) : Option Nat := Id.run do
  for p in [1:MAX_PLAYERS] do
    let q := s.peer p
    if q.state != ST_EMPTY && q.addr == a then return some p
  return none

private def freeSlot (s : Session) : Option Nat := Id.run do
  for p in [1:MAX_PLAYERS] do
    if (s.peer p).state == ST_EMPTY then return some p
  return none

private def rejectTo (s : Session) (to : Addr) (nonce : UInt32) (r : RejectReason) : Session :=
  let s := { s with stats := { s.stats with rejectsQueued := s.stats.rejectsQueued + 1 } }
  s.queue to (encode 0 (.reject nonce r PROTOCOL_VERSION))

private def onHello (s : Session) (src : Addr) (nonce : UInt32) (digest : ByteArray) (name : String)
    (now : UInt64) : Session :=
  match s.phase with
  | .lobby =>
    let digestOk := digest == s.cfg.iwadDigest
    match findAddr s src with
    | some p =>
      if !digestOk then markAllDirty ((s.updPeer p fun _ => {}).rejectTo src nonce .iwad)
      else
        let q := s.peer p
        let changed := q.nonce != nonce || q.name != name
        let s := s.updPeer p fun q => { q with lastHeard := now, nonce, name, dirty := true }
        if changed then markAllDirty s else s
    | none =>
      if !digestOk then s.rejectTo src nonce .iwad
      else match freeSlot s with
        | some p =>
          markAllDirty (s.updPeer p fun _ =>
            { state := ST_LOBBY, addr := src, nonce, name, lastHeard := now })
        | none => s.rejectTo src nonce .full
  | .running =>
    match findAddr s src with
    | some p =>
      let q := s.peer p
      if q.state == ST_GAME && q.nonce == nonce then
        if q.startAcked then s  -- stale HELLO from a playing joiner
        else s.updPeer p fun q => { q with dirty := true, lastHeard := now }
      else s.rejectTo src nonce .started
    | none => s.rejectTo src nonce .started
  | _ => s

/-- Host: settings and player numbers as sent to player `p`. -/
def startInfoFor (s : Session) (p : Nat) : Option StartInfo :=
  s.info.map fun i => { i with consoleplayer := p.toUInt8 }

/--
Host: start the game with the players currently in the lobby (renumbered
`0..n-1` in slot order; the host is player 0).
-/
def start (s0 : Session) (g : GameSettings) (nowMs : UInt64) : Except String Session := do
  if s0.role != .host || s0.phase != .lobby then throw "start: not a host lobby"
  if !g.valid then throw "start: invalid game settings"
  let mut order : Array Nat := #[]
  for p in [0:MAX_PLAYERS] do
    if (s0.peer p).state == ST_LOBBY then order := order.push p
  let n := order.size
  let mask : UInt8 := ((1 : UInt8) <<< n.toUInt8) - 1
  let t := g.startTic
  let mut peers : Array Peer := #[{}, {}, {}, {}]
  let mut names : Array String := #["", "", "", ""]
  for k in [0:n] do
    let old := s0.peer (order.getD k 0)
    names := names.setIfInBounds k old.name
    peers := peers.setIfInBounds k { old with
      state := ST_GAME, startAcked := k == 0, cmdNext := t, setAck := t, sumNext := t
      lastHeard := nowMs, lastSent := NEVER, dirty := true
      sentSetEnd := t, sentCmdAck := t, sentSumAck := t }
  let info : StartInfo := { settings := g, playerMask := mask, consoleplayer := 0
                            inputDelay := s0.inputDelay.toUInt8, names }
  let s := { s0 with
    phase := .running, peers, info := some info, startMask := mask, slot := 0
    gen := t, setNext := t, runNext := t, sumNext := t, sumCompared := t
    nextDue35 := nowMs * 35, drops := #[], desync := none }
  pure s.prefill.produce

/-! ## Host: in-game traffic -/

private def onCmds (s0 : Session) (src : Addr) (c : CmdsMsg) (now : UInt64) : Session := Id.run do
  if s0.phase != .running then return s0
  let p := c.slot.toNat
  if p == 0 || p ≥ MAX_PLAYERS then return s0
  let q := s0.peer p
  if q.addr != src then return s0
  if q.state == ST_DROPPED then return s0.queue src (encode s0.session (.leave c.slot .kicked))
  if q.state != ST_GAME then return s0
  let mut s := s0
  let mut cmdNext := q.cmdNext
  let mut ring := s.cmdRing
  s := { s with cmdRing := .empty }
  for i in [0:c.count] do
    let t := c.firstTic + i.toUInt32
    if t == cmdNext then
      if subSat cmdNext s.setNext ≥ RING32 then break
      ring := copyBytes ring (cmdOff p t) c.cmds (i * CMD_BYTES) CMD_BYTES
      cmdNext := cmdNext + 1
    else if t > cmdNext then break
  s := { s with cmdRing := ring }
  let mut sumNext := q.sumNext
  let mut sring := s.sumRing
  s := { s with sumRing := .empty }
  for i in [0:c.sumCount] do
    let t := c.sumFirst + i.toUInt32
    if t == sumNext then
      if subSat sumNext s.sumCompared ≥ RING32 then break
      sring := copyBytes sring (sumOff p t) c.sums (4 * i) 4
      sumNext := sumNext + 1
    else if t > sumNext then break
  s := { s with sumRing := sring }
  let setAck := if c.setAck > q.setAck && c.setAck ≤ s.setNext then c.setAck else q.setAck
  s := s.updPeer p fun q => { q with lastHeard := now, startAcked := true, cmdNext, sumNext, setAck }
  return s.produce.compareSums

private def onHostLeave (s : Session) (src : Addr) (slot : UInt8) : Session :=
  let p := slot.toNat
  if p == 0 || p ≥ MAX_PLAYERS || (s.peer p).addr != src then s
  else match s.phase with
    | .lobby => if (s.peer p).state == ST_LOBBY then markAllDirty (s.updPeer p fun _ => {}) else s
    | .running => s.dropPlayer p .quit
    | _ => s

/-! ## Joiner -/

private def onLobby (s : Session) (session nonce : UInt32) (you mask : UInt8) (names : Array String)
    (now : UInt64) : Session := Id.run do
  if nonce != s.cfg.nonce || (s.phase != .joining && s.phase != .lobby) then return s
  let mut peers : Array Peer := #[{}, {}, {}, {}]
  for p in [0:MAX_PLAYERS] do
    if bit mask p then peers := peers.setIfInBounds p { state := ST_LOBBY, name := names.getD p "" }
  return { s with phase := .lobby, session, slot := you, peers, lastHeard := now }

private def onStart (s0 : Session) (session nonce : UInt32) (info : StartInfo) (now : UInt64) :
    Session := Id.run do
  if nonce != s0.cfg.nonce then return s0
  if s0.phase == .running then
    return if session == s0.session then { s0 with lastHeard := now } else s0
  if s0.phase != .joining && s0.phase != .lobby then return s0
  let t := info.settings.startTic
  let delay := info.inputDelay.toUInt32
  let mut peers : Array Peer := #[{}, {}, {}, {}]
  for p in [0:MAX_PLAYERS] do
    if bit info.playerMask p then
      peers := peers.setIfInBounds p { state := ST_GAME, name := info.names.getD p "" }
  let s := { s0 with
    phase := .running, session, slot := info.consoleplayer, info := some info
    startMask := info.playerMask, inputDelay := delay
    maxLead := max s0.cfg.maxLead.toUInt32 (delay + 2)
    gen := t, setNext := t, runNext := t, cmdAck := t, sumNext := t, sumAck := t
    sentGen := t, sentSetNext := t, nextDue35 := now * 35, lastHeard := now
    lastSent := NEVER, peers, drops := #[], desync := none }
  return s.prefill

private def onSets (s0 : Session) (m : SetsMsg) (now : UInt64) : Session := Id.run do
  let mut s := { s0 with lastHeard := now }
  if m.cmdAck > s.cmdAck && m.cmdAck ≤ s.gen then s := { s with cmdAck := m.cmdAck }
  if m.sumAck > s.sumAck && m.sumAck ≤ s.sumNext then s := { s with sumAck := m.sumAck }
  for d in m.drops do
    if !(s.drops.any (·.slot == d.slot)) && s.drops.size < MAX_PLAYERS then
      s := { s with drops := s.drops.push d }
      s := s.updPeer d.slot.toNat fun q => { q with state := ST_DROPPED }
      if d.slot == s.slot then return finish s .dropped
  if m.desyncTic != NO_TIC && s.desync.isNone then
    s := { s with desync := some { tic := m.desyncTic, slots := m.desyncMask } }
  let before := s.setNext
  let mut setNext := s.setNext
  let mut ring := s.setRing
  s := { s with setRing := .empty }
  let mut pos := 0
  let mut dropped := false
  for k in [0:m.count] do
    let t := m.firstTic + k.toUInt32
    let mask := rd8 m.body pos
    if t == setNext && subSat t s.runNext < RING32 && t < s.gen then
      if !bit mask s.slot.toNat then
        dropped := true
        break
      let base := setOff t
      ring := put8 ring base mask
      let mut q := pos + 1
      for p in [0:MAX_PLAYERS] do
        if bit mask p then
          ring := copyBytes ring (base + 1 + p * CMD_BYTES) m.body q CMD_BYTES
          q := q + CMD_BYTES
        else ring := zeroRange ring (base + 1 + p * CMD_BYTES) CMD_BYTES
      setNext := setNext + 1
    pos := pos + setBytes mask
  s := { s with setRing := ring, setNext }
  if dropped then return finish s .dropped
  let ah := sext8 m.ahead
  s := { s with ahead := ah }
  if setNext > before && ah ≥ 2 then
    s := { s with nextDue35 := s.nextDue35 + 70 * ((min ah 8) - 1).toUInt32.toUInt64 }
  return s

private def onJoinerLeave (s : Session) (session : UInt32) (slot : UInt8) : Session :=
  if s.phase == .joining || s.phase == .ended || session != s.session then s
  else if slot == 0 then finish s .hostQuit
  else if slot == s.slot then finish s .dropped
  else s

/-! ## Datagram input -/

/-- Feed one received datagram. Invalid, foreign and stale datagrams are ignored. -/
def receive (s0 : Session) (src : Addr) (bytes : ByteArray) (nowMs : UInt64) : Session :=
  let s := { s0 with stats := { s0.stats with rxPackets := s0.stats.rxPackets + 1 } }
  match decode bytes with
  | .invalid => { s with stats := { s.stats with rxInvalid := s.stats.rxInvalid + 1 } }
  | .wrongVersion _ nonce =>
    if s.role == .host && (s.phase == .lobby || s.phase == .running) then
      s.rejectTo src nonce .version
    else s
  | .ok session msg =>
    match s.role with
    | .host =>
      match msg with
      | .hello nonce digest name => s.onHello src nonce digest name nowMs
      | .cmds c => if session == s.session then s.onCmds src c nowMs else s
      | .leave slot _ => if session == s.session then s.onHostLeave src slot else s
      | _ => s
    | .joiner =>
      if src != s.hostAddr then s
      else match msg with
        | .lobby nonce you mask names => s.onLobby session nonce you mask names nowMs
        | .reject nonce r _ =>
          if nonce == s.cfg.nonce && (s.phase == .joining || s.phase == .lobby) then
            finish s (.rejected r)
          else s
        | .start nonce info => s.onStart session nonce info nowMs
        | .sets m => if s.phase == .running && session == s.session then s.onSets m nowMs else s
        | .leave slot _ => s.onJoinerLeave session slot
        | _ => s

/-! ## Datagram output -/

/-- Host: the SETS message for player `p`. -/
private def setsFor (s : Session) (p : Nat) (q : Peer) : SetsMsg := Id.run do
  let first := q.setAck
  let n := min (subSat s.setNext first).toNat MAX_SETS_PER_PACKET
  let mut body := ByteArray.emptyWithCapacity (n * SET_STRIDE)
  for k in [0:n] do
    let t := first + k.toUInt32
    let base := setOff t
    let m := rd8 s.setRing base
    body := body.push m
    for pl in [0:MAX_PLAYERS] do
      if bit m pl then body := appendRange body s.setRing (base + 1 + pl * CMD_BYTES) CMD_BYTES
  let mut minNext := s.gen
  for x in [1:MAX_PLAYERS] do
    let r := s.peer x
    if r.state == ST_GAME && r.cmdNext < minNext then minNext := r.cmdNext
  let ah := min (subSat q.cmdNext minNext) 127
  let (dt, dm) := match s.desync with
    | some d => (d.tic, d.slots)
    | none => (NO_TIC, 0)
  let _ := p
  return { cmdAck := q.cmdNext, sumAck := q.sumNext, ahead := ah.toUInt8, firstTic := first
           count := n, body, drops := s.drops, desyncTic := dt, desyncMask := dm }

private def hostShouldSend (s : Session) (q : Peer) (now : UInt64) (c : Config) : Bool :=
  q.dirty || s.setNext > q.sentSetEnd || isDue now q.lastSent c.keepaliveMs
    || (isDue now q.lastSent c.resendMs
        && (q.setAck < s.setNext || q.cmdNext != q.sentCmdAck || q.sumNext != q.sentSumAck))

private def joinerShouldSend (s : Session) (now : UInt64) : Bool :=
  let c := s.cfg
  s.gen > s.sentGen || isDue now s.lastSent c.keepaliveMs
    || (isDue now s.lastSent c.resendMs
        && (s.cmdAck < s.gen || s.sumAck < s.sumNext || s.setNext != s.sentSetNext))

/-- Joiner: the CMDS message carrying every unacknowledged command and checksum. -/
private def cmdsMsg (s : Session) : CmdsMsg :=
  let n := min (subSat s.gen s.cmdAck).toNat MAX_CMDS_PER_PACKET
  let k := min (subSat s.sumNext s.sumAck).toNat MAX_SUMS_PER_PACKET
  let cmds := Id.run do
    let mut b := ByteArray.emptyWithCapacity (n * CMD_BYTES)
    for i in [0:n] do
      b := appendRange b s.cmdRing (cmdOff 0 (s.cmdAck + i.toUInt32)) CMD_BYTES
    return b
  let sums := Id.run do
    let mut b := ByteArray.emptyWithCapacity (4 * k)
    for i in [0:k] do
      b := appendRange b s.sumRing (sumOff 0 (s.sumAck + i.toUInt32)) 4
    return b
  { slot := s.slot, setAck := s.setNext, firstTic := s.cmdAck, count := n, cmds
    sumFirst := s.sumAck, sumCount := k, sums }

/--
Timers and output: returns every datagram to send now. Call at least once per
frame (and after `leave`, to flush the leave notices).
-/
def poll (s0 : Session) (nowMs : UInt64) : Session × Array (Addr × ByteArray) := Id.run do
  let now := nowMs
  let mut s := s0
  let mut out := s.outbox
  s := { s with outbox := #[] }
  let c := s.cfg
  match s.role, s.phase with
  | .host, .lobby =>
    for p in [1:MAX_PLAYERS] do
      let q := s.peer p
      if q.state == ST_LOBBY && isSilent now q.lastHeard c.joinTimeoutMs then
        s := markAllDirty (s.updPeer p fun _ => {})
    let (mask, names) := roster s ST_LOBBY
    for p in [1:MAX_PLAYERS] do
      let q := s.peer p
      if q.state == ST_LOBBY && (q.dirty || isDue now q.lastSent c.keepaliveMs) then
        out := out.push (q.addr, encode s.session (.lobby q.nonce p.toUInt8 mask names))
        s := s.updPeer p fun q => { q with dirty := false, lastSent := now }
  | .host, .running =>
    let loaded := (s.info.map (·.settings.startTic)).getD 0 + s.inputDelay
    for p in [1:MAX_PLAYERS] do
      let q := s.peer p
      let timeout := if q.cmdNext ≤ loaded then c.loadTimeoutMs else c.dropTimeoutMs
      if q.state == ST_GAME && isSilent now q.lastHeard timeout then
        s := s.dropPlayer p .timeout
    for p in [1:MAX_PLAYERS] do
      let q := s.peer p
      if q.state == ST_GAME then
        if !q.startAcked then
          if q.dirty || isDue now q.lastSent c.startResendMs then
            match s.startInfoFor p with
            | some info =>
              let b := encode s.session (.start q.nonce info)
              -- the first START goes out twice: the host may block (level load) right after
              out := if q.lastSent == NEVER then (out.push (q.addr, b)).push (q.addr, b)
                     else out.push (q.addr, b)
              s := s.updPeer p fun q => { q with dirty := false, lastSent := now }
            | none => pure ()
        else if hostShouldSend s q now c then
          let msg := setsFor s p q
          out := out.push (q.addr, encode s.session (.sets msg))
          let setEnd := s.setNext
          s := s.updPeer p fun q => { q with
            dirty := false, lastSent := now, sentSetEnd := setEnd
            sentCmdAck := q.cmdNext, sentSumAck := q.sumNext }
  | .joiner, .joining | .joiner, .lobby =>
    if s.firstPoll == NEVER then s := { s with firstPoll := now }
    if s.phase == .joining && isSilent now s.firstPoll c.joinTimeoutMs then
      s := finish s .noResponse
    else if s.phase == .lobby && isSilent now s.lastHeard c.joinTimeoutMs then
      s := finish s .hostLost
    else if isDue now s.lastSent c.helloMs then
      out := out.push (s.hostAddr, encode 0 (.hello c.nonce c.iwadDigest c.name))
      s := { s with lastSent := now }
  | .joiner, .running =>
    let loaded := (s.info.map (·.settings.startTic)).getD 0 + s.inputDelay
    let timeout := if s.setNext ≤ loaded then c.loadTimeoutMs else c.dropTimeoutMs
    if isSilent now s.lastHeard timeout then
      s := finish s .hostLost
    else if joinerShouldSend s now then
      out := out.push (s.hostAddr, encode s.session (.cmds (cmdsMsg s)))
      s := { s with lastSent := now, sentGen := s.gen, sentSetNext := s.setNext }
  | _, _ => pure ()
  let mut st := s.stats
  for (_, b) in out do
    st := { st with txPackets := st.txPackets + 1, txBytes := st.txBytes + b.size
                    maxPacket := max st.maxPacket b.size }
  return ({ s with stats := st }, out)

/--
Leave the session: queue LEAVE notices (flushed by the next `poll`) and end.
A host leaving ends the game for every joiner.
-/
def leave (s0 : Session) : Session := Id.run do
  if s0.phase == .ended then return s0
  let mut s := s0
  match s.role with
  | .host =>
    for p in [1:MAX_PLAYERS] do
      let q := s.peer p
      if q.state == ST_LOBBY || q.state == ST_GAME then
        let b := encode s.session (.leave 0 .quit)
        s := (s.queue q.addr b).queue q.addr b
  | .joiner =>
    if s.phase != .joining then
      let b := encode s.session (.leave s.slot .quit)
      s := (s.queue s.hostAddr b).queue s.hostAddr b
  return finish s .localQuit

/-! ## Views for the front end -/

instance : Inhabited Session := ⟨host {}⟩

/-- Players in the lobby (or in the game once running), in slot order. -/
def lobby (s : Session) : Array LobbyEntry := Id.run do
  let mut out := #[]
  for p in [0:MAX_PLAYERS] do
    let q := s.peer p
    if q.state == ST_LOBBY || q.state == ST_GAME then
      out := out.push { slot := p, name := q.name, isLocal := p == s.slot.toNat, isHost := p == 0 }
  return out

def playerCount (s : Session) : Nat := (s.lobby).size

def pacing (s : Session) : Pacing := {
  readyTics := (subSat s.setNext s.runNext).toNat
  localLead := (subSat s.gen s.runNext).toNat
  aheadTics := s.ahead.toInt
  windowOpen := s.windowOpen }

/-- The consoleplayer once running. -/
def consoleplayer (s : Session) : Nat := s.slot.toNat

end Session

end Doom.Net
