import Doom.Playsim.NameMap
import Doom.Playsim.Angle
import Doom.Playsim.Core
import Doom.Playsim.Defs
import Doom.Playsim.Doors
import Doom.Playsim.GameState
import Doom.Playsim.Level
import Doom.Playsim.MapUtil
import Doom.Playsim.MobjBase
import Doom.Playsim.Resources
import Doom.Playsim.Sound
import Doom.Playsim.SpecUtil
import Doom.Playsim.Tables
import Doom.Wad

/-!
# Doom.Playsim.Setup

`P_SetupLevel` (`p_setup.c`): map loading (geometry, texture/flat numbers,
blockmap, padded REJECT), `P_LoadThings` / `P_SpawnMapThing` /
`P_SpawnPlayer` (`p_mobj.c`), deathmatch spawning and reborn spot checks
(`g_game.c`), and `P_SpawnSpecials`.
-/

namespace Doom.Playsim.Setup

open Doom.Playsim.Angle
open Doom.Playsim.Core
open Doom.Playsim.Defs
open Doom.Playsim.Doors
open Doom.Playsim.Fixed
open Doom.Playsim.Flags
open Doom.Playsim.GameState
open Doom.Playsim.Level
open Doom.Playsim.MapUtil
open Doom.Playsim.Mobj
open Doom.Playsim.MobjBase
open Doom.Playsim.Player
open Doom.Playsim.Resources
open Doom.Playsim.Sound
open Doom.Playsim.SpecUtil
open Doom.Playsim.Tables
open Doom.Playsim.Thinker
open Doom.Wad

/-- Map label (`E1M1` / `MAP01`). -/
def mapLumpName (commercial : Bool) (episode map : Int32) : String :=
  if commercial then
    if map < 10 then s!"MAP0{map}" else s!"MAP{map}"
  else s!"E{episode}M{map}"

def loadLevelData (wad : WadDirectory) (label : String) : Except String (LevelData × Nat) := do
  match checkNumForName wad label with
  | none => throw s!"W_GetNumForName: {label} not found!"
  | some idx =>
    let l (o : Nat) := mapLumpData wad idx o
    let lv ← buildLevel (← l ML_THINGS) (← l ML_LINEDEFS) (← l ML_SIDEDEFS) (← l ML_VERTEXES)
      (← l ML_SEGS) (← l ML_SSECTORS) (← l ML_NODES) (← l ML_SECTORS) (← l ML_REJECT)
      (← l ML_BLOCKMAP)
    return (lv, idx)

/-- `PadRejectArray`. -/
def padReject (reject : ByteArray) (numsectors totallines : Nat) : ByteArray := Id.run do
  let minlength := (numsectors * numsectors + 7) / 8
  if reject.size >= minlength then return reject
  let pad : Array UInt32 := #[((totallines * 4 + 3) / 4 * 4 + 24).toUInt32, 0, 50, 0x1d4a11]
  let mut out := reject
  let len := minlength - reject.size
  for i in [0:len] do
    if i < 16 then out := out.push ((pad[i / 4]! >>> ((i % 4) * 8).toUInt32).toUInt8)
    else out := out.push 0
  return out

/-- Load a map into the state (the `P_Load*` / `P_GroupLines` / `P_LoadReject` part). -/
def loadMap (label : String) : M Unit := do
  let gs ← get
  match loadLevelData gs.res.wad label with
  | .error e => iError e
  | .ok (level, idx) =>
    let mut err : Option String := none
    let mut sectors : Array SectorRuntime := Array.emptyWithCapacity level.sectors.size
    for s in level.sectors do
      let f ← match flatNumForName gs.res s.floorpic with
        | .ok v => pure v | .error e => do err := some e; pure 0
      let c ← match flatNumForName gs.res s.ceilingpic with
        | .ok v => pure v | .error e => do err := some e; pure 0
      sectors := sectors.push {
        lightlevel := s.lightlevel, special := s.special, tag := s.tag
        floorheight := s.floorheight, ceilingheight := s.ceilingheight
        thinglist := -1, specialdata := -1, soundtarget := -1, soundtraversed := 0
        validcount := 0, lines := s.lines, floorpic := s.floorpic
        floorpicNum := f, ceilingpicNum := c }
    let tnum (name : ByteArray) : Int32 :=
      match textureNumForName gs.res name with | .ok t => t | .error _ => 0
    for sd in level.sides do
      for t in [sd.toptexture, sd.midtexture, sd.bottomtexture] do
        if let .error e := textureNumForName gs.res t then err := some e
    if let some e := err then
      iError e
      return
    let bmapLump ← match mapLumpData gs.res.wad idx ML_BLOCKMAP with
      | .ok b => pure b | .error _ => pure ByteArray.empty
    let mut totallines := 0
    for ld in level.lines do
      totallines := totallines + 1
      if ld.backsector >= 0 && ld.backsector != ld.frontsector then totallines := totallines + 1
    let reject := padReject level.reject level.sectors.size totallines
    let bm := level.blockmap
    let lineGeo := level.lines.map fun (ld : Line) =>
      let v1 := level.vertexes[ld.v1.toNat]!
      let v2 := level.vertexes[ld.v2.toNat]!
      ({ v1x := v1.x, v1y := v1.y, v2x := v2.x, v2y := v2.y, dx := ld.dx, dy := ld.dy
         bbTop := ld.bbox[BOXTOP]!, bbBottom := ld.bbox[BOXBOTTOM]!
         bbLeft := ld.bbox[BOXLEFT]!, bbRight := ld.bbox[BOXRIGHT]!
         flags := ld.flags, slopetype := ld.slopetype, front := ld.frontsector
         back := ld.backsector, sidenum1 := ld.sidenum1 } : LineGeo)
    set {
      gs with
      level
      lineGeo
      sectors
      sideTop := level.sides.map fun (s : Side) => tnum s.toptexture
      sideMid := level.sides.map fun (s : Side) => tnum s.midtexture
      sideBottom := level.sides.map fun (s : Side) => tnum s.bottomtexture
      bmapLump, reject
      bmaporgx := bm.originX, bmaporgy := bm.originY, bmapwidth := bm.width, bmapheight := bm.height
      blocklinks := Array.replicate (bm.width * bm.height).toNatClampNeg 0
      lineValidcount := Array.replicate level.lines.size 0
      mobjs := #[], thinkers := #[]
      lightFlashes := #[], strobes := #[], glows := #[], fireFlickers := #[]
      verticalDoors := #[], ceilings := #[], floors := #[], plats := #[] }

/-- `ST_Start` / `ST_initData` for the console player. -/
def stStart : M Unit := modify fun gs =>
  let p := gs.players[gs.consoleplayer]!
  { gs with stFace := { StFaceState.init with
                                             oldweaponsowned := p.weaponowned
                                             spawnWeaponsOwned := p.weaponowned } }

/-- `P_SpawnPlayer(mthing)`. -/
def spawnPlayer (mt : MapThing) : M Unit := do
  if mt.type_ == 0 then return
  let pn := (mt.type_ - 1).toNatClampNeg
  let gs ← get
  if !gs.playeringame[pn]! then return
  if gs.players[pn]!.playerstate == PST_REBORN then
    modifyPl pn playerReborn
  let mi ← spawnMobj (mt.x <<< 16) (mt.y <<< 16) ONFLOORZ MT_PLAYER
  let p ← getPl pn
  modifyMo mi fun m => {
    m with
    flags := if mt.type_ > 1 then m.flags ||| ((mt.type_ - 1).toUInt32 <<< MF_TRANSSHIFT) else m.flags
    angle := ANG45 * (mt.angle / 45).toUInt32
    player := pn.toInt32, health := p.health }
  modifyPl pn fun p => {
    p with
    mo := mi.toInt32, playerstate := PST_LIVE, refire := 0
    message := none, damagecount := 0, bonuscount := 0, extralight := 0, fixedcolormap := 0
    viewheight := VIEWHEIGHT }
  setupPsprites pn
  if (← get).deathmatchMode != 0 then
    modifyPl pn fun p => { p with cards := Array.replicate NUMCARDS true }
  if pn == (← get).consoleplayer then
    stStart
    modify fun gs => { gs with hu := HuState.init }

/-- Skill bit for `P_SpawnMapThing`. -/
def skillBit (skill : Int32) : Int32 :=
  if skill == sk_baby then 1
  else if skill == sk_nightmare then 4
  else ((1 : UInt32) <<< ((skill - 1).toUInt32 &&& 31)).toInt32

/-- `P_SpawnMapThing(mthing)`. -/
def spawnMapThing (mt : MapThing) : M Unit := do
  let gs ← get
  if mt.type_ == 11 then
    if gs.deathmatchstarts.size < 10 then
      set { gs with deathmatchstarts := gs.deathmatchstarts.push mt }
    return
  if mt.type_ <= 0 then return
  if mt.type_ <= 4 then
    let pn := (mt.type_ - 1).toNatClampNeg
    set { gs with playerstarts := gs.playerstarts.set! pn mt
                  playerstartsingame := gs.playerstartsingame.set! pn true }
    if gs.deathmatchMode == 0 then spawnPlayer mt
    return
  if !gs.netgame && (mt.options &&& 16) != 0 then return
  if (mt.options &&& skillBit gs.gameskill) == 0 then return
  match gs.mobjinfo.findIdx? (·.doomednum == mt.type_) with
  | none => iError s!"P_SpawnMapThing: Unknown type {mt.type_} at ({mt.x}, {mt.y})"
  | some i =>
    let info := gs.mobjinfo[i]!
    if gs.deathmatchMode != 0 && (info.flags &&& MF_NOTDMATCH) != 0 then return
    if gs.nomonsters && (i.toInt32 == MT_SKULL || (info.flags &&& MF_COUNTKILL) != 0) then return
    let z := if (info.flags &&& MF_SPAWNCEILING) != 0 then ONCEILINGZ else ONFLOORZ
    let mi ← spawnMobj (mt.x <<< 16) (mt.y <<< 16) z i.toInt32
    modifyMo mi fun m => { m with spawnX := mt.x, spawnY := mt.y, spawnAngle := mt.angle
                                  spawnType := mt.type_, spawnOptions := mt.options }
    let mo ← getMo mi
    if mo.tics > 0 then
      let r ← pRandomM
      modifyMo mi fun m => { m with tics := 1 + (r % m.tics) }
    let mo ← getMo mi
    if (mo.flags &&& MF_COUNTKILL) != 0 then modify fun gs => { gs with totalkills := gs.totalkills + 1 }
    if (mo.flags &&& MF_COUNTITEM) != 0 then modify fun gs => { gs with totalitems := gs.totalitems + 1 }
    modifyMo mi fun m => {
      m with
      angle := ANG45 * (mt.angle / 45).toUInt32
      flags := if (mt.options &&& 8) != 0 then m.flags ||| MF_AMBUSH else m.flags }

/-- `P_LoadThings`. -/
def loadThings : M Unit := do
  let gs ← get
  for t in gs.level.things do
    if gs.res.gamemode != gm_commercial then
      let ty := t.typeId
      if ty == 68 || ty == 64 || ty == 88 || ty == 89 || ty == 69 || ty == 67 || ty == 71 ||
          ty == 65 || ty == 66 || ty == 84 then break
    spawnMapThing { x := t.x, y := t.y, angle := t.angle, type_ := t.typeId, options := t.options }
    if (← get).error.isSome then return
  let gs ← get
  if gs.deathmatchMode == 0 then
    for i in [0:4] do
      if gs.playeringame[i]! && !(← get).playerstartsingame[i]! then
        iError s!"P_LoadThings: Player {i + 1} start missing (vanilla crashes here)"
        return
      modify fun gs => { gs with playerstartsingame := gs.playerstartsingame.set! i false }

/-- `G_CheckSpot(playernum, mthing)`. -/
def checkSpot (pn : Nat) (mt : MapThing) : M Bool := do
  let gs ← get
  let p := gs.players[pn]!
  if p.mo < 0 then
    for i in [0:pn] do
      let o := gs.mobjs[gs.players[i]!.mo.toNatClampNeg]!
      if o.x == mt.x <<< 16 && o.y == mt.y <<< 16 then return false
    return true
  let x := mt.x <<< 16
  let y := mt.y <<< 16
  if !(← checkPosition p.mo.toNatClampNeg x y) then return false
  let gs ← get
  if gs.bodyqueslot >= 32 then
    let b := gs.bodyque[gs.bodyqueslot % 32]!
    if b != 0 then removeMobj (b - 1)
  modify fun gs => { gs with bodyque := gs.bodyque.set! (gs.bodyqueslot % 32) (p.mo.toNatClampNeg + 1)
                             bodyqueslot := gs.bodyqueslot + 1 }
  let gs ← get
  let ss := pointInSubsector gs.level x y
  let an : Int32 := (ANG45 >>> 19).toInt32 * (mt.angle / 45)
  let (xa, ya) : Int32 × Int32 :=
    if an == 4096 then (finetangent[2048]!, finetangent[0]!)
    else if an == 5120 then (finetangent[3072]!, finetangent[1024]!)
    else if an == 6144 then (finesine[0]!, finetangent[2048]!)
    else if an == 7168 then (finesine[1024]!, finetangent[3072]!)
    else if an == 0 || an == 1024 || an == 2048 || an == 3072 then
      (finecosine[an.toNatClampNeg]!, finesine[an.toNatClampNeg]!)
    else if an == 8192 then (tantoangle[0]!.toInt32, finesine[8192]!)
    else (0, 0)
  if !(an == 4096 || an == 5120 || an == 6144 || an == 7168 || an == 0 || an == 1024 ||
       an == 2048 || an == 3072 || an == 8192) then
    iError s!"G_CheckSpot: unexpected angle {an}"
  let mo ← spawnMobj (x + 20 * xa) (y + 20 * ya)
    gs.sectors[gs.level.subsectors[ss]!.sector.toNat]!.floorheight MT_TFOG
  if (← get).players[(← get).consoleplayer]!.viewz != 1 then startSoundMo mo sfx_telept
  return true

/-- `G_DeathMatchSpawnPlayer(playernum)`. -/
def deathMatchSpawnPlayer (pn : Nat) : M Unit := do
  let selections := (← get).deathmatchstarts.size
  if selections < 4 then
    iError s!"Only {selections} deathmatch spots, 4 required"
    return
  for _ in [0:20] do
    let i := ((← pRandomM) % selections.toInt32).toNatClampNeg
    let spot := (← get).deathmatchstarts[i]!
    if ← checkSpot pn spot then
      let spot := { spot with type_ := pn.toInt32 + 1 }
      modify fun gs => { gs with deathmatchstarts := gs.deathmatchstarts.set! i spot }
      spawnPlayer spot
      return
  spawnPlayer (← get).playerstarts[pn]!

/-- `P_SpawnSpecials`. -/
def spawnSpecials : M Unit := do
  let gs ← get
  if gs.timelimit > 0 && gs.deathmatchMode != 0 then
    set { gs with levelTimer := true, levelTimeCount := gs.timelimit * 60 * 35 }
  else set { gs with levelTimer := false }
  for i in [0:(← get).sectors.size] do
    let sp := (← get).sectors[i]!.special
    if sp == 0 then continue
    if sp == 1 then spawnLightFlash i
    else if sp == 2 then spawnStrobeFlash i FASTDARK false
    else if sp == 3 then spawnStrobeFlash i SLOWDARK false
    else if sp == 4 then
      spawnStrobeFlash i FASTDARK false
      modifySec i fun s => { s with special := 4 }
    else if sp == 8 then spawnGlowingLight i
    else if sp == 9 then modify fun gs => { gs with totalsecret := gs.totalsecret + 1 }
    else if sp == 10 then spawnDoorCloseIn30 i
    else if sp == 12 then spawnStrobeFlash i SLOWDARK true
    else if sp == 13 then spawnStrobeFlash i FASTDARK true
    else if sp == 14 then spawnDoorRaiseIn5Mins i
    else if sp == 17 then
      if (← get).res.gameversion > exe_doom_1_2 then spawnFireFlicker i
  let gs ← get
  let mut ls : Array Nat := #[]
  for i in [0:gs.level.lines.size] do
    if gs.level.lines[i]!.special == 48 then
      if ls.size >= 64 then
        iError "P_SpawnSpecials: Too many scrolling wall linedefs! (Vanilla limit is 64)"
        return
      ls := ls.push i
  set { gs with lineSpecials := ls, activeCeilings := Array.replicate 30 0
                activePlats := Array.replicate 30 0, buttons := Array.replicate 16 {} }

/-- `P_SetupLevel(episode, map, playermask, skill)`. -/
def setupLevel : M Unit := do
  modify fun gs => {
    gs with
    traceIdCounter := 1, totalkills := 0, totalitems := 0, totalsecret := 0
    wminfo := { gs.wminfo with maxfrags := 0, partime := 180 }
    players := gs.players.map fun p => { p with killcount := 0, secretcount := 0, itemcount := 0 } }
  modify fun gs => { gs with players := gs.players.modify gs.consoleplayer fun p => { p with viewz := 1 } }
  sStart
  let gs ← get
  set { gs with leveltime := 0 }
  loadMap (mapLumpName (gs.res.gamemode == gm_commercial) gs.gameepisode gs.gamemap)
  if (← get).error.isSome then return
  modify fun gs => { gs with bodyqueslot := 0, deathmatchstarts := #[] }
  loadThings
  if (← get).error.isSome then return
  if (← get).deathmatchMode != 0 then
    for i in [0:4] do
      if (← get).playeringame[i]! then
        modifyPl i fun p => { p with mo := -1 }
        deathMatchSpawnPlayer i
  modify fun gs => { gs with iquehead := 0, iquetail := 0 }
  spawnSpecials

end Doom.Playsim.Setup
