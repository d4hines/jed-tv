/-
The H.264 decoding state: per-picture tables and the per-macroblock caches
in FFmpeg's "scan8" layout (h264dec.h, h264_mvpred.h), which the CABAC
syntax decoding, motion vector prediction and reconstruction share. Part of
the Lean H.264 decoder (follows FFmpeg 9.0.1, LGPL-2.1-or-later).

Signed small values live in ByteArrays as two's complement bytes (int8) or
little-endian pairs (int16); the helpers below read and write them.
-/
import Yt.H264.Cabac
import Yt.H264.Slice
import Yt.Bytes
import Yt.H264.Kernels

namespace Yt.H264
open Yt.Bytes (u8 u16 u32 u8set)

/-! ## Macroblock type flags (FFmpeg mpegutils.h, h264_parse.h) -/

def MB_INTRA4x4 : UInt32 := 0x1
def MB_INTRA16x16 : UInt32 := 0x2
def MB_INTRA_PCM : UInt32 := 0x4
def MB_16x16 : UInt32 := 0x8
def MB_16x8 : UInt32 := 0x10
def MB_8x16 : UInt32 := 0x20
def MB_8x8 : UInt32 := 0x40
def MB_INTERLACED : UInt32 := 0x80
def MB_DIRECT2 : UInt32 := 0x100
def MB_REF0 : UInt32 := 0x200
def MB_P0L0 : UInt32 := 0x1000
def MB_P1L0 : UInt32 := 0x2000
def MB_P0L1 : UInt32 := 0x4000
def MB_P1L1 : UInt32 := 0x8000
def MB_L0 : UInt32 := MB_P0L0 ||| MB_P1L0
def MB_L1 : UInt32 := MB_P0L1 ||| MB_P1L1
def MB_L0L1 : UInt32 := MB_L0 ||| MB_L1
def MB_SKIP : UInt32 := 0x20000
def MB_8x8DCT : UInt32 := 0x01000000

@[inline] def isIntra (t : UInt32) : Bool := t &&& 7 != 0
@[inline] def isIntra4x4 (t : UInt32) : Bool := t &&& MB_INTRA4x4 != 0
@[inline] def isIntra16x16 (t : UInt32) : Bool := t &&& MB_INTRA16x16 != 0
@[inline] def isPcm (t : UInt32) : Bool := t &&& MB_INTRA_PCM != 0
@[inline] def isInter (t : UInt32) : Bool := t &&& (MB_16x16 ||| MB_16x8 ||| MB_8x16 ||| MB_8x8) != 0
@[inline] def isSkip (t : UInt32) : Bool := t &&& MB_SKIP != 0
@[inline] def isDirect (t : UInt32) : Bool := t &&& MB_DIRECT2 != 0
@[inline] def is8x8dct (t : UInt32) : Bool := t &&& MB_8x8DCT != 0
@[inline] def usesList (t : UInt32) (list : Nat) : Bool := t &&& ((MB_P0L0 ||| MB_P1L0) <<< (2 * list).toUInt32) != 0
@[inline] def isDir (t : UInt32) (part list : Nat) : Bool := t &&& (MB_P0L0 <<< (part + 2 * list).toUInt32) != 0

def PART_NOT_AVAILABLE : Int := -2
def LIST_NOT_USED : Int := -1
def LUMA_DC_BLOCK_INDEX : Nat := 48
def CHROMA_DC_BLOCK_INDEX : Nat := 49

/-- FFmpeg's scan8: the cache index of each block (luma 0..15, Cb 16..31,
Cr 32..47, then the DC entries). -/
def scan8 : ByteArray := ⟨#[
  4 + 1 * 8, 5 + 1 * 8, 4 + 2 * 8, 5 + 2 * 8, 6 + 1 * 8, 7 + 1 * 8, 6 + 2 * 8, 7 + 2 * 8,
  4 + 3 * 8, 5 + 3 * 8, 4 + 4 * 8, 5 + 4 * 8, 6 + 3 * 8, 7 + 3 * 8, 6 + 4 * 8, 7 + 4 * 8,
  4 + 6 * 8, 5 + 6 * 8, 4 + 7 * 8, 5 + 7 * 8, 6 + 6 * 8, 7 + 6 * 8, 6 + 7 * 8, 7 + 7 * 8,
  4 + 8 * 8, 5 + 8 * 8, 4 + 9 * 8, 5 + 9 * 8, 6 + 8 * 8, 7 + 8 * 8, 6 + 9 * 8, 7 + 9 * 8,
  4 + 11 * 8, 5 + 11 * 8, 4 + 12 * 8, 5 + 12 * 8, 6 + 11 * 8, 7 + 11 * 8, 6 + 12 * 8, 7 + 12 * 8,
  4 + 13 * 8, 5 + 13 * 8, 4 + 14 * 8, 5 + 14 * 8, 6 + 13 * 8, 7 + 13 * 8, 6 + 14 * 8, 7 + 14 * 8,
  0 + 0 * 8, 0 + 5 * 8, 0 + 10 * 8]⟩

@[inline] def s8 (n : Nat) : Nat := scan8[n]!.toNat

/-- Position (x, y) in 4x4 units of luma block `n` (FFmpeg block order). -/
@[inline] def blkX (n : Nat) : Nat := (n &&& 1) ||| ((n >>> 1) &&& 2)
@[inline] def blkY (n : Nat) : Nat := ((n >>> 1) &&& 1) ||| ((n >>> 2) &&& 2)

/-! ## Byte helpers -/

-- Signed values go through the fixed-width types (two's complement): `Int`
-- `%` takes the runtime's 64-bit division path on 32-bit ARM.
@[inline] def getI8 (b : ByteArray) (i : Nat) : Int := b[i]!.toInt8.toInt
@[inline] def toByte (v : Int) : UInt8 := v.toInt8.toUInt8
@[inline] def setI8 (b : ByteArray) (i : Nat) (v : Int) : ByteArray := b.set! i (toByte v)
-- Multi-byte reads check bounds once (then unchecked byte reads); 0 past the end.
@[inline] def getU16 (b : ByteArray) (i : Nat) : Nat :=
  let o := 2 * i
  -- assembled in UInt32: a Nat shift is a runtime call
  if h : o + 1 < b.size then (b[o].toUInt32 ||| (b[o + 1].toUInt32 <<< 8)).toNat else 0
@[inline] def setU16 (b : ByteArray) (i v : Nat) : ByteArray :=
  let u := v.toUInt16
  (b.set! (2 * i) u.toUInt8).set! (2 * i + 1) (u >>> 8).toUInt8
@[inline] def getI16 (b : ByteArray) (i : Nat) : Int :=
  let o := 2 * i
  if h : o + 1 < b.size then (b[o].toUInt16 ||| (b[o + 1].toUInt16 <<< 8)).toInt16.toInt else 0
@[inline] def setI16 (b : ByteArray) (i : Nat) (v : Int) : ByteArray :=
  let u := v.toInt16.toUInt16
  (b.set! (2 * i) u.toUInt8).set! (2 * i + 1) (u >>> 8).toUInt8
@[inline] def getU32 (b : ByteArray) (i : Nat) : UInt32 :=
  let o := 4 * i
  if h : o + 3 < b.size then
    b[o].toUInt32 ||| (b[o + 1].toUInt32 <<< 8) ||| (b[o + 2].toUInt32 <<< 16) ||| (b[o + 3].toUInt32 <<< 24)
  else 0
@[inline] def setU32 (b : ByteArray) (i : Nat) (v : UInt32) : ByteArray :=
  (((b.set! (4 * i) v.toUInt8).set! (4 * i + 1) (v >>> 8).toUInt8).set! (4 * i + 2) (v >>> 16).toUInt8).set! (4 * i + 3) (v >>> 24).toUInt8

-- The fills are recursions, not `for` loops: each `for x in [0:w]` builds a
-- range object, so nested loops allocate on every row.

def zeros64 : ByteArray := Bytes.filled 64 0

/-- Eight copies of each byte value: rows for `fillRow`. -/
def bytePatterns : ByteArray := Id.run do
  let mut b := ByteArray.emptyWithCapacity 2048
  for v in [0:256] do
    for _ in [0:8] do b := b.push v.toUInt8
  return b

-- The fills are Kernels.fill: direct stores on the TV.

/-- `w` byte entries of `v` from byte `o`. -/
@[inline] def fillRow (b : ByteArray) (o : Nat) (v : UInt8) (w : Nat) : ByteArray :=
  Kernels.fill b o 0 w 1 1 v.toUInt32

/-- `fill_rectangle` over a byte cache with an 8-wide stride. -/
@[inline] def fillRect (b : ByteArray) (start w h : Nat) (v : UInt8) : ByteArray :=
  Kernels.fill b start 8 w h 1 v.toUInt32

/-- `fill_rectangle` over a cache of 2-byte entries (mvd). -/
@[inline] def fillRect16 (b : ByteArray) (start w h : Nat) (lo hi : UInt8) : ByteArray :=
  Kernels.fill b (2 * start) 16 w h 2 (lo.toUInt32 ||| (hi.toUInt32 <<< 8))

/-- `fill_rectangle` of a motion vector (cache of 4-byte entries: x, y int16). -/
@[inline] def fillMv (b : ByteArray) (start w h : Nat) (mx my : Int) : ByteArray :=
  Kernels.fill b (4 * start) 32 w h 4 (mx.toInt16.toUInt16.toUInt32 ||| (my.toInt16.toUInt16.toUInt32 <<< 16))

/-- `n` zero bytes at byte `o` (words or halfwords when aligned). -/
@[inline] def zeroBytes (b : ByteArray) (o n : Nat) : ByteArray :=
  if (o ||| n) &&& 3 == 0 then Kernels.fill b o 0 (n / 4) 1 4 0
  else if (o ||| n) &&& 1 == 0 then Kernels.fill b o 0 (n / 2) 1 2 0
  else Kernels.fill b o 0 n 1 1 0

@[inline] def mvX (b : ByteArray) (e : Nat) : Int := getI16 b (2 * e)
@[inline] def mvY (b : ByteArray) (e : Nat) : Int := getI16 b (2 * e + 1)

/-- The (sorted) middle of three. -/
@[inline] def mid3 (a b c : Int) : Int := max (min a b) (min (max a b) c)

def chromaQpTable : ByteArray := ⟨#[
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
  29, 30, 31, 32, 32, 33, 34, 34, 35, 35, 36, 36, 37, 37, 37, 38, 38, 38, 39, 39, 39, 39]⟩

/-- QPc for luma QP `qp` and chroma_qp_index_offset `off` (Table 8-15). -/
def chromaQp (off : Int) (qp : Nat) : Nat := chromaQpTable[(max 0 (min 51 (Int.ofNat qp + off))).toNat]!.toNat

/-- QPc for every luma QP 0..51 at offset `off`: per-slice lookups without `Int`. -/
def chromaQpTab (off : Int) : ByteArray := Id.run do
  let mut b := ByteArray.emptyWithCapacity 52
  for q in [0:52] do b := b.push (chromaQp off q).toUInt8
  return b

/-- Int16 entry `k` of `b`, sign-extended into a UInt32. -/
@[inline] def s16 (b : ByteArray) (k : Nat) : UInt32 :=
  let o := 2 * k
  if h : o + 1 < b.size then ((b[o].toUInt32 ||| (b[o + 1].toUInt32 <<< 8)) ^^^ 0x8000) - 0x8000 else 0

/-! ## The state -/

/-- One reference picture as the macroblock layer sees it: its planes, the
motion it was coded with (for direct prediction), and its identity. -/
structure RefPic where
  y : ByteArray
  cb : ByteArray
  cr : ByteArray
  poc : Int
  longTerm : Bool := false
  /-- Index in the decoder's frame store (identity across lists). -/
  id : Nat
  mbType : ByteArray
  mv0 : ByteArray
  mv1 : ByteArray
  ref0 : ByteArray
  ref1 : ByteArray
  frameNum : Nat := 0
  /-- FFmpeg's ref_poc of its last slice: 4 * frame_num + 3 of each of its
  references, per list (temporal direct maps the colocated references by it). -/
  refPoc0 : Array Int := #[]
  refPoc1 : Array Int := #[]
  deriving Inhabited

/-- What the loop filter needs of each slice (FFmpeg keeps these in the
slice context and ref2frm). -/
structure SliceInfo where
  /-- FFmpeg's deblocking_filter: 0 off, 1 on, 2 on but not across slice edges. -/
  deblock : Nat := 1
  alphaOffset : Int := 0
  betaOffset : Int := 0
  listCount : Nat := 0
  /-- The DPB index of each reference index's picture, per list. -/
  refIds0 : Array Nat := #[]
  refIds1 : Array Nat := #[]
  cqpOff0 : Int := 0
  cqpOff1 : Int := 0
  /-- `chromaQpTab cqpOff0`, `chromaQpTab cqpOff1`. -/
  cqpTab0 : ByteArray := .empty
  cqpTab1 : ByteArray := .empty
  /-- `refIds0`/`refIds1` as 128-byte maps from a stored reference index byte
  to the picture identity (0xFF: none), for the loop filter. -/
  refMap0 : ByteArray := Bytes.filled 128 0xFF
  refMap1 : ByteArray := Bytes.filled 128 0xFF
  deriving Inhabited

/-- The loop filter's map of reference index → picture identity. -/
def refMapOf (ids : Array Nat) : ByteArray := Id.run do
  let mut b := Bytes.filled 128 0xFF
  for k in [0:min ids.size 128] do b := b.set! k ids[k]!.toUInt8
  return b

def refCacheInit : ByteArray :=
  ⟨(Array.replicate 40 (0 : UInt8)).set! 16 0xFE |>.set! 24 0xFE |>.set! 32 0xFE⟩

/-- The picture being decoded: its planes and per-macroblock tables (a
record of its own so that copying or rewriting the decoder state touches
fewer fields). -/
structure Pic where
  y : ByteArray
  cb : ByteArray
  cr : ByteArray
  mbType : ByteArray        -- 4 bytes per MB
  sliceTable : ByteArray    -- 2 bytes per MB, 0xFFFF: not decoded
  cbpTable : ByteArray      -- 2 bytes per MB
  chromaPredTable : ByteArray
  qscaleTable : ByteArray
  nnzTable : ByteArray      -- 48 per MB (FFmpeg non_zero_count layout)
  i4Table : ByteArray       -- 8 per MB (FFmpeg intra4x4_pred_mode layout)
  mv0 : ByteArray           -- 4 bytes per 4x4 block, 16 blocks per MB (Dec.mb2b)
  mv1 : ByteArray
  ref0 : ByteArray          -- 4 per MB (8x8 blocks), int8
  ref1 : ByteArray
  mvd0 : ByteArray          -- 8 entries x 2 bytes per MB (FFmpeg mvd_table)
  mvd1 : ByteArray
  directTable : ByteArray   -- 4 per MB
  deriving Inhabited

/-- What stays constant while a slice's macroblocks are decoded (kept apart
from `Dec` so that copying or rewriting the state touches fewer fields). -/
structure SliceCtx where
  sliceType : SliceType := .i
  listCount : Nat := 0
  refCount0 : Nat := 0
  refCount1 : Nat := 0
  directSpatial : Bool := false
  cqpOff0 : Int := 0
  cqpOff1 : Int := 0
  cqpTab0 : ByteArray := .empty
  cqpTab1 : ByteArray := .empty
  transform8x8Mode : Bool := false
  constrainedIntra : Bool := false
  direct8x8Inference : Bool := true
  scaling4 : ByteArray := .empty
  scaling8 : ByteArray := .empty
  refList0 : Array RefPic := #[]
  refList1 : Array RefPic := #[]
  /-- Per slice number (index 0 unused). -/
  slices : Array SliceInfo := #[default]
  /-- Weighted prediction (FFmpeg pwt): 0 none, 1 explicit, 2 implicit. -/
  useWeight : Nat := 0
  useWeightChroma : Bool := false
  lumaDenom : Nat := 0
  chromaDenom : Nat := 0
  /-- Explicit (weight, offset) per reference, list 0 and list 1; chroma Cb then Cr. -/
  lumaW0 : Array (Int × Int) := #[]
  lumaW1 : Array (Int × Int) := #[]
  chromaW0 : Array ((Int × Int) × (Int × Int)) := #[]
  chromaW1 : Array ((Int × Int) × (Int × Int)) := #[]
  /-- Implicit weight of list 0 for (ref0, ref1), at ref0 * 32 + ref1. -/
  implicitW : Array Int := #[]
  /-- Temporal direct: distance scale factor per list 0 reference, and the
  colocated picture's list 0 / list 1 references mapped into list 0. -/
  distScale : Array Int := #[]
  mapCol0 : Array Nat := #[]
  mapCol1 : Array Nat := #[]
  deriving Inhabited

structure Dec where
  -- geometry
  mbW : Nat
  mbH : Nat
  /-- The picture being decoded (planes and per-macroblock tables). -/
  pic : Pic
  -- slice (the constants in `sl`)
  sl : SliceCtx := {}
  cabac : Cabac
  sliceNum : Nat := 0
  qscale : Nat := 0
  chromaQp0 : Nat := 0
  chromaQp1 : Nat := 0
  lastQscaleDiff : Int := 0
  -- current macroblock
  mbX : Nat := 0
  mbY : Nat := 0
  mbXY : Nat := 0
  topType : UInt32 := 0
  leftType : UInt32 := 0
  topleftType : UInt32 := 0
  toprightType : UInt32 := 0
  topSamples : UInt32 := 0
  leftSamples : UInt32 := 0
  topleftSamples : UInt32 := 0
  toprightSamples : UInt32 := 0
  leftCbp : Nat := 0
  topCbp : Nat := 0
  neighborTransformSize : Nat := 0
  intra16x16Mode : Nat := 0
  chromaPredMode : Nat := 0
  /-- The four sub-macroblock types (u32 each; read with `Dec.sub`). A
  `UInt32` stored in an `Array` is boxed, which allocates on 32-bit ARM. -/
  subMb : ByteArray := Bytes.filled 16 0
  /-- The macroblock type refined by direct prediction. -/
  dirType : UInt32 := 0
  prevMbSkipped : Bool := false
  cbp : Nat := 0
  -- caches (scan8 layout, 8 wide)
  nnzC : ByteArray := ⟨Array.replicate 120 0⟩
  i4C : ByteArray := ⟨Array.replicate 40 0⟩
  mvC0 : ByteArray := ⟨Array.replicate 160 0⟩
  mvC1 : ByteArray := ⟨Array.replicate 160 0⟩
  mvdC0 : ByteArray := ⟨Array.replicate 80 0⟩
  mvdC1 : ByteArray := ⟨Array.replicate 80 0⟩
  /-- Entries 16, 24 and 32 (right of the macroblock's rows 0..2) stay
  PART_NOT_AVAILABLE: the top right of the right-hand 8x8 and 4x4 blocks
  (FFmpeg sets them once per slice context). -/
  refC0 : ByteArray := refCacheInit
  refC1 : ByteArray := refCacheInit
  directC : ByteArray := ⟨Array.replicate 40 0⟩
  -- coefficients of the current macroblock (FFmpeg sl->mb layout, raster in
  -- each block) as int16, 16-aligned (an array over 512 bytes) for the kernels
  coef : ByteArray := Bytes.zeros (2 * 16 * 48)
  /-- The 384 samples of the current I_PCM macroblock (luma, Cb, Cr). -/
  pcm : ByteArray := .empty
  lumaDc : Array Int := Array.replicate 16 0
  /-- Significance map of the residual block being decoded: scan positions. -/
  sig : ByteArray := Bytes.filled 64 0
  -- scratch results
  v : Int := 0
  /-- A second result (decodeMvd: the clipped mvd magnitude). -/
  aux : Nat := 0
  err : Option String := none
  deriving Inhabited

/-- The bytes after a plane's last row: 0..63 for the kernels' wide loads,
64..191 scratch for Kernels.idct, the last 8 scratch for Kernels.loopFilter. -/
def planeSlack : Nat := 256

/-- A fresh state for a picture of `mbW` x `mbH` macroblocks. -/
def Dec.new (mbW mbH : Nat) : Dec :=
  let n := mbW * mbH
  let z (k : Nat) : ByteArray := Bytes.filled k 0
  -- planes: `planeSlack` bytes after the last row (Kernels.lean)
  { mbW, mbH,
    pic := { y := z (256 * n + planeSlack), cb := z (64 * n + planeSlack), cr := z (64 * n + planeSlack),
             mbType := z (4 * n), sliceTable := Bytes.filled (2 * n) 0xFF, cbpTable := z (2 * n),
             chromaPredTable := z n, qscaleTable := z n, nnzTable := z (48 * n), i4Table := z (8 * n),
             mv0 := z (4 * 16 * n), mv1 := z (4 * 16 * n), ref0 := Bytes.filled (4 * n) 0xFF,
             ref1 := Bytes.filled (4 * n) 0xFF, mvd0 := z (16 * n), mvd1 := z (16 * n), directTable := z (4 * n) },
    cabac := default }

@[inline] def Dec.stride (d : Dec) : Nat := 16 * d.mbW
@[inline] def Dec.cstride (d : Dec) : Nat := 8 * d.mbW
@[inline] def Dec.mvC (d : Dec) (l : Nat) : ByteArray := if l == 0 then d.mvC0 else d.mvC1
@[inline] def Dec.mvdC (d : Dec) (l : Nat) : ByteArray := if l == 0 then d.mvdC0 else d.mvdC1
@[inline] def Dec.refC (d : Dec) (l : Nat) : ByteArray := if l == 0 then d.refC0 else d.refC1
/-! State setters: not inlined, so the update happens where the state is
exclusive and unread. Inlined into a caller that reads the state through
borrowed helpers around it, an update has its field projections hoisted
above those reads and copies the whole state (RULES.md). -/

@[noinline] def Dec.withY (d : Dec) (b : ByteArray) : Dec := { d with pic := { d.pic with y := b } }
@[noinline] def Dec.withCb (d : Dec) (b : ByteArray) : Dec := { d with pic := { d.pic with cb := b } }
@[noinline] def Dec.withCr (d : Dec) (b : ByteArray) : Dec := { d with pic := { d.pic with cr := b } }

-- One structure update per setter: updates in two branches leave one of
-- them without in-place reuse (a whole new state per call; RULES.md).
-- One setter per field: a branch before the update would make it retain
-- and release every field.
@[noinline] def Dec.setMvC0 (d : Dec) (b : ByteArray) : Dec := { d with mvC0 := b }
@[noinline] def Dec.setMvC1 (d : Dec) (b : ByteArray) : Dec := { d with mvC1 := b }
@[noinline] def Dec.setMvdC0 (d : Dec) (b : ByteArray) : Dec := { d with mvdC0 := b }
@[noinline] def Dec.setMvdC1 (d : Dec) (b : ByteArray) : Dec := { d with mvdC1 := b }
@[noinline] def Dec.setRefC0 (d : Dec) (b : ByteArray) : Dec := { d with refC0 := b }
@[noinline] def Dec.setRefC1 (d : Dec) (b : ByteArray) : Dec := { d with refC1 := b }
@[inline] def Dec.setMvC (d : Dec) (l : Nat) (b : ByteArray) : Dec := if l == 0 then d.setMvC0 b else d.setMvC1 b
@[inline] def Dec.setMvdC (d : Dec) (l : Nat) (b : ByteArray) : Dec := if l == 0 then d.setMvdC0 b else d.setMvdC1 b
@[inline] def Dec.setRefC (d : Dec) (l : Nat) (b : ByteArray) : Dec := if l == 0 then d.setRefC0 b else d.setRefC1 b
@[inline] def Dec.mvT (d : Dec) (l : Nat) : ByteArray := if l == 0 then d.pic.mv0 else d.pic.mv1
@[inline] def Dec.refT (d : Dec) (l : Nat) : ByteArray := if l == 0 then d.pic.ref0 else d.pic.ref1
@[inline] def Dec.mvdT (d : Dec) (l : Nat) : ByteArray := if l == 0 then d.pic.mvd0 else d.pic.mvd1
@[inline] def Dec.refCount (d : Dec) (l : Nat) : Nat := if l == 0 then d.sl.refCount0 else d.sl.refCount1

/-! Cache writers. Each takes the cache out of the state before writing it
(a cache written while the state still holds it is copied), so callers
write `d.fillMvC ..`, never `d.setMvC l (fillMv d.mvC0 ..)`. -/

def Dec.fillMvC (d : Dec) (l e w h : Nat) (mx my : Int) : Dec :=
  let c := d.mvC l
  let d := d.setMvC l .empty
  d.setMvC l (fillMv c e w h mx my)

def Dec.fillMvdC (d : Dec) (l e w h : Nat) (lo hi : UInt8) : Dec :=
  let c := d.mvdC l
  let d := d.setMvdC l .empty
  d.setMvdC l (fillRect16 c e w h lo hi)

def Dec.fillRefC (d : Dec) (l e w h : Nat) (v : UInt8) : Dec :=
  let c := d.refC l
  let d := d.setRefC l .empty
  d.setRefC l (fillRect c e w h v)

def Dec.fillNnzC (d : Dec) (e w h : Nat) (v : UInt8) : Dec :=
  let c := d.nnzC
  let d := { d with nnzC := .empty }
  { d with nnzC := fillRect c e w h v }

def Dec.fillI4C (d : Dec) (e w h : Nat) (v : UInt8) : Dec :=
  let c := d.i4C
  let d := { d with i4C := .empty }
  { d with i4C := fillRect c e w h v }

def Dec.fillDirectC (d : Dec) (e w h : Nat) (v : UInt8) : Dec :=
  let c := d.directC
  let d := { d with directC := .empty }
  { d with directC := fillRect c e w h v }


@[inline] def Dec.sub (d : Dec) (i : Nat) : UInt32 := getU32 d.subMb i
def Dec.setSub (d : Dec) (i : Nat) (v : UInt32) : Dec :=
  let b := d.subMb
  let d := { d with subMb := .empty }
  { d with subMb := setU32 b i v }

/-- The type of macroblock `xy` if `ok` (inside the picture) and decoded in
the current slice; else 0. -/
@[noinline] def Dec.typeAt (d : @& Dec) (ok : Bool) (xy : USize) : UInt32 :=
  if ok && u16 d.pic.sliceTable (2 * xy) == d.sliceNum.toUInt32 then u32 d.pic.mbType (4 * xy) else 0
@[inline] def Dec.typeIf (d : @& Dec) (ok : Bool) (xy : Nat) : UInt32 := d.typeAt ok xy.toUSize

/-- CABAC helpers that update the engine inside the state and leave the bin in `v`. -/
@[inline] def Dec.decision (d : Dec) (ctx : Nat) : Dec :=
  let c := d.cabac.decision ctx
  { d with cabac := c, v := c.bin.toNat }
@[inline] def Dec.bypass (d : Dec) : Dec :=
  let c := d.cabac.bypass
  { d with cabac := c, v := c.bin.toNat }
@[inline] def Dec.terminate (d : Dec) : Dec :=
  let c := d.cabac.terminate
  { d with cabac := c, v := c.bin.toNat }

end Yt.H264
