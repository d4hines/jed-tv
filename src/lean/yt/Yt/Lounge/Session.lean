import Yt.Json.Doc
import Yt.Lounge.Frame
import Yt.Lounge.Types

/-! The screen (TV) side of the YouTube Lounge protocol as a pure state
machine: "Link with TV code", the phone's commands, and the state reports
phones show. The app performs the HTTPS requests the session emits and feeds
the results back; time and entropy are inputs, there is no IO.

Protocol (research/dial-lounge/README.md, verified live 2026-11-01 with the
host prototype, which this follows request for request):

1. `GET /api/lounge/pairing/generate_screen_id` (skipped when the identity
   already has a screen ID) -> text.
2. `POST /api/lounge/pairing/get_lounge_token_batch` `screen_ids=<id>` ->
   JSON `screens[0].loungeToken` (14-day lifetime).
3. `POST /api/lounge/bc/bind?<common>&RID=<r>&CVER=1`, body `count=0` ->
   frames with `c` (SID), `S` (gsessionid), `loungeStatus`, `getNowPlaying`,
   `getDiscoveryDeviceId`.
4. Back channel: `GET bind?<common>&RID=rpc&SID&gsessionid&CI=0&AID=<last
   array id>&TYPE=xmlhttp`, streamed; `noop` about every 30 s; the server
   ends it after about 5 minutes and we poll again with the same SID.
5. Forward channel: `POST bind?<common>&SID&gsessionid&RID=<r+1...>&AID`,
   body `count=k&ofs=<maps sent so far>&req0__sc=<name>&req0_<k>=<v>...`.
   One send in flight at a time; later maps wait in an outbox and go as one
   batch (the server expects RIDs in order).
6. 400 Unknown SID / 410 Gone: open a new session (fresh RID, AID, ofs).
   401: new lounge token, then a new session.
7. While no phone is connected: `POST get_pairing_code?ctx=pair` for the
   12-digit code, refreshed every 5 minutes. A DIAL launch's code goes to
   `register_pairing_code` (`registerPairingCode`).

<common> is `device=LOUNGE_SCREEN&id&name&app&theme&loungeIdToken&VER=8&v=2&
zx&t=1&capabilities&mdxVersion=2` (+ `deviceInfo&obfuscatedGaiaId&cst` when
`Config.deviceInfo`).

Integration (what the app's event loop does):
- `start` once; then perform every `Request` the session returns
  (`Http.request r.method url r.headers r.body`; it adds Host, Connection
  and Content-Length; the session supplies User-Agent and, for POSTs,
  Content-Type). Up to about four can be in flight: a step or the poll, one
  send, the pairing code, DIAL registrations.
- Non-streaming: `onResponse r.id status body now` (status 0 if the
  connection or TLS failed). Streaming (the poll): de-chunk the HTTP body
  (`Http.Parser`, `takeBody`) and pass each piece to `onChunk r.id bytes now`;
  then `onPollEnd r.id (some status)` when the response ends, or
  `onPollEnd r.id none` on a failure. When a new `.poll` request appears,
  close the previous poll's connection.
- `tick now` about once a second (retries, backoff, the poll restart and
  75 s stall watchdog, the 5-minute code refresh). `now` is monotonic ms.
- Commands: execute them in the player, then `report` the resulting state
  (and call `report` whenever the player changes; extra calls are cheap).
  `ended` when a video finishes. `stop` when leaving.
- Persist `identity.screenId` (after the first start) and `deviceId`
  (`uuidFromBytes` of 16 random bytes on first install); keep `name` stable.
- Entropy: a few bytes from getrandom; only nonces (zx, RID, cpn) use it. -/
namespace Yt.Lounge
open Yt.Json Yt.Fmt

def apiBase : String := "https://www.youtube.com/api/lounge"

/-- A forward-channel map: name and fields. -/
abbrev Map := String × Array (String × String)

inductive Phase where
  | screenId | token | opening | online
  deriving Inhabited, BEq

structure Session where
  cfg : Config
  ident : Identity
  rng : UInt64
  now : UInt64 := 0
  nextId : Nat := 1
  token : String := ""
  phase : Phase := .screenId
  /-- The outstanding generate/token/open request (0: none). -/
  stepId : Nat := 0
  /-- When to (re)issue the current step while `stepId` is 0. -/
  stepDue : UInt64 := 0
  failures : Nat := 0
  sid : String := ""
  gsid : String := ""
  rid : Nat := 0
  /-- Highest array id received on this session. -/
  aid : Nat := 0
  /-- Maps sent on this session. -/
  ofs : Nat := 0
  framer : Framer := {}
  pollId : Nat := 0
  pollDue : UInt64 := 0
  pollStartedAt : UInt64 := 0
  pollFailures : Nat := 0
  lastByteAt : UInt64 := 0
  sendId : Nat := 0
  outbox : Array Map := #[]
  codeId : Nat := 0
  code : Option String := none
  codeDue : UInt64 := 0
  registerIds : Array Nat := #[]
  pendingRegister : Array String := #[]
  lastRegisterStatus : Option Nat := none
  /-- Connected phones: (id, name). -/
  remotes : Array (String × String) := #[]
  /-- The app's player as last reported (or set provisionally on load). -/
  player : PlayerState := {}
  playerAt : UInt64 := 0
  /-- The position/state baseline phones were last told. -/
  sent : PlayerState := {}
  sentAt : UInt64 := 0
  queue : Array String := #[]
  index : Nat := 0
  listId : String := ""
  ctt : String := ""
  params : String := ""
  cpn : String := ""
  error : String := ""
  deriving Inhabited

/-! ## Small pieces -/

@[inline] def mix64 (x : UInt64) : UInt64 :=
  let z := (x ^^^ (x >>> 30)) * 0xBF58476D1CE4E5B9
  let z := (z ^^^ (z >>> 27)) * 0x94D049BB133111EB
  z ^^^ (z >>> 31)

/-- A splitmix64 seed from arbitrary entropy bytes (FNV-1a fold, then mix). -/
def seedOf (e : ByteArray) : UInt64 := Id.run do
  let mut h : UInt64 := 0xCBF29CE484222325
  for b in e do
    h := (h ^^^ b.toUInt64) * 0x100000001B3
  return mix64 h

def hasId (ids : Array Nat) (id : Nat) : Bool := Id.run do
  for x in ids do
    if x == id then return true
  return false

def removeId (ids : Array Nat) (id : Nat) : Array Nat := Id.run do
  let mut out := #[]
  for x in ids do
    if x != id then out := out.push x
  return out

def boolStr (b : Bool) : String := if b then "true" else "false"

/-- Exponential backoff: 1 s, 2 s, 4 s ... capped at 60 s. -/
def backoff (n : Nat) : UInt64 :=
  let d : UInt64 := (1000 : UInt64) <<< (min n 6).toUInt64
  if d > 60000 then 60000 else d

/-- JSON string literal for simple ASCII (escapes `"` and `\`). -/
def jsonStr (s : String) : String := Id.run do
  let mut o := ByteArray.empty.push 34
  for c in s.toUTF8 do
    if c == 34 || c == 92 then o := o.push 92
    if c ≥ 32 then o := o.push c
  return str (o.push 34)

namespace Session

def rand (s : Session) : Session × UInt64 :=
  let st := s.rng + 0x9E3779B97F4A7C15
  ({ s with rng := st }, mix64 st)

/-- `n` random characters from `[a-z0-9]` (zx, cpn). -/
def randStr (s : Session) (n : Nat) : Session × String := Id.run do
  let alpha := "abcdefghijklmnopqrstuvwxyz0123456789".toUTF8
  let mut s := s
  let mut o := ByteArray.emptyWithCapacity n
  for _ in [0:n] do
    let (s', r) := s.rand
    s := s'
    o := o.push (byteAt alpha (r % 36).toNat)
  return (s, str o)

/-- A request with a fresh id and the standard headers. -/
def mkReq (s : Session) (tag : Tag) (method url body : String) (streaming : Bool := false) :
    Session × Request :=
  let id := s.nextId
  let headers := if method == "POST" then
      #[("User-Agent", s.cfg.userAgent), ("Content-Type", "application/x-www-form-urlencoded")]
    else #[("User-Agent", s.cfg.userAgent)]
  ({ s with nextId := id + 1 }, { id, tag, method, url, headers, body := body.toUTF8, streaming })

def deviceInfoJson (c : Config) : String :=
  "{\"brand\":" ++ jsonStr c.brand ++ ",\"model\":" ++ jsonStr c.model ++
  ",\"year\":0,\"os\":\"Linux\",\"osVersion\":\"\",\"chipset\":\"\",\"clientName\":\"TVHTML5\"," ++
  "\"dialAdditionalDataSupportLevel\":\"supported\",\"mdxDialServerType\":\"MDX_DIAL_SERVER_TYPE_UNKNOWN\"}"

/-- Query parameters on every bind request (the prototype's order). -/
def common (s : Session) : Session × Array (String × String) :=
  let (s, z) := s.randStr 12
  let q := #[("device", "LOUNGE_SCREEN"), ("id", s.ident.deviceId), ("name", s.ident.name),
    ("app", s.cfg.app), ("theme", s.cfg.theme), ("loungeIdToken", s.token), ("VER", "8"), ("v", "2"),
    ("zx", z), ("t", "1")]
  let q := if s.cfg.capabilities.isEmpty then q else q.push ("capabilities", s.cfg.capabilities)
  let q := q.push ("mdxVersion", "2")
  let q := if s.cfg.deviceInfo then
      q ++ #[("deviceInfo", deviceInfoJson s.cfg), ("obfuscatedGaiaId", ""), ("cst", "m")]
    else q
  (s, q)

def bindUrl (q : Array (String × String)) : String := apiBase ++ "/bc/bind?" ++ formEncode q

def screenIdStr (s : Session) : String := s.ident.screenId.getD ""

/-! ## Requests -/

def reqOpen (s : Session) : Session × Request :=
  let (s, q) := s.common
  let q := q ++ #[("RID", natStr s.rid), ("CVER", "1")]
  let s := { s with rid := s.rid + 1 }
  s.mkReq .bindOpen "POST" (bindUrl q) "count=0"

def reqPoll (s : Session) : Session × Request :=
  let (s, q) := s.common
  let q := q ++ #[("RID", "rpc"), ("SID", s.sid), ("gsessionid", s.gsid), ("CI", "0"),
    ("AID", natStr s.aid), ("TYPE", "xmlhttp")]
  s.mkReq .poll "GET" (bindUrl q) "" (streaming := true)

def sendBody (ofs : Nat) (maps : Array Map) : String := Id.run do
  let mut f : Array (String × String) := #[("count", natStr maps.size), ("ofs", natStr ofs)]
  let mut i := 0
  for (name, fields) in maps do
    let p := "req" ++ natStr i ++ "_"
    f := f.push (p ++ "_sc", name)
    for (k, v) in fields do
      f := f.push (p ++ k, v)
    i := i + 1
  return formEncode f

def reqSend (s : Session) (maps : Array Map) : Session × Request :=
  let (s, q) := s.common
  let q := q ++ #[("SID", s.sid), ("gsessionid", s.gsid), ("RID", natStr s.rid), ("AID", natStr s.aid)]
  let body := sendBody s.ofs maps
  let s := { s with rid := s.rid + 1, ofs := s.ofs + maps.size }
  s.mkReq .send "POST" (bindUrl q) body

def reqTerminate (s : Session) : Session × Request :=
  let (s, q) := s.common
  let q := q ++ #[("SID", s.sid), ("gsessionid", s.gsid), ("RID", natStr s.rid), ("AID", natStr s.aid),
    ("CVER", "1")]
  let s := { s with rid := s.rid + 1 }
  s.mkReq .terminate "POST" (bindUrl q) (formEncode #[("TYPE", "terminate"), ("ui", "")])

def reqCode (s : Session) : Session × Request :=
  s.mkReq .pairingCode "POST" (apiBase ++ "/pairing/get_pairing_code?ctx=pair") (formEncode #[
    ("access_type", "permanent"), ("app", s.cfg.app), ("lounge_token", s.token),
    ("screen_id", s.screenIdStr), ("screen_name", s.ident.name), ("device_id", s.ident.deviceId)])

def reqRegister (s : Session) (code : String) : Session × Request :=
  s.mkReq .registerCode "POST" (apiBase ++ "/pairing/register_pairing_code") (formEncode #[
    ("access_type", "permanent"), ("app", s.cfg.app), ("pairing_code", code),
    ("screen_id", s.screenIdStr), ("screen_name", s.ident.name), ("device_id", s.ident.deviceId)])

/-! ## Steps: screen ID -> token -> bind session -/

/-- Issue the request for the current phase (a new bind session when opening). -/
def issueStep (s : Session) : Session × Array Request :=
  match s.phase with
  | .screenId =>
    let (s, r) := s.mkReq .generateScreenId "GET" (apiBase ++ "/pairing/generate_screen_id") ""
    ({ s with stepId := r.id }, #[r])
  | .token =>
    let (s, r) := s.mkReq .tokenBatch "POST" (apiBase ++ "/pairing/get_lounge_token_batch")
      (formEncode #[("screen_ids", s.screenIdStr)])
    ({ s with stepId := r.id }, #[r])
  | .opening =>
    let (s, x) := s.rand
    let s := { s with rid := 10000 + (x % 80000).toNat, aid := 0, ofs := 0, sid := "", gsid := "",
                      framer := {}, pollId := 0, sendId := 0 }
    let (s, r) := s.reqOpen
    ({ s with stepId := r.id }, #[r])
  | .online => (s, #[])

def failStep (s : Session) (why : String) : Session :=
  let n := s.failures + 1
  { s with stepId := 0, failures := n, stepDue := s.now + backoff (n - 1), error := why }

/-- Start over at `p` (dropping the current bind session). -/
def goPhase (s : Session) (p : Phase) (why : String) : Session × Array Request :=
  let s := { s with phase := p, stepId := 0, pollId := 0, sendId := 0, sid := "", gsid := "",
                    framer := {}, error := why }
  s.issueStep

/-- Registration requests for DIAL codes received before the screen ID. -/
def issueRegistrations (s : Session) : Session × Array Request := Id.run do
  if s.ident.screenId.isNone then return (s, #[])
  let mut s := s
  let mut out := #[]
  for c in s.pendingRegister do
    let (s', r) := s.reqRegister c
    s := { s' with registerIds := s'.registerIds.push r.id }
    out := out.push r
  return ({ s with pendingRegister := #[] }, out)

/-- Send the outbox if the session is open and no send is in flight. -/
def flush (s : Session) : Session × Array Request :=
  if s.phase != .online || s.sendId != 0 || s.outbox.isEmpty then (s, #[]) else
  let (s, r) := s.reqSend s.outbox
  ({ s with sendId := r.id, outbox := #[] }, #[r])

def issuePoll (s : Session) : Session × Array Request :=
  let (s, r) := s.reqPoll
  ({ s with pollId := r.id, framer := {}, pollStartedAt := s.now, lastByteAt := s.now }, #[r])

def wantsCode (s : Session) : Bool :=
  s.cfg.showCode && s.phase == .online && s.remotes.isEmpty && s.codeId == 0 && s.now ≥ s.codeDue

def maybeCode (s : Session) : Session × Array Request :=
  if !s.wantsCode then (s, #[]) else
  let (s, r) := s.reqCode
  ({ s with codeId := r.id }, #[r])

/-! ## State reports -/

/-- The player now, extrapolating the position while playing. -/
def current (s : Session) : PlayerState :=
  let p := s.player
  if p.state != .playing || s.now ≤ s.playerAt then p else
  let t := p.currentTimeMs + (s.now - s.playerAt)
  { p with currentTimeMs := if p.durationMs > 0 && t > p.durationMs then p.durationMs else t }

def timeFields (p : PlayerState) (cpn : String) : Array (String × String) :=
  let dur := p.durationMs
  let loaded := if p.loadedMs > 0 then p.loadedMs else dur
  let f := #[("currentTime", secondsStr p.currentTimeMs), ("state", p.state.code)]
  let f := if dur > 0 then f.push ("duration", secondsStr dur) else f
  let f := f.push ("loadedTime", secondsStr loaded)
  let f := f.push ("seekableStartTime", "0")
  let f := if dur > 0 then f.push ("seekableEndTime", secondsStr dur) else f
  f.push ("cpn", cpn)

def nowPlayingMap (s : Session) (p : PlayerState) : Map :=
  match p.videoId with
  | none => ("nowPlaying", #[])
  | some v =>
    let f := #[("videoId", v)] ++ timeFields p s.cpn
    let f := if s.listId.isEmpty then f else f ++ #[("listId", s.listId), ("currentIndex", natStr s.index)]
    let f := if s.ctt.isEmpty then f else f.push ("ctt", s.ctt)
    let f := if s.params.isEmpty then f else f.push ("params", s.params)
    ("nowPlaying", f)

def stateMap (s : Session) (p : PlayerState) : Map := ("onStateChange", timeFields p s.cpn)

def volumeMap (p : PlayerState) : Map :=
  ("onVolumeChanged", #[("volume", natStr p.volume), ("muted", boolStr p.muted)])

def prevNextMap (s : Session) : Map :=
  let has := !s.queue.isEmpty
  ("onHasPreviousNextChanged", #[("hasPrevious", boolStr (has && s.index > 0)),
    ("hasNext", boolStr (has && s.index + 1 < s.queue.size))])

def autoplayMap : Map := ("onAutoplayModeChanged", #[("autoplayMode", "UNSUPPORTED")])

def queueMaps (s : Session) (ms : Array Map) : Session := { s with outbox := s.outbox ++ ms }

/-- Tell the phones the full current state and make it the baseline. -/
def fullState (s : Session) (withState : Bool) : Session :=
  let p := s.current
  let ms := #[s.prevNextMap, s.nowPlayingMap p]
  let ms := if withState && p.videoId.isSome then ms.push (s.stateMap p) else ms
  let ms := if withState then ms ++ #[volumeMap p, autoplayMap] else ms.push autoplayMap
  { (s.queueMaps ms) with sent := p, sentAt := s.now }

/-! ## Incoming messages -/

def findRemote (rs : Array (String × String)) (id : String) : Option String := Id.run do
  for (i, n) in rs do
    if i == id then return some n
  return none

/-- Start `vid` at `startMs` (setPlaylist, setVideo, next, previous). -/
def load (s : Session) (vid : String) (startMs : UInt64) : Session × Array Command :=
  if !isToken vid 64 then (s, #[]) else
  let (s, cpn) := s.randStr 16
  let p := { s.player with
    videoId := some vid, state := .buffering, currentTimeMs := startMs, durationMs := 0, loadedMs := 0 }
  let s := { s with cpn, player := p, playerAt := s.now }
  let s := s.queueMaps #[s.nowPlayingMap p, s.stateMap p, s.prevNextMap]
  ({ s with sent := p, sentAt := s.now },
   #[.playVideo vid startMs (if s.listId.isEmpty then none else some s.listId)])

def indexOf (a : Array String) (x : String) : Option Nat := Id.run do
  for i in [0:a.size] do
    if a[i]! == x then return some i
  return none

def step (s : Session) (forward : Bool) : Session × Array Command :=
  if s.queue.isEmpty then (s, #[]) else
  if forward then
    if s.index + 1 < s.queue.size then
      let s := { s with index := s.index + 1 }
      s.load s.queue[s.index]! 0
    else (s, #[])
  else if s.index > 0 then
    let s := { s with index := s.index - 1 }
    s.load s.queue[s.index]! 0
  else (s, #[])

/-- Remotes listed in a `loungeStatus` `devices` JSON string. -/
def remotesOf (devices : String) : Array (String × String) := Id.run do
  match Doc.parse devices.toUTF8 with
  | .error _ => return #[]
  | .ok d =>
    let mut out := #[]
    for e in d.elems d.root do
      let ty := (d.get? e "type").bind d.str?
      if ty == some "REMOTE_CONTROL" then
        let id := ((d.get? e "id").bind d.str?).getD ""
        let name := ((d.get? e "name").bind d.str?).getD ""
        if !id.isEmpty then out := out.push (id, name)
    return out

def handle (s : Session) (m : Msg) : Session × Array Command :=
  let s := { s with aid := max s.aid m.aid }
  match m.name with
  | "c" => ({ s with sid := m.args[0]?.getD s.sid }, #[])
  | "S" => ({ s with gsid := m.args[0]?.getD s.gsid }, #[])
  | "loungeStatus" => Id.run do
    let rs := remotesOf (m.getD "devices" "[]")
    let mut cmds := #[]
    for (i, n) in rs do
      if (findRemote s.remotes i).isNone then cmds := cmds.push (Command.remoteConnected n)
    for (i, n) in s.remotes do
      if (findRemote rs i).isNone then cmds := cmds.push (Command.remoteDisconnected n)
    let s := { s with remotes := rs }
    return (if rs.isEmpty then s else s.fullState false, cmds)
  | "remoteConnected" =>
    let id := m.getD "id"
    let name := m.getD "name"
    let isNew := (findRemote s.remotes id).isNone && !id.isEmpty
    let s := if isNew then { s with remotes := s.remotes.push (id, name) } else s
    (s.fullState true, if isNew then #[.remoteConnected name] else #[])
  | "remoteDisconnected" =>
    let id := m.getD "id"
    match findRemote s.remotes id with
    | some n => ({ s with remotes := s.remotes.filter (fun r => r.1 != id) }, #[.remoteDisconnected n])
    | none => (s, #[])
  | "getNowPlaying" => (s.queueMaps #[s.nowPlayingMap s.current], #[])
  | "getVolume" => (s.queueMaps #[volumeMap s.player], #[])
  | "getDiscoveryDeviceId" =>
    (s.queueMaps #[("setDiscoveryDeviceId", #[("discoveryDeviceId", s.ident.deviceId),
      ("loungeDeviceId", s.ident.deviceId)])], #[])
  | "getSubtitlesTrack" =>
    (s.queueMaps #[("onSubtitlesTrackChanged", match s.player.videoId with
      | some v => #[("videoId", v)] | none => #[])], #[])
  | "setAutoplayMode" => (s.queueMaps #[autoplayMap], #[])
  | "setPlaylist" | "setVideo" =>
    let vid := m.getD "videoId"
    let ids := splitOn (m.getD "videoIds" vid) 44
    let idx := match parseNat? (m.getD "currentIndex") with
      | some i => if i < ids.size then i else 0
      | none => (indexOf ids vid).getD 0
    let start := (parseSecondsMs? (m.getD "currentTime" "0")).getD 0
    let s := { s with queue := ids, index := idx, listId := m.getD "listId", ctt := m.getD "ctt",
                      params := m.getD "params" }
    s.load vid start
  | "updatePlaylist" =>
    let ids := splitOn (m.getD "videoIds") 44
    let s := { s with queue := ids, listId := (m.get? "listId").getD s.listId }
    let s := match s.player.videoId.bind (indexOf ids) with
      | some i => { s with index := i } | none => s
    (s.queueMaps #[s.nowPlayingMap s.current, s.prevNextMap], #[])
  | "play" => (s, #[.play])
  | "pause" => (s, #[.pause])
  | "stopVideo" => (s, #[.stop])
  | "seekTo" =>
    match parseSecondsMs? (m.getD "newTime") with
    | some t => (s, #[.seek t])
    | none => (s, #[])
  | "next" => s.step true
  | "previous" => s.step false
  | "setVolume" =>
    let v := (parseSecondsMs? (m.getD "volume")).map (fun x => (x / 1000).toNat)
    match v with
    | some vol => (s, #[.setVolume (min vol 100) (m.getD "muted" "false" == "true")])
    | none => (s, #[])
  | "dpadCommand" => (s, #[.dpad (m.getD "key")])
  | _ => (s, #[])

def handleAll (s : Session) (ms : Array Msg) : Session × Array Command := Id.run do
  let mut s := s
  let mut cmds := #[]
  for m in ms do
    let (s', c) := s.handle m
    s := s'
    cmds := cmds ++ c
  return (s, cmds)

/-! ## The driver interface -/

/-- Begin: the first request(s). `entropy` seeds the request nonces (zx, RID,
cpn); a few bytes from getrandom are plenty. An invalid stored screen ID is
treated as absent. -/
def start (cfg : Config) (ident : Identity) (entropy : ByteArray) (nowMs : UInt64) : Session × Array Request :=
  let ident := match ident.screenId with
    | some id => if isToken id 64 then ident else { ident with screenId := none }
    | none => ident
  let s : Session := { cfg, ident, rng := seedOf entropy, now := nowMs,
                       phase := if ident.screenId.isSome then .token else .screenId }
  s.issueStep

def onStep (s : Session) (status : Nat) (body : ByteArray) : Session × Array Request × Array Command :=
  match s.phase with
  | .screenId =>
    let id := trimmed body
    if status == 200 && isToken id 64 then
      let s := { s with ident := { s.ident with screenId := some id }, phase := .token, failures := 0 }
      let (s, rs) := s.issueStep
      let (s, rs2) := s.issueRegistrations
      (s, rs ++ rs2, #[])
    else (s.failStep ("generate_screen_id: HTTP " ++ natStr status), #[], #[])
  | .token =>
    if status != 200 then (s.failStep ("get_lounge_token_batch: HTTP " ++ natStr status), #[], #[]) else
    match Doc.parse body with
    | .error e => (s.failStep e, #[], #[])
    | .ok d =>
      let tok := ((d.path? d.root #["screens"]).bind (d.at? · 0)) |>.bind (d.get? · "loungeToken")
        |>.bind d.str?
      match tok with
      | some t =>
        if t.isEmpty then (s.failStep "empty lounge token", #[], #[]) else
        let (s, rs) := { s with token := t, phase := .opening, failures := 0 }.issueStep
        (s, rs, #[])
      | none =>
        -- No token for this screen ID: it is unknown to YouTube; make a new one.
        let (s, rs) := { s.failStep "no lounge token for this screen ID" with
          ident := { s.ident with screenId := none }, phase := .screenId, stepDue := s.now }.issueStep
        (s, rs, #[])
  | .opening =>
    if status == 401 then
      let (s, rs) := s.goPhase .token "bind: 401"
      (s, rs, #[])
    else if status != 200 then (s.failStep ("bind open: HTTP " ++ natStr status), #[], #[]) else
    match parseBody body with
    | .error e => (s.failStep e, #[], #[])
    | .ok ms =>
      let (s, cmds) := s.handleAll ms
      if s.sid.isEmpty || s.gsid.isEmpty then (s.failStep "bind open: no SID", #[], cmds) else
      let s := { s with phase := .online, stepId := 0, failures := 0, pollFailures := 0, error := "" }
      let (s, r1) := s.issuePoll
      let (s, r2) := s.maybeCode
      let (s, r3) := s.flush
      (s, r1 ++ r2 ++ r3, cmds)
  | .online => (s, #[], #[])

/-- A non-streaming request finished: its HTTP status (0 if the connection
or TLS failed before a response) and body. -/
def onResponse (s : Session) (id : Nat) (status : Nat) (body : ByteArray) (nowMs : UInt64) :
    Session × Array Request × Array Command :=
  let s := { s with now := max s.now nowMs }
  if id == 0 then (s, #[], #[])
  else if id == s.stepId then s.onStep status body
  else if id == s.sendId then
    let s := { s with sendId := 0 }
    if status == 200 then let (s, rs) := s.flush; (s, rs, #[])
    else if status == 400 || status == 410 then
      let (s, rs) := s.goPhase .opening ("send: HTTP " ++ natStr status); (s, rs, #[])
    else if status == 401 then let (s, rs) := s.goPhase .token "send: 401"; (s, rs, #[])
    else
      -- Lost batch (network): drop it; phones get full state on reconnect.
      let (s, rs) := { s with error := "send: HTTP " ++ natStr status }.flush
      (s, rs, #[])
  else if id == s.codeId then
    let c := trimmed body
    if status == 200 && c.utf8ByteSize == 12 && (parseNat? c).isSome then
      ({ s with codeId := 0, code := some c, codeDue := s.now + s.cfg.codeRefreshMs }, #[], #[])
    else ({ s with codeId := 0, codeDue := s.now + 30000, error := "get_pairing_code: HTTP " ++ natStr status },
          #[], #[])
  else if hasId s.registerIds id then
    ({ s with registerIds := removeId s.registerIds id, lastRegisterStatus := some status }, #[], #[])
  else (s, #[], #[])

/-- Re-poll after a broken back channel (framing or JSON error). -/
def repoll (s : Session) (why : String) : Session × Array Request :=
  { s with error := why }.issuePoll

/-- Bytes of the current long poll's body (after HTTP de-chunking), in any
pieces. Bytes of a poll that is no longer current are ignored. -/
def onChunk (s : Session) (id : Nat) (data : ByteArray) (nowMs : UInt64) :
    Session × Array Request × Array Command :=
  if id == 0 || id != s.pollId then (s, #[], #[]) else
  let s := { s with now := max s.now nowMs, lastByteAt := max s.now nowMs }
  let (fr, frames) := s.framer.feed data
  let s := { s with framer := fr }
  let rec go (s : Session) (i : Nat) (cmds : Array Command) : Except String (Session × Array Command) :=
    if h : i < frames.size then
      match parseFrame frames[i] with
      | .error e => .error e
      | .ok ms => let (s, c) := s.handleAll ms; go s (i + 1) (cmds ++ c)
    else .ok (s, cmds)
  termination_by frames.size - i
  match go s 0 #[] with
  | .error e => let (s, rs) := s.repoll e; (s, rs, #[])
  | .ok (s, cmds) =>
    if s.framer.failed then let (s, rs) := s.repoll "framing error"; (s, rs, cmds) else
    let (s, rs) := s.flush
    (s, rs, cmds)

/-- The current long poll ended: `some status` when the response completed
(200: normal, about every 5 minutes) or was refused, `none` on a network
failure or timeout. -/
def onPollEnd (s : Session) (id : Nat) (status : Option Nat) (nowMs : UInt64) : Session × Array Request :=
  if id == 0 || id != s.pollId then (s, #[]) else
  let s := { s with now := max s.now nowMs, pollId := 0, framer := {} }
  match status with
  | some 400 | some 410 => s.goPhase .opening "poll: session gone"
  | some 401 => s.goPhase .token "poll: 401"
  | some 200 =>
    -- A response that ends at once would make a busy loop: back off then.
    if s.now < s.pollStartedAt + (1000 : UInt64) then
      let n := s.pollFailures + 1
      ({ s with pollFailures := n, pollDue := s.now + backoff (n - 1) }, #[])
    else { s with pollFailures := 0 }.issuePoll
  | some st =>
    let n := s.pollFailures + 1
    ({ s with pollFailures := n, pollDue := s.now + backoff (n - 1), error := "poll: HTTP " ++ natStr st }, #[])
  | none =>
    let n := s.pollFailures + 1
    ({ s with pollFailures := n, pollDue := s.now + backoff (n - 1), error := "poll: connection failed" }, #[])

/-- Timers: step retries, the poll (re)start and stall watchdog, and the
pairing code. Call it about once a second. -/
def tick (s : Session) (nowMs : UInt64) : Session × Array Request :=
  let s := { s with now := max s.now nowMs }
  if s.phase != .online then
    if s.stepId == 0 && s.now ≥ s.stepDue then s.issueStep else (s, #[])
  else
    let (s, r1) :=
      if s.pollId == 0 && s.now ≥ s.pollDue then s.issuePoll
      else if s.pollId != 0 && s.now - s.lastByteAt > s.cfg.pollStallMs then s.repoll "poll stalled"
      else (s, #[])
    let (s, r2) := s.maybeCode
    let (s, r3) := s.flush
    (s, r1 ++ r2 ++ r3)

/-- The app's player state. Call it on every change (load, play, pause, seek,
end, volume) and freely in between: it sends only what phones need
(`onStateChange` on state changes and jumps of more than 2 s from the
extrapolated position, `nowPlaying` on a new video or when leaving a
non-playing state, `onVolumeChanged` on volume changes), and nothing while
no phone is connected. -/
def report (s : Session) (p : PlayerState) (nowMs : UInt64) : Session × Array Request :=
  let s := { s with now := max s.now nowMs }
  -- A video the phone did not start (chosen on the TV): new playback nonce,
  -- and the phone's queue no longer describes it.
  let s := if p.videoId == s.player.videoId && !s.cpn.isEmpty then s else
    let (s, cpn) := s.randStr 16
    let inQueue := match p.videoId with
      | some v => (indexOf s.queue v).isSome
      | none => false
    if inQueue || p.videoId.isNone then { s with cpn }
    else { s with cpn, queue := #[], index := 0, listId := "", ctt := "", params := "" }
  let s := { s with player := p, playerAt := s.now }
  if s.remotes.isEmpty then ({ s with sent := p, sentAt := s.now }, #[]) else
  let b := s.sent
  let expected := b.currentTimeMs + (if b.state == .playing then s.now - s.sentAt else 0)
  let jump := if p.currentTimeMs > expected then p.currentTimeMs - expected > 2000
    else expected - p.currentTimeMs > 2000
  let newVideo := p.videoId != b.videoId
  let stateChanged := p.state != b.state || p.durationMs != b.durationMs
  let ms : Array Map := #[]
  let ms := if newVideo || stateChanged || jump then ms.push (s.stateMap p) else ms
  let ms := if newVideo || (stateChanged && b.state != .playing) then ms.push (s.nowPlayingMap p) else ms
  let ms := if p.volume != b.volume || p.muted != b.muted then ms.push (volumeMap p) else ms
  if ms.isEmpty then (s, #[]) else
  let s := { (s.queueMaps ms) with sent := p, sentAt := s.now }
  s.flush

/-- The video ended: play the next queue item, if any (phones expect the
screen to advance through their queue). -/
def ended (s : Session) (nowMs : UInt64) : Session × Array Request × Array Command :=
  let s := { s with now := max s.now nowMs }
  let (s, cmds) := s.step true
  let (s, rs) := s.flush
  (s, rs, cmds)

/-- A DIAL launch's `pairingCode` (from PID 1): attach it to this screen so
the phone can join. Sent at once if the screen ID is known, else after it. -/
def registerPairingCode (s : Session) (code : String) (nowMs : UInt64) : Session × Array Request :=
  let s := { s with now := max s.now nowMs }
  if !isToken code 64 then ({ s with error := "bad DIAL pairing code" }, #[]) else
  { s with pendingRegister := s.pendingRegister.push code }.issueRegistrations

/-- Leaving (Home, exit): tell phones the screen is gone and end the session.
Fire and forget; responses may be ignored. -/
def stop (s : Session) : Session × Array Request :=
  if s.phase != .online then (s, #[]) else
  let (s, r1) := s.reqSend (s.outbox.push ("loungeScreenDisconnected", #[]))
  let (s, r2) := s.reqTerminate
  ({ s with outbox := #[], phase := .opening, stepId := 0, stepDue := 0xFFFFFFFFFFFFFFFF }, #[r1, r2])

/-! ## For the UI -/

/-- The 12-digit "Link with TV code" (format with `formatCode`). -/
def pairingCode (s : Session) : Option String := s.code
def remoteNames (s : Session) : Array String := s.remotes.map (·.2)
def isOnline (s : Session) : Bool := s.phase == .online
/-- The identity to persist (the screen ID appears after the first start). -/
def identity (s : Session) : Identity := s.ident
def lastError (s : Session) : String := s.error

end Session

/-- `123456789012` -> `123 456 789 012`. -/
def formatCode (c : String) : String := Id.run do
  let b := c.toUTF8
  let mut o := ByteArray.emptyWithCapacity (b.size + 4)
  for i in [0:b.size] do
    if i > 0 && i % 3 == 0 then o := o.push 32
    o := o.push (byteAt b i)
  return str o

end Yt.Lounge
