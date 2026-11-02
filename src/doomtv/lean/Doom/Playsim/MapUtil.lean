import Doom.Playsim.Fixed
import Doom.Playsim.Flags
import Doom.Playsim.GameState
import Doom.Playsim.Level
import Doom.Playsim.Mobj

/-!
# Doom.Playsim.MapUtil

`p_maputl.c`: point/box line side, `P_LineOpening` (writes the `opentop` /
`openbottom` / `openrange` / `lowfloor` globals), thing position link and
unlink, blockmap iterators, and `P_PathTraverse` with the intercepts
overrun emulation.
-/

namespace Doom.Playsim.MapUtil

open Doom.Playsim.Fixed
open Doom.Playsim.Flags
open Doom.Playsim.GameState
open Doom.Playsim.Level
open Doom.Playsim.Mobj

/-- `P_AproxDistance`. -/
def aproxDistance (dx0 dy0 : Int32) : Int32 :=
  let dx := wabs dx0
  let dy := wabs dy0
  if dx < dy then dx + dy - (dx >>> 1) else dx + dy - (dy >>> 1)

/-- `P_PointOnLineSide` (raw line fields): 0 front, 1 back. -/
@[inline] def pointOnLineSideRaw (x y v1x v1y ldx ldy : Int32) : Nat :=
  if ldx == 0 then
    if x <= v1x then (if ldy > 0 then 1 else 0)
    else (if ldy < 0 then 1 else 0)
  else if ldy == 0 then
    if y <= v1y then (if ldx < 0 then 1 else 0)
    else (if ldx > 0 then 1 else 0)
  else
    let left := fixedMul (ldy >>> 16) (x - v1x)
    let right := fixedMul (y - v1y) (ldx >>> 16)
    if right < left then 0 else 1

/-- `P_PointOnLineSide`. -/
@[inline] def pointOnLineSide (gs : GameState) (x y : Int32) (ld : Line) : Nat :=
  let v1 := gs.level.vertexes[ld.v1.toNat]!
  pointOnLineSideRaw x y v1.x v1.y ld.dx ld.dy

/-- `P_PointOnLineSide` on cached geometry. -/
@[inline] def pointOnLineSideG (x y : Int32) (g : LineGeo) : Nat :=
  pointOnLineSideRaw x y g.v1x g.v1y g.dx g.dy

/-- `P_BoxOnLineSide` (tmbbox global) on cached geometry. -/
@[inline] def boxOnLineSideG (gs : GameState) (g : LineGeo) : Int32 :=
  let top := gs.tmbboxTop
  let bottom := gs.tmbboxBottom
  let left := gs.tmbboxLeft
  let right := gs.tmbboxRight
  let (p1, p2) : Nat × Nat :=
    if g.slopetype == ST_HORIZONTAL then
      let p1 := if top > g.v1y then 1 else 0
      let p2 := if bottom > g.v1y then 1 else 0
      if g.dx < 0 then (p1 ^^^ 1, p2 ^^^ 1) else (p1, p2)
    else if g.slopetype == ST_VERTICAL then
      let p1 := if right < g.v1x then 1 else 0
      let p2 := if left < g.v1x then 1 else 0
      if g.dy < 0 then (p1 ^^^ 1, p2 ^^^ 1) else (p1, p2)
    else if g.slopetype == ST_POSITIVE then
      (pointOnLineSideRaw left top g.v1x g.v1y g.dx g.dy,
       pointOnLineSideRaw right bottom g.v1x g.v1y g.dx g.dy)
    else
      (pointOnLineSideRaw right top g.v1x g.v1y g.dx g.dy,
       pointOnLineSideRaw left bottom g.v1x g.v1y g.dx g.dy)
  if p1 == p2 then p1.toInt32 else -1

/-- `P_LineOpening` on cached geometry. -/
def lineOpeningG (g : LineGeo) : M Unit := modify fun gs =>
  if g.sidenum1 == -1 then { gs with openrange := 0 }
  else
    let front := gs.sectors[g.front.toNatClampNeg]!
    let back := gs.sectors[g.back.toNatClampNeg]!
    let opentop := if front.ceilingheight < back.ceilingheight then front.ceilingheight
                   else back.ceilingheight
    let (openbottom, lowfloor) :=
      if front.floorheight > back.floorheight then (front.floorheight, back.floorheight)
      else (back.floorheight, front.floorheight)
    { gs with opentop, openbottom, lowfloor, openrange := opentop - openbottom }

/-- `P_BoxOnLineSide` on the `tmbbox` global: 0/1, or -1 if crossing. -/
def boxOnLineSide (gs : GameState) (ld : Line) : Int32 :=
  let v1 := gs.level.vertexes[ld.v1.toNat]!
  let top := gs.tmbboxTop
  let bottom := gs.tmbboxBottom
  let left := gs.tmbboxLeft
  let right := gs.tmbboxRight
  let (p1, p2) : Nat × Nat :=
    if ld.slopetype == ST_HORIZONTAL then
      let p1 := if top > v1.y then 1 else 0
      let p2 := if bottom > v1.y then 1 else 0
      if ld.dx < 0 then (p1 ^^^ 1, p2 ^^^ 1) else (p1, p2)
    else if ld.slopetype == ST_VERTICAL then
      let p1 := if right < v1.x then 1 else 0
      let p2 := if left < v1.x then 1 else 0
      if ld.dy < 0 then (p1 ^^^ 1, p2 ^^^ 1) else (p1, p2)
    else if ld.slopetype == ST_POSITIVE then
      (pointOnLineSideRaw left top v1.x v1.y ld.dx ld.dy,
       pointOnLineSideRaw right bottom v1.x v1.y ld.dx ld.dy)
    else
      (pointOnLineSideRaw right top v1.x v1.y ld.dx ld.dy,
       pointOnLineSideRaw left bottom v1.x v1.y ld.dx ld.dy)
  if p1 == p2 then p1.toInt32 else -1

/-- `P_PointOnDivlineSide`. -/
@[inline] def pointOnDivlineSide (x y lx ly ldx ldy : Int32) : Nat :=
  if ldx == 0 then
    if x <= lx then (if ldy > 0 then 1 else 0)
    else (if ldy < 0 then 1 else 0)
  else if ldy == 0 then
    if y <= ly then (if ldx < 0 then 1 else 0)
    else (if ldx > 0 then 1 else 0)
  else
    let dx := x - lx
    let dy := y - ly
    if ((ldy ^^^ ldx ^^^ dx ^^^ dy) &&& Int32.minValue) != 0 then
      if ((ldy ^^^ dx) &&& Int32.minValue) != 0 then 1 else 0
    else
      let left := fixedMul (ldy >>> 8) (dx >>> 8)
      let right := fixedMul (dy >>> 8) (ldx >>> 8)
      if right < left then 0 else 1

/-- `P_InterceptVector(v2, v1)`. -/
@[inline] def interceptVector (v2x v2y v2dx v2dy v1x v1y v1dx v1dy : Int32) : Int32 :=
  let den := fixedMul (v1dy >>> 8) v2dx - fixedMul (v1dx >>> 8) v2dy
  if den == 0 then 0
  else
    let num := fixedMul ((v1x - v2x) >>> 8) v1dy + fixedMul ((v2y - v1y) >>> 8) v1dx
    fixedDiv num den

/-- `P_LineOpening`: sets the `open*` globals (only `openrange` for one-sided). -/
def lineOpening (ld : Line) : M Unit := modify fun gs =>
  if ld.sidenum1 == -1 then { gs with openrange := 0 }
  else
    let front := gs.sectors[ld.frontsector.toNatClampNeg]!
    let back := gs.sectors[ld.backsector.toNatClampNeg]!
    let opentop := if front.ceilingheight < back.ceilingheight then front.ceilingheight
                   else back.ceilingheight
    let (openbottom, lowfloor) :=
      if front.floorheight > back.floorheight then (front.floorheight, back.floorheight)
      else (back.floorheight, front.floorheight)
    { gs with opentop, openbottom, lowfloor, openrange := opentop - openbottom }

/-- `R_PointOnSide`. -/
@[inline] def pointOnNodeSide (x y : Int32) (node : Node) : Nat :=
  if node.dx == 0 then
    if x <= node.x then (if node.dy > 0 then 1 else 0)
    else (if node.dy < 0 then 1 else 0)
  else if node.dy == 0 then
    if y <= node.y then (if node.dx < 0 then 1 else 0)
    else (if node.dx > 0 then 1 else 0)
  else
    let dx := x - node.x
    let dy := y - node.y
    if ((node.dy ^^^ node.dx ^^^ dx ^^^ dy) &&& Int32.minValue) != 0 then
      if ((node.dy ^^^ dx) &&& Int32.minValue) != 0 then 1 else 0
    else
      let left := fixedMul (node.dy >>> 16) dx
      let right := fixedMul dy (node.dx >>> 16)
      if right < left then 0 else 1

/-- `R_PointInSubsector`. -/
def pointInSubsector (level : LevelData) (x y : Int32) : Nat := Id.run do
  if level.nodes.size == 0 then return 0
  let mut nodenum : UInt32 := (level.nodes.size - 1).toUInt32
  let mut guard := level.nodes.size + 2
  while (nodenum &&& NF_SUBSECTOR) == 0 && guard > 0 do
    guard := guard - 1
    let node := level.nodes[nodenum.toNat]!
    nodenum := if pointOnNodeSide x y node == 0 then node.child0 else node.child1
  return (nodenum &&& (~~~NF_SUBSECTOR)).toNat

/-- Blockmap cell index for a position, if on the map. -/
@[inline] def blockCell (gs : GameState) (x y : Int32) : Option Nat :=
  let blockx := (x - gs.bmaporgx) >>> 23
  let blocky := (y - gs.bmaporgy) >>> 23
  if blockx >= 0 && blockx < gs.bmapwidth && blocky >= 0 && blocky < gs.bmapheight then
    some (blocky * gs.bmapwidth + blockx).toNatClampNeg
  else none

/-- `P_UnsetThingPosition`. -/
def unsetThingPosition (mi : Nat) : M Unit := modify fun (gs : GameState) => Id.run do
  let mo := gs.mobjs[mi]!
  let mut gs := gs
  if (mo.flags &&& MF_NOSECTOR) == 0 then
    if mo.snext >= 0 then
      gs := { gs with mobjs := gs.mobjs.modify mo.snext.toNatClampNeg fun m => { m with sprev := mo.sprev } }
    if mo.sprev >= 0 then
      gs := { gs with mobjs := gs.mobjs.modify mo.sprev.toNatClampNeg fun m => { m with snext := mo.snext } }
    else
      let s := moSector gs mo
      gs := { gs with sectors := gs.sectors.modify s fun sec => { sec with thinglist := mo.snext } }
  if (mo.flags &&& MF_NOBLOCKMAP) == 0 then
    if mo.bnext >= 0 then
      gs := { gs with mobjs := gs.mobjs.modify mo.bnext.toNatClampNeg fun m => { m with bprev := mo.bprev } }
    if mo.bprev >= 0 then
      gs := { gs with mobjs := gs.mobjs.modify mo.bprev.toNatClampNeg fun m => { m with bnext := mo.bnext } }
    else
      match blockCell gs mo.x mo.y with
      | some c => gs := { gs with blocklinks := gs.blocklinks.set! c (mo.bnext + 1).toNatClampNeg }
      | none => pure ()
  return gs

/-- `P_SetThingPosition`. -/
def setThingPosition (mi : Nat) : M Unit := modify fun (gs : GameState) => Id.run do
  let mo0 := gs.mobjs[mi]!
  let ss := pointInSubsector gs.level mo0.x mo0.y
  let mut gs := { gs with mobjs := gs.mobjs.modify mi fun m => { m with subsector := ss.toUInt32 } }
  if (mo0.flags &&& MF_NOSECTOR) == 0 then
    let s := gs.level.subsectors[ss]!.sector.toNat
    let head := gs.sectors[s]!.thinglist
    gs := { gs with mobjs := gs.mobjs.modify mi fun m => { m with sprev := -1, snext := head } }
    if head >= 0 then
      gs := { gs with mobjs := gs.mobjs.modify head.toNatClampNeg fun m => { m with sprev := mi.toInt32 } }
    gs := { gs with sectors := gs.sectors.modify s fun sec => { sec with thinglist := mi.toInt32 } }
  if (mo0.flags &&& MF_NOBLOCKMAP) == 0 then
    match blockCell gs mo0.x mo0.y with
    | some c =>
      let head : Int32 := (gs.blocklinks[c]!).toInt32 - 1
      gs := { gs with mobjs := gs.mobjs.modify mi fun m => { m with bprev := -1, bnext := head } }
      if head >= 0 then
        gs := { gs with mobjs := gs.mobjs.modify head.toNatClampNeg fun m => { m with bprev := mi.toInt32 } }
      gs := { gs with blocklinks := gs.blocklinks.set! c (mi + 1) }
    | none =>
      gs := { gs with mobjs := gs.mobjs.modify mi fun m => { m with bnext := -1, bprev := -1 } }
  return gs

/-- Signed 16-bit entry of the blockmap lump. -/
@[inline] def bmapAt (gs : GameState) (i : Nat) : Int32 :=
  let lo := gs.bmapLump[2 * i]!.toUInt32
  let hi := gs.bmapLump[2 * i + 1]!.toUInt32
  (lo ||| (hi <<< 8)).toUInt16.toInt16.toInt32

/-- `P_BlockLinesIterator`. -/
@[specialize] def blockLinesIterator (x y : Int32) (f : Nat → M Bool) : M Bool := do
  let gs ← get
  if x < 0 || y < 0 || x >= gs.bmapwidth || y >= gs.bmapheight then return true
  let offset := bmapAt gs (4 + (y * gs.bmapwidth + x).toNatClampNeg)
  let mut li : Int := offset.toInt
  let mut guard := gs.bmapLump.size
  while guard > 0 do
    guard := guard - 1
    let gs ← get
    let ldIdx := bmapAt gs li.toNat
    if ldIdx == -1 then return true
    let l := ldIdx.toNatClampNeg
    if gs.lineValidcount[l]! != gs.validcount then
      set { gs with lineValidcount := gs.lineValidcount.set! l gs.validcount }
      if !(← f l) then return false
    li := li + 1
  return true

/-- `P_BlockThingsIterator`; `bnext` is read after the callback, as in C. -/
@[specialize] def blockThingsIterator (x y : Int32) (f : Nat → M Bool) : M Bool := do
  let gs ← get
  if x < 0 || y < 0 || x >= gs.bmapwidth || y >= gs.bmapheight then return true
  let mut cur : Int32 := (gs.blocklinks[(y * gs.bmapwidth + x).toNatClampNeg]!).toInt32 - 1
  let mut guard := gs.mobjs.size + 1
  while cur >= 0 && guard > 0 do
    guard := guard - 1
    if !(← f cur.toNatClampNeg) then return false
    cur := (← get).mobjs[cur.toNatClampNeg]!.bnext
  return true

def PT_ADDLINES : Nat := 1
def PT_ADDTHINGS : Nat := 2
def PT_EARLYOUT : Nat := 4

def MAXINTERCEPTS_ORIGINAL : Nat := 128

@[inline] def rdU32 (b : ByteArray) (o : Nat) : UInt32 :=
  b[o]!.toUInt32 ||| (b[o+1]!.toUInt32 <<< 8) ||| (b[o+2]!.toUInt32 <<< 16) |||
    (b[o+3]!.toUInt32 <<< 24)

@[inline] def wrU32 (b : ByteArray) (o : Nat) (v : UInt32) : ByteArray :=
  ((((b.set! o v.toUInt8).set! (o+1) (v >>> 8).toUInt8).set! (o+2) (v >>> 16).toUInt8).set!
    (o+3) (v >>> 24).toUInt8)

/-- `InterceptsMemoryOverrun`: write `value` at byte `location` of the vanilla
memory that follows `intercepts[]`. Pointer-valued writes (`isPtr`) cannot be
reproduced (host addresses) and are written as 0. -/
def interceptsMemoryOverrun (location : Nat) (value : Int32) : M Unit := modify fun gs =>
  -- (len, target): 0 none, 1 lowfloor, 2 openbottom, 3 opentop, 4 openrange,
  -- 5 bulletslope, 6 playerstarts(int16), 7 bmapwidth, 8 bmaporgx, 9 bmaporgy, 10 bmapheight
  let table : Array (Nat × Nat) := #[(4,0),(4,0),(4,0),(4,1),(4,2),(4,3),(4,4),(4,0),(120,0),
    (8,0),(4,5),(4,0),(4,0),(4,0),(40,6),(4,0),(4,7),(4,0),(4,8),(4,9),(4,0),(4,10)]
  Id.run do
    let mut offset := 0
    for (len, target) in table do
      if offset + len > location then
        let gs := match target with
          | 1 => { gs with lowfloor := value }
          | 2 => { gs with openbottom := value }
          | 3 => { gs with opentop := value }
          | 4 => { gs with openrange := value }
          | 5 => { gs with bulletslope := value }
          | 6 =>
            -- playerstarts as an array of shorts: x, y, angle, type, options per start
            let idx := (location - offset) / 2
            let setShort (ps : Array MapThing) (k : Nat) (v : Int32) : Array MapThing :=
              let p := k / 5
              if p >= ps.size then ps else
              let s16 := v.toUInt32.toUInt16.toInt16.toInt32
              ps.modify p fun m => match k % 5 with
                | 0 => { m with x := s16 } | 1 => { m with y := s16 }
                | 2 => { m with angle := s16 } | 3 => { m with type_ := s16 }
                | _ => { m with options := s16 }
            let ps := setShort gs.playerstarts idx value
            let ps := setShort ps (idx + 1) (value >>> 16)
            { gs with playerstarts := ps }
          | 7 => { gs with bmapwidth := value }
          | 8 => { gs with bmaporgx := value }
          | 9 => { gs with bmaporgy := value }
          | 10 => { gs with bmapheight := value }
          | _ => gs
        return gs
      offset := offset + len
    return gs

/-- Append an intercept (`intercept_p++`) with overrun emulation. -/
def pushIntercept (frac : Int32) (isaline : Bool) (d : Nat) : M Unit := do
  let n := (← get).numIntercepts
  let o := n * 8
  let word : UInt32 := d.toUInt32 ||| (if isaline then 0x80000000 else 0)
  modify fun gs =>
    let buf := gs.intercepts
    let gs := { gs with intercepts := ByteArray.empty }
    let buf := if o + 8 > buf.size then buf.push 0 |>.push 0 |>.push 0 |>.push 0
                 |>.push 0 |>.push 0 |>.push 0 |>.push 0 else buf
    { gs with intercepts := wrU32 (wrU32 buf o frac.toUInt32) (o + 4) word, numIntercepts := n + 1 }
  if n > MAXINTERCEPTS_ORIGINAL then
    let location := (n - MAXINTERCEPTS_ORIGINAL - 1) * 12
    interceptsMemoryOverrun location frac
    interceptsMemoryOverrun (location + 4) (b2i isaline)
    interceptsMemoryOverrun (location + 8) 0

/-- `PIT_AddLineIntercepts`. -/
def pitAddLineIntercepts (l : Nat) : M Bool := do
  let gs ← get
  let g := gs.lineGeo[l]!
  let (s1, s2) :=
    if gs.traceDx > 16 * FRACUNIT || gs.traceDy > 16 * FRACUNIT ||
        gs.traceDx < -16 * FRACUNIT || gs.traceDy < -16 * FRACUNIT then
      (pointOnDivlineSide g.v1x g.v1y gs.traceX gs.traceY gs.traceDx gs.traceDy,
       pointOnDivlineSide g.v2x g.v2y gs.traceX gs.traceY gs.traceDx gs.traceDy)
    else
      (pointOnLineSideRaw gs.traceX gs.traceY g.v1x g.v1y g.dx g.dy,
       pointOnLineSideRaw (gs.traceX + gs.traceDx) (gs.traceY + gs.traceDy) g.v1x g.v1y g.dx g.dy)
  if s1 == s2 then return true
  let frac := interceptVector gs.traceX gs.traceY gs.traceDx gs.traceDy g.v1x g.v1y g.dx g.dy
  if frac < 0 then return true
  if gs.earlyout && frac < FRACUNIT && g.back < 0 then return false
  pushIntercept frac true l
  return true

/-- `PIT_AddThingIntercepts`. -/
def pitAddThingIntercepts (mi : Nat) : M Bool := do
  let gs ← get
  let th := gs.mobjs[mi]!
  let tracepositive := (gs.traceDx ^^^ gs.traceDy) > 0
  let (x1, y1, x2, y2) :=
    if tracepositive then
      (th.x - th.radius, th.y + th.radius, th.x + th.radius, th.y - th.radius)
    else
      (th.x - th.radius, th.y - th.radius, th.x + th.radius, th.y + th.radius)
  let s1 := pointOnDivlineSide x1 y1 gs.traceX gs.traceY gs.traceDx gs.traceDy
  let s2 := pointOnDivlineSide x2 y2 gs.traceX gs.traceY gs.traceDx gs.traceDy
  if s1 == s2 then return true
  let frac := interceptVector gs.traceX gs.traceY gs.traceDx gs.traceDy x1 y1 (x2 - x1) (y2 - y1)
  if frac < 0 then return true
  pushIntercept frac false mi
  return true

/-- `P_TraverseIntercepts`. `trav frac isaline index`. -/
@[specialize] def traverseIntercepts (trav : Int32 → Bool → Nat → M Bool) (maxfrac : Int32) :
    M Bool := do
  let mut count := (← get).numIntercepts
  while count > 0 do
    count := count - 1
    let gs ← get
    let mut dist : Int32 := Int32.maxValue
    let mut best : Nat := 0
    for i in [0:gs.numIntercepts] do
      let f := (rdU32 gs.intercepts (i * 8)).toInt32
      if f < dist then
        dist := f
        best := i
    if dist > maxfrac then return true
    let word := rdU32 gs.intercepts (best * 8 + 4)
    let isaline := (word &&& 0x80000000) != 0
    let d := (word &&& 0x7fffffff).toNat
    if !(← trav dist isaline d) then return false
    modify fun gs =>
      let buf := gs.intercepts
      let gs := { gs with intercepts := ByteArray.empty }
      { gs with intercepts := wrU32 buf (best * 8) Int32.maxValue.toUInt32 }
  return true

/-- `P_PathTraverse`. -/
@[specialize] def pathTraverse (x1_0 y1_0 x2_0 y2_0 : Int32) (flags : Nat)
    (trav : Int32 → Bool → Nat → M Bool) : M Bool := do
  bumpValidcount
  let gs ← get
  let earlyout := (flags &&& PT_EARLYOUT) != 0
  let mut sx := x1_0
  let mut sy := y1_0
  if ((sx - gs.bmaporgx) &&& (128 * FRACUNIT - 1)) == 0 then sx := sx + FRACUNIT
  if ((sy - gs.bmaporgy) &&& (128 * FRACUNIT - 1)) == 0 then sy := sy + FRACUNIT
  let orgx := gs.bmaporgx
  let orgy := gs.bmaporgy
  set { gs with earlyout, numIntercepts := 0, traceX := sx, traceY := sy,
                traceDx := x2_0 - sx, traceDy := y2_0 - sy }
  let x1 := sx - orgx
  let y1 := sy - orgy
  let xt1 := x1 >>> 23
  let yt1 := y1 >>> 23
  let x2 := x2_0 - orgx
  let y2 := y2_0 - orgy
  let xt2 := x2 >>> 23
  let yt2 := y2 >>> 23
  let (mapxstep, partialX, ystep) : Int32 × Int32 × Int32 :=
    if xt2 > xt1 then
      (1, FRACUNIT - ((x1 >>> 7) &&& (FRACUNIT - 1)), fixedDiv (y2 - y1) (wabs (x2 - x1)))
    else if xt2 < xt1 then
      (-1, (x1 >>> 7) &&& (FRACUNIT - 1), fixedDiv (y2 - y1) (wabs (x2 - x1)))
    else (0, FRACUNIT, 256 * FRACUNIT)
  let mut yintercept := (y1 >>> 7) + fixedMul partialX ystep
  let (mapystep, partialY, xstep) : Int32 × Int32 × Int32 :=
    if yt2 > yt1 then
      (1, FRACUNIT - ((y1 >>> 7) &&& (FRACUNIT - 1)), fixedDiv (x2 - x1) (wabs (y2 - y1)))
    else if yt2 < yt1 then
      (-1, (y1 >>> 7) &&& (FRACUNIT - 1), fixedDiv (x2 - x1) (wabs (y2 - y1)))
    else (0, FRACUNIT, 256 * FRACUNIT)
  let mut xintercept := (x1 >>> 7) + fixedMul partialY xstep
  let mut mapx := xt1
  let mut mapy := yt1
  let mut count := 0
  while count < 64 do
    count := count + 1
    if (flags &&& PT_ADDLINES) != 0 then
      if !(← blockLinesIterator mapx mapy pitAddLineIntercepts) then return false
    if (flags &&& PT_ADDTHINGS) != 0 then
      if !(← blockThingsIterator mapx mapy pitAddThingIntercepts) then return false
    if mapx == xt2 && mapy == yt2 then
      count := 64
    else if (yintercept >>> 16) == mapy then
      yintercept := yintercept + ystep
      mapx := mapx + mapxstep
    else if (xintercept >>> 16) == mapx then
      xintercept := xintercept + xstep
      mapy := mapy + mapystep
  traverseIntercepts trav FRACUNIT

end Doom.Playsim.MapUtil
