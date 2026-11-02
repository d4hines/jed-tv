/-
The deblocking filter over a decoded picture (FFmpeg h264_loopfilter.c
`ff_h264_filter_mb`/`filter_mb_dir` and h264_slice.c `fill_filter_caches`,
for frame macroblocks of 8-bit 4:2:0 video). Part of the Lean H.264 decoder
(follows FFmpeg 9.0.1, LGPL-2.1-or-later).

Runs after the whole picture is reconstructed, macroblocks in raster order,
each with its own slice's parameters.
-/
import Yt.H264.Caches
import Yt.H264.Filter
import Yt.H264.Kernels

namespace Yt.H264

open Yt.Bytes (u8 u16 u32)

-- Positions and table indices are USize here: Nat arithmetic costs
-- overflow checks and boxing at every step.

/-- Byte `i` of a packed quadruple. -/
@[inline] def byteAt (q : UInt32) (i : UInt32) : USize := ((q >>> (8 * i)) &&& 0xFF).toUSize

/-- The four segments' tC0' at indexA `ia` for their bS (bytes of `bS`),
plus `add` (1 for chroma), as int8 bytes. -/
def packTc (ia : USize) (bS : UInt32) (add : UInt8) : USize :=
  let t := Tables.tc0
  (u8 t (4 * ia + byteAt bS 0) + add).toUSize ||| ((u8 t (4 * ia + byteAt bS 1) + add).toUSize <<< 8) |||
    ((u8 t (4 * ia + byteAt bS 2) + add).toUSize <<< 16) ||| ((u8 t (4 * ia + byteAt bS 3) + add).toUSize <<< 24)

/-- `q + o` clipped to 0..51 (`o`: a slice's filter offset as 32-bit two's complement). -/
@[inline] def clipIdx (q : USize) (o : UInt32) : USize :=
  let v := q.toUInt32 + o
  if v >>> 31 != 0 then 0 else if v > 51 then 51 else v.toUSize

/-- Filter one chroma plane's edge (`filter_mb_edgec{v,h}`). -/
def filterChroma (d : Dec) (cr : Bool) (chromaOff cxs cys : USize) (bS : UInt32) (q : USize) (ao bo : UInt32)
    (intra : Bool) : Dec :=
  let ca := clipIdx q ao
  let calpha := u8 Tables.alpha ca
  let cbeta := u8 Tables.beta (clipIdx q bo)
  if calpha == 0 || cbeta == 0 then d
  else
    -- taken out of `d` so the filter writes it in place
    let plane := if cr then d.pic.cr else d.pic.cb
    let d := if cr then d.withCr .empty else d.withCb .empty
    let plane :=
      if bS &&& 0xFF < 4 || !intra then
        Kernels.loopFilter plane chromaOff (if cxs == 1 then cys else cxs) calpha.toUSize cbeta.toUSize
          (packTc ca bS 1) (if cxs == 1 then 0 else 1) 1
      else Kernels.intraFilter plane chromaOff cxs cys calpha.toUSize cbeta.toUSize 1
    if cr then d.withCr plane else d.withCb plane

/-- Filter one luma and the two chroma edges (`filter_mb_edge{v,h}` and
`filter_mb_edgec{v,h}`); `bS`: the four segments' strengths, one per byte.
`dir` 0: vertical edge (filtering across x). -/
def filterEdge (d : Dec) (dir lumaOff chromaOff : USize) (doChroma : Bool) (bS : UInt32) (qp cqp0 cqp1 : USize)
    (ao bo : UInt32) (intra : Bool) : Dec :=
  let stride := d.stride.toUSize
  let cstride := d.cstride.toUSize
  let ia := clipIdx qp ao
  let alpha := u8 Tables.alpha ia
  let beta := u8 Tables.beta (clipIdx qp bo)
  let d :=
    if alpha == 0 || beta == 0 then d
    else
      let y := d.pic.y
      let d := d.withY .empty
      let y :=
        if bS &&& 0xFF < 4 || !intra then
          Kernels.loopFilter y lumaOff stride alpha.toUSize beta.toUSize (packTc ia bS 0) dir 0
        else if dir == 0 then Kernels.intraFilter y lumaOff 1 stride alpha.toUSize beta.toUSize 0
        else Kernels.intraFilter y lumaOff stride 1 alpha.toUSize beta.toUSize 0
      d.withY y
  if doChroma then
    let (cxs, cys) := if dir == 0 then ((1 : USize), cstride) else (cstride, 1)
    let d := filterChroma d false chromaOff cxs cys bS cqp0 ao bo intra
    filterChroma d true chromaOff cxs cys bS cqp1 ao bo intra
  else d

/-! The boundary strengths of a macroblock come from Kernels.mbBs (straight
from the picture's tables: FFmpeg copies them into filter caches first; a
list a macroblock does not use keeps the tables' initial -1 references and
zero vectors, which is what those caches hold). -/

/-- The reference maps of the picture's slices for Kernels.mbBs: per
slice, `refMap0` then `refMap1` (128 bytes each). -/
def sliceMaps (d : @& Dec) : ByteArray :=
  d.sl.slices.foldl (fun b s => b ++ s.refMap0 ++ s.refMap1) (.emptyWithCapacity (256 * d.sl.slices.size))

/-- The slice of the neighbour of a macroblock in slice `s` (deblocking
mode `deblock`) at `nxy` when it exists (`avail`) and the filter crosses
into it, else 0xFFFF. -/
@[inline] def crossSlice (d : @& Dec) (avail : Bool) (nxy : USize) (s : UInt32) (deblock : Nat) : UInt32 :=
  if !avail then 0xFFFF
  else
    let ns := u16 d.pic.sliceTable (2 * nxy)
    if (if deblock == 2 then ns == s else ns != 0xFFFF) then ns else 0xFFFF

/-- The boundary strengths of macroblock `xy` (slice `s`, `w` macroblocks a
row) into `bs` (Kernels.mbBs: direction d's edge e at bytes 16 d + 4 e; 0:
not filtered). -/
def mbBsAt (d : @& Dec) (bs maps : ByteArray) (xy w : USize) (s : UInt32) (info : @& SliceInfo) : ByteArray :=
  let ls := crossSlice d (xy % w > 0) (xy - 1) s info.deblock
  let ts := crossSlice d (xy ≥ w) (xy - w) s info.deblock
  let types := d.pic.mbType
  let cbps := d.pic.cbpTable
  let leftT := if ls != 0xFFFF then u32 types (4 * (xy - 1)) else 0
  let topT := if ts != 0xFFFF then u32 types (4 * (xy - w)) else 0
  let cbpL : UInt32 := if ls != 0xFFFF then u16 cbps (2 * (xy - 1)) &&& 15 else 0
  let cbpT : UInt32 := if ts != 0xFFFF then u16 cbps (2 * (xy - w)) &&& 15 else 0
  let cbp : UInt32 := (u16 cbps (2 * xy) &&& 15) ||| (cbpL <<< (4 : UInt32)) ||| (cbpT <<< (8 : UInt32))
  Kernels.mbBsRaw bs d.pic.nnzTable d.pic.ref0 d.pic.ref1 d.pic.mv0 d.pic.mv1 maps xy w (u32 types (4 * xy)).toUSize
    leftT.toUSize topT.toUSize s.toUSize (if ls != 0xFFFF then ls else 0).toUSize (if ts != 0xFFFF then ts else 0).toUSize
    (if info.listCount == 2 then 1 else 0) cbp.toUSize

/-- An internal edge `edge` (1..3) of the macroblock with strengths `bS`. -/
@[inline] def innerEdge (d : Dec) (dir edge : USize) (bS : UInt32) (ao bo : UInt32) (lumaBase chromaBase qp c0 c1 : USize) :
    Dec :=
  if bS == 0 then d
  else
    let lo := if dir == 0 then lumaBase + 4 * edge else lumaBase + 4 * edge * d.stride.toUSize
    let co := if dir == 0 then chromaBase + 2 * edge else chromaBase + 2 * edge * d.cstride.toUSize
    filterEdge d dir lo co (edge &&& 1 == 0) bS qp c0 c1 ao bo false

/-- `filter_mb_dir` for frame macroblock `xy` at (x, y), its strengths in
`bs`; `mbmXY`: the neighbour across edge 0. -/
def filterMbDir (d : Dec) (bs : @& ByteArray) (x y xy mbmXY : USize) (info : @& SliceInfo) (dir : USize) : Dec :=
  let lumaBase := 16 * y * d.stride.toUSize + 16 * x
  let chromaBase := 8 * y * d.cstride.toUSize + 8 * x
  let qpCur := (u8 d.pic.qscaleTable xy).toUSize
  let cqp0 := (u8 info.cqpTab0 qpCur).toUSize
  let cqp1 := (u8 info.cqpTab1 qpCur).toUSize
  let ao := info.alphaOffset.toInt32.toUInt32
  let bo := info.betaOffset.toInt32.toUInt32
  let bS := u32 bs (16 * dir)
  let d :=
    if bS == 0 then d
    else
      let qpN := (u8 d.pic.qscaleTable mbmXY).toUSize
      let qp := (qpCur + qpN + 1) >>> 1
      let c0 := (cqp0 + (u8 info.cqpTab0 qpN).toUSize + 1) >>> 1
      let c1 := (cqp1 + (u8 info.cqpTab1 qpN).toUSize + 1) >>> 1
      filterEdge d dir lumaBase chromaBase true bS qp c0 c1 ao bo true
  let d := innerEdge d dir 1 (u32 bs (16 * dir + 4)) ao bo lumaBase chromaBase qpCur cqp0 cqp1
  let d := innerEdge d dir 2 (u32 bs (16 * dir + 8)) ao bo lumaBase chromaBase qpCur cqp0 cqp1
  innerEdge d dir 3 (u32 bs (16 * dir + 12)) ao bo lumaBase chromaBase qpCur cqp0 cqp1

/-- Position the state at macroblock `xy` of slice `s` for filtering. -/
@[noinline] def Dec.atMb (d : Dec) (x y xy s : Nat) : Dec := { d with mbX := x, mbY := y, mbXY := xy, sliceNum := s }

/-- Deblock the whole picture. -/
def deblockPicture (d : Dec) : Dec := go d (Yt.Bytes.zeros 32) (sliceMaps d) 0 (d.mbW * d.mbH)
where
  go (d : Dec) (bs maps : ByteArray) (i n : Nat) : Dec :=
    if i < n then
      let xy := i.toUSize
      let s := u16 d.pic.sliceTable (2 * xy)
      if h : s.toNat < d.sl.slices.size then
        let info := d.sl.slices[s.toNat]
        if info.deblock == 0 then go d bs maps (i + 1) n
        else
          let w := d.mbW.toUSize
          let bs := mbBsAt d bs maps xy w s info
          let x := xy % w
          let y := xy / w
          let d := d.atMb x.toNat y.toNat i s.toNat
          let d := filterMbDir d bs x y xy (xy - 1) info 0
          go (filterMbDir d bs x y xy (xy - w) info 1) bs maps (i + 1) n
      else go d bs maps (i + 1) n
    else d
  termination_by n - i

end Yt.H264
