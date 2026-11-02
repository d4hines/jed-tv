import DoomTest.Net.Rng
/-!
# net-test: codec round trips, TicCmd clamping and lobby/session unit checks

Exit code 0 iff every check passed.
-/

namespace Doom.Net.Test.Codec

open Doom.Net
open Doom.Net.Test
open Doom.Playsim.Player (TicCmd)

structure Ctx where
  fails : IO.Ref Nat
  checks : IO.Ref Nat

def check (c : Ctx) (name : String) (ok : Bool) : IO Unit := do
  c.checks.modify (· + 1)
  unless ok do
    c.fails.modify (· + 1)
    IO.eprintln s!"FAIL: {name}"

def mkCmd (f s a : Int) (b : UInt32) : TicCmd :=
  { forwardmove := Int32.ofInt f, sidemove := Int32.ofInt s, angleturn := Int32.ofInt a, buttons := b }

def roundTripCmd (c : TicCmd) : TicCmd := readCmd (packCmd c) 0

def clamping (c : Ctx) : IO Unit := do
  let cases : List (TicCmd × TicCmd) := [
    (mkCmd 300 (-300) 40000 0x1ff, mkCmd 127 (-128) 32767 0xff),
    (mkCmd (-129) 128 (-40000) 0x100, mkCmd (-128) 127 (-32768) 0),
    (mkCmd 127 (-128) 32767 0xff, mkCmd 127 (-128) 32767 0xff),
    (mkCmd (-2147483648) 2147483647 (-2147483648) 0xffffffff, mkCmd (-128) 127 (-32768) 0xff),
    (mkCmd 0 0 0 0, mkCmd 0 0 0 0),
    (mkCmd 50 (-50) (-640) 7, mkCmd 50 (-50) (-640) 7)]
  for (i, e) in cases do
    check c s!"clamp {repr i}" (roundTripCmd i == e && clampCmd i == e)
  -- exhaustive in-range identity
  let mut bad := 0
  for f in [0:256] do
    for b in [0:256] do
      let x := mkCmd (Int.ofNat f - 128) (Int.ofNat b - 128) 0 b.toUInt32
      if roundTripCmd x != x then bad := bad + 1
  for a in [0:65536] do
    let x := mkCmd 0 0 (Int.ofNat a - 32768) 0
    if roundTripCmd x != x then bad := bad + 1
  check c "in-range TicCmd round trip (fwd×buttons, side, all angleturn)" (bad == 0)
  IO.println s!"clamping: {cases.length} edge cases, 131072 in-range values, {bad} mismatches"

def roundTrips (c : Ctx) (n : Nat) : IO Unit := do
  let mut rng := Rng.ofSeed 42
  let mut bad := 0
  let mut maxSize := 0
  let mut perType : Array Nat := #[0, 0, 0, 0, 0, 0, 0]
  for i in [0:n] do
    let ((m, sess), r) := (do return (← randMsg, ← randU32)).run rng
    rng := r
    let b := encode sess m
    maxSize := max maxSize b.size
    let t := (rd8 b 3).toNat
    perType := perType.modify (t - 1) (· + 1)
    if decode b != .ok sess m || b.size > MAX_PACKET then
      bad := bad + 1
      if bad ≤ 3 then IO.eprintln s!"round trip mismatch #{i} type {t} size {b.size}"
  check c "random message round trips" (bad == 0)
  IO.println s!"round trips: {n} random messages (per type {perType}), {bad} mismatches, largest {maxSize} B"
  for m in maxMsgs do
    let b := encode 7 m
    check c s!"max message type {rd8 b 3} round trip, {b.size} B" (decode b == .ok 7 m && b.size ≤ MAX_PACKET)
    IO.println s!"max-size type {rd8 b 3}: {b.size} B"
  -- canonical encoders clip oversize inputs rather than overflow
  let big := Msg.cmds { slot := 1, setAck := 0, firstTic := 0, count := 500, cmds := zeroBytes 2500
                        sumFirst := 0, sumCount := 500, sums := zeroBytes 2000 }
  let bb := encode 1 big
  check c "oversize CMDS clipped" (bb.size ≤ MAX_PACKET && (decode bb matches .ok _ _))
  -- name sanitising on encode
  let h := encode 0 (.hello 5 .empty "bad\nname-that-is-far-too-long")
  check c "names sanitised to printable ≤16" (decode h == .ok 0 (.hello 5 .empty "bad?name-that-is"))

def versionChecks (c : Ctx) : IO Unit := do
  let h := encode 0 (.hello 77 .empty "x")
  let h2 := put8 h 2 9
  check c "foreign-version HELLO reports version + nonce" (decode h2 == .wrongVersion 9 77)
  let r := put8 (encode 0 (.reject 77 .version PROTOCOL_VERSION)) 2 9
  check c "REJECT decodes under any version" (decode r == .ok 0 (.reject 77 .version PROTOCOL_VERSION))
  let s := put8 (encode 3 (.leave 1 .quit)) 2 9
  check c "other foreign-version messages invalid" (decode s == .invalid)
  check c "bad magic invalid" (decode (put8 h 0 0) == .invalid)
  check c "trailing byte invalid" (decode (h.push 0) == .invalid)
  check c "truncated invalid" (decode (h.extract 0 (h.size - 1)) == .invalid)
  check c "oversize datagram invalid" (decode (h.append (zeroBytes 1400)) == .invalid)

/-! ## Session unit checks -/

def hostA : Addr := ⟨0x0A000001, PORT⟩
def joinA (i : Nat) : Addr := ⟨0x0A000001 + i.toUInt32, PORT⟩
def digest : ByteArray := ⟨#[1, 2, 3, 4]⟩

def hostCfg : Config := { name := "host", iwadDigest := digest, nonce := 0xABCD }
def joinCfg (i : Nat) : Config := { name := s!"j{i}", iwadDigest := digest, nonce := 100 + i.toUInt32 }

/-- Deliver all datagrams of `out` addressed to `to` into `s`. -/
def deliver (s : Session) (src to : Addr) (out : Array (Addr × ByteArray)) (now : UInt64) : Session :=
  out.foldl (init := s) fun s (a, b) => if a == to then s.receive src b now else s

/-- One host + `n` joiners exchange lobby traffic for `rounds` polls. -/
def lobbyRounds (h0 : Session) (js0 : Array Session) (rounds : Nat) (t0 : UInt64) :
    Session × Array Session := Id.run do
  let mut h := h0
  let mut js := js0
  for r in [0:rounds] do
    let now := t0 + r.toUInt64 * 10
    let mut toHost : Array (Addr × Addr × ByteArray) := #[]
    for i in [0:js.size] do
      let (j, out) := (js.getD i default).poll now
      js := js.setIfInBounds i j
      for (a, b) in out do toHost := toHost.push (joinA (i + 1), a, b)
    for (src, a, b) in toHost do
      if a == hostA then h := h.receive src b now
    let (h', out) := h.poll now
    h := h'
    for i in [0:js.size] do
      js := js.modify i fun j => deliver j hostA (joinA (i + 1)) out now
  return (h, js)

def sessionChecks (c : Ctx) : IO Unit := do
  -- lobby join
  let h := Session.host hostCfg
  let js := #[Session.join (joinCfg 1) hostA, Session.join (joinCfg 2) hostA]
  let (h, js) := lobbyRounds h js 5 0
  check c "host lobby has 3 players" (h.playerCount == 3)
  check c "joiners in lobby with slots 1,2" (js.all (·.phase == .lobby)
    && (js.getD 0 default).slot == 1 && (js.getD 1 default).slot == 2)
  check c "joiner roster names" (((js.getD 1 default).lobby.map (·.name)) == #["host", "j1", "j2"])
  -- iwad mismatch
  let bad := Session.join { joinCfg 3 with iwadDigest := ⟨#[9]⟩ } hostA
  let (h2, js2) := lobbyRounds h (js.push bad) 3 100
  check c "IWAD mismatch rejected" ((js2.getD 2 default).endReason == some (.rejected .iwad))
  check c "IWAD mismatch not admitted" (h2.playerCount == 3)
  -- full
  let full := #[Session.join (joinCfg 1) hostA, Session.join (joinCfg 2) hostA,
                Session.join (joinCfg 3) hostA, Session.join (joinCfg 4) hostA]
  let (hf, jf) := lobbyRounds (Session.host hostCfg) full 5 0
  check c "4 players admitted" (hf.playerCount == 4)
  check c "5th player rejected full" ((jf.getD 3 default).endReason == some (.rejected .full))
  -- version mismatch through the session
  let foreign := put8 (encode 0 (.hello 555 digest "old")) 2 0
  let (hv, outv) := (Session.host hostCfg |>.receive (joinA 9) foreign 0).poll 0
  let jv := deliver ({ Session.join { joinCfg 9 with nonce := 555 } hostA with phase := .joining })
    hostA (joinA 9) outv 0
  check c "version mismatch rejected" (jv.endReason == some (.rejected .version) && hv.playerCount == 1)
  -- start, then strangers
  match h.start { map := 3, skill := 3, deathmatch := 1 } 1000 with
  | .error e => check c s!"start: {e}" false
  | .ok hs =>
    check c "host running" (hs.phase == .running)
    let (hs, out) := hs.poll 1000
    let j1 := deliver (js.getD 0 default) hostA (joinA 1) out 1000
    check c "joiner got START" (j1.phase == .running && j1.consoleplayer == 1
      && (j1.info.map (·.settings.map)) == some 3)
    -- a stranger's HELLO is rejected 'started'; its CMDS are ignored
    let st := Session.join (joinCfg 7) hostA
    let (st, sout) := st.poll 1000
    let hs2 := sout.foldl (init := hs) fun s (_, b) => s.receive (joinA 7) b 1000
    let (hs2, out2) := hs2.poll 1001
    let st := deliver st hostA (joinA 7) out2 1001
    check c "post-start stranger rejected" (st.endReason == some (.rejected .started))
    let forged := encode hs.session (.cmds {
      slot := 1, setAck := 0, firstTic := 0, count := 1
      cmds := zeroBytes 5, sumFirst := 0, sumCount := 0, sums := .empty })
    let hs3 := hs2.receive (joinA 7) forged 1002
    check c "stranger CMDS ignored" ((hs3.peer 1).cmdNext == (hs2.peer 1).cmdNext
      && hs3.setNext == hs2.setNext)
    let stale := encode (hs.session + 1) (.cmds {
      slot := 1, setAck := 0, firstTic := 0, count := 1
      cmds := zeroBytes 5, sumFirst := 0, sumCount := 0, sums := .empty })
    let hs4 := hs2.receive (joinA 1) stale 1002
    check c "wrong-session CMDS ignored" ((hs4.peer 1).cmdNext == (hs2.peer 1).cmdNext)
  -- lobby leave
  let (hl, jl) := lobbyRounds (Session.host hostCfg) #[Session.join (joinCfg 1) hostA] 3 0
  let jq := (jl.getD 0 default).leave
  let (_, qo) := jq.poll 50
  let hl := qo.foldl (init := hl) fun s (_, b) => s.receive (joinA 1) b 50
  check c "lobby leave removes player" (hl.playerCount == 1 && jq.endReason == some .localQuit)
  -- lobby timeout
  let (ht, _) := (lobbyRounds (Session.host hostCfg) #[Session.join (joinCfg 1) hostA] 3 0).1.poll 6000
  check c "silent lobby joiner removed" (ht.playerCount == 1)
  -- join timeout
  let (jn, _) := (Session.join (joinCfg 1) hostA).poll 0
  let (jn, _) := jn.poll 5001
  check c "no response from host" (jn.endReason == some .noResponse)

def main : IO UInt32 := do
  let c : Ctx := { fails := ← IO.mkRef 0, checks := ← IO.mkRef 0 }
  clamping c
  roundTrips c 200000
  versionChecks c
  sessionChecks c
  let f ← c.fails.get
  let n ← c.checks.get
  IO.println s!"net-test: {n - f}/{n} checks passed"
  return if f == 0 then 0 else 1

end Doom.Net.Test.Codec

def main : IO UInt32 := Doom.Net.Test.Codec.main
