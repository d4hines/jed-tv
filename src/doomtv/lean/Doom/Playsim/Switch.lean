import Doom.Playsim.Defs
import Doom.Playsim.Doors
import Doom.Playsim.Floors
import Doom.Playsim.GameState
import Doom.Playsim.Sound
import Doom.Playsim.SpecDefs
import Doom.Playsim.SpecUtil

/-!
# Doom.Playsim.Switch

`p_switch.c`: buttons, `P_ChangeSwitchTexture`, `P_UseSpecialLine`; and
`P_ShootSpecialLine` (`p_spec.c`).
-/

namespace Doom.Playsim.Switch

open Doom.Playsim.Defs
open Doom.Playsim.Doors
open Doom.Playsim.Floors
open Doom.Playsim.GameState
open Doom.Playsim.Level
open Doom.Playsim.Sound
open Doom.Playsim.SpecDefs
open Doom.Playsim.SpecUtil

/-- Set one of a side's textures by number (and the renderer's name). -/
def setSideTexture (side : Nat) (where_ : Int32) (tex : Int32) : M Unit := modify fun gs =>
  let name := gs.res.textureNames[tex.toNatClampNeg]?.getD ByteArray.empty
  if where_ == bw_top then
    { gs with sideTop := gs.sideTop.set! side tex
              level := { gs.level with sides := gs.level.sides.modify side fun s => { s with toptexture := name } } }
  else if where_ == bw_middle then
    { gs with sideMid := gs.sideMid.set! side tex
              level := { gs.level with sides := gs.level.sides.modify side fun s => { s with midtexture := name } } }
  else
    { gs with sideBottom := gs.sideBottom.set! side tex
              level := { gs.level with sides := gs.level.sides.modify side fun s => { s with bottomtexture := name } } }

/-- `P_StartButton`. -/
def startButton (lineIdx : Nat) (w : Int32) (texture : Int32) (time : Int32) : M Unit := do
  let gs ← get
  for b in gs.buttons do
    if b.btimer != 0 && b.line == lineIdx.toInt32 then return
  match gs.buttons.findIdx? (·.btimer == 0) with
  | some i =>
    let front := gs.level.lines[lineIdx]!.frontsector
    set {
      gs with
      buttons := gs.buttons.set! i { line := lineIdx.toInt32, where_ := w, btexture := texture, btimer := time, soundorg := front } }
  | none => iError "P_StartButton: no button slots left!"

def setLineSpecial (lineIdx : Nat) (v : Int32) : M Unit := modify fun gs => { gs with level := { gs.level with lines := gs.level.lines.modify lineIdx fun l => { l with special := v } } }

/-- `P_ChangeSwitchTexture(line, useAgain)`. -/
def changeSwitchTexture (lineIdx : Nat) (useAgain : Bool) : M Unit := do
  if !useAgain then setLineSpecial lineIdx 0
  let gs ← get
  let ld := gs.level.lines[lineIdx]!
  let side := ld.sidenum0.toNatClampNeg
  let texTop := gs.sideTop[side]!
  let texMid := gs.sideMid[side]!
  let texBot := gs.sideBottom[side]!
  let sound := if ld.special == 11 then sfx_swtchx else sfx_swtchn
  let org : SndOrg := if gs.buttons[0]!.soundorg >= 0 then .sector gs.buttons[0]!.soundorg.toNatClampNeg
                      else .none
  let sl := gs.res.switchlist
  for i in [0:sl.size] do
    let w := sl[i]!
    let other := sl[i ^^^ 1]!
    if w == texTop then
      startSound org sound
      setSideTexture side bw_top other
      if useAgain then startButton lineIdx bw_top w BUTTONTIME
      return
    else if w == texMid then
      startSound org sound
      setSideTexture side bw_middle other
      if useAgain then startButton lineIdx bw_middle w BUTTONTIME
      return
    else if w == texBot then
      startSound org sound
      setSideTexture side bw_bottom other
      if useAgain then startButton lineIdx bw_bottom w BUTTONTIME
      return

/-- `P_UseSpecialLine(thing, line, side)`. -/
def useSpecialLine (thingIdx : Nat) (lineIdx : Nat) (side : Nat) : M Bool := do
  let ld ← getLine lineIdx
  let sp := ld.special
  if side != 0 then
    if sp != 124 then return false
  let th ← getMo thingIdx
  if th.player < 0 then
    if (ld.flags &&& ML_SECRET) != 0 then return false
    if !(sp == 1 || sp == 32 || sp == 33 || sp == 34) then return false
  let tag := ld.tag
  let front := ld.sidenum0
  let sw (ok : Bool) (useAgain : Bool) : M Unit := do
    if ok then changeSwitchTexture lineIdx useAgain
  if sp == 1 || sp == 26 || sp == 27 || sp == 28 || sp == 31 || sp == 32 || sp == 33 ||
      sp == 34 || sp == 117 || sp == 118 then
    evVerticalDoor lineIdx thingIdx
  else if sp == 7 then sw (← evBuildStairs tag build8) false
  else if sp == 9 then sw (← evDoDonut tag) false
  else if sp == 11 then
    changeSwitchTexture lineIdx false
    gExitLevel
  else if sp == 14 then sw (← evDoPlat tag front raiseAndChange 32) false
  else if sp == 15 then sw (← evDoPlat tag front raiseAndChange 24) false
  else if sp == 18 then sw (← evDoFloor tag ld.frontsector raiseFloorToNearest) false
  else if sp == 20 then sw (← evDoPlat tag front raiseToNearestAndChange 0) false
  else if sp == 21 then sw (← evDoPlat tag front downWaitUpStay 0) false
  else if sp == 23 then sw (← evDoFloor tag ld.frontsector lowerFloorToLowest) false
  else if sp == 29 then sw (← evDoDoor tag vld_normal) false
  else if sp == 41 then sw (← evDoCeiling tag lowerToFloor) false
  else if sp == 71 then sw (← evDoFloor tag ld.frontsector turboLower) false
  else if sp == 49 then sw (← evDoCeiling tag crushAndRaise) false
  else if sp == 50 then sw (← evDoDoor tag vld_close) false
  else if sp == 51 then
    changeSwitchTexture lineIdx false
    gSecretExitLevel
  else if sp == 55 then sw (← evDoFloor tag ld.frontsector raiseFloorCrush) false
  else if sp == 101 then sw (← evDoFloor tag ld.frontsector raiseFloor) false
  else if sp == 102 then sw (← evDoFloor tag ld.frontsector lowerFloor) false
  else if sp == 103 then sw (← evDoDoor tag vld_open) false
  else if sp == 111 then sw (← evDoDoor tag vld_blazeRaise) false
  else if sp == 112 then sw (← evDoDoor tag vld_blazeOpen) false
  else if sp == 113 then sw (← evDoDoor tag vld_blazeClose) false
  else if sp == 122 then sw (← evDoPlat tag front blazeDWUS 0) false
  else if sp == 127 then sw (← evBuildStairs tag turbo16) false
  else if sp == 131 then sw (← evDoFloor tag ld.frontsector raiseFloorTurbo) false
  else if sp == 133 || sp == 135 || sp == 137 then
    sw (← evDoLockedDoor ld vld_blazeOpen thingIdx) false
  else if sp == 140 then sw (← evDoFloor tag ld.frontsector raiseFloor512) false
  -- buttons
  else if sp == 42 then sw (← evDoDoor tag vld_close) true
  else if sp == 43 then sw (← evDoCeiling tag lowerToFloor) true
  else if sp == 45 then sw (← evDoFloor tag ld.frontsector lowerFloor) true
  else if sp == 60 then sw (← evDoFloor tag ld.frontsector lowerFloorToLowest) true
  else if sp == 61 then sw (← evDoDoor tag vld_open) true
  else if sp == 62 then sw (← evDoPlat tag front downWaitUpStay 1) true
  else if sp == 63 then sw (← evDoDoor tag vld_normal) true
  else if sp == 64 then sw (← evDoFloor tag ld.frontsector raiseFloor) true
  else if sp == 66 then sw (← evDoPlat tag front raiseAndChange 24) true
  else if sp == 67 then sw (← evDoPlat tag front raiseAndChange 32) true
  else if sp == 65 then sw (← evDoFloor tag ld.frontsector raiseFloorCrush) true
  else if sp == 68 then sw (← evDoPlat tag front raiseToNearestAndChange 0) true
  else if sp == 69 then sw (← evDoFloor tag ld.frontsector raiseFloorToNearest) true
  else if sp == 70 then sw (← evDoFloor tag ld.frontsector turboLower) true
  else if sp == 114 then sw (← evDoDoor tag vld_blazeRaise) true
  else if sp == 115 then sw (← evDoDoor tag vld_blazeOpen) true
  else if sp == 116 then sw (← evDoDoor tag vld_blazeClose) true
  else if sp == 123 then sw (← evDoPlat tag front blazeDWUS 0) true
  else if sp == 132 then sw (← evDoFloor tag ld.frontsector raiseFloorTurbo) true
  else if sp == 99 || sp == 134 || sp == 136 then
    sw (← evDoLockedDoor ld vld_blazeOpen thingIdx) true
  else if sp == 138 then
    evLightTurnOn tag 255
    changeSwitchTexture lineIdx true
  else if sp == 139 then
    evLightTurnOn tag 35
    changeSwitchTexture lineIdx true
  return true

/-- `P_ShootSpecialLine(thing, line)`. -/
def shootSpecialLine (thingIdx : Nat) (lineIdx : Nat) : M Unit := do
  let ld ← getLine lineIdx
  let th ← getMo thingIdx
  if th.player < 0 then
    if ld.special != 46 then return
  if ld.special == 24 then
    let _ ← evDoFloor ld.tag ld.frontsector raiseFloor
    changeSwitchTexture lineIdx false
  else if ld.special == 46 then
    let _ ← evDoDoor ld.tag vld_open
    changeSwitchTexture lineIdx true
  else if ld.special == 47 then
    let _ ← evDoPlat ld.tag ld.sidenum0 raiseToNearestAndChange 0
    changeSwitchTexture lineIdx false

end Doom.Playsim.Switch
