import Doom.Playsim.Angle
import Doom.Playsim.CoreBase
import Doom.Playsim.Defs
import Doom.Playsim.Doors
import Doom.Playsim.Fixed
import Doom.Playsim.Flags
import Doom.Playsim.Floors
import Doom.Playsim.GameState
import Doom.Playsim.Info
import Doom.Playsim.Inter
import Doom.Playsim.Level
import Doom.Playsim.MapUtil
import Doom.Playsim.MobjBase
import Doom.Playsim.Player
import Doom.Playsim.Sight
import Doom.Playsim.Sound
import Doom.Playsim.SpecDefs
import Doom.Playsim.SpecUtil
import Doom.Playsim.Switch
import Doom.Playsim.Tables
import Doom.Playsim.Weapons

/-!
# Doom.Playsim.Core

The mutually recursive heart of the playsim, ported function by function
from Chocolate Doom: `P_SetMobjState` and every mobj action (`p_enemy.c`),
`P_SetPsprite` and the weapon actions (`p_pspr.c`), `P_DamageMobj` /
`P_KillMobj` (`p_inter.c`), `P_CheckPosition` / `P_TryMove` / teleport /
hitscan / radius attack (`p_map.c`), missiles (`p_mobj.c`),
`P_CrossSpecialLine` (`p_spec.c`) and `EV_Teleport` (`p_telept.c`).
-/

namespace Doom.Playsim.Core

open Doom.Playsim.Angle
open Doom.Playsim.CoreBase
open Doom.Playsim.Defs
open Doom.Playsim.Doors
open Doom.Playsim.Fixed
open Doom.Playsim.Flags
open Doom.Playsim.Floors
open Doom.Playsim.GameState
open Doom.Playsim.Info
open Doom.Playsim.Inter
open Doom.Playsim.Level
open Doom.Playsim.MapUtil
open Doom.Playsim.Mobj
open Doom.Playsim.MobjBase
open Doom.Playsim.Player
open Doom.Playsim.Sight
open Doom.Playsim.Sound
open Doom.Playsim.SpecDefs
open Doom.Playsim.SpecUtil
open Doom.Playsim.Switch
open Doom.Playsim.Tables
open Doom.Playsim.Thinker
open Doom.Playsim.Weapons (weaponinfo am_noammo)

def WEAPONBOTTOM : Int32 := 128 * FRACUNIT
def WEAPONTOP : Int32 := 32 * FRACUNIT
def LOWERSPEED : Int32 := 6 * FRACUNIT
def RAISESPEED : Int32 := 6 * FRACUNIT
def SKULLSPEED : Int32 := 20 * FRACUNIT
def TRACEANGLE : UInt32 := 0xc000000
def FATSPREAD : UInt32 := ANG90 / 8
def CF_GODMODE : Int32 := 2

@[inline] def psp (p : Player) (pos : Nat) : Psprite := p.psprites[pos]!
@[inline] def setPsp (pi pos : Nat) (f : Psprite → Psprite) : M Unit :=
  modifyPl pi fun p => { p with psprites := p.psprites.modify pos f }

/-- `DecreaseAmmo` (with the out-of-range-into-maxammo emulation). -/
def decreaseAmmo (pi : Nat) (ammonum amount : Int32) : M Unit := modifyPl pi fun p =>
  if ammonum < 4 then { p with ammo := p.ammo.modify ammonum.toNatClampNeg (· - amount) }
  else { p with maxammo := p.maxammo.modify (ammonum - 4).toNatClampNeg (· - amount) }

/-- Bounds of a blockmap box `(xl, xh, yl, yh)` around a point. -/
@[inline] def blockBox (gs : GameState) (left right bottom top pad : Int32) :
    Int32 × Int32 × Int32 × Int32 :=
  ((left - gs.bmaporgx - pad) >>> 23, (right - gs.bmaporgx + pad) >>> 23,
   (bottom - gs.bmaporgy - pad) >>> 23, (top - gs.bmaporgy + pad) >>> 23)

/-- Iterate things over a block box; stops at the first `false`. -/
@[specialize] def iterThings (xl xh yl yh : Int32) (f : Nat → M Bool) : M Bool := do
  let mut bx := xl
  while bx <= xh do
    let mut by_ := yl
    while by_ <= yh do
      if !(← blockThingsIterator bx by_ f) then return false
      by_ := by_ + 1
    bx := bx + 1
  return true

mutual

-- ### Mobj states

/-- `P_SetMobjState`; returns false if the mobj removed itself. -/
partial def setMobjState (mi : Nat) (state0 : UInt32) : M Bool := do
  let mut state := state0
  let mut cycles : Nat := 0
  repeat
    if state == S_NULL then
      modifyMo mi fun m => { m with state := 0 }
      removeMobj mi
      return false
    let st ← getState state
    modifyMo mi fun m => { m with state, tics := st.tics, sprite := st.sprite, frame := st.frame }
    if st.action != actionNull then runAction mi st.action
    state := st.nextstate
    cycles := cycles + 1
    if cycles > 1000000 then
      iError "P_SetMobjState: Infinite state cycle detected!"
      return true
    if (← getMo mi).tics != 0 then break
  return true

/-- Mobj action dispatch (`st->action.acp1(mobj)`). -/
partial def runAction (mi : Nat) (action : UInt32) : M Unit := do
  match action.toNat with
  | 23 => aBFGSpray mi
  | 24 => aExplode mi
  | 25 => aPain mi
  | 26 => aPlayerScream mi
  | 27 => aFall mi
  | 28 => aXScream mi
  | 29 => aLook mi
  | 30 => aChase mi
  | 31 => aFaceTarget mi
  | 32 => aPosAttack mi
  | 33 => aScream mi
  | 34 => aSPosAttack mi
  | 35 => aVileChase mi
  | 36 => aVileStart mi
  | 37 => aVileTarget mi
  | 38 => aVileAttack mi
  | 39 => aStartFire mi
  | 40 => aFire mi
  | 41 => aFireCrackle mi
  | 42 => aTracer mi
  | 43 => aSkelWhoosh mi
  | 44 => aSkelFist mi
  | 45 => aSkelMissile mi
  | 46 => aFatRaise mi
  | 47 => aFatAttack1 mi
  | 48 => aFatAttack2 mi
  | 49 => aFatAttack3 mi
  | 50 => aBossDeath mi
  | 51 => aCPosAttack mi
  | 52 => aCPosRefire mi
  | 53 => aTroopAttack mi
  | 54 => aSargAttack mi
  | 55 => aHeadAttack mi
  | 56 => aBruisAttack mi
  | 57 => aSkullAttack mi
  | 58 => do startSoundMo mi sfx_metal; aChase mi
  | 59 => aSpidRefire mi
  | 60 => do startSoundMo mi sfx_bspwlk; aChase mi
  | 61 => aBspiAttack mi
  | 62 => do startSoundMo mi sfx_hoof; aChase mi
  | 63 => aCyberAttack mi
  | 64 => aPainAttack mi
  | 65 => aPainDie mi
  | 66 => aKeenDie mi
  | 67 => aBrainPain
  | 68 => aBrainScream mi
  | 69 => gExitLevel
  | 70 => aBrainAwake
  | 71 => aBrainSpit mi
  | 72 => do startSoundMo mi sfx_boscub; aSpawnFly mi
  | 73 => aSpawnFly mi
  | 74 => aBrainExplode mi
  | _ => pure ()

-- ### Damage

/-- `P_KillMobj(source, target)`. -/
partial def killMobj (source : Int32) (ti : Nat) : M Unit := do
  modifyMo ti fun t =>
    let f := t.flags &&& ~~~(MF_SHOOTABLE ||| MF_FLOAT ||| MF_SKULLFLY)
    let f := if t.typeId != MT_SKULL then f &&& ~~~MF_NOGRAVITY else f
    { t with flags := f ||| MF_CORPSE ||| MF_DROPOFF, height := t.height >>> 2 }
  let gs ← get
  let target := gs.mobjs[ti]!
  let srcPlayer : Int32 := if source >= 0 then gs.mobjs[source.toNatClampNeg]!.player else -1
  if source >= 0 && srcPlayer >= 0 then
    let sp := srcPlayer.toNatClampNeg
    if (target.flags &&& MF_COUNTKILL) != 0 then
      modifyPl sp fun p => { p with killcount := p.killcount + 1 }
    if target.player >= 0 then
      modifyPl sp fun p => { p with frags := p.frags.modify target.player.toNatClampNeg (· + 1) }
  else if !gs.netgame && (target.flags &&& MF_COUNTKILL) != 0 then
    modifyPl 0 fun p => { p with killcount := p.killcount + 1 }
  if target.player >= 0 then
    let tp := target.player.toNatClampNeg
    if source < 0 then
      modifyPl tp fun p => { p with frags := p.frags.modify tp (· + 1) }
    modifyMo ti fun t => { t with flags := t.flags &&& ~~~MF_SOLID }
    modifyPl tp fun p => { p with playerstate := PST_DEAD }
    dropWeapon tp
  let target ← getMo ti
  let info ← getInfo target.typeId
  let _ ← if target.health < -info.spawnhealth && info.xdeathstate != 0 then
      setMobjState ti info.xdeathstate
    else setMobjState ti info.deathstate
  let r ← pRandomM
  modifyMo ti fun t =>
    let tics := t.tics - (r &&& 3)
    { t with tics := if tics < 1 then 1 else tics }
  let t ← getMo ti
  let item :=
    if t.typeId == MT_WOLFSS || t.typeId == MT_POSSESSED then MT_CLIP
    else if t.typeId == MT_SHOTGUY then MT_SHOTGUN
    else if t.typeId == MT_CHAINGUY then MT_CHAINGUN
    else -1
  if item >= 0 then
    let mo ← spawnMobj t.x t.y ONFLOORZ item
    modifyMo mo fun m => { m with flags := m.flags ||| MF_DROPPED }

/-- `P_DamageMobj(target, inflictor, source, damage)`. -/
partial def damageMobj (ti : Nat) (inflictor source : Int32) (damage0 : Int32) : M Unit := do
  let gs ← get
  let target := gs.mobjs[ti]!
  if (target.flags &&& MF_SHOOTABLE) == 0 then return
  if target.health <= 0 then return
  let mut damage := damage0
  if target.player >= 0 && gs.gameskill == sk_baby then damage := damage >>> 1
  let srcChainsaw :=
    source >= 0 && (let s := gs.mobjs[source.toNatClampNeg]!
                    s.player >= 0 && gs.players[s.player.toNatClampNeg]!.readyweapon == wp_chainsaw)
  let inf := gs.mobjs[inflictor.toNatClampNeg]!
  let mass := gs.mobjinfo[target.typeId.toNatClampNeg]!.mass
  if (target.flags &&& MF_SKULLFLY) != 0 then
    modifyMo ti fun t => { t with momx := 0, momy := 0, momz := 0 }
  if inflictor >= 0 && (target.flags &&& MF_NOCLIP) == 0 && !srcChainsaw then
    let mut ang := pointToAngle2 inf.x inf.y target.x target.y
    let mut thrust := damage * (FRACUNIT >>> 3) * 100 / mass
    if damage < 40 && damage > target.health && target.z - inf.z > 64 * FRACUNIT then
      if ((← pRandomM) &&& 1) != 0 then
        ang := ang + ANG180
        thrust := thrust * 4
    modifyMo ti fun t => { t with momx := t.momx + fixedMul thrust (fineCos ang)
                                  momy := t.momy + fixedMul thrust (fineSin ang) }
  let gs ← get
  let target := gs.mobjs[ti]!
  if target.player >= 0 then
    let pi := target.player.toNatClampNeg
    if gs.sectors[moSector gs target]!.special == 11 && damage >= target.health then
      damage := target.health - 1
    let p := gs.players[pi]!
    if damage < 1000 && ((p.cheats &&& CF_GODMODE) != 0 || p.powers[pw_invulnerability]! != 0) then
      return
    let mut p := p
    if p.armortype != 0 then
      let mut saved := if p.armortype == 1 then damage / 3 else damage / 2
      if p.armorpoints <= saved then
        saved := p.armorpoints
        p := { p with armortype := 0 }
      p := { p with armorpoints := p.armorpoints - saved }
      damage := damage - saved
    let h := p.health - damage
    let dc := p.damagecount + damage
    p := { p with health := if h < 0 then 0 else h, attacker := source
                  damagecount := if dc > 100 then 100 else dc }
    modifyPl pi fun _ => p
  modifyMo ti fun t => { t with health := t.health - damage }
  let target ← getMo ti
  if target.health <= 0 then
    killMobj source ti
    return
  let info ← getInfo target.typeId
  if (← pRandomM) < info.painchance && (target.flags &&& MF_SKULLFLY) == 0 then
    modifyMo ti fun t => { t with flags := t.flags ||| MF_JUSTHIT }
    let _ ← setMobjState ti info.painstate
  modifyMo ti fun t => { t with reactiontime := 0 }
  let gs ← get
  let target := gs.mobjs[ti]!
  if (target.threshold == 0 || target.typeId == MT_VILE) && source >= 0 &&
      (source != ti.toInt32 || gs.res.gameversion < exe_doom_1_5) &&
      gs.mobjs[source.toNatClampNeg]!.typeId != MT_VILE then
    modifyMo ti fun t => { t with target := source, threshold := BASETHRESHOLD }
    if target.state == info.spawnstate && info.seestate != S_NULL then
      let _ ← setMobjState ti info.seestate

-- ### Movement clipping

/-- `PIT_CheckThing`. -/
partial def pitCheckThing (thi : Nat) : M Bool := do
  let gs ← get
  let thing := gs.mobjs[thi]!
  if (thing.flags &&& (MF_SOLID ||| MF_SPECIAL ||| MF_SHOOTABLE)) == 0 then return true
  let tmi := gs.tmthing.toNatClampNeg
  let tm := gs.mobjs[tmi]!
  let blockdist := thing.radius + tm.radius
  if wabs (thing.x - gs.tmx) >= blockdist || wabs (thing.y - gs.tmy) >= blockdist then return true
  if thi == tmi then return true
  let tmdamage := gs.mobjinfo[tm.typeId.toNatClampNeg]!.damage
  let tmTargetType := if tm.target >= 0 then gs.mobjs[tm.target.toNatClampNeg]!.typeId else -1
  let pickup := (gs.tmflags &&& MF_PICKUP) != 0
  if (tm.flags &&& MF_SKULLFLY) != 0 then
    let r ← pRandomM
    let damage := ((r % 8) + 1) * tmdamage
    damageMobj thi tmi.toInt32 tmi.toInt32 damage
    let tmi := (← get).tmthing.toNatClampNeg
    modifyMo tmi fun t => { t with flags := t.flags &&& ~~~MF_SKULLFLY, momx := 0, momy := 0, momz := 0 }
    let info ← getInfo (← getMo tmi).typeId
    let _ ← setMobjState tmi info.spawnstate
    return false
  if (tm.flags &&& MF_MISSILE) != 0 then
    if tm.z > thing.z + thing.height then return true
    if tm.z + tm.height < thing.z then return true
    if tm.target >= 0 then
      let tt := tmTargetType
      if tt == thing.typeId || (tt == MT_KNIGHT && thing.typeId == MT_BRUISER) ||
          (tt == MT_BRUISER && thing.typeId == MT_KNIGHT) then
        if thi.toInt32 == tm.target then return true
        if thing.typeId != MT_PLAYER then return false
    if (thing.flags &&& MF_SHOOTABLE) == 0 then return (thing.flags &&& MF_SOLID) == 0
    let r ← pRandomM
    let damage := ((r % 8) + 1) * tmdamage
    damageMobj thi tmi.toInt32 tm.target damage
    return false
  if (thing.flags &&& MF_SPECIAL) != 0 then
    let solid := (thing.flags &&& MF_SOLID) != 0
    if pickup then touchSpecialThing thi tmi
    return !solid
  return (thing.flags &&& MF_SOLID) == 0

/-- `P_CheckPosition(thing, x, y)`. -/
partial def checkPosition (mi : Nat) (x y : Int32) : M Bool := do
  let gs ← get
  let th := gs.mobjs[mi]!
  let ss := pointInSubsector gs.level x y
  let sec := gs.sectors[gs.level.subsectors[ss]!.sector.toNat]!
  set { gs with tmthing := mi.toInt32, tmflags := th.flags, tmx := x, tmy := y
                tmbboxTop := y + th.radius, tmbboxBottom := y - th.radius
                tmbboxRight := x + th.radius, tmbboxLeft := x - th.radius
                ceilingline := -1, tmfloorz := sec.floorheight, tmdropoffz := sec.floorheight
                tmceilingz := sec.ceilingheight }
  bumpValidcount
  modify fun gs => { gs with numspechit := 0 }
  if (th.flags &&& MF_NOCLIP) != 0 then return true
  let gs ← get
  let (xl, xh, yl, yh) := blockBox gs gs.tmbboxLeft gs.tmbboxRight gs.tmbboxBottom gs.tmbboxTop MAXRADIUS
  if !(← iterThings xl xh yl yh pitCheckThing) then return false
  let gs ← get
  let (xl, xh, yl, yh) := blockBox gs gs.tmbboxLeft gs.tmbboxRight gs.tmbboxBottom gs.tmbboxTop 0
  let mut bx := xl
  while bx <= xh do
    let mut by_ := yl
    while by_ <= yh do
      if !(← blockLinesIterator bx by_ pitCheckLine) then return false
      by_ := by_ + 1
    bx := bx + 1
  return true

/-- `P_TryMove(thing, x, y)`. -/
partial def tryMove (mi : Nat) (x y : Int32) : M Bool := do
  modify fun gs => { gs with floatok := false }
  if !(← checkPosition mi x y) then return false
  let gs ← get
  let th := gs.mobjs[mi]!
  if (th.flags &&& MF_NOCLIP) == 0 then
    if gs.tmceilingz - gs.tmfloorz < th.height then return false
    set { gs with floatok := true }
    if (th.flags &&& MF_TELEPORT) == 0 && gs.tmceilingz - th.z < th.height then return false
    if (th.flags &&& MF_TELEPORT) == 0 && gs.tmfloorz - th.z > 24 * FRACUNIT then return false
    if (th.flags &&& (MF_DROPOFF ||| MF_FLOAT)) == 0 && gs.tmfloorz - gs.tmdropoffz > 24 * FRACUNIT then
      return false
  unsetThingPosition mi
  let gs ← get
  let oldx := gs.mobjs[mi]!.x
  let oldy := gs.mobjs[mi]!.y
  set {
    gs with
    mobjs := gs.mobjs.modify mi fun m => { m with floorz := gs.tmfloorz, ceilingz := gs.tmceilingz, x, y } }
  setThingPosition mi
  let th ← getMo mi
  if (th.flags &&& (MF_TELEPORT ||| MF_NOCLIP)) == 0 then
    repeat
      let gs ← get
      let n := gs.numspechit
      set { gs with numspechit := n - 1 }
      if n == 0 then break
      let l := gs.spechit[(n - 1).toNatClampNeg]!
      let ld := gs.level.lines[l]!
      let th := gs.mobjs[mi]!
      let side := pointOnLineSide gs th.x th.y ld
      let oldside := pointOnLineSide gs oldx oldy ld
      if side != oldside then
        if ld.special != 0 then crossSpecialLine l oldside mi
  return true

-- ### Teleport

/-- `PIT_StompThing`. -/
partial def pitStompThing (thi : Nat) : M Bool := do
  let gs ← get
  let thing := gs.mobjs[thi]!
  if (thing.flags &&& MF_SHOOTABLE) == 0 then return true
  let tmi := gs.tmthing.toNatClampNeg
  let tm := gs.mobjs[tmi]!
  let blockdist := thing.radius + tm.radius
  if wabs (thing.x - gs.tmx) >= blockdist || wabs (thing.y - gs.tmy) >= blockdist then return true
  if thi == tmi then return true
  if tm.player < 0 && gs.gamemap != 30 then return false
  damageMobj thi tmi.toInt32 tmi.toInt32 10000
  return true

/-- `P_TeleportMove(thing, x, y)`. -/
partial def teleportMove (mi : Nat) (x y : Int32) : M Bool := do
  let gs ← get
  let th := gs.mobjs[mi]!
  let ss := pointInSubsector gs.level x y
  let sec := gs.sectors[gs.level.subsectors[ss]!.sector.toNat]!
  set { gs with tmthing := mi.toInt32, tmflags := th.flags, tmx := x, tmy := y
                tmbboxTop := y + th.radius, tmbboxBottom := y - th.radius
                tmbboxRight := x + th.radius, tmbboxLeft := x - th.radius
                ceilingline := -1, tmfloorz := sec.floorheight, tmdropoffz := sec.floorheight
                tmceilingz := sec.ceilingheight }
  bumpValidcount
  modify fun gs => { gs with numspechit := 0 }
  let gs ← get
  let (xl, xh, yl, yh) := blockBox gs gs.tmbboxLeft gs.tmbboxRight gs.tmbboxBottom gs.tmbboxTop MAXRADIUS
  if !(← iterThings xl xh yl yh pitStompThing) then return false
  unsetThingPosition mi
  let gs ← get
  set {
    gs with
    mobjs := gs.mobjs.modify mi fun m => { m with floorz := gs.tmfloorz, ceilingz := gs.tmceilingz, x, y } }
  setThingPosition mi
  return true

/-- `EV_Teleport(line, side, thing)`. -/
partial def evTeleport (lineIdx : Nat) (side : Nat) (mi : Nat) : M Bool := do
  let th ← getMo mi
  if (th.flags &&& MF_MISSILE) != 0 then return false
  if side == 1 then return false
  let tag := (← getLine lineIdx).tag
  for i in [0:(← get).sectors.size] do
    if (← get).sectors[i]!.tag != tag then continue
    for r in (← get).thinkers do
      if thKind r != TK_MOBJ then continue
      let gs ← get
      let m := gs.mobjs[thIdx r]!
      if m.func != THF_MOBJ then continue
      if m.typeId != MT_TELEPORTMAN then continue
      if moSector gs m != i then continue
      let th := gs.mobjs[mi]!
      let (oldx, oldy, oldz) := (th.x, th.y, th.z)
      if !(← teleportMove mi m.x m.y) then return false
      if gs.res.gameversion != exe_final then
        modifyMo mi fun t => { t with z := t.floorz }
      let th ← getMo mi
      if th.player >= 0 then
        modifyPl th.player.toNatClampNeg fun p => { p with viewz := th.z + p.viewheight }
      let fog ← spawnMobj oldx oldy oldz MT_TFOG
      startSoundMo fog sfx_telept
      let fog ← spawnMobj (m.x + 20 * fineCos m.angle) (m.y + 20 * fineSin m.angle) th.z MT_TFOG
      startSoundMo fog sfx_telept
      modifyMo mi fun t => {
        t with
        reactiontime := if t.player >= 0 then 18 else t.reactiontime
        angle := m.angle, momx := 0, momy := 0, momz := 0 }
      return true
  return false

/-- `P_CrossSpecialLine(linenum, side, thing)`. -/
partial def crossSpecialLine (lineIdx : Nat) (side : Nat) (mi : Nat) : M Unit := do
  let gs ← get
  let ld := gs.level.lines[lineIdx]!
  let thing := gs.mobjs[mi]!
  if gs.res.gameversion <= exe_doom_1_2 then
    if ld.special > 98 && ld.special != 104 then return
  else if thing.player < 0 then
    let t := thing.typeId
    if t == MT_ROCKET || t == MT_PLASMA || t == MT_BFG || t == MT_TROOPSHOT ||
        t == MT_HEADSHOT || t == MT_BRUISERSHOT then return
  let sp := ld.special
  if thing.player < 0 then
    if !(sp == 39 || sp == 97 || sp == 125 || sp == 126 || sp == 4 || sp == 10 || sp == 88) then
      return
  let tag := ld.tag
  let front := ld.frontsector
  let clr : M Unit := setLineSpecial lineIdx 0
  match sp.toNatClampNeg with
  | 2 => do let _ ← evDoDoor tag vld_open; clr
  | 3 => do let _ ← evDoDoor tag vld_close; clr
  | 4 => do let _ ← evDoDoor tag vld_normal; clr
  | 5 => do let _ ← evDoFloor tag front raiseFloor; clr
  | 6 => do let _ ← evDoCeiling tag fastCrushAndRaise; clr
  | 8 => do let _ ← evBuildStairs tag build8; clr
  | 10 => do let _ ← evDoPlat tag ld.sidenum0 downWaitUpStay 0; clr
  | 12 => do evLightTurnOn tag 0; clr
  | 13 => do evLightTurnOn tag 255; clr
  | 16 => do let _ ← evDoDoor tag vld_close30ThenOpen; clr
  | 17 => do evStartLightStrobing tag; clr
  | 19 => do let _ ← evDoFloor tag front lowerFloor; clr
  | 22 => do let _ ← evDoPlat tag ld.sidenum0 raiseToNearestAndChange 0; clr
  | 25 => do let _ ← evDoCeiling tag crushAndRaise; clr
  | 30 => do let _ ← evDoFloor tag front raiseToTexture; clr
  | 35 => do evLightTurnOn tag 35; clr
  | 36 => do let _ ← evDoFloor tag front turboLower; clr
  | 37 => do let _ ← evDoFloor tag front lowerAndChange; clr
  | 38 => do let _ ← evDoFloor tag front lowerFloorToLowest; clr
  | 39 => do let _ ← evTeleport lineIdx side mi; clr
  | 40 => do
    let _ ← evDoCeiling tag raiseToHighest
    let _ ← evDoFloor tag front lowerFloorToLowest
    clr
  | 44 => do let _ ← evDoCeiling tag lowerAndCrush; clr
  | 52 => gExitLevel
  | 53 => do let _ ← evDoPlat tag ld.sidenum0 perpetualRaise 0; clr
  | 54 => do evStopPlat tag; clr
  | 56 => do let _ ← evDoFloor tag front raiseFloorCrush; clr
  | 57 => do let _ ← evCeilingCrushStop tag; clr
  | 58 => do let _ ← evDoFloor tag front raiseFloor24; clr
  | 59 => do let _ ← evDoFloor tag front raiseFloor24AndChange; clr
  | 104 => do evTurnTagLightsOff tag; clr
  | 108 => do let _ ← evDoDoor tag vld_blazeRaise; clr
  | 109 => do let _ ← evDoDoor tag vld_blazeOpen; clr
  | 100 => do let _ ← evBuildStairs tag turbo16; clr
  | 110 => do let _ ← evDoDoor tag vld_blazeClose; clr
  | 119 => do let _ ← evDoFloor tag front raiseFloorToNearest; clr
  | 121 => do let _ ← evDoPlat tag ld.sidenum0 blazeDWUS 0; clr
  | 124 => gSecretExitLevel
  | 125 => if thing.player < 0 then do let _ ← evTeleport lineIdx side mi; clr
  | 130 => do let _ ← evDoFloor tag front raiseFloorTurbo; clr
  | 141 => do let _ ← evDoCeiling tag silentCrushAndRaise; clr
  -- retriggers
  | 72 => do let _ ← evDoCeiling tag lowerAndCrush
  | 73 => do let _ ← evDoCeiling tag crushAndRaise
  | 74 => do let _ ← evCeilingCrushStop tag
  | 75 => do let _ ← evDoDoor tag vld_close
  | 76 => do let _ ← evDoDoor tag vld_close30ThenOpen
  | 77 => do let _ ← evDoCeiling tag fastCrushAndRaise
  | 79 => evLightTurnOn tag 35
  | 80 => evLightTurnOn tag 0
  | 81 => evLightTurnOn tag 255
  | 82 => do let _ ← evDoFloor tag front lowerFloorToLowest
  | 83 => do let _ ← evDoFloor tag front lowerFloor
  | 84 => do let _ ← evDoFloor tag front lowerAndChange
  | 86 => do let _ ← evDoDoor tag vld_open
  | 87 => do let _ ← evDoPlat tag ld.sidenum0 perpetualRaise 0
  | 88 => do let _ ← evDoPlat tag ld.sidenum0 downWaitUpStay 0
  | 89 => evStopPlat tag
  | 90 => do let _ ← evDoDoor tag vld_normal
  | 91 => do let _ ← evDoFloor tag front raiseFloor
  | 92 => do let _ ← evDoFloor tag front raiseFloor24
  | 93 => do let _ ← evDoFloor tag front raiseFloor24AndChange
  | 94 => do let _ ← evDoFloor tag front raiseFloorCrush
  | 95 => do let _ ← evDoPlat tag ld.sidenum0 raiseToNearestAndChange 0
  | 96 => do let _ ← evDoFloor tag front raiseToTexture
  | 97 => do let _ ← evTeleport lineIdx side mi
  | 98 => do let _ ← evDoFloor tag front turboLower
  | 105 => do let _ ← evDoDoor tag vld_blazeRaise
  | 106 => do let _ ← evDoDoor tag vld_blazeOpen
  | 107 => do let _ ← evDoDoor tag vld_blazeClose
  | 120 => do let _ ← evDoPlat tag ld.sidenum0 blazeDWUS 0
  | 126 => if thing.player < 0 then do let _ ← evTeleport lineIdx side mi
  | 128 => do let _ ← evDoFloor tag front raiseFloorToNearest
  | 129 => do let _ ← evDoFloor tag front raiseFloorTurbo
  | _ => pure ()

-- ### Hitscan and explosions

/-- `P_SpawnPuff`. -/
partial def spawnPuff (x y z0 : Int32) : M Unit := do
  let z := z0 + ((← pSubRandomM) <<< 10)
  let th ← spawnMobj x y z MT_PUFF
  let r ← pRandomM
  modifyMo th fun m =>
    let tics := m.tics - (r &&& 3)
    { m with momz := FRACUNIT, tics := if tics < 1 then 1 else tics }
  if (← get).attackrange == MELEERANGE then
    let _ ← setMobjState th S_PUFF3

/-- `P_SpawnBlood`. -/
partial def spawnBlood (x y z0 : Int32) (damage : Int32) : M Unit := do
  let z := z0 + ((← pSubRandomM) <<< 10)
  let th ← spawnMobj x y z MT_BLOOD
  let r ← pRandomM
  modifyMo th fun m =>
    let tics := m.tics - (r &&& 3)
    { m with momz := FRACUNIT * 2, tics := if tics < 1 then 1 else tics }
  if damage <= 12 && damage >= 9 then let _ ← setMobjState th S_BLOOD2
  else if damage < 9 then let _ ← setMobjState th S_BLOOD3

/-- `PTR_ShootTraverse`. -/
partial def ptrShootTraverse (frac : Int32) (isaline : Bool) (d : Nat) : M Bool := do
  if isaline then
    let li ← getLine d
    if li.special != 0 then shootSpecialLine (← get).shootthing.toNatClampNeg d
    let li ← getLine d
    let mut hit := (li.flags &&& ML_TWOSIDED) == 0
    if !hit then
      lineOpening li
      let gs ← get
      let dist := fixedMul gs.attackrange frac
      if li.backsector < 0 then
        if fixedDiv (gs.openbottom - gs.shootz) dist > gs.aimslope then hit := true
        else if fixedDiv (gs.opentop - gs.shootz) dist < gs.aimslope then hit := true
      else
        let front := gs.sectors[li.frontsector.toNatClampNeg]!
        let back := gs.sectors[li.backsector.toNatClampNeg]!
        if front.floorheight != back.floorheight &&
            fixedDiv (gs.openbottom - gs.shootz) dist > gs.aimslope then hit := true
        else if front.ceilingheight != back.ceilingheight &&
            fixedDiv (gs.opentop - gs.shootz) dist < gs.aimslope then hit := true
      if !hit then return true
    let gs ← get
    let fr := frac - fixedDiv (4 * FRACUNIT) gs.attackrange
    let x := gs.traceX + fixedMul gs.traceDx fr
    let y := gs.traceY + fixedMul gs.traceDy fr
    let z := gs.shootz + fixedMul gs.aimslope (fixedMul fr gs.attackrange)
    let front := gs.sectors[li.frontsector.toNatClampNeg]!
    if front.ceilingpicNum == gs.res.skyflatnum then
      if z > front.ceilingheight then return false
      if li.backsector >= 0 && gs.sectors[li.backsector.toNatClampNeg]!.ceilingpicNum == gs.res.skyflatnum then
        return false
    spawnPuff x y z
    return false
  let gs ← get
  if d.toInt32 == gs.shootthing then return true
  let th := gs.mobjs[d]!
  if (th.flags &&& MF_SHOOTABLE) == 0 then return true
  let dist := fixedMul gs.attackrange frac
  let thingtopslope := fixedDiv (th.z + th.height - gs.shootz) dist
  if thingtopslope < gs.aimslope then return true
  let thingbottomslope := fixedDiv (th.z - gs.shootz) dist
  if thingbottomslope > gs.aimslope then return true
  let fr := frac - fixedDiv (10 * FRACUNIT) gs.attackrange
  let x := gs.traceX + fixedMul gs.traceDx fr
  let y := gs.traceY + fixedMul gs.traceDy fr
  let z := gs.shootz + fixedMul gs.aimslope (fixedMul fr gs.attackrange)
  if (th.flags &&& MF_NOBLOOD) != 0 then spawnPuff x y z
  else spawnBlood x y z gs.laDamage
  let gs ← get
  if gs.laDamage != 0 then damageMobj d gs.shootthing gs.shootthing gs.laDamage
  return false

/-- `P_LineAttack(t1, angle, distance, slope, damage)`. -/
partial def lineAttack (t1i : Nat) (angle : UInt32) (distance slope damage : Int32) : M Unit := do
  let gs ← get
  let t1 := gs.mobjs[t1i]!
  let x2 := t1.x + (distance >>> 16) * fineCos angle
  let y2 := t1.y + (distance >>> 16) * fineSin angle
  set { gs with shootthing := t1i.toInt32, laDamage := damage
                shootz := t1.z + (t1.height >>> 1) + 8 * FRACUNIT
                attackrange := distance, aimslope := slope }
  let _ ← pathTraverse t1.x t1.y x2 y2 (PT_ADDLINES ||| PT_ADDTHINGS) ptrShootTraverse

/-- `PIT_RadiusAttack`. -/
partial def pitRadiusAttack (thi : Nat) : M Bool := do
  let gs ← get
  let thing := gs.mobjs[thi]!
  if (thing.flags &&& MF_SHOOTABLE) == 0 then return true
  if thing.typeId == MT_CYBORG || thing.typeId == MT_SPIDER then return true
  let spot := gs.mobjs[gs.bombspot.toNatClampNeg]!
  let dx := wabs (thing.x - spot.x)
  let dy := wabs (thing.y - spot.y)
  let mut dist := if dx > dy then dx else dy
  dist := (dist - thing.radius) >>> 16
  if dist < 0 then dist := 0
  if dist >= gs.bombdamage then return true
  if ← checkSight thi gs.bombspot.toNatClampNeg then
    let gs ← get
    damageMobj thi gs.bombspot gs.bombsource (gs.bombdamage - dist)
  return true

/-- `P_RadiusAttack(spot, source, damage)`. -/
partial def radiusAttack (spot : Nat) (source : Int32) (damage : Int32) : M Unit := do
  let gs ← get
  let s := gs.mobjs[spot]!
  let dist := (damage + MAXRADIUS) <<< 16
  let yh := (s.y + dist - gs.bmaporgy) >>> 23
  let yl := (s.y - dist - gs.bmaporgy) >>> 23
  let xh := (s.x + dist - gs.bmaporgx) >>> 23
  let xl := (s.x - dist - gs.bmaporgx) >>> 23
  set { gs with bombspot := spot.toInt32, bombsource := source, bombdamage := damage }
  let mut y := yl
  while y <= yh do
    let mut x := xl
    while x <= xh do
      let _ ← blockThingsIterator x y pitRadiusAttack
      x := x + 1
    y := y + 1

-- ### Missiles

/-- `P_ExplodeMissile`. -/
partial def explodeMissile (mi : Nat) : M Unit := do
  modifyMo mi fun m => { m with momx := 0, momy := 0, momz := 0 }
  let info ← getInfo (← getMo mi).typeId
  let _ ← setMobjState mi info.deathstate
  let r ← pRandomM
  modifyMo mi fun m =>
    let tics := m.tics - (r &&& 3)
    { m with tics := if tics < 1 then 1 else tics, flags := m.flags &&& ~~~MF_MISSILE }
  if info.deathsound != 0 then startSoundMo mi info.deathsound.toNatClampNeg

/-- `P_CheckMissileSpawn`. -/
partial def checkMissileSpawn (mi : Nat) : M Unit := do
  let r ← pRandomM
  modifyMo mi fun m =>
    let tics := m.tics - (r &&& 3)
    { m with tics := if tics < 1 then 1 else tics
             x := m.x + (m.momx >>> 1), y := m.y + (m.momy >>> 1), z := m.z + (m.momz >>> 1) }
  let th ← getMo mi
  if !(← tryMove mi th.x th.y) then explodeMissile mi

/-- `P_SpawnMissile(source, dest, type)`; `dest = -1` is the dummy mobj. -/
partial def spawnMissile (source : Nat) (dest : Int32) (type_ : Int32) : M Nat := do
  let src ← getMo source
  let th ← spawnMobj src.x src.y (src.z + 4 * 8 * FRACUNIT) type_
  let info ← getInfo type_
  if info.seesound != 0 then startSoundMo th info.seesound.toNatClampNeg
  let gs ← get
  let d : Mobj := if dest >= 0 then gs.mobjs[dest.toNatClampNeg]!
    else { Mobj.empty with x := 0, y := 0, z := 0, flags := 0 }
  let src := gs.mobjs[source]!
  let mut an := pointToAngle2 src.x src.y d.x d.y
  if (d.flags &&& MF_SHADOW) != 0 then an := an + ((← pSubRandomM).toUInt32 <<< 20)
  let mut dist := aproxDistance (d.x - src.x) (d.y - src.y)
  dist := dist / info.speed
  if dist < 1 then dist := 1
  modifyMo th fun m => {
    m with
    target := source.toInt32, angle := an
    momx := fixedMul info.speed (fineCos an), momy := fixedMul info.speed (fineSin an)
    momz := (d.z - src.z) / dist }
  checkMissileSpawn th
  return th

/-- `P_SpawnPlayerMissile(source, type)`. -/
partial def spawnPlayerMissile (source : Nat) (type_ : Int32) : M Unit := do
  let src ← getMo source
  let mut an := src.angle
  let mut slope ← aimLineAttack source.toInt32 an (16 * 64 * FRACUNIT)
  if (← get).linetarget < 0 then
    an := an + ((1 : UInt32) <<< 26)
    slope ← aimLineAttack source.toInt32 an (16 * 64 * FRACUNIT)
    if (← get).linetarget < 0 then
      an := an - ((2 : UInt32) <<< 26)
      slope ← aimLineAttack source.toInt32 an (16 * 64 * FRACUNIT)
    if (← get).linetarget < 0 then
      an := src.angle
      slope := 0
  let src ← getMo source
  let th ← spawnMobj src.x src.y (src.z + 4 * 8 * FRACUNIT) type_
  let info ← getInfo type_
  if info.seesound != 0 then startSoundMo th info.seesound.toNatClampNeg
  modifyMo th fun m => {
    m with
    target := source.toInt32, angle := an
    momx := fixedMul info.speed (fineCos an), momy := fixedMul info.speed (fineSin an)
    momz := fixedMul info.speed slope }
  checkMissileSpawn th

-- ### Monster movement

/-- `P_Move`. -/
partial def pMove (mi : Nat) : M Bool := do
  let actor ← getMo mi
  if actor.movedir == DI_NODIR then return false
  if actor.movedir < 0 || actor.movedir >= 8 then
    iError "Weird actor->movedir!"
    return false
  let speed := (← getInfo actor.typeId).speed
  let dir := actor.movedir.toNatClampNeg
  let tryx := actor.x + speed * xspeed[dir]!
  let tryy := actor.y + speed * yspeed[dir]!
  if !(← tryMove mi tryx tryy) then
    let gs ← get
    let actor := gs.mobjs[mi]!
    let tmfloorz := gs.tmfloorz
    let numspechit := gs.numspechit
    if (actor.flags &&& MF_FLOAT) != 0 && gs.floatok then
      modifyMo mi fun m => {
        m with
        z := if m.z < tmfloorz then m.z + FLOATSPEED else m.z - FLOATSPEED
        flags := m.flags ||| MF_INFLOAT }
      return true
    if numspechit == 0 then return false
    modifyMo mi fun m => { m with movedir := DI_NODIR }
    let mut good := false
    repeat
      let gs ← get
      let n := gs.numspechit
      set { gs with numspechit := n - 1 }
      if n == 0 then break
      let l := gs.spechit[(n - 1).toNatClampNeg]!
      if ← useSpecialLine mi l 0 then good := true
    return good
  modifyMo mi fun m => { m with flags := m.flags &&& ~~~MF_INFLOAT }
  modifyMo mi fun m => if (m.flags &&& MF_FLOAT) == 0 then { m with z := m.floorz } else m
  return true

/-- `P_TryWalk`. -/
partial def tryWalk (mi : Nat) : M Bool := do
  if !(← pMove mi) then return false
  let r ← pRandomM
  modifyMo mi fun m => { m with movecount := r &&& 15 }
  return true

/-- `P_NewChaseDir`. -/
partial def newChaseDir (mi : Nat) : M Unit := do
  let gs ← get
  let actor := gs.mobjs[mi]!
  if actor.target < 0 then
    iError "P_NewChaseDir: called with no target"
    return
  let targ := gs.mobjs[actor.target.toNatClampNeg]!
  let olddir := actor.movedir
  let turnaround := opposite[olddir.toNatClampNeg]!
  let deltax := targ.x - actor.x
  let deltay := targ.y - actor.y
  let mut d1 := if deltax > 10 * FRACUNIT then DI_EAST
    else if deltax < -10 * FRACUNIT then DI_WEST else DI_NODIR
  let mut d2 := if deltay < -10 * FRACUNIT then DI_SOUTH
    else if deltay > 10 * FRACUNIT then DI_NORTH else DI_NODIR
  let tryDir (dir : Int32) : M Bool := do
    modifyMo mi fun m => { m with movedir := dir }
    tryWalk mi
  if d1 != DI_NODIR && d2 != DI_NODIR then
    let md := diags[(if deltay < 0 then 2 else 0) + (if deltax > 0 then 1 else 0)]!
    modifyMo mi fun m => { m with movedir := md }
    if md != turnaround then
      if ← tryWalk mi then return
  if (← pRandomM) > 200 || wabs deltay > wabs deltax then
    let t := d1
    d1 := d2
    d2 := t
  if d1 == turnaround then d1 := DI_NODIR
  if d2 == turnaround then d2 := DI_NODIR
  if d1 != DI_NODIR then
    if ← tryDir d1 then return
  if d2 != DI_NODIR then
    if ← tryDir d2 then return
  if olddir != DI_NODIR then
    if ← tryDir olddir then return
  if ((← pRandomM) &&& 1) != 0 then
    let mut tdir := DI_EAST
    while tdir <= DI_SOUTHEAST do
      if tdir != turnaround then
        if ← tryDir tdir then return
      tdir := tdir + 1
  else
    let mut tdir := DI_SOUTHEAST
    while tdir != DI_EAST - 1 do
      if tdir != turnaround then
        if ← tryDir tdir then return
      tdir := tdir - 1
  if turnaround != DI_NODIR then
    if ← tryDir turnaround then return
  modifyMo mi fun m => { m with movedir := DI_NODIR }

-- ### Monster actions

/-- `A_Look`. -/
partial def aLook (mi : Nat) : M Unit := do
  modifyMo mi fun m => { m with threshold := 0 }
  let gs ← get
  let actor := gs.mobjs[mi]!
  let targ := gs.sectors[moSector gs actor]!.soundtarget
  let mut seeyou := false
  if targ >= 0 && (gs.mobjs[targ.toNatClampNeg]!.flags &&& MF_SHOOTABLE) != 0 then
    modifyMo mi fun m => { m with target := targ }
    if (actor.flags &&& MF_AMBUSH) != 0 then
      if ← checkSight mi targ.toNatClampNeg then seeyou := true
    else seeyou := true
  if !seeyou then
    if !(← lookForPlayers mi false) then return
  let actor ← getMo mi
  let info ← getInfo actor.typeId
  if info.seesound != 0 then
    let ss := info.seesound.toNatClampNeg
    let sound ←
      if ss == sfx_posit1 || ss == sfx_posit2 || ss == sfx_posit3 then
        pure (sfx_posit1 + ((← pRandomM) % 3).toNatClampNeg)
      else if ss == sfx_bgsit1 || ss == sfx_bgsit2 then
        pure (sfx_bgsit1 + ((← pRandomM) % 2).toNatClampNeg)
      else pure ss
    if actor.typeId == MT_SPIDER || actor.typeId == MT_CYBORG then startSoundNull sound
    else startSoundMo mi sound
  let _ ← setMobjState mi info.seestate

/-- `A_Chase`. -/
partial def aChase (mi : Nat) : M Unit := do
  let gs ← get
  let actor := gs.mobjs[mi]!
  let dead := actor.target < 0 || gs.mobjs[actor.target.toNatClampNeg]!.health <= 0
  let v12 := gs.res.gameversion > exe_doom_1_2
  if actor.reactiontime != 0 then modifyMo mi fun m => { m with reactiontime := m.reactiontime - 1 }
  if actor.threshold != 0 then
    if v12 && dead then modifyMo mi fun m => { m with threshold := 0 }
    else modifyMo mi fun m => { m with threshold := m.threshold - 1 }
  if actor.movedir < 8 then
    modifyMo mi fun m =>
      let ang := m.angle &&& ((7 : UInt32) <<< 29)
      let delta := (ang - (m.movedir.toUInt32 <<< 29)).toInt32
      { m with angle := if delta > 0 then ang - ANG90 / 2 else if delta < 0 then ang + ANG90 / 2 else ang }
  let gs ← get
  let actor := gs.mobjs[mi]!
  let info := gs.mobjinfo[actor.typeId.toNatClampNeg]!
  let slowChase := gs.gameskill != sk_nightmare && !gs.fastparm
  if actor.target < 0 || (gs.mobjs[actor.target.toNatClampNeg]!.flags &&& MF_SHOOTABLE) == 0 then
    if ← lookForPlayers mi true then return
    let _ ← setMobjState mi info.spawnstate
    return
  if (actor.flags &&& MF_JUSTATTACKED) != 0 then
    modifyMo mi fun m => { m with flags := m.flags &&& ~~~MF_JUSTATTACKED }
    if slowChase then newChaseDir mi
    return
  if info.meleestate != 0 then
    if ← checkMeleeRange mi then
      if info.attacksound != 0 then startSoundMo mi info.attacksound.toNatClampNeg
      let _ ← setMobjState mi info.meleestate
      return
  let mut nomissile := false
  if info.missilestate != 0 then
    let gs ← get
    if gs.gameskill < sk_nightmare && !gs.fastparm && gs.mobjs[mi]!.movecount != 0 then
      nomissile := true
    else if !(← checkMissileRange mi) then nomissile := true
    if !nomissile then
      let _ ← setMobjState mi info.missilestate
      modifyMo mi fun m => { m with flags := m.flags ||| MF_JUSTATTACKED }
      return
  let gs ← get
  if gs.netgame && gs.mobjs[mi]!.threshold == 0 then
    if !(← checkSight mi gs.mobjs[mi]!.target.toNatClampNeg) then
      if ← lookForPlayers mi true then return
  modifyMo mi fun m => { m with movecount := m.movecount - 1 }
  if (← getMo mi).movecount < 0 then newChaseDir mi
  else if !(← pMove mi) then newChaseDir mi
  if info.activesound != 0 then
    if (← pRandomM) < 3 then startSoundMo mi info.activesound.toNatClampNeg

partial def aPosAttack (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  aFaceTarget mi
  let angle := (← getMo mi).angle
  let slope ← aimLineAttack mi.toInt32 angle MISSILERANGE
  startSoundMo mi sfx_pistol
  let angle := angle + ((← pSubRandomM).toUInt32 <<< 20)
  let damage := (((← pRandomM) % 5) + 1) * 3
  lineAttack mi angle MISSILERANGE slope damage

partial def aSPosAttack (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  startSoundMo mi sfx_shotgn
  aFaceTarget mi
  let bangle := (← getMo mi).angle
  let slope ← aimLineAttack mi.toInt32 bangle MISSILERANGE
  for _ in [0:3] do
    let angle := bangle + ((← pSubRandomM).toUInt32 <<< 20)
    let damage := (((← pRandomM) % 5) + 1) * 3
    lineAttack mi angle MISSILERANGE slope damage

partial def aCPosAttack (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  startSoundMo mi sfx_shotgn
  aFaceTarget mi
  let bangle := (← getMo mi).angle
  let slope ← aimLineAttack mi.toInt32 bangle MISSILERANGE
  let angle := bangle + ((← pSubRandomM).toUInt32 <<< 20)
  let damage := (((← pRandomM) % 5) + 1) * 3
  lineAttack mi angle MISSILERANGE slope damage

/-- Shared body of `A_CPosRefire` / `A_SpidRefire`. -/
partial def refireCheck (mi : Nat) (chance : Int32) : M Unit := do
  aFaceTarget mi
  if (← pRandomM) < chance then return
  let gs ← get
  let actor := gs.mobjs[mi]!
  let lost ←
    if actor.target < 0 then pure true
    else if gs.mobjs[actor.target.toNatClampNeg]!.health <= 0 then pure true
    else pure !(← checkSight mi actor.target.toNatClampNeg)
  if lost then
    let _ ← setMobjState mi (← getInfo actor.typeId).seestate

partial def aCPosRefire (mi : Nat) : M Unit := refireCheck mi 40
partial def aSpidRefire (mi : Nat) : M Unit := refireCheck mi 10

partial def aBspiAttack (mi : Nat) : M Unit := do
  let a ← getMo mi
  if a.target < 0 then return
  aFaceTarget mi
  let _ ← spawnMissile mi a.target MT_ARACHPLAZ

partial def aTroopAttack (mi : Nat) : M Unit := do
  let a ← getMo mi
  if a.target < 0 then return
  aFaceTarget mi
  if ← checkMeleeRange mi then
    startSoundMo mi sfx_claw
    let damage := ((← pRandomM) % 8 + 1) * 3
    damageMobj (← getMo mi).target.toNatClampNeg mi.toInt32 mi.toInt32 damage
    return
  let _ ← spawnMissile mi (← getMo mi).target MT_TROOPSHOT

partial def aSargAttack (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  aFaceTarget mi
  let gs ← get
  if gs.res.gameversion >= exe_doom_1_5 then
    if !(← checkMeleeRange mi) then return
  let damage := (((← pRandomM) % 10) + 1) * 4
  if (← get).res.gameversion <= exe_doom_1_2 then
    lineAttack mi (← getMo mi).angle MELEERANGE 0 damage
  else
    damageMobj (← getMo mi).target.toNatClampNeg mi.toInt32 mi.toInt32 damage

partial def aHeadAttack (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  aFaceTarget mi
  if ← checkMeleeRange mi then
    let damage := ((← pRandomM) % 6 + 1) * 10
    damageMobj (← getMo mi).target.toNatClampNeg mi.toInt32 mi.toInt32 damage
    return
  let _ ← spawnMissile mi (← getMo mi).target MT_HEADSHOT

partial def aCyberAttack (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  aFaceTarget mi
  let _ ← spawnMissile mi (← getMo mi).target MT_ROCKET

partial def aBruisAttack (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  if ← checkMeleeRange mi then
    startSoundMo mi sfx_claw
    let damage := ((← pRandomM) % 8 + 1) * 10
    damageMobj (← getMo mi).target.toNatClampNeg mi.toInt32 mi.toInt32 damage
    return
  let _ ← spawnMissile mi (← getMo mi).target MT_BRUISERSHOT

partial def aSkelMissile (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  aFaceTarget mi
  modifyMo mi fun m => { m with z := m.z + 16 * FRACUNIT }
  let mo ← spawnMissile mi (← getMo mi).target MT_TRACER
  modifyMo mi fun m => { m with z := m.z - 16 * FRACUNIT }
  let t := (← getMo mi).target
  modifyMo mo fun m => { m with x := m.x + m.momx, y := m.y + m.momy, tracer := t }

partial def aTracer (mi : Nat) : M Unit := do
  if ((← get).gametic &&& 3) != 0 then return
  let actor ← getMo mi
  spawnPuff actor.x actor.y actor.z
  let actor ← getMo mi
  let th ← spawnMobj (actor.x - actor.momx) (actor.y - actor.momy) actor.z MT_SMOKE
  let r ← pRandomM
  modifyMo th fun m =>
    let tics := m.tics - (r &&& 3)
    { m with momz := FRACUNIT, tics := if tics < 1 then 1 else tics }
  let gs ← get
  let actor := gs.mobjs[mi]!
  if actor.tracer < 0 then return
  let dest := gs.mobjs[actor.tracer.toNatClampNeg]!
  if dest.health <= 0 then return
  let exact := pointToAngle2 actor.x actor.y dest.x dest.y
  let mut angle := actor.angle
  if exact != angle then
    if exact - angle > 0x80000000 then
      angle := angle - TRACEANGLE
      if exact - angle < 0x80000000 then angle := exact
    else
      angle := angle + TRACEANGLE
      if exact - angle > 0x80000000 then angle := exact
  let speed := gs.mobjinfo[actor.typeId.toNatClampNeg]!.speed
  let mut dist := aproxDistance (dest.x - actor.x) (dest.y - actor.y)
  dist := dist / speed
  if dist < 1 then dist := 1
  let slope := (dest.z + 40 * FRACUNIT - actor.z) / dist
  modifyMo mi fun m => {
    m with
    angle
    momx := fixedMul speed (fineCos angle), momy := fixedMul speed (fineSin angle)
    momz := if slope < m.momz then m.momz - FRACUNIT / 8 else m.momz + FRACUNIT / 8 }

partial def aSkelFist (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  aFaceTarget mi
  if ← checkMeleeRange mi then
    let damage := (((← pRandomM) % 10) + 1) * 6
    startSoundMo mi sfx_skepch
    damageMobj (← getMo mi).target.toNatClampNeg mi.toInt32 mi.toInt32 damage

/-- `PIT_VileCheck`. -/
partial def pitVileCheck (thi : Nat) : M Bool := do
  let gs ← get
  let thing := gs.mobjs[thi]!
  if (thing.flags &&& MF_CORPSE) == 0 then return true
  if thing.tics != -1 then return true
  let info := gs.mobjinfo[thing.typeId.toNatClampNeg]!
  if info.raisestate == S_NULL then return true
  let maxdist := info.radius + gs.mobjinfo[MT_VILE.toNatClampNeg]!.radius
  if wabs (thing.x - gs.viletryx) > maxdist || wabs (thing.y - gs.viletryy) > maxdist then return true
  set { gs with corpsehit := thi.toInt32 }
  modifyMo thi fun m => { m with momx := 0, momy := 0, height := m.height <<< 2 }
  let t ← getMo thi
  let check ← checkPosition thi t.x t.y
  modifyMo thi fun m => { m with height := m.height >>> 2 }
  return !check

partial def aVileChase (mi : Nat) : M Unit := do
  let actor ← getMo mi
  if actor.movedir != DI_NODIR then
    let gs ← get
    let speed := gs.mobjinfo[actor.typeId.toNatClampNeg]!.speed
    let dir := actor.movedir.toNatClampNeg
    let vx := actor.x + speed * xspeed[dir]!
    let vy := actor.y + speed * yspeed[dir]!
    set { gs with viletryx := vx, viletryy := vy, vileobj := mi.toInt32 }
    let xl := (vx - gs.bmaporgx - MAXRADIUS * 2) >>> 23
    let xh := (vx - gs.bmaporgx + MAXRADIUS * 2) >>> 23
    let yl := (vy - gs.bmaporgy - MAXRADIUS * 2) >>> 23
    let yh := (vy - gs.bmaporgy + MAXRADIUS * 2) >>> 23
    let mut bx := xl
    while bx <= xh do
      let mut by_ := yl
      while by_ <= yh do
        if !(← blockThingsIterator bx by_ pitVileCheck) then
          let corpse := (← get).corpsehit.toNatClampNeg
          let temp := (← getMo mi).target
          modifyMo mi fun m => { m with target := corpse.toInt32 }
          aFaceTarget mi
          modifyMo mi fun m => { m with target := temp }
          let _ ← setMobjState mi S_VILE_HEAL1
          startSoundMo corpse sfx_slop
          let info ← getInfo (← getMo corpse).typeId
          let _ ← setMobjState corpse info.raisestate
          modifyMo corpse fun m => { m with height := m.height <<< 2, flags := info.flags
                                            health := info.spawnhealth, target := -1 }
          return
        by_ := by_ + 1
      bx := bx + 1
  aChase mi

partial def aVileTarget (mi : Nat) : M Unit := do
  let actor ← getMo mi
  if actor.target < 0 then return
  aFaceTarget mi
  let t ← getMoI (← getMo mi).target
  let fog ← spawnMobj t.x t.x t.z MT_FIRE
  let tgt := (← getMo mi).target
  modifyMo mi fun m => { m with tracer := fog.toInt32 }
  modifyMo fog fun m => { m with target := mi.toInt32, tracer := tgt }
  aFire fog

partial def aVileAttack (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  aFaceTarget mi
  let actor ← getMo mi
  if !(← checkSight mi actor.target.toNatClampNeg) then return
  startSoundMo mi sfx_barexp
  let ti := actor.target.toNatClampNeg
  damageMobj ti mi.toInt32 mi.toInt32 20
  let mass := (← getInfo (← getMo ti).typeId).mass
  modifyMo ti fun m => { m with momz := 1000 * FRACUNIT / mass }
  let actor ← getMo mi
  if actor.tracer < 0 then return
  let t ← getMo ti
  modifyMo actor.tracer.toNatClampNeg fun f => {
    f with
    x := t.x - fixedMul (24 * FRACUNIT) (fineCos actor.angle)
    y := t.y - fixedMul (24 * FRACUNIT) (fineSin actor.angle) }
  radiusAttack actor.tracer.toNatClampNeg mi.toInt32 70

/-- Re-aim a missile after an angle change (`FatAttack` tail). -/
partial def reaimMissile (mo : Nat) (delta : UInt32) (add : Bool) : M Unit := do
  let speed := (← getInfo (← getMo mo).typeId).speed
  modifyMo mo fun m =>
    let a := if add then m.angle + delta else m.angle - delta
    { m with angle := a, momx := fixedMul speed (fineCos a), momy := fixedMul speed (fineSin a) }

partial def aFatAttack1 (mi : Nat) : M Unit := do
  aFaceTarget mi
  modifyMo mi fun m => { m with angle := m.angle + FATSPREAD }
  let target := (← getMo mi).target
  let _ ← spawnMissile mi target MT_FATSHOT
  let mo ← spawnMissile mi target MT_FATSHOT
  reaimMissile mo FATSPREAD true

partial def aFatAttack2 (mi : Nat) : M Unit := do
  aFaceTarget mi
  modifyMo mi fun m => { m with angle := m.angle - FATSPREAD }
  let target := (← getMo mi).target
  let _ ← spawnMissile mi target MT_FATSHOT
  let mo ← spawnMissile mi target MT_FATSHOT
  reaimMissile mo (FATSPREAD * 2) false

partial def aFatAttack3 (mi : Nat) : M Unit := do
  aFaceTarget mi
  let target := (← getMo mi).target
  let mo ← spawnMissile mi target MT_FATSHOT
  reaimMissile mo (FATSPREAD / 2) false
  let mo ← spawnMissile mi target MT_FATSHOT
  reaimMissile mo (FATSPREAD / 2) true

partial def aSkullAttack (mi : Nat) : M Unit := do
  let actor ← getMo mi
  if actor.target < 0 then return
  let dest := actor.target.toNatClampNeg
  modifyMo mi fun m => { m with flags := m.flags ||| MF_SKULLFLY }
  startSoundMo mi (← getInfo actor.typeId).attacksound.toNatClampNeg
  aFaceTarget mi
  let actor ← getMo mi
  let d ← getMo dest
  let mut dist := aproxDistance (d.x - actor.x) (d.y - actor.y)
  dist := dist / SKULLSPEED
  if dist < 1 then dist := 1
  modifyMo mi fun m => {
    m with
    momx := fixedMul SKULLSPEED (fineCos m.angle), momy := fixedMul SKULLSPEED (fineSin m.angle)
    momz := (d.z + (d.height >>> 1) - m.z) / dist }

/-- `A_PainShootSkull(actor, angle)`. -/
partial def aPainShootSkull (mi : Nat) (angle : UInt32) : M Unit := do
  let gs ← get
  let mut count := 0
  for r in gs.thinkers do
    if thKind r == TK_MOBJ then
      let m := gs.mobjs[thIdx r]!
      if m.func == THF_MOBJ && m.typeId == MT_SKULL then count := count + 1
  if count > 20 then return
  let actor := gs.mobjs[mi]!
  let prestep := 4 * FRACUNIT +
    3 * (gs.mobjinfo[actor.typeId.toNatClampNeg]!.radius + gs.mobjinfo[MT_SKULL.toNatClampNeg]!.radius) / 2
  let x := actor.x + fixedMul prestep (fineCos angle)
  let y := actor.y + fixedMul prestep (fineSin angle)
  let z := actor.z + 8 * FRACUNIT
  let nm ← spawnMobj x y z MT_SKULL
  let n ← getMo nm
  if !(← tryMove nm n.x n.y) then
    damageMobj nm mi.toInt32 mi.toInt32 10000
    return
  let t := (← getMo mi).target
  modifyMo nm fun m => { m with target := t }
  aSkullAttack nm

partial def aPainAttack (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  aFaceTarget mi
  aPainShootSkull mi (← getMo mi).angle

partial def aPainDie (mi : Nat) : M Unit := do
  aFall mi
  let a := (← getMo mi).angle
  aPainShootSkull mi (a + ANG90)
  aPainShootSkull mi (a + ANG180)
  aPainShootSkull mi (a + ANG270)

/-- True if no other mobj of `mo`'s type is alive (thinker scan). -/
partial def allOthersDead (mi : Nat) : M Bool := do
  let gs ← get
  let mo := gs.mobjs[mi]!
  for r in gs.thinkers do
    if thKind r != TK_MOBJ then continue
    let j := thIdx r
    let m2 := gs.mobjs[j]!
    if m2.func != THF_MOBJ then continue
    if j != mi && m2.typeId == mo.typeId && m2.health > 0 then return false
  return true

partial def aKeenDie (mi : Nat) : M Unit := do
  aFall mi
  if !(← allOthersDead mi) then return
  let _ ← evDoDoor 666 vld_open

/-- `CheckBossEnd`. -/
partial def checkBossEnd (motype : Int32) : M Bool := do
  let gs ← get
  if gs.res.gameversion < exe_ultimate then
    if gs.gamemap != 8 then return false
    if motype == MT_BRUISER && gs.gameepisode != 1 then return false
    return true
  match gs.gameepisode.toNatClampNeg with
  | 1 => return gs.gamemap == 8 && motype == MT_BRUISER
  | 2 => return gs.gamemap == 8 && motype == MT_CYBORG
  | 3 => return gs.gamemap == 8 && motype == MT_SPIDER
  | 4 => return (gs.gamemap == 6 && motype == MT_CYBORG) || (gs.gamemap == 8 && motype == MT_SPIDER)
  | _ => return gs.gamemap == 8

partial def aBossDeath (mi : Nat) : M Unit := do
  let gs ← get
  let mo := gs.mobjs[mi]!
  if gs.res.gamemode == gm_commercial then
    if gs.gamemap != 7 then return
    if mo.typeId != MT_FATSO && mo.typeId != MT_BABY then return
  else
    if !(← checkBossEnd mo.typeId) then return
  let mut alive := false
  for i in [0:4] do
    if gs.playeringame[i]! && gs.players[i]!.health > 0 then alive := true
  if !alive then return
  if !(← allOthersDead mi) then return
  if gs.res.gamemode == gm_commercial then
    if gs.gamemap == 7 then
      if mo.typeId == MT_FATSO then
        let _ ← evDoFloor 666 (-1) lowerFloorToLowest
        return
      if mo.typeId == MT_BABY then
        let _ ← evDoFloor 667 (-1) raiseToTexture
        return
  else
    if gs.gameepisode == 1 then
      let _ ← evDoFloor 666 (-1) lowerFloorToLowest
      return
    if gs.gameepisode == 4 then
      if gs.gamemap == 6 then
        let _ ← evDoDoor 666 vld_blazeOpen
        return
      if gs.gamemap == 8 then
        let _ ← evDoFloor 666 (-1) lowerFloorToLowest
        return
  gExitLevel

partial def aBrainScream (mi : Nat) : M Unit := do
  let mo ← getMo mi
  let mut x := mo.x - 196 * FRACUNIT
  while x < mo.x + 320 * FRACUNIT do
    let y := mo.y - 320 * FRACUNIT
    let z := 128 + (← pRandomM) * 2 * FRACUNIT
    let th ← spawnMobj x y z MT_ROCKET
    let r ← pRandomM
    modifyMo th fun m => { m with momz := r * 512 }
    let _ ← setMobjState th S_BRAINEXPLODE1
    let r ← pRandomM
    modifyMo th fun m => let t := m.tics - (r &&& 7); { m with tics := if t < 1 then 1 else t }
    x := x + FRACUNIT * 8
  startSoundNull sfx_bosdth

partial def aBrainExplode (mi : Nat) : M Unit := do
  let mo ← getMo mi
  let x := mo.x + (← pSubRandomM) * 2048
  let y := mo.y
  let z := 128 + (← pRandomM) * 2 * FRACUNIT
  let th ← spawnMobj x y z MT_ROCKET
  let r ← pRandomM
  modifyMo th fun m => { m with momz := r * 512 }
  let _ ← setMobjState th S_BRAINEXPLODE1
  let r ← pRandomM
  modifyMo th fun m => let t := m.tics - (r &&& 7); { m with tics := if t < 1 then 1 else t }

partial def aBrainSpit (mi : Nat) : M Unit := do
  modify fun gs => { gs with brainEasy := gs.brainEasy ^^^ 1 }
  let gs ← get
  if gs.gameskill <= sk_easy && gs.brainEasy == 0 then return
  if gs.braintargets.size == 0 then
    iError "A_BrainSpit: numbraintargets was 0 (vanilla crashes here)"
    return
  let targ := gs.braintargets[gs.braintargeton]!
  set { gs with braintargeton := (gs.braintargeton + 1) % gs.braintargets.size }
  let nm ← spawnMissile mi targ.toInt32 MT_SPAWNSHOT
  let gs ← get
  let n := gs.mobjs[nm]!
  let t := gs.mobjs[targ]!
  let mo := gs.mobjs[mi]!
  let stTics := gs.states[n.state.toNat]!.tics
  modifyMo nm fun m => { m with target := targ.toInt32
                                reactiontime := ((t.y - mo.y) / n.momy) / stTics }
  startSoundNull sfx_bospit

partial def aSpawnFly (mi : Nat) : M Unit := do
  modifyMo mi fun m => { m with reactiontime := m.reactiontime - 1 }
  let mo ← getMo mi
  if mo.reactiontime != 0 then return
  let gs ← get
  let targ : Mobj := if mo.target >= 0 then gs.mobjs[mo.target.toNatClampNeg]!
    else { Mobj.empty with x := 0, y := 0, z := 0, flags := 0 }
  let fog ← spawnMobj targ.x targ.y targ.z MT_SPAWNFIRE
  startSoundMo fog sfx_telept
  let r ← pRandomM
  let type_ :=
    if r < 50 then MT_TROOP else if r < 90 then MT_SERGEANT else if r < 120 then MT_SHADOWS
    else if r < 130 then MT_PAIN else if r < 160 then MT_HEAD else if r < 162 then MT_VILE
    else if r < 172 then MT_UNDEAD else if r < 192 then MT_BABY else if r < 222 then MT_FATSO
    else if r < 246 then MT_KNIGHT else MT_BRUISER
  let nm ← spawnMobj targ.x targ.y targ.z type_
  if ← lookForPlayers nm true then
    let _ ← setMobjState nm (← getInfo type_).seestate
  let n ← getMo nm
  let _ ← teleportMove nm n.x n.y
  removeMobj mi

partial def aExplode (mi : Nat) : M Unit := do
  radiusAttack mi (← getMo mi).target 128

partial def aBFGSpray (mi : Nat) : M Unit := do
  for i in [0:40] do
    let mo ← getMo mi
    let an := mo.angle - ANG90 / 2 + (ANG90 / 40) * i.toUInt32
    let _ ← aimLineAttack mo.target an (16 * 64 * FRACUNIT)
    let lt := (← get).linetarget
    if lt < 0 then continue
    let t ← getMoI lt
    let _ ← spawnMobj t.x t.y (t.z + (t.height >>> 2)) MT_EXTRABFG
    let mut damage : Int32 := 0
    for _ in [0:15] do
      damage := damage + ((← pRandomM) &&& 7) + 1
    let tgt := (← getMo mi).target
    damageMobj lt.toNatClampNeg tgt tgt damage

-- ### Player weapons

/-- `P_SetPsprite(player, position, stnum)`. -/
partial def setPsprite (pi pos : Nat) (stnum0 : UInt32) : M Unit := do
  let mut stnum := stnum0
  repeat
    if stnum == 0 then
      setPsp pi pos fun s => { s with state := 0 }
      break
    let st ← getState stnum
    setPsp pi pos fun s =>
      let s := { s with state := stnum, tics := st.tics }
      if st.misc1 != 0 then { s with sx := st.misc1 <<< 16, sy := st.misc2 <<< 16 } else s
    if st.action != actionNull then
      runPspAction pi pos st.action
      if (psp (← getPl pi) pos).state == 0 then break
    let cur := psp (← getPl pi) pos
    stnum := (← getState cur.state).nextstate
    if cur.tics != 0 then break

/-- Weapon action dispatch (`st->action.acp2(player, psp)`). -/
partial def runPspAction (pi pos : Nat) (action : UInt32) : M Unit := do
  match action.toNat with
  | 1 => modifyPl pi fun p => { p with extralight := 0 }
  | 2 => aWeaponReady pi pos
  | 3 => aLower pi pos
  | 4 => aRaise pi pos
  | 5 => aPunch pi
  | 6 => aReFire pi
  | 7 => aFirePistol pi
  | 8 => modifyPl pi fun p => { p with extralight := 1 }
  | 9 => aFireShotgun pi
  | 10 => modifyPl pi fun p => { p with extralight := 2 }
  | 11 => aFireShotgun2 pi
  | 12 => let _ ← checkAmmo pi
  | 13 => startSoundMo (← getPl pi).mo.toNatClampNeg sfx_dbopn
  | 14 => startSoundMo (← getPl pi).mo.toNatClampNeg sfx_dbload
  | 15 => do startSoundMo (← getPl pi).mo.toNatClampNeg sfx_dbcls; aReFire pi
  | 16 => aFireCGun pi pos
  | 17 => aGunFlash pi
  | 18 => aFireMissile pi
  | 19 => aSaw pi
  | 20 => aFirePlasma pi
  | 21 => startSoundMo (← getPl pi).mo.toNatClampNeg sfx_bfg
  | 22 => aFireBFG pi
  | _ => pure ()

/-- `P_BringUpWeapon`. -/
partial def bringUpWeapon (pi : Nat) : M Unit := do
  let p ← getPl pi
  if p.pendingweapon == wp_nochange then modifyPl pi fun p => { p with pendingweapon := p.readyweapon }
  let p ← getPl pi
  if p.pendingweapon == wp_chainsaw then startSoundMo p.mo.toNatClampNeg sfx_sawup
  let newstate := weaponinfo[p.pendingweapon.toNatClampNeg]!.upstate
  modifyPl pi fun p => { p with pendingweapon := wp_nochange }
  setPsp pi ps_weapon fun s => { s with sy := WEAPONBOTTOM }
  setPsprite pi ps_weapon newstate

/-- `P_CheckAmmo`. -/
partial def checkAmmo (pi : Nat) : M Bool := do
  let gs ← get
  let p := gs.players[pi]!
  let ammo := weaponinfo[p.readyweapon.toNatClampNeg]!.ammo
  let count : Int32 := if p.readyweapon == wp_bfg then 40
    else if p.readyweapon == wp_supershotgun then 2 else 1
  if ammo == am_noammo || p.ammo[ammo.toNatClampNeg]! >= count then return true
  let owned (w : Int32) := p.weaponowned[w.toNatClampNeg]! != 0
  let am (a : Nat) := p.ammo[a]!
  let gm := gs.res.gamemode
  let pw :=
    if owned wp_plasma && am 2 != 0 && gm != gm_shareware then wp_plasma
    else if owned wp_supershotgun && am 1 > 2 && gm == gm_commercial then wp_supershotgun
    else if owned wp_chaingun && am 0 != 0 then wp_chaingun
    else if owned wp_shotgun && am 1 != 0 then wp_shotgun
    else if am 0 != 0 then wp_pistol
    else if owned wp_chainsaw then wp_chainsaw
    else if owned wp_missile && am 3 != 0 then wp_missile
    else if owned wp_bfg && am 2 > 40 && gm != gm_shareware then wp_bfg
    else wp_fist
  modifyPl pi fun p => { p with pendingweapon := pw }
  setPsprite pi ps_weapon weaponinfo[p.readyweapon.toNatClampNeg]!.downstate
  return false

/-- `P_FireWeapon`. -/
partial def fireWeapon (pi : Nat) : M Unit := do
  if !(← checkAmmo pi) then return
  let p ← getPl pi
  let _ ← setMobjState p.mo.toNatClampNeg S_PLAY_ATK1
  let p ← getPl pi
  setPsprite pi ps_weapon weaponinfo[p.readyweapon.toNatClampNeg]!.atkstate
  let p ← getPl pi
  noiseAlert p.mo.toNatClampNeg p.mo.toNatClampNeg

/-- `P_DropWeapon`. -/
partial def dropWeapon (pi : Nat) : M Unit := do
  let p ← getPl pi
  setPsprite pi ps_weapon weaponinfo[p.readyweapon.toNatClampNeg]!.downstate

partial def aWeaponReady (pi pos : Nat) : M Unit := do
  let p ← getPl pi
  let mo ← getMoI p.mo
  if mo.state == S_PLAY_ATK1 || mo.state == S_PLAY_ATK2 then
    let _ ← setMobjState p.mo.toNatClampNeg S_PLAY
  let p ← getPl pi
  if p.readyweapon == wp_chainsaw && (psp p pos).state == S_SAW then
    startSoundMo p.mo.toNatClampNeg sfx_sawidl
  let p ← getPl pi
  if p.pendingweapon != wp_nochange || p.health == 0 then
    setPsprite pi ps_weapon weaponinfo[p.readyweapon.toNatClampNeg]!.downstate
    return
  if (p.cmd.buttons &&& BT_ATTACK) != 0 then
    if !p.attackdown || (p.readyweapon != wp_missile && p.readyweapon != wp_bfg) then
      modifyPl pi fun p => { p with attackdown := true }
      fireWeapon pi
      return
  else modifyPl pi fun p => { p with attackdown := false }
  let gs ← get
  let p := gs.players[pi]!
  let angle := (128 * gs.leveltime) &&& 8191
  let angle2 := angle &&& (8192 / 2 - 1)
  setPsp pi pos fun s => { s with sx := FRACUNIT + fixedMul p.bob finecosine[angle.toNat]!
                                  sy := WEAPONTOP + fixedMul p.bob finesine[angle2.toNat]! }

partial def aReFire (pi : Nat) : M Unit := do
  let p ← getPl pi
  if (p.cmd.buttons &&& BT_ATTACK) != 0 && p.pendingweapon == wp_nochange && p.health != 0 then
    modifyPl pi fun p => { p with refire := p.refire + 1 }
    fireWeapon pi
  else
    modifyPl pi fun p => { p with refire := 0 }
    let _ ← checkAmmo pi

partial def aLower (pi pos : Nat) : M Unit := do
  setPsp pi pos fun s => { s with sy := s.sy + LOWERSPEED }
  let p ← getPl pi
  if (psp p pos).sy < WEAPONBOTTOM then return
  if p.playerstate == PST_DEAD then
    setPsp pi pos fun s => { s with sy := WEAPONBOTTOM }
    return
  if p.health == 0 then
    setPsprite pi ps_weapon S_NULL
    return
  modifyPl pi fun p => { p with readyweapon := p.pendingweapon }
  bringUpWeapon pi

partial def aRaise (pi pos : Nat) : M Unit := do
  setPsp pi pos fun s => { s with sy := s.sy - RAISESPEED }
  let p ← getPl pi
  if (psp p pos).sy > WEAPONTOP then return
  setPsp pi pos fun s => { s with sy := WEAPONTOP }
  setPsprite pi ps_weapon weaponinfo[p.readyweapon.toNatClampNeg]!.readystate

partial def aGunFlash (pi : Nat) : M Unit := do
  let p ← getPl pi
  let _ ← setMobjState p.mo.toNatClampNeg S_PLAY_ATK2
  let p ← getPl pi
  setPsprite pi ps_flash weaponinfo[p.readyweapon.toNatClampNeg]!.flashstate

partial def aPunch (pi : Nat) : M Unit := do
  let mut damage := ((← pRandomM) % 10 + 1) <<< 1
  let p ← getPl pi
  if p.powers[pw_strength]! != 0 then damage := damage * 10
  let mi := p.mo.toNatClampNeg
  let angle := (← getMo mi).angle + ((← pSubRandomM).toUInt32 <<< 18)
  let slope ← aimLineAttack mi.toInt32 angle MELEERANGE
  lineAttack mi angle MELEERANGE slope damage
  let gs ← get
  if gs.linetarget >= 0 then
    startSoundMo mi sfx_punch
    let gs ← get
    let mo := gs.mobjs[mi]!
    let lt := gs.mobjs[gs.linetarget.toNatClampNeg]!
    modifyMo mi fun m => { m with angle := pointToAngle2 mo.x mo.y lt.x lt.y }

partial def aSaw (pi : Nat) : M Unit := do
  let damage := 2 * ((← pRandomM) % 10 + 1)
  let p ← getPl pi
  let mi := p.mo.toNatClampNeg
  let angle := (← getMo mi).angle + ((← pSubRandomM).toUInt32 <<< 18)
  let slope ← aimLineAttack mi.toInt32 angle (MELEERANGE + 1)
  lineAttack mi angle (MELEERANGE + 1) slope damage
  if (← get).linetarget < 0 then
    startSoundMo mi sfx_sawful
    return
  startSoundMo mi sfx_sawhit
  let gs ← get
  let mo := gs.mobjs[mi]!
  let lt := gs.mobjs[gs.linetarget.toNatClampNeg]!
  let angle := pointToAngle2 mo.x mo.y lt.x lt.y
  modifyMo mi fun m =>
    let a :=
      if angle - m.angle > ANG180 then
        if (angle - m.angle).toInt32 < -((ANG90 / 20).toInt32) then angle + ANG90 / 21
        else m.angle - ANG90 / 20
      else
        if angle - m.angle > ANG90 / 20 then angle - ANG90 / 21
        else m.angle + ANG90 / 20
    { m with angle := a, flags := m.flags ||| MF_JUSTATTACKED }

partial def aFireMissile (pi : Nat) : M Unit := do
  let p ← getPl pi
  decreaseAmmo pi weaponinfo[p.readyweapon.toNatClampNeg]!.ammo 1
  spawnPlayerMissile p.mo.toNatClampNeg MT_ROCKET

partial def aFireBFG (pi : Nat) : M Unit := do
  let p ← getPl pi
  decreaseAmmo pi weaponinfo[p.readyweapon.toNatClampNeg]!.ammo 40
  spawnPlayerMissile p.mo.toNatClampNeg MT_BFG

partial def aFirePlasma (pi : Nat) : M Unit := do
  let p ← getPl pi
  let wi := weaponinfo[p.readyweapon.toNatClampNeg]!
  decreaseAmmo pi wi.ammo 1
  let r ← pRandomM
  setPsprite pi ps_flash (wi.flashstate + (r &&& 1).toUInt32)
  spawnPlayerMissile (← getPl pi).mo.toNatClampNeg MT_PLASMA

/-- `P_GunShot(mo, accurate)`. -/
partial def gunShot (mi : Nat) (accurate : Bool) : M Unit := do
  let damage := 5 * ((← pRandomM) % 3 + 1)
  let mut angle := (← getMo mi).angle
  if !accurate then angle := angle + ((← pSubRandomM).toUInt32 <<< 18)
  lineAttack mi angle MISSILERANGE (← get).bulletslope damage

partial def aFirePistol (pi : Nat) : M Unit := do
  let p ← getPl pi
  let mi := p.mo.toNatClampNeg
  startSoundMo mi sfx_pistol
  let _ ← setMobjState mi S_PLAY_ATK2
  let wi := weaponinfo[(← getPl pi).readyweapon.toNatClampNeg]!
  decreaseAmmo pi wi.ammo 1
  setPsprite pi ps_flash wi.flashstate
  bulletSlope mi
  gunShot mi ((← getPl pi).refire == 0)

partial def aFireShotgun (pi : Nat) : M Unit := do
  let p ← getPl pi
  let mi := p.mo.toNatClampNeg
  startSoundMo mi sfx_shotgn
  let _ ← setMobjState mi S_PLAY_ATK2
  let wi := weaponinfo[(← getPl pi).readyweapon.toNatClampNeg]!
  decreaseAmmo pi wi.ammo 1
  setPsprite pi ps_flash wi.flashstate
  bulletSlope mi
  for _ in [0:7] do gunShot mi false

partial def aFireShotgun2 (pi : Nat) : M Unit := do
  let p ← getPl pi
  let mi := p.mo.toNatClampNeg
  startSoundMo mi sfx_dshtgn
  let _ ← setMobjState mi S_PLAY_ATK2
  let wi := weaponinfo[(← getPl pi).readyweapon.toNatClampNeg]!
  decreaseAmmo pi wi.ammo 2
  setPsprite pi ps_flash wi.flashstate
  bulletSlope mi
  for _ in [0:20] do
    let damage := 5 * ((← pRandomM) % 3 + 1)
    let angle := (← getMo mi).angle + ((← pSubRandomM).toUInt32 <<< 19)
    let slope := (← get).bulletslope + ((← pSubRandomM) <<< 5)
    lineAttack mi angle MISSILERANGE slope damage

partial def aFireCGun (pi pos : Nat) : M Unit := do
  let p ← getPl pi
  let mi := p.mo.toNatClampNeg
  startSoundMo mi sfx_pistol
  let p ← getPl pi
  let wi := weaponinfo[p.readyweapon.toNatClampNeg]!
  if p.ammo[wi.ammo.toNatClampNeg]! == 0 then return
  let _ ← setMobjState mi S_PLAY_ATK2
  decreaseAmmo pi wi.ammo 1
  let cur := (psp (← getPl pi) pos).state
  setPsprite pi ps_flash (wi.flashstate + cur - S_CHAIN1)
  bulletSlope mi
  gunShot mi ((← getPl pi).refire == 0)

end

/-- `P_MovePsprites`. -/
def movePsprites (pi : Nat) : M Unit := do
  for i in [0:2] do
    let s := psp (← getPl pi) i
    if s.state != 0 && s.tics != -1 then
      setPsp pi i fun s => { s with tics := s.tics - 1 }
      if (psp (← getPl pi) i).tics == 0 then
        setPsprite pi i (← getState s.state).nextstate
  modifyPl pi fun p =>
    let w := p.psprites[ps_weapon]!
    { p with psprites := p.psprites.modify ps_flash fun f => { f with sx := w.sx, sy := w.sy } }

/-- `P_SetupPsprites`. -/
def setupPsprites (pi : Nat) : M Unit := do
  modifyPl pi fun p => { p with psprites := p.psprites.map fun s => { s with state := 0 }
                                pendingweapon := p.readyweapon }
  bringUpWeapon pi

end Doom.Playsim.Core
