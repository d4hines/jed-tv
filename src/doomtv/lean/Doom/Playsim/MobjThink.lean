import Doom.Playsim.Angle
import Doom.Playsim.Core
import Doom.Playsim.CoreBase
import Doom.Playsim.Defs
import Doom.Playsim.GameState
import Doom.Playsim.MapUtil
import Doom.Playsim.MobjBase
import Doom.Playsim.Sound

/-!
# Doom.Playsim.MobjThink

`P_MobjThinker` and its movement: `P_XYMovement` with `P_SlideMove`
(`p_map.c`), `P_ZMovement`, `P_NightmareRespawn` (`p_mobj.c`).
-/

namespace Doom.Playsim.MobjThink

open Doom.Playsim.Angle
open Doom.Playsim.Core
open Doom.Playsim.CoreBase
open Doom.Playsim.Defs
open Doom.Playsim.Fixed
open Doom.Playsim.Flags
open Doom.Playsim.GameState
open Doom.Playsim.Level
open Doom.Playsim.MapUtil
open Doom.Playsim.Mobj
open Doom.Playsim.MobjBase
open Doom.Playsim.Player
open Doom.Playsim.Sound
open Doom.Playsim.Thinker

def STOPSPEED : Int32 := 0x1000
def FRICTION : Int32 := 0xe800
def GRAVITY : Int32 := FRACUNIT
def CF_NOMOMENTUM : Int32 := 4

/-- `P_HitSlideLine`. -/
def hitSlideLine (l : Nat) : M Unit := do
  let gs ← get
  let ld := gs.level.lines[l]!
  if ld.slopetype == ST_HORIZONTAL then set { gs with tmymove := 0 }; return
  if ld.slopetype == ST_VERTICAL then set { gs with tmxmove := 0 }; return
  let sm := gs.mobjs[gs.slidemo.toNatClampNeg]!
  let side := pointOnLineSide gs sm.x sm.y ld
  let mut lineangle := pointToAngle2 0 0 ld.dx ld.dy
  if side == 1 then lineangle := lineangle + ANG180
  let moveangle := pointToAngle2 0 0 gs.tmxmove gs.tmymove
  let mut deltaangle := moveangle - lineangle
  if deltaangle > ANG180 then deltaangle := deltaangle + ANG180
  let movelen := aproxDistance gs.tmxmove gs.tmymove
  let newlen := fixedMul movelen (fineCos deltaangle)
  set { gs with tmxmove := fixedMul newlen (fineCos lineangle)
                tmymove := fixedMul newlen (fineSin lineangle) }

/-- `PTR_SlideTraverse`. -/
def ptrSlideTraverse (frac : Int32) (isaline : Bool) (d : Nat) : M Bool := do
  if !isaline then
    iError "PTR_SlideTraverse: not a line?"
    return false
  let gs ← get
  let li := gs.level.lines[d]!
  let sm := gs.mobjs[gs.slidemo.toNatClampNeg]!
  let mut blocking := false
  if (li.flags &&& ML_TWOSIDED) == 0 then
    if pointOnLineSide gs sm.x sm.y li != 0 then return true
    blocking := true
  else
    lineOpening li
    let gs ← get
    if gs.openrange < sm.height then blocking := true
    else if gs.opentop - sm.z < sm.height then blocking := true
    else if gs.openbottom - sm.z > 24 * FRACUNIT then blocking := true
  if !blocking then return true
  let gs ← get
  if frac < gs.bestslidefrac then
    set { gs with secondslidefrac := gs.bestslidefrac, secondslideline := gs.bestslideline
                  bestslidefrac := frac, bestslideline := d.toInt32 }
  return false

/-- `P_SlideMove`. -/
partial def slideMove (mi : Nat) : M Unit := do
  modify fun gs => { gs with slidemo := mi.toInt32 }
  let stairstep : M Unit := do
    let mo ← getMo mi
    if !(← tryMove mi mo.x (mo.y + mo.momy)) then
      let mo ← getMo mi
      let _ ← tryMove mi (mo.x + mo.momx) mo.y
  let mut hitcount := 0
  repeat
    hitcount := hitcount + 1
    if hitcount == 3 then stairstep; return
    let mo ← getMo mi
    let (leadx, trailx) := if mo.momx > 0 then (mo.x + mo.radius, mo.x - mo.radius)
                           else (mo.x - mo.radius, mo.x + mo.radius)
    let (leady, traily) := if mo.momy > 0 then (mo.y + mo.radius, mo.y - mo.radius)
                           else (mo.y - mo.radius, mo.y + mo.radius)
    modify fun gs => { gs with bestslidefrac := FRACUNIT + 1 }
    let _ ← pathTraverse leadx leady (leadx + mo.momx) (leady + mo.momy) PT_ADDLINES ptrSlideTraverse
    let _ ← pathTraverse trailx leady (trailx + mo.momx) (leady + mo.momy) PT_ADDLINES ptrSlideTraverse
    let _ ← pathTraverse leadx traily (leadx + mo.momx) (traily + mo.momy) PT_ADDLINES ptrSlideTraverse
    if (← get).bestslidefrac == FRACUNIT + 1 then stairstep; return
    modify fun gs => { gs with bestslidefrac := gs.bestslidefrac - 0x800 }
    let bsf := (← get).bestslidefrac
    if bsf > 0 then
      let mo ← getMo mi
      let newx := fixedMul mo.momx bsf
      let newy := fixedMul mo.momy bsf
      if !(← tryMove mi (mo.x + newx) (mo.y + newy)) then stairstep; return
    let gs ← get
    let mut b := FRACUNIT - (gs.bestslidefrac + 0x800)
    if b > FRACUNIT then b := FRACUNIT
    set { gs with bestslidefrac := b }
    if b <= 0 then return
    let mo ← getMo mi
    modify fun gs => { gs with tmxmove := fixedMul mo.momx b, tmymove := fixedMul mo.momy b }
    hitSlideLine (← get).bestslideline.toNatClampNeg
    let gs ← get
    let tmx := gs.tmxmove
    let tmy := gs.tmymove
    modifyMo mi fun m => { m with momx := tmx, momy := tmy }
    let mo ← getMo mi
    if !(← tryMove mi (mo.x + tmx) (mo.y + tmy)) then continue
    return

/-- `P_XYMovement`. -/
def xyMovement (mi : Nat) : M Unit := do
  let mo ← getMo mi
  if mo.momx == 0 && mo.momy == 0 then
    if (mo.flags &&& MF_SKULLFLY) != 0 then
      modifyMo mi fun m => { m with flags := m.flags &&& ~~~MF_SKULLFLY, momx := 0, momy := 0, momz := 0 }
      let _ ← setMobjState mi (← getInfo mo.typeId).spawnstate
    return
  let clamp (v : Int32) : Int32 := if v > MAXMOVE then MAXMOVE else if v < -MAXMOVE then -MAXMOVE else v
  modifyMo mi fun m => { m with momx := clamp m.momx, momy := clamp m.momy }
  let mo ← getMo mi
  let mut xmove := mo.momx
  let mut ymove := mo.momy
  repeat
    let mo ← getMo mi
    let mut ptryx := mo.x + xmove
    let mut ptryy := mo.y + ymove
    if xmove > MAXMOVE / 2 || ymove > MAXMOVE / 2 then
      ptryx := mo.x + xmove / 2
      ptryy := mo.y + ymove / 2
      xmove := xmove >>> 1
      ymove := ymove >>> 1
    else
      xmove := 0
      ymove := 0
    if !(← tryMove mi ptryx ptryy) then
      let mo ← getMo mi
      if mo.player >= 0 then slideMove mi
      else if (mo.flags &&& MF_MISSILE) != 0 then
        let gs ← get
        let cl := gs.ceilingline
        if cl >= 0 then
          let ld := gs.level.lines[cl.toNatClampNeg]!
          if ld.backsector >= 0 && gs.sectors[ld.backsector.toNatClampNeg]!.ceilingpicNum == gs.res.skyflatnum then
            removeMobj mi
            return
        explodeMissile mi
      else
        modifyMo mi fun m => { m with momx := 0, momy := 0 }
    if xmove == 0 && ymove == 0 then break
  let gs ← get
  let mo := gs.mobjs[mi]!
  let player? := if mo.player >= 0 then some gs.players[mo.player.toNatClampNeg]! else none
  if let some p := player? then
    if (p.cheats &&& CF_NOMOMENTUM) != 0 then
      modifyMo mi fun m => { m with momx := 0, momy := 0 }
      return
  if (mo.flags &&& (MF_MISSILE ||| MF_SKULLFLY)) != 0 then return
  if mo.z > mo.floorz then return
  if (mo.flags &&& MF_CORPSE) != 0 then
    if mo.momx > FRACUNIT / 4 || mo.momx < -FRACUNIT / 4 || mo.momy > FRACUNIT / 4 || mo.momy < -FRACUNIT / 4 then
      if mo.floorz != gs.sectors[moSector gs mo]!.floorheight then return
  let idle := match player? with
    | none => true
    | some p => p.cmd.forwardmove == 0 && p.cmd.sidemove == 0
  if mo.momx > -STOPSPEED && mo.momx < STOPSPEED && mo.momy > -STOPSPEED && mo.momy < STOPSPEED && idle then
    if let some p := player? then
      let pmo := gs.mobjs[p.mo.toNatClampNeg]!
      if pmo.state - S_PLAY_RUN1 < 4 then
        let _ ← setMobjState p.mo.toNatClampNeg S_PLAY
    modifyMo mi fun m => { m with momx := 0, momy := 0 }
  else
    modifyMo mi fun m => { m with momx := fixedMul m.momx FRICTION, momy := fixedMul m.momy FRICTION }

/-- `P_ZMovement`. -/
def zMovement (mi : Nat) : M Unit := do
  let gs ← get
  let mo := gs.mobjs[mi]!
  if mo.player >= 0 && mo.z < mo.floorz then
    modifyPl mo.player.toNatClampNeg fun p =>
      let vh := p.viewheight - (mo.floorz - mo.z)
      { p with viewheight := vh, deltaviewheight := (VIEWHEIGHT - vh) >>> 3 }
  modifyMo mi fun m => { m with z := m.z + m.momz }
  let mo ← getMo mi
  if (mo.flags &&& MF_FLOAT) != 0 && mo.target >= 0 then
    if (mo.flags &&& MF_SKULLFLY) == 0 && (mo.flags &&& MF_INFLOAT) == 0 then
      let t ← getMoI mo.target
      let dist := aproxDistance (mo.x - t.x) (mo.y - t.y)
      let delta := (t.z + (mo.height >>> 1)) - mo.z
      if delta < 0 && dist < -(delta * 3) then modifyMo mi fun m => { m with z := m.z - FLOATSPEED }
      else if delta > 0 && dist < delta * 3 then modifyMo mi fun m => { m with z := m.z + FLOATSPEED }
  let gs ← get
  let correctBounce := gs.res.gameversion >= exe_ultimate
  let mo := gs.mobjs[mi]!
  if mo.z <= mo.floorz then
    if correctBounce && (mo.flags &&& MF_SKULLFLY) != 0 then
      modifyMo mi fun m => { m with momz := -m.momz }
    let mo ← getMo mi
    if mo.momz < 0 then
      if mo.player >= 0 && mo.momz < -GRAVITY * 8 then
        modifyPl mo.player.toNatClampNeg fun p => { p with deltaviewheight := mo.momz >>> 3 }
        startSoundMo mi sfx_oof
      modifyMo mi fun m => { m with momz := 0 }
    modifyMo mi fun m => { m with z := m.floorz }
    if !correctBounce && (mo.flags &&& MF_SKULLFLY) != 0 then
      modifyMo mi fun m => { m with momz := -m.momz }
    if (mo.flags &&& MF_MISSILE) != 0 && (mo.flags &&& MF_NOCLIP) == 0 then
      explodeMissile mi
      return
  else if (mo.flags &&& MF_NOGRAVITY) == 0 then
    modifyMo mi fun m => { m with momz := if m.momz == 0 then -GRAVITY * 2 else m.momz - GRAVITY }
  let mo ← getMo mi
  if mo.z + mo.height > mo.ceilingz then
    modifyMo mi fun m =>
      let m := if m.momz > 0 then { m with momz := 0 } else m
      { m with z := m.ceilingz - m.height }
    if (mo.flags &&& MF_SKULLFLY) != 0 then modifyMo mi fun m => { m with momz := -m.momz }
    if (mo.flags &&& MF_MISSILE) != 0 && (mo.flags &&& MF_NOCLIP) == 0 then
      explodeMissile mi

/-- `P_NightmareRespawn`. -/
def nightmareRespawn (mi : Nat) : M Unit := do
  let mobj ← getMo mi
  let x := mobj.spawnX <<< 16
  let y := mobj.spawnY <<< 16
  if !(← checkPosition mi x y) then return
  let gs ← get
  let mobj := gs.mobjs[mi]!
  let fog ← spawnMobj mobj.x mobj.y gs.sectors[moSector gs mobj]!.floorheight MT_TFOG
  startSoundMo fog sfx_telept
  let gs ← get
  let ss := pointInSubsector gs.level x y
  let fog ← spawnMobj x y gs.sectors[gs.level.subsectors[ss]!.sector.toNat]!.floorheight MT_TFOG
  startSoundMo fog sfx_telept
  let info ← getInfo mobj.typeId
  let z := if (info.flags &&& MF_SPAWNCEILING) != 0 then ONCEILINGZ else ONFLOORZ
  let mo ← spawnMobj x y z mobj.typeId
  modifyMo mo fun m => {
    m with
    spawnX := mobj.spawnX, spawnY := mobj.spawnY, spawnAngle := mobj.spawnAngle
    spawnType := mobj.spawnType, spawnOptions := mobj.spawnOptions
    angle := ANG45 * (mobj.spawnAngle / 45).toUInt32
    flags := if (mobj.spawnOptions &&& 8) != 0 then m.flags ||| MF_AMBUSH else m.flags
    reactiontime := 18 }
  removeMobj mi

/-- `P_MobjThinker`. -/
def mobjThinker (mi : Nat) : M Unit := do
  let mut mo ← getMo mi
  if mo.momx != 0 || mo.momy != 0 || (mo.flags &&& MF_SKULLFLY) != 0 then
    xyMovement mi
    mo ← getMo mi
    if mo.func == THF_REMOVED then return
  if mo.z != mo.floorz || mo.momz != 0 then
    zMovement mi
    mo ← getMo mi
    if mo.func == THF_REMOVED then return
  if mo.tics != -1 then
    modifyMo mi fun m => { m with tics := m.tics - 1 }
    if mo.tics - 1 == 0 then
      let _ ← setMobjState mi (← getState mo.state).nextstate
  else
    if (mo.flags &&& MF_COUNTKILL) == 0 then return
    let gs ← get
    if !gs.respawnmonsters then return
    modifyMo mi fun m => { m with movecount := m.movecount + 1 }
    if (← getMo mi).movecount < 12 * 35 then return
    if ((← get).leveltime &&& 31) != 0 then return
    if (← pRandomM) > 4 then return
    nightmareRespawn mi

end Doom.Playsim.MobjThink
