/-! H.264 inverse transforms for 8-bit samples.

A translation of FFmpeg 9.0.1's `libavcodec/h264idct_template.c` at
`BIT_DEPTH` 8 (copyright (c) 2004-2011 Michael Niedermayer):
`ff_h264_idct_add_8_c`, `ff_h264_idct8_add_8_c`, `ff_h264_idct_dc_add_8_c`,
`ff_h264_idct8_dc_add_8_c`, `ff_h264_luma_dc_dequant_idct_8_c` and
`ff_h264_chroma_dc_dequant_idct_8_c`. Results are identical to FFmpeg's C
functions, including its 16-bit coefficient storage (`dctcoef` is `int16_t`
at 8 bits): input coefficients, the rounded DC, the intermediate rows of the
4x4/8x8 transforms and the DC transform outputs are all wrapped to 16 bits
as FFmpeg's stores wrap them; 32-bit arithmetic wraps as FFmpeg's unsigned
arithmetic does. The wraps only show on extreme coefficients (ITU-T H.264
8.5.12 keeps a conforming stream's intermediates within 16 bits).

License: like FFmpeg, this module is free software; you can redistribute it
and/or modify it under the terms of the GNU Lesser General Public License as
published by the Free Software Foundation; either version 2.1 of the
License, or (at your option) any later version (LGPL-2.1-or-later). It is
distributed WITHOUT ANY WARRANTY; see the GNU LGPL for details.

Coefficient layout: `Array Int` in raster order, index `row * N + col`,
where the row is the vertical frequency and the column the horizontal one
(the inverse-scanned matrix; FFmpeg's block is the transpose: it stores
coefficient (row, col) at `col * N + row`). Values are the dequantized
coefficients handed to the transform (8.5.12.1). Missing elements count as
0. Planes are `ByteArray`s of samples with a row `stride`; the block's
top-left sample is at byte offset `off`; out-of-plane samples read as 0
and are not written. Pass the plane uniquely so it is updated in place. -/
namespace Yt.H264.Transform

private theorem usize_lt (a : ByteArray) (i : USize) (h : i < a.usize) : i.toNat < a.size := by
  have h1 : i.toNat < a.usize.toNat := h
  simp [ByteArray.usize] at h1
  exact Nat.lt_of_lt_of_le h1 (Nat.mod_le _ _)

@[inline] private def ld (p : ByteArray) (i : USize) : UInt32 :=
  if h : i < p.usize then (p.uget i (usize_lt p i h)).toUInt32 else 0

@[inline] private def st (p : ByteArray) (i : USize) (v : UInt32) : ByteArray :=
  if h : i < p.usize then p.uset i v.toUInt8 (usize_lt p i h) else p

/-- Arithmetic shift right of a two's-complement 32-bit value. -/
@[inline] private def sar (x n : UInt32) : UInt32 :=
  let m : UInt32 := (0 : UInt32) - (x >>> 31)
  ((x ^^^ m) >>> n) ^^^ m

/-- Wrap to a signed 16-bit value (sign-extended into 32 bits). -/
@[inline] private def s16 (x : UInt32) : UInt32 := ((x &&& 0xFFFF) ^^^ 0x8000) - 0x8000

/-- Coefficient `k` of `c` (which has at least `n` elements) as FFmpeg's
`int16_t`. -/
@[inline] private def cf (c : Array Int) {n : Nat} (h : n ≤ c.size) (k : Nat) (hk : k < n) :
    UInt32 :=
  s16 (c[k]'(Nat.lt_of_lt_of_le hk h)).toInt32.toUInt32

/-- `c` extended with zeros to at least `n` elements (itself when long
enough), so that missing coefficients count as 0. -/
private def pad (c : Array Int) (n : Nat) : Array Int :=
  if c.size < n then pad (c.push 0) n else c
termination_by n - c.size
decreasing_by simp [Array.size_push]; omega

/-- A two's-complement 32-bit value as an `Int`. -/
@[inline] private def toInt (v : UInt32) : Int :=
  if v >>> 31 == 0 then Int.ofNat v.toNat else -Int.ofNat ((0 : UInt32) - v).toNat

/-- `dst = av_clip_uint8(dst + r)`. -/
@[inline] private def addClip (p : ByteArray) (i : USize) (r : UInt32) : ByteArray :=
  let x := ld p i + r
  st p i (if x >>> 31 != 0 then 0 else if x > 255 then 255 else x)

/-! ### 4x4 -/

/-- The first (horizontal) pass of the 4x4 transform on one row, the four
results wrapped to 16 bits and packed (sample `k` in bits `16k..16k+15`). -/
@[inline] private def row4 (a0 a1 a2 a3 : UInt32) : UInt64 :=
  let z0 := a0 + a2
  let z1 := a0 - a2
  let z2 := sar a1 1 - a3
  let z3 := a1 + sar a3 1
  ((z0 + z3) &&& 0xFFFF).toUInt64 ||| (((z1 + z2) &&& 0xFFFF).toUInt64 <<< 16) |||
  (((z1 - z2) &&& 0xFFFF).toUInt64 <<< 32) ||| (((z0 - z3) &&& 0xFFFF).toUInt64 <<< 48)

/-- Row `y` of the first pass, coefficients `4y .. 4y+3`; `r` (the rounding
32 for row 0, else 0) is added to the first coefficient with 16-bit wrap. -/
@[inline] private def row4At (c : Array Int) (h : 16 ≤ c.size) (y : Nat) (hy : y < 4)
    (r : UInt32) : UInt64 :=
  row4 (s16 (cf c h (4 * y) (by omega) + r)) (cf c h (4 * y + 1) (by omega))
    (cf c h (4 * y + 2) (by omega)) (cf c h (4 * y + 3) (by omega))

/-- Element `sh / 16` of a packed row. -/
@[inline] private def h16 (r : UInt64) (sh : UInt64) : UInt32 := s16 (r >>> sh).toUInt32

/-- The second (vertical) pass on one column, `(x + 32) >> 6` (the 32 was
added to the DC up front) added to the four samples from `i` down. -/
private def col4 (p : ByteArray) (i s : USize) (a0 a1 a2 a3 : UInt32) : ByteArray :=
  let z0 := a0 + a2
  let z1 := a0 - a2
  let z2 := sar a1 1 - a3
  let z3 := a1 + sar a3 1
  let p := addClip p i (sar (z0 + z3) 6)
  let p := addClip p (i + s) (sar (z1 + z2) 6)
  let p := addClip p (i + 2 * s) (sar (z1 - z2) 6)
  addClip p (i + 3 * s) (sar (z0 - z3) 6)

/-- Add the 4x4 inverse transform (8.5.12.2) of the 16 coefficients `c`
(raster, `c[row * 4 + col]`) to the block at `off`, clipping to 0..255:
FFmpeg's `ff_h264_idct_add_8_c`. -/
def idct4Add (p : ByteArray) (off stride : Nat) (c : Array Int) : ByteArray :=
  let c := pad c 16
  if h : 16 ≤ c.size then
    let o := off.toUSize
    let s := stride.toUSize
    let r0 := row4At c h 0 (by decide) 32
    let r1 := row4At c h 1 (by decide) 0
    let r2 := row4At c h 2 (by decide) 0
    let r3 := row4At c h 3 (by decide) 0
    let p := col4 p o s (h16 r0 0) (h16 r1 0) (h16 r2 0) (h16 r3 0)
    let p := col4 p (o + 1) s (h16 r0 16) (h16 r1 16) (h16 r2 16) (h16 r3 16)
    let p := col4 p (o + 2) s (h16 r0 32) (h16 r1 32) (h16 r2 32) (h16 r3 32)
    col4 p (o + 3) s (h16 r0 48) (h16 r1 48) (h16 r2 48) (h16 r3 48)
  else p

/-! ### 8x8 -/

/-- Eight values: one row of the 8x8 first pass. -/
private structure V8 where
  (v0 v1 v2 v3 v4 v5 v6 v7 : UInt32)

/-- The first (horizontal) pass of the 8x8 transform on one row, results
wrapped to 16 bits. Inlined, so the `V8` is never allocated. -/
@[inline] private def row8 (c0 c1 c2 c3 c4 c5 c6 c7 : UInt32) : V8 :=
  let a0 := c0 + c4
  let a2 := c0 - c4
  let a4 := sar c2 1 - c6
  let a6 := sar c6 1 + c2
  let b0 := a0 + a6
  let b2 := a2 + a4
  let b4 := a2 - a4
  let b6 := a0 - a6
  let a1 := c5 - c3 - c7 - sar c7 1
  let a3 := c1 + c7 - c3 - sar c3 1
  let a5 := c7 - c1 + c5 + sar c5 1
  let a7 := c3 + c5 + c1 + sar c1 1
  let b1 := sar a7 2 + a1
  let b3 := a3 + sar a5 2
  let b5 := sar a3 2 - a5
  let b7 := a7 - sar a1 2
  ⟨s16 (b0 + b7), s16 (b2 + b5), s16 (b4 + b3), s16 (b6 + b1),
   s16 (b6 - b1), s16 (b4 - b3), s16 (b2 - b5), s16 (b0 - b7)⟩

/-- Row `y` of the first pass, coefficients `8y .. 8y+7`; `r` (the rounding
32 for row 0, else 0) is added to the first coefficient with 16-bit wrap. -/
@[inline] private def rowAt (c : Array Int) (h : 64 ≤ c.size) (y : Nat) (hy : y < 8)
    (r : UInt32) : V8 :=
  row8 (s16 (cf c h (8 * y) (by omega) + r)) (cf c h (8 * y + 1) (by omega))
    (cf c h (8 * y + 2) (by omega))
    (cf c h (8 * y + 3) (by omega)) (cf c h (8 * y + 4) (by omega)) (cf c h (8 * y + 5) (by omega))
    (cf c h (8 * y + 6) (by omega)) (cf c h (8 * y + 7) (by omega))

/-- The second (vertical) pass on one column `u0..u7` (rows 0..7), each
result `>> 6` added to the samples from `i` down. -/
private def col8 (p : ByteArray) (i s : USize) (u0 u1 u2 u3 u4 u5 u6 u7 : UInt32) : ByteArray :=
  let a0 := u0 + u4
  let a2 := u0 - u4
  let a4 := sar u2 1 - u6
  let a6 := sar u6 1 + u2
  let b0 := a0 + a6
  let b2 := a2 + a4
  let b4 := a2 - a4
  let b6 := a0 - a6
  let a1 := u5 - u3 - u7 - sar u7 1
  let a3 := u1 + u7 - u3 - sar u3 1
  let a5 := u7 - u1 + u5 + sar u5 1
  let a7 := u3 + u5 + u1 + sar u1 1
  let b1 := sar a7 2 + a1
  let b3 := a3 + sar a5 2
  let b5 := sar a3 2 - a5
  let b7 := a7 - sar a1 2
  let p := addClip p i (sar (b0 + b7) 6)
  let p := addClip p (i + s) (sar (b2 + b5) 6)
  let p := addClip p (i + 2 * s) (sar (b4 + b3) 6)
  let p := addClip p (i + 3 * s) (sar (b6 + b1) 6)
  let p := addClip p (i + 4 * s) (sar (b6 - b1) 6)
  let p := addClip p (i + 5 * s) (sar (b4 - b3) 6)
  let p := addClip p (i + 6 * s) (sar (b2 - b5) 6)
  addClip p (i + 7 * s) (sar (b0 - b7) 6)

/-- Add the 8x8 inverse transform (8.5.12.2) of the 64 coefficients `c`
(raster, `c[row * 8 + col]`) to the block at `off`, clipping to 0..255:
FFmpeg's `ff_h264_idct8_add_8_c`. -/
def idct8Add (p : ByteArray) (off stride : Nat) (c : Array Int) : ByteArray :=
  let c := pad c 64
  if h : 64 ≤ c.size then
    let o := off.toUSize
    let s := stride.toUSize
    let r0 := rowAt c h 0 (by decide) 32
    let r1 := rowAt c h 1 (by decide) 0
    let r2 := rowAt c h 2 (by decide) 0
    let r3 := rowAt c h 3 (by decide) 0
    let r4 := rowAt c h 4 (by decide) 0
    let r5 := rowAt c h 5 (by decide) 0
    let r6 := rowAt c h 6 (by decide) 0
    let r7 := rowAt c h 7 (by decide) 0
    let p := col8 p o s r0.v0 r1.v0 r2.v0 r3.v0 r4.v0 r5.v0 r6.v0 r7.v0
    let p := col8 p (o + 1) s r0.v1 r1.v1 r2.v1 r3.v1 r4.v1 r5.v1 r6.v1 r7.v1
    let p := col8 p (o + 2) s r0.v2 r1.v2 r2.v2 r3.v2 r4.v2 r5.v2 r6.v2 r7.v2
    let p := col8 p (o + 3) s r0.v3 r1.v3 r2.v3 r3.v3 r4.v3 r5.v3 r6.v3 r7.v3
    let p := col8 p (o + 4) s r0.v4 r1.v4 r2.v4 r3.v4 r4.v4 r5.v4 r6.v4 r7.v4
    let p := col8 p (o + 5) s r0.v5 r1.v5 r2.v5 r3.v5 r4.v5 r5.v5 r6.v5 r7.v5
    let p := col8 p (o + 6) s r0.v6 r1.v6 r2.v6 r3.v6 r4.v6 r5.v6 r6.v6 r7.v6
    col8 p (o + 7) s r0.v7 r1.v7 r2.v7 r3.v7 r4.v7 r5.v7 r6.v7 r7.v7
  else p

/-! ### DC only -/

/-- Add `d` to `n` samples from `i`, clipping. -/
private def addRow (p : ByteArray) (i : USize) (d : UInt32) : Nat → ByteArray
  | 0 => p
  | n + 1 => addRow (addClip p i d) (i + 1) d n

/-- Add `d` to `h` rows of `n` samples from `i`, clipping. -/
private def addRows (p : ByteArray) (i s : USize) (d : UInt32) (n : Nat) : Nat → ByteArray
  | 0 => p
  | h + 1 => addRows (addRow p i d n) (i + s) s d n h

/-- Add a 4x4 block whose only nonzero coefficient is the DC `dc`
(`c[0]`): FFmpeg's `ff_h264_idct_dc_add_8_c`, `(dc + 32) >> 6` added to
the 16 samples. FFmpeg uses it instead of `idct4Add` when a block's only
coded coefficient is a nonzero DC (and for Intra16x16 / chroma blocks with
no coded AC); the two differ when `dc + 32` overflows 16 bits. -/
def idct4DcAdd (p : ByteArray) (off stride : Nat) (dc : Int) : ByteArray :=
  let d := sar (s16 dc.toInt32.toUInt32 + 32) 6
  addRows p off.toUSize stride.toUSize d 4 4

/-- The 8x8 counterpart of `idct4DcAdd` (FFmpeg `ff_h264_idct8_dc_add_8_c`). -/
def idct8DcAdd (p : ByteArray) (off stride : Nat) (dc : Int) : ByteArray :=
  let d := sar (s16 dc.toInt32.toUInt32 + 32) 6
  addRows p off.toUSize stride.toUSize d 8 8

/-! ### DC transforms -/

/-- `(int16)(((x * q + 128) as int32) >> 8)`, the luma DC dequantization. -/
@[inline] private def dqL (x q : UInt32) : Int := toInt (s16 (sar (x * q + 128) 8))

/-- Four values (one column of the luma DC first pass). -/
private structure V4 where
  (v0 v1 v2 v3 : UInt32)

/-- The vertical pass of the luma DC transform on column `bx` (FFmpeg's
first loop): rows 0..3 of the result. Inlined, so the `V4` is never
allocated. -/
@[inline] private def lumaDcCol (c : Array Int) (h : 16 ≤ c.size) (bx : Nat) (hb : bx < 4) : V4 :=
  let x0 := cf c h bx (by omega)
  let x1 := cf c h (4 + bx) (by omega)
  let x2 := cf c h (8 + bx) (by omega)
  let x3 := cf c h (12 + bx) (by omega)
  let z0 := x0 + x1; let z1 := x0 - x1; let z2 := x2 - x3; let z3 := x2 + x3
  ⟨z0 + z3, z0 - z3, z1 - z2, z1 + z2⟩

/-- The horizontal pass of the luma DC transform on one row (columns
`t0..t3`), appending the four dequantized values. -/
private def lumaDcRow (o : Array Int) (q t0 t1 t2 t3 : UInt32) : Array Int :=
  let z0 := t0 + t2; let z1 := t0 - t2; let z2 := t1 - t3; let z3 := t1 + t3
  let o := (o.push (dqL (z0 + z3) q)).push (dqL (z1 + z2) q)
  (o.push (dqL (z1 - z2) q)).push (dqL (z0 - z3) q)

/-- Intra16x16 luma DC: the inverse Hadamard transform of the 16 DC levels
and their dequantization (8.5.10), FFmpeg's `ff_h264_luma_dc_dequant_idct_8_c`.
`c` holds the (not dequantized) Intra16x16DCLevel values inverse-scanned
into the 4x4 matrix exactly like a 4x4 block's coefficients (raster
`c[row * 4 + col]`, same zig-zag/field scan table). The result is raster by
block position: index `by * 4 + bx` is the DC coefficient (index 0 of its
coefficients) of the 4x4 block in column `bx`, row `by` of the macroblock
(`luma4x4BlkIdx = 8 * (by / 2) + 4 * (bx / 2) + 2 * (by % 2) + bx % 2`).
Each output is `((f * qmul + 128) >> 8)` wrapped to 16 bits, with `f` the
transform value.

`qmul` is FFmpeg's `dequant4_coeff[0][qP][0]` for the macroblock's luma
`qP`: `LevelScale4x4(qP % 6, 0, 0) << (qP / 6 + 2)` where
`LevelScale4x4(m, 0, 0) = weightScale4x4(0, 0) * normAdjust4x4(m, 0, 0)`
(the Intra Y scaling list; 16 * {10, 11, 13, 14, 16, 18}[m] when flat). -/
def lumaDcDequantIdct (c : Array Int) (qmul : Int) : Array Int :=
  let c := pad c 16
  if h : 16 ≤ c.size then
    let q := qmul.toInt32.toUInt32
    let k0 := lumaDcCol c h 0 (by decide)
    let k1 := lumaDcCol c h 1 (by decide)
    let k2 := lumaDcCol c h 2 (by decide)
    let k3 := lumaDcCol c h 3 (by decide)
    -- Horizontal pass over each row (FFmpeg's second loop), raster output.
    let o := lumaDcRow (Array.emptyWithCapacity 16) q k0.v0 k1.v0 k2.v0 k3.v0
    let o := lumaDcRow o q k0.v1 k1.v1 k2.v1 k3.v1
    let o := lumaDcRow o q k0.v2 k1.v2 k2.v2 k3.v2
    lumaDcRow o q k0.v3 k1.v3 k2.v3 k3.v3
  else c

/-- `(int16)(((x * q) as int32) >> 7)`, the chroma DC dequantization. -/
@[inline] private def dqC (x q : UInt32) : Int := toInt (s16 (sar (x * q) 7))

/-- 4:2:0 chroma DC: the 2x2 inverse transform of the four DC levels and
their dequantization (8.5.11), FFmpeg's `ff_h264_chroma_dc_dequant_idct_8_c`.
`c` holds the four (not dequantized) chroma DC levels in decoding order,
i.e. the raster 2x2 matrix `#[c00, c01, c10, c11]`. Result index `i` is the
DC coefficient of chroma 4x4 block `i` (raster `by * 2 + bx`). Each output
is `((f * qmul) >> 7)` wrapped to 16 bits.

`qmul` is FFmpeg's `dequant4_coeff[list][qPc][0]` for the component's
chroma `qPc` (list 1 Intra Cb, 2 Intra Cr, 4 Inter Cb, 5 Inter Cr):
`LevelScale4x4(qPc % 6, 0, 0) << (qPc / 6 + 2)`. -/
def chromaDcDequantIdct (c : Array Int) (qmul : Int) : Array Int :=
  let c := pad c 4
  if h : 4 ≤ c.size then
    let q := qmul.toInt32.toUInt32
    let a := cf c h 0 (by decide)
    let b := cf c h 1 (by decide)
    let c2 := cf c h 2 (by decide)
    let d := cf c h 3 (by decide)
    let e := a - b
    let a := a + b
    let b := c2 - d
    let c2 := c2 + d
    #[dqC (a + c2) q, dqC (e + b) q, dqC (a - c2) q, dqC (e - b) q]
  else c

end Yt.H264.Transform
