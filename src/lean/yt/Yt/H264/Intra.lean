import Yt.H264.Kernels
/-! H.264 intra prediction for 8-bit samples.

A translation of FFmpeg 9.0.1's `libavcodec/h264pred_template.c` at
`BIT_DEPTH` 8 (copyright (c) 2003-2011 Michael Niedermayer), with the mode
numbering of `h264pred.h` and the function table that `ff_h264_pred_init`
installs for H.264 with 4:2:0 chroma. Output is identical to FFmpeg's C
functions byte for byte.

License: like FFmpeg, this module is free software; you can redistribute it
and/or modify it under the terms of the GNU Lesser General Public License as
published by the Free Software Foundation; either version 2.1 of the
License, or (at your option) any later version (LGPL-2.1-or-later). It is
distributed WITHOUT ANY WARRANTY; see the GNU LGPL for details.

Conventions: a plane is a `ByteArray` of samples with a row `stride` in
bytes; the block's top-left sample is at byte offset `off`. Neighbours are
read from the plane exactly where FFmpeg reads them (the row above the
block, the column to its left, the top-left corner and, where used, the
samples above and to the right). As in FFmpeg, the caller picks the mode
after the availability check (`ff_h264_check_intra4x4_pred_mode` /
`ff_h264_check_intra_pred_mode` turn modes with missing neighbours into the
LEFT_DC / TOP_DC / DC_128 / ALZHEIMER variants), so every sample read is a
decoded neighbour. Only the block is written. Reads outside the plane give
0 and writes outside it are dropped (FFmpeg would access memory there).
Pass the plane uniquely so that it is updated in place.

Mode numbers (FFmpeg `h264pred.h`):
* `pred4x4` and `pred8x8l`: 0 VERT, 1 HOR, 2 DC, 3 DIAG_DOWN_LEFT,
  4 DIAG_DOWN_RIGHT, 5 VERT_RIGHT, 6 HOR_DOWN, 7 VERT_LEFT, 8 HOR_UP,
  9 LEFT_DC, 10 TOP_DC, 11 DC_128.
* `pred16x16`: 0 DC, 1 HOR, 2 VERT, 3 PLANE, 4 LEFT_DC, 5 TOP_DC, 6 DC_128.
* `pred8x8` (4:2:0 chroma): 0 DC, 1 HOR, 2 VERT, 3 PLANE, 4 LEFT_DC,
  5 TOP_DC, 6 DC_128, and the H.264 partial-availability DC variants
  7 ALZHEIMER_DC_L0T (`mad_cow_dc_l0t`), 8 ALZHEIMER_DC_0LT,
  9 ALZHEIMER_DC_L00, 10 ALZHEIMER_DC_0L0.
Other mode numbers leave the plane unchanged. -/
namespace Yt.H264.Intra

private theorem usize_lt (a : ByteArray) (i : USize) (h : i < a.usize) : i.toNat < a.size := by
  have h1 : i.toNat < a.usize.toNat := h
  simp [ByteArray.usize] at h1
  exact Nat.lt_of_lt_of_le h1 (Nat.mod_le _ _)

/-- Sample `i` (0 outside the plane). -/
@[inline] private def ld (p : ByteArray) (i : USize) : UInt32 :=
  if h : i < p.usize then (p.uget i (usize_lt p i h)).toUInt32 else 0

/-- Set sample `i` to the low 8 bits of `v` (dropped outside the plane). -/
@[inline] private def st (p : ByteArray) (i : USize) (v : UInt32) : ByteArray :=
  if h : i < p.usize then p.uset i v.toUInt8 (usize_lt p i h) else p

/-- Index of sample `(x, y)` of the block at `o` (row stride `s`). -/
@[inline] private def ix (o s x y : USize) : USize := o + y * s + x

/-- Store `v` at the `n` indices `i, i + d, i + 2d, ...` (index arithmetic
wraps, so `d` may encode a step to the left or upwards). -/
private def run (p : ByteArray) (i d : USize) (v : UInt32) : Nat → ByteArray
  | 0 => p
  | n + 1 => run (st p i v) (i + d) d v n

/-- Fill `h` rows of `w` samples from `i` with `v` (Kernels.fill). -/
@[inline] private def fill (p : ByteArray) (i s : USize) (w : Nat) (v : UInt32) (h : Nat) : ByteArray :=
  Kernels.fillRaw p i s w.toUSize h.toUSize (v &&& 0xFF).toUSize 1

/-- Vertical: `n` columns of the block at `i` (height `h`) repeat the
sample above them (Kernels.predCopy). -/
@[inline] private def vert (p : ByteArray) (i s : USize) (h n : Nat) : ByteArray :=
  Kernels.predCopy p i s n.toUSize h.toUSize 0

/-- Horizontal: `n` rows of the block at `i` (width `w`) repeat the sample
to their left (Kernels.predCopy). -/
@[inline] private def horiz (p : ByteArray) (i s : USize) (w n : Nat) : ByteArray :=
  Kernels.predCopy p i s w.toUSize n.toUSize 1

/-- Sum of `n` samples from `i` in steps of `d`. -/
private def sum (p : ByteArray) (i d : USize) (acc : UInt32) : Nat → UInt32
  | 0 => acc
  | n + 1 => sum p (i + d) d (acc + ld p i) n

/-- The 3-tap filter `(a + 2b + c + 2) >> 2`. -/
@[inline] private def f3 (a b c : UInt32) : UInt32 := (a + 2 * b + c + 2) >>> 2

/-- The 2-tap average `(a + b + 1) >> 1`. -/
@[inline] private def f2 (a b : UInt32) : UInt32 := (a + b + 1) >>> 1

/-- Arithmetic shift right of a two's-complement 32-bit value. -/
@[inline] private def sar (x n : UInt32) : UInt32 :=
  let m : UInt32 := (0 : UInt32) - (x >>> 31)
  ((x ^^^ m) >>> n) ^^^ m

/-- FFmpeg's `av_clip_uint8` of a two's-complement 32-bit value. -/
@[inline] private def clip8 (x : UInt32) : UInt32 :=
  if x >>> 31 != 0 then 0 else if x > 255 then 255 else x

/-! ### 4x4 luma -/

private def pred4x4DownLeft (p : ByteArray) (o s : USize) (tr : Bool) : ByteArray :=
  let a := o - s
  let t0 := ld p a; let t1 := ld p (a + 1); let t2 := ld p (a + 2); let t3 := ld p (a + 3)
  let t4 := if tr then ld p (a + 4) else t3
  let t5 := if tr then ld p (a + 5) else t3
  let t6 := if tr then ld p (a + 6) else t3
  let t7 := if tr then ld p (a + 7) else t3
  let d := s - 1
  let p := run p (ix o s 0 0) d (f3 t0 t1 t2) 1
  let p := run p (ix o s 1 0) d (f3 t1 t2 t3) 2
  let p := run p (ix o s 2 0) d (f3 t2 t3 t4) 3
  let p := run p (ix o s 3 0) d (f3 t3 t4 t5) 4
  let p := run p (ix o s 3 1) d (f3 t4 t5 t6) 3
  let p := run p (ix o s 3 2) d (f3 t5 t6 t7) 2
  run p (ix o s 3 3) d ((t6 + 3 * t7 + 2) >>> 2) 1

private def pred4x4DownRight (p : ByteArray) (o s : USize) : ByteArray :=
  let a := o - s
  let lt := ld p (a - 1)
  let t0 := ld p a; let t1 := ld p (a + 1); let t2 := ld p (a + 2); let t3 := ld p (a + 3)
  let l0 := ld p (o - 1); let l1 := ld p (o + s - 1)
  let l2 := ld p (o + 2 * s - 1); let l3 := ld p (o + 3 * s - 1)
  let d := s + 1
  let p := run p (ix o s 0 3) d (f3 l3 l2 l1) 1
  let p := run p (ix o s 0 2) d (f3 l2 l1 l0) 2
  let p := run p (ix o s 0 1) d (f3 l1 l0 lt) 3
  let p := run p (ix o s 0 0) d (f3 l0 lt t0) 4
  let p := run p (ix o s 1 0) d (f3 lt t0 t1) 3
  let p := run p (ix o s 2 0) d (f3 t0 t1 t2) 2
  run p (ix o s 3 0) d (f3 t1 t2 t3) 1

private def pred4x4VertRight (p : ByteArray) (o s : USize) : ByteArray :=
  let a := o - s
  let lt := ld p (a - 1)
  let t0 := ld p a; let t1 := ld p (a + 1); let t2 := ld p (a + 2); let t3 := ld p (a + 3)
  let l0 := ld p (o - 1); let l1 := ld p (o + s - 1); let l2 := ld p (o + 2 * s - 1)
  let d := 2 * s + 1
  let p := run p (ix o s 0 0) d (f2 lt t0) 2
  let p := run p (ix o s 1 0) d (f2 t0 t1) 2
  let p := run p (ix o s 2 0) d (f2 t1 t2) 2
  let p := run p (ix o s 3 0) d (f2 t2 t3) 1
  let p := run p (ix o s 0 1) d (f3 l0 lt t0) 2
  let p := run p (ix o s 1 1) d (f3 lt t0 t1) 2
  let p := run p (ix o s 2 1) d (f3 t0 t1 t2) 2
  let p := run p (ix o s 3 1) d (f3 t1 t2 t3) 1
  let p := run p (ix o s 0 2) d (f3 lt l0 l1) 1
  run p (ix o s 0 3) d (f3 l0 l1 l2) 1

private def pred4x4HorDown (p : ByteArray) (o s : USize) : ByteArray :=
  let a := o - s
  let lt := ld p (a - 1)
  let t0 := ld p a; let t1 := ld p (a + 1); let t2 := ld p (a + 2)
  let l0 := ld p (o - 1); let l1 := ld p (o + s - 1)
  let l2 := ld p (o + 2 * s - 1); let l3 := ld p (o + 3 * s - 1)
  let d := s + 2
  let p := run p (ix o s 0 0) d (f2 lt l0) 2
  let p := run p (ix o s 1 0) d (f3 l0 lt t0) 2
  let p := run p (ix o s 2 0) d (f3 lt t0 t1) 1
  let p := run p (ix o s 3 0) d (f3 t0 t1 t2) 1
  let p := run p (ix o s 0 1) d (f2 l0 l1) 2
  let p := run p (ix o s 1 1) d (f3 lt l0 l1) 2
  let p := run p (ix o s 0 2) d (f2 l1 l2) 2
  let p := run p (ix o s 1 2) d (f3 l0 l1 l2) 2
  let p := run p (ix o s 0 3) d (f2 l2 l3) 1
  run p (ix o s 1 3) d (f3 l1 l2 l3) 1

private def pred4x4VertLeft (p : ByteArray) (o s : USize) (tr : Bool) : ByteArray :=
  let a := o - s
  let t0 := ld p a; let t1 := ld p (a + 1); let t2 := ld p (a + 2); let t3 := ld p (a + 3)
  let t4 := if tr then ld p (a + 4) else t3
  let t5 := if tr then ld p (a + 5) else t3
  let t6 := if tr then ld p (a + 6) else t3
  let d := 2 * s - 1
  let p := run p (ix o s 0 0) d (f2 t0 t1) 1
  let p := run p (ix o s 1 0) d (f2 t1 t2) 2
  let p := run p (ix o s 2 0) d (f2 t2 t3) 2
  let p := run p (ix o s 3 0) d (f2 t3 t4) 2
  let p := run p (ix o s 3 2) d (f2 t4 t5) 1
  let p := run p (ix o s 0 1) d (f3 t0 t1 t2) 1
  let p := run p (ix o s 1 1) d (f3 t1 t2 t3) 2
  let p := run p (ix o s 2 1) d (f3 t2 t3 t4) 2
  let p := run p (ix o s 3 1) d (f3 t3 t4 t5) 2
  run p (ix o s 3 3) d (f3 t4 t5 t6) 1

private def pred4x4HorUp (p : ByteArray) (o s : USize) : ByteArray :=
  let l0 := ld p (o - 1); let l1 := ld p (o + s - 1)
  let l2 := ld p (o + 2 * s - 1); let l3 := ld p (o + 3 * s - 1)
  let d := s - 2
  let p := run p (ix o s 0 0) d (f2 l0 l1) 1
  let p := run p (ix o s 1 0) d (f3 l0 l1 l2) 1
  let p := run p (ix o s 2 0) d (f2 l1 l2) 2
  let p := run p (ix o s 3 0) d (f3 l1 l2 l3) 2
  let p := run p (ix o s 2 1) d (f2 l2 l3) 2
  let p := run p (ix o s 3 1) d ((l2 + 3 * l3 + 2) >>> 2) 2
  let p := run p (ix o s 2 2) 1 l3 2
  run p (ix o s 0 3) 1 l3 4

/-- 4x4 luma prediction of the block at `off` (FFmpeg `h->hpc.pred4x4[mode]`).
`topRightAvail` matters only for modes 3 and 7: when false, the four
samples above-right are taken to equal the last sample of the row above
(as `hl_decode_mb_predict_luma` arranges); when true they are read from
the plane. -/
def pred4x4 (p : ByteArray) (off stride mode : Nat) (topRightAvail : Bool) : ByteArray :=
  let o := off.toUSize
  let s := stride.toUSize
  match mode with
  | 0 => vert p o s 4 4
  | 1 => horiz p o s 4 4
  | 2 => fill p o s 4 ((sum p (o - s) 1 (sum p (o - 1) s 4 4) 4) >>> 3) 4
  | 3 => pred4x4DownLeft p o s topRightAvail
  | 4 => pred4x4DownRight p o s
  | 5 => pred4x4VertRight p o s
  | 6 => pred4x4HorDown p o s
  | 7 => pred4x4VertLeft p o s topRightAvail
  | 8 => pred4x4HorUp p o s
  | 9 => fill p o s 4 ((sum p (o - 1) s 2 4) >>> 2) 4
  | 10 => fill p o s 4 ((sum p (o - s) 1 2 4) >>> 2) 4
  | 11 => fill p o s 4 128 4
  | _ => p

/-! ### 16x16 luma -/

/-- Rows of a plane prediction: row `y` holds `clip((b + x*h) >> 5)` with
`b = a + y*v`. -/
private def planeRow (p : ByteArray) (i : USize) (b h : UInt32) : Nat → ByteArray
  | 0 => p
  | n + 1 => planeRow (st p i (clip8 (sar b 5))) (i + 1) (b + h) h n

private def planeRows (p : ByteArray) (i s : USize) (a h v : UInt32) (w : Nat) : Nat → ByteArray
  | 0 => p
  | n + 1 => planeRows (planeRow p i a h w) (i + s) s (a + v) h v w n

/-- `Σ_{k=1..n} k * (p[i + k*d] - p[i - k*d])` (two's complement). -/
private def grad (p : ByteArray) (i d : USize) (k : UInt32) (acc : UInt32) : Nat → UInt32
  | 0 => acc
  | n + 1 =>
    let kd := k.toUSize * d
    grad p i d (k + 1) (acc + k * (ld p (i + kd) - ld p (i - kd))) n

private def pred16x16Plane (p : ByteArray) (o s : USize) : ByteArray :=
  -- H around the 8th sample above, V around the 8th sample to the left; the
  -- outermost terms use the top-left corner.
  let h0 := grad p (o + 7 - s) 1 1 0 8
  let v0 := grad p (o + 7 * s - 1) s 1 0 8
  let h := sar (5 * h0 + 32) 6
  let v := sar (5 * v0 + 32) 6
  let a := 16 * (ld p (o + 15 * s - 1) + ld p (o + 15 - s) + 1) - 7 * (v + h)
  planeRows p o s a h v 16 16

/-- 16x16 luma prediction (FFmpeg `h->hpc.pred16x16[mode]`): 0 DC, 1 HOR,
2 VERT, 3 PLANE, 4 LEFT_DC, 5 TOP_DC, 6 DC_128. -/
def pred16x16 (p : ByteArray) (off stride mode : Nat) : ByteArray :=
  let o := off.toUSize
  let s := stride.toUSize
  match mode with
  | 0 => fill p o s 16 ((sum p (o - s) 1 (sum p (o - 1) s 16 16) 16) >>> 5) 16
  | 1 => horiz p o s 16 16
  | 2 => vert p o s 16 16
  | 3 => pred16x16Plane p o s
  | 4 => fill p o s 16 ((sum p (o - 1) s 8 16) >>> 4) 16
  | 5 => fill p o s 16 ((sum p (o - s) 1 8 16) >>> 4) 16
  | 6 => fill p o s 16 128 16
  | _ => p

/-! ### 8x8 chroma (4:2:0) -/

private def pred8x8Plane (p : ByteArray) (o s : USize) : ByteArray :=
  let h0 := grad p (o + 3 - s) 1 1 0 4
  let v0 := grad p (o + 3 * s - 1) s 1 0 4
  let h := sar (17 * h0 + 16) 5
  let v := sar (17 * v0 + 16) 5
  let a := 16 * (ld p (o + 7 * s - 1) + ld p (o + 7 - s) + 1) - 3 * (v + h)
  planeRows p o s a h v 8 8

/-- FFmpeg `pred8x8_left_dc`: rows 0-3 and 4-7 from their left samples. -/
private def pred8x8LeftDc (p : ByteArray) (o s : USize) : ByteArray :=
  let p := fill p o s 8 ((sum p (o - 1) s 2 4) >>> 2) 4
  fill p (o + 4 * s) s 8 ((sum p (o + 4 * s - 1) s 2 4) >>> 2) 4

/-- FFmpeg `pred8x8_top_dc`: columns 0-3 and 4-7 from the samples above. -/
private def pred8x8TopDc (p : ByteArray) (o s : USize) : ByteArray :=
  let p := fill p o s 4 ((sum p (o - s) 1 2 4) >>> 2) 8
  fill p (o + 4) s 4 ((sum p (o + 4 - s) 1 2 4) >>> 2) 8

/-- FFmpeg `pred8x8_dc`: a DC per 4x4 quadrant. -/
private def pred8x8Dc (p : ByteArray) (o s : USize) : ByteArray :=
  let dc0 := sum p (o - s) 1 (sum p (o - 1) s 0 4) 4
  let dc1 := sum p (o + 4 - s) 1 0 4
  let dc2 := sum p (o + 4 * s - 1) s 0 4
  let p := fill p o s 4 ((dc0 + 4) >>> 3) 4
  let p := fill p (o + 4) s 4 ((dc1 + 2) >>> 2) 4
  let p := fill p (o + 4 * s) s 4 ((dc2 + 2) >>> 2) 4
  fill p (o + 4 * s + 4) s 4 ((dc1 + dc2 + 4) >>> 3) 4

/-- 8x8 chroma prediction for 4:2:0 (FFmpeg `h->hpc.pred8x8[mode]`, H.264):
0 DC, 1 HOR, 2 VERT, 3 PLANE, 4 LEFT_DC, 5 TOP_DC, 6 DC_128,
7 ALZHEIMER_DC_L0T (TOP_DC, then the top-left 4x4 by 4x4 DC),
8 ALZHEIMER_DC_0LT (DC, then the top-left 4x4 by 4x4 TOP_DC),
9 ALZHEIMER_DC_L00 (LEFT_DC, then rows 4-7 set to 128),
10 ALZHEIMER_DC_0L0 (LEFT_DC, then rows 0-3 set to 128). -/
def pred8x8 (p : ByteArray) (off stride mode : Nat) : ByteArray :=
  let o := off.toUSize
  let s := stride.toUSize
  match mode with
  | 0 => pred8x8Dc p o s
  | 1 => horiz p o s 8 8
  | 2 => vert p o s 8 8
  | 3 => pred8x8Plane p o s
  | 4 => pred8x8LeftDc p o s
  | 5 => pred8x8TopDc p o s
  | 6 => fill p o s 8 128 8
  | 7 =>
    let p := pred8x8TopDc p o s
    fill p o s 4 ((sum p (o - s) 1 (sum p (o - 1) s 4 4) 4) >>> 3) 4
  | 8 =>
    let p := pred8x8Dc p o s
    fill p o s 4 ((sum p (o - s) 1 2 4) >>> 2) 4
  | 9 => fill (pred8x8LeftDc p o s) (o + 4 * s) s 8 128 4
  | 10 => fill (pred8x8LeftDc p o s) o s 8 128 4
  | _ => p

/-! ### 8x8 luma (High profile `transform_size_8x8_flag`)

The reference samples are low-pass filtered first (FFmpeg's
`PREDICT_8x8_LOAD_*`). Filtered edges are packed eight samples to a
`UInt64` (byte `k` = sample `k`), which keeps them out of the heap. -/

@[inline] private def pk (a0 a1 a2 a3 a4 a5 a6 a7 : UInt32) : UInt64 :=
  a0.toUInt64 ||| (a1.toUInt64 <<< 8) ||| (a2.toUInt64 <<< 16) ||| (a3.toUInt64 <<< 24) |||
  (a4.toUInt64 <<< 32) ||| (a5.toUInt64 <<< 40) ||| (a6.toUInt64 <<< 48) ||| (a7.toUInt64 <<< 56)

/-- Sample `k` of a packed edge, `sh` = 8k. -/
@[inline] private def b8 (w : UInt64) (sh : UInt64) : UInt32 := ((w >>> sh) &&& 0xFF).toUInt32

/-- `PREDICT_8x8_LOAD_TOP`: t0..t7. -/
private def loadTop (p : ByteArray) (o s : USize) (tl tr : Bool) : UInt64 :=
  let a := o - s
  let r0 := ld p a; let r1 := ld p (a + 1); let r2 := ld p (a + 2); let r3 := ld p (a + 3)
  let r4 := ld p (a + 4); let r5 := ld p (a + 5); let r6 := ld p (a + 6); let r7 := ld p (a + 7)
  let t0 := ((if tl then ld p (a - 1) else r0) + 2 * r0 + r1 + 2) >>> 2
  let t7 := ((if tr then ld p (a + 8) else r7) + 2 * r7 + r6 + 2) >>> 2
  pk t0 (f3 r0 r1 r2) (f3 r1 r2 r3) (f3 r2 r3 r4) (f3 r3 r4 r5) (f3 r4 r5 r6) (f3 r5 r6 r7) t7

/-- `PREDICT_8x8_LOAD_TOPRIGHT`: t8..t15 (all the unfiltered sample above
column 7 when the top right is unavailable). -/
private def loadTopRight (p : ByteArray) (o s : USize) (tr : Bool) : UInt64 :=
  let a := o - s
  if tr then
    let r7 := ld p (a + 7); let r8 := ld p (a + 8); let r9 := ld p (a + 9)
    let r10 := ld p (a + 10); let r11 := ld p (a + 11); let r12 := ld p (a + 12)
    let r13 := ld p (a + 13); let r14 := ld p (a + 14); let r15 := ld p (a + 15)
    pk (f3 r7 r8 r9) (f3 r8 r9 r10) (f3 r9 r10 r11) (f3 r10 r11 r12) (f3 r11 r12 r13)
      (f3 r12 r13 r14) (f3 r13 r14 r15) ((r14 + 3 * r15 + 2) >>> 2)
  else
    let r7 := ld p (a + 7)
    pk r7 r7 r7 r7 r7 r7 r7 r7

/-- `PREDICT_8x8_LOAD_LEFT`: l0..l7. -/
private def loadLeft (p : ByteArray) (o s : USize) (tl : Bool) : UInt64 :=
  let a := o - 1
  let r0 := ld p a; let r1 := ld p (a + s); let r2 := ld p (a + 2 * s); let r3 := ld p (a + 3 * s)
  let r4 := ld p (a + 4 * s); let r5 := ld p (a + 5 * s); let r6 := ld p (a + 6 * s)
  let r7 := ld p (a + 7 * s)
  let l0 := ((if tl then ld p (a - s) else r0) + 2 * r0 + r1 + 2) >>> 2
  pk l0 (f3 r0 r1 r2) (f3 r1 r2 r3) (f3 r2 r3 r4) (f3 r3 r4 r5) (f3 r4 r5 r6) (f3 r5 r6 r7)
    ((r6 + 3 * r7 + 2) >>> 2)

/-- `PREDICT_8x8_LOAD_TOPLEFT`. -/
@[inline] private def loadTopLeft (p : ByteArray) (o s : USize) : UInt32 :=
  f3 (ld p (o - 1)) (ld p (o - s - 1)) (ld p (o - s))

/-- Sum of the eight samples of a packed edge. -/
@[inline] private def sum8 (w : UInt64) : UInt32 :=
  b8 w 0 + b8 w 8 + b8 w 16 + b8 w 24 + b8 w 32 + b8 w 40 + b8 w 48 + b8 w 56

/-- Vertical: column `x` of the block repeats sample `x` of `t`. -/
private def vert8 (p : ByteArray) (i s : USize) (t : UInt64) (sh : UInt64) : Nat → ByteArray
  | 0 => p
  | n + 1 => vert8 (run p i s (b8 t sh) 8) (i + 1) s t (sh + 8) n

/-- Horizontal: row `y` of the block repeats sample `y` of `l`. -/
private def horiz8 (p : ByteArray) (i s : USize) (l : UInt64) (sh : UInt64) : Nat → ByteArray
  | 0 => p
  | n + 1 => horiz8 (run p i 1 (b8 l sh) 8) (i + s) s l (sh + 8) n

private def pred8x8lDownLeft (p : ByteArray) (o s : USize) (tl tr : Bool) : ByteArray :=
  let t := loadTop p o s tl tr
  let u := loadTopRight p o s tr
  let t0 := b8 t 0; let t1 := b8 t 8; let t2 := b8 t 16; let t3 := b8 t 24
  let t4 := b8 t 32; let t5 := b8 t 40; let t6 := b8 t 48; let t7 := b8 t 56
  let t8 := b8 u 0; let t9 := b8 u 8; let t10 := b8 u 16; let t11 := b8 u 24
  let t12 := b8 u 32; let t13 := b8 u 40; let t14 := b8 u 48; let t15 := b8 u 56
  let d := 1 - s
  let p := run p (ix o s 0 0) d (f3 t0 t1 t2) 1
  let p := run p (ix o s 0 1) d (f3 t1 t2 t3) 2
  let p := run p (ix o s 0 2) d (f3 t2 t3 t4) 3
  let p := run p (ix o s 0 3) d (f3 t3 t4 t5) 4
  let p := run p (ix o s 0 4) d (f3 t4 t5 t6) 5
  let p := run p (ix o s 0 5) d (f3 t5 t6 t7) 6
  let p := run p (ix o s 0 6) d (f3 t6 t7 t8) 7
  let p := run p (ix o s 0 7) d (f3 t7 t8 t9) 8
  let p := run p (ix o s 1 7) d (f3 t8 t9 t10) 7
  let p := run p (ix o s 2 7) d (f3 t9 t10 t11) 6
  let p := run p (ix o s 3 7) d (f3 t10 t11 t12) 5
  let p := run p (ix o s 4 7) d (f3 t11 t12 t13) 4
  let p := run p (ix o s 5 7) d (f3 t12 t13 t14) 3
  let p := run p (ix o s 6 7) d (f3 t13 t14 t15) 2
  run p (ix o s 7 7) d ((t14 + 3 * t15 + 2) >>> 2) 1

private def pred8x8lDownRight (p : ByteArray) (o s : USize) (tl tr : Bool) : ByteArray :=
  let t := loadTop p o s tl tr
  let l := loadLeft p o s tl
  let lt := loadTopLeft p o s
  let t0 := b8 t 0; let t1 := b8 t 8; let t2 := b8 t 16; let t3 := b8 t 24
  let t4 := b8 t 32; let t5 := b8 t 40; let t6 := b8 t 48; let t7 := b8 t 56
  let l0 := b8 l 0; let l1 := b8 l 8; let l2 := b8 l 16; let l3 := b8 l 24
  let l4 := b8 l 32; let l5 := b8 l 40; let l6 := b8 l 48; let l7 := b8 l 56
  let d := s + 1
  let p := run p (ix o s 0 7) d (f3 l7 l6 l5) 1
  let p := run p (ix o s 0 6) d (f3 l6 l5 l4) 2
  let p := run p (ix o s 0 5) d (f3 l5 l4 l3) 3
  let p := run p (ix o s 0 4) d (f3 l4 l3 l2) 4
  let p := run p (ix o s 0 3) d (f3 l3 l2 l1) 5
  let p := run p (ix o s 0 2) d (f3 l2 l1 l0) 6
  let p := run p (ix o s 0 1) d (f3 l1 l0 lt) 7
  let p := run p (ix o s 0 0) d (f3 l0 lt t0) 8
  let p := run p (ix o s 1 0) d (f3 lt t0 t1) 7
  let p := run p (ix o s 2 0) d (f3 t0 t1 t2) 6
  let p := run p (ix o s 3 0) d (f3 t1 t2 t3) 5
  let p := run p (ix o s 4 0) d (f3 t2 t3 t4) 4
  let p := run p (ix o s 5 0) d (f3 t3 t4 t5) 3
  let p := run p (ix o s 6 0) d (f3 t4 t5 t6) 2
  run p (ix o s 7 0) d (f3 t5 t6 t7) 1

private def pred8x8lVertRight (p : ByteArray) (o s : USize) (tl tr : Bool) : ByteArray :=
  let t := loadTop p o s tl tr
  let l := loadLeft p o s tl
  let lt := loadTopLeft p o s
  let t0 := b8 t 0; let t1 := b8 t 8; let t2 := b8 t 16; let t3 := b8 t 24
  let t4 := b8 t 32; let t5 := b8 t 40; let t6 := b8 t 48; let t7 := b8 t 56
  let l0 := b8 l 0; let l1 := b8 l 8; let l2 := b8 l 16; let l3 := b8 l 24
  let l4 := b8 l 32; let l5 := b8 l 40; let l6 := b8 l 48
  let d := 2 * s + 1
  let p := run p (ix o s 0 6) d (f3 l5 l4 l3) 1
  let p := run p (ix o s 0 7) d (f3 l6 l5 l4) 1
  let p := run p (ix o s 0 4) d (f3 l3 l2 l1) 2
  let p := run p (ix o s 0 5) d (f3 l4 l3 l2) 2
  let p := run p (ix o s 0 2) d (f3 l1 l0 lt) 3
  let p := run p (ix o s 0 3) d (f3 l2 l1 l0) 3
  let p := run p (ix o s 0 1) d (f3 l0 lt t0) 4
  let p := run p (ix o s 0 0) d (f2 lt t0) 4
  let p := run p (ix o s 1 1) d (f3 lt t0 t1) 4
  let p := run p (ix o s 1 0) d (f2 t0 t1) 4
  let p := run p (ix o s 2 1) d (f3 t0 t1 t2) 4
  let p := run p (ix o s 2 0) d (f2 t1 t2) 4
  let p := run p (ix o s 3 1) d (f3 t1 t2 t3) 4
  let p := run p (ix o s 3 0) d (f2 t2 t3) 4
  let p := run p (ix o s 4 1) d (f3 t2 t3 t4) 4
  let p := run p (ix o s 4 0) d (f2 t3 t4) 4
  let p := run p (ix o s 5 1) d (f3 t3 t4 t5) 3
  let p := run p (ix o s 5 0) d (f2 t4 t5) 3
  let p := run p (ix o s 6 1) d (f3 t4 t5 t6) 2
  let p := run p (ix o s 6 0) d (f2 t5 t6) 2
  let p := run p (ix o s 7 1) d (f3 t5 t6 t7) 1
  run p (ix o s 7 0) d (f2 t6 t7) 1

private def pred8x8lHorDown (p : ByteArray) (o s : USize) (tl tr : Bool) : ByteArray :=
  let t := loadTop p o s tl tr
  let l := loadLeft p o s tl
  let lt := loadTopLeft p o s
  let t0 := b8 t 0; let t1 := b8 t 8; let t2 := b8 t 16; let t3 := b8 t 24
  let t4 := b8 t 32; let t5 := b8 t 40; let t6 := b8 t 48
  let l0 := b8 l 0; let l1 := b8 l 8; let l2 := b8 l 16; let l3 := b8 l 24
  let l4 := b8 l 32; let l5 := b8 l 40; let l6 := b8 l 48; let l7 := b8 l 56
  let d := s + 2
  let p := run p (ix o s 0 7) d (f2 l6 l7) 1
  let p := run p (ix o s 1 7) d (f3 l5 l6 l7) 1
  let p := run p (ix o s 0 6) d (f2 l5 l6) 2
  let p := run p (ix o s 1 6) d (f3 l4 l5 l6) 2
  let p := run p (ix o s 0 5) d (f2 l4 l5) 3
  let p := run p (ix o s 1 5) d (f3 l3 l4 l5) 3
  let p := run p (ix o s 0 4) d (f2 l3 l4) 4
  let p := run p (ix o s 1 4) d (f3 l2 l3 l4) 4
  let p := run p (ix o s 0 3) d (f2 l2 l3) 4
  let p := run p (ix o s 1 3) d (f3 l1 l2 l3) 4
  let p := run p (ix o s 0 2) d (f2 l1 l2) 4
  let p := run p (ix o s 1 2) d (f3 l0 l1 l2) 4
  let p := run p (ix o s 0 1) d (f2 l0 l1) 4
  let p := run p (ix o s 1 1) d (f3 lt l0 l1) 4
  let p := run p (ix o s 0 0) d (f2 lt l0) 4
  let p := run p (ix o s 1 0) d (f3 l0 lt t0) 4
  let p := run p (ix o s 2 0) d (f3 t1 t0 lt) 3
  let p := run p (ix o s 3 0) d (f3 t2 t1 t0) 3
  let p := run p (ix o s 4 0) d (f3 t3 t2 t1) 2
  let p := run p (ix o s 5 0) d (f3 t4 t3 t2) 2
  let p := run p (ix o s 6 0) d (f3 t5 t4 t3) 1
  run p (ix o s 7 0) d (f3 t6 t5 t4) 1

private def pred8x8lVertLeft (p : ByteArray) (o s : USize) (tl tr : Bool) : ByteArray :=
  let t := loadTop p o s tl tr
  let u := loadTopRight p o s tr
  let t0 := b8 t 0; let t1 := b8 t 8; let t2 := b8 t 16; let t3 := b8 t 24
  let t4 := b8 t 32; let t5 := b8 t 40; let t6 := b8 t 48; let t7 := b8 t 56
  let t8 := b8 u 0; let t9 := b8 u 8; let t10 := b8 u 16; let t11 := b8 u 24
  let t12 := b8 u 32
  let d := 1 - 2 * s
  let p := run p (ix o s 0 0) d (f2 t0 t1) 1
  let p := run p (ix o s 0 1) d (f3 t0 t1 t2) 1
  let p := run p (ix o s 0 2) d (f2 t1 t2) 2
  let p := run p (ix o s 0 3) d (f3 t1 t2 t3) 2
  let p := run p (ix o s 0 4) d (f2 t2 t3) 3
  let p := run p (ix o s 0 5) d (f3 t2 t3 t4) 3
  let p := run p (ix o s 0 6) d (f2 t3 t4) 4
  let p := run p (ix o s 0 7) d (f3 t3 t4 t5) 4
  let p := run p (ix o s 1 6) d (f2 t4 t5) 4
  let p := run p (ix o s 1 7) d (f3 t4 t5 t6) 4
  let p := run p (ix o s 2 6) d (f2 t5 t6) 4
  let p := run p (ix o s 2 7) d (f3 t5 t6 t7) 4
  let p := run p (ix o s 3 6) d (f2 t6 t7) 4
  let p := run p (ix o s 3 7) d (f3 t6 t7 t8) 4
  let p := run p (ix o s 4 6) d (f2 t7 t8) 4
  let p := run p (ix o s 4 7) d (f3 t7 t8 t9) 4
  let p := run p (ix o s 5 6) d (f2 t8 t9) 3
  let p := run p (ix o s 5 7) d (f3 t8 t9 t10) 3
  let p := run p (ix o s 6 6) d (f2 t9 t10) 2
  let p := run p (ix o s 6 7) d (f3 t9 t10 t11) 2
  let p := run p (ix o s 7 6) d (f2 t10 t11) 1
  run p (ix o s 7 7) d (f3 t10 t11 t12) 1

private def pred8x8lHorUp (p : ByteArray) (o s : USize) (tl : Bool) : ByteArray :=
  let l := loadLeft p o s tl
  let l0 := b8 l 0; let l1 := b8 l 8; let l2 := b8 l 16; let l3 := b8 l 24
  let l4 := b8 l 32; let l5 := b8 l 40; let l6 := b8 l 48; let l7 := b8 l 56
  let d := 2 - s
  let p := run p (ix o s 0 0) d (f2 l0 l1) 1
  let p := run p (ix o s 1 0) d (f3 l0 l1 l2) 1
  let p := run p (ix o s 0 1) d (f2 l1 l2) 2
  let p := run p (ix o s 1 1) d (f3 l1 l2 l3) 2
  let p := run p (ix o s 0 2) d (f2 l2 l3) 3
  let p := run p (ix o s 1 2) d (f3 l2 l3 l4) 3
  let p := run p (ix o s 0 3) d (f2 l3 l4) 4
  let p := run p (ix o s 1 3) d (f3 l3 l4 l5) 4
  let p := run p (ix o s 0 4) d (f2 l4 l5) 4
  let p := run p (ix o s 1 4) d (f3 l4 l5 l6) 4
  let p := run p (ix o s 0 5) d (f2 l5 l6) 4
  let p := run p (ix o s 1 5) d (f3 l5 l6 l7) 4
  let p := run p (ix o s 0 6) d (f2 l6 l7) 4
  let p := run p (ix o s 1 6) d ((l6 + 3 * l7 + 2) >>> 2) 4
  let p := run p (ix o s 6 4) 1 l7 2
  let p := run p (ix o s 4 5) 1 l7 4
  let p := run p (ix o s 2 6) 1 l7 6
  run p (ix o s 0 7) 1 l7 8

/-- 8x8 luma prediction with reference filtering (FFmpeg
`h->hpc.pred8x8l[mode]`), modes numbered as for `pred4x4`. `hasTopLeft`
and `hasTopRight` are FFmpeg's `has_topleft` / `has_topright` (the
`topleft_samples_available` / `topright_samples_available` bits for the
block); when `hasTopRight` is false the samples above-right are not read. -/
def pred8x8l (p : ByteArray) (off stride mode : Nat) (hasTopLeft hasTopRight : Bool) : ByteArray :=
  let o := off.toUSize
  let s := stride.toUSize
  match mode with
  | 0 => vert8 p o s (loadTop p o s hasTopLeft hasTopRight) 0 8
  | 1 => horiz8 p o s (loadLeft p o s hasTopLeft) 0 8
  | 2 =>
    let l := sum8 (loadLeft p o s hasTopLeft)
    let t := sum8 (loadTop p o s hasTopLeft hasTopRight)
    fill p o s 8 ((l + t + 8) >>> 4) 8
  | 3 => pred8x8lDownLeft p o s hasTopLeft hasTopRight
  | 4 => pred8x8lDownRight p o s hasTopLeft hasTopRight
  | 5 => pred8x8lVertRight p o s hasTopLeft hasTopRight
  | 6 => pred8x8lHorDown p o s hasTopLeft hasTopRight
  | 7 => pred8x8lVertLeft p o s hasTopLeft hasTopRight
  | 8 => pred8x8lHorUp p o s hasTopLeft
  | 9 => fill p o s 8 ((sum8 (loadLeft p o s hasTopLeft) + 4) >>> 3) 8
  | 10 => fill p o s 8 ((sum8 (loadTop p o s hasTopLeft hasTopRight) + 4) >>> 3) 8
  | 11 => fill p o s 8 128 8
  | _ => p

end Yt.H264.Intra
