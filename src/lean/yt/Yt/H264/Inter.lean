import Yt.H264.Kernels
import Yt.Bytes

/-! H.264 inter prediction, 8-bit 4:2:0: quarter-sample luma and
eighth-sample chroma motion compensation (8.4.2.2) and explicit/implicit
weighted sample prediction (8.4.2.3), bit-exact with FFmpeg 9.0.1's C
implementation (h264qpel_template.c, h264chroma_template.c, the weighting of
h264dsp_template.c, and the reference-edge replication of
videodsp_template.c's emulated_edge_mc as h264_mb.c's mc_dir_part uses it).

Derived from FFmpeg, Copyright (c) 2003-2011 Michael Niedermayer
<michaelni@gmx.at> and the FFmpeg developers. Like FFmpeg, this file is free
software under the GNU Lesser General Public License, version 2.1 or (at
your option) any later version; it comes WITHOUT ANY WARRANTY (see the
LGPL).

Planes are `ByteArray`s of 8-bit samples addressed by byte offset and row
stride. Destinations are passed uniquely and updated in place.

The specified entry points take `Int` (positions, weights). Our runtime has
no `Int` support yet, so each converts its `Int` arguments once with `twos`
and calls the `…32` variant, which takes the same values as 32-bit two's
complement in `UInt32`; everything below that boundary is `UInt32`/`Nat`.
-/
namespace Yt.H264.Inter

/-- `v` as a 32-bit two's-complement value (`v` must lie in [-2^31, 2^31)). -/
@[inline] def twos (v : Int) : UInt32 :=
  match v with
  | .ofNat n => n.toUInt32
  | .negSucc n => ~~~ n.toUInt32

/-- Sample `i` of `b` (0 past the end, which valid calls never read). -/
@[inline] def px (b : ByteArray) (i : Nat) : UInt32 :=
  if h : i < b.size then b[i].toUInt32 else 0

/-- Arithmetic shift right of a two's-complement value. -/
@[inline] def sar (x n : UInt32) : UInt32 :=
  let s : UInt32 := (0 : UInt32) - (x >>> 31)
  ((x ^^^ s) >>> n) ^^^ s

/-- `av_clip_uint8` of a two's-complement value. -/
@[inline] def clip8 (v : UInt32) : UInt32 :=
  if v >>> 31 != 0 then 0 else if v > 255 then 255 else v

/-- `rnd_avg` of two samples: (a + b + 1) >> 1. -/
@[inline] def avg2 (a b : UInt32) : UInt32 := (a + b + 1) >>> 1

/-- Store prediction `v` at `d`: as is (`put`), or averaged with the sample
already there (`avg`, FFmpeg's avg_ variants). -/
@[inline] def store (dst : ByteArray) (d : Nat) (v : UInt32) (avg : Bool) : ByteArray :=
  if avg then dst.set! d (avg2 (px dst d) v).toUInt8 else dst.set! d v.toUInt8

/-! ## Reference-edge replication

Every reference read clamps its coordinates to the picture, which is what
FFmpeg's emulated_edge_mc produces for the blocks h264_mb.c gives it. A
position is first reduced to `bias`: `clamp(x, -62, n + 64) + 64`. Windows
are at most 16 + 5 samples wide (luma, from x - 2) or 8 + 1 (chroma), so a
block 62 or more samples left of (or 64 or more right of) the picture reads
only the edge column, as it does from the clamped position. -/

/-- `clamp(x, -62, n + 64) + 64` for a two's-complement `x`. -/
@[inline] def bias (x : UInt32) (n : Nat) : Nat :=
  let s := x ^^^ 0x80000000            -- order-preserving as unsigned
  if s ≤ 0x80000000 - 62 then 2
  else min (s - (0x80000000 - 64)).toNat (n + 128)

/-- Picture coordinate of biased coordinate `v`, clamped to [0, n - 1]. -/
@[inline] def clampIdx (v n : Nat) : Nat :=
  if v < 64 then 0 else min (v - 64) (n - 1)

def pushNGo (out : ByteArray) (v : UInt8) : Nat → ByteArray
  | 0 => out
  | k + 1 => pushNGo (out.push v) v k

def zeros64 : ByteArray := Yt.Bytes.filled 64 0

/-- `k` copies of `v` appended to `out` (zeros: one block copy). -/
def pushN (out : ByteArray) (v : UInt8) (k : Nat) : ByteArray :=
  if v == 0 && k ≤ 64 then zeros64.copySlice 0 out out.size k else pushNGo out v k

/-- `bw` columns from biased column `c` of the reference row at byte `row`,
clamped, appended to `out`: the replicated left edge, the samples inside the
picture (one block copy), the replicated right edge. -/
def emuRow (out ref : ByteArray) (row c refW bw : Nat) : ByteArray :=
  let l := if c < 64 then min bw (64 - c) else 0
  let first := c + l
  let m := if first ≥ refW + 64 then 0 else min (bw - l) (refW + 64 - first)
  let out := pushN out (px ref row).toUInt8 l
  let out := ref.copySlice (row + (first - 64)) out out.size m
  pushN out (px ref (row + refW - 1)).toUInt8 (bw - l - m)

def emuRows (out ref : ByteArray) (refStride refW refH bx r bw : Nat) : Nat → ByteArray
  | 0 => out
  | k + 1 =>
    let out := emuRow out ref (clampIdx r refH * refStride) bx refW bw
    emuRows out ref refStride refW refH bx (r + 1) bw k

/-- The `bw × bh` window (row stride `bw`) of the reference picture whose
top-left sample is at biased position (`cx`, `cy`), with edge replication. -/
def emuWindow (ref : ByteArray) (refStride refW refH cx cy bw bh : Nat) : ByteArray :=
  emuRows (ByteArray.emptyWithCapacity (bw * bh)) ref refStride refW refH cx cy bw bh

/-- `h` rows of `w` samples from `s[q]` (stride `ss`) to `dst[d]` (stride
`ds`) (Kernels.copyRows): the integer-position `put` (mc00, and chroma with
both fractions 0). -/
@[inline] def copyRows (dst : ByteArray) (d ds : Nat) (s : ByteArray) (q ss w h : Nat) : ByteArray :=
  Kernels.copyRows dst d ds s q ss w h

def emuRowsPadded (out ref : ByteArray) (refStride refW refH bx r bw ws : Nat) : Nat → ByteArray
  | 0 => out
  | k + 1 =>
    let out := pushN (emuRow out ref (clampIdx r refH * refStride) bx refW bw) 0 (ws - bw)
    emuRowsPadded out ref refStride refW refH bx (r + 1) bw ws k

/-- The same window with rows of `ws` ≥ `bw` bytes and 64 bytes of slack, as
the block kernels need (one stride for source and destination, aligned rows
when `ws` is a multiple of 16, wide loads past the window). -/
def emuWindowPadded (ref : ByteArray) (refStride refW refH cx cy bw bh ws : Nat) : ByteArray :=
  pushN (emuRowsPadded (ByteArray.emptyWithCapacity (ws * bh + 64)) ref refStride refW refH cx cy bw ws bh) 0 64

/-- Block `w × h` of `dst` (at `off`, stride `ds`) in a buffer of stride `ws`
with 64 bytes of slack: the destination for a kernel run on a padded window
(its current samples matter when averaging). -/
def blockOut (dst : ByteArray) (off ds w h ws : Nat) : ByteArray :=
  copyRows (Yt.Bytes.zeros (ws * h + 64)) 0 ws dst off ds w h

/-! ## Luma (8.4.2.2.1, FFmpeg h264qpel_template.c)

Samples of the 6-tap filter (1, -5, 20, 20, -5, 1) stay unclipped in two's
complement until FFmpeg's rounding: `(v + 16) >> 5` for one pass,
`(v + 512) >> 10` for the centre sample `j` (both passes), then
`av_clip_uint8`. Quarter positions average two clipped samples with
rounding, as FFmpeg's `_l2` helpers do. -/

@[inline] def tap6 (a b c d e f : UInt32) : UInt32 := (c + d) * 20 - (b + e) * 5 + (a + f)

/-- clip((v + 16) >> 5) for |v| < 2^19. -/
@[inline] def clip5 (v : UInt32) : UInt32 :=
  let u := (v + 0x80010) >>> 5
  if u < 0x4000 then 0 else let t := u - 0x4000; if t > 255 then 255 else t

/-- clip((v + 512) >> 10) for |v| < 2^20. -/
@[inline] def clip10 (v : UInt32) : UInt32 :=
  let u := (v + 0x100200) >>> 10
  if u < 0x400 then 0 else let t := u - 0x400; if t > 255 then 255 else t

/-- Unclipped horizontal 6-tap over `s[q .. q + 5]`. -/
@[inline] def hTap (s : ByteArray) (q : Nat) : UInt32 :=
  tap6 (px s q) (px s (q + 1)) (px s (q + 2)) (px s (q + 3)) (px s (q + 4)) (px s (q + 5))

/-- Unclipped vertical 6-tap over `s[q], s[q + ss], …, s[q + 5 ss]`. -/
@[inline] def vTap (s : ByteArray) (q ss : Nat) : UInt32 :=
  tap6 (px s q) (px s (q + ss)) (px s (q + 2 * ss)) (px s (q + 3 * ss)) (px s (q + 4 * ss))
    (px s (q + 5 * ss))

/-- One predicted luma sample for the fractions without the centre sample
`j` (`xy = dx + 4 dy`, not 6, 9, 10, 11 or 14). `q` is the window offset of
the top-left of the sample's 6×6 neighbourhood: its integer sample G is at
`q + 2 ss + 2`. Names as in Figure 8-4: b (half, right), h (half, below),
s (b of the next row), m (h of the next column). -/
@[inline] def lumaSample (s : ByteArray) (q ss xy : Nat) : UInt32 :=
  let g := q + 2 * ss + 2
  match xy with
  | 0 => px s g                                                             -- mc00
  | 1 => avg2 (px s g) (clip5 (hTap s (q + 2 * ss)))                        -- mc10
  | 2 => clip5 (hTap s (q + 2 * ss))                                        -- mc20
  | 3 => avg2 (px s (g + 1)) (clip5 (hTap s (q + 2 * ss)))                  -- mc30
  | 4 => avg2 (px s g) (clip5 (vTap s (q + 2) ss))                          -- mc01
  | 8 => clip5 (vTap s (q + 2) ss)                                          -- mc02
  | 12 => avg2 (px s (g + ss)) (clip5 (vTap s (q + 2) ss))                  -- mc03
  | 5 => avg2 (clip5 (hTap s (q + 2 * ss))) (clip5 (vTap s (q + 2) ss))     -- mc11
  | 7 => avg2 (clip5 (hTap s (q + 2 * ss))) (clip5 (vTap s (q + 3) ss))     -- mc31
  | 13 => avg2 (clip5 (hTap s (q + 3 * ss))) (clip5 (vTap s (q + 2) ss))    -- mc13
  | _ => avg2 (clip5 (hTap s (q + 3 * ss))) (clip5 (vTap s (q + 3) ss))     -- mc33

def lumaCols (dst : ByteArray) (d : Nat) (s : ByteArray) (q ss xy : Nat) (avg : Bool) : Nat → ByteArray
  | 0 => dst
  | n + 1 => lumaCols (store dst d (lumaSample s q ss xy) avg) (d + 1) s (q + 1) ss xy avg n

def lumaRows (dst : ByteArray) (d ds : Nat) (s : ByteArray) (q ss w xy : Nat) (avg : Bool) : Nat → ByteArray
  | 0 => dst
  | n + 1 => lumaRows (lumaCols dst d s q ss xy avg w) (d + ds) ds s (q + ss) ss w xy avg n

/-- One column of a fraction with the centre sample j (mc21, mc12, mc22,
mc32, mc23), top to bottom. `h0..h4` are the unclipped horizontal taps of
the five window rows above the current one; `r` is the window offset of the
current row's taps and `qv` the neighbourhood offset of the output sample. -/
def lumaJRows (dst : ByteArray) (d ds : Nat) (s : ByteArray) (r qv ss xy : Nat) (avg : Bool)
    (h0 h1 h2 h3 h4 : UInt32) : Nat → ByteArray
  | 0 => dst
  | n + 1 =>
    let h5 := hTap s r
    let j := clip10 (tap6 h0 h1 h2 h3 h4 h5)
    let v := match xy with
      | 10 => j                                    -- mc22
      | 6 => avg2 (clip5 h2) j                     -- mc21: b
      | 14 => avg2 (clip5 h3) j                    -- mc23: s
      | 9 => avg2 (clip5 (vTap s (qv + 2) ss)) j   -- mc12: h
      | _ => avg2 (clip5 (vTap s (qv + 3) ss)) j   -- mc32: m
    lumaJRows (store dst d v avg) (d + ds) ds s (r + ss) (qv + ss) ss xy avg h1 h2 h3 h4 h5 n

def lumaJCols (dst : ByteArray) (d ds : Nat) (s : ByteArray) (q ss h xy : Nat) (avg : Bool) : Nat → ByteArray
  | 0 => dst
  | n + 1 =>
    let dst := lumaJRows dst d ds s (q + 5 * ss) q ss xy avg
      (hTap s q) (hTap s (q + ss)) (hTap s (q + 2 * ss)) (hTap s (q + 3 * ss)) (hTap s (q + 4 * ss)) h
    lumaJCols dst (d + 1) ds s (q + 1) ss h xy avg n


/-- Luma prediction from a `(w + 5) × (h + 5)` window `s` (offset `q`, stride
`ss`) whose top-left is two samples left of and above the block. -/
def lumaFrom (dst : ByteArray) (d ds : Nat) (s : ByteArray) (q ss dx dy w h : Nat) (avg : Bool) : ByteArray :=
  let xy := dx + 4 * dy
  if xy == 0 && !avg then copyRows dst d ds s (q + 2 * ss + 2) ss w h
  else if xy == 6 || xy == 9 || xy == 10 || xy == 11 || xy == 14 then
    lumaJCols dst d ds s q ss h xy avg w
  else
    lumaRows dst d ds s q ss w xy avg h

/-- `bias` with sizes and result in USize. -/
@[inline] def biasU (x : UInt32) (n : USize) : USize :=
  let s := x ^^^ 0x80000000
  if s ≤ 0x80000000 - 62 then 2 else min (s - (0x80000000 - 64)).toUSize (n + 128)

/-- The blocks `lumaMcU` does not give the kernel directly (`cx`, `cy`:
the biased position): small ones and those reading outside the picture. -/
def lumaMcRest (dst : ByteArray) (dstOff dstStride : Nat) (ref : ByteArray) (refStride refW refH : Nat)
    (cx cy dx dy w h : Nat) (avg : Bool) : ByteArray :=
  if cx ≥ 66 && cy ≥ 66 && cx + w + 3 ≤ refW + 64 && cy + h + 3 ≤ refH + 64 then
    lumaFrom dst dstOff dstStride ref ((cy - 66) * refStride + (cx - 66)) refStride dx dy w h avg
  else if w ≥ 8 && h ≥ 8 then
    -- edge emulation into a 32-stride window, the block kernel into a
    -- 32-stride copy of the block, then the rows back
    let win := emuWindowPadded ref refStride refW refH (cx - 2) (cy - 2) (w + 5) (h + 5) 32
    let tmp := Kernels.qpel (blockOut dst dstOff dstStride w h 32) 0 win (2 * 32 + 2) 32 w.toUSize h.toUSize
      ((dx + 4 * dy).toUSize ||| (if avg then 16 else 0))
    copyRows dst dstOff dstStride tmp 0 32 w h
  else
    let win := emuWindow ref refStride refW refH (cx - 2) (cy - 2) (w + 5) (h + 5)
    lumaFrom dst dstOff dstStride win 0 (w + 5) dx dy w h avg

/-- `lumaMc` with positions and sizes in USize and the integer position (x,
y) as 32-bit two's complement. -/
def lumaMcU (dst : ByteArray) (dstOff dstStride : USize) (ref : ByteArray) (refStride refW refH : USize)
    (x y : UInt32) (dx dy w h : USize) (avg : Bool) : ByteArray :=
  let cx := biasU x refW
  let cy := biasU y refH
  -- the samples the filters read: 2 before and 3 after the block in a
  -- direction with a fractional position, none otherwise
  let l : USize := if dx == 0 then 0 else 2
  let r : USize := if dx == 0 then 0 else 3
  let t : USize := if dy == 0 then 0 else 2
  let b : USize := if dy == 0 then 0 else 3
  if w ≥ 8 && h ≥ 8 && dstStride == refStride &&
      cx ≥ 64 + l && cy ≥ 64 + t && cx + w + r ≤ refW + 64 && cy + h + b ≤ refH + 64 then
    -- inside the picture: the block kernel (NEON on the TV)
    Kernels.qpel dst dstOff ref ((cy - 64) * refStride + (cx - 64)) refStride w h
      ((dx + 4 * dy) ||| (if avg then 16 else 0))
  else lumaMcRest dst dstOff.toNat dstStride.toNat ref refStride.toNat refW.toNat refH.toNat cx.toNat cy.toNat
    dx.toNat dy.toNat w.toNat h.toNat avg

/-- `lumaMc` with the integer position (x, y) as 32-bit two's complement. -/
def lumaMc32 (dst : ByteArray) (dstOff dstStride : Nat) (ref : ByteArray) (refStride refW refH : Nat)
    (x y : UInt32) (dx dy w h : Nat) (avg : Bool) : ByteArray :=
  lumaMcU dst dstOff.toUSize dstStride.toUSize ref refStride.toUSize refW.toUSize refH.toUSize x y dx.toUSize
    dy.toUSize w.toUSize h.toUSize avg

/-- Quarter-sample luma prediction (8.4.2.2.1) of the `w × h` block
(w, h ∈ {16, 8, 4}) at `dst[dstOff]` (row stride `dstStride`) from the
reference picture `ref` (row stride `refStride`, coded size `refW × refH`).
(x, y) is the block's integer sample position in the reference (any value
in [-2^31, 2^31), far outside the picture included) and (dx, dy) ∈ 0..3 its
quarter-sample fraction: for the absolute quarter-sample position
mx = 4 · (block x) + mvx, x = mx >> 2 and dx = mx & 3 (FFmpeg's mc_dir_part).
Every reference read clamps to [0, refW - 1] × [0, refH - 1], which is
FFmpeg's emulated_edge_mc. `avg = false` stores the prediction
(put_h264_qpel{16,8,4}_mcXY_c); `avg = true` stores (dst + pred + 1) >> 1
(avg_h264_qpel…). Non-square blocks equal FFmpeg's two square calls. -/
def lumaMc (dst : ByteArray) (dstOff dstStride : Nat) (ref : ByteArray) (refStride refW refH : Nat)
    (x y : Int) (dx dy w h : Nat) (avg : Bool) : ByteArray :=
  lumaMc32 dst dstOff dstStride ref refStride refW refH (twos x) (twos y) dx dy w h avg

/-! ## Chroma (8.4.2.2.2, FFmpeg h264chroma_template.c) -/

def chromaCols (dst : ByteArray) (d : Nat) (s : ByteArray) (q ss : Nat) (a b c e : UInt32) (avg : Bool) :
    Nat → ByteArray
  | 0 => dst
  | n + 1 =>
    let v := (a * px s q + b * px s (q + 1) + c * px s (q + ss) + e * px s (q + ss + 1) + 32) >>> 6
    chromaCols (store dst d v avg) (d + 1) s (q + 1) ss a b c e avg n

def chromaRows (dst : ByteArray) (d ds : Nat) (s : ByteArray) (q ss w : Nat) (a b c e : UInt32) (avg : Bool) :
    Nat → ByteArray
  | 0 => dst
  | n + 1 => chromaRows (chromaCols dst d s q ss a b c e avg w) (d + ds) ds s (q + ss) ss w a b c e avg n

/-- The blocks `chromaMcU` does not give the kernel directly (`cx`, `cy`:
the biased position). -/
def chromaMcRest (dst : ByteArray) (dstOff dstStride : Nat) (ref : ByteArray) (refStride refW refH : Nat)
    (cx cy dx dy w h : Nat) (avg : Bool) : ByteArray :=
  let fx := dx.toUInt32
  let fy := dy.toUInt32
  if cx ≥ 64 && cy ≥ 64 && cx + w + (if dx == 0 then 0 else 1) ≤ refW + 64 &&
      cy + h + (if dy == 0 then 0 else 1) ≤ refH + 64 then
    chromaRows dst dstOff dstStride ref ((cy - 64) * refStride + (cx - 64)) refStride w ((8 - fx) * (8 - fy))
      (fx * (8 - fy)) ((8 - fx) * fy) (fx * fy) avg h
  else
    -- edge emulation into a 16-stride window, the block kernel into a
    -- 16-stride copy of the block, then the rows back
    let win := emuWindowPadded ref refStride refW refH cx cy (w + 1) (h + 1) 16
    let tmp := Kernels.chroma (blockOut dst dstOff dstStride w h 16) 0 win 0 16 w.toUSize h.toUSize dx.toUSize dy.toUSize
      (if avg then 1 else 0)
    copyRows dst dstOff dstStride tmp 0 16 w h

/-- `chromaMc` with positions and sizes in USize and the integer position
(x, y) as 32-bit two's complement. -/
def chromaMcU (dst : ByteArray) (dstOff dstStride : USize) (ref : ByteArray) (refStride refW refH : USize)
    (x y : UInt32) (dx dy w h : USize) (avg : Bool) : ByteArray :=
  let cx := biasU x refW
  let cy := biasU y refH
  -- the next column (row) is read only with a fractional x (y)
  if dstStride == refStride && cx ≥ 64 && cy ≥ 64 && cx + w + (if dx == 0 then 0 else 1) ≤ refW + 64 &&
      cy + h + (if dy == 0 then 0 else 1) ≤ refH + 64 then
    Kernels.chroma dst dstOff ref ((cy - 64) * refStride + (cx - 64)) refStride w h dx dy (if avg then 1 else 0)
  else chromaMcRest dst dstOff.toNat dstStride.toNat ref refStride.toNat refW.toNat refH.toNat cx.toNat cy.toNat
    dx.toNat dy.toNat w.toNat h.toNat avg

/-- `chromaMc` with the integer position (x, y) as 32-bit two's complement. -/
def chromaMc32 (dst : ByteArray) (dstOff dstStride : Nat) (ref : ByteArray) (refStride refW refH : Nat)
    (x y : UInt32) (dx dy w h : Nat) (avg : Bool) : ByteArray :=
  chromaMcU dst dstOff.toUSize dstStride.toUSize ref refStride.toUSize refW.toUSize refH.toUSize x y dx.toUSize
    dy.toUSize w.toUSize h.toUSize avg

/-- Eighth-sample chroma prediction (8.4.2.2.2) of the `w × h` block
(w ∈ {8, 4, 2}, any h; 4:2:0 uses h ∈ {8, 4, 2}) at `dst[dstOff]` from the
chroma plane `ref` (coded size `refW × refH`). (x, y) is the integer chroma
position and (dx, dy) ∈ 0..7 the eighth-sample fraction: for the absolute
position mx = 8 · (chroma block x) + mvx (the luma vector, in chroma
eighths for 4:2:0), x = mx >> 3 and dx = mx & 7. Same clamping and put/avg meaning as `lumaMc`;
FFmpeg's put_/avg_h264_chroma_mc{8,4,2}_8_c. -/
def chromaMc (dst : ByteArray) (dstOff dstStride : Nat) (ref : ByteArray) (refStride refW refH : Nat)
    (x y : Int) (dx dy w h : Nat) (avg : Bool) : ByteArray :=
  chromaMc32 dst dstOff dstStride ref refStride refW refH (twos x) (twos y) dx dy w h avg

/-! ## Weighted prediction (8.4.2.3, FFmpeg h264dsp_template.c) -/

def weightCols (dst : ByteArray) (d : Nat) (wt o ld : UInt32) : Nat → ByteArray
  | 0 => dst
  | n + 1 => weightCols (dst.set! d (clip8 (sar (px dst d * wt + o) ld)).toUInt8) (d + 1) wt o ld n

def weightRows (dst : ByteArray) (d stride w : Nat) (wt o ld : UInt32) : Nat → ByteArray
  | 0 => dst
  | n + 1 => weightRows (weightCols dst d wt o ld w) (d + stride) stride w wt o ld n

/-- `weight` with the weight and offset as 32-bit two's complement. -/
def weight32 (dst : ByteArray) (off stride w h log2Denom : Nat) (weight offset : UInt32) : ByteArray :=
  let ld := log2Denom.toUInt32
  let o := (offset <<< ld) + (if log2Denom > 0 then (1 : UInt32) <<< (ld - 1) else 0)
  weightRows dst off stride w weight o ld h

/-- Explicit weighted prediction of one list, in place on the `w × h`
prediction at `dst[off]` (row stride `stride`): FFmpeg's
weight_h264_pixels{16,8,4,2}_8_c(block, stride, h, log2Denom, weight,
offset), i.e. clip((p · weight + (offset << log2Denom) + round) >> log2Denom)
with round = 2^(log2Denom - 1) if log2Denom > 0 (8-8 of the spec).
`log2Denom` is luma_log2_weight_denom or chroma_log2_weight_denom (0..7),
`weight`/`offset` the slice's weight and offset for that reference and
component (the offset unscaled, as in the bitstream at 8 bits). -/
def weight (dst : ByteArray) (off stride w h log2Denom : Nat) (weight offset : Int) : ByteArray :=
  weight32 dst off stride w h log2Denom (twos weight) (twos offset)

def biweightCols (dst : ByteArray) (d : Nat) (src : ByteArray) (sp : Nat) (w0 w1 o ld : UInt32) :
    Nat → ByteArray
  | 0 => dst
  | n + 1 =>
    let v := clip8 (sar (px src sp * w1 + px dst d * w0 + o) ld)
    biweightCols (dst.set! d v.toUInt8) (d + 1) src (sp + 1) w0 w1 o ld n

def biweightRows (dst : ByteArray) (d ds : Nat) (src : ByteArray) (sp ss w : Nat) (w0 w1 o ld : UInt32) :
    Nat → ByteArray
  | 0 => dst
  | n + 1 => biweightRows (biweightCols dst d src sp w0 w1 o ld w) (d + ds) ds src (sp + ss) ss w w0 w1 o ld n

/-- `biweight` with the weights and offset as 32-bit two's complement. -/
def biweight32 (dst : ByteArray) (dstOff dstStride : Nat) (src : ByteArray) (srcOff srcStride w h log2Denom : Nat)
    (w0 w1 offset : UInt32) : ByteArray :=
  let ld := log2Denom.toUInt32
  biweightRows dst dstOff dstStride src srcOff srcStride w w0 w1 (((offset + 1) ||| 1) <<< ld) (ld + 1) h

/-- Bi-predictive weighted prediction, in place: `dst` holds the L0
prediction, `src` the L1 prediction (each `w × h` with its own offset and
stride), and the result replaces `dst`. This is FFmpeg's
biweight_h264_pixels{16,8,4,2}_8_c(dst, src, stride, h, log2_denom,
weightd = w0, weights = w1, offset):
clip((L1 · w1 + L0 · w0 + (((offset + 1) | 1) << log2Denom)) >> (log2Denom + 1)),
which equals 8-301 with logWD = log2Denom.
- Explicit (weighted_bipred_idc 1): `log2Denom` is the slice's
  luma/chroma_log2_weight_denom, `w0`/`w1` the L0/L1 weights of the two
  references, and `offset` is the SUM o0 + o1 of their offsets (unscaled,
  not yet halved; the formula's ((offset + 1) | 1) does the
  (o0 + o1 + 1) >> 1 rounding).
- Implicit (weighted_bipred_idc 2): `log2Denom = 5`, `w0` the implicit
  weight (64 - DistScaleFactor >> 2, or 32), `w1 = 64 - w0`, `offset = 0`,
  as h264_mb.c passes them. -/
def biweight (dst : ByteArray) (dstOff dstStride : Nat) (src : ByteArray) (srcOff srcStride w h log2Denom : Nat)
    (w0 w1 offset : Int) : ByteArray :=
  biweight32 dst dstOff dstStride src srcOff srcStride w h log2Denom (twos w0) (twos w1) (twos offset)

end Yt.H264.Inter
