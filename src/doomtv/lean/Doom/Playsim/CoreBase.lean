import Doom.Playsim.Angle
import Doom.Playsim.Defs
import Doom.Playsim.Fixed
import Doom.Playsim.Flags
import Doom.Playsim.GameState
import Doom.Playsim.Level
import Doom.Playsim.MapUtil
import Doom.Playsim.MobjBase
import Doom.Playsim.Sight
import Doom.Playsim.Sound
import Doom.Playsim.Tables

/-!
# Doom.Playsim.CoreBase

Non-recursive pieces used by the action/damage/movement knot in `Core`:
target acquisition (`P_LookForPlayers`, `A_FaceTarget`, range checks),
noise alerts, `P_AimLineAttack`, `PIT_CheckLine`, and the simple sound-only
actions.
-/

namespace Doom.Playsim.CoreBase

open Doom.Playsim.Angle
open Doom.Playsim.Defs
open Doom.Playsim.Fixed
open Doom.Playsim.Flags
open Doom.Playsim.GameState
open Doom.Playsim.Level
open Doom.Playsim.MapUtil
open Doom.Playsim.Mobj
open Doom.Playsim.MobjBase
open Doom.Playsim.Sight
open Doom.Playsim.Sound
open Doom.Playsim.Tables

def MELEERANGE : Int32 := 64 * FRACUNIT
def MISSILERANGE : Int32 := 32 * 64 * FRACUNIT
def MAXMOVE : Int32 := 30 * FRACUNIT
def FLOATSPEED : Int32 := 4 * FRACUNIT
def BASETHRESHOLD : Int32 := 100
def USERANGE : Int32 := 64 * FRACUNIT
def ML_BLOCKING : Int32 := 1
def ML_BLOCKMONSTERS : Int32 := 2
def MAXSPECIALCROSS_ORIGINAL : Int32 := 8
def DEFAULT_SPECHIT_MAGIC : UInt32 := 0x01C09C98

def DI_EAST : Int32 := 0
def DI_NORTHEAST : Int32 := 1
def DI_NORTH : Int32 := 2
def DI_NORTHWEST : Int32 := 3
def DI_WEST : Int32 := 4
def DI_SOUTHWEST : Int32 := 5
def DI_SOUTH : Int32 := 6
def DI_SOUTHEAST : Int32 := 7
def DI_NODIR : Int32 := 8

def opposite : Array Int32 := #[DI_WEST, DI_SOUTHWEST, DI_SOUTH, DI_SOUTHEAST,
  DI_EAST, DI_NORTHEAST, DI_NORTH, DI_NORTHWEST, DI_NODIR]
def diags : Array Int32 := #[DI_NORTHWEST, DI_NORTHEAST, DI_SOUTHWEST, DI_SOUTHEAST]
def xspeed : Array Int32 := #[FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000, 0, 47000]
def yspeed : Array Int32 := #[0, 47000, FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000]

@[inline] def fineCos (an : UInt32) : Int32 := finecosine[(an >>> 19).toNat]!
@[inline] def fineSin (an : UInt32) : Int32 := finesine[(an >>> 19).toNat]!

/-- `A_FaceTarget`. -/
def aFaceTarget (mi : Nat) : M Unit := do
  let actor ← getMo mi
  if actor.target < 0 then return
  let targ ← getMoI actor.target
  let ang := pointToAngle2 actor.x actor.y targ.x targ.y
  modifyMo mi fun m => { m with flags := m.flags &&& ~~~MF_AMBUSH, angle := ang }
  if (targ.flags &&& MF_SHADOW) != 0 then
    let d ← pSubRandomM
    modifyMo mi fun m => { m with angle := m.angle + (d.toUInt32 <<< 21) }

/-- `P_CheckMeleeRange`. -/
def checkMeleeRange (mi : Nat) : M Bool := do
  let actor ← getMo mi
  if actor.target < 0 then return false
  let gs ← get
  let pl := gs.mobjs[actor.target.toNatClampNeg]!
  let dist := aproxDistance (pl.x - actor.x) (pl.y - actor.y)
  let range := if gs.res.gameversion < exe_doom_1_5 then MELEERANGE
    else MELEERANGE - 20 * FRACUNIT + gs.mobjinfo[pl.typeId.toNatClampNeg]!.radius
  if dist >= range then return false
  checkSight mi actor.target.toNatClampNeg

/-- `P_CheckMissileRange`. -/
def checkMissileRange (mi : Nat) : M Bool := do
  let actor ← getMo mi
  if !(← checkSight mi actor.target.toNatClampNeg) then return false
  let actor ← getMo mi
  if (actor.flags &&& MF_JUSTHIT) != 0 then
    modifyMo mi fun m => { m with flags := m.flags &&& ~~~MF_JUSTHIT }
    return true
  if actor.reactiontime != 0 then return false
  let gs ← get
  let targ := gs.mobjs[actor.target.toNatClampNeg]!
  let mut dist := aproxDistance (actor.x - targ.x) (actor.y - targ.y) - 64 * FRACUNIT
  if gs.mobjinfo[actor.typeId.toNatClampNeg]!.meleestate == 0 then dist := dist - 128 * FRACUNIT
  dist := dist >>> 16
  if actor.typeId == MT_VILE then
    if dist > 14 * 64 then return false
  if actor.typeId == MT_UNDEAD then
    if dist < 196 then return false
    dist := dist >>> 1
  if actor.typeId == MT_CYBORG || actor.typeId == MT_SPIDER || actor.typeId == MT_SKULL then
    dist := dist >>> 1
  if dist > 200 then dist := 200
  if actor.typeId == MT_CYBORG && dist > 160 then dist := 160
  if (← pRandomM) < dist then return false
  return true

/-- `P_LookForPlayers`. -/
def lookForPlayers (mi : Nat) (allaround : Bool) : M Bool := do
  let mut c : Nat := 0
  let stop := ((← getMo mi).lastlook - 1) &&& 3
  repeat
    let gs ← get
    let actor := gs.mobjs[mi]!
    let look := actor.lastlook
    let next : M Unit := modifyMo mi fun m => { m with lastlook := (m.lastlook + 1) &&& 3 }
    if !gs.playeringame[look.toNatClampNeg]! then next; continue
    let prevc := c
    c := c + 1
    if prevc == 2 || look == stop then return false
    let player := gs.players[look.toNatClampNeg]!
    if player.health <= 0 then next; continue
    if !(← checkSight mi player.mo.toNatClampNeg) then next; continue
    if !allaround then
      let gs ← get
      let actor := gs.mobjs[mi]!
      let pmo := gs.mobjs[player.mo.toNatClampNeg]!
      let an := pointToAngle2 actor.x actor.y pmo.x pmo.y - actor.angle
      if an > ANG90 && an < ANG270 then
        let dist := aproxDistance (pmo.x - actor.x) (pmo.y - actor.y)
        if dist > MELEERANGE then next; continue
    modifyMo mi fun m => { m with target := player.mo }
    return true
  return false

/-- `P_RecursiveSound`. -/
partial def recursiveSound (sec : Nat) (soundblocks : Int32) : M Unit := do
  let gs ← get
  let s := gs.sectors[sec]!
  if s.validcount == gs.validcount && s.soundtraversed <= soundblocks + 1 then return
  set {
    gs with
    sectors := gs.sectors.modify sec fun s => { s with validcount := gs.validcount, soundtraversed := soundblocks + 1, soundtarget := gs.soundtarget } }
  for li in s.lines do
    let check := (← get).level.lines[li.toNat]!
    if (check.flags &&& ML_TWOSIDED) == 0 then continue
    lineOpening check
    let gs ← get
    if gs.openrange <= 0 then continue
    let other :=
      if gs.level.sides[check.sidenum0.toNatClampNeg]!.sector.toNat == sec then
        gs.level.sides[check.sidenum1.toNatClampNeg]!.sector.toNat
      else gs.level.sides[check.sidenum0.toNatClampNeg]!.sector.toNat
    if (check.flags &&& ML_SOUNDBLOCK) != 0 then
      if soundblocks == 0 then recursiveSound other 1
    else recursiveSound other soundblocks

/-- `P_NoiseAlert(target, emitter)`. -/
def noiseAlert (target emitter : Nat) : M Unit := do
  modify fun gs => { gs with soundtarget := target.toInt32 }
  bumpValidcount
  let gs ← get
  recursiveSound (moSector gs gs.mobjs[emitter]!) 0

/-- `PTR_AimTraverse`. -/
def ptrAimTraverse (frac : Int32) (isaline : Bool) (d : Nat) : M Bool := do
  if isaline then
    let gs ← get
    let li := gs.level.lines[d]!
    if (li.flags &&& ML_TWOSIDED) == 0 then return false
    lineOpening li
    let gs ← get
    if gs.openbottom >= gs.opentop then return false
    let dist := fixedMul gs.attackrange frac
    let front := gs.sectors[li.frontsector.toNatClampNeg]!
    let backNull := li.backsector < 0
    let back := if backNull then front else gs.sectors[li.backsector.toNatClampNeg]!
    let mut bottomslope := gs.bottomslope
    let mut topslope := gs.topslope
    if backNull || front.floorheight != back.floorheight then
      let slope := fixedDiv (gs.openbottom - gs.shootz) dist
      if slope > bottomslope then bottomslope := slope
    if backNull || front.ceilingheight != back.ceilingheight then
      let slope := fixedDiv (gs.opentop - gs.shootz) dist
      if slope < topslope then topslope := slope
    set { gs with topslope, bottomslope }
    if topslope <= bottomslope then return false
    return true
  let gs ← get
  if d.toInt32 == gs.shootthing then return true
  let th := gs.mobjs[d]!
  if (th.flags &&& MF_SHOOTABLE) == 0 then return true
  let dist := fixedMul gs.attackrange frac
  let mut thingtopslope := fixedDiv (th.z + th.height - gs.shootz) dist
  if thingtopslope < gs.bottomslope then return true
  let mut thingbottomslope := fixedDiv (th.z - gs.shootz) dist
  if thingbottomslope > gs.topslope then return true
  if thingtopslope > gs.topslope then thingtopslope := gs.topslope
  if thingbottomslope < gs.bottomslope then thingbottomslope := gs.bottomslope
  set { gs with aimslope := (thingtopslope + thingbottomslope) / 2, linetarget := d.toInt32 }
  return false

/-- `P_AimLineAttack(t1, angle, distance)`; `t1 = -1` is `P_SubstNullMobj`. -/
def aimLineAttack (t1i : Int32) (angle : UInt32) (distance : Int32) : M Int32 := do
  let gs ← get
  let t1 : Mobj := if t1i >= 0 then gs.mobjs[t1i.toNatClampNeg]!
    else { Mobj.empty with x := 0, y := 0, z := 0, flags := 0 }
  let x2 := t1.x + (distance >>> 16) * fineCos angle
  let y2 := t1.y + (distance >>> 16) * fineSin angle
  set { gs with shootthing := t1i, shootz := t1.z + (t1.height >>> 1) + 8 * FRACUNIT
                topslope := (100 * FRACUNIT) / 160, bottomslope := -(100 * FRACUNIT) / 160
                attackrange := distance, linetarget := -1 }
  let _ ← pathTraverse t1.x t1.y x2 y2 (PT_ADDLINES ||| PT_ADDTHINGS) ptrAimTraverse
  let gs ← get
  return if gs.linetarget >= 0 then gs.aimslope else 0

/-- `P_BulletSlope(mo)`. -/
def bulletSlope (mi : Nat) : M Unit := do
  let mo ← getMo mi
  let mut an := mo.angle
  let mut s ← aimLineAttack mi.toInt32 an (16 * 64 * FRACUNIT)
  if (← get).linetarget < 0 then
    an := an + ((1 : UInt32) <<< 26)
    s ← aimLineAttack mi.toInt32 an (16 * 64 * FRACUNIT)
    if (← get).linetarget < 0 then
      an := an - ((2 : UInt32) <<< 26)
      s ← aimLineAttack mi.toInt32 an (16 * 64 * FRACUNIT)
  modify fun gs => { gs with bulletslope := s }

/-- Emulate a spechit overrun write (`SpechitOverrun`). -/
def spechitOverrun (lineIdx : Nat) : M Unit := modify fun gs =>
  let addr : UInt32 := DEFAULT_SPECHIT_MAGIC + lineIdx.toUInt32 * 0x3E
  let a := addr.toInt32
  match gs.numspechit.toNatClampNeg with
  | 9 => { gs with tmbboxTop := a }
  | 10 => { gs with tmbboxBottom := a }
  | 11 => { gs with tmbboxLeft := a }
  | 12 => { gs with tmbboxRight := a }
  | 13 => { gs with crushchange := a }
  | 14 => { gs with nofit := a }
  | _ => gs

/-- `PIT_CheckLine`. -/
def pitCheckLine (l : Nat) : M Bool := do
  let gs ← get
  let g := gs.lineGeo[l]!
  if gs.tmbboxRight <= g.bbLeft || gs.tmbboxLeft >= g.bbRight ||
      gs.tmbboxTop <= g.bbBottom || gs.tmbboxBottom >= g.bbTop then
    return true
  if boxOnLineSideG gs g != -1 then return true
  if g.back < 0 then return false
  let tm := gs.mobjs[gs.tmthing.toNatClampNeg]!
  if (tm.flags &&& MF_MISSILE) == 0 then
    if (g.flags &&& ML_BLOCKING) != 0 then return false
    if tm.player < 0 && (g.flags &&& ML_BLOCKMONSTERS) != 0 then return false
  let special := gs.level.lines[l]!.special
  lineOpeningG g
  let gs ← get
  let mut gs := gs
  if gs.opentop < gs.tmceilingz then
    gs := { gs with tmceilingz := gs.opentop, ceilingline := l.toInt32 }
  if gs.openbottom > gs.tmfloorz then gs := { gs with tmfloorz := gs.openbottom }
  if gs.lowfloor < gs.tmdropoffz then gs := { gs with tmdropoffz := gs.lowfloor }
  if special != 0 then
    let n := gs.numspechit.toNatClampNeg
    gs := { gs with spechit := if n < gs.spechit.size then gs.spechit.set! n l else gs.spechit.push l
                    numspechit := gs.numspechit + 1 }
    set gs
    if gs.numspechit > MAXSPECIALCROSS_ORIGINAL then spechitOverrun l
  else set gs
  return true

/-! ## Sound-only and other non-recursive actions -/

/-- `A_Pain`. -/
def aPain (mi : Nat) : M Unit := do
  let info ← getInfo (← getMo mi).typeId
  if info.painsound != 0 then startSoundMo mi info.painsound.toNatClampNeg

/-- `A_Scream`. -/
def aScream (mi : Nat) : M Unit := do
  let actor ← getMo mi
  let ds := (← getInfo actor.typeId).deathsound.toNatClampNeg
  if ds == 0 then return
  let sound ←
    if ds == sfx_podth1 || ds == sfx_podth2 || ds == sfx_podth3 then
      pure (sfx_podth1 + ((← pRandomM) % 3).toNatClampNeg)
    else if ds == sfx_bgdth1 || ds == sfx_bgdth2 then
      pure (sfx_bgdth1 + ((← pRandomM) % 2).toNatClampNeg)
    else pure ds
  if actor.typeId == MT_SPIDER || actor.typeId == MT_CYBORG then startSoundNull sound
  else startSoundMo mi sound

def aXScream (mi : Nat) : M Unit := startSoundMo mi sfx_slop

def aPlayerScream (mi : Nat) : M Unit := do
  let mo ← getMo mi
  let sound := if (← get).res.gamemode == gm_commercial && mo.health < -50 then sfx_pdiehi
               else sfx_pldeth
  startSoundMo mi sound

def aFall (mi : Nat) : M Unit := modifyMo mi fun m => { m with flags := m.flags &&& ~~~MF_SOLID }

def aVileStart (mi : Nat) : M Unit := startSoundMo mi sfx_vilatk

def aSkelWhoosh (mi : Nat) : M Unit := do
  if (← getMo mi).target < 0 then return
  aFaceTarget mi
  startSoundMo mi sfx_skeswg

def aFatRaise (mi : Nat) : M Unit := do
  aFaceTarget mi
  startSoundMo mi sfx_manatk

/-- `A_Fire`: keep the arch-vile fire in front of its victim. -/
def aFire (mi : Nat) : M Unit := do
  let actor ← getMo mi
  if actor.tracer < 0 then return
  let dest := actor.tracer.toNatClampNeg
  -- P_SubstNullMobj(actor->target): a NULL target would crash in P_CheckSight.
  if actor.target < 0 then return
  if !(← checkSight actor.target.toNatClampNeg dest) then return
  let d ← getMo dest
  unsetThingPosition mi
  modifyMo mi fun m => { m with x := d.x + fixedMul (24 * FRACUNIT) (fineCos d.angle)
                                y := d.y + fixedMul (24 * FRACUNIT) (fineSin d.angle)
                                z := d.z }
  setThingPosition mi

def aStartFire (mi : Nat) : M Unit := do startSoundMo mi sfx_flamst; aFire mi
def aFireCrackle (mi : Nat) : M Unit := do startSoundMo mi sfx_flame; aFire mi

/-- `A_BrainAwake`. -/
def aBrainAwake : M Unit := do
  let gs ← get
  let mut targets : Array Nat := #[]
  for r in gs.thinkers do
    if Thinker.thKind r == Thinker.TK_MOBJ then
      let m := gs.mobjs[Thinker.thIdx r]!
      if m.func == Thinker.THF_MOBJ && m.typeId == MT_BOSSTARGET then
        targets := targets.push (Thinker.thIdx r)
  set { gs with braintargets := targets, braintargeton := 0 }
  startSoundNull sfx_bossit

def aBrainPain : M Unit := startSoundNull sfx_bospn

end Doom.Playsim.CoreBase
