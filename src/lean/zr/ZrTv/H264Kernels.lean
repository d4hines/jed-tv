/-
DSL NEON kernels for the Lean H.264 decoder (`Yt.H264.Kernels`): FFmpeg's
32-bit ARM assembly as written in ZrAsm.Ffmpeg (h264qpel_neon,
h264cmc_neon, hpeldsp_neon, h264dsp_neon; assembled into OUT/obj/h264 by
`zr-build rt`),
called on the bytes of the decoder's planes. ZrLow style.

Each entry takes the destination plane owned (made exclusive, returned)
and the reference plane owned (released), with byte offsets of the block;
both have the row stride `stride`. The caller (Yt.H264.Inter) guarantees that the
block and the samples the filters read lie inside the planes; the planes
have 64 bytes of slack after the last row for the kernels' wide loads (the
last 8 of them are scratch for `loopFilter`).
-/
import ZrTv.Tv

set_option compiler.extract_closed false

namespace ZrTv.H264Kernels
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Arr ZrLean.Low.Rc

@[extern "ff_put_h264_qpel16_mc10_neon"] opaque put16_10 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc20_neon"] opaque put16_20 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc30_neon"] opaque put16_30 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc01_neon"] opaque put16_01 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc11_neon"] opaque put16_11 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc21_neon"] opaque put16_21 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc31_neon"] opaque put16_31 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc02_neon"] opaque put16_02 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc12_neon"] opaque put16_12 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc22_neon"] opaque put16_22 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc32_neon"] opaque put16_32 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc03_neon"] opaque put16_03 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc13_neon"] opaque put16_13 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc23_neon"] opaque put16_23 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel16_mc33_neon"] opaque put16_33 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc10_neon"] opaque put8_10 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc20_neon"] opaque put8_20 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc30_neon"] opaque put8_30 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc01_neon"] opaque put8_01 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc11_neon"] opaque put8_11 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc21_neon"] opaque put8_21 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc31_neon"] opaque put8_31 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc02_neon"] opaque put8_02 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc12_neon"] opaque put8_12 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc22_neon"] opaque put8_22 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc32_neon"] opaque put8_32 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc03_neon"] opaque put8_03 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc13_neon"] opaque put8_13 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc23_neon"] opaque put8_23 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_put_h264_qpel8_mc33_neon"] opaque put8_33 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc10_neon"] opaque avg16_10 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc20_neon"] opaque avg16_20 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc30_neon"] opaque avg16_30 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc01_neon"] opaque avg16_01 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc11_neon"] opaque avg16_11 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc21_neon"] opaque avg16_21 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc31_neon"] opaque avg16_31 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc02_neon"] opaque avg16_02 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc12_neon"] opaque avg16_12 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc22_neon"] opaque avg16_22 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc32_neon"] opaque avg16_32 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc03_neon"] opaque avg16_03 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc13_neon"] opaque avg16_13 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc23_neon"] opaque avg16_23 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel16_mc33_neon"] opaque avg16_33 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc10_neon"] opaque avg8_10 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc20_neon"] opaque avg8_20 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc30_neon"] opaque avg8_30 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc01_neon"] opaque avg8_01 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc11_neon"] opaque avg8_11 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc21_neon"] opaque avg8_21 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc31_neon"] opaque avg8_31 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc02_neon"] opaque avg8_02 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc12_neon"] opaque avg8_12 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc22_neon"] opaque avg8_22 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc32_neon"] opaque avg8_32 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc03_neon"] opaque avg8_03 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc13_neon"] opaque avg8_13 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc23_neon"] opaque avg8_23 (dst src stride : USize) : BaseIO UInt8
@[extern "ff_avg_h264_qpel8_mc33_neon"] opaque avg8_33 (dst src stride : USize) : BaseIO UInt8

@[extern "ff_put_pixels16_neon"] opaque putPixels16 (dst src stride h : USize) : BaseIO UInt8
@[extern "ff_put_pixels8_neon"] opaque putPixels8 (dst src stride h : USize) : BaseIO UInt8
@[extern "ff_avg_pixels16_neon"] opaque avgPixels16 (dst src stride h : USize) : BaseIO UInt8
@[extern "ff_avg_pixels8_neon"] opaque avgPixels8 (dst src stride h : USize) : BaseIO UInt8

@[extern "ff_put_h264_chroma_mc8_neon"] opaque putChroma8 (dst src stride h x y : USize) : BaseIO UInt8
@[extern "ff_put_h264_chroma_mc4_neon"] opaque putChroma4 (dst src stride h x y : USize) : BaseIO UInt8
@[extern "ff_put_h264_chroma_mc2_neon"] opaque putChroma2 (dst src stride h x y : USize) : BaseIO UInt8
@[extern "ff_avg_h264_chroma_mc8_neon"] opaque avgChroma8 (dst src stride h x y : USize) : BaseIO UInt8
@[extern "ff_avg_h264_chroma_mc4_neon"] opaque avgChroma4 (dst src stride h x y : USize) : BaseIO UInt8
@[extern "ff_avg_h264_chroma_mc2_neon"] opaque avgChroma2 (dst src stride h x y : USize) : BaseIO UInt8

/-- One `n`×`n` block (n = 16 or 8): the quarter-sample position `xy` = dx + 4 dy. -/
def qpelBlock (d s stride : USize) (n xy : USize) (avg : Bool) : BaseIO UInt8 :=
  if n == 16 then
    if avg then
      if xy == 0 then avgPixels16 d s stride 16
      else if xy == 1 then avg16_10 d s stride
      else if xy == 2 then avg16_20 d s stride
      else if xy == 3 then avg16_30 d s stride
      else if xy == 4 then avg16_01 d s stride
      else if xy == 5 then avg16_11 d s stride
      else if xy == 6 then avg16_21 d s stride
      else if xy == 7 then avg16_31 d s stride
      else if xy == 8 then avg16_02 d s stride
      else if xy == 9 then avg16_12 d s stride
      else if xy == 10 then avg16_22 d s stride
      else if xy == 11 then avg16_32 d s stride
      else if xy == 12 then avg16_03 d s stride
      else if xy == 13 then avg16_13 d s stride
      else if xy == 14 then avg16_23 d s stride
      else if xy == 15 then avg16_33 d s stride
      else pure 0
    else
      if xy == 0 then putPixels16 d s stride 16
      else if xy == 1 then put16_10 d s stride
      else if xy == 2 then put16_20 d s stride
      else if xy == 3 then put16_30 d s stride
      else if xy == 4 then put16_01 d s stride
      else if xy == 5 then put16_11 d s stride
      else if xy == 6 then put16_21 d s stride
      else if xy == 7 then put16_31 d s stride
      else if xy == 8 then put16_02 d s stride
      else if xy == 9 then put16_12 d s stride
      else if xy == 10 then put16_22 d s stride
      else if xy == 11 then put16_32 d s stride
      else if xy == 12 then put16_03 d s stride
      else if xy == 13 then put16_13 d s stride
      else if xy == 14 then put16_23 d s stride
      else if xy == 15 then put16_33 d s stride
      else pure 0
  else
    if avg then
      if xy == 0 then avgPixels8 d s stride 8
      else if xy == 1 then avg8_10 d s stride
      else if xy == 2 then avg8_20 d s stride
      else if xy == 3 then avg8_30 d s stride
      else if xy == 4 then avg8_01 d s stride
      else if xy == 5 then avg8_11 d s stride
      else if xy == 6 then avg8_21 d s stride
      else if xy == 7 then avg8_31 d s stride
      else if xy == 8 then avg8_02 d s stride
      else if xy == 9 then avg8_12 d s stride
      else if xy == 10 then avg8_22 d s stride
      else if xy == 11 then avg8_32 d s stride
      else if xy == 12 then avg8_03 d s stride
      else if xy == 13 then avg8_13 d s stride
      else if xy == 14 then avg8_23 d s stride
      else if xy == 15 then avg8_33 d s stride
      else pure 0
    else
      if xy == 0 then putPixels8 d s stride 8
      else if xy == 1 then put8_10 d s stride
      else if xy == 2 then put8_20 d s stride
      else if xy == 3 then put8_30 d s stride
      else if xy == 4 then put8_01 d s stride
      else if xy == 5 then put8_11 d s stride
      else if xy == 6 then put8_21 d s stride
      else if xy == 7 then put8_31 d s stride
      else if xy == 8 then put8_02 d s stride
      else if xy == 9 then put8_12 d s stride
      else if xy == 10 then put8_22 d s stride
      else if xy == 11 then put8_32 d s stride
      else if xy == 12 then put8_03 d s stride
      else if xy == 13 then put8_13 d s stride
      else if xy == 14 then put8_23 d s stride
      else if xy == 15 then put8_33 d s stride
      else pure 0

/-- Luma quarter-sample prediction (put, or average with `dst` when bit 4 of
`mode` is set) of the `w`×`h` block at `dst[dOff]` from `src[sOff]` (the
block's integer position in the reference), `xy` in bits 0..3 of `mode`;
w, h ∈ {16, 8} (a 16x8 or 8x16 block is two square ones, as in FFmpeg). -/
@[export zr_h264_qpel]
def qpel (dst dOff src sOff stride w h mode : USize) : BaseIO USize := do
  let dst ← sarrayEnsureExclusive dst
  let d := dst + sarrData 0 + dOff
  let s := src + sarrData 0 + sOff
  let xy := mode &&& 15
  let avg := mode &&& 16 != 0
  let n := if w < h then w else h
  let _ ← qpelBlock d s stride n xy avg
  if w > n then let _ ← qpelBlock (d + n) (s + n) stride n xy avg
  if h > n then let _ ← qpelBlock (d + n * stride) (s + n * stride) stride n xy avg
  dec src
  pure dst

/-- Chroma eighth-sample prediction of the `w`×`h` block (w ∈ {8, 4, 2}),
fraction (`fx`, `fy`), put or average (`avg` ≠ 0). -/
@[export zr_h264_chroma]
def chroma (dst dOff src sOff stride w h fx fy avg : USize) : BaseIO USize := do
  let dst ← sarrayEnsureExclusive dst
  let d := dst + sarrData 0 + dOff
  let s := src + sarrData 0 + sOff
  let _ ←
    if avg != 0 then
      if w == 8 then avgChroma8 d s stride h fx fy
      else if w == 4 then avgChroma4 d s stride h fx fy
      else avgChroma2 d s stride h fx fy
    else
      if w == 8 then putChroma8 d s stride h fx fy
      else if w == 4 then putChroma4 d s stride h fx fy
      else putChroma2 d s stride h fx fy
  dec src
  pure dst

@[extern "ff_h264_h_loop_filter_luma_neon"] opaque hLuma (pix stride alpha beta tc0 : USize) : BaseIO UInt8
@[extern "ff_h264_v_loop_filter_luma_neon"] opaque vLuma (pix stride alpha beta tc0 : USize) : BaseIO UInt8
@[extern "ff_h264_h_loop_filter_chroma_neon"] opaque hChroma (pix stride alpha beta tc0 : USize) : BaseIO UInt8
@[extern "ff_h264_v_loop_filter_chroma_neon"] opaque vChroma (pix stride alpha beta tc0 : USize) : BaseIO UInt8

/-- The bS < 4 loop filter along a 16-line luma or 8-line chroma edge whose
q0 sample is at `plane[off]`: `dir` 0 a vertical edge (FFmpeg's h_ filters),
1 a horizontal one; `tc`: the four segments' tC0 (chroma: tC0 + 1) as int8,
segment i in byte i. `plane` owned, returned. -/
@[export zr_h264_loop_filter]
def loopFilter (plane off stride alpha beta tc dir chroma : USize) : BaseIO USize := do
  let plane ← sarrayEnsureExclusive plane
  -- the four tC0 bytes go in the plane's slack (its last 8 bytes are
  -- scratch for this kernel, Yt.H264.Kernels)
  let k := plane + sarrData 0 + (← ldp (plane + sarrSizeOff 0)) - 8
  st32 k tc.toUInt32
  let p := plane + sarrData 0 + off
  let _ ←
    if chroma != 0 then (if dir == 0 then hChroma p stride alpha beta k else vChroma p stride alpha beta k)
    else if dir == 0 then hLuma p stride alpha beta k else vLuma p stride alpha beta k
  pure plane

/-! ### Block fills and copies

The decoder's caches and tables are byte arrays of 1, 2 or 4-byte entries
written in small rectangles (FFmpeg's fill_rectangle, its cache write-backs):
here direct stores. A block outside its array leaves the array unchanged
(the check is here, not in the caller, so the caller's state updates stay
free of branches). -/

/-- Whether `h` rows, `stride` bytes apart, of `n` bytes from `off` lie in
`size` bytes (bounded factors: no overflow). -/
@[inline] def blockInside (size off stride n h : USize) : Bool :=
  h != 0 && h ≤ 4096 && stride ≤ 65536 && n ≤ 65536 && off ≤ size && off + (h - 1) * stride + n ≤ size

partial def row32 (p n v : USize) : BaseIO Unit := do
  if n != 0 then
    st32 p v.toUInt32
    row32 (p + 4) (n - 1) v

partial def rows32 (p stride n h v : USize) : BaseIO Unit := do
  if h != 0 then
    row32 p n v
    rows32 (p + stride) stride n (h - 1) v

partial def row16 (p n v : USize) : BaseIO Unit := do
  if n != 0 then
    st16 p v.toUInt16
    row16 (p + 2) (n - 1) v

partial def rows16 (p stride n h v : USize) : BaseIO Unit := do
  if h != 0 then
    row16 p n v
    rows16 (p + stride) stride n (h - 1) v

partial def row8 (p n v : USize) : BaseIO Unit := do
  if n != 0 then
    st8 p v.toUInt8
    row8 (p + 1) (n - 1) v

partial def rows8 (p stride n h v : USize) : BaseIO Unit := do
  if h != 0 then
    row8 p n v
    rows8 (p + stride) stride n (h - 1) v

/-- `h` rows, `stride` bytes apart, of `w` entries of `esize` ∈ {1, 2, 4}
bytes (aligned to their size), each the low bytes of `v`, from `b[off]`:
stored in the widest units (4, 2 or 1 bytes) the addresses allow, with `v`
replicated to the unit. -/
@[export zr_h264_fill]
def fill (b off stride w h v esize : USize) : BaseIO USize := do
  let n := w * esize
  if !blockInside (← ldp (b + sarrSizeOff 0)) off stride n h || w > 4096 then return b
  let b ← sarrayEnsureExclusive b
  let p := b + sarrData 0 + off
  let a := p ||| stride ||| n
  let v16 := if esize == 1 then (v &&& 0xFF) * 0x0101 else v &&& 0xFFFF
  if a &&& 3 == 0 then rows32 p stride (n >>> 2) h (if esize == 4 then v else v16 * 0x10001)
  else if a &&& 1 == 0 && esize ≤ 2 then rows16 p stride (n >>> 1) h v16
  else if esize == 1 then rows8 p stride n h v
  else if esize == 2 then rows16 p stride w h v
  else rows32 p stride w h v
  pure b

partial def copyWords (d s n : USize) : BaseIO Unit := do
  if n != 0 then
    st32 d (← ld32 s)
    copyWords (d + 4) (s + 4) (n - 4)

partial def copyRowsGo (d ds s ss w h : USize) (words : Bool) : BaseIO Unit := do
  if h != 0 then
    if words then copyWords d s w else copy d s w
    copyRowsGo (d + ds) ds (s + ss) ss w (h - 1) words

/-- `h` rows of `w` bytes from `src[sOff]` (stride `sStride`) to `dst[dOff]`
(stride `dStride`): word stores when everything is word-aligned. -/
@[export zr_h264_copy_rows]
def copyRows (dst dOff dStride src sOff sStride w h : USize) : BaseIO USize := do
  if !blockInside (← ldp (dst + sarrSizeOff 0)) dOff dStride w h ||
     !blockInside (← ldp (src + sarrSizeOff 0)) sOff sStride w h then
    dec src
    return dst
  let dst ← sarrayEnsureExclusive dst
  let d := dst + sarrData 0 + dOff
  let s := src + sarrData 0 + sOff
  copyRowsGo d dStride s sStride w h ((d ||| s ||| dStride ||| sStride ||| w) &&& 3 == 0 && w ≤ 64)
  dec src
  pure dst

/-! ### Inverse transforms

FFmpeg's NEON transforms take the block transposed (FFmpeg transposes its
scan tables for them) and clear the coefficients they read; they run on a
transposed copy in the plane's slack (bytes 64..191 after the last row,
16-aligned as their loads need), so the decoder's coefficients stay as they
are. -/

/-- `n` int16 values from `src` to `dst` transposed: entry i of `dst` is
entry ((i & mask) << shift) | (i >> shift) of `src` (4x4: 2, 3; 8x8: 3, 7). -/
partial def transposeCopy (dst src i n shift mask : USize) : BaseIO Unit := do
  if i < n then
    st16 (dst + 2 * i) (← ld16 (src + 2 * (((i &&& mask) <<< shift) ||| (i >>> shift))))
    transposeCopy dst src (i + 1) n shift mask

@[extern "ff_h264_idct_add_neon"] opaque idctAddN (dst block stride : USize) : BaseIO UInt8
@[extern "ff_h264_idct_dc_add_neon"] opaque idctDcAddN (dst block stride : USize) : BaseIO UInt8
@[extern "ff_h264_idct8_add_neon"] opaque idct8AddN (dst block stride : USize) : BaseIO UInt8
@[extern "ff_h264_idct8_dc_add_neon"] opaque idct8DcAddN (dst block stride : USize) : BaseIO UInt8

/-- The inverse transform of the block at `coef[2 * off]` (int16; `kind` 0
4x4, 1 4x4 DC, 2 8x8, 3 8x8 DC) added to the plane at `plane[po]`. -/
@[export zr_h264_idct]
def idct (plane po stride coef off kind : USize) : BaseIO USize := do
  let n : USize := if kind ≥ 2 then 64 else 16
  let rows : USize := if kind ≥ 2 then 8 else 4
  let psize ← ldp (plane + sarrSizeOff 0)
  if psize < 512 || !blockInside (psize - 256) po stride rows rows || off > 4096 ||
     2 * (off + n) > (← ldp (coef + sarrSizeOff 0)) then
    dec coef
    return plane
  let plane ← sarrayEnsureExclusive plane
  let scratch := plane + sarrData 0 + psize - 192
  if kind ≥ 2 then transposeCopy scratch (coef + sarrData 0 + 2 * off) 0 64 3 7
  else transposeCopy scratch (coef + sarrData 0 + 2 * off) 0 16 2 3
  let dst := plane + sarrData 0 + po
  let _ ←
    if kind == 0 then idctAddN dst scratch stride
    else if kind == 1 then idctDcAddN dst scratch stride
    else if kind == 2 then idct8AddN dst scratch stride
    else idct8DcAddN dst scratch stride
  dec coef
  pure plane

/-! ### Intra prediction by copying, and the bS = 4 loop filters

FFmpeg's 32-bit ARM code has no NEON for these (4x4 and 8x8 blocks, intra
edges); they are scalar here. -/

partial def horizRows (p stride w h : USize) (words : Bool) : BaseIO Unit := do
  if h != 0 then
    let v := (← ld8 (p - 1)).toUSize
    if words then row32 p (w >>> (2 : USize)) (v * 0x01010101) else row8 p w v
    horizRows (p + stride) stride w (h - 1) words

/-- Intra prediction of the `w`×`h` block at `b[off]` by copying: `dir` 0
(vertical) repeats the row above it, 1 (horizontal) fills each row with the
sample to its left. Unchanged unless the block and the samples it copies
(the row above, or the column to the left) lie inside `b` (w ≤ 64). -/
@[export zr_h264_pred_copy]
def predCopy (b off stride w h dir : USize) : BaseIO USize := do
  let size ← ldp (b + sarrSizeOff 0)
  let ok :=
    if dir == 0 then off ≥ stride && blockInside size (off - stride) stride w (h + 1)
    else off ≥ 1 && blockInside size (off - 1) stride (w + 1) h
  if !ok || w > 64 then return b
  let b ← sarrayEnsureExclusive b
  let p := b + sarrData 0 + off
  let words := (p ||| stride ||| w) &&& 3 == 0
  if dir == 0 then copyRowsGo p stride (p - stride) 0 w h words else horizRows p stride w h words
  pure b

@[inline] def absd (a b : USize) : USize := if a ≥ b then a - b else b - a

/-- One line of the bS = 4 luma filter across the edge at `k` (samples `xs` apart). -/
def lumaIntraLine (k xs alpha beta : USize) : BaseIO Unit := do
  let p0 := (← ld8 (k - xs)).toUSize
  let p1 := (← ld8 (k - 2 * xs)).toUSize
  let q0 := (← ld8 k).toUSize
  let q1 := (← ld8 (k + xs)).toUSize
  if absd p0 q0 < alpha && absd p1 p0 < beta && absd q1 q0 < beta then
    if absd p0 q0 < (alpha >>> (2 : USize)) + 2 then
      let p2 := (← ld8 (k - 3 * xs)).toUSize
      let q2 := (← ld8 (k + 2 * xs)).toUSize
      let p3 := (← ld8 (k - 4 * xs)).toUSize
      let q3 := (← ld8 (k + 3 * xs)).toUSize
      if absd p2 p0 < beta then
        st8 (k - xs) ((p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4) >>> (3 : USize)).toUInt8
        st8 (k - 2 * xs) ((p2 + p1 + p0 + q0 + 2) >>> (2 : USize)).toUInt8
        st8 (k - 3 * xs) ((2 * p3 + 3 * p2 + p1 + p0 + q0 + 4) >>> (3 : USize)).toUInt8
      else
        st8 (k - xs) ((2 * p1 + p0 + q1 + 2) >>> (2 : USize)).toUInt8
      if absd q2 q0 < beta then
        st8 k ((p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4) >>> (3 : USize)).toUInt8
        st8 (k + xs) ((p0 + q0 + q1 + q2 + 2) >>> (2 : USize)).toUInt8
        st8 (k + 2 * xs) ((2 * q3 + 3 * q2 + q1 + q0 + p0 + 4) >>> (3 : USize)).toUInt8
      else
        st8 k ((2 * q1 + q0 + p1 + 2) >>> (2 : USize)).toUInt8
    else
      st8 (k - xs) ((2 * p1 + p0 + q1 + 2) >>> (2 : USize)).toUInt8
      st8 k ((2 * q1 + q0 + p1 + 2) >>> (2 : USize)).toUInt8

/-- One line of the bS = 4 chroma filter. -/
def chromaIntraLine (k xs alpha beta : USize) : BaseIO Unit := do
  let p0 := (← ld8 (k - xs)).toUSize
  let p1 := (← ld8 (k - 2 * xs)).toUSize
  let q0 := (← ld8 k).toUSize
  let q1 := (← ld8 (k + xs)).toUSize
  if absd p0 q0 < alpha && absd p1 p0 < beta && absd q1 q0 < beta then
    st8 (k - xs) ((2 * p1 + p0 + q1 + 2) >>> (2 : USize)).toUInt8
    st8 k ((2 * q1 + q0 + p1 + 2) >>> (2 : USize)).toUInt8

partial def intraLines (k xs ys alpha beta n chroma : USize) : BaseIO Unit := do
  if n != 0 then
    if chroma != 0 then chromaIntraLine k xs alpha beta else lumaIntraLine k xs alpha beta
    intraLines (k + ys) xs ys alpha beta (n - 1) chroma

/-- The bS = 4 filter along an edge whose q0 sample is at `b[off]`, samples
across it `xs` apart, lines `ys` apart: 16 lines of luma, or 8 of chroma
when `chroma` ≠ 0 (FFmpeg's h264_loop_filter_luma_intra/chroma_intra).
Unchanged unless every sample read lies inside `b`. -/
@[export zr_h264_intra_filter]
def intraFilter (b off xs ys alpha beta chroma : USize) : BaseIO USize := do
  let n : USize := if chroma != 0 then 8 else 16
  let reach : USize := if chroma != 0 then 2 else 4
  if xs > 65536 || ys > 65536 || off < reach * xs ||
     off + (n - 1) * ys + (reach - 1) * xs ≥ (← ldp (b + sarrSizeOff 0)) then return b
  let b ← sarrayEnsureExclusive b
  intraLines (b + sarrData 0 + off) xs ys alpha beta n chroma
  pure b

/-! ### Deblocking boundary strengths

FFmpeg's filter_mb_dir bS decisions for a frame macroblock, read straight
from the picture's tables (Yt.H264.Loop keeps no filter caches): nnz (48
bytes per macroblock: luma 4x4 blocks at 4 row + col), ref0/ref1 (4 per
macroblock: its 8x8 blocks), mv0/mv1 (16 packed x, y int16 vectors per
macroblock, 4 per row), and per slice two 128-byte maps from a stored
reference index byte to its picture (0xFF: none). -/

/-- The picture of stored reference byte `r` through the map at `m`. -/
@[inline] def mapRef (m r : USize) : BaseIO USize := do
  if r ≥ 128 then pure 0xFF else pure (← ld8 (m + r)).toUSize

/-- Whether packed vectors `a`, `b` differ by 4 or more in either component. -/
@[inline] def mvFar (a b : UInt32) : Bool :=
  let dx : UInt32 := ((a &&& 0xFFFF) ^^^ 0x8000) - ((b &&& 0xFFFF) ^^^ 0x8000) + 3
  let dy : UInt32 := ((a >>> (16 : UInt32)) ^^^ 0x8000) - ((b >>> (16 : UInt32)) ^^^ 0x8000) + 3
  dx ≥ 7 || dy ≥ 7

/-- `check_mv` (1 or 0) between 4x4 block (qc, qr) of macroblock `qxy`
(maps `qa`, `qb`) and (pc, pr) of `pxy` (maps `pa`, `pb`); `two`: two lists. -/
def checkMv (r0 r1 v0 v1 qa qb pa pb two qxy qc qr pxy pc pr : USize) : BaseIO USize := do
  let one : USize := 1
  let qri := 4 * qxy + 2 * (qr >>> one) + (qc >>> one)
  let pri := 4 * pxy + 2 * (pr >>> one) + (pc >>> one)
  let qm := 4 * (16 * qxy + qc + 4 * qr)
  let pm := 4 * (16 * pxy + pc + 4 * pr)
  let q0 ← mapRef qa (← ld8 (r0 + qri)).toUSize
  let p0 ← mapRef pa (← ld8 (r0 + pri)).toUSize
  let a0 ← ld32 (v0 + qm)
  let b0 ← ld32 (v0 + pm)
  let v := q0 != p0 || (q0 != 0xFF && mvFar a0 b0)
  if two == 0 then return (if v then 1 else 0)
  let q1 ← mapRef qb (← ld8 (r1 + qri)).toUSize
  let p1 ← mapRef pb (← ld8 (r1 + pri)).toUSize
  let a1 ← ld32 (v1 + qm)
  let b1 ← ld32 (v1 + pm)
  let v := v || q1 != p1 || mvFar a1 b1
  if !v then return 0
  if q0 != p1 || q1 != p0 then return 1
  return (if mvFar a0 b1 || mvFar a1 b0 then 1 else 0)

/-- bS of segment `i` of edge `edge` (`dir` 0: Q at (edge, i), P one
column left; 1: Q at (i, edge), P one row up; at edge 0 P is in `nxy`): 2
when either block has coefficients, else `v` when `known`, else check_mv. -/
def segBs (nz r0 r1 v0 v1 qa qb pa pb two dir edge i xy nxy known v : USize) : BaseIO USize := do
  let qc := if dir == 0 then edge else i
  let qr := if dir == 0 then i else edge
  let pc := if dir == 0 then (if edge == 0 then 3 else edge - 1) else i
  let pr := if dir == 0 then i else (if edge == 0 then 3 else edge - 1)
  let pxy := if edge == 0 then nxy else xy
  let nq ← ld8 (nz + 48 * xy + 4 * qr + qc)
  let np ← ld8 (nz + 48 * pxy + 4 * pr + pc)
  if nq != 0 || np != 0 then return 2
  if known != 0 then return v
  checkMv r0 r1 v0 v1 qa qb pa pb two xy qc qr pxy pc pr

/-- The four segments' bS of an edge, one per byte. -/
def edgeBs (nz r0 r1 v0 v1 qa qb pa pb two dir edge xy nxy known v : USize) : BaseIO USize := do
  let b0 ← segBs nz r0 r1 v0 v1 qa qb pa pb two dir edge 0 xy nxy known v
  let b1 ← segBs nz r0 r1 v0 v1 qa qb pa pb two dir edge 1 xy nxy known v
  let b2 ← segBs nz r0 r1 v0 v1 qa qb pa pb two dir edge 2 xy nxy known v
  let b3 ← segBs nz r0 r1 v0 v1 qa qb pa pb two dir edge 3 xy nxy known v
  return b0 ||| (b1 <<< (8 : USize)) ||| (b2 <<< (16 : USize)) ||| (b3 <<< (24 : USize))

/-- FFmpeg's edge_mask[dir][(mb_type >> 3) & 7]. -/
@[inline] def maskEdge (dir k : USize) : USize :=
  if k == 0 then 0
  else if dir == 0 then (if k < 4 then 3 else 1)
  else if k == 1 then 3 else if k < 4 then 1 else 3

/-- An internal edge (1..3) of macroblock `xy` of type `t` in direction `dir`. -/
def innerBs (nz r0 r1 v0 v1 qa qb two dir edge xy t edges me par0 cbp : USize) : BaseIO USize := do
  if edge ≥ edges || (t &&& 0x01000000 != 0 && edge &&& 1 == 1) then return 0
  if t &&& 7 != 0 then return 0x03030303
  let skipMv := edge &&& me != 0
  let v ←
    if !skipMv && par0 != 0 then
      checkMv r0 r1 v0 v1 qa qb qa qb two xy (if dir == 0 then edge else 0) (if dir == 0 then 0 else edge)
        xy (if dir == 0 then edge - 1 else 0) (if dir == 0 then 0 else edge - 1)
    else pure 0
  let known : USize := if skipMv || par0 != 0 then 1 else 0
  if known != 0 && cbp == 0 then return v * 0x01010101
  edgeBs nz r0 r1 v0 v1 qa qb qa qb two dir edge xy xy known v

/-- Edge 0 of macroblock `xy` (type `t`) in direction `dir`, whose
neighbour has type `mbmT` (0: not filtered) at `mbmXY` with maps `na`, `nb`;
`cbp`, `cbpN`: the luma cbp bits. -/
def outerBs (nz r0 r1 v0 v1 qa qb na nb two dir xy t mbmT mbmXY cbp cbpN part par0 : USize) : BaseIO USize := do
  if mbmT == 0 then return 0
  if (t ||| mbmT) &&& 7 != 0 then return 0x04040404
  let known : USize := if par0 != 0 && mbmT &&& part != 0 then 1 else 0
  let v ←
    if known != 0 then
      checkMv r0 r1 v0 v1 qa qb na nb two xy 0 0 mbmXY (if dir == 0 then 3 else 0) (if dir == 0 then 0 else 3)
    else pure 0
  if known != 0 && cbp == 0 && cbpN == 0 then return v * 0x01010101
  edgeBs nz r0 r1 v0 v1 qa qb na nb two dir 0 xy mbmXY known v

/-- The edges of macroblock `xy` in direction `dir` (see `outerBs`): bS
into `o[0..16)`. -/
def dirBs (o nz r0 r1 v0 v1 qa qb na nb two dir xy t mbmT mbmXY cbp cbpN : USize) : BaseIO Unit := do
  let me := maskEdge dir ((t >>> (3 : USize)) &&& 7)
  let edges : USize := if me == 3 && cbp == 0 then 1 else 4
  let part : USize := (0x8 : USize) ||| ((0x20 : USize) >>> dir)
  let par0 : USize := if t &&& part != 0 then 1 else 0
  st32 o (← outerBs nz r0 r1 v0 v1 qa qb na nb two dir xy t mbmT mbmXY cbp cbpN part par0).toUInt32
  st32 (o + 4) (← innerBs nz r0 r1 v0 v1 qa qb two dir 1 xy t edges me par0 cbp).toUInt32
  st32 (o + 8) (← innerBs nz r0 r1 v0 v1 qa qb two dir 2 xy t edges me par0 cbp).toUInt32
  st32 (o + 12) (← innerBs nz r0 r1 v0 v1 qa qb two dir 3 xy t edges me par0 cbp).toUInt32

/-- The boundary strengths of frame macroblock `xy` (type `t`; picture
`mbW` macroblocks wide) into `out[0..32)`: direction d's edge e at bytes
16 d + 4 e, one byte per segment, 0 where an edge is not filtered.
`leftT`/`topT`: the neighbours' types, 0 when not filtered across; `sc`,
`sl`, `st`: the slices (map pairs in `maps`) of the macroblock and its
left and top neighbours; `two`: two reference lists; `cbps`: luma cbp bits
of the macroblock, left (<< 4) and top (<< 8). `out` unchanged unless
every table covers the blocks read. -/
@[export zr_h264_mb_bs]
def mbBs (out nnz ref0 ref1 mv0 mv1 maps xy mbW t leftT topT sc sl st two cbps : USize) : BaseIO USize := do
  let n := xy + 1
  let nm := (← ldp (maps + sarrSizeOff 0)) >>> (8 : USize)
  let ok := (← ldp (out + sarrSizeOff 0)) ≥ 32 && n ≤ 0x100000 &&
    (← ldp (nnz + sarrSizeOff 0)) ≥ 48 * n && (← ldp (ref0 + sarrSizeOff 0)) ≥ 4 * n &&
    (← ldp (mv0 + sarrSizeOff 0)) ≥ 64 * n &&
    (two == 0 || ((← ldp (ref1 + sarrSizeOff 0)) ≥ 4 * n && (← ldp (mv1 + sarrSizeOff 0)) ≥ 64 * n)) &&
    sc < nm && sl < nm && st < nm && (leftT == 0 || xy ≥ 1) && (topT == 0 || xy ≥ mbW)
  if !ok then
    dec nnz; dec ref0; dec ref1; dec mv0; dec mv1; dec maps
    return out
  let out ← sarrayEnsureExclusive out
  let o := out + sarrData 0
  let nz := nnz + sarrData 0
  let r0 := ref0 + sarrData 0
  let r1 := ref1 + sarrData 0
  let v0 := mv0 + sarrData 0
  let v1 := mv1 + sarrData 0
  let mp := maps + sarrData 0
  let qa := mp + 256 * sc
  let la := mp + 256 * sl
  let ta := mp + 256 * st
  let cbp := cbps &&& 15
  dirBs o nz r0 r1 v0 v1 qa (qa + 128) la (la + 128) two 0 xy t leftT (xy - 1) cbp ((cbps >>> (4 : USize)) &&& 15)
  dirBs (o + 16) nz r0 r1 v0 v1 qa (qa + 128) ta (ta + 128) two 1 xy t topT (xy - mbW) cbp
    ((cbps >>> (8 : USize)) &&& 15)
  dec nnz; dec ref0; dec ref1; dec mv0; dec mv1; dec maps
  pure out

end ZrTv.H264Kernels
