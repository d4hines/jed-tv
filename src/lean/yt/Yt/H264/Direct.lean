/-
B direct motion prediction (FFmpeg h264_direct.c), for frame pictures.
Part of the Lean H.264 decoder (follows FFmpeg 9.0.1, LGPL-2.1-or-later).

Spatial and temporal direct; field pictures and MBAFF are not supported
(the decoder rejects them).
-/
import Yt.H264.Caches

namespace Yt.H264

def MB_16x16_OR_INTRA : UInt32 := MB_16x16 ||| MB_INTRA4x4 ||| MB_INTRA16x16 ||| MB_INTRA_PCM

/-- FFMIN of references compared as unsigned (negative = huge). -/
def minRefUnsigned (a b c : Int) : Int :=
  let u (v : Int) : Int := if v < 0 then v + 4294967296 else v
  let m := min (u a) (min (u b) (u c))
  if m ≥ 2147483648 then m - 4294967296 else m

@[inline] def absLe1 (v : Int) : Bool := v ≥ -1 && v ≤ 1

/-- A motion vector as one `UInt32`: int16 x, then int16 y in the high half. -/
@[inline] def packMv (x y : Int) : UInt32 := x.toInt16.toUInt16.toUInt32 ||| (y.toInt16.toUInt16.toUInt32 <<< 16)
@[inline] def packedX (p : UInt32) : Int := p.toUInt16.toInt16.toInt
@[inline] def packedY (p : UInt32) : Int := (p >>> 16).toUInt16.toInt16.toInt

/-- Spatial direct reference index of one list from its cache's neighbours
A (left), B (top) and C (top right, else top left); -1: none. -/
def spatialRef (rc : @& ByteArray) : Int :=
  let c := getI8 rc 8
  minRefUnsigned (getI8 rc 11) (getI8 rc 4) (if c == PART_NOT_AVAILABLE then getI8 rc 3 else c)

/-- Its motion vector for reference `r` (≥ 0), packed. -/
def spatialMv (rc mc : @& ByteArray) (r : Int) : UInt32 :=
  let leftRef := getI8 rc 11
  let topRef := getI8 rc 4
  let c0 := getI8 rc 8
  let useD := c0 == PART_NOT_AVAILABLE
  let refc := if useD then getI8 rc 3 else c0
  let ce := if useD then 3 else 8
  let nMatch := (if leftRef == r then 1 else 0) + (if topRef == r then 1 else 0) + (if refc == r then 1 else 0)
  if nMatch > 1 then packMv (mid3 (mvX mc 11) (mvX mc 4) (mvX mc ce)) (mid3 (mvY mc 11) (mvY mc 4) (mvY mc ce))
  else if leftRef == r then packMv (mvX mc 11) (mvY mc 11)
  else if topRef == r then packMv (mvX mc 4) (mvY mc 4)
  else packMv (mvX mc ce) (mvY mc ce)

/-- Whether the colocated vector at table block `c` of `l1mv` is within ±1. -/
@[inline] def colZero (l1mv : @& ByteArray) (c : Nat) : Bool := absLe1 (mvX l1mv c) && absLe1 (mvY l1mv c)

/-- The 4x4 blocks `i4 < 4` of a direct 8x8 partition with a still colocated
block: zero vectors where the reference is 0; the count of still blocks in `aux`. -/
def spatialSubs (d : Dec) (l1mv : @& ByteArray) (i8 b bs : Nat) (ref0 ref1 : Int) (i4 m : Nat) : Dec :=
  if i4 ≥ 4 then { d with aux := m }
  else
    let x8 := i8 &&& 1
    let y8 := i8 >>> 1
    let c := b + x8 * 2 + (i4 &&& 1) + (y8 * 2 + (i4 >>> 1)) * bs
    if colZero l1mv c then
      let ei := s8 (4 * i8 + i4)
      let d := if ref0 == 0 then d.fillMvC 0 ei 1 1 0 0 else d
      let d := if ref1 == 0 then d.fillMvC 1 ei 1 1 0 0 else d
      spatialSubs d l1mv i8 b bs ref0 ref1 (i4 + 1) (m + 1)
    else spatialSubs d l1mv i8 b bs ref0 ref1 (i4 + 1) m
termination_by 4 - i4

/-- The direct 8x8 partitions `i8 < 4` (all of them, or the direct ones of a
B_8x8): their motion and sub-types; the count of still 4x4 blocks in `aux`. -/
def spatial8x8 (d : Dec) (col : RefPic) (colOk isB8x8 : Bool) (subType : UInt32) (ref0 ref1 : Int)
    (mv0 mv1 : UInt32) (i8 n : Nat) : Dec :=
  if i8 ≥ 4 then { d with aux := n }
  else if isB8x8 && !isDirect (d.sub i8) then spatial8x8 d col colOk isB8x8 subType ref0 ref1 mv0 mv1 (i8 + 1) n
  else
    let x8 := i8 &&& 1
    let y8 := i8 >>> 1
    let bs := d.bStride
    let b := d.mb2b d.mbXY
    let b8 := 4 * d.mbXY
    let d := d.setSub i8 subType
    let e := s8 (4 * i8)
    let d := d.fillMvC 0 e 2 2 (packedX mv0) (packedY mv0)
    let d := d.fillMvC 1 e 2 2 (packedX mv1) (packedY mv1)
    let d := d.fillRefC 0 e 2 2 (toByte ref0)
    let d := d.fillRefC 1 e 2 2 (toByte ref1)
    let l1r0 := getI8 col.ref0 (b8 + i8)
    let l1r1 := getI8 col.ref1 (b8 + i8)
    if colOk && (l1r0 == 0 || (l1r0 < 0 && l1r1 == 0)) then
      let l1mv := if l1r0 == 0 then col.mv0 else col.mv1
      if subType &&& MB_16x16 != 0 then
        if colZero l1mv (b + x8 * 3 + y8 * 3 * bs) then
          let d := if ref0 == 0 then d.fillMvC 0 e 2 2 0 0 else d
          let d := if ref1 == 0 then d.fillMvC 1 e 2 2 0 0 else d
          spatial8x8 d col colOk isB8x8 subType ref0 ref1 mv0 mv1 (i8 + 1) (n + 4)
        else spatial8x8 d col colOk isB8x8 subType ref0 ref1 mv0 mv1 (i8 + 1) n
      else
        let d := spatialSubs d l1mv i8 b bs ref0 ref1 0 0
        let m := d.aux
        let d := if m &&& 3 == 0 then d.setSub i8 (d.sub i8 + MB_16x16 - MB_8x8) else d
        spatial8x8 d col colOk isB8x8 subType ref0 ref1 mv0 mv1 (i8 + 1) (n + m)
    else spatial8x8 d col colOk isB8x8 subType ref0 ref1 mv0 mv1 (i8 + 1) n
termination_by 4 - i8

/-- `pred_spatial_direct_motion` (frame macroblocks): the refined type in `dirType`. -/
def predSpatialDirect (d : Dec) (t : UInt32) : Dec :=
  let isB8x8 := t &&& MB_8x8 != 0
  let r0 := spatialRef d.refC0
  let r1 := spatialRef d.refC1
  let mv0 := if r0 ≥ 0 then spatialMv d.refC0 d.mvC0 r0 else 0
  let mv1 := if r1 ≥ 0 then spatialMv d.refC1 d.mvC1 r1 else 0
  let mask : UInt32 := (if r0 ≥ 0 then 0xFFFFFFFF else ~~~MB_L0) &&& (if r1 ≥ 0 then 0xFFFFFFFF else ~~~(MB_L0 <<< (2 : UInt32)))
  let none := r0 < 0 && r1 < 0
  let t := if isB8x8 then t else if none then (t &&& mask) ||| MB_L0L1 else t &&& mask
  let subType := if none then MB_L0L1 else MB_L0L1 &&& mask
  -- an unavailable list's reference is -1 (minRefUnsigned may give -2)
  let ref0 := if none then 0 else if r0 < 0 then -1 else r0
  let ref1 := if none then 0 else if r1 < 0 then -1 else r1
  if !isB8x8 && mv0 == 0 && mv1 == 0 then
    let d := (((d.fillRefC 0 12 4 4 (toByte ref0)).fillRefC 1 12 4 4 (toByte ref1)).fillMvC 0 12 4 4 0 0).fillMvC 1 12 4 4 0 0
    { d with dirType := (t &&& ~~~(MB_8x8 ||| MB_16x8 ||| MB_8x16 ||| MB_P1L0 ||| MB_P1L1)) ||| MB_16x16 ||| MB_DIRECT2 }
  else
    let col := d.sl.refList1[0]!
    let colType := getU32 col.mbType d.mbXY
    let subType := subType ||| MB_16x16 ||| MB_DIRECT2
    let big := !isB8x8 && colType &&& MB_16x16_OR_INTRA != 0
    let half := !isB8x8 && !big && colType &&& (MB_16x8 ||| MB_8x16) != 0
    let t :=
      if big then t ||| MB_16x16 ||| MB_DIRECT2
      else if half then t ||| MB_DIRECT2 ||| (colType &&& (MB_16x8 ||| MB_8x16))
      else t ||| MB_8x8
    let subType := if !big && !half && !d.sl.direct8x8Inference then subType + (MB_8x8 - MB_16x16) else subType
    let b := d.mb2b d.mbXY
    let b8 := 4 * d.mbXY
    let colOk := !isIntra colType && !col.longTerm
    if t &&& MB_16x16 != 0 then
      let d := (d.fillRefC 0 12 4 4 (toByte ref0)).fillRefC 1 12 4 4 (toByte ref1)
      let l1r0 := getI8 col.ref0 b8
      let zeroCol := colOk &&
        ((l1r0 == 0 && colZero col.mv0 b) || (l1r0 < 0 && getI8 col.ref1 b8 == 0 && colZero col.mv1 b))
      let a := if zeroCol && ref0 ≤ 0 then 0 else mv0
      let bb := if zeroCol && ref1 ≤ 0 then 0 else mv1
      let d := (d.fillMvC 0 12 4 4 (packedX a) (packedY a)).fillMvC 1 12 4 4 (packedX bb) (packedY bb)
      { d with dirType := t }
    else
      let d := spatial8x8 d col colOk isB8x8 subType ref0 ref1 mv0 mv1 0 0
      let n := d.aux
      let t := if !isB8x8 && n &&& 15 == 0 then
          (t &&& ~~~(MB_8x8 ||| MB_16x8 ||| MB_8x16 ||| MB_P1L0 ||| MB_P1L1)) ||| MB_16x16 ||| MB_DIRECT2
        else t
      { d with dirType := t }

/-- `pred_temp_direct_motion` (frame macroblock, frame colocated). -/
def predTemporalDirect (d : Dec) (t : UInt32) : Dec := Id.run do
  let mut d := d
  let mut t := t
  let isB8x8 := t &&& MB_8x8 != 0
  let col := d.sl.refList1[0]!
  let colType := getU32 col.mbType d.mbXY
  let mut subType : UInt32 := MB_16x16 ||| MB_P0L0 ||| MB_P0L1 ||| MB_DIRECT2
  if !isB8x8 && colType &&& MB_16x16_OR_INTRA != 0 then
    t := t ||| MB_16x16 ||| MB_P0L0 ||| MB_P0L1 ||| MB_DIRECT2
  else if !isB8x8 && colType &&& (MB_16x8 ||| MB_8x16) != 0 then
    t := t ||| MB_L0L1 ||| MB_DIRECT2 ||| (colType &&& (MB_16x8 ||| MB_8x16))
  else
    if !d.sl.direct8x8Inference then subType := MB_8x8 ||| MB_P0L0 ||| MB_P0L1 ||| MB_DIRECT2
    t := t ||| MB_8x8 ||| MB_L0L1
  let bs := d.bStride
  let b := d.mb2b d.mbXY
  let b8 := 4 * d.mbXY
  let l1ref0 (i : Nat) := getI8 col.ref0 (b8 + i)
  let l1ref1 (i : Nat) := getI8 col.ref1 (b8 + i)
  let mapRef (i : Nat) : Nat × ByteArray :=
    if l1ref0 i ≥ 0 then (d.sl.mapCol0.getD (l1ref0 i).toNat 0, col.mv0)
    else (d.sl.mapCol1.getD (l1ref1 i).toNat 0, col.mv1)
  let scaled (scale c : Int) : Int := (scale * c + 128) >>> 8
  if t &&& MB_16x16 != 0 then
    d := d.fillRefC 1 (s8 0) 4 4 0
    if isIntra colType then
      d := d.fillRefC 0 (s8 0) 4 4 0
      d := d.fillMvC 0 (s8 0) 4 4 0 0
      d := d.fillMvC 1 (s8 0) 4 4 0 0
    else
      let (ref0, l1mv) := mapRef 0
      let scale := d.sl.distScale.getD ref0 256
      let cx := mvX l1mv b
      let cy := mvY l1mv b
      let mx := scaled scale cx
      let my := scaled scale cy
      d := d.fillRefC 0 (s8 0) 4 4 ref0.toUInt8
      d := d.fillMvC 0 (s8 0) 4 4 mx my
      d := d.fillMvC 1 (s8 0) 4 4 (mx - cx) (my - cy)
    return { d with dirType := t }
  for i8 in [0:4] do
    let x8 := i8 &&& 1
    let y8 := i8 >>> 1
    if isB8x8 && !isDirect (d.sub i8) then continue
    d := d.setSub i8 subType
    let e := s8 (4 * i8)
    d := d.fillRefC 1 e 2 2 0
    if isIntra colType then
      d := d.fillRefC 0 e 2 2 0
      d := d.fillMvC 0 e 2 2 0 0
      d := d.fillMvC 1 e 2 2 0 0
      continue
    let (ref0, l1mv) := mapRef i8
    let scale := d.sl.distScale.getD ref0 256
    d := d.fillRefC 0 e 2 2 ref0.toUInt8
    if subType &&& MB_16x16 != 0 then
      let c := b + x8 * 3 + y8 * 3 * bs
      let mx := scaled scale (mvX l1mv c)
      let my := scaled scale (mvY l1mv c)
      d := d.fillMvC 0 e 2 2 mx my
      d := d.fillMvC 1 e 2 2 (mx - mvX l1mv c) (my - mvY l1mv c)
    else
      for i4 in [0:4] do
        let c := b + x8 * 2 + (i4 &&& 1) + (y8 * 2 + (i4 >>> 1)) * bs
        let mx := scaled scale (mvX l1mv c)
        let my := scaled scale (mvY l1mv c)
        let ei := s8 (4 * i8 + i4)
        d := d.fillMvC 0 ei 1 1 mx my
        d := d.fillMvC 1 ei 1 1 (mx - mvX l1mv c) (my - mvY l1mv c)
  return { d with dirType := t }

/-- `ff_h264_pred_direct_motion`: fills the motion caches of a direct
macroblock (or the direct 8x8 partitions of a B_8x8); the refined
macroblock type in `dirType`. -/
def predDirectMotion (d : Dec) (t : UInt32) : Dec :=
  if d.sl.directSpatial then predSpatialDirect d t else predTemporalDirect d t

end Yt.H264
