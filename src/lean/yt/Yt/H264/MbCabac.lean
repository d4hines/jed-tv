/-
CABAC macroblock layer (FFmpeg h264_cabac.c, `ff_h264_decode_mb_cabac`, for
frame macroblocks of 8-bit 4:2:0 video). Part of the Lean H.264 decoder
(follows FFmpeg 9.0.1, LGPL-2.1-or-later).

Every function takes and returns the state; values come back in `d.v`.
-/
import Yt.H264.Direct

namespace Yt.H264
open Yt.Bytes (u8 u16 u32 u8set u16set u32set)

/-! ## Macroblock type tables (FFmpeg h264data.c) -/

/-- I macroblock types 0..25: (type, intra 16x16 prediction mode, cbp). -/
def iMbInfo (k : Nat) : UInt32 × Nat × Nat :=
  if k == 0 then (MB_INTRA4x4, 0, 0)
  else if k == 25 then (MB_INTRA_PCM, 0, 0)
  else
    let pred := #[2, 1, 0, 3][(k - 1) % 4]!
    let cbp := #[0, 16, 32][((k - 1) / 4) % 3]! + (if k ≥ 13 then 15 else 0)
    (MB_INTRA16x16, pred, cbp)

def pMbInfo : Array (UInt32 × Nat) := #[
  (MB_16x16 ||| MB_P0L0, 1), (MB_16x8 ||| MB_P0L0 ||| MB_P1L0, 2), (MB_8x16 ||| MB_P0L0 ||| MB_P1L0, 2),
  (MB_8x8 ||| MB_P0L0 ||| MB_P1L0, 4), (MB_8x8 ||| MB_P0L0 ||| MB_P1L0 ||| MB_REF0, 4)]

def pSubInfo : Array (UInt32 × Nat) := #[
  (MB_16x16 ||| MB_P0L0, 1), (MB_16x8 ||| MB_P0L0, 2), (MB_8x16 ||| MB_P0L0, 2), (MB_8x8 ||| MB_P0L0, 4)]

def bMbInfo : Array (UInt32 × Nat) := #[
  (MB_DIRECT2 ||| MB_L0L1, 1), (MB_16x16 ||| MB_P0L0, 1), (MB_16x16 ||| MB_P0L1, 1),
  (MB_16x16 ||| MB_P0L0 ||| MB_P0L1, 1),
  (MB_16x8 ||| MB_P0L0 ||| MB_P1L0, 2), (MB_8x16 ||| MB_P0L0 ||| MB_P1L0, 2),
  (MB_16x8 ||| MB_P0L1 ||| MB_P1L1, 2), (MB_8x16 ||| MB_P0L1 ||| MB_P1L1, 2),
  (MB_16x8 ||| MB_P0L0 ||| MB_P1L1, 2), (MB_8x16 ||| MB_P0L0 ||| MB_P1L1, 2),
  (MB_16x8 ||| MB_P0L1 ||| MB_P1L0, 2), (MB_8x16 ||| MB_P0L1 ||| MB_P1L0, 2),
  (MB_16x8 ||| MB_P0L0 ||| MB_P1L0 ||| MB_P1L1, 2), (MB_8x16 ||| MB_P0L0 ||| MB_P1L0 ||| MB_P1L1, 2),
  (MB_16x8 ||| MB_P0L1 ||| MB_P1L0 ||| MB_P1L1, 2), (MB_8x16 ||| MB_P0L1 ||| MB_P1L0 ||| MB_P1L1, 2),
  (MB_16x8 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L0, 2), (MB_8x16 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L0, 2),
  (MB_16x8 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L1, 2), (MB_8x16 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L1, 2),
  (MB_16x8 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L0 ||| MB_P1L1, 2),
  (MB_8x16 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L0 ||| MB_P1L1, 2),
  (MB_8x8 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L0 ||| MB_P1L1, 4)]

def bSubInfo : Array (UInt32 × Nat) := #[
  (MB_DIRECT2, 1), (MB_16x16 ||| MB_P0L0, 1), (MB_16x16 ||| MB_P0L1, 1), (MB_16x16 ||| MB_P0L0 ||| MB_P0L1, 1),
  (MB_16x8 ||| MB_P0L0 ||| MB_P1L0, 2), (MB_8x16 ||| MB_P0L0 ||| MB_P1L0, 2),
  (MB_16x8 ||| MB_P0L1 ||| MB_P1L1, 2), (MB_8x16 ||| MB_P0L1 ||| MB_P1L1, 2),
  (MB_16x8 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L0 ||| MB_P1L1, 2),
  (MB_8x16 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L0 ||| MB_P1L1, 2),
  (MB_8x8 ||| MB_P0L0 ||| MB_P1L0, 4), (MB_8x8 ||| MB_P0L1 ||| MB_P1L1, 4),
  (MB_8x8 ||| MB_P0L0 ||| MB_P0L1 ||| MB_P1L0 ||| MB_P1L1, 4)]

/-! ## Syntax elements -/

def b01 (b : Bool) : Nat := if b then 1 else 0

/-- `decode_cabac_intra_mb_type` -> v. -/
def decodeIntraMbType (d : Dec) (base : Nat) (intraSlice : Bool) : Dec := Id.run do
  let mut d := d
  let mut st := base
  if intraSlice then
    let m := MB_INTRA16x16 ||| MB_INTRA_PCM
    d := d.decision (base + b01 (d.leftType &&& m != 0) + b01 (d.topType &&& m != 0))
    if d.v == 0 then return { d with v := 0 }
    st := base + 2
  else
    d := d.decision base
    if d.v == 0 then return { d with v := 0 }
  d := d.terminate
  if d.v == 1 then return { d with v := 25 }
  let i := b01 intraSlice
  d := d.decision (st + 1)
  let mut t : Int := 1 + 12 * d.v
  d := d.decision (st + 2)
  if d.v == 1 then
    d := d.decision (st + 2 + i)
    t := t + 4 + 4 * d.v
  d := d.decision (st + 3 + i)
  t := t + 2 * d.v
  d := d.decision (st + 3 + 2 * i)
  return { d with v := t + d.v }

/-- The context increment of mb_skip_flag (reads only). -/
-- Small setters: an update inlined after borrowed reads of the state loses
-- its in-place reuse (every field is retained and released; RULES.md).
@[noinline] def Dec.decisionAt (d : Dec) (ctx : Nat) : Dec := d.decision ctx
@[noinline] def Dec.setNnz (d : Dec) (e : Nat) (v : UInt8) : Dec := { d with nnzC := d.nnzC.set! e v }
@[noinline] def Dec.setCoef (d : Dec) (i : Nat) (v : Int) : Dec := { d with coef := setI16 d.coef i v }
@[noinline] def Dec.setLumaDc (d : Dec) (j : Nat) (v : Int) : Dec := { d with lumaDc := d.lumaDc.set! j v }
@[noinline] def Dec.clearLumaDc (d : Dec) : Dec := { d with lumaDc := Array.replicate 16 0 }
@[noinline] def Dec.setLastQscaleDiff (d : Dec) (v : Int) : Dec := { d with lastQscaleDiff := v }
@[noinline] def Dec.setQscale (d : Dec) (delta : Int) (q c0 c1 : Nat) : Dec :=
  { d with lastQscaleDiff := delta, qscale := q, chromaQp0 := c0, chromaQp1 := c1 }
@[noinline] def Dec.clearCbp (d : Dec) : Dec := { d with cbp := 0 }
@[noinline] def Dec.setI4C (d : Dec) (c : ByteArray) : Dec := { d with i4C := c }
@[noinline] def Dec.setErr (d : Dec) (e : String) : Dec := { d with err := some e }

def skipCtx (d : @& Dec) : Nat :=
  let xy := d.mbXY.toUSize
  let a := d.typeAt (d.mbX != 0) (xy - 1)
  let b := d.typeAt (d.mbY != 0) (xy - d.mbW.toUSize)
  b01 (a != 0 && !isSkip a) + b01 (b != 0 && !isSkip b) + (if d.sl.sliceType == .b then 13 else 0)

-- Reads in borrowed helpers, then the update: a projection of the state
-- followed by a call taking the state rules out its in-place update.
/-- `decode_cabac_mb_skip` -> v. -/
def decodeMbSkip (d : Dec) : Dec := d.decisionAt (11 + skipCtx d)

def decodeIntra4x4Mode (d : Dec) (pred : Nat) : Dec := Id.run do
  let mut d := d.decision 68
  if d.v == 1 then return { d with v := pred }
  d := d.decision 69
  let mut m := d.v
  d := d.decision 69
  m := m + 2 * d.v
  d := d.decision 69
  m := m + 4 * d.v
  return { d with v := m + (if m ≥ pred then 1 else 0) }

def decodeChromaPredMode (d : Dec) : Dec := Id.run do
  let ctx := b01 (d.leftType != 0 && d.pic.chromaPredTable[d.leftXY]! != 0) +
             b01 (d.topType != 0 && d.pic.chromaPredTable[d.topXY]! != 0)
  let mut d := d.decision (64 + ctx)
  if d.v == 0 then return { d with v := 0 }
  d := d.decision 67
  if d.v == 0 then return { d with v := 1 }
  d := d.decision 67
  return { d with v := if d.v == 0 then 2 else 3 }

def decodeCbpLuma (d : Dec) : Dec := Id.run do
  let a := d.leftCbp
  let b := d.topCbp
  let nb (v k : Nat) : Nat := if v &&& k == 0 then 1 else 0
  let mut d := d.decision (73 + nb a 2 + 2 * nb b 4)
  let mut cbp := d.v.toNat
  d := d.decision (73 + nb cbp 1 + 2 * nb b 8)
  cbp := cbp + 2 * d.v.toNat
  d := d.decision (73 + nb a 8 + 2 * nb cbp 1)
  cbp := cbp + 4 * d.v.toNat
  d := d.decision (73 + nb cbp 4 + 2 * nb cbp 2)
  cbp := cbp + 8 * d.v.toNat
  return { d with v := cbp }

def decodeCbpChroma (d : Dec) : Dec := Id.run do
  let a := (d.leftCbp >>> 4) &&& 3
  let b := (d.topCbp >>> 4) &&& 3
  let mut d := d.decision (77 + b01 (a > 0) + 2 * b01 (b > 0))
  if d.v == 0 then return { d with v := 0 }
  d := d.decision (77 + 4 + b01 (a == 2) + 2 * b01 (b == 2))
  return { d with v := 1 + d.v }

def decodePSubType (d : Dec) : Dec := Id.run do
  let mut d := d.decision 21
  if d.v == 1 then return { d with v := 0 }
  d := d.decision 22
  if d.v == 0 then return { d with v := 1 }
  d := d.decision 23
  return { d with v := if d.v == 1 then 2 else 3 }

def decodeBSubType (d : Dec) : Dec := Id.run do
  let mut d := d.decision 36
  if d.v == 0 then return { d with v := 0 }
  d := d.decision 37
  if d.v == 0 then
    d := d.decision 39
    return { d with v := 1 + d.v }
  let mut t : Int := 3
  d := d.decision 38
  if d.v == 1 then
    d := d.decision 39
    if d.v == 1 then
      d := d.decision 39
      return { d with v := 11 + d.v }
    t := t + 4
  d := d.decision 39
  t := t + 2 * d.v
  d := d.decision 39
  return { d with v := t + d.v }

/-- The unary part of ref_idx: bins with context `54 + ctx` while 1. -/
def decodeRefGo (d : Dec) (ctx : Nat) (ref : Int) : Dec :=
  if ref ≥ 32 then { d with v := -1 }
  else
    let d := d.decision (54 + ctx)
    if d.v == 0 then { d with v := ref } else decodeRefGo d ((ctx >>> 2) + 4) (ref + 1)
termination_by (32 - ref).toNat
decreasing_by omega

/-- `decode_cabac_mb_ref` -> v (-1 on error). -/
def decodeRef (d : Dec) (list n : Nat) : Dec :=
  let rc := d.refC list
  let i := s8 n
  let refa := getI8 rc (i - 1)
  let refb := getI8 rc (i - 8)
  let dm := (MB_DIRECT2 >>> 1).toUInt8
  let ctx :=
    if d.sl.sliceType == .b then
      b01 (refa > 0 && d.directC[i - 1]! &&& dm == 0) + 2 * b01 (refb > 0 && d.directC[i - 8]! &&& dm == 0)
    else b01 (refa > 0) + 2 * b01 (refb > 0)
  decodeRefGo d ctx 0

-- The bin loops below are recursions with the state as a direct argument: a
-- `for`/`repeat` loop with several mutable variables keeps the state in a
-- tuple, so the CABAC engine inside it is shared and every bin would copy
-- the 1024 context states.

/-- Unary bins with context `ctx` (incremented while `n < 4`): the count, up to 9. -/
def mvdPrefix (d : Dec) (ctx mvd : Nat) : Dec :=
  if mvd ≥ 9 then { d with aux := mvd }
  else
    let d := d.decision ctx
    if d.v == 0 then { d with aux := mvd }
    else mvdPrefix d (if mvd < 4 then ctx + 1 else ctx) (mvd + 1)
termination_by 9 - mvd

/-- Exp-Golomb prefix in bypass bins (k from 3): the final k, or 25 on overflow. -/
def egkPrefix (d : Dec) (k mvd : Nat) : Dec :=
  if k > 24 then { d with aux := 25 }
  else
    let d := d.bypass
    if d.v == 0 then { d with aux := k, v := mvd } else egkPrefix d (k + 1) (mvd + (1 <<< k))
termination_by 25 - k

/-- `n` suffix bits (bypass), most significant first, added to `acc`. -/
def bypassBits (d : Dec) (acc : Nat) : Nat → Dec
  | 0 => { d with v := acc }
  | n + 1 => let d := d.bypass; bypassBits d (acc + (d.v.toNat <<< n)) n

/-- The escape of a mvd prefix of 9: |mvd| in `aux`. -/
def mvdSuffix (d : Dec) : Dec :=
  let d := egkPrefix d 3 9
  if d.aux > 24 then { d with err := some "H.264: mvd overflow", aux := 0 }
  else
    let d := bypassBits d d.v.toNat d.aux
    { d with aux := d.v.toNat }

/-- `decode_cabac_mb_mvd`: the component in `v`, its clipped magnitude in `aux`. -/
def decodeMvd (d : Dec) (base : Nat) (amvd : Int) : Dec :=
  let inc := (if amvd < 3 then 0 else if amvd < 33 then 1 else 2)
  let d := d.decision (base + inc)
  if d.v == 0 then { d with v := 0, aux := 0 }
  else
    let d := mvdPrefix d (base + 3) 1
    let d := if d.aux < 9 then d else mvdSuffix d
    let d := d.bypass
    { d with v := if d.v == 1 then -Int.ofNat d.aux else Int.ofNat d.aux, aux := min d.aux 70 }

/-! ## Residuals -/

def sigOffset : Array Nat := #[105 + 0, 105 + 15, 105 + 29, 105 + 44, 105 + 47, 402]
def lastOffset : Array Nat := #[166 + 0, 166 + 15, 166 + 29, 166 + 44, 166 + 47, 417]
def absOffset : Array Nat := #[227 + 0, 227 + 10, 227 + 20, 227 + 30, 227 + 39, 426]
def cbfBase : Array Nat := #[85, 89, 93, 97, 101, 1012]
def level1Ctx : Array Nat := #[1, 2, 3, 4, 0, 0, 0, 0]
def levelGt1Ctx : Array Nat := #[5, 5, 5, 5, 6, 7, 8, 9]
def levelTrans0 : Array Nat := #[1, 2, 3, 3, 4, 5, 6, 7]
def levelTrans1 : Array Nat := #[4, 4, 4, 4, 5, 6, 7, 7]

/-- int16 wrap, as FFmpeg's int16_t coefficient storage. -/
@[inline] def wrap16 (v : Int) : Int := v.toInt16.toInt

/-- Dequantization factor of a 4x4 coefficient at raster position `j`
(FFmpeg dequant4_coeff[list][qp][...]). -/
def qmul4 (d : Dec) (list qp j : Nat) : Int :=
  let cls := (j &&& 1) + ((j >>> 2) &&& 1)
  let norm := Tables.dequant4Init[3 * (qp % 6) + cls]!.toNat
  -- UInt32: a Nat shift is a runtime call
  Int.ofNat ((norm.toUInt32 * d.sl.scaling4[16 * list + j]!.toUInt32) <<< (qp / 6 + 2).toUInt32).toNat

/-- The same for an 8x8 coefficient (list 0 intra, 1 inter). -/
def qmul8 (d : Dec) (list qp j : Nat) : Int :=
  let cls := Tables.dequant8Class[((j >>> 1) &&& 12) ||| (j &&& 3)]!.toNat
  let norm := Tables.dequant8Init[6 * (qp % 6) + cls]!.toNat
  Int.ofNat ((norm.toUInt32 * d.sl.scaling8[64 * list + j]!.toUInt32) <<< (qp / 6).toUInt32).toNat

/-! Where a residual block's levels go, as scalars (an object per block
would be allocated): kind `dk` 0: 4x4 coefficients at `d.coef[off ..]`,
dequantized with (list, qp); 1: the same for 8x8; 2: raw chroma DC levels at
`d.coef` offsets off + 16 k; 3: raw levels into `d.lumaDc`. -/

/-- The significance map of an 8x8 block from scan position `last`:
positions into `d.sig`, their count in `v`. -/
def sigMap8x8 (d : Dec) (sigB lastB last count : Nat) : Dec :=
  if last ≥ 63 then { d with sig := d.sig.set! count 63, v := count + 1 }
  else
    let d := d.decision (sigB + Tables.sigCoeffFlagOffset8x8[last]!.toNat)
    if d.v == 1 then
      let d := { d with sig := d.sig.set! count last.toUInt8 }
      let d := d.decision (lastB + Tables.lastCoeffFlagOffset8x8[last]!.toNat)
      if d.v == 1 then { d with v := count + 1 } else sigMap8x8 d sigB lastB (last + 1) (count + 1)
    else sigMap8x8 d sigB lastB (last + 1) count
termination_by 63 - last

/-- The same for a block whose last position is `coefs`. -/
def sigMap (d : Dec) (sigB lastB coefs last count : Nat) : Dec :=
  if last ≥ coefs then { d with sig := d.sig.set! count coefs.toUInt8, v := count + 1 }
  else
    let d := d.decision (sigB + last)
    if d.v == 1 then
      let d := { d with sig := d.sig.set! count last.toUInt8 }
      let d := d.decision (lastB + last)
      if d.v == 1 then { d with v := count + 1 } else sigMap d sigB lastB coefs (last + 1) (count + 1)
    else sigMap d sigB lastB coefs (last + 1) count
termination_by coefs - last

/-- Unary bins of coeff_abs_level_minus1 with context `ctx` after its first
1: the level (2..15) in `v`. -/
def levelUnary (d : Dec) (ctx a : Nat) : Dec :=
  if a ≥ 15 then { d with v := a }
  else
    let d := d.decision ctx
    if d.v == 0 then { d with v := a } else levelUnary d ctx (a + 1)
termination_by 15 - a

/-- Exp-Golomb (k = 0) prefix in bypass bins: its length in `v`. -/
def egPrefix (d : Dec) (e : Nat) : Dec :=
  if e ≥ 16 + 7 then { d with v := e }
  else
    let d := d.bypass
    if d.v == 0 then { d with v := e } else egPrefix d (e + 1)
termination_by 23 - e

/-- `n` bypass bits appended to `v` (most significant first). -/
def egSuffix (d : Dec) (v : Nat) : Nat → Dec
  | 0 => { d with v := v }
  | n + 1 => let d := d.bypass; egSuffix d (2 * v + d.v.toNat) n

/-- coeff_abs_level_minus1 + 1 after its first bin was 1, in `v`. -/
def levelAbs (d : Dec) (ctx : Nat) : Dec :=
  let d := levelUnary d ctx 2
  if d.v < 15 then d
  else
    let d := egPrefix d 0
    let d := egSuffix d 1 d.v.toNat
    { d with v := d.v + 14 }

/-- Store a level (sign: the bypass bin in `v`) at raster position `j`. -/
def storeLevel (d : Dec) (dk off list qp j : Nat) (level : Int) : Dec :=
  let signed := if d.v == 1 then -level else level
  if dk ≤ 1 then
    let q := if dk == 1 then qmul8 d list qp j else qmul4 d list qp j
    d.setCoef (off + j) (wrap16 ((signed * q + 32) >>> 6))
  else if dk == 2 then d.setCoef (off + 16 * j) (wrap16 signed)
  else d.setLumaDc j (wrap16 signed)

/-- The levels of the significant positions `d.sig[0..k)`, last first. -/
def levels (d : Dec) (absB maxCoeff dk off list qp node : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let pos := d.sig[k]!.toNat
    -- raster position in the block
    let j :=
      if dk == 1 then Tables.zigzag8x8[pos]!.toNat
      else if dk == 0 then Tables.zigzag4x4[if maxCoeff == 15 then pos + 1 else pos]!.toNat
      else if dk == 2 then pos
      else Tables.zigzag4x4[pos]!.toNat
    let d := d.decision (absB + level1Ctx[node]!)
    if d.v == 0 then
      levels (storeLevel d.bypass dk off list qp j 1) absB maxCoeff dk off list qp levelTrans0[node]! k
    else
      let d := levelAbs d (absB + levelGt1Ctx[node]!)
      let level := d.v
      levels (storeLevel d.bypass dk off list qp j level) absB maxCoeff dk off list qp levelTrans1[node]! k

/-- Coded DC block `n` (category `cat`: 3 chroma, else luma) in the cbp table. -/
@[noinline] def Dec.markDc (d : Dec) (cat n : Nat) : Dec :=
  let xy := d.mbXY.toUSize
  let bit : UInt32 :=
    if cat == 3 then (0x40 : UInt32) <<< (n - CHROMA_DC_BLOCK_INDEX).toUInt32
    else (0x100 : UInt32) <<< (n - LUMA_DC_BLOCK_INDEX).toUInt32
  let t := d.pic.cbpTable
  let d := { d with pic := { d.pic with cbpTable := .empty } }
  let t := u16set t (2 * xy) (u16 t (2 * xy) ||| bit)
  { d with pic := { d.pic with cbpTable := t } }

/-- `decode_cabac_residual_internal` after a set coded_block_flag. -/
def residualBlock (d : Dec) (cat n maxCoeff : Nat) (isDc : Bool) (dk off list qp : Nat) : Dec :=
  let sigB := sigOffset[cat]!
  let lastB := lastOffset[cat]!
  let d := if !isDc && maxCoeff == 64 then sigMap8x8 d sigB lastB 0 0 else sigMap d sigB lastB (maxCoeff - 1) 0 0
  let count := d.v.toNat
  -- record the coded block
  let d :=
    if isDc then (d.markDc cat n).setNnz (s8 n) count.toUInt8
    else if maxCoeff == 64 then d.fillNnzC (s8 n) 2 2 count.toUInt8
    else d.setNnz (s8 n) count.toUInt8
  levels d absOffset[cat]! maxCoeff dk off list qp 0 count

/-- `get_cabac_cbf_ctx`. -/
def cbfCtx (d : Dec) (cat idx : Nat) (isDc : Bool) : Nat :=
  let (nza, nzb) :=
    if isDc then
      if cat == 3 then
        let c := idx - CHROMA_DC_BLOCK_INDEX
        ((d.leftCbp >>> (6 + c)) &&& 1, (d.topCbp >>> (6 + c)) &&& 1)
      else (d.leftCbp &&& 0x100, d.topCbp &&& 0x100)
    else (d.nnzC[s8 idx - 1]!.toNat, d.nnzC[s8 idx - 8]!.toNat)
  cbfBase[cat]! + b01 (nza > 0) + 2 * b01 (nzb > 0)

def residualDc (d : Dec) (cat n maxCoeff dk off : Nat) : Dec :=
  let d := d.decisionAt (cbfCtx d cat n true)
  if d.v == 0 then d.setNnz (s8 n) 0
  else residualBlock d cat n maxCoeff true dk off 0 0

def residualNonDc (d : Dec) (cat n maxCoeff dk off list qp : Nat) : Dec :=
  if cat == 5 then residualBlock d cat n maxCoeff false dk off list qp
  else
    let d := d.decisionAt (cbfCtx d cat n false)
    if d.v == 0 then d.setNnz (s8 n) 0
    else residualBlock d cat n maxCoeff false dk off list qp

/-- Intra 16x16 AC blocks `i` .. 15. -/
def luma16Ac (d : Dec) (qp i : Nat) : Nat → Dec
  | 0 => d
  | k + 1 => luma16Ac (residualNonDc d 1 i 15 0 (16 * i) 0 qp) qp (i + 1) k

/-- 4x4 blocks `n` .. of an 8x8 block. -/
def luma4s (d : Dec) (cqm qp n : Nat) : Nat → Dec
  | 0 => d
  | k + 1 => luma4s (residualNonDc d 2 n 16 0 (16 * n) cqm qp) cqm qp (n + 1) k

/-- 8x8 blocks `i8` .. 3 (coded per `cbp`). -/
def luma8s (d : Dec) (t : UInt32) (cbp qp cqm i8 : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let d :=
      if (cbp.toUInt32 >>> i8.toUInt32) &&& 1 != 0 then
        if is8x8dct t then residualNonDc d 5 (4 * i8) 64 1 (64 * i8) (if isIntra t then 0 else 1) qp
        else luma4s d cqm qp (4 * i8) 4
      else d.fillNnzC (s8 (4 * i8)) 2 2 0
    luma8s d t cbp qp cqm (i8 + 1) k

/-- `decode_cabac_luma_residual` for the luma plane. -/
def lumaResidual (d : Dec) (t : UInt32) (cbp : Nat) : Dec :=
  let qp := d.qscale
  if isIntra16x16 t then
    let d := residualDc d.clearLumaDc 0 LUMA_DC_BLOCK_INDEX 16 3 0
    if cbp &&& 15 != 0 then luma16Ac d qp 0 16 else d.fillNnzC (s8 0) 4 4 0
  else luma8s d t cbp qp (if isIntra t then 0 else 3) 0 4

/-- Chroma AC blocks `i` .. 3 of plane `c`. -/
def chromaAc (d : Dec) (c list qp i : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let n := 16 + 16 * c + i
    chromaAc (residualNonDc d 4 n 15 0 (16 * n) list qp) c list qp (i + 1) k

/-- The chroma residual (DC of both planes, then their AC blocks). -/
def chromaResidual (d : Dec) (t : UInt32) (cbp : Nat) : Dec :=
  let d :=
    if cbp &&& 0x30 != 0 then residualDc (residualDc d 3 CHROMA_DC_BLOCK_INDEX 4 2 256) 3 (CHROMA_DC_BLOCK_INDEX + 1) 4 2 512
    else d
  if cbp &&& 0x20 != 0 then
    let li := if isIntra t then 0 else 3
    let d := chromaAc d 0 (1 + li) d.chromaQp0 0 4
    chromaAc d 1 (2 + li) d.chromaQp1 0 4
  else (d.fillNnzC (s8 16) 4 4 0).fillNnzC (s8 32) 4 4 0

/-! ## The macroblock -/

/-- Read the motion vector difference of block `n` (prediction (px, py)) and
store the vector and its mvd magnitudes over the w x h block at cache entry `e`. -/
def readStoreMv (d : Dec) (list n e w h : Nat) (px py : Int) : Dec :=
  let dc := d.mvdC list
  let i := s8 n
  let amvd0 : Int := dc[2 * (i - 1)]!.toNat + dc[2 * (i - 8)]!.toNat
  let amvd1 : Int := dc[2 * (i - 1) + 1]!.toNat + dc[2 * (i - 8) + 1]!.toNat
  let d := decodeMvd d 40 amvd0
  let mx := px + d.v
  let ax := d.aux
  let d := decodeMvd d 47 amvd1
  let my := py + d.v
  let d := d.fillMvC list e w h mx my
  d.fillMvdC list e w h ax.toUInt8 d.aux.toUInt8

def sixteens48 : ByteArray := Bytes.filled 48 16

/-! Setters for the macroblock layer, not inlined (RULES.md: an inlined
update next to borrowed reads of the state copies it whole). -/

@[noinline] def Dec.setMbXY (d : Dec) (xy : Nat) : Dec := { d with mbXY := xy }
@[noinline] def Dec.setPrevSkipped (d : Dec) (b : Bool) : Dec := { d with prevMbSkipped := b }
@[noinline] def Dec.setIntra16 (d : Dec) (m : Nat) : Dec := { d with intra16x16Mode := m }
@[noinline] def Dec.setChromaPredMode (d : Dec) (m : Nat) : Dec := { d with chromaPredMode := m }
@[noinline] def Dec.setI4 (d : Dec) (e : Nat) (v : UInt8) : Dec := { d with i4C := d.i4C.set! e v }
@[noinline] def Dec.markSlice (d : Dec) : Dec :=
  { d with pic := { d.pic with sliceTable := setU16 d.pic.sliceTable d.mbXY d.sliceNum } }
@[noinline] def Dec.setChromaPred (d : Dec) (v : UInt8) : Dec :=
  { d with pic := { d.pic with chromaPredTable := d.pic.chromaPredTable.set! d.mbXY v } }
@[noinline] def Dec.setCbp (d : Dec) (cbp : Nat) : Dec :=
  { d with cbp := cbp, pic := { d.pic with cbpTable := setU16 d.pic.cbpTable d.mbXY cbp } }
@[noinline] def Dec.setMbType (d : Dec) (t : UInt32) : Dec :=
  { d with pic := { d.pic with mbType := setU32 d.pic.mbType d.mbXY t } }
@[noinline] def Dec.storeQscale (d : Dec) : Dec :=
  { d with pic := { d.pic with qscaleTable := d.pic.qscaleTable.set! d.mbXY d.qscale.toUInt8 } }
/-- After a skipped macroblock: no residual, chroma prediction DC. -/
@[noinline] def Dec.finishSkip (d : Dec) : Dec :=
  let xy := d.mbXY.toUSize
  let (ct, pt) := (d.pic.cbpTable, d.pic.chromaPredTable)
  let d := { d with pic := { d.pic with cbpTable := .empty, chromaPredTable := .empty } }
  let ct := u16set ct (2 * xy) 0
  let pt := u8set pt xy 0
  { d with lastQscaleDiff := 0, pic := { d.pic with cbpTable := ct, chromaPredTable := pt } }

/-- A skipped macroblock's non_zero_count table entries: zero. -/
@[noinline] def Dec.zeroNnzTable (d : Dec) : Dec :=
  let nnz := d.pic.nnzTable
  let d := { d with pic := { d.pic with nnzTable := .empty } }
  { d with pic := { d.pic with nnzTable := Kernels.fillRaw nnz (48 * d.mbXY.toUSize) 0 12 1 0 4 } }

/-- A skipped macroblock's type `t`, qscale and slice into the tables. -/
@[noinline] def Dec.storeSkipped (d : Dec) (t : UInt32) : Dec :=
  let xy := d.mbXY.toUSize
  let q := d.qscale.toUInt8
  let sn := d.sliceNum.toUInt32
  let (mt, qt, st) := (d.pic.mbType, d.pic.qscaleTable, d.pic.sliceTable)
  let d := { d with pic := { d.pic with mbType := .empty, qscaleTable := .empty, sliceTable := .empty } }
  let mt := u32set mt (4 * xy) t
  let qt := u8set qt xy q
  let st := u16set st (2 * xy) sn
  { d with prevMbSkipped := true, pic := { d.pic with mbType := mt, qscaleTable := qt, sliceTable := st } }

/-- `decode_mb_skip`. -/
def decodeSkip (d : Dec) : Dec :=
  let d := d.zeroNnzTable
  if d.sl.sliceType == .b then
    let t := MB_L0L1 ||| MB_DIRECT2 ||| MB_SKIP
    let d := if d.sl.directSpatial then fillCaches (fillNeighbors d) t else d
    let d := predDirectMotion d t
    let t := d.dirType ||| MB_SKIP
    (writeBackMotion d t).storeSkipped t
  else
    let t := MB_16x16 ||| MB_P0L0 ||| MB_P1L0 ||| MB_SKIP
    (writeBackMotion (predPskip (fillNeighbors d)) t).storeSkipped t

/-- The intra 4x4 mode cache `c` with modes that need an unavailable
neighbour replaced (FFmpeg's check_intra4x4_pred_mode); empty if one cannot be. -/
def fixIntra4x4 (c : ByteArray) (topSamples leftSamples : UInt32) : ByteArray := Id.run do
  let top : Array Int := #[-1, 0, 9, -1, -1, -1, -1, -1, 0, 0, 0, 0]
  let left : Array Int := #[0, -1, 10, 0, -1, -1, -1, 0, -1, 11, 0, 0]
  let mut c := c
  if topSamples &&& 0x8000 == 0 then
    for i in [0:4] do
      let st := top[(getI8 c (s8 0 + i)).toNat]!
      if st < 0 then return .empty
      if st > 0 then c := setI8 c (s8 0 + i) st
  if leftSamples &&& 0x8888 != 0x8888 then
    let masks : Array UInt32 := #[0x8000, 0x2000, 0x80, 0x20]
    for i in [0:4] do
      if leftSamples &&& masks[i]! == 0 then
        let st := left[(getI8 c (s8 0 + 8 * i)).toNat]!
        if st < 0 then return .empty
        if st > 0 then c := setI8 c (s8 0 + 8 * i) st
  return c

/-- `ff_h264_check_intra4x4_pred_mode` on the cache. -/
def checkIntra4x4 (d : Dec) : Dec :=
  let ts := d.topSamples
  let ls := d.leftSamples
  let c := d.i4C
  let d := d.setI4C .empty
  let c := fixIntra4x4 c ts ls
  if c.size == 0 then d.setErr "H.264: block unavailable for intra 4x4 mode" else d.setI4C c

/-- `ff_h264_check_intra_pred_mode` (16x16 and chroma) -> v (-1: error). -/
def checkIntraMode (d : Dec) (mode : Nat) (isChroma : Bool) : Int := Id.run do
  let top : Array Int := #[4, 1, -1, -1]
  let left : Array Int := #[5, -1, 2, -1, 6]
  if mode > 3 then return -1
  let mut m : Int := mode
  if d.topSamples &&& 0x8000 == 0 then
    m := top[m.toNat]!
    if m < 0 then return -1
  if d.leftSamples &&& 0x8080 != 0x8080 then
    m := left[m.toNat]!
    if m < 0 then return -1
    if isChroma && d.leftSamples &&& 0x8080 != 0 then
      m := 7 + b01 (d.leftSamples &&& 0x8000 == 0) + 2 * b01 (m == 6)
  return m

/-- `get_dct8x8_allowed`. -/
def dct8x8AllowedSubs (d : Dec) : Bool :=
  let m := MB_16x8 ||| MB_8x16 ||| MB_8x8 ||| (if d.sl.direct8x8Inference then 0 else MB_DIRECT2)
  d.sub 0 &&& m == 0 && d.sub 1 &&& m == 0 && d.sub 2 &&& m == 0 && d.sub 3 &&& m == 0

/-! The macroblock's loops, as recursions (a `for` over a range allocates
the range each time it starts). -/

/-- Intra 4x4 prediction modes of blocks `i` .. 15. -/
def intraModes4 (d : Dec) (i : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let d := decodeIntra4x4Mode d (predIntraMode d i)
    intraModes4 (d.setI4 (s8 i) d.v.toNat.toUInt8) (i + 1) k

/-- Intra 8x8 prediction modes of 8x8 blocks `i8` .. 3. -/
def intraModes8 (d : Dec) (i8 : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let d := decodeIntra4x4Mode d (predIntraMode d (4 * i8))
    intraModes8 (d.fillI4C (s8 (4 * i8)) 2 2 d.v.toNat.toUInt8) (i8 + 1) k

/-- The sub-macroblock types of 8x8 blocks `i` .. 3. -/
def subTypes (d : Dec) (isB : Bool) (i : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let d := if isB then decodeBSubType d else decodePSubType d
    let st := if isB then bSubInfo[d.v.toNat]!.1 else pSubInfo[d.v.toNat]!.1
    subTypes (d.setSub i st) isB (i + 1) k

/-- The number of partitions of sub-macroblock type `sub`. -/
@[inline] def subPartCount (sub : UInt32) : Nat :=
  if sub &&& MB_16x16 != 0 then 1 else if sub &&& (MB_16x8 ||| MB_8x16) != 0 then 2 else 4

/-- The direct sub-blocks' caches after spatial/temporal direct prediction:
references of blocks 4 and 12 unavailable, the direct flags of 8x8 blocks `i` .. 3. -/
def directSubCaches (d : Dec) (i : Nat) : Nat → Dec
  | 0 =>
    let pna := toByte PART_NOT_AVAILABLE
    (((d.fillRefC 0 (s8 4) 1 1 pna).fillRefC 0 (s8 12) 1 1 pna).fillRefC 1 (s8 4) 1 1 pna).fillRefC 1 (s8 12) 1 1 pna
  | k + 1 => directSubCaches (d.fillDirectC (s8 (4 * i)) 2 2 (d.sub i >>> (1 : UInt32)).toUInt8) (i + 1) k

/-- Reference indices of `list` for 8x8 blocks `i` .. 3 (an error stops). -/
def subRefs (d : Dec) (list i : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let sub := d.sub i
    if isDirect sub then subRefs d list (i + 1) k
    else
      let (d, ref) :=
        if isDir sub 0 list then
          if d.refCount list > 1 then let d := decodeRef d list (4 * i); (d, d.v) else (d, 0)
        else (d, -1)
      if isDir sub 0 list && d.refCount list > 1 && (ref < 0 || ref ≥ d.refCount list) then
        d.setErr "H.264: bad reference index"
      else
        let e := s8 (4 * i)
        subRefs (d.setRefC list (setI8 (setI8 (setI8 (d.refC list) (e + 1) ref) (e + 8) ref) (e + 9) ref)) list (i + 1) k

/-- The motion of partitions `j` .. of sub-macroblock `i` (type `sub`, width `bw` blocks). -/
def subPartMvs (d : Dec) (list i : Nat) (sub : UInt32) (bw j : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let idx := 4 * i + bw * j
    let (px, py) := predMotion d idx bw list (getI8 (d.refC list) (s8 idx))
    let w := if sub &&& (MB_16x16 ||| MB_16x8) != 0 then 2 else 1
    let h := if sub &&& (MB_16x16 ||| MB_8x16) != 0 then 2 else 1
    subPartMvs (readStoreMv d list idx (s8 idx) w h px py) list i sub bw (j + 1) k

/-- The motion vectors of `list` for 8x8 blocks `i` .. 3. -/
def subMvs (d : Dec) (list i : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let e := s8 (4 * i)
    let d := d.setRefC list (setI8 (d.refC list) e (getI8 (d.refC list) (e + 1)))
    let sub := d.sub i
    let d :=
      if isDirect sub then d.fillMvdC list e 2 2 0 0
      else if isDir sub 0 list then
        subPartMvs d list i sub (if sub &&& (MB_16x16 ||| MB_16x8) != 0 then 2 else 1) 0 (subPartCount sub)
      else (d.fillMvC list e 2 2 0 0).fillMvdC list e 2 2 0 0
    subMvs d list (i + 1) k

/-- Partition `i` of a 16x16/16x8/8x16 macroblock: (block, cache entry, width, height). -/
@[inline] def partN (t : UInt32) (i : Nat) : Nat := if i == 0 then 0 else if t &&& MB_16x8 != 0 then 8 else 4
@[inline] def partE (t : UInt32) (i : Nat) : Nat := if i == 0 then 12 else if t &&& MB_16x8 != 0 then 28 else 14
@[inline] def partW (t : UInt32) : Nat := if t &&& MB_8x16 != 0 then 2 else 4
@[inline] def partH (t : UInt32) : Nat := if t &&& MB_16x8 != 0 then 2 else 4

/-- Reference indices of `list` for partitions `i` .. (an error stops). -/
def partRefs (d : Dec) (t : UInt32) (list i : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let e := partE t i
    if isDir t i list then
      let (d, ref) := if d.refCount list > 1 then let d := decodeRef d list (partN t i); (d, d.v) else (d, 0)
      if d.refCount list > 1 && (ref < 0 || ref ≥ d.refCount list) then d.setErr "H.264: bad reference index"
      else partRefs (d.fillRefC list e (partW t) (partH t) (toByte ref)) t list (i + 1) k
    else partRefs (d.fillRefC list e (partW t) (partH t) (toByte LIST_NOT_USED)) t list (i + 1) k

/-- The motion vectors of `list` for partitions `i` ... -/
def partMvs (d : Dec) (t : UInt32) (list i : Nat) : Nat → Dec
  | 0 => d
  | k + 1 =>
    let (n, e, w, h) := (partN t i, partE t i, partW t, partH t)
    let d :=
      if isDir t i list then
        let ref := getI8 (d.refC list) e
        let (px, py) :=
          if t &&& MB_16x16 != 0 then predMotion d 0 4 list ref
          else if t &&& MB_16x8 != 0 then pred16x8Motion d n list ref
          else pred8x16Motion d n list ref
        readStoreMv d list n e w h px py
      else (d.fillMvdC list e w h 0 0).fillMvC list e w h 0 0
    partMvs d t list (i + 1) k

/-- One macroblock (`ff_h264_decode_mb_cabac`). The macroblock type ends up
in `d.pic.mbType`; errors in `d.err`. -/
def decodeMb (d : Dec) : Dec := Id.run do
  let mut d := d.setMbXY (d.mbY * d.mbW + d.mbX)
  let mut dct8x8Allowed := d.sl.transform8x8Mode
  if d.sl.sliceType != .i then
    d := decodeMbSkip d
    if d.v == 1 then
      d := decodeSkip d
      return d.finishSkip
  d := d.setPrevSkipped false
  d := fillNeighbors d
  let mut t : UInt32 := 0
  let mut partitions := 0
  -- the intra macroblock type index, or 0xFFFF
  let mut intraIdx : Nat := 0xFFFF
  if d.sl.sliceType == .b then
    -- FFmpeg's !IS_DIRECT(type - 1): available and not direct (stored direct
    -- types always carry a partition bit, so type - 1 keeps DIRECT2)
    d := d.decisionAt (27 + b01 (d.leftType != 0 && !isDirect d.leftType) + b01 (d.topType != 0 && !isDirect d.topType))
    let mut bt := 0
    if d.v == 0 then bt := 0
    else
      d := d.decision (27 + 3)
      if d.v == 0 then
        d := d.decision (27 + 5)
        bt := 1 + d.v.toNat
      else
        d := d.decision (27 + 4)
        let mut bits := 8 * d.v.toNat
        d := d.decision (27 + 5)
        bits := bits + 4 * d.v.toNat
        d := d.decision (27 + 5)
        bits := bits + 2 * d.v.toNat
        d := d.decision (27 + 5)
        bits := bits + d.v.toNat
        if bits < 8 then bt := bits + 3
        else if bits == 13 then
          d := decodeIntraMbType d 32 false
          intraIdx := d.v.toNat
        else if bits == 14 then bt := 11
        else if bits == 15 then bt := 22
        else
          d := d.decision (27 + 5)
          bt := 2 * bits + d.v.toNat - 4
    if intraIdx == 0xFFFF then
      partitions := bMbInfo[bt]!.2
      t := bMbInfo[bt]!.1
  else if d.sl.sliceType == .p then
    d := d.decision 14
    if d.v == 0 then
      d := d.decision 15
      let mut pt := 0
      if d.v == 0 then
        d := d.decision 16
        pt := 3 * d.v.toNat
      else
        d := d.decision 17
        pt := 2 - d.v.toNat
      partitions := pMbInfo[pt]!.2
      t := pMbInfo[pt]!.1
    else
      d := decodeIntraMbType d 17 false
      intraIdx := d.v.toNat
  else
    d := decodeIntraMbType d 3 true
    intraIdx := d.v.toNat
  let mut cbp := 0
  if intraIdx != 0xFFFF then
    let (it, pm, c) := iMbInfo intraIdx
    t := it
    cbp := c
    d := d.setIntra16 pm
  d := d.markSlice
  if isPcm t then
    -- the samples start at the byte boundary after the terminate bin; CABAC
    -- restarts after them, keeping its contexts
    let pos := (d.cabac.bitPos.toNat + 7) / 8
    if pos + 384 > d.cabac.data.size then return d.setErr "H.264: truncated I_PCM macroblock"
    let nnz := sixteens48.copySlice 0 d.pic.nnzTable (48 * d.mbXY) 48
    return { d with pcm := d.cabac.data.extract pos (pos + 384),
                    cabac := Cabac.init d.cabac.data (pos + 384) d.cabac.ctx,
                    lastQscaleDiff := 0, pic := { d.pic with cbpTable := setU16 d.pic.cbpTable d.mbXY 0xf7ef, chromaPredTable := d.pic.chromaPredTable.set! d.mbXY 0, qscaleTable := d.pic.qscaleTable.set! d.mbXY 0, nnzTable := nnz, mbType := setU32 d.pic.mbType d.mbXY t } }
  d := fillCaches d t
  if isIntra t then
    if isIntra4x4 t then
      let use8x8 ←
        if dct8x8Allowed then
          d := d.decisionAt (399 + d.neighborTransformSize)
          pure (d.v == 1)
        else pure false
      if use8x8 then
        t := t ||| MB_8x8DCT
        d := intraModes8 d 0 4
      else
        d := intraModes4 d 0 16
      d := writeBackIntraModes d
      d := checkIntra4x4 d
      if d.err.isSome then return d
    else
      let m := checkIntraMode d d.intra16x16Mode false
      if m < 0 then return d.setErr "H.264: bad intra 16x16 mode"
      d := d.setIntra16 m.toNat
    d := decodeChromaPredMode d
    let pm := d.v.toNat
    d := d.setChromaPred pm.toUInt8
    let m := checkIntraMode d pm true
    if m < 0 then return d.setErr "H.264: bad intra chroma mode"
    d := d.setChromaPredMode m.toNat
  else if partitions == 4 then
    let isB := d.sl.sliceType == .b
    d := subTypes d isB 0 4
    if isB && (isDirect (d.sub 0) || isDirect (d.sub 1) || isDirect (d.sub 2) || isDirect (d.sub 3)) then
      d := predDirectMotion d t
      t := d.dirType
      d := directSubCaches d 0 4
    d := subRefs d 0 0 4
    if d.sl.listCount == 2 && d.err.isNone then d := subRefs d 1 0 4
    if d.err.isSome then return d
    if dct8x8Allowed then dct8x8Allowed := dct8x8AllowedSubs d
    d := subMvs d 0 0 4
    if d.sl.listCount == 2 then d := subMvs d 1 0 4
  else if isDirect t then
    d := predDirectMotion d t
    t := d.dirType
    d := d.fillMvdC 0 12 4 4 0 0
    d := d.fillMvdC 1 12 4 4 0 0
    dct8x8Allowed := dct8x8Allowed && d.sl.direct8x8Inference
  else
    let parts := if t &&& MB_16x16 != 0 then 1 else 2
    d := partRefs d t 0 0 parts
    if d.sl.listCount == 2 && d.err.isNone then d := partRefs d t 1 0 parts
    if d.err.isSome then return d
    d := partMvs d t 0 0 parts
    if d.sl.listCount == 2 then d := partMvs d t 1 0 parts
  if d.err.isSome then return d
  if isInter t then
    d := d.setChromaPred 0
    d := writeBackMotion d t
  if !isIntra16x16 t then
    d := decodeCbpLuma d
    cbp := d.v.toNat
    d := decodeCbpChroma d
    cbp := cbp ||| (16 * d.v.toNat)
  d := d.setCbp cbp
  if dct8x8Allowed && cbp &&& 15 != 0 && !isIntra t then
    d := d.decisionAt (399 + d.neighborTransformSize)
    if d.v == 1 then t := t ||| MB_8x8DCT
  d := d.setMbType t
  if cbp != 0 || isIntra16x16 t then
    d := d.decisionAt (60 + b01 (d.lastQscaleDiff != 0))
    if d.v == 1 then
      let mut val : Int := 1
      let mut ctx := 2
      repeat
        d := d.decision (60 + ctx)
        if d.v == 0 then break
        ctx := 3
        val := val + 1
        if val > 2 * 51 then return d.setErr "H.264: bad mb_qp_delta"
      let delta := if val % 2 == 1 then (val + 1) / 2 else -((val + 1) / 2)
      let mut q := Int.ofNat d.qscale + delta
      if q < 0 then q := q + 52 else if q > 51 then q := q - 52
      d := d.setQscale delta q.toNat d.sl.cqpTab0[q.toNat]!.toNat d.sl.cqpTab1[q.toNat]!.toNat
    else
      d := d.setLastQscaleDiff 0
    d := chromaResidual (lumaResidual d t cbp) t cbp
  else
    d := (((d.fillNnzC (s8 0) 4 4 0).fillNnzC (s8 16) 4 4 0).fillNnzC (s8 32) 4 4 0).setLastQscaleDiff 0
  d := d.storeQscale
  return writeBackNnz d

end Yt.H264
