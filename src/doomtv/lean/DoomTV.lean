import Doom.Game
import Doom.Playsim.Input
import Doom.Front.Menu
import Doom.Front.Screens
import Doom.Audio.Sound
import Doom.Net.Session
import Doom.TV.NetGame
import Doom.Render.Frame
import Doom.TV.Keys
import Doom.TV.Platform
import Doom.TV.Audio

/-!
# doomtv

The TV application: the Lean engine driven by the TV app ABI. Run by the
launcher as `/bin/doomtv` (confined, no PID-1 duties). Home is handled by the
launcher; quitting from the menu exits 0, an engine error exits 1.

It starts like Doom: title pages with the menu one key away (Back/Escape or
any key). New Game picks episode (Doom 1 IWADs) and skill.

Arguments: `doom.iwad=PATH` (default /doom/freedoom1.wad, /freedoom1.wad,
/doom/freedm.wad), `doom.warp=E,M` (skip the title and start that map),
`doom.skill=0..4`, `doom.stats=SECONDS`.
`doom.bench=N` runs N headless frames instead (no display or input): the
player turns in place while every tic is simulated, rendered and converted to
a 1280x720 buffer, then per-stage timings are printed.
-/

open Doom.Playsim.GameState
open Doom.Playsim.Input
open Doom.Playsim.Player
open Doom.Render.Data
open Doom.TV.Keys
open Doom.TV.Platform
open Doom.Wad

namespace Menu
export Doom.Front.Menu (GameMode State Effect Context gameMode respond tick draw)
end Menu

def log (msg : String) : IO Unit := do
  let ms ← IO.monoMsNow
  IO.println s!"[{ms / 1000}.{toString (1000 + ms % 1000) |>.drop 1}] {msg}"
  (← IO.getStdout).flush

def argValue (args : List String) (key : String) : Option String :=
  args.findSome? fun a => if a.startsWith (key ++ "=") then some (a.drop (key.length + 1)).toString else none

/-- The attract sequence (`D_DoAdvanceDemo`): pages with their tics, and demo
lumps (tics 0). Ultimate-format IWADs add DEMO4. -/
def pages (mode : Menu.GameMode) : Array (String × Nat) :=
  let p4 := if mode == .commercial then ("TITLEPIC", 35 * 11)
    else (if mode == .retail then "CREDIT" else "HELP2", 200)
  let seq := #[("TITLEPIC", if mode == .commercial then 35 * 11 else 170), ("DEMO1", 0),
    ("CREDIT", 200), ("DEMO2", 0), p4, ("DEMO3", 0)]
  if mode == .retail then seq.push ("DEMO4", 0) else seq

def isDemoStep (name : String) : Bool := name.startsWith "DEMO"

inductive Phase
  /-- Title sequence: page index and tics left on it. -/
  | title (page : Nat) (left : Nat)
  /-- A level, intermission or finale is being played. -/
  | game
  /-- LAN game lobby (hosting, or looking for / waiting on a host). -/
  | lobby
  /-- An attract-sequence demo (step index) playing in `gs`. -/
  | demo (step : Nat)

/-- A LAN game: lobby and lockstep session (`Doom.Net`). -/
structure NetState where
  isHost : Bool
  /-- `none` while a joiner is still looking for a host. -/
  session : Option Doom.Net.Session := none
  cfg : Doom.Net.Config
  settings : Doom.Net.GameSettings := {}
  /-- Join this host directly instead of discovering one. -/
  joinAddr : Option Doom.Net.Addr := none
  found : Array Doom.TV.NetGame.Found := #[]
  lastQueryMs : Nat := 0
  /-- Host starts by itself once this many players are in (0: on OK). -/
  autoStart : Nat := 0
  status : String := ""

structure Session where
  wad : WadDirectory
  data : RenderData
  mode : Menu.GameMode
  gs : GameState
  phase : Phase := .title 0 170
  menu : Menu.State := {}
  inp : InputState := InputState.init
  /-- Tics run since the program started (`gametic`). -/
  gametic : Nat := 0
  frame : Doom.Render.Frame.Cache := {}
  quit : Bool := false
  /-- Sound effects and music before the PCM worker takes ownership. -/
  sound : Option Doom.Audio.Sound.System := none
  /-- One PCM producer independent of video frames; updates wait for its mailbox. -/
  audioWorker : Option Doom.TV.Audio.Worker := none
  audioPending : Array (Doom.Audio.Sound.System → Doom.Audio.Sound.System) := #[]
  /-- Sound commands collected since the last tic. -/
  soundCmds : Array Doom.Audio.Sound.Cmd := #[]
  net : Option NetState := none
  /-- LAN menu effects waiting for the IO loop (opening sockets needs IO). -/
  pendingNet : Array Menu.Effect := #[]
  lobbyStart : Bool := false
  lobbyLeave : Bool := false
  /-- Save slots. A game state is an immutable value, so a save is just a
  reference to it (sharing structure with the running game); saves last
  until the app exits. -/
  saves : Array (Option GameState) := Array.replicate 6 none
  /-- Program arguments (LAN game options). -/
  args : List String := []
  /-- Automap, intermission animation and finale (`Doom.Front.Screens`). -/
  front : Doom.Front.Screens.State := {}

def mapName (mode : Menu.GameMode) (episode map : Nat) : String :=
  if mode == .commercial then (if map < 10 then s!"MAP0{map}" else s!"MAP{map}")
  else s!"E{episode}M{map}"

/-- `G_InitNew` through the playsim's game API (`Doom.Game`). -/
def startLevel (wad : WadDirectory) (_mode : Menu.GameMode) (episode map : Nat) (skill : Int32)
    (ingame : Array Bool := #[true, false, false, false]) (consoleplayer : Nat := 0)
    (deathmatch : Int32 := 0) (nomonsters respawn fast : Bool := false) (netgame : Option Bool := none) :
    Except String GameState :=
  let o : Doom.Game.NewGameOptions := { skill, episode := Int32.ofNat episode, map := Int32.ofNat map }
  Doom.Game.newGame wad { o with playeringame := ingame, consoleplayer, deathmatch, nomonsters, respawn, fast, netgame }

def menuContext (s : Session) : Menu.Context :=
  { mode := s.mode, usergame := s.phase matches .game, netgame := s.net.isSome, gametic := s.gametic }

def argValue' (args : List String) (key : String) : Option String :=
  args.findSome? fun a => if a.startsWith (key ++ "=") then some (a.drop (key.length + 1)).toString else none

/-- Parse a dotted IPv4 address. -/
def parseIp (s : String) : Option UInt32 :=
  match s.splitOn "." |>.map String.toNat? with
  | [some a, some b, some c, some d] =>
    if a < 256 && b < 256 && c < 256 && d < 256 then
      some ((a * 16777216 + b * 65536 + c * 256 + d).toUInt32) else none
  | _ => none

/-- Start hosting (mode 1 co-op, 2 deathmatch) or looking for a LAN game. -/
def openNet (s : Session) (host : Bool) (mode skill episode : Nat) (nowMs : Nat) : IO Session := do
  let r ← udpOpen Doom.Net.PORT
  if r != 0 then
    IO.println s!"net: cannot open UDP port {Doom.Net.PORT} ({statusText r})"
    return s
  let nonce := (← IO.getRandomBytes 4).foldl (fun a b => a * 256 + b.toUInt32) (0 : UInt32)
  let name := (argValue' s.args "doom.name").getD (if host then "host" else "player")
  -- doom.netlead: most local commands in flight; raise it for slow frames or
  -- high latency (the session's default 6 suits a TV LAN at 35 fps).
  let maxLead := (argValue' s.args "doom.netlead").bind String.toNat? |>.getD 6
  let cfg : Doom.Net.Config := { name, iwadDigest := Doom.TV.NetGame.iwadDigest s.wad, nonce := nonce ||| 1, maxLead }
  let autoStart := (argValue' s.args "doom.netauto").bind String.toNat? |>.getD 0
  let dm : UInt8 := if mode == 2 then 1 else 0
  let settings : Doom.Net.GameSettings := { episode := episode.toUInt8, map := 1, skill := skill.toUInt8, deathmatch := dm }
  let joinAddr := match (argValue' s.args "doom.net").map (·.splitOn ",") with
    | some ["join", ip] => (parseIp ip).map fun a => { ip := a, port := Doom.Net.PORT }
    | _ => none
  let base : NetState := { isHost := host, cfg, settings, autoStart, joinAddr }
  let ns : NetState :=
    if host then { base with session := some (Doom.Net.Session.host cfg), status := "waiting for players" }
    else match joinAddr with
      | some a => { base with session := some (Doom.Net.Session.join cfg a), status := s!"joining {a}" }
      | none => { base with status := "looking for a lan game" }
  IO.println s!"net: {if host then s!"hosting {if mode == 2 then "deathmatch" else "co-op"}" else "joining"} on UDP {Doom.Net.PORT}"
  let _ := nowMs
  return { s with net := some ns, phase := .lobby, menu := Doom.Front.Menu.close s.menu }

/-- Update sound before worker startup; afterwards queue the immutable command. -/
def Session.withSound (s : Session) (f : Doom.Audio.Sound.System → Doom.Audio.Sound.System) : Session :=
  match s.sound with
  | some snd =>
    let s := { s with sound := none }
    { s with sound := some (f snd) }
  | none => if s.audioWorker.isSome then { s with audioPending := s.audioPending.push f } else s

/-- Title music (`D_DoAdvanceDemo` page 0). -/
def titleMusic (s : Session) : Session :=
  let name := if s.mode == .commercial then "dm2ttl" else "intro"
  match Doom.Audio.Sound.musicId? name with
  | some m => s.withSound (·.changeMusic m false)
  | none => s

/-- Level music and channel reset (`S_Start`). -/
def levelSound (s : Session) : Session :=
  s.withSound (·.startLevel (s.mode == .commercial) s.gs.gameepisode.toNatClampNeg s.gs.gamemap.toNatClampNeg)

/-- Keep the mixer's volumes equal to the menu's. -/
def syncVolumes (s : Session) : Session :=
  let st := s.menu.settings
  s.withSound fun snd =>
    if snd.sfxVolume == st.sfxVolume && snd.musicVolume == st.musicVolume then snd
    else (snd.setSfxVolume st.sfxVolume).setMusicVolume st.musicVolume

/-- Sound origin ids for the mixer: map objects by index, sector sound origins
(doors, lifts) by position. The listener is its map object's id. -/
def originId : SoundOrigin → Option Doom.Audio.Sound.Origin
  | .none => none
  | .mobj i x y => some { id := i.toNat + 1, x, y }
  | .sector x y => some { id := 0x40000000 + ((x.toUInt32 ^^^ (y.toUInt32 * 2654435761)) >>> 2).toNat, x, y }

/-- Move this tic's playsim sound events (`S_StartSound` etc.) to the mixer. -/
def drainSound (s : Session) : Session :=
  let ev := s.gs.soundEvents
  if ev.isEmpty then s else
  let s := { s with gs := { s.gs with soundEvents := #[] } }
  ev.foldl (init := s) fun s e => match e with
    | .start sfx origin audible =>
      if audible then { s with soundCmds := s.soundCmds.push (.start sfx (originId origin)) } else s
    | .stop origin => { s with soundCmds := s.soundCmds.push (.stop ((originId origin).map (·.id))) }
    | .music mus looping => s.withSound (·.changeMusic mus looping)

/-- Apply menu effects. -/
def applyEffects (s : Session) (fx : Array Menu.Effect) : Except String Session := do
  let mut s := s
  for e in fx do
    match e with
    | .newGame skill episode =>
      let gs ← startLevel s.wad s.mode episode 1 (Int32.ofNat skill)
      s := drainSound (levelSound { s with gs, phase := .game, inp := InputState.init, front := s.front.newGame })
    | .endGame =>
      let ps := pages s.mode
      s := titleMusic { s with phase := .title 0 (ps[0]!.2), inp := InputState.init, front := s.front.newGame }
    | .quit => s := { s with quit := true }
    | .message text => s := { s with gs := { s.gs with hu := { messageOn := true, messageCounter := 140, messageText := text } } }
    | .sound sfx => s := { s with soundCmds := s.soundCmds.push (.start sfx none) }
    | .hostGame .. | .joinGame => s := { s with pendingNet := s.pendingNet.push e }
    | .saveGame k =>
      if s.gs.gamestate == gamestateLevel && k < 6 then
        let secs := s.gs.leveltime.toNat / 35
        let pad (n : Nat) := if n < 10 then s!"0{n}" else toString n
        let name := s!"{mapName s.mode s.gs.gameepisode.toNatClampNeg s.gs.gamemap.toNatClampNeg}  {secs / 60}:{pad (secs % 60)}"
        s := { s with saves := s.saves.set! k (some s.gs),
                      menu := { s.menu with slotNames := s.menu.slotNames.set! k name } }
        s := { s with gs := { s.gs with hu := { messageOn := true, messageCounter := 140, messageText := "game saved." } } }
    | .loadGame k =>
      if let some gs := (s.saves[k]?).join then
        s := drainSound (levelSound { s with gs := { gs with soundEvents := #[] }, phase := .game,
                                             inp := InputState.init, front := s.front.newGame })
  pure (syncVolumes s)

/-- Apply front-end events: sounds and music now, the next map at `.worldDone`. -/
def frontEvents (s : Session) (ev : Array Doom.Front.Screens.Event) : Except String Session :=
  ev.foldlM (init := s) fun s e => match e with
    | .sound sfx => pure { s with soundCmds := s.soundCmds.push (.start sfx none) }
    | .music name looping => pure (match Doom.Audio.Sound.musicId? name with
      | some m => s.withSound (·.changeMusic m looping)
      | none => s)
    | .worldDone => pure s  -- the playsim's G_Ticker loads the next map itself

/-- The front-end responders (`AM_Responder`, `F_Responder`) for one key. -/
def frontKey (s : Session) (key : UInt32) (down : Bool) : Except String (Session × Bool) := do
  let (front, gs, ev, eaten) := s.front.respond s.gs key.toNat down
  return (← frontEvents { s with front, gs } ev, eaten)

/-- Route one key event: the menu first (`M_Responder`), then the game.
Releases always reach the game so no key stays held. -/
def keyEvent (s : Session) (code value : UInt32) : Except String Session := do
  let keys := doomKeys code
  if value == 0 then
    let s ← keys.foldlM (fun s k => return (← frontKey s k false).1) s
    return { s with inp := keys.foldl (fun i k => gResponder i false k) s.inp }
  let mut s := s
  let mut eaten := false
  for k in keys do
    if eaten then break
    let (menu, fx, used) := Menu.respond s.menu (menuContext s) k.toNat
    if used then
      s ← applyEffects { s with menu } fx
      eaten := true
  if eaten || value == 2 then return s
  match s.phase with
  | .lobby =>
    -- OK starts the game (host, two or more players); Back leaves the lobby.
    if keys.contains Doom.TV.Keys.KEY_ENTER then return { s with lobbyStart := true }
    if keys.contains Doom.TV.Keys.KEY_ESCAPE then return { s with lobbyLeave := true }
    return s
  | .title .. | .demo .. =>
    -- Any key on the title pages or during a demo opens the menu (`G_Responder`).
    return { s with menu := Doom.Front.Menu.open_ s.menu }
  | .game =>
    if s.menu.active || s.menu.message.isSome then return s
    keys.foldlM (init := s) fun s k => do
      let (s, used) ← frontKey s k true
      return if used then s else { s with inp := gResponder s.inp true k }

/-- Drain the remote/keyboard queue. -/
partial def pollInput (s : Session) : IO (Except String Session) := do
  let ev ← inputPoll
  if ev == 0 then return .ok s
  match keyEvent s (ev &&& 0xffff) ((ev >>> 16) &&& 0x3) with
  | .ok s => pollInput s
  | .error e => return .error e

/-- The console player as the sound listener. -/
def listener (gs : GameState) : Doom.Audio.Sound.Listener :=
  match gs.players[gs.consoleplayer]? with
  | some p =>
    match gs.mobjs[p.mo.toNatClampNeg]? with
    | some m => if p.mo < 0 then default else { id := p.mo.toNatClampNeg + 1, x := m.x, y := m.y, angle := m.angle }
    | none => default
  | none => default

/-- `S_UpdateSounds` with this tic's commands. -/
def soundTic (s : Session) : Session :=
  let cmds := s.soundCmds
  let l := listener s.gs
  { s with soundCmds := #[] }.withSound (·.tick l cmds)

/-- Go to attract step `i` (`D_DoAdvanceDemo`): a page, or a demo through the
game API (skipped if the IWAD lacks it or it fails to start). -/
def advanceTitle (s : Session) (i : Nat) : Session := Id.run do
  let ps := pages s.mode
  let mut i := i % ps.size
  for _ in [0:ps.size] do
    let (name, tics) := ps[i]!
    if !isDemoStep name then
      let s := { s with phase := .title i tics }
      -- Pages 0 (and 4 in Doom II) restart the title music.
      return if name == "TITLEPIC" then titleMusic s else s
    match (checkNumForName s.wad name).bind (fun l => (lumpData s.wad l).toOption) with
    | some demo =>
      match Doom.Game.playDemo s.wad demo with
      | .ok gs => return { s with gs, phase := .demo i, front := s.front.newGame }
      | .error _ => pure ()
    | none => pure ()
    i := (i + 1) % ps.size
  return { s with phase := .title 0 (ps[0]!.2) }

/-- One 35 Hz tic of whatever is running. -/
def runTic (s : Session) : Except String Session := do
  let s := soundTic { s with menu := Menu.tick s.menu, gametic := s.gametic + 1 }
  match s.phase with
  | .title page left =>
    -- Hold the page while the menu is open so its text stays legible on a TV
    -- (vanilla keeps cycling the attract sequence underneath).
    if s.menu.active || s.menu.message.isSome then return s
    if left > 0 then return { s with phase := .title page (left - 1) }
    return advanceTitle s (page + 1)
  | .demo step =>
    -- The demo plays under the menu, as in vanilla (`demoplayback`).
    let gs := Doom.Game.ticker s.gs #[]
    if gs.error.isSome || gs.demoEnded then return advanceTitle { s with gs } (step + 1)
    let (front, gs, _) := s.front.tic gs
    return drainSound { s with gs, front }
  | .lobby => return s
  | .game =>
    -- LAN games run tics as the lockstep session delivers them (netFrame).
    if s.net.isSome then return s
    -- Single player pauses under the menu (`P_Ticker`).
    if s.menu.active || s.menu.message.isSome then return s
    let (inp, cmd) := buildTiccmd s.inp
    let cmds := (List.range 4).toArray.map fun i => if i == s.gs.consoleplayer then cmd else TicCmd.zero
    let gs := Doom.Game.ticker s.gs cmds
    if let some e := gs.error then throw e
    let (front, gs, ev) := s.front.tic gs
    let s ← frontEvents { s with gs, inp, front } ev
    pure (drainSound s)

/-- Lobby screen text. -/
def lobbyLines (s : Session) : List String :=
  match s.net with
  | none => []
  | some ns =>
    let kind := if ns.settings.deathmatch != 0 then "deathmatch" else "co-op"
    let head := if ns.isHost then s!"hosting lan {kind}" else "lan game"
    let players := match ns.session with
      | some ses => (ses.lobby.toList.map fun e => s!"  {e.name}{if e.isHost then " (host)" else ""}{if e.isLocal then " - you" else ""}")
      | none => ns.found.toList.map fun f => s!"  found {f.name} at {f.addr}"
    let hint := if ns.isHost then "ok: start (2+ players)   back: leave" else "back: leave"
    [head, "", ns.status] ++ players ++ ["", hint]

/-- Start the level the session's START described. -/
def startNetGame (s : Session) (info : Doom.Net.StartInfo) : Except String Session := do
  let g := info.settings
  let ingame := (List.range 4).toArray.map fun p => (info.playerMask >>> p.toUInt8) &&& 1 == 1
  let gs ← startLevel s.wad s.mode g.episode.toNat g.map.toNat g.skill.toNat.toUInt32.toInt32 ingame
    info.consoleplayer.toNat g.deathmatch.toNat.toUInt32.toInt32 g.nomonsters g.respawn g.fast (some true)
  pure (drainSound (levelSound { s with gs, phase := .game, inp := InputState.init, front := s.front.newGame }))

/-- One frame of a LAN game: datagrams in, discovery, lobby, local commands,
datagrams out, then every tic the session has ready. Returns tics run. -/
def netFrame (s0 : Session) (now : Nat) : IO (Except String (Session × Nat)) := do
  let some ns0 := s0.net | return .ok (s0, 0)
  let dropsBefore := match ns0.session with | some ses => ses.drops.size | none => 0
  let mut s := { s0 with net := none }
  let mut ns := ns0
  let nowU := now.toUInt64
  -- Datagrams in.
  for _ in [0:64] do
    let d ← udpRecv
    if d.size < 6 then break
    let ip := (d[0]!.toUInt32 <<< 24) ||| (d[1]!.toUInt32 <<< 16) ||| (d[2]!.toUInt32 <<< 8) ||| d[3]!.toUInt32
    let src : Doom.Net.Addr := { ip, port := (d[4]!.toUInt16 <<< 8) ||| d[5]!.toUInt16 }
    let payload := d.extract 6 d.size
    if Doom.TV.NetGame.isQuery payload then
      if ns.isHost then
        if let some ses := ns.session then
          if ses.phase == .lobby then
            let _ ← udpSend src.ip src.port (Doom.TV.NetGame.answer (if ns.settings.deathmatch != 0 then 2 else 1) ses.playerCount ns.cfg.name)
    else match Doom.TV.NetGame.parseAnswer src payload with
      | some f => if ns.session.isNone && !(ns.found.any (·.addr == f.addr)) then ns := { ns with found := ns.found.push f }
      | none => if let some ses := ns.session then ns := { ns with session := some (ses.receive src payload nowU) }
  -- Joiner: discover a host, then join the first one found.
  if ns.session.isNone then
    if let some f := ns.found[0]? then
      IO.println s!"net: found {f.name} at {f.addr}; joining"
      ns := { ns with session := some (Doom.Net.Session.join ns.cfg f.addr), status := s!"joining {f.name}" }
    else if now - ns.lastQueryMs ≥ 500 then
      let r ← udpBroadcast Doom.Net.PORT Doom.TV.NetGame.query
      if r >= 0x80000000 && ns.lastQueryMs == 0 then IO.println s!"net: discovery broadcast failed ({statusText r})"
      ns := { ns with lastQueryMs := now }
  let some ses := ns.session | return .ok ({ s with net := some ns }, 0)
  let mut ses := ses
  -- Host: start on OK (or automatically) with two or more players.
  if ns.isHost && ses.phase == .lobby && ses.playerCount ≥ 2 &&
      (s.lobbyStart || (ns.autoStart ≥ 2 && ses.playerCount ≥ ns.autoStart)) then
    match ses.start ns.settings nowU with
    | .ok ses' => ses := ses'; IO.println s!"net: starting with {ses.playerCount} players"
    | .error e => IO.println s!"net: cannot start: {e}"
  s := { s with lobbyStart := false }
  if s.lobbyLeave then
    ses := ses.leave
    let (_, out) := ses.poll nowU
    for (a, b) in out do let _ ← udpSend a.ip a.port b
    IO.println "net: left the lobby"
    let ps := pages s.mode
    return .ok (titleMusic { s with net := none, lobbyLeave := false, phase := .title 0 (ps[0]!.2) }, 0)
  -- Lobby: the game starts when START arrives (or was just sent). Send first
  -- (START, or our acknowledgement), then load the level: loading takes
  -- seconds and peers time out on silence.
  if (s.phase matches .lobby) && ses.phase == .running then
    let (ses', out) := ses.poll nowU
    ses := ses'
    for (a, b) in out do let _ ← udpSend a.ip a.port b
    if let some info := ses.info then
      match startNetGame s info with
      | .ok s' =>
        s := s'
        ns := { ns with settings := info.settings }
        IO.println s!"net: game started as player {info.consoleplayer.toNat + 1}, players mask {info.playerMask}"
      | .error e => return .error e
  if ses.phase == .lobby then
    ns := { ns with status := s!"{ses.playerCount} player(s) in lobby" }
  -- Local commands, then datagrams out.
  let mut inp := s.inp
  if s.phase matches .game then
    while ses.localDue nowU do
      let (inp', cmd) := buildTiccmd inp
      inp := inp'
      ses := ses.submitLocal cmd nowU
  let (ses', out) := ses.poll nowU
  ses := ses'
  for (a, b) in out do let _ ← udpSend a.ip a.port b
  s := { s with inp }
  -- Ready tics.
  let mut ran := 0
  if s.phase matches .game then
    while ran < 10 do
      let some rt := ses.nextTic? | break
      let mut gs := s.gs
      -- A player missing from the tic's mask left the game (`PlayerQuitGame`).
      let ingame := (List.range 4).toArray.map fun p => (rt.ingame >>> p.toUInt8) &&& 1 == 1
      gs := Doom.Game.tickerNet gs (rt.cmds.extract 0 4) ingame
      if let some e := gs.error then return .error e
      -- Automap ticker, and the playsim's sound events to the mixer.
      let (front, gs2, ev) := s.front.tic gs
      match frontEvents { s with gs := gs2, front } ev with
      | .ok s' => s := drainSound s'; gs := s.gs
      | .error e => return .error e
      let sum := Doom.Game.checksum gs
      if rt.tic % 70 == 0 then
        let pc := ses.pacing
        IO.println s!"net: tic {rt.tic} sum {sum} (ready {pc.readyTics}, lead {pc.localLead}, ahead {pc.aheadTics}, window {pc.windowOpen})"
      ses := ses.advance.submitChecksum rt.tic sum
      s := { s with gs, gametic := s.gametic + 1 }
      ran := ran + 1
  -- A player who left (Home on that TV, or silent for `dropTimeoutMs`), once each.
  for d in ses.drops.extract dropsBefore ses.drops.size do
    let why := match d.reason with | .timeout => "timed out" | .quit => "quit" | .kicked => "removed"
    IO.println s!"net: player {d.slot.toNat + 1} left the game at tic {d.tic} ({why})"
  if let some d := ses.desync then
    if ns.status != "desync" then IO.println s!"net: desync at tic {d.tic}"
    ns := { ns with status := "desync" }
  if ses.phase == .ended then
    let why := (ses.endReason.map (·.describe)).getD "game over"
    IO.println s!"net: game ended: {why}"
    let ps := pages s.mode
    return .ok (titleMusic { s with net := none, phase := .title 0 (ps[0]!.2) }, ran)
  return .ok ({ s with net := some { ns with session := some ses } }, ran)

/-- The frame for the current phase with the menu on top. -/
def render (s : Session) (frame : Doom.Render.Frame.Cache) :
    Except String (ByteArray × Doom.Render.Frame.Cache) := do
  let (pal, frame) ← match s.phase with
    | .title page _ => Doom.Render.Frame.renderPage s.data s.wad ((pages s.mode)[page]!.1) frame
    | .demo _ => s.front.render s.data s.wad s.gs frame s.menu.settings.screenblocks
    | .lobby => do
      let (pal, frame) ← Doom.Render.Frame.renderPage s.data s.wad
        (if (checkNumForName s.wad "INTERPIC").isSome then "INTERPIC" else "TITLEPIC") frame
      let mut fb := frame.fb
      let lines := lobbyLines s
      let mut y : Int32 := 40
      for l in lines do
        fb ← Doom.Render.Hud.drawTextLine s.wad fb 40 y l
        y := y + 12
      pure (pal, { frame with fb })
    | .game =>
      -- Options > Messages off: player messages are not shown (`showMessages`).
      let gs := if s.menu.settings.showMessages then s.gs else { s.gs with hu := { s.gs.hu with messageOn := false } }
      s.front.render s.data s.wad gs frame s.menu.settings.screenblocks
  let fb ← Menu.draw s.menu s.mode s.wad frame.fb
  pure (pal, { frame with fb })

/-- Submit sound commands; the worker replenishes PCM while video renders. -/
def pumpAudio (s : Session) : IO Session := do
  let (worker, s) ← match s.audioWorker, s.sound with
    | some worker, _ => pure (worker, s)
    | none, some sound => do
      let worker ← Doom.TV.Audio.Worker.start sound
      pure (worker, { s with sound := none, audioWorker := some worker })
    | none, none => return s
  worker.submit s.audioPending
  return { s with audioPending := #[] }

/-- An engine error ends the game, not the app: log it, drop any LAN game,
and show it over the title pages so it can be read and reported. -/
def recover (s : Session) (what : String) (e : String) : IO Session := do
  log s!"doom: {what}: {e}"
  let ps := pages s.mode
  let text := s!"the game stopped:\n{(e.take 40).toString}\n\n{Doom.Front.Menu.pressKey}"
  let menu := { s.menu with active := true, message := some (text, .info, false), messageReturnsToMenu := false }
  return titleMusic { s with net := none, phase := .title 0 (ps[0]!.2), inp := InputState.init, menu,
                             front := s.front.newGame }

/-- Headless timing on the target: simulate, render and convert N frames. -/
def bench (s0 : Session) (frames : Nat) : IO UInt32 := do
  let mut s := s0
  let mut sim : Array Nat := #[]
  let mut ren : Array Nat := #[]
  let mut blit : Array Nat := #[]
  for _ in [0:frames] do
    let a ← IO.monoNanosNow
    -- Turn right as if the remote's arrow were held.
    let inp := gResponder s.inp true Doom.TV.Keys.KEY_RIGHTARROW.toNat.toUInt32
    match runTic { s with inp } with
    | .ok s' => s := s'
    | .error e => log s!"bench: engine error: {e}"; return 1
    let b ← IO.monoNanosNow
    let frame := s.frame
    s := { s with frame := {} }
    match render s frame with
    | .error e => log s!"bench: render error: {e}"; return 1
    | .ok (pal, frame) =>
      let c ← IO.monoNanosNow
      let _ ← benchBlit frame.fb.pixels pal
      let d ← IO.monoNanosNow
      s := { s with frame }
      sim := sim.push (b - a); ren := ren.push (c - b); blit := blit.push (d - c)
  let report (name : String) (xs : Array Nat) : IO Unit := do
    let v := xs.qsort (· < ·)
    let ms (ns : Nat) := s!"{ns / 1000000}.{toString (1000 + ns / 1000 % 1000) |>.drop 1}"
    let mean := v.foldl (· + ·) 0 / max v.size 1
    log s!"bench: {name}: mean {ms mean} ms, median {ms v[v.size / 2]!} ms, p95 {ms v[v.size * 95 / 100]!} ms"
  log s!"bench: {frames} frames of E{s.gs.gameepisode}M{s.gs.gamemap}, turning"
  report "playsim" sim
  report "render" ren
  report "convert 1280x720" blit
  let total := (sim.zip ren).zip blit |>.map (fun ((a, b), c) => a + b + c)
  report "total" total
  return 0

def main (args : List String) : IO UInt32 := do
  let t0 ← IO.monoMsNow
  let iwad ← match argValue args "doom.iwad" with
    | some p => pure p
    | none => do
      let cands : List String := ["/doom/freedoom1.wad", "/freedoom1.wad", "/doom/freedm.wad"]
      let found ← cands.filterM fun (p : String) => System.FilePath.pathExists ⟨p⟩
      pure (found.headD "/doom/freedoom1.wad")
  let warp := match (argValue args "doom.warp").map (·.splitOn ",") with
    | some [e, m] => some (e.toNat?.getD 1, m.toNat?.getD 1)
    | some [m] => some (1, m.toNat?.getD 1)
    | _ => none
  let skill := (argValue args "doom.skill").bind String.toNat? |>.getD 2
  let statsEvery := (argValue args "doom.stats").bind String.toNat? |>.getD 5
  let wad ← loadFile iwad
  let mode := Menu.gameMode wad
  log s!"doomtv (Lean engine) iwad {iwad} ({repr mode}), skill {skill}"
  let .ok data := initData wad | log "doom: renderer init failed"; return 1
  let bench? := (argValue args "doom.bench").bind String.toNat?
  let (episode, map) := warp.getD (1, 1)
  let start := if warp.isSome || bench?.isSome then startLevel wad mode episode map (Int32.ofNat skill)
    else Doom.Game.initState wad
  let gs ← match start with
    | .ok gs => pure gs
    | .error e => log s!"doom: cannot start {mapName mode episode map}: {e}"; return 1
  let mut s : Session := { wad, data, mode, gs, phase := .title 0 ((pages mode)[0]!.2), args,
                           front := .init wad (mode == .commercial) (mode == .retail) }
  if warp.isSome || bench?.isSome then s := { s with phase := .game }
  if let some n := bench? then return (← bench s n)
  let r ← displayOpen
  if r != 0 then log s!"display: open failed ({statusText r})"; return 1
  let r ← inputOpen
  if r != 0 then log s!"input: open failed ({statusText r})"
  if (argValue args "doom.sound").getD "1" != "0" then
    let r ← audioOpen 48000 2
    if r != 0 then log s!"audio: open failed ({statusText r}); no sound"
    else
      let lump (name : String) := (checkNumForName wad name).bind fun i => (lumpData wad i).toOption
      match Doom.Audio.Sound.System.create lump { rate := 48000 } with
      | .ok snd =>
        s := { s with sound := some snd }
        s := if s.phase matches .title .. then titleMusic s else drainSound (levelSound s)
        log "audio: 48000 Hz stereo, 8 sfx channels, OPL music"
      | .error e => log s!"audio: {e}; no sound"
  log s!"doom: engine up in {(← IO.monoMsNow) - t0} ms"
  -- doom.net=host[,dm] or doom.net=join[,IP]: go straight to a LAN lobby.
  match (argValue args "doom.net").map (·.splitOn ",") with
  | some ("host" :: rest) => s ← openNet s true (if rest == ["dm"] then 2 else 1) skill episode (← IO.monoMsNow)
  | some ("join" :: _) => s ← openNet s false 1 skill episode (← IO.monoMsNow)
  | _ => pure ()
  let start ← IO.monoMsNow
  let mut ticsRun := 0
  let mut inGame := false
  let mut lastDemo : Option Nat := none
  let mut statAt := start
  let mut statFrames := 0
  let mut statTics := 0
  let mut renderMs := 0
  let mut ticMs := 0
  s ← pumpAudio s
  let audioWorker := s.audioWorker
  try
    repeat
      let now ← IO.monoMsNow
      let due := (now - start) * 35 / 1000
      let a ← IO.monoMsNow
      match ← pollInput s with
      | .ok s' => s := s'
      | .error e => log s!"doom: menu error: {e}"; return 1
      if s.quit then
        log "doom: quit from the menu"
        return 0
      for e in s.pendingNet do
        match e with
        | .hostGame m k ep => s ← openNet s true m k ep now
        | .joinGame => s ← openNet s false 1 skill 1 now
        | _ => pure ()
      s := { s with pendingNet := #[] }
      let mut netRan := 0
      if s.net.isSome then
        match ← netFrame s now with
        | .ok (s', n) => s := s'; netRan := n
        | .error e => s ← recover s "engine error in lan game" e
      let mut ran := 0
      while ticsRun ≤ due && ran < 10 do
        match runTic s with
        | .ok s' => s := s'
        | .error e => s ← recover s s!"engine error at tic {s.gametic}" e; break
        ticsRun := ticsRun + 1
        ran := ran + 1
      if ran == 10 then ticsRun := due + 1  -- fell behind: drop tics rather than spiral
      s ← pumpAudio s
      let nowDemo := match s.phase with | .demo i => some i | _ => none
      if nowDemo.isSome && nowDemo != lastDemo then
        log s!"doom: demo {((pages s.mode)[nowDemo.getD 0]?.map (·.1)).getD "?"} playing"
      lastDemo := nowDemo
      let nowGame := s.phase matches .game
      if nowGame && !inGame then
        log s!"doom: level {mapName s.mode s.gs.gameepisode.toNatClampNeg s.gs.gamemap.toNatClampNeg} started, skill {s.gs.gameskill}{if s.net.isSome then " (lan game)" else ""}"
      inGame := nowGame
      let b ← IO.monoMsNow
      -- In a LAN game the session runs the game tics (netRan); the local tic
      -- clock then only ticks the menu and sound, which alone needs no new
      -- picture unless the menu is up. Drawing the same scene again would
      -- take the CPU the lockstep needs to keep up with the other players.
      let lanGame := s.net.isSome && (s.phase matches .game)
      let gameTics := if lanGame then netRan else ran + netRan
      let redraw := gameTics > 0 || (lanGame && ran > 0 && (s.menu.active || s.menu.message.isSome))
      if redraw then
        -- `D_Display`'s wipe start draws M_Random when the game state changes.
        if s.phase matches .game | .demo _ then s := { s with gs := Doom.Game.displayFrame s.gs }
        let frame := s.frame
        s := { s with frame := {} }  -- keep the reused framebuffer uniquely referenced
        match render s frame with
        | .error e => s ← recover { s with frame := {} } "render error" e
        | .ok (pal, frame) =>
          let r ← displayPresent frame.fb.pixels pal
          if r != 0 then log s!"display: present failed ({statusText r})"; return 1
          s := { s with frame }
        statFrames := statFrames + 1
      else
        IO.sleep 2
      let c ← IO.monoMsNow
      statTics := statTics + gameTics
      ticMs := ticMs + (b - a)
      renderMs := renderMs + (if redraw then c - b else 0)
      if statsEvery > 0 && c - statAt ≥ statsEvery * 1000 then
        let wall := (c - statAt).toFloat / 1000
        let per (t : Nat) := (t.toFloat / (max statFrames 1).toFloat)
        log s!"doom: {(statFrames.toFloat / wall).toString} fps, {(statTics.toFloat / wall).toString} tics/s (35 = full speed); per frame: playsim {(per ticMs).toString} ms, render+present {(per renderMs).toString} ms"
        statAt := c
        statFrames := 0
        statTics := 0
        renderMs := 0
        ticMs := 0
    return 0
  finally
    if let some worker := audioWorker then worker.stop
