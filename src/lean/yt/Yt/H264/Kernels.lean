/-
The H.264 decoder's sample kernels, by block: implemented on the TV by the
runtime (`ZrTv.H264Kernels`: FFmpeg's NEON assembly as written in the ZrAsm
DSL) and on the host by `Yt.Host.H264Kernels` (this decoder's Lean
implementation in `Yt.H264.Inter`, which is the specification the NEON
kernels are checked against: `zr-build transcript test/h264`).

Both planes are passed owned (exports cannot take borrowed arguments): the
destination is returned, the reference released. Both have the row stride
`stride`. Callers keep the block and every
sample the filters read inside the planes. Planes carry 256 bytes after the
last row (State.planeSlack): 64 for the kernels' wide loads, then 128 of
scratch for `idct`, and the last 8 scratch for `loopFilter`.
-/
namespace Yt.H264.Kernels

/-- Luma quarter-sample prediction of the `w`×`h` block (w, h ∈ {16, 8}) at
`dst[dOff]` from the reference at `src[sOff]` (the block's integer position);
`mode`: bits 0..3 the position dx + 4 dy, bit 4 average with `dst` instead of
storing (FFmpeg's avg_ variants). -/
@[extern "zr_h264_qpel"]
opaque qpel (dst : ByteArray) (dOff : USize) (src : ByteArray) (sOff stride w h mode : USize) : ByteArray

/-- Chroma eighth-sample prediction of the `w`×`h` block (w ∈ {8, 4, 2}) with
fraction (`fx`, `fy`) ∈ 0..7; `avg` ≠ 0 averages with `dst`. -/
@[extern "zr_h264_chroma"]
opaque chroma (dst : ByteArray) (dOff : USize) (src : ByteArray) (sOff stride w h fx fy avg : USize) : ByteArray

/-- The bS < 4 loop filter along an edge whose q0 sample is at `plane[off]`
(16 lines of luma, or 8 of chroma when `chroma` ≠ 0): `dir` 0 a vertical
edge, 1 a horizontal one; `tc`: the four segments' tC0 (chroma: tC0 + 1) as
int8, segment i in byte i (negative: segment skipped). -/
@[extern "zr_h264_loop_filter"]
opaque loopFilter (plane : ByteArray) (off stride alpha beta tc dir chroma : USize) : ByteArray

/-- `h` rows, `stride` bytes apart, of `w` entries of `esize` ∈ {1, 2, 4}
bytes (offsets and strides multiples of `esize`), each the low bytes of `v`
(little-endian), from `b[off]`; unchanged unless the block lies inside `b`
(and h, w ≤ 4096, stride ≤ 65536). The check is the kernel's: a branch here
would split callers' state updates (see RULES.md). -/
@[extern "zr_h264_fill"]
opaque fillRaw (b : ByteArray) (off stride w h v esize : USize) : ByteArray

/-- `h` rows of `w` bytes from `src[sOff]` (stride `sStride`) to `dst[dOff]`
(stride `dStride`); unchanged unless both blocks lie inside their arrays
(and h ≤ 4096, w and the strides ≤ 65536). -/
@[extern "zr_h264_copy_rows"]
opaque copyRowsRaw (dst : ByteArray) (dOff dStride : USize) (src : ByteArray) (sOff sStride w h : USize) : ByteArray

@[inline] def fill (b : ByteArray) (off stride w h esize : Nat) (v : UInt32) : ByteArray :=
  fillRaw b off.toUSize stride.toUSize w.toUSize h.toUSize v.toUSize esize.toUSize

@[inline] def copyRows (dst : ByteArray) (dOff ds : Nat) (src : ByteArray) (sOff ss w h : Nat) : ByteArray :=
  copyRowsRaw dst dOff.toUSize ds.toUSize src sOff.toUSize ss.toUSize w.toUSize h.toUSize

/-- The inverse transform of a residual block added to the plane at
`plane[po]` (row stride `stride`), from the int16 coefficients at
`coef[2 * off]`: `kind` 0 a 4x4 block, 1 its DC only, 2 an 8x8 block, 3 its
DC only (FFmpeg's idct_add, idct_dc_add, idct8_add, idct8_dc_add; the
coefficients are left as they are). Unchanged unless the block lies inside
the plane, which has its slack, and the coefficients inside `coef`. -/
@[extern "zr_h264_idct"]
opaque idct (plane : ByteArray) (po stride : USize) (coef : ByteArray) (off kind : USize) : ByteArray

/-- Intra prediction of the `w`×`h` block at `b[off]` (row stride `stride`)
by copying: `dir` 0 repeats the row above, 1 fills each row with the sample
to its left. Unchanged unless the block and the samples it copies (the row
above, or the column to the left) lie inside `b` (w ≤ 64). -/
@[extern "zr_h264_pred_copy"]
opaque predCopy (b : ByteArray) (off stride w h dir : USize) : ByteArray

/-- The bS = 4 loop filter along an edge whose q0 sample is at `b[off]`
(samples across it `xs` apart, lines `ys` apart): 16 luma lines, or 8 chroma
lines when `chroma` ≠ 0. Unchanged unless every sample read lies inside. -/
@[extern "zr_h264_intra_filter"]
opaque intraFilter (b : ByteArray) (off xs ys alpha beta chroma : USize) : ByteArray

/-- The deblocking boundary strengths of frame macroblock `xy` (type `t`;
picture `mbW` macroblocks wide) into `out[0..32)`: direction d's edge e at
bytes 16 d + 4 e, one byte per segment, 0 for an edge not filtered. From
the picture's tables (nnz, ref0, ref1, mv0, mv1) and per slice two 128-byte
reference maps in `maps`; `leftT`/`topT`: the neighbours' types (0: not
filtered across); `sc`, `sl`, `st`: the slices of the macroblock, its left
and top neighbours; `two`: two reference lists; `cbps`: luma cbp bits of the
macroblock, left (<< 4) and top (<< 8). `out` unchanged unless the tables
cover the blocks read. -/
@[extern "zr_h264_mb_bs"]
opaque mbBsRaw (out nnz ref0 ref1 mv0 mv1 maps : ByteArray) (xy mbW t leftT topT sc sl st two cbps : USize) : ByteArray

@[inline] def mbBs (out nnz ref0 ref1 mv0 mv1 maps : ByteArray) (xy mbW t leftT topT sc sl st two cbps : Nat) : ByteArray :=
  mbBsRaw out nnz ref0 ref1 mv0 mv1 maps xy.toUSize mbW.toUSize t.toUSize leftT.toUSize topT.toUSize sc.toUSize
    sl.toUSize st.toUSize two.toUSize cbps.toUSize

end Yt.H264.Kernels
