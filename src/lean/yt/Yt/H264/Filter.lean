import Yt.H264.Inter

/-! H.264 deblocking edge filters, 8-bit 4:2:0 (8.7.2.3, 8.7.2.4),
bit-exact with FFmpeg 9.0.1's h264_loop_filter_luma, _luma_intra, _chroma
and _chroma_intra (h264dsp_template.c at BIT_DEPTH 8).

Derived from FFmpeg, Copyright (c) 2003-2011 Michael Niedermayer
<michaelni@gmx.at> and the FFmpeg developers. Like FFmpeg, this file is free
software under the GNU Lesser General Public License, version 2.1 or (at
your option) any later version; it comes WITHOUT ANY WARRANTY (see the
LGPL).

Common arguments:
- `p` is the plane (8-bit samples), passed uniquely and filtered in place.
- `off` is the offset of the first q0 sample, the first sample past the
  edge (FFmpeg's `pix`). Samples p0, p1, … lie at `off - xstride`,
  `off - 2 xstride`, …, so `off` must be at least 3 `xstride` (4 for the
  luma intra filter, 2 for chroma).
- `xstride` steps across the edge, `ystride` along it: a vertical edge
  (FFmpeg's h_loop_filter_*) has xstride 1 and ystride the row stride; a
  horizontal edge (v_loop_filter_*) has xstride the row stride and
  ystride 1.
- `alpha`, `beta` are the already-looked-up α and β (Table 8-16 at
  indexA/indexB, i.e. FFmpeg's alpha_table/beta_table); `tc0` holds the four
  already-looked-up per-segment values (Table 8-17), one per 4 luma or 2
  chroma lines. The caller skips the edge when α or β is 0, as FFmpeg's
  filter_mb_edge* do.

The `Int` boundary is as in `Yt.H264.Inter`: `luma`/`chroma` convert `tc0`
and call `luma32`/`chroma32`, which take the four values as 32-bit two's
complement. -/
namespace Yt.H264.Filter
open Yt.H264.Inter (px sar twos)
open Yt.Bytes (u8 u8set)

/-- |a - b| for samples. -/
@[inline] def absd (a b : UInt32) : UInt32 := if a ≥ b then a - b else b - a

/-- Clamp a two's-complement `v` to [-t, t] (`t` ≥ 0). -/
@[inline] def clipTc (v t : UInt32) : UInt32 :=
  let s := v ^^^ 0x80000000
  if s < (0x80000000 - t) then (0 : UInt32) - t else if s > 0x80000000 + t then t else v

/-- `av_clip_uint8` of a two's-complement value, as a byte. -/
@[inline] def clipPixel (v : UInt32) : UInt8 :=
  if v >>> 31 != 0 then 0 else if v > 255 then 255 else v.toUInt8

/-- `tc0[i]` as 32-bit two's complement (-1, "skip", if absent). -/
@[inline] def tcAt (tc0 : Array Int) (i : Nat) : UInt32 :=
  if h : i < tc0.size then twos tc0[i] else 0xFFFFFFFF

/-! ## Luma, bS < 4 -/

/-- One line of the bS < 4 luma filter at q0 offset `k`; `tc` ≥ 0 is tC0. -/
@[inline] def lumaLine (p : ByteArray) (k xs : Nat) (alpha beta tc : UInt32) : ByteArray :=
  let p0 := px p (k - xs)
  let p1 := px p (k - 2 * xs)
  let p2 := px p (k - 3 * xs)
  let q0 := px p k
  let q1 := px p (k + xs)
  let q2 := px p (k + 2 * xs)
  if absd p0 q0 < alpha && absd p1 p0 < beta && absd q1 q0 < beta then
    let m := (p0 + q0 + 1) >>> 1
    let ap := absd p2 p0 < beta
    let aq := absd q2 q0 < beta
    -- p1', q1' (only when tC0 != 0; each side's test still widens tC).
    let p := if tc == 0 then p else
      let p := if ap then p.set! (k - 2 * xs) (p1 + clipTc (((p2 + m) >>> 1) - p1) tc).toUInt8 else p
      if aq then p.set! (k + xs) (q1 + clipTc (((q2 + m) >>> 1) - q1) tc).toUInt8 else p
    let t := tc + (if ap then 1 else 0) + (if aq then 1 else 0)
    let delta := clipTc (sar (((q0 - p0) <<< 2) + (p1 - q1) + 4) 3) t
    (p.set! (k - xs) (clipPixel (p0 + delta))).set! k (clipPixel (q0 - delta))
  else p

def lumaLines (p : ByteArray) (k xs ys : Nat) (alpha beta tc : UInt32) : Nat → ByteArray
  | 0 => p
  | n + 1 => lumaLines (lumaLine p k xs alpha beta tc) (k + ys) xs ys alpha beta tc n

/-- Four lines of segment `tc` (skipped when tC0 < 0). -/
@[inline] def lumaSeg (p : ByteArray) (k xs ys : Nat) (alpha beta tc : UInt32) : ByteArray :=
  if tc >>> 31 != 0 then p else lumaLines p k xs ys alpha beta tc 4

/-- `luma` with tc0[0..3] as 32-bit two's complement `t0 t1 t2 t3`. -/
def luma32 (p : ByteArray) (off xstride ystride alpha beta : Nat) (t0 t1 t2 t3 : UInt32) : ByteArray :=
  let a := alpha.toUInt32
  let b := beta.toUInt32
  let p := lumaSeg p off xstride ystride a b t0
  let p := lumaSeg p (off + 4 * ystride) xstride ystride a b t1
  let p := lumaSeg p (off + 8 * ystride) xstride ystride a b t2
  lumaSeg p (off + 12 * ystride) xstride ystride a b t3

/-- The bS < 4 luma filter along a 16-line edge (8.7.2.3), in 4 segments of
4 lines; segment i uses tC0 = `tc0[i]` (Table 8-17 at indexA and that
segment's bS) and is skipped when `tc0[i]` < 0 (FFmpeg's tc0_table gives
-1 for bS 0). FFmpeg's h264_loop_filter_luma(pix, xstride, ystride,
inner_iters 4, alpha, beta, tc0). -/
def luma (p : ByteArray) (off xstride ystride alpha beta : Nat) (tc0 : Array Int) : ByteArray :=
  luma32 p off xstride ystride alpha beta (tcAt tc0 0) (tcAt tc0 1) (tcAt tc0 2) (tcAt tc0 3)

/-! ## Luma, bS = 4 -/

-- The bS = 4 filters run in Lean on the TV too (FFmpeg's 32-bit ARM code
-- has no NEON for them): positions are USize, reads checked one by one.

@[inline] def lumaIntraLine (p : ByteArray) (k xs : USize) (alpha beta : UInt32) : ByteArray :=
  let p0 := (u8 p (k - xs)).toUInt32
  let p1 := (u8 p (k - 2 * xs)).toUInt32
  let q0 := (u8 p k).toUInt32
  let q1 := (u8 p (k + xs)).toUInt32
  if absd p0 q0 < alpha && absd p1 p0 < beta && absd q1 q0 < beta then
    let p2 := (u8 p (k - 3 * xs)).toUInt32
    let q2 := (u8 p (k + 2 * xs)).toUInt32
    if absd p0 q0 < (alpha >>> 2) + 2 then
      let p3 := (u8 p (k - 4 * xs)).toUInt32
      let q3 := (u8 p (k + 3 * xs)).toUInt32
      let p :=
        if absd p2 p0 < beta then
          let p := u8set p (k - xs) ((p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4) >>> 3).toUInt8
          let p := u8set p (k - 2 * xs) ((p2 + p1 + p0 + q0 + 2) >>> 2).toUInt8
          u8set p (k - 3 * xs) ((2 * p3 + 3 * p2 + p1 + p0 + q0 + 4) >>> 3).toUInt8
        else
          u8set p (k - xs) ((2 * p1 + p0 + q1 + 2) >>> 2).toUInt8
      if absd q2 q0 < beta then
        let p := u8set p k ((p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4) >>> 3).toUInt8
        let p := u8set p (k + xs) ((p0 + q0 + q1 + q2 + 2) >>> 2).toUInt8
        u8set p (k + 2 * xs) ((2 * q3 + 3 * q2 + q1 + q0 + p0 + 4) >>> 3).toUInt8
      else
        u8set p k ((2 * q1 + q0 + p1 + 2) >>> 2).toUInt8
    else
      u8set (u8set p (k - xs) ((2 * p1 + p0 + q1 + 2) >>> 2).toUInt8) k ((2 * q1 + q0 + p1 + 2) >>> 2).toUInt8
  else p

def lumaIntraLines (p : ByteArray) (k xs ys : USize) (alpha beta : UInt32) : Nat → ByteArray
  | 0 => p
  | n + 1 => lumaIntraLines (lumaIntraLine p k xs alpha beta) (k + ys) xs ys alpha beta n

/-- The bS = 4 luma filter along a 16-line edge at `p[off]`: FFmpeg's
h264_loop_filter_luma_intra(pix, xstride, ystride, inner_iters 4, alpha, beta). -/
def lumaIntraU (p : ByteArray) (off xstride ystride : USize) (alpha beta : UInt32) : ByteArray :=
  lumaIntraLines p off xstride ystride alpha beta 16

def lumaIntra (p : ByteArray) (off xstride ystride alpha beta : Nat) : ByteArray :=
  lumaIntraU p off.toUSize xstride.toUSize ystride.toUSize alpha.toUInt32 beta.toUInt32

/-! ## Chroma (4:2:0), bS < 4 and bS = 4 -/

@[inline] def chromaLine (p : ByteArray) (k xs : Nat) (alpha beta tc : UInt32) : ByteArray :=
  let p0 := px p (k - xs)
  let p1 := px p (k - 2 * xs)
  let q0 := px p k
  let q1 := px p (k + xs)
  if absd p0 q0 < alpha && absd p1 p0 < beta && absd q1 q0 < beta then
    let delta := clipTc (sar (((q0 - p0) <<< 2) + (p1 - q1) + 4) 3) tc
    (p.set! (k - xs) (clipPixel (p0 + delta))).set! k (clipPixel (q0 - delta))
  else p

/-- Two lines of segment `tc` (skipped when tC ≤ 0). -/
@[inline] def chromaSeg (p : ByteArray) (k xs ys : Nat) (alpha beta tc : UInt32) : ByteArray :=
  if tc >>> 31 != 0 || tc == 0 then p
  else chromaLine (chromaLine p k xs alpha beta tc) (k + ys) xs alpha beta tc

/-- `chroma` with tc0[0..3] as 32-bit two's complement `t0 t1 t2 t3`. -/
def chroma32 (p : ByteArray) (off xstride ystride alpha beta : Nat) (t0 t1 t2 t3 : UInt32) : ByteArray :=
  let a := alpha.toUInt32
  let b := beta.toUInt32
  let p := chromaSeg p off xstride ystride a b t0
  let p := chromaSeg p (off + 2 * ystride) xstride ystride a b t1
  let p := chromaSeg p (off + 4 * ystride) xstride ystride a b t2
  chromaSeg p (off + 6 * ystride) xstride ystride a b t3

/-- The bS < 4 chroma filter along an 8-line 4:2:0 chroma edge, 4 segments
of 2 lines: FFmpeg's h264_loop_filter_chroma(pix, xstride, ystride,
inner_iters 2, alpha, beta, tc0). Note FFmpeg's convention, kept here:
`tc0[i]` is already tC = tC0 + 1 (h264_loopfilter.c passes
tc0_table[indexA][bS] + 1), and a segment is skipped when it is ≤ 0. -/
def chroma (p : ByteArray) (off xstride ystride alpha beta : Nat) (tc0 : Array Int) : ByteArray :=
  chroma32 p off xstride ystride alpha beta (tcAt tc0 0) (tcAt tc0 1) (tcAt tc0 2) (tcAt tc0 3)

@[inline] def chromaIntraLine (p : ByteArray) (k xs : USize) (alpha beta : UInt32) : ByteArray :=
  let p0 := (u8 p (k - xs)).toUInt32
  let p1 := (u8 p (k - 2 * xs)).toUInt32
  let q0 := (u8 p k).toUInt32
  let q1 := (u8 p (k + xs)).toUInt32
  if absd p0 q0 < alpha && absd p1 p0 < beta && absd q1 q0 < beta then
    u8set (u8set p (k - xs) ((2 * p1 + p0 + q1 + 2) >>> 2).toUInt8) k ((2 * q1 + q0 + p1 + 2) >>> 2).toUInt8
  else p

def chromaIntraLines (p : ByteArray) (k xs ys : USize) (alpha beta : UInt32) : Nat → ByteArray
  | 0 => p
  | n + 1 => chromaIntraLines (chromaIntraLine p k xs alpha beta) (k + ys) xs ys alpha beta n

/-- The bS = 4 chroma filter along an 8-line 4:2:0 chroma edge: FFmpeg's
h264_loop_filter_chroma_intra(pix, xstride, ystride, inner_iters 2, alpha,
beta). -/
def chromaIntraU (p : ByteArray) (off xstride ystride : USize) (alpha beta : UInt32) : ByteArray :=
  chromaIntraLines p off xstride ystride alpha beta 8

def chromaIntra (p : ByteArray) (off xstride ystride alpha beta : Nat) : ByteArray :=
  chromaIntraU p off.toUSize xstride.toUSize ystride.toUSize alpha.toUInt32 beta.toUInt32

end Yt.H264.Filter
