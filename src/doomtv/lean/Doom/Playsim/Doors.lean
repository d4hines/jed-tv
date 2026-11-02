import Doom.Playsim.Defs
import Doom.Playsim.GameState
import Doom.Playsim.Player
import Doom.Playsim.Sound
import Doom.Playsim.SpecDefs
import Doom.Playsim.SpecUtil

/-!
# Doom.Playsim.Doors

`p_doors.c` door creation: `EV_DoDoor`, `EV_DoLockedDoor`, `EV_VerticalDoor`
and the sector-special doors. `T_VerticalDoor` is in `Movers`.
-/

namespace Doom.Playsim.Doors

open Doom.Playsim.Defs
open Doom.Playsim.GameState
open Doom.Playsim.Player
open Doom.Playsim.Sound
open Doom.Playsim.SpecDefs
open Doom.Playsim.SpecUtil
open Doom.Playsim.Thinker
open Doom.Playsim.Fixed

/-- Allocate a door thinker on `sec` (`Z_Malloc` + `P_AddThinker` + specialdata). -/
def newDoor (sec : Nat) : M Nat := do
  let idx := (← get).verticalDoors.size
  let tid ← addThinkerRef TK_DOOR idx
  modify fun gs => {
    gs with
    verticalDoors := gs.verticalDoors.push
      { traceId := tid, type_ := 0, sector := sec.toUInt32, topheight := 0, speed := 0,
        direction := 0, topwait := 0, topcountdown := 0 }
    sectors := gs.sectors.modify sec fun s => { s with specialdata := sdRef TK_DOOR idx } }
  return idx

@[inline] def modifyDoor (i : Nat) (f : VerticalDoor → VerticalDoor) : M Unit :=
  modify fun gs => { gs with verticalDoors := gs.verticalDoors.modify i f }

/-- `EV_DoDoor(line, type)`. -/
def evDoDoor (tag : Int32) (type_ : Int32) : M Bool := do
  let mut secnum : Int32 := -1
  let mut rtn := false
  repeat
    secnum := findSectorFromLineTag (← get) tag secnum
    if secnum < 0 then break
    let sec := secnum.toNatClampNeg
    if (← getSec sec).specialdata != -1 then continue
    rtn := true
    let d ← newDoor sec
    modifyDoor d fun dr => { dr with type_, topwait := VDOORWAIT, speed := VDOORSPEED }
    let gs ← get
    let s := gs.sectors[sec]!
    let lowc := findLowestCeilingSurrounding gs sec - 4 * FRACUNIT
    if type_ == vld_blazeClose then
      modifyDoor d fun dr => { dr with topheight := lowc, direction := -1, speed := VDOORSPEED * 4 }
      startSoundSec sec sfx_bdcls
    else if type_ == vld_close then
      modifyDoor d fun dr => { dr with topheight := lowc, direction := -1 }
      startSoundSec sec sfx_dorcls
    else if type_ == vld_close30ThenOpen then
      modifyDoor d fun dr => { dr with topheight := s.ceilingheight, direction := -1 }
      startSoundSec sec sfx_dorcls
    else if type_ == vld_blazeRaise || type_ == vld_blazeOpen then
      modifyDoor d fun dr => { dr with direction := 1, topheight := lowc, speed := VDOORSPEED * 4 }
      if lowc != s.ceilingheight then startSoundSec sec sfx_bdopn
    else if type_ == vld_normal || type_ == vld_open then
      modifyDoor d fun dr => { dr with direction := 1, topheight := lowc }
      if lowc != s.ceilingheight then startSoundSec sec sfx_doropn
  return rtn

private def keyMsg (pi : Nat) (s : String) : M Unit := do
  let m := dehString (← get) s
  modifyPl pi fun p => { p with message := some m }
  startSoundNull sfx_oof

/-- `EV_DoLockedDoor(line, type, thing)`. -/
def evDoLockedDoor (ld : Level.Line) (type_ : Int32) (thingIdx : Nat) : M Bool := do
  let th ← getMo thingIdx
  if th.player < 0 then return false
  let pi := th.player.toNatClampNeg
  let p ← getPl pi
  let has (a b : Nat) := p.cards[a]! || p.cards[b]!
  let sp := ld.special
  if sp == 99 || sp == 133 then
    if !has it_bluecard it_blueskull then
      keyMsg pi "You need a blue key to activate this object"; return false
  else if sp == 134 || sp == 135 then
    if !has it_redcard it_redskull then
      keyMsg pi "You need a red key to activate this object"; return false
  else if sp == 136 || sp == 137 then
    if !has it_yellowcard it_yellowskull then
      keyMsg pi "You need a yellow key to activate this object"; return false
  evDoDoor ld.tag type_

/-- Read the `int` at `vldoor_t.direction`'s offset (56 on the 64-bit oracle)
of whatever thinker `specialdata` points at. -/
def readDoorDirection (gs : GameState) (sd : Int32) : Int32 :=
  let r := sd.toNatClampNeg
  let i := thIdx r
  let k := thKind r
  if k == TK_DOOR then gs.verticalDoors[i]!.direction
  else if k == TK_PLAT then gs.plats[i]!.count
  else if k == TK_CEILING then gs.ceilings[i]!.speed
  else if k == TK_FLOOR then
    let f := gs.floors[i]!
    ((f.texture.toUInt32 &&& (0xffff : UInt32)) ||| (f.texPad.toUInt32 <<< (16 : UInt32))).toInt32
  else 0

/-- Write -1 at that offset (the "not a door" fallback). -/
def writeDoorDirectionM1 (sd : Int32) : M Unit := modify fun gs =>
  let r := sd.toNatClampNeg
  let i := thIdx r
  let k := thKind r
  if k == TK_DOOR then { gs with verticalDoors := gs.verticalDoors.modify i fun d => { d with direction := -1 } }
  else if k == TK_PLAT then { gs with plats := gs.plats.modify i fun p => { p with count := -1 } }
  else if k == TK_CEILING then { gs with ceilings := gs.ceilings.modify i fun c => { c with speed := -1 } }
  else if k == TK_FLOOR then
    { gs with floors := gs.floors.modify i fun f => { f with texture := -1, texPad := -1 } }
  else gs

/-- Write `direction` (any value) at the door offset; used for "go back up". -/
def writeDoorDirection (sd : Int32) (v : Int32) : M Unit := modify fun gs =>
  let r := sd.toNatClampNeg
  let i := thIdx r
  let k := thKind r
  if k == TK_DOOR then { gs with verticalDoors := gs.verticalDoors.modify i fun d => { d with direction := v } }
  else if k == TK_PLAT then { gs with plats := gs.plats.modify i fun p => { p with count := v } }
  else if k == TK_CEILING then { gs with ceilings := gs.ceilings.modify i fun c => { c with speed := v } }
  else if k == TK_FLOOR then
    { gs with
        floors := gs.floors.modify i fun f => {
                 f with
                 texture := (v.toUInt32 &&& (0xffff : UInt32)).toUInt16.toInt16.toInt32,
                 texPad := (v.toUInt32 >>> (16 : UInt32)).toUInt16.toInt16.toInt32 } }
  else gs

/-- `EV_VerticalDoor(line, thing)`. -/
def evVerticalDoor (lineIdx : Nat) (thingIdx : Nat) : M Unit := do
  let ld ← getLine lineIdx
  let th ← getMo thingIdx
  let sp := ld.special
  if sp == 26 || sp == 32 || sp == 27 || sp == 34 || sp == 28 || sp == 33 then
    if th.player < 0 then return
    let pi := th.player.toNatClampNeg
    let p ← getPl pi
    let has (a b : Nat) := p.cards[a]! || p.cards[b]!
    if sp == 26 || sp == 32 then
      if !has it_bluecard it_blueskull then keyMsg pi "You need a blue key to open this door"; return
    else if sp == 27 || sp == 34 then
      if !has it_yellowcard it_yellowskull then keyMsg pi "You need a yellow key to open this door"; return
    else
      if !has it_redcard it_redskull then keyMsg pi "You need a red key to open this door"; return
  if ld.sidenum1 == -1 then
    iError "EV_VerticalDoor: DR special type on 1-sided linedef"
    return
  let gs ← get
  let sec := gs.level.sides[ld.sidenum1.toNatClampNeg]!.sector.toNat
  let sd := gs.sectors[sec]!.specialdata
  if sd != -1 then
    if sp == 1 || sp == 26 || sp == 27 || sp == 28 || sp == 117 then
      if readDoorDirection gs sd == -1 then
        writeDoorDirection sd 1
      else
        if th.player < 0 then return
        let r := sd.toNatClampNeg
        let k := thKind r
        let isDoor := k == TK_DOOR && gs.verticalDoors[thIdx r]!.func == THF_VERTICALDOOR
        let isPlat := k == TK_PLAT && gs.plats[thIdx r]!.func == THF_PLATRAISE
        if isDoor then writeDoorDirection sd (-1)
        else if isPlat then
          modify fun gs => { gs with plats := gs.plats.modify (thIdx r) fun p => { p with wait := -1 } }
        else writeDoorDirectionM1 sd
      return
  if sp == 117 || sp == 118 then startSoundSec sec sfx_bdopn
  else startSoundSec sec sfx_doropn
  let d ← newDoor sec
  modifyDoor d fun dr => { dr with direction := 1, speed := VDOORSPEED, topwait := VDOORWAIT }
  if sp == 1 || sp == 26 || sp == 27 || sp == 28 then
    modifyDoor d fun dr => { dr with type_ := vld_normal }
  else if sp == 31 || sp == 32 || sp == 33 || sp == 34 then
    modifyDoor d fun dr => { dr with type_ := vld_open }
    modify fun gs => { gs with level := {
      gs.level with
      lines := gs.level.lines.modify lineIdx fun l => { l with special := 0 } } }
  else if sp == 117 then
    modifyDoor d fun dr => { dr with type_ := vld_blazeRaise, speed := VDOORSPEED * 4 }
  else if sp == 118 then
    modifyDoor d fun dr => { dr with type_ := vld_blazeOpen, speed := VDOORSPEED * 4 }
    modify fun gs => { gs with level := {
      gs.level with
      lines := gs.level.lines.modify lineIdx fun l => { l with special := 0 } } }
  let top := findLowestCeilingSurrounding (← get) sec - 4 * FRACUNIT
  modifyDoor d fun dr => { dr with topheight := top }

/-- `P_SpawnDoorCloseIn30`. -/
def spawnDoorCloseIn30 (sec : Nat) : M Unit := do
  let d ← newDoor sec
  modifySec sec fun s => { s with special := 0 }
  modifyDoor d fun dr => { dr with direction := 0, type_ := vld_normal, speed := VDOORSPEED,
                                   topcountdown := 30 * TICRATE }

/-- `P_SpawnDoorRaiseIn5Mins`. -/
def spawnDoorRaiseIn5Mins (sec : Nat) : M Unit := do
  let d ← newDoor sec
  modifySec sec fun s => { s with special := 0 }
  let top := findLowestCeilingSurrounding (← get) sec - 4 * FRACUNIT
  modifyDoor d fun dr => {
    dr with
    direction := 2, type_ := vld_raiseIn5Mins, speed := VDOORSPEED,
    topheight := top, topwait := VDOORWAIT, topcountdown := 5 * 60 * TICRATE }

end Doom.Playsim.Doors
