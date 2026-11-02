import Doom.Playsim.Fixed
import Doom.Playsim.GameState
import Doom.Playsim.Level
import Doom.Playsim.MapUtil

/-!
# Doom.Playsim.Sight

`P_CheckSight` (`p_sight.c`): REJECT lookup, then the BSP walk from t1's eye
to t2. Updates `validcount`, line stamps and the shared `topslope` /
`bottomslope` / `sightzstart` globals.
-/

namespace Doom.Playsim.Sight

open Doom.Playsim.Fixed
open Doom.Playsim.GameState
open Doom.Playsim.Level
open Doom.Playsim.MapUtil

/-- `P_DivlineSide`: 0 front, 1 back, 2 on (with the vanilla `x == node->y` bug). -/
@[inline] def divlineSide (x y nx ny ndx ndy : Int32) : Nat :=
  if ndx == 0 then
    if x == nx then 2
    else if x <= nx then (if ndy > 0 then 1 else 0)
    else (if ndy < 0 then 1 else 0)
  else if ndy == 0 then
    if x == ny then 2
    else if y <= ny then (if ndx < 0 then 1 else 0)
    else (if ndx > 0 then 1 else 0)
  else
    let left := (ndy >>> 16) * ((x - nx) >>> 16)
    let right := ((y - ny) >>> 16) * (ndx >>> 16)
    if right < left then 0
    else if left == right then 2
    else 1

structure SightCtx where
  sx : Int32
  sy : Int32
  sdx : Int32
  sdy : Int32
  t2x : Int32
  t2y : Int32
  sightzstart : Int32
  vc : Nat

/-- Mutable part of the walk: line stamps and the shared slope globals. -/
structure SightAcc where
  lvc : Array Nat
  topslope : Int32
  bottomslope : Int32

/-- `P_CrossSubsector` (pure over the read-only state). -/
def crossSubsector (gs : GameState) (c : SightCtx) (num : Nat) (acc0 : SightAcc) :
    Bool × SightAcc := Id.run do
  let sub := gs.level.subsectors[num]!
  let first := sub.firstseg.toNat
  let mut acc := acc0
  for k in [0:sub.numsegs.toNat] do
    let seg := gs.level.segs[first + k]!
    let l := seg.linedef.toNat
    if acc.lvc[l]! == c.vc then continue
    acc := { acc with lvc := acc.lvc.set! l c.vc }
    let g := gs.lineGeo[l]!
    let s1 := divlineSide g.v1x g.v1y c.sx c.sy c.sdx c.sdy
    let s2 := divlineSide g.v2x g.v2y c.sx c.sy c.sdx c.sdy
    if s1 == s2 then continue
    let ddx := g.v2x - g.v1x
    let ddy := g.v2y - g.v1y
    let s1 := divlineSide c.sx c.sy g.v1x g.v1y ddx ddy
    let s2 := divlineSide c.t2x c.t2y g.v1x g.v1y ddx ddy
    if s1 == s2 then continue
    if g.back < 0 then return (false, acc)
    if (g.flags &&& ML_TWOSIDED) == 0 then return (false, acc)
    let front := gs.sectors[seg.frontsector.toNatClampNeg]!
    let back := gs.sectors[seg.backsector.toNatClampNeg]!
    if front.floorheight == back.floorheight && front.ceilingheight == back.ceilingheight then
      continue
    let opentop := if front.ceilingheight < back.ceilingheight then front.ceilingheight
                   else back.ceilingheight
    let openbottom := if front.floorheight > back.floorheight then front.floorheight
                      else back.floorheight
    if openbottom >= opentop then return (false, acc)
    let frac := interceptVector c.sx c.sy c.sdx c.sdy g.v1x g.v1y ddx ddy
    if front.floorheight != back.floorheight then
      let slope := fixedDiv (openbottom - c.sightzstart) frac
      if slope > acc.bottomslope then acc := { acc with bottomslope := slope }
    if front.ceilingheight != back.ceilingheight then
      let slope := fixedDiv (opentop - c.sightzstart) frac
      if slope < acc.topslope then acc := { acc with topslope := slope }
    if acc.topslope <= acc.bottomslope then return (false, acc)
  return (true, acc)

/-- `P_CrossBSPNode`. -/
partial def crossBSPNode (gs : GameState) (c : SightCtx) (bspnum : Int) (acc : SightAcc) :
    Bool × SightAcc :=
  if bspnum == -1 then crossSubsector gs c 0 acc
  else
    let b := bspnum.toNat
    if (b &&& 0x8000) != 0 then crossSubsector gs c (b &&& 0x7fff) acc
    else
      let bsp := gs.level.nodes[b]!
      let side0 := divlineSide c.sx c.sy bsp.x bsp.y bsp.dx bsp.dy
      let side := if side0 == 2 then 0 else side0
      let child := if side == 0 then bsp.child0 else bsp.child1
      match crossBSPNode gs c child.toNat acc with
      | (false, acc) => (false, acc)
      | (true, acc) =>
        if side == divlineSide c.t2x c.t2y bsp.x bsp.y bsp.dx bsp.dy then (true, acc)
        else
          let child2 := if side == 0 then bsp.child1 else bsp.child0
          crossBSPNode gs c child2.toNat acc

/-- `P_CheckSight(t1, t2)`. -/
def checkSight (t1i t2i : Nat) : M Bool := do
  let gs ← get
  let t1 := gs.mobjs[t1i]!
  let t2 := gs.mobjs[t2i]!
  let s1 := moSector gs t1
  let s2 := moSector gs t2
  let pnum := s1 * gs.level.sectors.size + s2
  let byte := gs.reject[pnum / 8]!
  if (byte &&& ((1 : UInt8) <<< (pnum % 8).toUInt8)) != 0 then return false
  let root : Int := (gs.level.nodes.size : Int) - 1
  bumpValidcount
  let gs ← get
  let sightzstart := t1.z + t1.height - (t1.height >>> 2)
  let c : SightCtx := { sx := t1.x, sy := t1.y, sdx := t2.x - t1.x, sdy := t2.y - t1.y,
                        t2x := t2.x, t2y := t2.y, sightzstart, vc := gs.validcount }
  let acc : SightAcc := { lvc := gs.lineValidcount, topslope := (t2.z + t2.height) - sightzstart
                          bottomslope := t2.z - sightzstart }
  let gs := { gs with lineValidcount := #[] }
  let (ok, acc) := crossBSPNode gs c root acc
  set { gs with lineValidcount := acc.lvc, sightzstart, topslope := acc.topslope
                bottomslope := acc.bottomslope }
  return ok

end Doom.Playsim.Sight
