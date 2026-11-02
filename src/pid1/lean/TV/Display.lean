prelude
import Init.System.IO
set_option compiler.extract_closed false

/-! Allocation-free pixel composition. Only the explicit BaseIO primitives below
access memory. The native caller owns nonoverlapping source/destination slices;
this entry point validates their lengths before any writes. It uses no Zr heap
or global state and may run on the native display worker. -/
namespace TV.Display

@[extern "tv_display_load"] private opaque load (address : USize) : BaseIO UInt32
@[extern "tv_display_store"] private opaque store (address : USize) (pixel : UInt32) : BaseIO UInt8
@[extern "tv_display_copy"] private opaque copy (dst src : USize) (pixels : UInt32) : BaseIO UInt8

private partial def fillRow (dst : USize) (count : UInt32) : BaseIO UInt8 := do
  if count == 0 then return 0
  let _ ← store dst 0xff000000
  fillRow (dst + 4) (count - 1)

private partial def clear (dst : USize) (width height stride row : UInt32) : BaseIO UInt8 := do
  if row >= height then return 0
  let _ ← fillRow (dst + (row * stride).toUSize * 4) width
  clear dst width height stride (row + 1)

private partial def scaleRow (dst src : USize) (width sourceWidth x : UInt32) : BaseIO UInt8 := do
  if x >= width then return 0
  let pixel ← load (src + (x * sourceWidth / width).toUSize * 4)
  let _ ← store (dst + x.toUSize * 4) pixel
  scaleRow dst src width sourceWidth (x + 1)

private partial def imageRows (dst src : USize) (stride sourceWidth sourceHeight width height left top row : UInt32) : BaseIO UInt8 := do
  if row >= height then return 0
  let dstRow := dst + ((top + row) * stride + left).toUSize * 4
  let srcRow := src + (row * sourceHeight / height * sourceWidth).toUSize * 4
  let _ ← if width == sourceWidth && height == sourceHeight then copy dstRow srcRow width
    else scaleRow dstRow srcRow width sourceWidth 0
  imageRows dst src stride sourceWidth sourceHeight width height left top (row + 1)

private partial def overlayRows (dst osd : USize) (stride left top row : UInt32) : BaseIO UInt8 := do
  if row >= 88 then return 0
  let _ ← copy (dst + ((top + row) * stride + left).toUSize * 4) (osd + (row * 480).toUSize * 4) 480
  overlayRows dst osd stride left top (row + 1)

/-- 0=success, 1=invalid buffer/dimensions, 2=scale overflow, 3=bad overlay.
No initializer is required. All arithmetic and memory work is bounded by the
validated buffers. The native caller supplies aligned live slices, an exclusive
destination and its actual lengths. Padding beyond visible width is untouched. -/
@[export tv_display_compose]
def compose (dst : USize) (dstPixels dw dh stride : UInt32)
    (src : USize) (srcPixels sw sh : UInt32) (osd : USize) (osdPixels : UInt32) : BaseIO UInt32 := do
  if dst == 0 || src == 0 || dw == 0 || dh == 0 || sw == 0 || sh == 0 || stride < dw then return 1
  let target := stride.toUInt64 * dh.toUInt64
  let source := sw.toUInt64 * sh.toUInt64
  if target > dstPixels.toUInt64 || source != srcPixels.toUInt64 then return 1
  let maxBytes := ((0 : USize) - 1).toUInt64
  if target * 4 > maxBytes || source * 4 > maxBytes then return 1
  if (osd == 0 && osdPixels != 0) || (osd != 0 && osdPixels != 480 * 88) then return 3
  let wide := dh.toUInt64 * sw.toUInt64
  let tall := dw.toUInt64 * sh.toUInt64
  if wide > 0xffffffff || tall > 0xffffffff then return 2
  let width := min dw (wide.toUInt32 / sh)
  let height := min dh (tall.toUInt32 / sw)
  if width.toUInt64 * sw.toUInt64 > 0xffffffff || height.toUInt64 * sh.toUInt64 > 0xffffffff then return 2
  let left := (dw - width) / 2
  let top := (dh - height) / 2
  let _ ← clear dst dw dh stride 0
  let _ ← imageRows dst src stride sw sh width height left top 0
  if osd != 0 && dw >= 480 && dh >= 116 then
    let _ ← overlayRows dst osd stride ((dw - 480) / 2) (dh - 116) 0
  return 0

end TV.Display
