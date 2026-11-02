import Yt.App.View
import Yt.App.Player
import Yt.Net.Fetch
import Yt.Json.YouTube
import Yt.Jpeg.Decode
import Yt.Lounge.Session

/-! The YouTube app's entry point and event loop: remote keys and (later)
network results become `Event`s for `update`; effects are carried out here;
the screen is redrawn only when the model changed. -/
namespace Yt.App
open Yt.Platform Yt.Platform.Tv Yt.Ui

/-- Draw `m` into a fresh frame and publish it. -/
def render (d : Display) (f : Fonts) (m : Model) : BaseIO Unit := do
  match ← d.acquire with
  | .ok () =>
    draw d f m
    let _ ← d.present
  | .error e => Sys.log ("ytlean: acquire failed, errno " ++ toString e)

/-- Wait `ms` milliseconds (poll(2) with no descriptors). -/
def sleepMs (ms : Nat) : BaseIO Unit := do let _ ← Sys.poll .empty ms.toUSize

/-- `s` starts with `p` (bytewise). -/
def hasPrefix (s p : String) : Bool :=
  let b := s.toUTF8
  let q := p.toUTF8
  b.size ≥ q.size && b.extract 0 q.size == q

/-- What a request in flight is for. -/
inductive Pending where
  | search (q : String)
  | thumb (videoId : String)
  | player (item : Item) (withVisitor : Bool)
  /-- A search result's playability check (the same /player request). -/
  | check (item : Item) (withVisitor : Bool)
  /-- The player's ranged download: track (0 video, 1 audio), its
  generation, and fragment (none: the track header). -/
  | media (track gen : Nat) (seg : Option Nat)
  /-- A Lounge (phone link) request: its session request id, and whether it
  is the streamed long poll. -/
  | lounge (rid : Nat) (streaming : Bool)

/-- The effect runner's state: requests in flight and queued thumbnails. -/
structure Runner where
  cfg : Config
  fetcher : Net.Fetcher
  pending : Array (Nat × Pending) := #[]
  thumbQueue : Array String := #[]
  thumbsActive : Nat := 0
  /-- Search results waiting for their playability check. -/
  checkQueue : Array Item := #[]
  checksActive : Nat := 0
  player : Option Player.State := none
  /-- The player's state changed in a way that needs a redraw. -/
  playerDirty : Bool := false
  /-- The phone link ("Link with TV code"), when the app can reach YouTube. -/
  lounge : Option Lounge.Session := none
  /-- Where the next player should start (ms): from a phone's request. -/
  castStartMs : UInt64 := 0
  lastTick : UInt64 := 0
  lastReport : UInt64 := 0
  lastCast : Option String × Array String := (none, #[])
  /-- The phone's volume slider (applied as in-app attenuation). -/
  castVolume : Nat := 100
  castMuted : Bool := false

def maxThumbs : Nat := 4
def maxChecks : Nat := 4

def jsonHeaders : Array (String × String) :=
  #[("Content-Type", "application/json"), ("User-Agent", Json.YouTube.userAgent)]

/-- Thumbnails come from the test server when it is plain http (QEMU), else
from YouTube's image host. -/
def thumbUrl (c : Config) (id : String) : String :=
  (if hasPrefix c.server "http://" then c.server else "https://i.ytimg.com") ++ "/vi/" ++ id ++ "/mqdefault.jpg"

def Runner.request (r : Runner) (method url : String) (headers : Array (String × String))
    (body : ByteArray) (why : Pending) (streaming : Bool := false) : BaseIO (Runner × Option String) := do
  let (f, res) ← r.fetcher.start method url headers body streaming
  let r := { r with fetcher := f }
  match res with
  | .ok id => return ({ r with pending := r.pending.push (id, why) }, none)
  | .error e => return (r, some e)

def Runner.startPlayer (r : Runner) (it : Item) (visitor : Option String) : BaseIO (Runner × Array Event) := do
  let (r, err) ← r.request "POST" (r.cfg.server ++ "/youtubei/v1/player?prettyPrint=false") jsonHeaders
    (Json.YouTube.playerBody it.id visitor) (.player it visitor.isSome)
  return (r, match err with | some e => #[.playerDone ("Could not open: " ++ e)] | none => #[])

def Runner.startCheck (r : Runner) (it : Item) (visitor : Option String) : BaseIO (Runner × Array Event) := do
  let (r, err) ← r.request "POST" (r.cfg.server ++ "/youtubei/v1/player?prettyPrint=false") jsonHeaders
    (Json.YouTube.playerBody it.id visitor) (.check it visitor.isSome)
  -- a check that cannot start shows the result: playing it reports any problem
  return match err with
    | some _ => (r, #[.checked it.id true])
    | none => ({ r with checksActive := r.checksActive + 1 }, #[])

/-- Start queued playability checks up to the limit. -/
partial def Runner.pumpChecks (r : Runner) (evs : Array Event := #[]) : BaseIO (Runner × Array Event) := do
  if r.checksActive ≥ maxChecks then return (r, evs)
  let some it := r.checkQueue[0]? | return (r, evs)
  let r := { r with checkQueue := r.checkQueue.extract 1 r.checkQueue.size }
  let (r, e) ← r.startCheck it none
  r.pumpChecks (evs ++ e)

/-- Start queued thumbnail downloads up to the limit. -/
partial def Runner.pumpThumbs (r : Runner) : BaseIO Runner := do
  if r.thumbsActive ≥ maxThumbs || r.thumbQueue.isEmpty then return r
  let id := r.thumbQueue[0]!
  let r := { r with thumbQueue := r.thumbQueue.extract 1 r.thumbQueue.size }
  let (r, err) ← r.request "GET" (thumbUrl r.cfg id) #[] .empty (.thumb id)
  let r := if err.isNone then { r with thumbsActive := r.thumbsActive + 1 } else r
  r.pumpThumbs

/-- Carry out an effect; events it produces at once. -/
def Runner.perform (r : Runner) (e : Effect) : BaseIO (Runner × Array Event) := do
  match e with
  | .search q =>
    Sys.log ("ytlean: search \"" ++ q ++ "\"")
    let (r, err) ← r.request "POST" (r.cfg.server ++ "/youtubei/v1/search?prettyPrint=false") jsonHeaders
      (Json.YouTube.searchBody q) (.search q)
    return (r, match err with | some e => #[.searchFailed e] | none => #[])
  | .thumbnail _ id => return ({ r with thumbQueue := r.thumbQueue.push id }, #[])
  | .check it => return ({ r with checkQueue := r.checkQueue.push it }, #[])
  | .play it =>
    Sys.log ("ytlean: play " ++ it.id ++ " \"" ++ it.title ++ "\"")
    r.startPlayer it none
  | .playerKey a =>
    match r.player with
    | some p => return ({ r with player := some (← p.key a), playerDirty := true }, #[])
    | none => return (r, #[])
  | .stopPlayer =>
    match r.player with
    | some p =>
      p.close
      let mut f := r.fetcher
      for (id, why) in r.pending do
        if let .media .. := why then f ← f.cancel id
      Sys.log ("ytlean: stopped after " ++ toString p.frames ++ " frames; network: " ++ r.fetcher.stats)
      return ({ r with player := none, fetcher := f, pending := r.pending.filter (fun (_, w) => match w with | .media .. => false | _ => true) }, #[.playerDone ""])
    | none => return (r, #[])
  | .exit => return (r, #[])

def fmtFormat (f : Json.YouTube.Format) : String :=
  "itag " ++ toString f.itag.toNat ++ " " ++ toString f.width.toNat ++ "x" ++ toString f.height.toNat ++
    "@" ++ toString f.fps.toNat ++ " (" ++ f.mime ++ ")"

/-! ### The phone link (YouTube Lounge, "Link with TV code") -/

def isPoll (w : Pending) : Bool := match w with | .lounge _ true => true | _ => false

/-- Start the session's requests; a request that cannot start goes straight
back to the session as a failure (it retries with backoff from `tick`). -/
partial def Runner.loungeIssue (r : Runner) (reqs : Array Lounge.Request) : BaseIO (Runner × Array Lounge.Command) := do
  let mut r := r
  let mut cmds : Array Lounge.Command := #[]
  for q in reqs do
    if q.streaming then
      -- a new long poll replaces the old one
      let mut f := r.fetcher
      for (id, why) in r.pending do
        if isPoll why then f ← f.cancel id
      r := { r with fetcher := f, pending := r.pending.filter (fun (_, w) => !isPoll w) }
    let (r', err) ← r.request q.method q.url q.headers q.body (.lounge q.id q.streaming) q.streaming
    r := r'
    if err.isSome then
      if let some s := r.lounge then
        let now ← Sys.monotonicMs
        if q.streaming then
          let (s, more) := s.onPollEnd q.id none now
          let (r', c) ← ({ r with lounge := some s } : Runner).loungeIssue more
          r := r'
          cmds := cmds ++ c
        else
          let (s, more, c) := s.onResponse q.id 0 .empty now
          let (r', c2) ← ({ r with lounge := some s } : Runner).loungeIssue more
          r := r'
          cmds := cmds ++ c ++ c2
  return (r, cmds)

/-- What the phone asked for, as app events and player actions. -/
def Runner.loungeCommands (r : Runner) (cmds : Array Lounge.Command) : BaseIO (Runner × Array Event) := do
  let mut r := r
  let mut evs : Array Event := #[]
  for c in cmds do
    Sys.log ("ytlean: phone: " ++ c.describe)
    match c with
    | .playVideo vid startMs _ =>
      if r.player.isSome then
        let (r', e) ← r.perform .stopPlayer
        r := r'
        evs := evs ++ e
      r := { r with castStartMs := startMs }
      evs := evs.push (.castPlay { id := vid, title := vid })
    | .play =>
      if let some p := r.player then
        if p.paused then r := { r with player := some (← p.key .ok), playerDirty := true }
    | .pause =>
      if let some p := r.player then
        if !p.paused then r := { r with player := some (← p.key .ok), playerDirty := true }
    | .seek ms =>
      if let some p := r.player then r := { r with player := some (← p.seekTo ms), playerDirty := true }
    | .setVolume v muted =>
      r := { r with castVolume := min v 100, castMuted := muted,
                    player := r.player.map fun p => { p with gain := min v 100, muted } }
    | .stop =>
      if r.player.isSome then
        let (r', e) ← r.perform .stopPlayer
        r := r'
        evs := evs ++ e
    | _ => pure ()
  return (r, evs)

/-- Issue requests, then turn commands into events. -/
def Runner.loungeStep (r : Runner) (reqs : Array Lounge.Request) (cmds : Array Lounge.Command) :
    BaseIO (Runner × Array Event) := do
  let (r, more) ← r.loungeIssue reqs
  r.loungeCommands (cmds ++ more)

/-- The player as the phone should see it. -/
def Runner.playerState (r : Runner) : BaseIO Lounge.PlayerState := do
  match r.player with
  | none => return { volume := r.castVolume, muted := r.castMuted }
  | some p =>
    let media ← p.media (← Sys.monotonicMs)
    let st : Lounge.PlayState :=
      if p.ended then .ended else if p.paused then .paused else if p.frames == 0 then .buffering else .playing
    return { videoId := some p.videoId, state := st, currentTimeMs := media, durationMs := p.duration,
             volume := r.castVolume, muted := r.castMuted }

/-- A finished Lounge request, or a streamed piece of the long poll. -/
def Runner.loungeResult (r : Runner) (rid : Nat) (streaming : Bool) (res : Net.Result) : BaseIO (Runner × Array Event) := do
  let some s := r.lounge | return (r, #[])
  let now ← Sys.monotonicMs
  if streaming then
    let status := match res with | .ok resp => some resp.status | .error _ => none
    -- a final piece of body may come with the end
    let (s, reqs0, cmds0) := match res with
      | .ok resp => if resp.body.size > 0 then s.onChunk rid resp.body now else (s, #[], #[])
      | .error _ => (s, #[], #[])
    let (s, reqs) := s.onPollEnd rid status now
    ({ r with lounge := some s } : Runner).loungeStep (reqs0 ++ reqs) cmds0
  else
    let (status, body) := match res with | .ok resp => (resp.status, resp.body) | .error _ => (0, ByteArray.empty)
    let (s, reqs, cmds) := s.onResponse rid status body now
    ({ r with lounge := some s } : Runner).loungeStep reqs cmds

/-- Periodic work: `tick` (about once a second), state reports while a
phone is connected, and the UI's view of the link. -/
def Runner.loungeTick (r : Runner) : BaseIO (Runner × Array Event) := do
  let some s := r.lounge | return (r, #[])
  let now ← Sys.monotonicMs
  let mut r := r
  let mut evs : Array Event := #[]
  if now ≥ r.lastTick + 1000 then
    let (s, reqs) := s.tick now
    let (r', e) ← ({ r with lounge := some s, lastTick := now } : Runner).loungeStep reqs #[]
    r := r'
    evs := evs ++ e
  if let some s := r.lounge then
    if !s.remoteNames.isEmpty && now ≥ r.lastReport + 500 then
      let (s, reqs) := s.report (← r.playerState) now
      let (r', e) ← ({ r with lounge := some s, lastReport := now } : Runner).loungeStep reqs #[]
      r := r'
      evs := evs ++ e
  if let some s := r.lounge then
    let cur := (s.pairingCode, s.remoteNames)
    if cur.1 != r.lastCast.1 || cur.2 != r.lastCast.2 then
      Sys.log ("ytlean: phone link code " ++ cur.1.getD "-" ++ ", phones " ++ toString cur.2.size ++
        cur.2.foldl (fun acc n => acc ++ " [" ++ n ++ "]") "")
      r := { r with lastCast := cur }
      evs := evs.push (.castStatus cur.1 cur.2)
  return (r, evs)

/-- A finished request: the events it produces (and possibly a follow-up). -/
def Runner.complete (r : Runner) (id : Nat) (res : Net.Result) : BaseIO (Runner × Array Event) := do
  let some (_, why) := r.pending.find? (·.1 == id) | return (r, #[])
  let r := { r with pending := r.pending.filter (·.1 != id) }
  match why with
  | .search q =>
    match res with
    | .error e => return (r, #[.searchFailed e])
    | .ok resp =>
      if resp.status != 200 then return (r, #[.searchFailed ("HTTP " ++ toString resp.status)])
      match Json.YouTube.parseSearch resp.body with
      | .error e => return (r, #[.searchFailed e])
      | .ok items =>
        Sys.log ("ytlean: search \"" ++ q ++ "\": " ++ toString items.size ++ " results")
        let items := items.map fun i => ({ id := i.id, title := i.title, channel := i.channel, length := i.length } : Item)
        -- checks still queued for an older search are moot
        return ({ r with checkQueue := #[] }, #[.searchDone q items])
  | .thumb vid =>
    let r := { r with thumbsActive := r.thumbsActive - 1 }
    match res with
    | .ok resp =>
      if resp.status != 200 then return (r, #[])
      match Jpeg.decode resp.body with
      | .ok img => return (r, #[.thumbDone vid (Jpeg.scaleNearest img thumbW thumbH)])
      | .error e => Sys.log ("ytlean: thumbnail " ++ vid ++ ": " ++ e); return (r, #[])
    | .error e => Sys.log ("ytlean: thumbnail " ++ vid ++ ": " ++ e); return (r, #[])
  | .lounge rid streaming => r.loungeResult rid streaming res
  | .check it withVisitor =>
    let r := { r with checksActive := r.checksActive - 1 }
    -- only YouTube's answer hides a result; transport or parse trouble shows it
    match res with
    | .error _ => return (r, #[.checked it.id true])
    | .ok resp =>
      if resp.status != 200 then return (r, #[.checked it.id true])
      match Json.YouTube.parsePlayer resp.body with
      | .error _ => return (r, #[.checked it.id true])
      | .ok p =>
        if !p.playable && !withVisitor && p.visitorData.isSome then return ← r.startCheck it p.visitorData
        if r.checksActive == 0 && r.checkQueue.isEmpty then
          Sys.log ("ytlean: search checks done; network: " ++ r.fetcher.stats)
        match Json.YouTube.unplayableReason p with
        | some why =>
          Sys.log ("ytlean: hiding " ++ it.id ++ ": " ++ why)
          return (r, #[.checked it.id false])
        | none => return (r, #[.checked it.id true])
  | .media track gen seg =>
    match r.player with
    | none => return (r, #[])
    | some p =>
      let fail (msg : String) : Player.State :=
        if track == 0 then { p with error := some msg }
        else (Sys.log ("player: audio stopped: " ++ msg) |> fun _ => { p with audio := none })
      match res with
      | .error e => return ({ r with player := some (fail e) }, #[])
      | .ok resp =>
        if resp.status != 200 && resp.status != 206 then
          return ({ r with player := some (fail ("media HTTP " ++ toString resp.status)) }, #[])
        return ({ r with player := some (← p.onData track gen seg resp.body) }, #[])
  | .player it withVisitor =>
    match res with
    | .error e => return (r, #[.playerDone ("Could not open: " ++ e)])
    | .ok resp =>
      if resp.status != 200 then return (r, #[.playerDone ("Player API: HTTP " ++ toString resp.status)])
      match Json.YouTube.parsePlayer resp.body with
      | .error e => return (r, #[.playerDone ("Player API: " ++ e)])
      | .ok p =>
        if !p.playable && !withVisitor && p.visitorData.isSome then return ← r.startPlayer it p.visitorData
        match Json.YouTube.unplayableReason p, Json.YouTube.pickVideo p.formats 720 with
        | none, some vf =>
          let a := Json.YouTube.pickAudio p.formats
          Sys.log ("ytlean: \"" ++ p.title ++ "\" resolved, " ++ toString p.formats.size ++ " formats; video " ++
            fmtFormat vf ++ (match a with | some a => ", audio " ++ fmtFormat a | none => ", no AAC audio"))
          if let some old := r.player then old.close
          let st0 := Player.State.new p.title p.lengthS vf a
          let st := { st0 with videoId := it.id, startMs := r.castStartMs, gain := r.castVolume, muted := r.castMuted }
          return ({ r with player := some st, castStartMs := 0 }, #[.playerStarted p.title])
        | why, _ => return (r, #[.playerDone ("Not playable: " ++ why.getD "no H.264 stream")])

@[export yt_app_main]
def main (_argc : UInt32) : BaseIO UInt32 := do
  let c := Config.parse (← Sys.args) (← Sys.getenv "TV_HTTP_PROXY")
  Sys.log ("ytlean: mode " ++ c.mode ++ ", server " ++ c.server ++
    (match c.proxy with | some p => ", proxy " ++ p | none => ""))
  let d ← match ← Display.open with
    | .ok d => pure d
    | .error e => Sys.log ("ytlean: display open failed, errno " ++ toString e); return 1
  let input ← match ← Input.open with
    | .ok i => pure i
    | .error e => Sys.log ("ytlean: input open failed, errno " ++ toString e); d.close; return 1
  let fonts := Fonts.load
  let proxy := c.proxy.bind Net.parseHostPort
  let mut run : Runner := { cfg := c, fetcher := { proxy } }
  -- the phone link needs YouTube itself (through the proxy on the TV)
  if proxy.isSome then
    let ident : Lounge.Identity :=
      { deviceId := Lounge.uuidFromBytes ((← Sys.getrandom 16).toOption.getD (ByteArray.mk #[0])), name := "Dande TV" }
    let entropy := (← Sys.getrandom 32).toOption.getD .empty
    let (s, reqs) := Lounge.Session.start {} ident entropy (← Sys.monotonicMs)
    let (r', _) ← ({ run with lounge := some s } : Runner).loungeStep reqs #[]
    run := r'
    Sys.log "ytlean: phone link starting"
  let mut m : Model := { castAvailable := proxy.isSome }
  render d fonts m
  Sys.log "ytlean: home screen"
  let mut running := true
  -- play mode (ytplay.video=ID, as the launcher's video tiles start it): that
  -- video at once, then exit
  let mut seed : Array Event :=
    if c.mode == "play" && !c.video.isEmpty then #[.castPlay { id := c.video, title := c.video }] else #[]
  while running do
    let mut events : Array Event := seed
    seed := #[]
    -- drain the remote
    let mut more := true
    while more do
      match ← input.poll with
      | .ok (some k) =>
        if k.value != 0 then
          if let some a := Action.ofCode k.code then events := events.push (.key a (k.value == 2))
      | _ => more := false
    -- the player: downloads, decoding, pictures due
    let mut wait := if events.isEmpty then 15 else 0
    if let some p := run.player then
      let mut p := p
      for (track, gen, seg, a, b) in p.wants do
        let url := if track == 0 then p.video.url else (p.audio.map (·.url)).getD ""
        let (r', err) ← run.request "GET" url #[Http.rangeHeader a b] .empty (.media track gen seg)
        run := r'
        p := match err with
          | some e => if track == 0 then { p with error := some e } else { p with audio := none }
          | none => p.markInflight track
      p ← p.decode
      p ← p.feedAudio
      let (p', pw) ← p.present d fonts
      p := p'
      if run.playerDirty then
        p.redraw d fonts
        run := { run with playerDirty := false }
      wait := min wait pw
      run := { run with player := some p }
      if p.error.isSome || (p.ended && !p.pending) || (run.cfg.frames > 0 && p.frames ≥ run.cfg.frames) then
        let msg := match p.error with | some e => "Playback failed: " ++ e | none => ""
        Sys.log ("ytlean: playback ended after " ++ toString p.frames ++ " frames")
        let (r', _) ← run.perform .stopPlayer
        run := r'
        events := events.push (.playerDone msg)
        if let some s := run.lounge then
          let (s, reqs, cmds) := s.ended (← Sys.monotonicMs)
          let (r', evs) ← ({ run with lounge := some s } : Runner).loungeStep reqs cmds
          run := r'
          events := events ++ evs
    -- network (also the idle wait)
    let (f, done, chunks) ← run.fetcher.step wait
    run := { run with fetcher := f }
    for (id, bytes) in chunks do
      if let some (_, .lounge rid true) := run.pending.find? (·.1 == id) then
        if let some s := run.lounge then
          let (s, reqs, cmds) := s.onChunk rid bytes (← Sys.monotonicMs)
          let (r', evs) ← ({ run with lounge := some s } : Runner).loungeStep reqs cmds
          run := r'
          events := events ++ evs
    let (r', evs) ← run.loungeTick
    run := r'
    events := events ++ evs
    for (id, res) in done do
      let (r', evs) ← run.complete id res
      run := r'
      events := events ++ evs
    if !events.isEmpty then
      let mut i := 0
      while i < events.size do
        if let .playerDone msg := events[i]! then
          -- why the player closed, also when it never opened (the screen shows it)
          unless msg.isEmpty do Sys.log ("ytlean: " ++ msg)
          if c.mode == "play" then running := false
        let (m', fx) := update m events[i]!
        if m'.screen != m.screen then
          Sys.log ("ytlean: screen " ++ match m'.screen with | .home => "home" | .search => "search" | .player => "player")
        m := m'
        for e in fx do
          if let .exit := e then running := false
          let (r', evs) ← run.perform e
          run := r'
          events := events ++ evs
        i := i + 1
      if m.screen != .player then render d fonts m
    let (r', evs) ← run.pumpChecks
    run ← r'.pumpThumbs
    seed := evs
  Sys.log "ytlean: exit"
  input.close
  d.close
  return 0

end Yt.App
