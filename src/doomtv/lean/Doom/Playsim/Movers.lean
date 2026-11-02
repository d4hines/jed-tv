import Doom.Playsim.Core
import Doom.Playsim.CoreBase
import Doom.Playsim.Defs
import Doom.Playsim.Floors
import Doom.Playsim.GameState
import Doom.Playsim.MapUtil
import Doom.Playsim.MobjBase
import Doom.Playsim.Sound
import Doom.Playsim.SpecDefs

/-!
# Doom.Playsim.Movers

Sector movers and light thinkers: `T_MovePlane` + `P_ChangeSector`
(`p_floor.c` / `p_map.c`), `T_VerticalDoor`, `T_MoveFloor`, `T_MoveCeiling`,
`T_PlatRaise`, and `p_lights.c` thinkers.
-/

namespace Doom.Playsim.Movers

open Doom.Playsim.Core
open Doom.Playsim.CoreBase
open Doom.Playsim.Defs
open Doom.Playsim.Fixed
open Doom.Playsim.Flags
open Doom.Playsim.Floors
open Doom.Playsim.GameState
open Doom.Playsim.Level
open Doom.Playsim.MapUtil
open Doom.Playsim.MobjBase
open Doom.Playsim.Sound
open Doom.Playsim.SpecDefs
open Doom.Playsim.Thinker

/-- `P_ThingHeightClip`. -/
def thingHeightClip (mi : Nat) : M Bool := do
  let th ← getMo mi
  let onfloor := th.z == th.floorz
  let _ ← checkPosition mi th.x th.y
  let gs ← get
  let tmfloorz := gs.tmfloorz
  let tmceilingz := gs.tmceilingz
  modifyMo mi fun m =>
    let m := { m with floorz := tmfloorz, ceilingz := tmceilingz }
    if onfloor then { m with z := m.floorz }
    else if m.z + m.height > m.ceilingz then { m with z := m.ceilingz - m.height }
    else m
  let th ← getMo mi
  return !(th.ceilingz - th.floorz < th.height)

/-- `PIT_ChangeSector`. -/
def pitChangeSector (mi : Nat) : M Bool := do
  if ← thingHeightClip mi then return true
  let th ← getMo mi
  if th.health <= 0 then
    let _ ← setMobjState mi S_GIBS
    let v12 := (← get).res.gameversion > exe_doom_1_2
    modifyMo mi fun m => {
      m with
      flags := if v12 then m.flags &&& ~~~MF_SOLID else m.flags
      height := 0, radius := 0 }
    return true
  if (th.flags &&& MF_DROPPED) != 0 then
    removeMobj mi
    return true
  if (th.flags &&& MF_SHOOTABLE) == 0 then return true
  modify fun gs => { gs with nofit := 1 }
  let gs ← get
  if gs.crushchange != 0 && (gs.leveltime &&& 3) == 0 then
    damageMobj mi (-1) (-1) 10
    let th ← getMo mi
    let mo ← spawnMobj th.x th.y (th.z + th.height / 2) MT_BLOOD
    let rx ← pSubRandomM
    let ry ← pSubRandomM
    modifyMo mo fun m => { m with momx := rx <<< 12, momy := ry <<< 12 }
  return true

/-- `P_ChangeSector(sector, crunch)`; returns the raw `nofit` value. -/
def changeSector (sec : Nat) (crunch : Int32) : M Int32 := do
  modify fun gs => { gs with nofit := 0, crushchange := crunch }
  let bb := (← get).level.sectors[sec]!.blockbox
  let mut x := bb[BOXLEFT]!
  while x <= bb[BOXRIGHT]! do
    let mut y := bb[BOXBOTTOM]!
    while y <= bb[BOXTOP]! do
      let _ ← blockThingsIterator x y pitChangeSector
      y := y + 1
    x := x + 1
  return (← get).nofit

/-- `T_MovePlane(sector, speed, dest, crush, floorOrCeiling, direction)`. -/
def movePlane (sec : Nat) (speed dest : Int32) (crush : Int32) (floorOrCeiling : Nat)
    (direction : Int32) : M Nat := do
  let getH : M Int32 := do
    let s ← getSec sec
    return if floorOrCeiling == 0 then s.floorheight else s.ceilingheight
  let setH (h : Int32) : M Unit := modifySec sec fun s =>
    if floorOrCeiling == 0 then { s with floorheight := h } else { s with ceilingheight := h }
  let cur ← getH
  if floorOrCeiling == 0 then
    if direction == -1 then
      if cur - speed < dest then
        setH dest
        if (← changeSector sec crush) == 1 then
          setH cur
          let _ ← changeSector sec crush
        return res_pastdest
      else
        setH (cur - speed)
        if (← changeSector sec crush) == 1 then
          setH cur
          let _ ← changeSector sec crush
          return res_crushed
    else if direction == 1 then
      if cur + speed > dest then
        setH dest
        if (← changeSector sec crush) == 1 then
          setH cur
          let _ ← changeSector sec crush
        return res_pastdest
      else
        setH (cur + speed)
        if (← changeSector sec crush) == 1 then
          if crush == 1 then return res_crushed
          setH cur
          let _ ← changeSector sec crush
          return res_crushed
  else
    if direction == -1 then
      if cur - speed < dest then
        setH dest
        if (← changeSector sec crush) == 1 then
          setH cur
          let _ ← changeSector sec crush
        return res_pastdest
      else
        setH (cur - speed)
        if (← changeSector sec crush) == 1 then
          if crush == 1 then return res_crushed
          setH cur
          let _ ← changeSector sec crush
          return res_crushed
    else if direction == 1 then
      if cur + speed > dest then
        setH dest
        if (← changeSector sec crush) == 1 then
          setH cur
          let _ ← changeSector sec crush
        return res_pastdest
      else
        setH (cur + speed)
        let _ ← changeSector sec crush
  return res_ok

@[inline] def getDoor (i : Nat) : M VerticalDoor := return (← get).verticalDoors[i]!
@[inline] def modDoor (i : Nat) (f : VerticalDoor → VerticalDoor) : M Unit :=
  modify fun gs => { gs with verticalDoors := gs.verticalDoors.modify i f }

/-- Remove a door/floor thinker and clear its sector's specialdata. -/
def clearSpecial (sec : Nat) : M Unit := modifySec sec fun s => { s with specialdata := -1 }

/-- `T_VerticalDoor`. -/
def verticalDoorThinker (i : Nat) : M Unit := do
  let d ← getDoor i
  let sec := d.sector.toNat
  if d.direction == 0 then
    modDoor i fun d => { d with topcountdown := d.topcountdown - 1 }
    if (← getDoor i).topcountdown == 0 then
      if d.type_ == vld_blazeRaise then
        modDoor i fun d => { d with direction := -1 }
        startSoundSec sec sfx_bdcls
      else if d.type_ == vld_normal then
        modDoor i fun d => { d with direction := -1 }
        startSoundSec sec sfx_dorcls
      else if d.type_ == vld_close30ThenOpen then
        modDoor i fun d => { d with direction := 1 }
        startSoundSec sec sfx_doropn
  else if d.direction == 2 then
    modDoor i fun d => { d with topcountdown := d.topcountdown - 1 }
    if (← getDoor i).topcountdown == 0 then
      if d.type_ == vld_raiseIn5Mins then
        modDoor i fun d => { d with direction := 1, type_ := vld_normal }
        startSoundSec sec sfx_doropn
  else if d.direction == -1 then
    let res ← movePlane sec d.speed (← getSec sec).floorheight 0 1 d.direction
    let d ← getDoor i
    if res == res_pastdest then
      if d.type_ == vld_blazeRaise || d.type_ == vld_blazeClose then
        clearSpecial sec
        modDoor i fun d => { d with func := THF_REMOVED }
        startSoundSec sec sfx_bdcls
      else if d.type_ == vld_normal || d.type_ == vld_close then
        clearSpecial sec
        modDoor i fun d => { d with func := THF_REMOVED }
      else if d.type_ == vld_close30ThenOpen then
        modDoor i fun d => { d with direction := 0, topcountdown := TICRATE * 30 }
    else if res == res_crushed then
      if !(d.type_ == vld_blazeClose || d.type_ == vld_close) then
        modDoor i fun d => { d with direction := 1 }
        startSoundSec sec sfx_doropn
  else if d.direction == 1 then
    let res ← movePlane sec d.speed d.topheight 0 1 d.direction
    let d ← getDoor i
    if res == res_pastdest then
      if d.type_ == vld_blazeRaise || d.type_ == vld_normal then
        modDoor i fun d => { d with direction := 0, topcountdown := d.topwait }
      else if d.type_ == vld_close30ThenOpen || d.type_ == vld_blazeOpen || d.type_ == vld_open then
        clearSpecial sec
        modDoor i fun d => { d with func := THF_REMOVED }

/-- `T_MoveFloor`. -/
def moveFloorThinker (i : Nat) : M Unit := do
  let f := (← get).floors[i]!
  let sec := f.sector.toNat
  let res ← movePlane sec f.speed f.floordestheight f.crush 0 f.direction
  if ((← get).leveltime &&& 7) == 0 then startSoundSec sec sfx_stnmov
  if res == res_pastdest then
    clearSpecial sec
    let f := (← get).floors[i]!
    if (f.direction == 1 && f.type_ == donutRaise) || (f.direction == -1 && f.type_ == lowerAndChange) then
      modifySec sec fun s => { s with special := f.newspecial }
      Doom.Playsim.SpecUtil.setFloorpic sec f.texture
    modifyFloor i fun f => { f with func := THF_REMOVED }
    startSoundSec sec sfx_pstop

/-- `T_MoveCeiling`. -/
def moveCeilingThinker (i : Nat) : M Unit := do
  let c := (← get).ceilings[i]!
  let sec := c.sector.toNat
  if c.direction == 1 then
    let res ← movePlane sec c.speed c.topheight 0 1 c.direction
    if ((← get).leveltime &&& 7) == 0 && c.type_ != silentCrushAndRaise then
      startSoundSec sec sfx_stnmov
    if res == res_pastdest then
      if c.type_ == raiseToHighest then removeActiveCeiling i
      else if c.type_ == silentCrushAndRaise then
        startSoundSec sec sfx_pstop
        modifyCeiling i fun c => { c with direction := -1 }
      else if c.type_ == fastCrushAndRaise || c.type_ == crushAndRaise then
        modifyCeiling i fun c => { c with direction := -1 }
  else if c.direction == -1 then
    let res ← movePlane sec c.speed c.bottomheight c.crush 1 c.direction
    if ((← get).leveltime &&& 7) == 0 && c.type_ != silentCrushAndRaise then
      startSoundSec sec sfx_stnmov
    if res == res_pastdest then
      if c.type_ == silentCrushAndRaise then
        startSoundSec sec sfx_pstop
        modifyCeiling i fun c => { c with speed := CEILSPEED, direction := 1 }
      else if c.type_ == crushAndRaise then
        modifyCeiling i fun c => { c with speed := CEILSPEED, direction := 1 }
      else if c.type_ == fastCrushAndRaise then
        modifyCeiling i fun c => { c with direction := 1 }
      else if c.type_ == lowerAndCrush || c.type_ == lowerToFloor then
        removeActiveCeiling i
    else if res == res_crushed then
      if c.type_ == silentCrushAndRaise || c.type_ == crushAndRaise || c.type_ == lowerAndCrush then
        modifyCeiling i fun c => { c with speed := CEILSPEED / 8 }

/-- `T_PlatRaise`. -/
def platRaiseThinker (i : Nat) : M Unit := do
  let p := (← get).plats[i]!
  let sec := p.sector.toNat
  if p.status == plat_up then
    let res ← movePlane sec p.speed p.high p.crush 0 1
    if p.type_ == raiseAndChange || p.type_ == raiseToNearestAndChange then
      if ((← get).leveltime &&& 7) == 0 then startSoundSec sec sfx_stnmov
    if res == res_crushed && p.crush == 0 then
      modifyPlat i fun p => { p with count := p.wait, status := plat_down }
      startSoundSec sec sfx_pstart
    else if res == res_pastdest then
      modifyPlat i fun p => { p with count := p.wait, status := plat_waiting }
      startSoundSec sec sfx_pstop
      if p.type_ == blazeDWUS || p.type_ == downWaitUpStay || p.type_ == raiseAndChange ||
          p.type_ == raiseToNearestAndChange then
        removeActivePlat i
  else if p.status == plat_down then
    let res ← movePlane sec p.speed p.low 0 0 (-1)
    if res == res_pastdest then
      modifyPlat i fun p => { p with count := p.wait, status := plat_waiting }
      startSoundSec sec sfx_pstop
  else if p.status == plat_waiting then
    modifyPlat i fun p => { p with count := p.count - 1 }
    let p ← (do return (← get).plats[i]!)
    if p.count == 0 then
      let fh := (← getSec sec).floorheight
      modifyPlat i fun p => { p with status := if fh == p.low then plat_up else plat_down }
      startSoundSec sec sfx_pstart

/-- `T_FireFlicker`. -/
def fireFlickerThinker (i : Nat) : M Unit := do
  modify fun gs => { gs with fireFlickers := gs.fireFlickers.modify i fun f => { f with count := f.count - 1 } }
  let f := (← get).fireFlickers[i]!
  if f.count != 0 then return
  let amount := ((← pRandomM) &&& 3) * 16
  let sec := f.sector.toNat
  modifySec sec fun s => { s with lightlevel := if s.lightlevel - amount < f.minlight then f.minlight else f.maxlight - amount }
  modify fun gs => { gs with fireFlickers := gs.fireFlickers.modify i fun f => { f with count := 4 } }

/-- `T_LightFlash`. -/
def lightFlashThinker (i : Nat) : M Unit := do
  modify fun gs => { gs with lightFlashes := gs.lightFlashes.modify i fun f => { f with count := f.count - 1 } }
  let f := (← get).lightFlashes[i]!
  if f.count != 0 then return
  let sec := f.sector.toNat
  if (← getSec sec).lightlevel == f.maxlight then
    modifySec sec fun s => { s with lightlevel := f.minlight }
    let r ← pRandomM
    modify fun gs => { gs with lightFlashes := gs.lightFlashes.modify i fun f => { f with count := (r &&& f.mintime) + 1 } }
  else
    modifySec sec fun s => { s with lightlevel := f.maxlight }
    let r ← pRandomM
    modify fun gs => { gs with lightFlashes := gs.lightFlashes.modify i fun f => { f with count := (r &&& f.maxtime) + 1 } }

/-- `T_StrobeFlash`. -/
def strobeFlashThinker (i : Nat) : M Unit := do
  modify fun gs => { gs with strobes := gs.strobes.modify i fun f => { f with count := f.count - 1 } }
  let f := (← get).strobes[i]!
  if f.count != 0 then return
  let sec := f.sector.toNat
  if (← getSec sec).lightlevel == f.minlight then
    modifySec sec fun s => { s with lightlevel := f.maxlight }
    modify fun gs => { gs with strobes := gs.strobes.modify i fun f => { f with count := f.brighttime } }
  else
    modifySec sec fun s => { s with lightlevel := f.minlight }
    modify fun gs => { gs with strobes := gs.strobes.modify i fun f => { f with count := f.darktime } }

/-- `T_Glow`. -/
def glowThinker (i : Nat) : M Unit := do
  let g := (← get).glows[i]!
  let sec := g.sector.toNat
  if g.direction == -1 then
    modifySec sec fun s => { s with lightlevel := s.lightlevel - GLOWSPEED }
    if (← getSec sec).lightlevel <= g.minlight then
      modifySec sec fun s => { s with lightlevel := s.lightlevel + GLOWSPEED }
      modify fun gs => { gs with glows := gs.glows.modify i fun g => { g with direction := 1 } }
  else if g.direction == 1 then
    modifySec sec fun s => { s with lightlevel := s.lightlevel + GLOWSPEED }
    if (← getSec sec).lightlevel >= g.maxlight then
      modifySec sec fun s => { s with lightlevel := s.lightlevel - GLOWSPEED }
      modify fun gs => { gs with glows := gs.glows.modify i fun g => { g with direction := -1 } }

end Doom.Playsim.Movers
