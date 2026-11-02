/-
Macroblock reconstruction (FFmpeg h264_mb.c / h264_mb_template.c for 8-bit
4:2:0 frame macroblocks): intra prediction, motion compensation (with weighted prediction) and the residual transforms. Part of the Lean
H.264 decoder (follows FFmpeg 9.0.1, LGPL-2.1-or-later).

The loop filter runs over the whole picture afterwards, so intra prediction
sees unfiltered neighbours directly (FFmpeg filters row by row and swaps the
unfiltered border back in: xchg_mb_border).
-/
import Yt.H264.MbCabac
import Yt.H264.Intra
import Yt.H264.Transform
import Yt.H264.Inter

namespace Yt.H264
open Yt.Bytes (u8 u16 u32 u8set)

/-- Byte offset of luma block `n` (FFmpeg order) of the current macroblock. -/
@[inline] def Dec.lumaOff (d : Dec) (n : Nat) : Nat :=
  (16 * d.mbY + 4 * blkY n) * d.stride + 16 * d.mbX + 4 * blkX n
/-- Byte offset of chroma block `k` (0..3, raster) of the current macroblock. -/
@[inline] def Dec.chromaOff (d : Dec) (k : Nat) : Nat :=
  (8 * d.mbY + 4 * (k >>> 1)) * d.cstride + 8 * d.mbX + 4 * (k &&& 1)

/-- The planes and `d` without them. A plane written while `d` still holds it
would be copied whole (about 1 MB per write), so writers take the planes out
first and use only the returned state until they put them back. -/
@[inline] def Dec.takePlanes (d : Dec) : ByteArray × ByteArray × ByteArray × Dec :=
  (d.pic.y, d.pic.cb, d.pic.cr, { d with pic := { d.pic with y := .empty, cb := .empty, cr := .empty } })

-- Not inlined: in a caller that reads the state through borrowed helpers
-- between taking and putting, an inlined update would have its field
-- projections hoisted above those reads and copy the state (RULES.md).
@[noinline] def Dec.withoutPlanes (d : Dec) : Dec :=
  { d with pic := { d.pic with y := .empty, cb := .empty, cr := .empty } }
@[noinline] def Dec.withPlanes (d : Dec) (y cb cr : ByteArray) : Dec :=
  { d with pic := { d.pic with y := y, cb := cb, cr := cr } }

/-- The int16 coefficient `i`. -/
@[inline] def coefAt (c : @& ByteArray) (i : Nat) : Int := getI16 c i

/-- The coefficients `off .. off + n - 1` (n even) cleared. -/
@[inline] def zeroCoefs (c : ByteArray) (off n : Nat) : ByteArray :=
  Kernels.fillRaw c (2 * off).toUSize 0 (n / 2).toUSize 1 0 4

/-- How FFmpeg applies a block's residual: `coded` blocks (intra 4x4/8x8,
inter) use the DC-only add when their one coefficient is the DC
(`idct_add16`, `idct8_add4`); `dcFallback` blocks (intra 16x16, chroma) use
the full transform when they have AC coefficients and else the DC-only add
when the DC is set (`idct_add16intra`, `idct_add8`). The DC-only add
differs from the full transform when DC + 32 overflows 16 bits. -/
inductive Rule where
  | coded
  | dcFallback

/-- Whether a 4x4 block is applied as DC only (`true`) per `rule`. -/
@[inline] def useDcOnly (dc : Int) (nnz : Nat) : Rule → Bool
  | .coded => nnz == 1 && dc != 0
  | .dcFallback => nnz == 0 && dc != 0

/-- `plane` with the residual of the 4x4 block at `off` of the coefficients
`c` added at byte `po` (row `stride`). -/
def addBlock4 (c : @& ByteArray) (plane : ByteArray) (po stride off nnz : Nat) (rule : Rule) : ByteArray :=
  if useDcOnly (coefAt c off) nnz rule then Kernels.idct plane po.toUSize stride.toUSize c off.toUSize 1
  else if nnz != 0 then Kernels.idct plane po.toUSize stride.toUSize c off.toUSize 0
  else plane

/-- Whether `addBlock4` uses the block (so it must be cleared). -/
@[inline] def blockUsed (c : @& ByteArray) (off nnz : Nat) (rule : Rule) : Bool :=
  nnz != 0 || useDcOnly (coefAt c off) nnz rule

-- The residual passes work on the plane and the coefficients taken out of
-- the state once per macroblock (the state, borrowed, gives nnz and the
-- prediction modes), and put them back in one update.

/-- Luma 4x4 blocks `i`, `i + 1`, … (`k` of them): intra 4x4 prediction
first when `intra`, then the residual, its coefficients cleared. -/
def luma4Pass (d : @& Dec) (rule : Rule) (intra : Bool) (y : ByteArray) (c : ByteArray) (i : Nat) :
    Nat → ByteArray × ByteArray
  | 0 => (y, c)
  | k + 1 =>
    let y := if intra then
        Intra.pred4x4 y (d.lumaOff i) d.stride (getI8 d.i4C (s8 i)).toNat
          ((d.toprightSamples <<< i.toUInt32) &&& 0x8000 != 0)
      else y
    let nnz := d.nnzC[s8 i]!.toNat
    if blockUsed c (16 * i) nnz rule then
      luma4Pass d rule intra (addBlock4 c y (d.lumaOff i) d.stride (16 * i) nnz rule) (zeroCoefs c (16 * i) 16) (i + 1) k
    else luma4Pass d rule intra y c (i + 1) k

/-- The same for 8x8 blocks `i`, `i + 4`, … (intra 8x8 prediction when `intra`). -/
def luma8Pass (d : @& Dec) (intra : Bool) (y : ByteArray) (c : ByteArray) (i : Nat) : Nat → ByteArray × ByteArray
  | 0 => (y, c)
  | k + 1 =>
    let y := if intra then
        Intra.pred8x8l y (d.lumaOff i) d.stride (getI8 d.i4C (s8 i)).toNat
          ((d.topleftSamples <<< i.toUInt32) &&& 0x8000 != 0) ((d.toprightSamples <<< i.toUInt32) &&& 0x4000 != 0)
      else y
    let nnz := d.nnzC[s8 i]!.toNat
    if nnz == 0 then luma8Pass d intra y c (i + 4) k
    else
      let y := Kernels.idct y (d.lumaOff i).toUSize d.stride.toUSize c (16 * i).toUSize
        (if nnz == 1 && coefAt c (16 * i) != 0 then 3 else 2)
      luma8Pass d intra y (zeroCoefs c (16 * i) 64) (i + 4) k

/-- Chroma 4x4 blocks `k` .. 3 of plane `cp` (Cr if `cr`). -/
def chromaPass (d : @& Dec) (cr : Bool) (cp : ByteArray) (c : ByteArray) (k : Nat) : Nat → ByteArray × ByteArray
  | 0 => (cp, c)
  | m + 1 =>
    let n := 16 + (if cr then 16 else 0) + k
    let nnz := d.nnzC[s8 n]!.toNat
    if blockUsed c (16 * n) nnz .dcFallback then
      chromaPass d cr (addBlock4 c cp (d.chromaOff k) d.cstride (16 * n) nnz .dcFallback) (zeroCoefs c (16 * n) 16)
        (k + 1) m
    else chromaPass d cr cp c (k + 1) m

@[noinline] def Dec.withoutYCoef (d : Dec) : Dec := { d with pic := { d.pic with y := .empty }, coef := .empty }
@[noinline] def Dec.withYCoef (d : Dec) (y : ByteArray) (c : ByteArray) : Dec :=
  { d with pic := { d.pic with y := y }, coef := c }
@[noinline] def Dec.withoutChromaCoef (d : Dec) : Dec :=
  { d with pic := { d.pic with cb := .empty, cr := .empty }, coef := .empty }
@[noinline] def Dec.withChromaCoef (d : Dec) (cb cr : ByteArray) (c : ByteArray) : Dec :=
  { d with pic := { d.pic with cb := cb, cr := cr }, coef := c }

/-- The intra 16x16 luma DCs, dequantized and transformed, into the blocks' DC
coefficients (block (c, r) in 4x4 units is FFmpeg block
(r >>> 1) * 8 + (c >>> 1) * 4 + (r &&& 1) * 2 + (c &&& 1)). -/
def setLumaDcs (c : ByteArray) (dcs : Array Int) : Nat → ByteArray
  | 0 => c
  | k + 1 =>
    let i := 15 - k
    let r := i / 4
    let col := i % 4
    let n := (r >>> 1) * 8 + (col >>> 1) * 4 + (r &&& 1) * 2 + (col &&& 1)
    setLumaDcs (setI16 c (16 * n) dcs[i]!) dcs k

/-- Intra prediction and residual of the luma plane (`hl_decode_mb_predict_luma`
and the intra 16x16 part of `hl_decode_mb_idct_luma`). -/
def reconIntraLuma (d : Dec) (t : UInt32) : Dec :=
  let y := d.pic.y
  let c := d.coef
  let d := d.withoutYCoef
  let (y, c) :=
    if isIntra4x4 t then
      if is8x8dct t then luma8Pass d true y c 0 4 else luma4Pass d .coded true y c 0 16
    else
      let y := Intra.pred16x16 y (d.lumaOff 0) d.stride d.intra16x16Mode
      let c := if d.nnzC[s8 LUMA_DC_BLOCK_INDEX]! != 0 then
          setLumaDcs c (Transform.lumaDcDequantIdct d.lumaDc (qmul4 d 0 d.qscale 0)) 16
        else c
      luma4Pass d .dcFallback false y c 0 16
  d.withYCoef y c

/-- Luma residual of an inter macroblock. -/
def reconInterLuma (d : Dec) (t : UInt32) : Dec :=
  if d.cbp &&& 15 == 0 then d
  else
    let y := d.pic.y
    let c := d.coef
    let d := d.withoutYCoef
    let (y, c) := if is8x8dct t then luma8Pass d false y c 0 4 else luma4Pass d .coded false y c 0 16
    d.withYCoef y c

/-- The chroma DC of plane `k` (0 Cb, 1 Cr) dequantized and transformed into
its blocks' DC coefficients. -/
def chromaDcs (d : @& Dec) (t : UInt32) (c : ByteArray) (k : Nat) : ByteArray :=
  let base := 256 + 256 * k
  if d.nnzC[s8 (CHROMA_DC_BLOCK_INDEX + k)]! == 0 then c
  else
    let qp := if k == 0 then d.chromaQp0 else d.chromaQp1
    let list := (if isIntra t then 1 else 4) + k
    let dcs := Transform.chromaDcDequantIdct #[coefAt c base, coefAt c (base + 16), coefAt c (base + 32), coefAt c (base + 48)]
      (qmul4 d list qp 0)
    setI16 (setI16 (setI16 (setI16 c base dcs[0]!) (base + 16) dcs[1]!) (base + 32) dcs[2]!) (base + 48) dcs[3]!

/-- Chroma DC transforms and residuals (`hl_decode_mb`, `idct_add8`). -/
def reconChromaResidual (d : Dec) (t : UInt32) : Dec :=
  if d.cbp &&& 0x30 == 0 then d
  else
    let cb := d.pic.cb
    let cr := d.pic.cr
    let c := d.coef
    let d := d.withoutChromaCoef
    let c := chromaDcs d t (chromaDcs d t c 0) 1
    let (cb, c) := chromaPass d false cb c 0 4
    let (cr, c) := chromaPass d true cr c 0 4
    d.withChromaCoef cb cr c

-- Motion compensation computes positions in USize (Nat arithmetic costs
-- overflow checks and boxing per step); the rare paths convert to Nat.

/-- The reference index (≥ 0) of cache entry `e` of `list`. -/
@[inline] def refIdx (d : @& Dec) (list : Nat) (e : USize) : Nat :=
  let r := u8 (d.refC list) e
  if r ≥ 128 then 0 else r.toNat

/-- The reference picture of cache entry `e` in `list` (the index must be in the list). -/
@[inline] def refPicOf (d : @& Dec) (list : Nat) (e : USize) : RefPic :=
  (if list == 0 then d.sl.refList0 else d.sl.refList1)[refIdx d list e]!

/-- A 16-bit vector component as 32-bit two's complement. -/
@[inline] def sx16 (v : UInt32) : UInt32 := ((v &&& 0xFFFF) ^^^ 0x8000) - 0x8000

/-- Predict the luma of one partition (x, y, w, h within the macroblock;
motion at cache entry `e`) from `list` into `yP` at `yOff` (`mc_dir_part`). -/
def mcLuma (d : @& Dec) (list : Nat) (e x y w h : USize) (yP : ByteArray) (yOff yStride : USize) (avg : Bool) :
    ByteArray :=
  let pic := refPicOf d list e
  let mx := sx16 (u16 (d.mvC list) (4 * e))
  let my := sx16 (u16 (d.mvC list) (4 * e + 2))
  Inter.lumaMcU yP yOff yStride pic.y d.stride.toUSize (16 * d.mbW.toUSize) (16 * d.mbH.toUSize)
    ((16 * d.mbX.toUSize + x).toUInt32 + Inter.sar mx 2) ((16 * d.mbY.toUSize + y).toUInt32 + Inter.sar my 2)
    (mx &&& 3).toUSize (my &&& 3).toUSize w h avg

/-- The same for one chroma plane (Cr if `cr`). -/
def mcChroma (d : @& Dec) (list : Nat) (e x y w h : USize) (cP : ByteArray) (cr : Bool) (cOff cStride : USize)
    (avg : Bool) : ByteArray :=
  let pic := refPicOf d list e
  let mx := sx16 (u16 (d.mvC list) (4 * e))
  let my := sx16 (u16 (d.mvC list) (4 * e + 2))
  Inter.chromaMcU cP cOff cStride (if cr then pic.cr else pic.cb) d.cstride.toUSize (8 * d.mbW.toUSize)
    (8 * d.mbH.toUSize) ((8 * d.mbX.toUSize + (x / 2)).toUInt32 + Inter.sar mx 3)
    ((8 * d.mbY.toUSize + (y / 2)).toUInt32 + Inter.sar my 3) (mx &&& 7).toUSize (my &&& 7).toUSize (w / 2)
    (h / 2) avg

/-- The luma plane with one partition predicted: plain (put, then average
for a bi-predicted one) or weighted (explicit, or implicit for bi-prediction). -/
def predLuma (d : @& Dec) (e x y w h : USize) (useL0 useL1 weighted : Bool) (ref0 ref1 : Nat) (yP : ByteArray)
    (yOff : USize) : ByteArray :=
  let ys := d.stride.toUSize
  if !weighted then
    let yP := mcLuma d (if useL0 then 0 else 1) e x y w h yP yOff ys false
    if useL0 && useL1 then mcLuma d 1 e x y w h yP yOff ys true else yP
  else if useL0 && useL1 then
    let ly := mcLuma d 0 e x y w h yP yOff ys false
    let ty := mcLuma d 1 e x y w h (Bytes.zeros 256) 0 16 false
    if d.sl.useWeight == 2 then
      let iw := d.sl.implicitW[32 * ref0 + ref1]!
      Inter.biweight ly yOff.toNat ys.toNat ty 0 16 w.toNat h.toNat 5 iw (64 - iw) 0
    else
      let (a0, b0) := d.sl.lumaW0[ref0]!
      let (a1, b1) := d.sl.lumaW1[ref1]!
      Inter.biweight ly yOff.toNat ys.toNat ty 0 16 w.toNat h.toNat d.sl.lumaDenom a0 a1 (b0 + b1)
  else
    let list := if useL1 then 1 else 0
    let ly := mcLuma d list e x y w h yP yOff ys false
    let (lw, lo) := (if list == 0 then d.sl.lumaW0 else d.sl.lumaW1)[if useL1 then ref1 else ref0]!
    Inter.weight ly yOff.toNat ys.toNat w.toNat h.toNat d.sl.lumaDenom lw lo

/-- The same for one chroma plane (Cr if `cr`). -/
def predChroma (d : @& Dec) (e x y w h : USize) (useL0 useL1 weighted : Bool) (ref0 ref1 : Nat) (cP : ByteArray)
    (cr : Bool) (cOff : USize) : ByteArray :=
  let cs := d.cstride.toUSize
  let pick (w : (Int × Int) × (Int × Int)) : Int × Int := if cr then w.2 else w.1
  if !weighted then
    let cP := mcChroma d (if useL0 then 0 else 1) e x y w h cP cr cOff cs false
    if useL0 && useL1 then mcChroma d 1 e x y w h cP cr cOff cs true else cP
  else if useL0 && useL1 then
    let lc := mcChroma d 0 e x y w h cP cr cOff cs false
    let tc := mcChroma d 1 e x y w h (Bytes.zeros 64) cr 0 8 false
    if d.sl.useWeight == 2 then
      let iw := d.sl.implicitW[32 * ref0 + ref1]!
      Inter.biweight lc cOff.toNat cs.toNat tc 0 8 (w / 2).toNat (h / 2).toNat 5 iw (64 - iw) 0
    else
      let (a0, b0) := pick d.sl.chromaW0[ref0]!
      let (a1, b1) := pick d.sl.chromaW1[ref1]!
      Inter.biweight lc cOff.toNat cs.toNat tc 0 8 (w / 2).toNat (h / 2).toNat d.sl.chromaDenom a0 a1 (b0 + b1)
  else
    let list := if useL1 then 1 else 0
    let lc := mcChroma d list e x y w h cP cr cOff cs false
    if !d.sl.useWeightChroma then lc
    else
      let (cw, co) := pick (if list == 0 then d.sl.chromaW0 else d.sl.chromaW1)[if useL1 then ref1 else ref0]!
      Inter.weight lc cOff.toNat cs.toNat (w / 2).toNat (h / 2).toNat d.sl.chromaDenom cw co

/-- Motion compensation of one partition (`mc_part`): luma (x, y, w, h) in
pixels within the macroblock, from the motion of block `n`. The planes are
taken out, predicted and put back in one update. -/
def mcPartition (d : Dec) (n x y w h : USize) (useL0 useL1 : Bool) : Dec :=
  let e := (u8 scan8 n).toUSize
  let ref0 := refIdx d 0 e
  let ref1 := refIdx d 1 e
  if (useL0 && ref0 ≥ d.sl.refList0.size) || (useL1 && ref1 ≥ d.sl.refList1.size) then
    d.setErr "H.264: reference index past the list"
  else
    let weighted := d.sl.useWeight == 1 ||
      (d.sl.useWeight == 2 && useL0 && useL1 && d.sl.implicitW[32 * ref0 + ref1]! != 32)
    let yOff := (16 * d.mbY.toUSize + y) * d.stride.toUSize + 16 * d.mbX.toUSize + x
    let cOff := (8 * d.mbY.toUSize + (y / 2)) * d.cstride.toUSize + 8 * d.mbX.toUSize + (x / 2)
    let yP := d.pic.y
    let cbP := d.pic.cb
    let crP := d.pic.cr
    let d := d.withoutPlanes
    let yP := predLuma d e x y w h useL0 useL1 weighted ref0 ref1 yP yOff
    let cbP := predChroma d e x y w h useL0 useL1 weighted ref0 ref1 cbP false cOff
    let crP := predChroma d e x y w h useL0 useL1 weighted ref0 ref1 crP true cOff
    d.withPlanes yP cbP crP

/-- The partitions of 8x8 block `i8` (its sub-macroblock type's shape). -/
def mc8x8 (d : Dec) (i8 : USize) : Dec :=
  let sub := d.sub i8.toNat
  let n := 4 * i8
  let x0 := 8 * (i8 &&& 1)
  let y0 := 8 * (i8 / 2)
  let l0 := isDir sub 0 0
  let l1 := isDir sub 0 1
  if sub &&& MB_16x16 != 0 then mcPartition d n x0 y0 8 8 l0 l1
  else if sub &&& MB_16x8 != 0 then mcPartition (mcPartition d n x0 y0 8 4 l0 l1) (n + 2) x0 (y0 + 4) 8 4 l0 l1
  else if sub &&& MB_8x16 != 0 then mcPartition (mcPartition d n x0 y0 4 8 l0 l1) (n + 1) (x0 + 4) y0 4 8 l0 l1
  else
    let d := mcPartition d n x0 y0 4 4 l0 l1
    let d := mcPartition d (n + 1) (x0 + 4) y0 4 4 l0 l1
    let d := mcPartition d (n + 2) x0 (y0 + 4) 4 4 l0 l1
    mcPartition d (n + 3) (x0 + 4) (y0 + 4) 4 4 l0 l1

/-- `hl_motion_420`. -/
def reconMotion (d : Dec) (t : UInt32) : Dec :=
  if t &&& MB_16x16 != 0 then mcPartition d 0 0 0 16 16 (isDir t 0 0) (isDir t 0 1)
  else if t &&& MB_16x8 != 0 then
    mcPartition (mcPartition d 0 0 0 16 8 (isDir t 0 0) (isDir t 0 1)) 8 0 8 16 8 (isDir t 1 0) (isDir t 1 1)
  else if t &&& MB_8x16 != 0 then
    mcPartition (mcPartition d 0 0 0 8 16 (isDir t 0 0) (isDir t 0 1)) 4 8 0 8 16 (isDir t 1 0) (isDir t 1 1)
  else mc8x8 (mc8x8 (mc8x8 (mc8x8 d 0) 1) 2) 3

/-- Copy the I_PCM samples into the planes. -/
def reconPcm (d : Dec) : Dec := Id.run do
  let (y, cb, cr, d) := d.takePlanes
  let mut y := y
  let mut cb := cb
  let mut cr := cr
  for r in [0:16] do
    for c in [0:16] do y := y.set! ((16 * d.mbY + r) * d.stride + 16 * d.mbX + c) d.pcm[16 * r + c]!
  for r in [0:8] do
    for c in [0:8] do
      cb := cb.set! ((8 * d.mbY + r) * d.cstride + 8 * d.mbX + c) d.pcm[256 + 8 * r + c]!
      cr := cr.set! ((8 * d.mbY + r) * d.cstride + 8 * d.mbX + c) d.pcm[320 + 8 * r + c]!
  return d.withPlanes y cb cr

/-- Reconstruct the current macroblock (after `decodeMb`). -/
def reconMb (d : Dec) : Dec := Id.run do
  let t := getU32 d.pic.mbType d.mbXY
  let mut d := d
  if isPcm t then return reconPcm d
  if isIntra t then
    let co := d.chromaOff 0
    let cb := d.pic.cb
    let cr := d.pic.cr
    d := (d.withCb .empty).withCr .empty
    d := (d.withCb (Intra.pred8x8 cb co d.cstride d.chromaPredMode)).withCr (Intra.pred8x8 cr co d.cstride d.chromaPredMode)
    d := reconIntraLuma d t
  else
    if isSkip t then d := d.clearCbp
    d := reconMotion d t
    d := reconInterLuma d t
  return reconChromaResidual d t

end Yt.H264
