import Doom.Playsim.Defs
import Doom.Playsim.GameState
import Doom.Playsim.Sound
import Doom.Playsim.SpecDefs
import Doom.Playsim.SpecUtil

/-!
# Doom.Playsim.Floors

`p_floor.c` creation side (`EV_DoFloor`, `EV_BuildStairs`), `EV_DoDonut`
(`p_spec.c`), `p_ceilng.c` (`EV_DoCeiling`, crush stop, stasis) and
`p_plats.c` (`EV_DoPlat`, `EV_StopPlat`, active list). Thinkers are in
`Movers`.
-/

namespace Doom.Playsim.Floors

open Doom.Playsim.Defs
open Doom.Playsim.Fixed
open Doom.Playsim.GameState
open Doom.Playsim.Level
open Doom.Playsim.Sound
open Doom.Playsim.SpecDefs
open Doom.Playsim.SpecUtil
open Doom.Playsim.Thinker

/-- New floor thinker on `sec` (also sets `specialdata`). -/
def newFloor (sec : Nat) : M Nat := do
  let idx := (← get).floors.size
  let tid ← addThinkerRef TK_FLOOR idx
  modify fun gs => {
    gs with
    floors := gs.floors.push
      { traceId := tid, type_ := 0, crush := 0, sector := sec.toUInt32, direction := 0,
        newspecial := 0, texture := 0, floordestheight := 0, speed := 0 }
    sectors := gs.sectors.modify sec fun s => { s with specialdata := sdRef TK_FLOOR idx } }
  return idx

@[inline] def modifyFloor (i : Nat) (f : FloorMove → FloorMove) : M Unit :=
  modify fun gs => { gs with floors := gs.floors.modify i f }

/-- Texture number of a side's bottom texture (`sides[...].bottomtexture`). -/
@[inline] def sideBottomTex (gs : GameState) (side : Int32) : Int32 :=
  gs.sideBottom[side.toNatClampNeg]!

/-- `EV_DoFloor(line, floortype)`; `lineIdx = none` for the boss "junk" lines. -/
def evDoFloor (tag : Int32) (lineFront : Int32) (floortype : Int32) : M Bool := do
  let mut secnum : Int32 := -1
  let mut rtn := false
  repeat
    secnum := findSectorFromLineTag (← get) tag secnum
    if secnum < 0 then break
    let sec := secnum.toNatClampNeg
    if (← getSec sec).specialdata != -1 then continue
    rtn := true
    let f ← newFloor sec
    modifyFloor f fun fl => { fl with type_ := floortype, crush := 0 }
    let gs ← get
    let s := gs.sectors[sec]!
    if floortype == lowerFloor then
      modifyFloor f fun fl => {
        fl with
        direction := -1, speed := FLOORSPEED,
        floordestheight := findHighestFloorSurrounding gs sec }
    else if floortype == lowerFloorToLowest then
      modifyFloor f fun fl => {
        fl with
        direction := -1, speed := FLOORSPEED,
        floordestheight := findLowestFloorSurrounding gs sec }
    else if floortype == turboLower then
      let mut dest := findHighestFloorSurrounding gs sec
      if gs.res.gameversion <= exe_doom_1_2 || dest != s.floorheight then dest := dest + 8 * FRACUNIT
      modifyFloor f fun fl => { fl with direction := -1, speed := FLOORSPEED * 4, floordestheight := dest }
    else if floortype == raiseFloorCrush || floortype == raiseFloor then
      let mut dest := findLowestCeilingSurrounding gs sec
      if dest > s.ceilingheight then dest := s.ceilingheight
      if floortype == raiseFloorCrush then dest := dest - 8 * FRACUNIT
      modifyFloor f fun fl => {
        fl with
        direction := 1, speed := FLOORSPEED, floordestheight := dest,
        crush := if floortype == raiseFloorCrush then 1 else 0 }
    else if floortype == raiseFloorTurbo || floortype == raiseFloorToNearest then
      match findNextHighestFloor gs sec s.floorheight with
      | .error e => iError e
      | .ok dest =>
        modifyFloor f fun fl => {
          fl with
          direction := 1, floordestheight := dest,
          speed := if floortype == raiseFloorTurbo then FLOORSPEED * 4 else FLOORSPEED }
    else if floortype == raiseFloor24 then
      modifyFloor f fun fl => {
        fl with
        direction := 1, speed := FLOORSPEED,
        floordestheight := s.floorheight + 24 * FRACUNIT }
    else if floortype == raiseFloor512 then
      modifyFloor f fun fl => {
        fl with
        direction := 1, speed := FLOORSPEED,
        floordestheight := s.floorheight + 512 * FRACUNIT }
    else if floortype == raiseFloor24AndChange then
      modifyFloor f fun fl => {
        fl with
        direction := 1, speed := FLOORSPEED,
        floordestheight := s.floorheight + 24 * FRACUNIT }
      let front := gs.sectors[lineFront.toNatClampNeg]!
      setFloorpic sec front.floorpicNum
      modifySec sec fun s => { s with special := front.special }
    else if floortype == raiseToTexture then
      let mut minsize : Int32 := Int32.maxValue
      for li in s.lines do
        let ld := gs.level.lines[li.toNat]!
        if (ld.flags &&& ML_TWOSIDED) != 0 then
          let b0 := sideBottomTex gs ld.sidenum0
          if b0 >= 0 then
            let h := gs.res.textureHeights[b0.toNatClampNeg]!
            if h < minsize then minsize := h
          let b1 := sideBottomTex gs ld.sidenum1
          if b1 >= 0 then
            let h := gs.res.textureHeights[b1.toNatClampNeg]!
            if h < minsize then minsize := h
      modifyFloor f fun fl => {
        fl with
        direction := 1, speed := FLOORSPEED,
        floordestheight := s.floorheight + minsize }
    else if floortype == lowerAndChange then
      let dest := findLowestFloorSurrounding gs sec
      modifyFloor f fun fl => {
        fl with
        direction := -1, speed := FLOORSPEED,
        floordestheight := dest, texture := s.floorpicNum }
      for li in s.lines do
        let ld := gs.level.lines[li.toNat]!
        if (ld.flags &&& ML_TWOSIDED) != 0 then
          let other :=
            if gs.level.sides[ld.sidenum0.toNatClampNeg]!.sector.toNat == sec then
              gs.level.sides[ld.sidenum1.toNatClampNeg]!.sector.toNat
            else gs.level.sides[ld.sidenum0.toNatClampNeg]!.sector.toNat
          let o := gs.sectors[other]!
          if o.floorheight == dest then
            modifyFloor f fun fl => { fl with texture := o.floorpicNum, newspecial := o.special }
            break
  return rtn

/-- `EV_BuildStairs(line, type)`. -/
def evBuildStairs (tag : Int32) (type_ : Int32) : M Bool := do
  let mut secnum : Int32 := -1
  let mut rtn := false
  repeat
    secnum := findSectorFromLineTag (← get) tag secnum
    if secnum < 0 then break
    let mut sec := secnum.toNatClampNeg
    if (← getSec sec).specialdata != -1 then continue
    rtn := true
    let (speed, stairsize) :=
      if type_ == build8 then (FLOORSPEED / 4, 8 * FRACUNIT) else (FLOORSPEED * 4, 16 * FRACUNIT)
    let f ← newFloor sec
    let s ← getSec sec
    let mut height := s.floorheight + stairsize
    modifyFloor f fun fl => {
      fl with
      direction := 1, speed, floordestheight := height,
      type_ := lowerFloor, crush := STAIRS_UNINITIALIZED_CRUSH_FIELD_VALUE }
    let texture := s.floorpicNum
    let mut ok := true
    while ok do
      ok := false
      let gs ← get
      let cur := gs.sectors[sec]!
      for li in cur.lines do
        let ld := gs.level.lines[li.toNat]!
        if (ld.flags &&& ML_TWOSIDED) == 0 then continue
        if ld.frontsector.toNatClampNeg != secnum.toNatClampNeg then continue
        let tsec := ld.backsector.toNatClampNeg
        let t := (← get).sectors[tsec]!
        if t.floorpicNum != texture then continue
        height := height + stairsize
        if t.specialdata != -1 then continue
        sec := tsec
        secnum := tsec.toInt32
        let f2 ← newFloor sec
        modifyFloor f2 fun fl => {
          fl with
          direction := 1, speed, floordestheight := height,
          type_ := lowerFloor, crush := STAIRS_UNINITIALIZED_CRUSH_FIELD_VALUE }
        ok := true
        break
  return rtn

/-- `EV_DoDonut(line)`. -/
def evDoDonut (tag : Int32) : M Bool := do
  let mut secnum : Int32 := -1
  let mut rtn := false
  repeat
    secnum := findSectorFromLineTag (← get) tag secnum
    if secnum < 0 then break
    let s1 := secnum.toNatClampNeg
    let gs ← get
    let sec1 := gs.sectors[s1]!
    if sec1.specialdata != -1 then continue
    rtn := true
    match nextSector gs.level.lines[sec1.lines[0]!.toNat]! s1 with
    | none => break
    | some s2 =>
      let sec2 := gs.sectors[s2]!
      for li in sec2.lines do
        let gs ← get
        let ld := gs.level.lines[li.toNat]!
        let s3 := ld.backsector
        if s3 == s1.toInt32 then continue
        let (s3fh, s3fp) :=
          if s3 < 0 then ((0 : Int32), (0x16 : Int32))
          else (gs.sectors[s3.toNatClampNeg]!.floorheight, gs.sectors[s3.toNatClampNeg]!.floorpicNum)
        let fa ← newFloor s2
        modifyFloor fa fun fl => {
          fl with
          type_ := donutRaise, crush := 0, direction := 1,
          speed := FLOORSPEED / 2, texture := s3fp, newspecial := 0, floordestheight := s3fh }
        let fb ← newFloor s1
        modifyFloor fb fun fl => {
          fl with
          type_ := lowerFloor, crush := 0, direction := -1,
          speed := FLOORSPEED / 2, floordestheight := s3fh }
        break
  return rtn

/-! ## Ceilings -/

def newCeiling (sec : Nat) : M Nat := do
  let idx := (← get).ceilings.size
  let tid ← addThinkerRef TK_CEILING idx
  modify fun gs => {
    gs with
    ceilings := gs.ceilings.push
      { traceId := tid, type_ := 0, sector := sec.toUInt32, bottomheight := 0, topheight := 0,
        speed := 0, crush := 0, direction := 0, tag := 0, olddirection := 0 }
    sectors := gs.sectors.modify sec fun s => { s with specialdata := sdRef TK_CEILING idx } }
  return idx

@[inline] def modifyCeiling (i : Nat) (f : Ceiling → Ceiling) : M Unit :=
  modify fun gs => { gs with ceilings := gs.ceilings.modify i f }

/-- `P_AddActiveCeiling`. -/
def addActiveCeiling (c : Nat) : M Unit := modify fun gs =>
  match gs.activeCeilings.findIdx? (· == 0) with
  | some i => { gs with activeCeilings := gs.activeCeilings.set! i (c + 1) }
  | none => gs

/-- `P_RemoveActiveCeiling`. -/
def removeActiveCeiling (c : Nat) : M Unit := modify fun gs =>
  match gs.activeCeilings.findIdx? (· == c + 1) with
  | some i =>
    let sec := gs.ceilings[c]!.sector.toNat
    { gs with
      activeCeilings := gs.activeCeilings.set! i 0
      sectors := gs.sectors.modify sec fun s => { s with specialdata := -1 }
      ceilings := gs.ceilings.modify c fun cl => { cl with func := THF_REMOVED } }
  | none => gs

/-- `P_ActivateInStasisCeiling`. -/
def activateInStasisCeiling (tag : Int32) : M Unit := modify fun (gs : GameState) => Id.run do
  let acts : Array Nat := gs.activeCeilings
  let mut gs := gs
  for a in acts do
    if a != 0 then
      let c := gs.ceilings[a - 1]!
      if c.tag == tag && c.direction == 0 then
        gs := {
          gs with
          ceilings := gs.ceilings.modify (a - 1) fun c => { c with direction := c.olddirection, func := THF_MOVECEILING } }
  return gs

/-- `EV_CeilingCrushStop`. -/
def evCeilingCrushStop (tag : Int32) : M Bool := do
  let mut rtn := false
  for a in (← get).activeCeilings do
    if a != 0 then
      let c := (← get).ceilings[a - 1]!
      if c.tag == tag && c.direction != 0 then
        modifyCeiling (a - 1) fun c => { c with olddirection := c.direction, func := THF_NULL,
                                                direction := 0 }
        rtn := true
  return rtn

/-- `EV_DoCeiling(line, type)`. -/
def evDoCeiling (tag : Int32) (type_ : Int32) : M Bool := do
  if type_ == fastCrushAndRaise || type_ == silentCrushAndRaise || type_ == crushAndRaise then
    activateInStasisCeiling tag
  let mut secnum : Int32 := -1
  let mut rtn := false
  repeat
    secnum := findSectorFromLineTag (← get) tag secnum
    if secnum < 0 then break
    let sec := secnum.toNatClampNeg
    if (← getSec sec).specialdata != -1 then continue
    rtn := true
    let c ← newCeiling sec
    let gs ← get
    let s := gs.sectors[sec]!
    if type_ == fastCrushAndRaise then
      modifyCeiling c fun cl => {
        cl with
        crush := 1, topheight := s.ceilingheight,
        bottomheight := s.floorheight + 8 * FRACUNIT, direction := -1, speed := CEILSPEED * 2 }
    else if type_ == silentCrushAndRaise || type_ == crushAndRaise || type_ == lowerAndCrush ||
        type_ == lowerToFloor then
      if type_ == silentCrushAndRaise || type_ == crushAndRaise then
        modifyCeiling c fun cl => { cl with crush := 1, topheight := s.ceilingheight }
      let bottom := if type_ != lowerToFloor then s.floorheight + 8 * FRACUNIT else s.floorheight
      modifyCeiling c fun cl => { cl with bottomheight := bottom, direction := -1, speed := CEILSPEED }
    else if type_ == raiseToHighest then
      modifyCeiling c fun cl => {
        cl with
        topheight := findHighestCeilingSurrounding gs sec,
        direction := 1, speed := CEILSPEED }
    modifyCeiling c fun cl => { cl with tag := s.tag, type_ }
    addActiveCeiling c
  return rtn

/-! ## Plats -/

def newPlat (sec : Nat) : M Nat := do
  let idx := (← get).plats.size
  let tid ← addThinkerRef TK_PLAT idx
  modify fun gs => {
    gs with
    plats := gs.plats.push
      { traceId := tid, sector := sec.toUInt32, speed := 0, low := 0, high := 0, wait := 0,
        count := 0, status := 0, oldstatus := 0, crush := 0, tag := 0, type_ := 0 }
    sectors := gs.sectors.modify sec fun s => { s with specialdata := sdRef TK_PLAT idx } }
  return idx

@[inline] def modifyPlat (i : Nat) (f : Plat → Plat) : M Unit :=
  modify fun gs => { gs with plats := gs.plats.modify i f }

/-- `P_AddActivePlat`. -/
def addActivePlat (p : Nat) : M Unit := do
  match (← get).activePlats.findIdx? (· == 0) with
  | some i => modify fun gs => { gs with activePlats := gs.activePlats.set! i (p + 1) }
  | none => iError "P_AddActivePlat: no more plats!"

/-- `P_RemoveActivePlat`. -/
def removeActivePlat (p : Nat) : M Unit := do
  match (← get).activePlats.findIdx? (· == p + 1) with
  | some i => modify fun gs =>
    let sec := gs.plats[p]!.sector.toNat
    { gs with
      activePlats := gs.activePlats.set! i 0
      sectors := gs.sectors.modify sec fun s => { s with specialdata := -1 }
      plats := gs.plats.modify p fun pl => { pl with func := THF_REMOVED } }
  | none => iError "P_RemoveActivePlat: can't find plat!"

/-- `P_ActivateInStasis`. -/
def activateInStasis (tag : Int32) : M Unit := modify fun (gs : GameState) => Id.run do
  let acts : Array Nat := gs.activePlats
  let mut gs := gs
  for a in acts do
    if a != 0 then
      let p := gs.plats[a - 1]!
      if p.tag == tag && p.status == plat_in_stasis then
        gs := {
          gs with
          plats := gs.plats.modify (a - 1) fun p => { p with status := p.oldstatus, func := THF_PLATRAISE } }
  return gs

/-- `EV_StopPlat`. -/
def evStopPlat (tag : Int32) : M Unit := modify fun (gs : GameState) => Id.run do
  let acts : Array Nat := gs.activePlats
  let mut gs := gs
  for a in acts do
    if a != 0 then
      let p := gs.plats[a - 1]!
      if p.status != plat_in_stasis && p.tag == tag then
        gs := {
          gs with
          plats := gs.plats.modify (a - 1) fun p => { p with oldstatus := p.status, status := plat_in_stasis, func := THF_NULL } }
  return gs

/-- `EV_DoPlat(line, type, amount)`. `frontSide` is `line->sidenum[0]`. -/
def evDoPlat (tag : Int32) (frontSide : Int32) (type_ : Int32) (amount : Int32) : M Bool := do
  if type_ == perpetualRaise then activateInStasis tag
  let mut secnum : Int32 := -1
  let mut rtn := false
  repeat
    secnum := findSectorFromLineTag (← get) tag secnum
    if secnum < 0 then break
    let sec := secnum.toNatClampNeg
    if (← getSec sec).specialdata != -1 then continue
    rtn := true
    let p ← newPlat sec
    modifyPlat p fun pl => { pl with type_, crush := 0, tag }
    let gs ← get
    let s := gs.sectors[sec]!
    if type_ == raiseToNearestAndChange then
      let frontSec := gs.level.sides[frontSide.toNatClampNeg]!.sector.toNat
      setFloorpic sec gs.sectors[frontSec]!.floorpicNum
      match findNextHighestFloor gs sec s.floorheight with
      | .error e => iError e
      | .ok high =>
        modifyPlat p fun pl => { pl with speed := PLATSPEED / 2, high, wait := 0, status := plat_up }
      modifySec sec fun s => { s with special := 0 }
      startSoundSec sec sfx_stnmov
    else if type_ == raiseAndChange then
      let frontSec := gs.level.sides[frontSide.toNatClampNeg]!.sector.toNat
      setFloorpic sec gs.sectors[frontSec]!.floorpicNum
      modifyPlat p fun pl => {
        pl with
        speed := PLATSPEED / 2, high := s.floorheight + amount * FRACUNIT,
        wait := 0, status := plat_up }
      startSoundSec sec sfx_stnmov
    else if type_ == downWaitUpStay || type_ == blazeDWUS then
      let mut low := findLowestFloorSurrounding gs sec
      if low > s.floorheight then low := s.floorheight
      modifyPlat p fun pl => {
        pl with
        speed := if type_ == blazeDWUS then PLATSPEED * 8 else PLATSPEED * 4
        low, high := s.floorheight, wait := TICRATE * PLATWAIT, status := plat_down }
      startSoundSec sec sfx_pstart
    else if type_ == perpetualRaise then
      let mut low := findLowestFloorSurrounding gs sec
      if low > s.floorheight then low := s.floorheight
      let mut high := findHighestFloorSurrounding gs sec
      if high < s.floorheight then high := s.floorheight
      let r ← pRandomM
      modifyPlat p fun pl => {
        pl with
        speed := PLATSPEED, low, high, wait := TICRATE * PLATWAIT,
        status := r &&& 1 }
      startSoundSec sec sfx_pstart
    addActivePlat p
  return rtn

end Doom.Playsim.Floors
