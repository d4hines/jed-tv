import Yt.Lounge.Session
import LoungeTest.Fixtures
open Yt.Lounge Yt.Fmt

/-! Transcript test for the Lounge screen session: recorded YouTube replies
and long-poll chunks (Fixtures, real chunk boundaries) through `Session`,
with a simulated app player answering the commands. Prints every request
and command plus PASS/FAIL checks; host and 32-bit ARM output must match. -/
namespace LoungeTest
namespace F
export LoungeTest.Fixtures (screenIdBody tokenBatchBody pairingCodeBody bindOpenBody sendOkBody
  soakBindOpenBody e2ePoll soakPoll1 soakPoll2)
end F

/-- `Option.get!` without its panic message (not available on our runtime). -/
def _root_.Option.req {α : Type} [Inhabited α] (o : Option α) : α := o.getD default

structure T where
  o : ByteArray := ByteArray.empty
  pass : Nat := 0
  fail : Nat := 0

def T.line (t : T) (s : String) : T := { t with o := t.o ++ (s ++ "\n").toUTF8 }
def T.check (t : T) (label : String) (ok : Bool) : T :=
  let t := t.line ((if ok then "PASS " else "FAIL ") ++ label)
  if ok then { t with pass := t.pass + 1 } else { t with fail := t.fail + 1 }

def reqText (r : Request) : String :=
  "  REQ #" ++ toString r.id ++ " " ++ r.tag.name ++ " " ++ r.method ++ (if r.streaming then " stream" else "")
  ++ "\n    " ++ r.url ++ (if r.body.size > 0 then "\n    body " ++ str r.body else "")

def T.reqs (t : T) (rs : Array Request) : T := rs.foldl (fun t r => t.line (reqText r)) t
def T.cmds (t : T) (cs : Array Command) : T := cs.foldl (fun t c => t.line ("  CMD " ++ c.describe)) t

def contains (hay needle : String) : Bool := Yt.Fmt.contains hay.toUTF8 needle.toUTF8

def tagged (rs : Array Request) (tag : Tag) : Option Request := Id.run do
  for r in rs do
    if r.tag == tag then return some r
  return none

def countTag (rs : Array Request) (tag : Tag) : Nat := rs.foldl (fun n r => if r.tag == tag then n + 1 else n) 0

def ident0 : Identity := { deviceId := "00000000-0000-4000-8000-000000000001", name := "Dande TV (prototype)" }
def entropy0 : ByteArray := ByteArray.mk #[1, 2, 3, 4, 5, 6, 7, 8]

/-- Answer every send in `rs` with 200 (repeatedly, as outboxes flush). -/
def ackSends (t : T) (s : Session) (rs : Array Request) (now : UInt64) : T × Session := Id.run do
  let mut t := t
  let mut s := s
  let mut pending := rs.filter (fun r => r.tag == .send)
  let mut rounds := 0
  while !pending.isEmpty && rounds < 8 do
    let mut next : Array Request := #[]
    for r in pending do
      let (s', rs', _) := s.onResponse r.id 200 F.sendOkBody.toUTF8 now
      s := s'
      t := t.reqs rs'
      next := next ++ rs'.filter (fun r => r.tag == .send)
    pending := next
    rounds := rounds + 1
  return (t, s)

/-- A session that is online after the recorded bind (fresh install). -/
def bootstrap (cfg : Config := {}) : Session × Array Request := Id.run do
  let (s, rs) := Session.start cfg ident0 entropy0 1000
  let (s, rs, _) := s.onResponse rs[0]!.id 200 F.screenIdBody.toUTF8 1100
  let (s, rs, _) := s.onResponse rs[0]!.id 200 F.tokenBatchBody.toUTF8 1200
  let (s, rs, _) := s.onResponse rs[0]!.id 200 F.bindOpenBody.toUTF8 1300
  return (s, rs)

/-- The simulated app player: apply a command. -/
def apply (p : PlayerState) (c : Command) : PlayerState :=
  match c with
  | .playVideo v t _ => { p with videoId := some v, state := .buffering, currentTimeMs := t, durationMs := 0 }
  | .play => { p with state := .playing }
  | .pause => { p with state := .paused }
  | .seek ms => { p with currentTimeMs := ms }
  | .stop => { p with state := .unstarted, currentTimeMs := 0 }
  | .setVolume v m => { p with volume := v, muted := m }
  | _ => p

/-- Scenario 1: fresh install up to an open bind channel and a TV code. -/
def scenarioStart (t : T) : T := Id.run do
  let mut t := t.line "== 1. fresh install: screen ID, token, bind, poll, code"
  let (s, rs) := Session.start {} ident0 entropy0 1000
  t := t.reqs rs
  t := t.line ("  headers " ++ (rs[0]!.headers.foldl (fun a (k, v) => a ++ k ++ ": " ++ v ++ "; ") ""))
  t := t.check "start: one generate_screen_id GET" (rs.size == 1 && rs[0]!.tag == .generateScreenId && rs[0]!.method == "GET")
  let (s, rs, _) := s.onResponse rs[0]!.id 200 F.screenIdBody.toUTF8 1100
  t := t.reqs rs
  t := t.line ("  headers " ++ (rs[0]!.headers.foldl (fun a (k, v) => a ++ k ++ ": " ++ v ++ "; ") ""))
  t := t.check "screen ID learned" (s.identity.screenId == some "testscreenid00000000000000")
  t := t.check "token batch POST with screen_ids" (rs.size == 1 && str rs[0]!.body == "screen_ids=testscreenid00000000000000")
  let (s, rs, _) := s.onResponse rs[0]!.id 200 F.tokenBatchBody.toUTF8 1200
  t := t.reqs rs
  t := t.check "bind open: count=0, token in query, CVER=1" (rs.size == 1 && rs[0]!.tag == .bindOpen &&
    str rs[0]!.body == "count=0" && contains rs[0]!.url "loungeIdToken=FAKE-LOUNGE-TOKEN-0123456789" &&
    contains rs[0]!.url "&CVER=1" && contains rs[0]!.url "name=Dande+TV+%28prototype%29" &&
    contains rs[0]!.url "capabilities=dsp%2Cmic%2Cdpa")
  let openRid := s.rid - 1
  let (s, rs, cs) := s.onResponse rs[0]!.id 200 F.bindOpenBody.toUTF8 1300
  t := t.reqs rs
  t := t.cmds cs
  t := t.check "online with SID and gsessionid from c/S" (s.isOnline && s.sid == "F2CAD2E244BF47B5" &&
    s.gsid == "H5FbczIi42otxzlsO5Lrgm283iegxyQm")
  t := t.check "AID = last array id of the open reply (4)" (s.aid == 4)
  let poll := tagged rs .poll
  t := t.check "poll: streaming GET, RID=rpc, AID=4, TYPE=xmlhttp" (match poll with
    | some p => p.streaming && p.method == "GET" && contains p.url "&RID=rpc&SID=F2CAD2E244BF47B5&gsessionid=H5FbczIi42otxzlsO5Lrgm283iegxyQm&CI=0&AID=4&TYPE=xmlhttp"
    | none => false)
  t := t.check "pairing code requested" ((tagged rs .pairingCode).isSome)
  let send := tagged rs .send
  t := t.check "replies to getNowPlaying + getDiscoveryDeviceId in one send, RID = open RID + 1, ofs=0"
    (match send with
     | some r => str r.body == "count=2&ofs=0&req0__sc=nowPlaying&req1__sc=setDiscoveryDeviceId&req1_discoveryDeviceId=00000000-0000-4000-8000-000000000001&req1_loungeDeviceId=00000000-0000-4000-8000-000000000001"
         && contains r.url ("&RID=" ++ toString (openRid + 1) ++ "&AID=4")
     | none => false)
  t := t.check "no phone yet: no commands" cs.isEmpty
  let code := (tagged rs .pairingCode).req
  let (s, rs2, _) := s.onResponse code.id 200 F.pairingCodeBody.toUTF8 1400
  t := t.check "pairing code shown" (s.pairingCode == some "076706486798" && rs2.isEmpty)
  t := t.line ("  code: " ++ formatCode (s.pairingCode.getD ""))
  let (s, rs3, _) := s.onResponse send.req.id 200 F.sendOkBody.toUTF8 1500
  t := t.check "send acknowledged, nothing queued" (rs3.isEmpty && s.lastError == "")
  return t

/-- Scenario 2: the recorded long poll (phone connects, casts, controls). -/
def scenarioE2e (t : T) : T := Id.run do
  let mut t := t.line "== 2. recorded long poll: remote connects and controls playback"
  let (s0, rs0) := bootstrap
  let pollId := (tagged rs0 .poll).req.id
  let (t1, s1) := ackSends t s0 rs0 1300
  t := t1
  let mut s := s1
  let mut now : UInt64 := 2000
  let mut p : PlayerState := {}
  let mut all : Array Command := #[]
  for chunk in F.e2ePoll do
    now := now + 1500
    let (s', rs, cs) := s.onChunk pollId chunk.toUTF8 now
    s := s'
    t := t.reqs rs
    t := t.cmds cs
    let (t', s'') := ackSends t s rs now
    t := t'
    s := s''
    for c in cs do
      all := all.push c
      p := apply p c
      let (s', rs) := s.report p now
      s := s'
      t := t.reqs rs
      let (t', s'') := ackSends t s rs now
      t := t'
      s := s''
      -- The player finishes loading a second later and starts playing.
      match c with
      | .playVideo .. =>
        p := { p with state := .playing, durationMs := 600000 }
        let (s', rs) := s.report p (now + 1000)
        s := s'
        t := t.reqs rs
        let (t', s'') := ackSends t s rs (now + 1000)
        t := t'
        s := s''
      | _ => pure ()
  let names := all.map Command.describe
  t := t.line ("  commands: " ++ names.foldl (fun a n => a ++ n ++ " | ") "")
  let list := " list RQ3P9WICKq2gIYW0_N1Zf_4jFPP3I"
  t := t.check "command sequence as recorded" (names == #[
    "remoteConnected fake_remote (dande-tv test)",
    "playVideo PVGeM40dABA from 0.000s" ++ list,
    "pause", "play", "seek 60.000s", "setVolume 30",
    "playVideo aqz-KE-bpKQ from 0.000s" ++ list,
    "remoteDisconnected fake_remote (dande-tv test)"])
  t := t.check "remote list empty again" s.remoteNames.isEmpty
  t := t.check "queue from setPlaylist; next advanced to index 1" (s.queue == #["PVGeM40dABA", "aqz-KE-bpKQ"] && s.index == 1)
  t := t.check "AID tracked to the last array id seen" (s.aid ≥ 20)
  t := t.check "frames' declared lengths all matched the bytes" (s.framer.mismatches == 0)
  return t

/-- Scenario 3: framing at every split point. -/
def framesOf (pieces : Array ByteArray) : Array ByteArray × Bool × Nat := Id.run do
  let mut f : Framer := {}
  let mut out := #[]
  for b in pieces do
    let (f', fr) := f.feed b
    f := f'
    out := out ++ fr
  return (out, f.failed, f.mismatches)

def scenarioSplits (t : T) (label : String) (whole : ByteArray) : T := Id.run do
  let (ref, failed, mism) := framesOf #[whole]
  let mut t := t.line ("  " ++ label ++ ": " ++ toString whole.size ++ " bytes, " ++ toString ref.size ++
    " frames, failed=" ++ toString failed ++ ", length mismatches=" ++ toString mism)
  let mut ok := 0
  for k in [0:whole.size + 1] do
    let (fr, f2, _) := framesOf #[whole.extract 0 k, whole.extract k whole.size]
    if fr == ref && !f2 then ok := ok + 1
  t := t.check (label ++ ": identical frames at all " ++ toString (whole.size + 1) ++ " split points") (ok == whole.size + 1)
  let mut bytes : Array ByteArray := #[]
  for i in [0:whole.size] do
    bytes := bytes.push (whole.extract i (i + 1))
  let (fr, f3, _) := framesOf bytes
  t := t.check (label ++ ": byte-at-a-time feed") (fr == ref && !f3)
  return t

def joinChunks (cs : Array String) : ByteArray := cs.foldl (fun b c => b ++ c.toUTF8) ByteArray.empty

def scenarioFraming (t : T) : T := Id.run do
  let mut t := t.line "== 3. framing"
  t := scenarioSplits t "bind open body" F.bindOpenBody.toUTF8
  t := scenarioSplits t "recorded long poll" (joinChunks F.e2ePoll)
  -- Messages: a non-ASCII name as the server sends it (\u escapes, a surrogate pair).
  let frame := "[[7,[\"remoteConnected\",{\"name\":\"Dande TV \\u2013 pr\\u00fcfung \\ud83d\\udcfa\",\"id\":\"r1\"}]]]".toUTF8
  match parseFrame frame with
  | .ok ms =>
    t := t.check "\\uXXXX and surrogate pairs decode to UTF-8" (ms.size == 1 && ms[0]!.aid == 7 &&
      ms[0]!.getD "name" == "Dande TV – prüfung 📺")
  | .error e => t := t.check ("unicode frame parses: " ++ e) false
  let (f, _) := ({} : Framer).feed "12x\n[[1,[\"noop\"]]]\n".toUTF8
  t := t.check "a bad length line fails the framer" f.failed
  return t

/-- Scenario 4: reconnect paths. -/
def scenarioReconnect (t : T) : T := Id.run do
  let mut t := t.line "== 4. reconnect paths"
  let (s0, rs0) := bootstrap
  let poll := (tagged rs0 .poll).req
  let sid0 := s0.sid
  -- Normal end after minutes: poll again on the same session with the new AID.
  let (s, rs) := s0.onPollEnd poll.id (some 200) 300000
  t := t.reqs rs
  t := t.check "200 after 5 min: new poll, same SID" (rs.size == 1 && rs[0]!.tag == .poll &&
    contains rs[0]!.url ("SID=" ++ sid0) && rs[0]!.id != poll.id)
  let newPoll := rs[0]!.id
  t := t.check "old poll's bytes are ignored" (let (s', rs', cs') := s.onChunk poll.id "16\n[[9,[\"pause\"]]\n]\n".toUTF8 300100
    rs'.isEmpty && cs'.isEmpty && s'.aid == s.aid)
  -- Ends at once: back off instead of a busy loop.
  let (s2, rs2) := s.onPollEnd newPoll (some 200) 300200
  t := t.check "200 within 1 s: no immediate poll" rs2.isEmpty
  let (_, rs3) := s2.tick 300700
  t := t.check "tick before the backoff: nothing" (countTag rs3 .poll == 0)
  let (_, rs4) := s2.tick 301300
  t := t.check "tick after 1 s: poll" (countTag rs4 .poll == 1)
  -- Network failure.
  let (s5, rs5) := s0.onPollEnd poll.id none 5000
  t := t.check "failure: backoff, no request yet" rs5.isEmpty
  let (_, rs6) := s5.tick 6000
  t := t.check "failure: poll after 1 s" (countTag rs6 .poll == 1)
  -- Session gone: a new bind session (fresh RID, AID 0, ofs 0).
  let (s7, rs7) := s0.onPollEnd poll.id (some 400) 5000
  t := t.reqs rs7
  t := t.check "400: reopen (bind open, no SID in URL)" (rs7.size == 1 && rs7[0]!.tag == .bindOpen &&
    !contains rs7[0]!.url "SID=" && s7.aid == 0 && s7.ofs == 0 && !s7.isOnline)
  let (s8, rs8, _) := s7.onResponse rs7[0]!.id 200 F.soakBindOpenBody.toUTF8 5100
  t := t.check "reopened: online, new SID, poll + send" (s8.isOnline && s8.sid != sid0 &&
    countTag rs8 .poll == 1 && countTag rs8 .send == 1)
  let (_, rs9) := s0.onPollEnd poll.id (some 410) 5000
  t := t.check "410: reopen" (rs9.size == 1 && rs9[0]!.tag == .bindOpen)
  -- Token expired.
  let (_, rs10) := s0.onPollEnd poll.id (some 401) 5000
  t := t.reqs rs10
  t := t.check "401: new lounge token" (rs10.size == 1 && rs10[0]!.tag == .tokenBatch)
  -- Stalled poll: no bytes for 75 s -> a superseding poll.
  let (_, rs11) := s0.tick (1300 + 70000)
  t := t.check "no stall at 70 s" (countTag rs11 .poll == 0)
  let (_, rs12) := s0.tick (1300 + 76000)
  t := t.check "stall at 76 s: new poll" (countTag rs12 .poll == 1)
  -- Garbage on the back channel: re-poll.
  let (s13, rs13, _) := s0.onChunk poll.id "<html>".toUTF8 2000
  t := t.check "framing error: re-poll" (countTag rs13 .poll == 1 && s13.lastError == "framing error")
  -- A send refused with 410 reopens; 401 refreshes the token.
  let send := (tagged rs0 .send).req
  let (_, rs14, _) := s0.onResponse send.id 410 ByteArray.empty 2000
  t := t.check "send 410: reopen" (rs14.size == 1 && rs14[0]!.tag == .bindOpen)
  let (_, rs15, _) := s0.onResponse send.id 401 ByteArray.empty 2000
  t := t.check "send 401: token" (rs15.size == 1 && rs15[0]!.tag == .tokenBatch)
  -- Bind open refused: backoff, then a fresh RID.
  let (s16, rs16) := Session.start {} { ident0 with screenId := some "testscreenid00000000000000" } entropy0 1000
  t := t.check "stored screen ID: starts with the token" (rs16.size == 1 && rs16[0]!.tag == .tokenBatch)
  let (s17, rs17, _) := s16.onResponse rs16[0]!.id 200 F.tokenBatchBody.toUTF8 1100
  let rid1 := s17.rid - 1
  let (s18, rs18, _) := s17.onResponse rs17[0]!.id 503 ByteArray.empty 1200
  t := t.check "bind open 503: wait" rs18.isEmpty
  let (_, rs19) := s18.tick 2300
  t := t.check "bind open retried after 1 s with a new RID" (rs19.size == 1 && rs19[0]!.tag == .bindOpen &&
    !contains rs19[0]!.url ("RID=" ++ toString rid1 ++ "&"))
  let (_, rs20, _) := s17.onResponse rs17[0]!.id 401 ByteArray.empty 1200
  t := t.check "bind open 401: token" (rs20.size == 1 && rs20[0]!.tag == .tokenBatch)
  -- A token reply without screens: the stored screen ID is unknown; make a new one.
  let (_, rs21, _) := s16.onResponse rs16[0]!.id 200 "{\"screens\":[]}".toUTF8 1100
  t := t.check "no token for the screen ID: generate a new one" (rs21.size == 1 && rs21[0]!.tag == .generateScreenId)
  return t

/-- Scenario 5: the recorded idle soak (noops, a server-ended poll, code refresh). -/
def scenarioSoak (t : T) : T := Id.run do
  let mut t := t.line "== 5. recorded idle soak"
  let (s, rs) := Session.start {} ident0 entropy0 0
  let (s, rs, _) := s.onResponse rs[0]!.id 200 F.screenIdBody.toUTF8 0
  let (s, rs, _) := s.onResponse rs[0]!.id 200 F.tokenBatchBody.toUTF8 0
  let (s, rs, _) := s.onResponse rs[0]!.id 200 F.soakBindOpenBody.toUTF8 0
  let mut s := s
  let code := (tagged rs .pairingCode).req
  let (s', _, _) := s.onResponse code.id 200 F.pairingCodeBody.toUTF8 100
  s := s'
  let mut pollId := (tagged rs .poll).req.id
  let mut now : UInt64 := 0
  let mut cmds := 0
  let mut reqsSeen := 0
  for c in F.soakPoll1 do
    now := now + 22000
    let (s', rs, cs) := s.onChunk pollId c.toUTF8 now
    s := s'
    cmds := cmds + cs.size
    let (s', rs2) := s.tick now
    s := s'
    reqsSeen := reqsSeen + countTag (rs ++ rs2) .poll
  t := t.check "noops only: no commands, no extra polls" (cmds == 0 && reqsSeen == 0)
  let (s', rs) := s.onPollEnd pollId (some 200) 296000
  s := s'
  t := t.check "server ends the poll at 4 min 56 s: poll again" (countTag rs .poll == 1)
  pollId := (tagged rs .poll).req.id
  let (s', rs) := s.tick 300100
  s := s'
  t := t.check "5 min after the code: refresh it" (countTag rs .pairingCode == 1)
  for c in F.soakPoll2 do
    now := now + 5000
    let (s', _, cs) := s.onChunk pollId c.toUTF8 300200
    s := s'
    cmds := cmds + cs.size
  t := t.check "second poll parsed" (cmds == 0 && s.framer.mismatches == 0 && !s.framer.failed)
  return t

/-- Scenario 6: what `report` sends. -/
def scenarioReport (t : T) : T := Id.run do
  let mut t := t.line "== 6. state reports"
  let (s0, rs0) := bootstrap
  -- Acknowledge the open's replies first (one send in flight at a time).
  let (t', s0) := ackSends t s0 rs0 1300
  t := t'
  -- Nobody connected: nothing goes out.
  let (_, rs) := s0.report { videoId := some "PVGeM40dABA", state := .playing } 2000
  t := t.check "no phone: report sends nothing" rs.isEmpty
  let joined := "16\n[[5,[\"noop\"]]\n]\n" ++ "120\n[[6,[\"remoteConnected\",{\"id\":\"r1\",\"name\":\"Phone\",\"app\":\"android-phone-21.40.163\"}]]\n]\n"
  let pollId := s0.pollId
  let (s, rs, cs) := s0.onChunk pollId joined.toUTF8 3000
  t := t.reqs rs
  t := t.cmds cs
  t := t.check "remoteConnected: command + full state" (cs == #[.remoteConnected "Phone"] && countTag rs .send == 1)
  let (t', s) := ackSends t s rs 3000
  t := t'
  let p : PlayerState := { videoId := some "PVGeM40dABA", state := .buffering, currentTimeMs := 0, volume := 100 }
  let (s, rs) := s.report p 4000
  t := t.reqs rs
  let (t', s) := ackSends t s rs 4000
  t := t'
  let p := { p with state := .playing, durationMs := 140000 }
  let (s, rs) := s.report p 5000
  t := t.reqs rs
  t := t.check "buffering -> playing: onStateChange + nowPlaying" (match tagged rs .send with
    | some r => contains (str r.body) "req0__sc=onStateChange&req0_currentTime=0.000&req0_state=1&req0_duration=140.000" &&
        contains (str r.body) "req1__sc=nowPlaying&req1_videoId=PVGeM40dABA"
    | none => false)
  let (t', s) := ackSends t s rs 5000
  t := t'
  let (s, rs) := s.report { p with currentTimeMs := 10000 } 15000
  t := t.check "steady playback: nothing sent" rs.isEmpty
  let (s, rs) := s.report { p with currentTimeMs := 60000 } 16000
  t := t.check "seek (jump > 2 s): onStateChange" (match tagged rs .send with
    | some r => contains (str r.body) "count=1" && contains (str r.body) "req0__sc=onStateChange&req0_currentTime=60.000&req0_state=1"
    | none => false)
  let (t', s) := ackSends t s rs 16000
  t := t'
  let (s, rs) := s.report { p with currentTimeMs := 61000, state := .paused } 17000
  t := t.check "pause: onStateChange state 2" (match tagged rs .send with
    | some r => contains (str r.body) "req0_state=2"
    | none => false)
  let (t', s) := ackSends t s rs 17000
  t := t'
  let (s, rs) := s.report { p with currentTimeMs := 61000, state := .paused, volume := 30 } 18000
  t := t.check "volume: onVolumeChanged only" (match tagged rs .send with
    | some r => str r.body == "count=1&ofs=" ++ toString (s.ofs - 1) ++ "&req0__sc=onVolumeChanged&req0_volume=30&req0_muted=false"
    | none => false)
  -- A second report while that send is in flight waits in the outbox.
  let (s, rs) := s.report { p with currentTimeMs := 0, state := .ended, volume := 30 } 19000
  t := t.check "send in flight: queued, not sent" rs.isEmpty
  let (_, rs, _) := s.onResponse s.sendId 200 F.sendOkBody.toUTF8 19100
  t := t.check "ack flushes the queued report" (countTag rs .send == 1)
  return t

/-- Scenario 7: DIAL pairing codes, ended, stop, helpers. -/
def scenarioMisc (t : T) : T := Id.run do
  let mut t := t.line "== 7. DIAL code, ended, stop, helpers"
  -- A DIAL code before the screen ID exists waits for it.
  let (s, rs) := Session.start {} ident0 entropy0 1000
  let (s, rs1) := s.registerPairingCode "853d5d9b-5cbb-4dba-ba0f-09e9f63b57f6" 1050
  t := t.check "DIAL code before the screen ID: deferred" rs1.isEmpty
  let (s, rs2, _) := s.onResponse rs[0]!.id 200 F.screenIdBody.toUTF8 1100
  t := t.reqs rs2
  t := t.check "then registered with the screen ID" (match tagged rs2 .registerCode with
    | some r => str r.body == "access_type=permanent&app=lb-v4&pairing_code=853d5d9b-5cbb-4dba-ba0f-09e9f63b57f6&screen_id=testscreenid00000000000000&screen_name=Dande+TV+%28prototype%29&device_id=00000000-0000-4000-8000-000000000001"
    | none => false)
  let reg := (tagged rs2 .registerCode).req
  let (s, _, _) := s.onResponse reg.id 200 ByteArray.empty 1200
  t := t.check "register status kept" (s.lastRegisterStatus == some 200)
  let (_, rs3) := s.registerPairingCode "bad code&x=1" 1300
  t := t.check "malformed DIAL code rejected" rs3.isEmpty
  -- ended: next queue item.
  let (s0, rs0) := bootstrap
  let pl := "200\n[[6,[\"setPlaylist\",{\"videoId\":\"aaaaaaaaaaa\",\"videoIds\":\"aaaaaaaaaaa,bbbbbbbbbbb\",\"currentIndex\":\"0\",\"currentTime\":\"12.5\",\"listId\":\"RQx\"}]]\n]\n"
  let (s1, _, cs) := s0.onChunk (tagged rs0 .poll).req.id pl.toUTF8 2000
  t := t.check "setPlaylist: playVideo with start 12.5 s" (cs == #[.playVideo "aaaaaaaaaaa" 12500 (some "RQx")])
  let (s2, _, cs2) := s1.ended 3000
  t := t.check "ended: plays the next queue item" (cs2 == #[.playVideo "bbbbbbbbbbb" 0 (some "RQx")])
  let (_, _, cs3) := s2.ended 4000
  t := t.check "ended at the last item: nothing" cs3.isEmpty
  let (s3, _) := s1.report { videoId := some "ccccccccccc", state := .playing } 4000
  t := t.check "a video chosen on the TV: new cpn, phone queue dropped" (s3.cpn.utf8ByteSize == 16 &&
    s3.cpn != s1.cpn && s3.listId == "" && s3.queue.isEmpty)
  let (s3b, _) := s1.report { videoId := some "aaaaaaaaaaa", state := .playing } 4000
  t := t.check "the phone's own video: cpn and queue kept" (s3b.cpn == s1.cpn && s3b.listId == "RQx")
  -- stop: tell phones, end the session.
  let (s4, rs4) := s0.stop
  t := t.reqs rs4
  t := t.check "stop: loungeScreenDisconnected + terminate" (rs4.size == 2 && rs4[0]!.tag == .send &&
    contains (str rs4[0]!.body) "loungeScreenDisconnected" && rs4[1]!.tag == .terminate &&
    str rs4[1]!.body == "TYPE=terminate&ui=" && !s4.isOnline)
  let (_, rs5) := s4.tick 999999
  t := t.check "stopped: tick does nothing" rs5.isEmpty
  -- deviceInfo variant.
  let (_, rs6) := bootstrap { deviceInfo := true }
  t := t.check "deviceInfo config adds deviceInfo, obfuscatedGaiaId, cst" (match tagged rs6 .poll with
    | some r => contains r.url "deviceInfo=%7B%22brand%22%3A%22TCL%22" && contains r.url "&obfuscatedGaiaId=&cst=m"
    | none => false)
  -- Helpers.
  t := t.check "parseSecondsMs" (parseSecondsMs? "0" == some 0 && parseSecondsMs? "12.5" == some 12500 &&
    parseSecondsMs? "60" == some 60000 && parseSecondsMs? "1.2345" == some 1234 &&
    parseSecondsMs? "-1" == none && parseSecondsMs? "1e3" == none && parseSecondsMs? "" == none)
  t := t.check "secondsStr" (secondsStr 0 == "0.000" && secondsStr 12345 == "12.345" && secondsStr 60007 == "60.007")
  t := t.check "pctEncode (quote_plus)" (pctEncode "a b(c),d_e.f-g~h/é" == "a+b%28c%29%2Cd_e.f-g~h%2F%C3%A9")
  t := t.check "uuidFromBytes" (uuidFromBytes (ByteArray.mk #[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]) ==
    "00010203-0405-4607-8809-0a0b0c0d0e0f")
  t := t.check "formatCode" (formatCode "076706486798" == "076 706 486 798")
  return t

@[export zr_test_transcript]
def transcript (k : Nat) : ByteArray := Id.run do
  let t : T := { o := ByteArray.empty, pass := k, fail := 0 }
  let t := scenarioStart t
  let t := scenarioE2e t
  let t := scenarioFraming t
  let t := scenarioReconnect t
  let t := scenarioSoak t
  let t := scenarioReport t
  let t := scenarioMisc t
  let t := t.line ("== checks: " ++ toString t.pass ++ " passed, " ++ toString t.fail ++ " failed")
  return t.o

end LoungeTest
