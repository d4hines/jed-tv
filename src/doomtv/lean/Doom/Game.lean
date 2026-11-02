import Doom.Playsim.Core
import Doom.Playsim.Defs
import Doom.Playsim.Finale
import Doom.Playsim.GameState
import Doom.Playsim.Intermission
import Doom.Playsim.MobjBase
import Doom.Playsim.Player
import Doom.Playsim.Random
import Doom.Playsim.Resources
import Doom.Playsim.Setup
import Doom.Playsim.Sound
import Doom.Playsim.Tick
import Doom.Wad

/-!
# Doom.Game

Pure game flow (`g_game.c`) for the front end: start a game, run one tic
with one command per player, and everything that follows from it — level
exits and secret exits, the intermission, the next map (Doom 1 E?M9 rules,
Doom II MAP31/32), episode/game end into the finale, player death and reborn
(single player reloads the level; coop and deathmatch respawn), demo
playback, a 32-bit checksum for net desync detection, and the per-tic audio
event list.

Drawing (menus, intermission, finale, automap) and the main loop are not
here; the front end reads `GameState` (`gamestate`, `wi`, `finale`,
`players`, ...) to draw.

## Use

```
let gs ← IO.ofExcept (Game.newGame wad { skill := 2, episode := 1, map := 1 })
-- each tic:
let gs := Game.ticker gs #[cmd0, cmd1, cmd2, cmd3]   -- one per player slot
-- gs.soundEvents: sounds/music of this tic
-- after drawing a frame:
let gs := Game.displayFrame gs
```
-/

namespace Doom.Game

open Doom.Playsim.Core
open Doom.Playsim.Defs
open Doom.Playsim.Finale
open Doom.Playsim.GameState
open Doom.Playsim.Intermission
open Doom.Playsim.MobjBase
open Doom.Playsim.Player
open Doom.Playsim.Random
open Doom.Playsim.Resources
open Doom.Playsim.Setup
open Doom.Playsim.Sound
open Doom.Playsim.Thinker
open Doom.Playsim.Tick
open Doom.Wad

/-- Options of a new game (`G_InitNew` plus the netgame parameters). -/
structure NewGameOptions where
  skill : Int32 := sk_medium
  episode : Int32 := 1
  map : Int32 := 1
  /-- Player slots in the game (`playeringame[]`). -/
  playeringame : Array Bool := #[true, false, false, false]
  /-- This machine's player. -/
  consoleplayer : Nat := 0
  /-- 0 = coop / single, 1 = deathmatch, 2 = altdeath (items respawn). -/
  deathmatch : Int32 := 0
  nomonsters : Bool := false
  respawn : Bool := false
  fast : Bool := false
  /-- `netgame`; `none` means "more than one player in game". -/
  netgame : Option Bool := none
  /-- Deathmatch time limit in minutes (`-timer`); 0 = none. -/
  timelimit : Int32 := 0
  deriving Repr

/-! ## g_game.c -/

/-- `G_PlayerFinishLevel`. -/
def playerFinishLevel (i : Nat) : M Unit := do
  let p ← getPl i
  modifyPl i fun p => {
    p with
    powers := Array.replicate NUMPOWERS 0
    cards := Array.replicate NUMCARDS false, extralight := 0, fixedcolormap := 0
    damagecount := 0, bonuscount := 0 }
  if p.mo >= 0 then
    modifyMo p.mo.toNatClampNeg fun m => { m with flags := m.flags &&& ~~~Doom.Playsim.Flags.MF_SHADOW }

/-- `G_DoLoadLevel`. -/
def doLoadLevel : M Unit := do
  modify fun gs => {
    gs with
    levelstarttic := gs.gametic
    wipegamestate := if gs.wipegamestate == gamestateLevel then -1 else gs.wipegamestate
    gamestate := gamestateLevel }
  for i in [0:4] do
    modify fun gs => { gs with turbodetected := gs.turbodetected.set! i false }
    let gs ← get
    if gs.playeringame[i]! && gs.players[i]!.playerstate == PST_DEAD then
      modifyPl i fun p => { p with playerstate := PST_REBORN }
    modifyPl i fun p => { p with frags := Array.replicate 4 0 }
  setupLevel
  modify fun gs => { gs with displayplayer := gs.consoleplayer, gameaction := ga_nothing
                             paused := false }

/-- `G_InitNew(skill, episode, map)`. -/
def initNew (skill0 episode0 map0 : Int32) : M Unit := do
  let gs ← get
  let skill := if skill0 > sk_nightmare then sk_nightmare else skill0
  let mut episode := episode0
  if gs.res.gameversion >= exe_ultimate then
    if episode == 0 then episode := 4
  else
    if episode < 1 then episode := 1
    if episode > 3 then episode := 3
  if episode > 1 && gs.res.gamemode == gm_shareware then episode := 1
  let mut map := map0
  if map < 1 then map := 1
  if map > 9 && gs.res.gamemode != gm_commercial then map := 9
  set { gs with rng := clearRandom, paused := false
                respawnmonsters := skill == sk_nightmare || gs.respawnparm }
  let gs ← get
  if gs.fastparm || (skill == sk_nightmare && gs.gameskill != sk_nightmare) then
    let st := Id.run do
      let mut st := gs.states
      for i in [S_SARG_RUN1.toNat:S_SARG_PAIN2.toNat + 1] do
        st := st.modify i fun s => { s with tics := s.tics >>> 1 }
      return st
    let mi := gs.mobjinfo
      |>.modify MT_BRUISERSHOT.toNatClampNeg (fun m => { m with speed := 20 * 65536 })
      |>.modify MT_HEADSHOT.toNatClampNeg (fun m => { m with speed := 20 * 65536 })
      |>.modify MT_TROOPSHOT.toNatClampNeg (fun m => { m with speed := 20 * 65536 })
    set { gs with states := st, mobjinfo := mi }
  else if skill != sk_nightmare && gs.gameskill == sk_nightmare then
    let st := Id.run do
      let mut st := gs.states
      for i in [S_SARG_RUN1.toNat:S_SARG_PAIN2.toNat + 1] do
        st := st.modify i fun s => { s with tics := s.tics <<< 1 }
      return st
    let mi := gs.mobjinfo
      |>.modify MT_BRUISERSHOT.toNatClampNeg (fun m => { m with speed := 15 * 65536 })
      |>.modify MT_HEADSHOT.toNatClampNeg (fun m => { m with speed := 10 * 65536 })
      |>.modify MT_TROOPSHOT.toNatClampNeg (fun m => { m with speed := 10 * 65536 })
    set { gs with states := st, mobjinfo := mi }
  modify fun gs => {
    gs with
    players := gs.players.map fun p => { p with playerstate := PST_REBORN }
    usergame := true, paused := false, demoplayback := false, automapactive := false
    viewactive := true, gameepisode := episode, gamemap := map, gameskill := skill }
  doLoadLevel

/-- `G_DoReborn(playernum)`. -/
def doReborn (pn : Nat) : M Unit := do
  let gs ← get
  if !gs.netgame then
    set { gs with gameaction := ga_loadlevel }
    return
  let p := gs.players[pn]!
  if p.mo >= 0 then modifyMo p.mo.toNatClampNeg fun m => { m with player := -1 }
  if gs.deathmatchMode != 0 then
    deathMatchSpawnPlayer pn
    return
  if ← checkSpot pn (← get).playerstarts[pn]! then
    spawnPlayer (← get).playerstarts[pn]!
    return
  for i in [0:4] do
    if ← checkSpot pn (← get).playerstarts[i]! then
      modify fun gs => { gs with playerstarts := gs.playerstarts.modify i fun s => { s with type_ := pn.toInt32 + 1 } }
      spawnPlayer (← get).playerstarts[i]!
      modify fun gs => { gs with playerstarts := gs.playerstarts.modify i fun s => { s with type_ := i.toInt32 + 1 } }
      return
  spawnPlayer (← get).playerstarts[pn]!

def pars : Array (Array Int32) := #[#[0],
  #[0,30,75,120,90,165,180,180,30,165], #[0,90,90,90,120,90,360,240,30,170],
  #[0,90,45,90,150,90,90,165,30,135]]
def cpars : Array Int32 := #[30,90,120,120,90,150,120,120,270,90, 210,150,150,150,210,150,420,150,210,150,
  240,150,180,150,150,300,330,420,300,180, 120,30]

/-- `G_DoCompleted`. -/
def doCompleted : M Unit := do
  modify fun gs => { gs with gameaction := ga_nothing }
  for i in [0:4] do
    if (← get).playeringame[i]! then playerFinishLevel i
  modify fun gs => { gs with automapactive := false }
  let gs ← get
  let commercial := gs.res.gamemode == gm_commercial
  if !commercial then
    if gs.gamemap == 8 then
      set { gs with gameaction := ga_victory }
      return
    if gs.gamemap == 9 then
      modify fun gs => { gs with players := gs.players.map fun p => { p with didsecret := true } }
  let gs ← get
  let m := gs.gamemap
  let next : Int32 :=
    if commercial then
      if gs.secretexit then (if m == 15 then 30 else if m == 31 then 31 else gs.wminfo.next)
      else if m == 31 || m == 32 then 15 else m
    else if gs.secretexit then 8
    else if m == 9 then
      (if gs.gameepisode == 1 then 3 else if gs.gameepisode == 2 then 5
       else if gs.gameepisode == 3 then 6 else if gs.gameepisode == 4 then 2 else gs.wminfo.next)
    else m
  let partime : Int32 :=
    if commercial then 35 * (cpars[(m - 1).toNatClampNeg]?.getD 0)
    else if gs.gameepisode < 4 then
      35 * ((pars[gs.gameepisode.toNatClampNeg]?.getD #[]).getD m.toNatClampNeg 0)
    else 35 * (cpars[m.toNatClampNeg]?.getD 0)
  let plyr := Array.ofFn (n := 4) fun i =>
    let p := gs.players[i]!
    ({ in_ := gs.playeringame[i]!, skills := p.killcount, sitems := p.itemcount
       ssecret := p.secretcount, stime := gs.leveltime.toInt32, frags := p.frags } : WbPlayer)
  set {
    gs with
    wminfo := { didsecret := gs.players[gs.consoleplayer]!.didsecret, epsd := gs.gameepisode - 1
                last := m - 1, next, maxkills := gs.totalkills, maxitems := gs.totalitems
                maxsecret := gs.totalsecret, maxfrags := 0, partime
                pnum := gs.consoleplayer.toInt32, plyr }
    gamestate := gamestateIntermission, viewactive := false, automapactive := false }
  wiStart

/-- `G_WorldDone`. -/
def worldDone : M Unit := do
  modify fun gs => { gs with gameaction := ga_worlddone }
  let gs ← get
  if gs.secretexit then
    modifyPl gs.consoleplayer fun p => { p with didsecret := true }
  if gs.res.gamemode == gm_commercial then
    let m := gs.gamemap
    if ((m == 15 || m == 31) && gs.secretexit) || m == 6 || m == 11 || m == 20 || m == 30 then
      startFinale

/-- `G_DoWorldDone`. -/
def doWorldDone : M Unit := do
  modify fun gs => { gs with gamestate := gamestateLevel, gamemap := gs.wminfo.next + 1 }
  doLoadLevel
  modify fun gs => { gs with gameaction := ga_nothing, viewactive := true }

/-- `G_DoPlayDemo` from `demoBytes` (13-byte v1.9 header). -/
def doPlayDemo : M Unit := do
  let gs ← get
  let d := gs.demoBytes
  set { gs with gameaction := ga_nothing }
  if d.size < 13 then iError "demo too short"; return
  let ver := d[0]!
  if ver != 109 then
    iError s!"Demo is from a different game version! (read {ver}, should be 109)"
    return
  let pig := Array.ofFn (n := 4) fun i => d[9 + i.val]! != 0
  set { gs with deathmatchMode := d[4]!.toUInt32.toInt32, deathmatch := d[4]! != 0
                respawnparm := d[5]! != 0, fastparm := d[6]! != 0, nomonsters := d[7]! != 0
                consoleplayer := d[8]!.toNat, playeringame := pig, demoCursor := 13
                netgame := pig[1]! }
  initNew d[1]!.toUInt32.toInt32 d[2]!.toUInt32.toInt32 d[3]!.toUInt32.toInt32
  modify fun gs => { gs with usergame := false, demoplayback := true }

/-- `G_ReadDemoTiccmd` for player `i`. Sets `demoEnded` at the marker. -/
def readDemoTiccmd (i : Nat) : M Unit := do
  let gs ← get
  let c := gs.demoCursor
  let d := gs.demoBytes
  if c >= d.size || d[c]! == 0x80 then
    -- G_CheckDemoStatus: playback ends (the oracle quits here under -tracedemo)
    set { gs with demoEnded := true, demoplayback := false }
    return
  let sx (b : UInt8) : Int32 := b.toUInt32.toUInt8.toInt8.toInt32
  let cmd : TicCmd := { forwardmove := sx d[c]!, sidemove := sx d[c+1]!
                        angleturn := sx d[c+2]! <<< 8, buttons := d[c+3]!.toUInt32 }
  set { gs with demoCursor := c + 4, players := gs.players.modify i fun p => { p with cmd } }

def playerNames : Array String := #["Green", "Indigo", "Brown", "Red"]

/-- `G_Ticker`. `cmds[i]` is player `i`'s command (`netcmds`). -/
def gTicker (cmds : Array TicCmd) : M Unit := do
  modify fun gs => { gs with soundEvents := #[] }
  for i in [0:4] do
    let gs ← get
    if gs.playeringame[i]! && gs.players[i]!.playerstate == PST_REBORN then doReborn i
  let mut guard := 0
  while (← get).gameaction != ga_nothing && guard < 16 do
    guard := guard + 1
    let ga := (← get).gameaction
    if ga == ga_loadlevel then doLoadLevel
    else if ga == ga_playdemo then doPlayDemo
    else if ga == ga_completed then doCompleted
    else if ga == ga_victory then startFinale
    else if ga == ga_worlddone then doWorldDone
    else modify fun gs => { gs with gameaction := ga_nothing }
    if (← get).error.isSome then return
  for i in [0:4] do
    let gs ← get
    if !gs.playeringame[i]! then continue
    let cmd := cmds[i]?.getD TicCmd.zero
    modifyPl i fun p => { p with cmd }
    if (← get).demoplayback then readDemoTiccmd i
    let gs ← get
    if gs.players[i]!.cmd.forwardmove > 0x32 then
      set { gs with turbodetected := gs.turbodetected.set! i true }
    let gs ← get
    if (gs.gametic &&& 31) == 0 && ((gs.gametic >>> 5) % 4).toNat == i && gs.turbodetected[i]! then
      modifyPl gs.consoleplayer fun p => { p with message := some s!"{playerNames[i]!} is turbo!" }
      modify fun gs => { gs with turbodetected := gs.turbodetected.set! i false }
  for i in [0:4] do
    let gs ← get
    if gs.playeringame[i]! then
      let b := gs.players[i]!.cmd.buttons
      if (b &&& 128) != 0 && (b &&& 3) == 1 then
        set { gs with paused := !gs.paused }
  modify fun gs => { gs with oldgamestate := gs.gamestate }
  let gs ← get
  if gs.gamestate == gamestateLevel then
    pTicker
    stTicker
    huTicker
  else if gs.gamestate == gamestateIntermission then
    wiTicker worldDone
  else if gs.gamestate == gamestateFinale then
    fTicker

/-! ## Public API -/

/-- Resources and defaults for an IWAD, before any game is started. -/
def initState (wad : WadDirectory) : Except String GameState := do
  let (res, states) ← buildRes wad
  let gs : GameState := default
  return {
    gs with
    res, states, mobjinfo := Doom.Playsim.Info.mobjinfo
    players := Array.replicate MAXPLAYERS { Doom.Playsim.Player.empty with playerstate := PST_LIVE }
    playeringame := #[true, false, false, false]
    commercial := res.gamemode == gm_commercial
    picAnims := res.picAnims
    flattranslation := Array.ofFn (n := res.flatNames.size + 1) fun i => i.val.toInt32
    texturetranslation := Array.ofFn (n := res.textureNames.size + 1) fun i => i.val.toInt32
    gamestate := gamestateDemoScreen }

/-- Apply `NewGameOptions` and run `G_InitNew` on an existing state (keeps the
mutable `states[]` / `mobjinfo[]` history, as a C session does). -/
def restartGame (gs : GameState) (o : NewGameOptions) : GameState :=
  let pig := Array.ofFn (n := 4) fun i => o.playeringame[i]?.getD false
  let nplayers := (pig.filter id).size
  let gs := {
    gs with
    playeringame := pig, consoleplayer := o.consoleplayer
    deathmatchMode := o.deathmatch, deathmatch := o.deathmatch != 0
    nomonsters := o.nomonsters, respawnparm := o.respawn, fastparm := o.fast
    netgame := o.netgame.getD (nplayers > 1), timelimit := o.timelimit
    demoplayback := false, demoEnded := false, error := none, soundEvents := #[] }
  (initNew o.skill o.episode o.map |>.run gs).2

/-- Start a new game on a fresh state for this IWAD. -/
def newGame (wad : WadDirectory) (o : NewGameOptions) : Except String GameState := do
  let gs := restartGame (← initState wad) o
  match gs.error with
  | some e => throw e
  | none => return gs

/-- Prepare demo playback; the first `ticker` call loads the level
(`G_DeferedPlayDemo`). Commands passed to `ticker` are ignored while the
demo plays. -/
def playDemo (wad : WadDirectory) (demo : ByteArray) : Except String GameState := do
  let gs ← initState wad
  return { gs with demoBytes := demo, gameaction := ga_playdemo }

/-- Run one tic (`G_Ticker`) with one command per player slot. -/
def ticker (gs : GameState) (cmds : Array TicCmd) : GameState :=
  (gTicker cmds |>.run gs).2

/-- `PlayerQuitGame` (`d_net.c`): player `i` left a netgame. Its map object
stays in the level; the console player gets the message. Run it before the
tic in which the player is no longer in game (`tickerNet` does this). -/
def playerQuit (gs : GameState) (i : Nat) : GameState :=
  if !(gs.playeringame[i]?.getD false) then gs
  else
    let cs := (dehString gs "Player 1 left the game").toList
    let cs := cs.set 7 (Char.ofNat ((cs.getD 7 '1').toNat + i))
    { gs with playeringame := gs.playeringame.set! i false
              players := gs.players.modify gs.consoleplayer fun p =>
                { p with message := some (String.ofList cs) } }

/-- One netgame tic (`RunTic`): players whose slot is no longer in game quit,
then `G_Ticker` runs. -/
def tickerNet (gs : GameState) (cmds : Array TicCmd) (ingame : Array Bool) : GameState := Id.run do
  let mut gs := gs
  if !gs.demoplayback then
    for i in [0:4] do
      if gs.playeringame[i]! && !(ingame[i]?.getD true) then gs := playerQuit gs i
  return ticker gs cmds

/-- `D_Display` side effect on the game: entering a new `gamestate` (or a
forced wipe) starts a screen melt whose setup draws `SCREENWIDTH` (320)
`M_Random` values. Call once per displayed frame after a tic. -/
def displayFrame (gs : GameState) : GameState := Id.run do
  if gs.gamestate == gs.wipegamestate then return gs
  let mut rng := gs.rng
  for _ in [0:320] do
    rng := (mRandom rng).2
  return { gs with rng, wipegamestate := gs.gamestate }

/-- `F_CastResponder`: a key press during the Doom II cast. -/
def castKeyPress (gs : GameState) : GameState := (castResponder.run gs).2

private def fnv32 (h : UInt32) (v : UInt32) : UInt32 := Id.run do
  let mut h := h
  for k in [0:4] do
    h := (h ^^^ ((v >>> (8 * k).toUInt32) &&& 0xff)) * 16777619
  return h

/-- Deterministic 32-bit checksum of the synchronised game state (players,
live map objects, sectors, RNG) for netgame desync detection. -/
def checksum (gs : GameState) : UInt32 := Id.run do
  let mut h : UInt32 := 2166136261
  h := fnv32 h gs.gametic
  h := fnv32 h gs.gamestate.toUInt32
  h := fnv32 h gs.leveltime
  h := fnv32 h gs.rng.prndindex
  h := fnv32 h gs.gamemap.toUInt32
  for i in [0:4] do
    if gs.playeringame[i]! then
      let p := gs.players[i]!
      h := fnv32 h p.health.toUInt32
      h := fnv32 h p.armorpoints.toUInt32
      h := fnv32 h p.readyweapon.toUInt32
      h := fnv32 h p.playerstate.toUInt32
      if p.mo >= 0 then
        let mo := gs.mobjs[p.mo.toNatClampNeg]!
        h := fnv32 h mo.x.toUInt32
        h := fnv32 h mo.y.toUInt32
        h := fnv32 h mo.z.toUInt32
        h := fnv32 h mo.angle
  if gs.gamestate == gamestateLevel then
    for r in gs.thinkers do
      if thKind r == TK_MOBJ then
        let mo := gs.mobjs[thIdx r]!
        if mo.func == THF_MOBJ then
          h := fnv32 h mo.x.toUInt32
          h := fnv32 h mo.y.toUInt32
          h := fnv32 h mo.z.toUInt32
          h := fnv32 h mo.health.toUInt32
          h := fnv32 h mo.state
    for s in gs.sectors do
      h := fnv32 h s.floorheight.toUInt32
      h := fnv32 h s.ceilingheight.toUInt32
  return h

end Doom.Game
