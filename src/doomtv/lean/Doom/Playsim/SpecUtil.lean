import Doom.Playsim.Defs
import Doom.Playsim.Fixed
import Doom.Playsim.GameState
import Doom.Playsim.Level
import Doom.Playsim.Sound

/-!
# Doom.Playsim.SpecUtil

`p_spec.c` utilities (surrounding-sector searches, tag lookup) and
`p_lights.c` (light thinker spawning and the light linedef actions).
-/

namespace Doom.Playsim.SpecUtil

open Doom.Playsim.Defs
open Doom.Playsim.Fixed
open Doom.Playsim.GameState
open Doom.Playsim.Level
open Doom.Playsim.Sound
open Doom.Playsim.Thinker

/-- `getNextSector(line, sec)`. -/
@[inline] def nextSector (ld : Line) (sec : Nat) : Option Nat :=
  if (ld.flags &&& ML_TWOSIDED) == 0 then none
  else if ld.frontsector == sec.toInt32 then
    if ld.backsector >= 0 then some ld.backsector.toNatClampNeg else none
  else if ld.frontsector >= 0 then some ld.frontsector.toNatClampNeg else none

/-- Fold over neighbouring sectors (`getNextSector` over `sec->lines`). -/
@[inline] def foldNeighbours {α : Type} (gs : GameState) (sec : Nat) (init : α)
    (f : α → SectorRuntime → α) : α := Id.run do
  let s := gs.sectors[sec]!
  let mut acc := init
  for li in s.lines do
    match nextSector gs.level.lines[li.toNat]! sec with
    | some o => acc := f acc gs.sectors[o]!
    | none => pure ()
  return acc

def findLowestFloorSurrounding (gs : GameState) (sec : Nat) : Int32 :=
  foldNeighbours gs sec gs.sectors[sec]!.floorheight fun fl o =>
    if o.floorheight < fl then o.floorheight else fl

def findHighestFloorSurrounding (gs : GameState) (sec : Nat) : Int32 :=
  foldNeighbours gs sec (-500 * FRACUNIT) fun fl o =>
    if o.floorheight > fl then o.floorheight else fl

def findLowestCeilingSurrounding (gs : GameState) (sec : Nat) : Int32 :=
  foldNeighbours gs sec Int32.maxValue fun h o =>
    if o.ceilingheight < h then o.ceilingheight else h

def findHighestCeilingSurrounding (gs : GameState) (sec : Nat) : Int32 :=
  foldNeighbours gs sec 0 fun h o =>
    if o.ceilingheight > h then o.ceilingheight else h

/-- `P_FindNextHighestFloor` with the 22-sector overflow emulation. -/
def findNextHighestFloor (gs : GameState) (sec : Nat) (currentheight : Int32) :
    Except String Int32 := do
  let s := gs.sectors[sec]!
  let mut height := currentheight
  let mut list : Array Int32 := #[]
  for li in s.lines do
    match nextSector gs.level.lines[li.toNat]! sec with
    | none => pure ()
    | some o =>
      let of := gs.sectors[o]!.floorheight
      if of > height then
        if list.size == 21 then height := of
        else if list.size == 22 then
          throw "Sector with more than 22 adjoining sectors. Vanilla will crash here"
        list := list.push of
  if list.size == 0 then return currentheight
  let mut mn := list[0]!
  for h in list do
    if h < mn then mn := h
  return mn

/-- `P_FindSectorFromLineTag(line, start)`; `-1` when none. -/
def findSectorFromLineTag (gs : GameState) (tag : Int32) (start : Int32) : Int32 := Id.run do
  let mut i := (start + 1).toNatClampNeg
  if start + 1 < 0 then i := 0
  while i < gs.sectors.size do
    if gs.sectors[i]!.tag == tag then return i.toInt32
    i := i + 1
  return -1

/-- `P_FindMinSurroundingLight`. -/
def findMinSurroundingLight (gs : GameState) (sec : Nat) (max : Int32) : Int32 :=
  foldNeighbours gs sec max fun m o => if o.lightlevel < m then o.lightlevel else m

/-- Specialdata thinker reference. -/
@[inline] def sdRef (kind idx : Nat) : Int32 := (thRef kind idx).toInt32

def STROBEBRIGHT : Int32 := 5
def FASTDARK : Int32 := 15
def SLOWDARK : Int32 := 35

/-- `P_SpawnFireFlicker`. -/
def spawnFireFlicker (sec : Nat) : M Unit := do
  modifySec sec fun s => { s with special := 0 }
  let idx := (← get).fireFlickers.size
  let tid ← addThinkerRef TK_FIRE idx
  let gs ← get
  let s := gs.sectors[sec]!
  let minlight := findMinSurroundingLight gs sec s.lightlevel + 16
  set {
    gs with
    fireFlickers := gs.fireFlickers.push { traceId := tid, sector := sec.toUInt32, maxlight := s.lightlevel, minlight, count := 4 } }

/-- `P_SpawnLightFlash`. -/
def spawnLightFlash (sec : Nat) : M Unit := do
  modifySec sec fun s => { s with special := 0 }
  let idx := (← get).lightFlashes.size
  let tid ← addThinkerRef TK_FLASH idx
  let gs ← get
  let s := gs.sectors[sec]!
  let minlight := findMinSurroundingLight gs sec s.lightlevel
  let r ← pRandomM
  modify fun gs => {
    gs with
    lightFlashes := gs.lightFlashes.push {
      traceId := tid, sector := sec.toUInt32, maxlight := s.lightlevel, minlight,
      maxtime := 64, mintime := 7, count := (r &&& 64) + 1 } }

/-- `P_SpawnStrobeFlash`. -/
def spawnStrobeFlash (sec : Nat) (fastOrSlow : Int32) (inSync : Bool) : M Unit := do
  let idx := (← get).strobes.size
  let tid ← addThinkerRef TK_STROBE idx
  let gs ← get
  let s := gs.sectors[sec]!
  let mut minlight := findMinSurroundingLight gs sec s.lightlevel
  if minlight == s.lightlevel then minlight := 0
  modifySec sec fun s => { s with special := 0 }
  let mut count : Int32 := 1
  if !inSync then
    let r ← pRandomM
    count := (r &&& 7) + 1
  modify fun gs => {
    gs with
    strobes := gs.strobes.push {
      traceId := tid, sector := sec.toUInt32, darktime := fastOrSlow, brighttime := STROBEBRIGHT,
      maxlight := s.lightlevel, minlight, count } }

/-- `P_SpawnGlowingLight`. -/
def spawnGlowingLight (sec : Nat) : M Unit := do
  let idx := (← get).glows.size
  let tid ← addThinkerRef TK_GLOW idx
  let gs ← get
  let s := gs.sectors[sec]!
  let minlight := findMinSurroundingLight gs sec s.lightlevel
  set {
    gs with
    glows := gs.glows.push { traceId := tid, sector := sec.toUInt32, minlight, maxlight := s.lightlevel, direction := -1 } }
  modifySec sec fun s => { s with special := 0 }

/-- `EV_StartLightStrobing`. -/
def evStartLightStrobing (tag : Int32) : M Unit := do
  let mut secnum : Int32 := -1
  repeat
    secnum := findSectorFromLineTag (← get) tag secnum
    if secnum < 0 then break
    if (← getSec secnum.toNatClampNeg).specialdata != -1 then continue
    spawnStrobeFlash secnum.toNatClampNeg SLOWDARK false

/-- `EV_TurnTagLightsOff`. -/
def evTurnTagLightsOff (tag : Int32) : M Unit := do
  for j in [0:(← get).sectors.size] do
    let gs ← get
    let s := gs.sectors[j]!
    if s.tag == tag then
      let mn := foldNeighbours gs j s.lightlevel fun m o => if o.lightlevel < m then o.lightlevel else m
      modifySec j fun s => { s with lightlevel := mn }

/-- `EV_LightTurnOn` (`bright` is reused across sectors, as in C). -/
def evLightTurnOn (tag : Int32) (bright0 : Int32) : M Unit := do
  let mut bright := bright0
  for i in [0:(← get).sectors.size] do
    let gs ← get
    let s := gs.sectors[i]!
    if s.tag == tag then
      if bright == 0 then
        bright := foldNeighbours gs i bright fun b o => if o.lightlevel > b then o.lightlevel else b
      modifySec i fun s => { s with lightlevel := bright }

/-- `G_ExitLevel`. -/
def gExitLevel : M Unit := modify fun gs => { gs with secretexit := false, gameaction := ga_completed }

/-- `G_SecretExitLevel`. -/
def gSecretExitLevel : M Unit := modify fun gs =>
  let secret := !(gs.res.gamemode == gm_commercial && !gs.res.hasMap31)
  { gs with secretexit := secret, gameaction := ga_completed }

/-- Set a sector's floor flat by number (keeps the renderer's name in sync). -/
def setFloorpic (sec : Nat) (num : Int32) : M Unit := modify fun gs =>
  let name := gs.res.flatNames[num.toNatClampNeg]?.getD ByteArray.empty
  { gs with sectors := gs.sectors.modify sec fun s => { s with floorpicNum := num, floorpic := name } }

end Doom.Playsim.SpecUtil
