/-
Neighbours and per-macroblock caches (FFmpeg h264_mvpred.h, frame
macroblocks, CABAC, 4:2:0), the write-backs to the picture tables, and
motion vector prediction. Part of the Lean H.264 decoder (follows FFmpeg
9.0.1, LGPL-2.1-or-later).
-/
import Yt.H264.State

namespace Yt.H264
open Yt.Bytes (u8 u16 u32 u8set u16set u32set)

/-- `fill_decode_neighbors`: the neighbour macroblock types (0 when not
available in this slice). -/
@[noinline] def setNeighbors (d : Dec) (tt lt tlt trt : UInt32) : Dec :=
  { d with topType := tt, leftType := lt, topleftType := tlt, toprightType := trt }

def fillNeighbors (d : Dec) : Dec :=
  let mx := d.mbX.toUSize
  let xy := d.mbXY.toUSize
  let w := d.mbW.toUSize
  let top := d.mbY != 0
  let tt := d.typeAt top (xy - w)
  let lt := d.typeAt (mx > 0) (xy - 1)
  let tlt := d.typeAt (top && mx > 0) (xy - w - 1)
  let trt := d.typeAt (top && mx + 1 < w) (xy - w + 1)
  setNeighbors d tt lt tlt trt

@[inline] def Dec.topXY (d : Dec) : Nat := d.mbXY - d.mbW
@[inline] def Dec.leftXY (d : Dec) : Nat := d.mbXY - 1
@[inline] def Dec.topleftXY (d : Dec) : Nat := d.mbXY - d.mbW - 1
@[inline] def Dec.toprightXY (d : Dec) : Nat := d.mbXY - d.mbW + 1
/-- Row stride of the motion tables, in 4x4 blocks: each macroblock's 16
entries are contiguous (row by row), so a macroblock's first block is
`16 * xy` with no division by the picture width. -/
@[inline] def Dec.bStride (_ : Dec) : Nat := 4
/-- Index of the top-left 4x4 block of macroblock `xy` in the motion tables. -/
@[inline] def Dec.mb2b (_ : Dec) (xy : Nat) : Nat := 16 * xy

/-- Copy a 4-byte motion vector entry from table `t` (block `b`) into cache `c` (entry `e`). -/
@[inline] def copyMv (c : ByteArray) (e : Nat) (t : ByteArray) (b : Nat) : ByteArray :=
  let o := 4 * b
  if h : o + 3 < t.size then
    (((c.set! (4 * e) t[o]).set! (4 * e + 1) t[o + 1]).set! (4 * e + 2) t[o + 2]).set! (4 * e + 3) t[o + 3]
  else c

@[inline] def zeroMv (c : ByteArray) (e : Nat) : ByteArray := fillMv c e 1 1 0 0

@[inline] def copyMvd (c : ByteArray) (e : Nat) (t : ByteArray) (b : Nat) : ByteArray :=
  let o := 2 * b
  if h : o + 1 < t.size then (c.set! (2 * e) t[o]).set! (2 * e + 1) t[o + 1] else c

/-- The non_zero_count cache entries of the top and left neighbours (from
the table at their bases; `e` where a neighbour is unavailable). -/
def nnzCacheIn (tb nz : ByteArray) (top : Bool) (tb0 : Nat) (left : Bool) (b : Nat) (e : UInt8) : ByteArray :=
  let nz :=
    if top then tb.copySlice (tb0 + 36) (tb.copySlice (tb0 + 20) (tb.copySlice (tb0 + 12) nz 4 4) 44 4) 84 4
    else fillRow (fillRow (fillRow nz 4 e 4) 44 e 4) 84 e 4
  if left then
    -- left_block[8..15] = 3, 7, 11, 15, 17, 33, 21, 37
    let nz := (((nz.set! 11 tb[b + 3]!).set! 19 tb[b + 7]!).set! 27 tb[b + 11]!).set! 35 tb[b + 15]!
    (((nz.set! 51 tb[b + 17]!).set! 91 tb[b + 33]!).set! 59 tb[b + 21]!).set! 99 tb[b + 37]!
  else
    let nz := (((nz.set! 11 e).set! 19 e).set! 27 e).set! 35 e
    (((nz.set! 51 e).set! 91 e).set! 59 e).set! 99 e

/-- The reference cache value of an unavailable neighbour of type `nt`. -/
@[inline] def unavail (nt : UInt32) : UInt8 := toByte (if nt != 0 then LIST_NOT_USED else PART_NOT_AVAILABLE)

/-- The direct cache value of the left neighbour's 8x8 block `b8`. -/
def leftDirect (leftT : UInt32) (dt : @& ByteArray) (leftXY b8 : Nat) : UInt8 :=
  if isDirect leftT then (MB_DIRECT2 >>> 1).toUInt8
  else if leftT &&& MB_8x8 != 0 then dt[4 * leftXY + b8]! else (MB_16x16 >>> 1).toUInt8

/-- The neighbours' motion of one list into the caches (the inter part of
`fill_decode_caches`): row copies and single entries, no loops; byte
positions in USize. Motion tables: 16 4-byte vectors per macroblock (4 per
row), mvd tables 8 2-byte entries, reference tables 4 bytes. -/
def fillMotionList (d : Dec) (t topT leftT : UInt32) (list : Nat) : Dec :=
  let mvT := d.mvT list
  let refT := d.refT list
  let mc := d.mvC list
  let rc := d.refC list
  let d := (d.setMvC list .empty).setRefC list .empty
  let xy := d.mbXY.toUSize
  let w := d.mbW.toUSize
  let topXY := xy - w
  let leftXY := xy - 1
  -- top (entries 4..7): the bottom row of the macroblock above
  let topUsed := usesList topT list
  let mc := if topUsed then Kernels.copyRowsRaw mc 16 0 mvT (64 * topXY + 48) 0 16 1 else Kernels.fillRaw mc 16 0 4 1 0 4
  let rc :=
    if topUsed then
      let r2 := u8 refT (4 * topXY + 2)
      let r3 := u8 refT (4 * topXY + 3)
      u8set (u8set (u8set (u8set rc 4 r2) 5 r2) 6 r3) 7 r3
    else
      let v := unavail topT
      u8set (u8set (u8set (u8set rc 4 v) 5 v) 6 v) 7 v
  -- left (entries 11, 19, 27, 35): the right column of the macroblock left
  let leftUsed := usesList leftT list
  let lb := 4 * (16 * leftXY + 3)
  let split := t &&& (MB_16x8 ||| MB_8x8) != 0
  let mc :=
    if leftUsed then
      let mc := u32set mc 44 (u32 mvT lb)
      if split then u32set (u32set (u32set mc 76 (u32 mvT (lb + 16))) 108 (u32 mvT (lb + 32))) 140 (u32 mvT (lb + 48))
      else mc
    else
      let mc := u32set mc 44 0
      if split then u32set (u32set (u32set mc 76 0) 108 0) 140 0 else mc
  let rc :=
    if leftUsed then
      let ra := u8 refT (4 * leftXY + 1)
      let rc := u8set rc 11 ra
      if split then
        let rb := u8 refT (4 * leftXY + 3)
        u8set (u8set (u8set rc 19 ra) 27 rb) 35 rb
      else rc
    else
      let v := unavail leftT
      let rc := u8set rc 11 v
      if split then u8set (u8set (u8set rc 19 v) 27 v) 35 v else rc
  -- top right (entry 8)
  let trT := d.toprightType
  let trUsed := usesList trT list
  let trXY := topXY + 1
  let mc := u32set mc 32 (if trUsed then u32 mvT (4 * (16 * trXY + 12)) else 0)
  let rc := u8set rc 8 (if trUsed then u8 refT (4 * trXY + 2) else unavail trT)
  -- top left (entry 3), only when needed
  let needTl := u8 rc 6 ≥ 128 || u8 rc 8 ≥ 128
  let tlT := d.topleftType
  let tlUsed := usesList tlT list
  let tlXY := topXY - 1
  let mc := if !needTl then mc else u32set mc 12 (if tlUsed then u32 mvT (4 * (16 * tlXY + 15)) else 0)
  let rc := if !needTl then rc else u8set rc 3 (if tlUsed then u8 refT (4 * tlXY + 3) else unavail tlT)
  if t &&& (MB_SKIP ||| MB_DIRECT2) != 0 then (d.setMvC list mc).setRefC list rc
  else
    let pna := toByte PART_NOT_AVAILABLE
    let rc := u8set (u8set rc 14 pna) 30 pna
    let mc := u32set (u32set mc 56 0) 120 0
    -- mvd (CABAC)
    let mvdT := d.mvdT list
    let dc := d.mvdC list
    let d := d.setMvdC list .empty
    let dc := if topUsed then Kernels.copyRowsRaw dc 8 0 mvdT (16 * topXY) 0 8 1 else Kernels.fillRaw dc 8 0 2 1 0 4
    let dc :=
      if leftUsed then
        let b := 16 * leftXY
        let dc := u16set dc 22 (u16 mvdT (b + 12))
        let dc := u16set dc 38 (u16 mvdT (b + 10))
        let dc := u16set dc 54 (u16 mvdT (b + 8))
        u16set dc 70 (u16 mvdT (b + 6))
      else u16set (u16set (u16set (u16set dc 22 0) 38 0) 54 0) 70 0
    let dc := u16set (u16set dc 28 0) 60 0
    let d := d.setMvdC list dc
    let d :=
      if d.sl.sliceType == .b then
        let v16 := (MB_16x16 >>> 1).toUInt8
        let vD := (MB_DIRECT2 >>> 1).toUInt8
        let dt := d.pic.directTable
        let dr0 := d.directC
        let d := { d with directC := .empty }
        let dr := fillRect dr0 12 4 4 v16
        let dr :=
          if isDirect topT then fillRow dr 4 vD 4
          else if topT &&& MB_8x8 != 0 then u8set (u8set dr 4 (u8 dt (4 * topXY + 2))) 6 (u8 dt (4 * topXY + 3))
          else fillRow dr 4 v16 4
        { d with directC := u8set (u8set dr 11 (leftDirect leftT dt leftXY.toNat 1)) 27 (leftDirect leftT dt leftXY.toNat 3) }
      else d
    (d.setMvC list mc).setRefC list rc

-- fillCaches's parts update the state at one site each (a `let mut` state
-- updated in several branches is rebuilt field by field where they join).

/-- The intra sample availability (FFmpeg's `*_samples_available`) and the
intra 4x4 mode cache, for intra macroblock type `t`. -/
@[noinline] def Dec.fillIntraCaches (d : Dec) (t topT leftT : UInt32) : Dec :=
  let mask : UInt32 := if d.sl.constrainedIntra then 7 else 0xFFFFFFFF
  let topOut := topT &&& mask == 0
  let leftOut := leftT &&& mask == 0
  let tl : UInt32 := if topOut then 0xB3FF else 0xFFFF
  let tl := if leftOut then tl &&& 0xDF5F else tl
  let tl := if d.topleftType &&& mask == 0 then tl &&& 0x7FFF else tl
  let tr : UInt32 := if topOut then 0x26EA else 0xEEEA
  let tr := if d.toprightType &&& mask == 0 then tr &&& 0xFBFF else tr
  let c := d.i4C
  let d := { d with topleftSamples := tl, topSamples := if topOut then 0x33FF else 0xFFFF,
                    leftSamples := if leftOut then 0x5F5F else 0xFFFF, toprightSamples := tr, i4C := .empty }
  if isIntra4x4 t then
    let c :=
      if isIntra4x4 topT then d.pic.i4Table.copySlice (8 * d.topXY) c 4 4
      else fillRow c 4 (toByte (if topT &&& mask != 0 then 2 else -1)) 4
    let c :=
      if isIntra4x4 leftT then
        let b := 8 * d.leftXY
        (((c.set! 11 d.pic.i4Table[b + 6]!).set! 19 d.pic.i4Table[b + 5]!).set! 27 d.pic.i4Table[b + 4]!).set! 35 d.pic.i4Table[b + 3]!
      else
        let v := toByte (if leftT &&& mask != 0 then 2 else -1)
        (((c.set! 11 v).set! 19 v).set! 27 v).set! 35 v
    { d with i4C := c }
  else { d with i4C := c }

/-- The non_zero_count cache and the neighbours' coded block patterns. -/
@[noinline] def Dec.fillNnzCbp (d : Dec) (t topT leftT : UInt32) : Dec :=
  let nz := d.nnzC
  let d := { d with nnzC := .empty }
  let nz := nnzCacheIn d.pic.nnzTable nz (topT != 0) (48 * d.topXY) (leftT != 0) (48 * d.leftXY)
    (if !isIntra t then 0 else 64)
  let topCbp := if topT != 0 then getU16 d.pic.cbpTable d.topXY else if isIntra t then 0x7CF else 0x00F
  let leftCbp :=
    if leftT != 0 then
      let c := getU16 d.pic.cbpTable d.leftXY
      (c &&& 0x7F0) ||| (c &&& 2) ||| (c &&& 8)
    else if isIntra t then 0x7CF else 0x00F
  { d with nnzC := nz, topCbp, leftCbp }

@[noinline] def Dec.setNeighborTransformSize (d : Dec) (n : Nat) : Dec := { d with neighborTransformSize := n }

/-- `fill_decode_caches` for macroblock type `t`. -/
def fillCaches (d : Dec) (t : UInt32) : Dec :=
  let topT := d.topType
  let leftT := d.leftType
  let d := if isSkip t then d else (if isIntra t then d.fillIntraCaches t topT leftT else d).fillNnzCbp t topT leftT
  let d :=
    if isInter t || (isDirect t && d.sl.directSpatial) then
      let d := if usesList t 0 then fillMotionList d t topT leftT 0 else d
      if d.sl.listCount == 2 && usesList t 1 then fillMotionList d t topT leftT 1 else d
    else d
  d.setNeighborTransformSize ((if is8x8dct topT then 1 else 0) + (if is8x8dct leftT then 1 else 0))

/-- `pred_intra_mode`: the predicted intra 4x4/8x8 mode of block `n`. -/
def predIntraMode (d : Dec) (n : Nat) : Nat :=
  let i := s8 n
  let m := min (getI8 d.i4C (i - 1)) (getI8 d.i4C (i - 8))
  if m < 0 then 2 else m.toNat

-- The write-backs take each table out of the state before changing it: a
-- table still referenced by the state would be copied on every write.

def writeBackIntraModes (d : Dec) : Dec := Id.run do
  let mut t := d.pic.i4Table
  let d := { d with pic := { d.pic with i4Table := .empty } }
  let b := 8 * d.mbXY
  for k in [0:4] do t := t.set! (b + k) d.i4C[36 + k]!
  t := ((t.set! (b + 4) d.i4C[31]!).set! (b + 5) d.i4C[23]!).set! (b + 6) d.i4C[15]!
  return { d with pic := { d.pic with i4Table := t } }

/-- The nnz cache rows into the table at `b` (FFmpeg's non_zero_count layout). -/
def nnzOut (nz t : ByteArray) (b : Nat) : ByteArray :=
  let t := Kernels.copyRows t b 4 nz 12 8 4 4        -- luma rows 12, 20, 28, 36
  let t := Kernels.copyRows t (b + 16) 4 nz 52 8 4 2 -- Cb 52, 60
  Kernels.copyRows t (b + 32) 4 nz 92 8 4 2          -- Cr 92, 100

def writeBackNnz (d : Dec) : Dec :=
  let t := d.pic.nnzTable
  let d := { d with pic := { d.pic with nnzTable := .empty } }
  { d with pic := { d.pic with nnzTable := nnzOut d.nnzC t (48 * d.mbXY) } }

/-- The cache's 4x4 motion vectors (rows at entries 12, 20, 28, 36) into
the table at block `b` (row stride `bs` blocks). -/
@[inline] def mvRowsOut (mc mv : ByteArray) (b bs : Nat) : ByteArray :=
  Kernels.copyRows mv (4 * b) (4 * bs) mc (4 * 12) 32 16 4

/-- The mvd_table entries of macroblock `xy` (bottom row, then the right
column bottom up, as FFmpeg keeps them), or zeros for a skipped one. -/
def mvdOut (dc md : ByteArray) (xy : Nat) (skip : Bool) : ByteArray :=
  if skip then zeroBytes md (16 * xy) 16
  else
    let md := dc.copySlice (2 * 36) md (16 * xy) 8
    copyMvd (copyMvd (copyMvd md (8 * xy + 6) dc 15) (8 * xy + 5) dc 23) (8 * xy + 4) dc 31

/-- The four 8x8 reference indices of the cache into the table at `b8`. -/
@[inline] def refsOut (rc rt : ByteArray) (b8 : Nat) : ByteArray :=
  (((rt.set! b8 rc[12]!).set! (b8 + 1) rc[14]!).set! (b8 + 2) rc[28]!).set! (b8 + 3) rc[30]!

/-- `write_back_motion` of one list (the tables taken out of the state first). -/
@[noinline] def Dec.setMotionT0 (d : Dec) (mv md rt : ByteArray) : Dec :=
  { d with pic := { d.pic with mv0 := mv, mvd0 := md, ref0 := rt } }
@[noinline] def Dec.setMotionT1 (d : Dec) (mv md rt : ByteArray) : Dec :=
  { d with pic := { d.pic with mv1 := mv, mvd1 := md, ref1 := rt } }

/-- The cache's motion vectors (rows at entries 12, 20, 28, 36) into the
table at macroblock `xy`. -/
@[inline] def mvRowsOutU (mc mv : ByteArray) (xy : USize) : ByteArray := Kernels.copyRowsRaw mv (64 * xy) 16 mc 48 32 16 4

/-- The mvd table entries of macroblock `xy` (bottom row, then the right
column bottom up, as FFmpeg keeps them), or zeros for a skipped one. -/
@[inline] def mvdOutU (dc md : ByteArray) (xy : USize) (skip : Bool) : ByteArray :=
  let o := 16 * xy
  if skip then Kernels.fillRaw md o 0 4 1 0 4
  else
    let md := Kernels.copyRowsRaw md o 0 dc 72 0 8 1
    u16set (u16set (u16set md (o + 12) (u16 dc 30)) (o + 10) (u16 dc 46)) (o + 8) (u16 dc 62)

/-- The four 8x8 reference indices of the cache into the table at macroblock `xy`. -/
@[inline] def refsOutU (rc rt : ByteArray) (xy : USize) : ByteArray :=
  let b8 := 4 * xy
  u8set (u8set (u8set (u8set rt b8 (u8 rc 12)) (b8 + 1) (u8 rc 14)) (b8 + 2) (u8 rc 28)) (b8 + 3) (u8 rc 30)

def writeBackList0 (d : Dec) (t : UInt32) : Dec :=
  let (mv, md, rt) := (d.pic.mv0, d.pic.mvd0, d.pic.ref0)
  let d := d.setMotionT0 .empty .empty .empty
  let xy := d.mbXY.toUSize
  d.setMotionT0 (mvRowsOutU d.mvC0 mv xy) (mvdOutU d.mvdC0 md xy (isSkip t)) (refsOutU d.refC0 rt xy)

def writeBackList1 (d : Dec) (t : UInt32) : Dec :=
  let (mv, md, rt) := (d.pic.mv1, d.pic.mvd1, d.pic.ref1)
  let d := d.setMotionT1 .empty .empty .empty
  let xy := d.mbXY.toUSize
  d.setMotionT1 (mvRowsOutU d.mvC1 mv xy) (mvdOutU d.mvdC1 md xy (isSkip t)) (refsOutU d.refC1 rt xy)

@[inline] def writeBackList (d : Dec) (t : UInt32) (list : Nat) : Dec :=
  if list == 0 then writeBackList0 d t else writeBackList1 d t

/-- List 0 unused by the macroblock at `b8 = 4 * mbXY`: its references
`LIST_NOT_USED`. (Small setters keep the state's update in place.) -/
@[noinline] def Dec.ref0Unused (d : Dec) (b8 : Nat) : Dec :=
  let rt := d.pic.ref0
  let d := { d with pic := { d.pic with ref0 := .empty } }
  let v := toByte LIST_NOT_USED
  { d with pic := { d.pic with ref0 := (((rt.set! b8 v).set! (b8 + 1) v).set! (b8 + 2) v).set! (b8 + 3) v } }

/-- A B_8x8 macroblock's direct table entries for sub-macroblocks 1..3. -/
@[noinline] def Dec.storeDirect (d : Dec) (b8 : Nat) (s1 s2 s3 : UInt8) : Dec :=
  let dt := d.pic.directTable
  let d := { d with pic := { d.pic with directTable := .empty } }
  { d with pic := { d.pic with directTable := ((dt.set! (b8 + 1) s1).set! (b8 + 2) s2).set! (b8 + 3) s3 } }

/-- `write_back_motion` for macroblock type `t`. -/
def writeBackMotion (d : Dec) (t : UInt32) : Dec :=
  let b8 := 4 * d.mbXY
  let d := if usesList t 0 then writeBackList d t 0 else d.ref0Unused b8
  let d := if usesList t 1 then writeBackList d t 1 else d
  if d.sl.sliceType == .b && t &&& MB_8x8 != 0 then
    d.storeDirect b8 (d.sub 1 >>> 1).toUInt8 (d.sub 2 >>> 1).toUInt8 (d.sub 3 >>> 1).toUInt8
  else d

/-- `fetch_diagonal_mv`: (reference, cache entry of C). -/
def fetchDiagonal (d : Dec) (i list partWidth : Nat) : Int × Nat :=
  let rc := d.refC list
  let tr := getI8 rc (i - 8 + partWidth)
  if tr != PART_NOT_AVAILABLE then (tr, i - 8 + partWidth) else (getI8 rc (i - 8 - 1), i - 8 - 1)

/-- `pred_motion`: the predicted motion vector of block `n` (width
`partWidth` 4x4 blocks) for `ref`. -/
def predMotion (d : Dec) (n partWidth list : Nat) (ref : Int) : Int × Int :=
  let i := s8 n
  let rc := d.refC list
  let mc := d.mvC list
  let topRef := getI8 rc (i - 8)
  let leftRef := getI8 rc (i - 1)
  let (diagRef, ce) := fetchDiagonal d i list partWidth
  let a := i - 1
  let b := i - 8
  let nMatch := (if diagRef == ref then 1 else 0) + (if topRef == ref then 1 else 0) + (if leftRef == ref then 1 else 0)
  let median := (mid3 (mvX mc a) (mvX mc b) (mvX mc ce), mid3 (mvY mc a) (mvY mc b) (mvY mc ce))
  if nMatch > 1 then median
  else if nMatch == 1 then
    if leftRef == ref then (mvX mc a, mvY mc a)
    else if topRef == ref then (mvX mc b, mvY mc b)
    else (mvX mc ce, mvY mc ce)
  else if topRef == PART_NOT_AVAILABLE && diagRef == PART_NOT_AVAILABLE && leftRef != PART_NOT_AVAILABLE then
    (mvX mc a, mvY mc a)
  else median

def pred16x8Motion (d : Dec) (n list : Nat) (ref : Int) : Int × Int :=
  let rc := d.refC list
  let mc := d.mvC list
  if n == 0 then
    let e := s8 0 - 8
    if getI8 rc e == ref then (mvX mc e, mvY mc e) else predMotion d n 4 list ref
  else
    let e := s8 8 - 1
    if getI8 rc e == ref then (mvX mc e, mvY mc e) else predMotion d n 4 list ref

def pred8x16Motion (d : Dec) (n list : Nat) (ref : Int) : Int × Int :=
  let rc := d.refC list
  let mc := d.mvC list
  if n == 0 then
    let e := s8 0 - 1
    if getI8 rc e == ref then (mvX mc e, mvY mc e) else predMotion d n 2 list ref
  else
    let (diag, ce) := fetchDiagonal d (s8 4) list 2
    if diag == ref then (mvX mc ce, mvY mc ce) else predMotion d n 2 list ref

/-- One P_Skip neighbour (left or top): `none` when the skip vector is
zero because of it (unavailable, or reference 0 with a zero vector). -/
def pskipNeighbour (d : Dec) (t : UInt32) (r : Int) (b : Nat) : Option (Int × Int × Int) :=
  if usesList t 0 then
    let x := mvX d.pic.mv0 b
    let y := mvY d.pic.mv0 b
    if r == 0 && x == 0 && y == 0 then none else some (r, x, y)
  else if t != 0 then some (LIST_NOT_USED, 0, 0)
  else none

/-- `pred_pskip_motion`: the P_Skip motion vector, into the list 0 caches. -/
def predPskip (d : Dec) : Dec :=
  let d := d.fillRefC 0 12 4 4 0
  let bs := d.bStride
  let left := pskipNeighbour d d.leftType (if d.leftType != 0 then getI8 d.pic.ref0 (4 * d.leftXY + 1) else 0) (if d.leftType != 0 then d.mb2b d.leftXY + 3 else 0)
  let top := pskipNeighbour d d.topType (if d.topType != 0 then getI8 d.pic.ref0 (4 * d.topXY + 2) else 0) (if d.topType != 0 then d.mb2b d.topXY + 3 * bs else 0)
  match left, top with
  | some (leftRef, ax, ay), some (topRef, bx, byy) =>
    let (diagRef, cx, cy) : Int × Int × Int :=
      if usesList d.toprightType 0 then
        let b := d.mb2b d.toprightXY + 3 * bs
        (getI8 d.pic.ref0 (4 * d.toprightXY + 2), mvX d.pic.mv0 b, mvY d.pic.mv0 b)
      else if d.toprightType != 0 then (LIST_NOT_USED, 0, 0)
      else if usesList d.topleftType 0 then
        let b := d.mb2b d.topleftXY + 3 + 3 * bs
        (getI8 d.pic.ref0 (4 * d.topleftXY + 3), mvX d.pic.mv0 b, mvY d.pic.mv0 b)
      else if d.topleftType != 0 then (LIST_NOT_USED, 0, 0)
      else (PART_NOT_AVAILABLE, 0, 0)
    let nMatch := (if diagRef == 0 then 1 else 0) + (if topRef == 0 then 1 else 0) + (if leftRef == 0 then 1 else 0)
    let (mx, my) :=
      if nMatch == 1 then
        if leftRef == 0 then (ax, ay) else if topRef == 0 then (bx, byy) else (cx, cy)
      else (mid3 ax bx cx, mid3 ay byy cy)
    d.fillMvC 0 12 4 4 mx my
  | _, _ => d.fillMvC 0 12 4 4 0 0

end Yt.H264
