import Doom.Playsim.Angle
import Doom.Playsim.Core
import Doom.Playsim.CoreBase
import Doom.Playsim.Defs
import Doom.Playsim.GameState
import Doom.Playsim.MapUtil
import Doom.Playsim.Sound
import Doom.Playsim.SpecUtil
import Doom.Playsim.Switch

/-!
# Doom.Playsim.User

`p_user.c` (`P_PlayerThink`, movement, view height, death) plus
`P_UseLines` (`p_map.c`) and `P_PlayerInSpecialSector` (`p_spec.c`).
-/

namespace Doom.Playsim.User

open Doom.Playsim.Angle
open Doom.Playsim.Core
open Doom.Playsim.CoreBase
open Doom.Playsim.Defs
open Doom.Playsim.Fixed
open Doom.Playsim.Flags
open Doom.Playsim.GameState
open Doom.Playsim.MapUtil
open Doom.Playsim.Player
open Doom.Playsim.Sound
open Doom.Playsim.SpecUtil
open Doom.Playsim.Switch
open Doom.Playsim.Tables

def MAXBOB : Int32 := 0x100000
def CF_NOCLIP : Int32 := 1
def CF_GODMODE : Int32 := 2
def CF_NOMOMENTUM : Int32 := 4
def BT_SPECIAL : UInt32 := 128
def INVERSECOLORMAP : Int32 := 32

/-- `P_Thrust`. -/
def thrust (pi : Nat) (angle : UInt32) (move : Int32) : M Unit := do
  let p ← getPl pi
  modifyMo p.mo.toNatClampNeg fun m => { m with momx := m.momx + fixedMul move (fineCos angle)
                                                momy := m.momy + fixedMul move (fineSin angle) }

/-- `P_CalcHeight`. -/
def calcHeight (pi : Nat) : M Unit := do
  let gs ← get
  let p := gs.players[pi]!
  let mo := gs.mobjs[p.mo.toNatClampNeg]!
  let mut bob := (fixedMul mo.momx mo.momx + fixedMul mo.momy mo.momy) >>> 2
  if bob > MAXBOB then bob := MAXBOB
  if (p.cheats &&& CF_NOMOMENTUM) != 0 || !gs.onground then
    modifyPl pi fun p => { p with bob, viewz := mo.z + p.viewheight }
    return
  let angle := (8192 / 20 * gs.leveltime) &&& 8191
  let b := fixedMul (bob / 2) finesine[angle.toNat]!
  let mut p := { p with bob }
  if p.playerstate == PST_LIVE then
    p := { p with viewheight := p.viewheight + p.deltaviewheight }
    if p.viewheight > VIEWHEIGHT then p := { p with viewheight := VIEWHEIGHT, deltaviewheight := 0 }
    if p.viewheight < VIEWHEIGHT / 2 then
      p := { p with viewheight := VIEWHEIGHT / 2 }
      if p.deltaviewheight <= 0 then p := { p with deltaviewheight := 1 }
    if p.deltaviewheight != 0 then
      let d := p.deltaviewheight + FRACUNIT / 4
      p := { p with deltaviewheight := if d == 0 then 1 else d }
  let mut viewz := mo.z + p.viewheight + b
  if viewz > mo.ceilingz - 4 * FRACUNIT then viewz := mo.ceilingz - 4 * FRACUNIT
  modifyPl pi fun _ => { p with viewz }

/-- `P_MovePlayer`. -/
def movePlayer (pi : Nat) : M Unit := do
  let p ← getPl pi
  let mi := p.mo.toNatClampNeg
  let cmd := p.cmd
  modifyMo mi fun m => { m with angle := m.angle + (cmd.angleturn.toUInt32 <<< 16) }
  let mo ← getMo mi
  let onground := mo.z <= mo.floorz
  modify fun gs => { gs with onground }
  if cmd.forwardmove != 0 && onground then thrust pi mo.angle (cmd.forwardmove * 2048)
  if cmd.sidemove != 0 && onground then thrust pi (mo.angle - ANG90) (cmd.sidemove * 2048)
  if (cmd.forwardmove != 0 || cmd.sidemove != 0) && (← getMo mi).state == S_PLAY then
    let _ ← setMobjState mi S_PLAY_RUN1

/-- `P_DeathThink`. -/
def deathThink (pi : Nat) : M Unit := do
  movePsprites pi
  modifyPl pi fun p =>
    let vh := if p.viewheight > 6 * FRACUNIT then p.viewheight - FRACUNIT else p.viewheight
    let vh := if vh < 6 * FRACUNIT then 6 * FRACUNIT else vh
    { p with viewheight := vh, deltaviewheight := 0 }
  let p ← getPl pi
  let mo ← getMoI p.mo
  modify fun gs => { gs with onground := mo.z <= mo.floorz }
  calcHeight pi
  let p ← getPl pi
  if p.attacker >= 0 && p.attacker != p.mo then
    let gs ← get
    let mo := gs.mobjs[p.mo.toNatClampNeg]!
    let att := gs.mobjs[p.attacker.toNatClampNeg]!
    let angle := pointToAngle2 mo.x mo.y att.x att.y
    let delta := angle - mo.angle
    if delta < ANG5 || delta > (0 : UInt32) - ANG5 then
      modifyMo p.mo.toNatClampNeg fun m => { m with angle }
      modifyPl pi fun p => if p.damagecount != 0 then { p with damagecount := p.damagecount - 1 } else p
    else if delta < ANG180 then
      modifyMo p.mo.toNatClampNeg fun m => { m with angle := m.angle + ANG5 }
    else
      modifyMo p.mo.toNatClampNeg fun m => { m with angle := m.angle - ANG5 }
  else
    modifyPl pi fun p => if p.damagecount != 0 then { p with damagecount := p.damagecount - 1 } else p
  if ((← getPl pi).cmd.buttons &&& BT_USE) != 0 then
    modifyPl pi fun p => { p with playerstate := PST_REBORN }

/-- `PTR_UseTraverse`. -/
def ptrUseTraverse (_frac : Int32) (_isaline : Bool) (d : Nat) : M Bool := do
  let gs ← get
  let ld := gs.level.lines[d]!
  if ld.special == 0 then
    lineOpening ld
    if (← get).openrange <= 0 then
      startSoundMo (← get).usething.toNatClampNeg sfx_noway
      return false
    return true
  let ut := gs.mobjs[gs.usething.toNatClampNeg]!
  let side := if pointOnLineSide gs ut.x ut.y ld == 1 then 1 else 0
  let _ ← useSpecialLine gs.usething.toNatClampNeg d side
  return false

/-- `P_UseLines`. -/
def useLines (pi : Nat) : M Unit := do
  let p ← getPl pi
  let mo ← getMoI p.mo
  modify fun gs => { gs with usething := p.mo }
  let x1 := mo.x
  let y1 := mo.y
  let x2 := x1 + (USERANGE >>> 16) * fineCos mo.angle
  let y2 := y1 + (USERANGE >>> 16) * fineSin mo.angle
  let _ ← pathTraverse x1 y1 x2 y2 PT_ADDLINES ptrUseTraverse

/-- `P_PlayerInSpecialSector`. -/
def playerInSpecialSector (pi : Nat) : M Unit := do
  let gs ← get
  let p := gs.players[pi]!
  let mo := gs.mobjs[p.mo.toNatClampNeg]!
  let s := moSector gs mo
  let sec := gs.sectors[s]!
  if mo.z != sec.floorheight then return
  let lt := gs.leveltime
  let mi := p.mo.toNatClampNeg
  match sec.special.toNatClampNeg with
  | 5 => if p.powers[pw_ironfeet]! == 0 && (lt &&& 0x1f) == 0 then damageMobj mi (-1) (-1) 10
  | 7 => if p.powers[pw_ironfeet]! == 0 && (lt &&& 0x1f) == 0 then damageMobj mi (-1) (-1) 5
  | 16 | 4 =>
    let hurt ← if p.powers[pw_ironfeet]! == 0 then pure true else pure ((← pRandomM) < 5)
    if hurt && (lt &&& 0x1f) == 0 then damageMobj mi (-1) (-1) 20
  | 9 =>
    modifyPl pi fun p => { p with secretcount := p.secretcount + 1 }
    modifySec s fun sec => { sec with special := 0 }
  | 11 =>
    modifyPl pi fun p => { p with cheats := p.cheats &&& ~~~CF_GODMODE }
    if (lt &&& 0x1f) == 0 then damageMobj mi (-1) (-1) 20
    if (← getPl pi).health <= 10 then gExitLevel
  | _ => iError s!"P_PlayerInSpecialSector: unknown special {sec.special}"

/-- `P_PlayerThink`. -/
def playerThink (pi : Nat) : M Unit := do
  let p ← getPl pi
  let mi := p.mo.toNatClampNeg
  modifyMo mi fun m => { m with flags := if (p.cheats &&& CF_NOCLIP) != 0 then m.flags ||| MF_NOCLIP
                                         else m.flags &&& ~~~MF_NOCLIP }
  if ((← getMo mi).flags &&& MF_JUSTATTACKED) != 0 then
    modifyPl pi fun p => { p with cmd := { p.cmd with angleturn := 0, forwardmove := 0xc800 / 512, sidemove := 0 } }
    modifyMo mi fun m => { m with flags := m.flags &&& ~~~MF_JUSTATTACKED }
  if (← getPl pi).playerstate == PST_DEAD then
    deathThink pi
    return
  if (← getMo mi).reactiontime != 0 then
    modifyMo mi fun m => { m with reactiontime := m.reactiontime - 1 }
  else movePlayer pi
  calcHeight pi
  let gs ← get
  let gamemode := gs.res.gamemode
  if gs.sectors[moSector gs gs.mobjs[mi]!]!.special != 0 then playerInSpecialSector pi
  let p ← getPl pi
  if (p.cmd.buttons &&& BT_SPECIAL) != 0 then
    modifyPl pi fun p => { p with cmd := { p.cmd with buttons := 0 } }
  let p ← getPl pi
  if (p.cmd.buttons &&& BT_CHANGE) != 0 then
    let mut newweapon : Int32 := ((p.cmd.buttons &&& BT_WEAPONMASK) >>> BT_WEAPONSHIFT).toInt32
    let owned (w : Int32) := p.weaponowned[w.toNatClampNeg]! != 0
    if newweapon == wp_fist && owned wp_chainsaw &&
        !(p.readyweapon == wp_chainsaw && p.powers[pw_strength]! != 0) then
      newweapon := wp_chainsaw
    if gamemode == gm_commercial && newweapon == wp_shotgun && owned wp_supershotgun &&
        p.readyweapon != wp_supershotgun then
      newweapon := wp_supershotgun
    if owned newweapon && newweapon != p.readyweapon then
      if (newweapon != wp_plasma && newweapon != wp_bfg) || gamemode != gm_shareware then
        modifyPl pi fun p => { p with pendingweapon := newweapon }
  let p ← getPl pi
  if (p.cmd.buttons &&& BT_USE) != 0 then
    if !p.usedown then
      useLines pi
      modifyPl pi fun p => { p with usedown := true }
  else modifyPl pi fun p => { p with usedown := false }
  movePsprites pi
  let p ← getPl pi
  let mut pw := p.powers
  if pw[pw_strength]! != 0 then pw := pw.modify pw_strength (· + 1)
  if pw[pw_invulnerability]! != 0 then pw := pw.modify pw_invulnerability (· - 1)
  if pw[pw_invisibility]! != 0 then
    pw := pw.modify pw_invisibility (· - 1)
    if pw[pw_invisibility]! == 0 then
      modifyMo mi fun m => { m with flags := m.flags &&& ~~~MF_SHADOW }
  if pw[pw_infrared]! != 0 then pw := pw.modify pw_infrared (· - 1)
  if pw[pw_ironfeet]! != 0 then pw := pw.modify pw_ironfeet (· - 1)
  let inv := pw[pw_invulnerability]!
  let ir := pw[pw_infrared]!
  let fcm : Int32 :=
    if inv != 0 then (if inv > 4 * 32 || (inv &&& 8) != 0 then INVERSECOLORMAP else 0)
    else if ir != 0 then (if ir > 4 * 32 || (ir &&& 8) != 0 then 1 else 0)
    else 0
  modifyPl pi fun p => {
    p with
    powers := pw
    damagecount := if p.damagecount != 0 then p.damagecount - 1 else 0
    bonuscount := if p.bonuscount != 0 then p.bonuscount - 1 else 0
    fixedcolormap := fcm }

end Doom.Playsim.User
