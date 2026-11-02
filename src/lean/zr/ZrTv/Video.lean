/-
The blit of decoded pictures into the display's frame for Lean apps, written
here over two DSL routines (`ZrAsm.Yuv`, assembled into the runtime's
`tv.S`): the NEON YUV 4:2:0 row kernel `zr_yuv420_row_neon` and the
nearest-neighbour row scaler `zr_u32_scale_row`. The pictures come from the
Lean H.264 decoder (Yt.H264) as byte arrays. ZrLow style, raw results as in
ZrTv.Linux.
-/
import ZrTv.Tv

set_option compiler.extract_closed false

namespace ZrTv.Video
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc ZrTv.Linux ZrTv.Tv

/-- `blocks` × 16 pixels of one row (ZrAsm.Yuv.row); `k` the constants block. -/
@[extern "zr_yuv420_row_neon"] opaque yuvRow (y u v out blocks k : USize) : BaseIO UInt8
/-- `dst[x] = src[x * sw / w]` for `x < n`, given `q = sw / w`, `r = sw % w` (ZrAsm.Yuv.scaleRow). -/
@[extern "zr_u32_scale_row"] opaque scaleRow (dst src n w q r : USize) : BaseIO UInt8

/-! ### The blit

BT.709 limited range in 10-bit fixed point, as src/ytplay/src/yuv.rs: per
pixel `R = clamp(((Y - 16) KY + 512 + (V - 128) KRV) >> 10, 0, 255)` and
likewise G, B, packed as `(R << rs) | (G << gs) | (B << bs) | alpha`. The
kernel needs the four fields disjoint (it adds them). -/

/-- The kernel's constants block (`struct zr_yuv_k`, 48 bytes) at `k`:
`KY KRV KGU KGV KBU` as int16 (padded to 8), `CR CG CB alpha` as int32 (the
offsets folded in, two's complement), `1 << rs`, `1 << gs`, `1 << bs`, 0. -/
def putConsts (k : USize) (rs gs bs alpha : UInt32) : BaseIO Unit := do
  st16 k 1192
  st16 (k + 2) 1836
  st16 (k + 4) 218
  st16 (k + 6) 546
  st16 (k + 8) 2163
  st16 (k + 10) 0
  st32 (k + 12) 0
  st32 (k + 16) ((512 : UInt32) - 16 * 1192 - 128 * 1836)
  st32 (k + 20) ((512 : UInt32) - 16 * 1192 + 128 * 218 + 128 * 546)
  st32 (k + 24) ((512 : UInt32) - 16 * 1192 - 128 * 2163)
  st32 (k + 28) alpha
  st32 (k + 32) ((1 : UInt32) <<< rs)
  st32 (k + 36) ((1 : UInt32) <<< gs)
  st32 (k + 40) ((1 : UInt32) <<< bs)
  st32 (k + 44) 0

/-- The channels are bytes inside the word and no two fields overlap. -/
def layoutOk (rs gs bs alpha : UInt32) : Bool :=
  let r := (0xff : UInt32) <<< rs
  let g := (0xff : UInt32) <<< gs
  let b := (0xff : UInt32) <<< bs
  rs ≤ 24 && gs ≤ 24 && bs ≤ 24 && r &&& g == 0 && r &&& b == 0 && g &&& b == 0 &&
    (r ||| g ||| b) &&& alpha == 0

/-- Convert `sw` pixels (even, ≥ 16) of a source row into `out`: whole
blocks, then one block ending at `sw` (it recomputes a few pixels, but never
reads past the row). -/
def convertRow (y u v out sw k : USize) : BaseIO Unit := do
  let _ ← yuvRow y u v out (sw / 16) k
  if sw % 16 != 0 then
    let off := sw - 16
    let _ ← yuvRow (y + off) (u + off / 2) (v + off / 2) (out + 4 * off) 1 k

/-- Destination rows `i ..< hh`: row `i` shows source row `i * sh / hh`; a
row showing the same source row as the previous one is a copy of it. -/
partial def blitRows (yp up vp ys cs sw sh dst stride w hh tmp i prev : USize) : BaseIO Unit := do
  if i < hh then
    let sy := i * sh / hh
    let out := dst + i * stride
    if i > 0 && sy == prev then
      copy out (out - stride) (4 * w)
    else
      let y := yp + sy * ys
      let u := up + sy / 2 * cs
      let v := vp + sy / 2 * cs
      if w == sw then convertRow y u v out sw tmp
      else
        let row := tmp + 48
        convertRow y u v row sw tmp
        let _ ← scaleRow out row w w (sw / w) (sw % w)
    blitRows yp up vp ys cs sw sh dst stride w hh tmp (i + 1) sy

/-- Draw a 4:2:0 picture (planes at `yp`, `up`, `vp`, rows every `ys`,
`cs`, `cs` bytes, `sw` x `sh` pixels) into the `w`×`h` rectangle at (`dx`,
`dy`) of the display's leased frame, scaled by nearest neighbour; pixels
outside the rectangle are untouched. 0, or -22 if the rectangle, the picture
or the pixel layout is not supported. -/
def blitPlanes (disp yp up vp ys cs vs sw sh dx dy w hh : USize) : BaseIO USize := do
  let fw := (← field disp 0).toUSize
  let fh := (← field disp 1).toUSize
  let stride := (← field disp 2).toUSize
  let rs ← field disp 3
  let gs ← field disp 4
  let bs ← field disp 5
  let alpha ← field disp 6
  if !(← leased disp) || sw < 16 || sw % 2 != 0 || sw > 8192 || sh == 0 || sh > 8192 || w == 0 || hh == 0 ||
      dx > fw || w > fw - dx || dy > fh || hh > fh - dy || stride < 4 * fw || ys < sw || cs < sw / 2 ||
      vs != cs || !layoutOk rs gs bs alpha then
    pure (0 - 22)
  else
    let tmp ← allocSArray 1 0 (48 + 4 * sw)
    let k := tmp + sarrData 0
    putConsts k rs gs bs alpha
    blitRows yp up vp ys cs sw sh ((← pixels disp) + dy * stride + 4 * dx) stride w hh k 0 0
    dec tmp
    pure 0

/-- Draw a picture given as three byte arrays (borrowed; Y `sw` x `sh`, Cb and
Cr `sw / 2` x `sh / 2`, packed rows), as the Lean H.264 decoder produces. -/
@[export zr_tv_video_blit_yuv]
def blitYuv (disp y u v sw sh dx dy w hh : USize) : BaseIO USize := do
  let ylen ← ldp (y + sarrSizeOff 0)
  let ulen ← ldp (u + sarrSizeOff 0)
  let vlen ← ldp (v + sarrSizeOff 0)
  if ylen < sw * sh || ulen < (sw / 2) * (sh / 2) || vlen < (sw / 2) * (sh / 2) then pure (0 - 22)
  else blitPlanes disp (y + sarrData 0) (u + sarrData 0) (v + sarrData 0) sw (sw / 2) (sw / 2) sw sh dx dy w hh

end ZrTv.Video
