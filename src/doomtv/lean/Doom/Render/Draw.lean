import Doom.Playsim.Fixed
import Doom.Render.Data
import Doom.Render.Palette
import Doom.Render.Types
import Doom.Render.Util
import Doom.Render.Video
import Doom.Wad

/-!
# Doom.Render.Draw

Column/span drawing (`r_draw.c` `R_DrawColumn`, `R_DrawSpan`) and the view
bezel (`R_FillBackScreen`, `R_VideoErase`, `R_DrawViewBorder`).
-/

namespace Doom.Render.Draw

open Doom.Playsim.Fixed
open Doom.Render.Data
open Doom.Render.Palette
open Doom.Render.Types
open Doom.Render.Util
open Doom.Render.Video
open Doom.Wad

def defaultCentery : Int32 := 84

structure ColumnDrawParams where
  x : Int32
  yl : Int32
  yh : Int32
  iscale : Int32
  texturemid : Int32
  /-- Column texels are `source[sourceBase + i]` (no per-column copies). -/
  source : ByteArray
  sourceBase : Nat := 0
  colormapLevel : Nat
  centery : Int32 := defaultCentery
  viewwindowx : Int32 := 0
  viewwindowy : Int32 := 0

structure MaskedColumnDrawParams where
  x : Int32
  sprtopscreen : Int32
  spryscale : Int32
  texturemid : Int32
  /-- Column posts start at `posts[postsBase]`. -/
  posts : ByteArray
  postsBase : Nat := 0
  mceilingclip : Array Int32
  mfloorclip : Array Int32
  colormapLevel : Nat
  centery : Int32 := defaultCentery
  /-- Sprite `dc_iscale = abs(xiscale)`. `none` keeps the wall `0xffffffff / spryscale` formula. -/
  iscaleOverride : Option Int32 := none
  viewwindowx : Int32 := 0
  viewwindowy : Int32 := 0

structure SpanDrawParams where
  x1 : Int32
  x2 : Int32
  y : Int32
  xfrac : Int32
  yfrac : Int32
  xstep : Int32
  ystep : Int32
  source : ByteArray
  sourceBase : Nat := 0
  colormapLevel : Nat
  viewwindowx : Int32 := 0
  viewwindowy : Int32 := 0

private def columnRangeError (yl yh x : Int32) : String :=
  s!"R_DrawColumn: {yl} to {yh} at {x}"

private def spanRangeError (x1 x2 y : Int32) : String :=
  s!"R_DrawSpan: {x1} to {x2} at {y}"

private def columnSpot (frac : Int32) : Nat :=
  i32ToNat ((frac >>> 16) &&& (127 : Int32))

@[inline] private def spanSpot (position : UInt32) : Nat :=
  let ytemp := (position >>> 4) &&& (0x0fc0 : UInt32)
  let xtemp := position >>> 26
  (xtemp ||| ytemp).toNat

private def packSpanPosition (xfrac yfrac : Int32) : UInt32 :=
  ((xfrac.toUInt32 <<< 10) &&& (0xffff0000 : UInt32)) ||| ((yfrac.toUInt32 >>> 6) &&& (0xffff : UInt32))

private def packSpanStep (xstep ystep : Int32) : UInt32 :=
  ((xstep.toUInt32 <<< 10) &&& (0xffff0000 : UInt32)) ||| ((ystep.toUInt32 >>> 6) &&& (0xffff : UInt32))

/-- Offset of a 256-byte light level inside `COLORMAP`. -/
@[inline] private def colormapBase (level : Nat) : Except String Nat :=
  if level >= 32 then throw s!"colormap level {level} out of range (max 31)" else pure (level * 256)

@[inline] private def shade (colormaps : ByteArray) (cmBase : Nat) (source : ByteArray) (spot : Nat) : UInt8 :=
  match source[spot]? with
  | some tex => colormaps[cmBase + tex.toNat]?.getD 0
  | none => 0

/-- Column inner loop: `n` texels down from `idx`, stepping `frac` by `iscale`.
Tail recursion with scalar arguments compiles to a plain C loop; a `for` with
several mutable variables boxes its state every iteration. -/
def columnLoop (cm src : ByteArray) (cmBase base iscale : Nat) (step : Int32)
    (px : ByteArray) (idx : Nat) (frac : Int32) : Nat → ByteArray
  | 0 => px
  | n + 1 =>
    let tex := byteAt src (base + i32ToNat ((frac >>> 16) &&& (127 : Int32)))
    columnLoop cm src cmBase base iscale step (setByteAt px idx (byteAt cm (cmBase + tex.toNat)))
      (idx + iscale) (frac + step) n

/-- Span inner loop (`R_DrawSpan`): `n` pixels right from `idx`. -/
def spanLoop (cm src : ByteArray) (cmBase base : Nat) (step : UInt32)
    (px : ByteArray) (idx : Nat) (position : UInt32) : Nat → ByteArray
  | 0 => px
  | n + 1 =>
    let ytemp := (position >>> 4) &&& (0x0fc0 : UInt32)
    let spot := ((position >>> 26) ||| ytemp).toNat
    let tex := byteAt src (base + spot)
    spanLoop cm src cmBase base step (setByteAt px idx (byteAt cm (cmBase + tex.toNat)))
      (idx + 1) (position + step) n

/-- `R_DrawColumn` — vertical wall column blit. Range-check is view-local. -/
def drawColumn (data : RenderData) (fb : Framebuffer) (p : ColumnDrawParams) :
    Except String Framebuffer := do
  let count := p.yh - p.yl
  if count < 0 then
    pure fb
  else if i32AsU32 p.x >= screenWidth.toUInt32 || p.yl < 0 || p.yh >= Int32.ofNat screenHeight then
    throw (columnRangeError p.yl p.yh p.x)
  else do
    let cmBase ← colormapBase p.colormapLevel
    let x := i32ToNat p.x + i32ToNat p.viewwindowx
    let y0 := i32ToNat p.yl + i32ToNat p.viewwindowy
    let frac := p.texturemid + (p.yl - p.centery) * p.iscale
    let px := columnLoop data.colormaps p.source cmBase p.sourceBase screenWidth p.iscale
      fb.pixels (y0 * screenWidth + x) frac (i32ToNat count + 1)
    pure { pixels := px }

/-- `R_DrawMaskedColumn` — masked midtexture / sprite posts (`r_things.c`). -/
def drawMaskedColumn (data : RenderData) (fb : Framebuffer) (p : MaskedColumnDrawParams) :
    Except String Framebuffer := do
  let xNat := i32ToNat p.x
  let ceilClip := p.mceilingclip.getD xNat 0
  let floorClip := p.mfloorclip.getD xNat (Int32.ofNat screenHeight)
  let basetexturemid := p.texturemid
  let iscale :=
    match p.iscaleOverride with
    | some v => v
    | none => (0xffffffff / p.spryscale.toUInt32).toInt32
  let mut out := fb
  let mut pos : Nat := p.postsBase
  let mut guard : Nat := p.posts.size + 8
  while guard > 0 do
    guard := guard - 1
    match p.posts[pos]? with
    | none => break
    | some topdelta =>
      if topdelta == 0xff then
        break
      else
        match p.posts[pos + 1]? with
        | none => throw "R_DrawMaskedColumn: truncated post"
        | some length =>
          let len := length.toNat
          let topscreen := p.sprtopscreen + p.spryscale * Int32.ofNat topdelta.toNat
          let bottomscreen := topscreen + p.spryscale * Int32.ofNat len
          let mut dcYl := (topscreen + FRACUNIT - 1) >>> 16
          let mut dcYh := (bottomscreen - 1) >>> 16
          if dcYh >= floorClip then
            dcYh := floorClip - 1
          if dcYl <= ceilClip then
            dcYl := ceilClip + 1
          if dcYl <= dcYh then
            let colParams : ColumnDrawParams := {
              x := p.x
              yl := dcYl
              yh := dcYh
              iscale := iscale
              texturemid := basetexturemid - Int32.ofNat topdelta.toNat * FRACUNIT
              source := p.posts
              sourceBase := pos + 3
              colormapLevel := p.colormapLevel
              centery := p.centery
              viewwindowx := p.viewwindowx
              viewwindowy := p.viewwindowy
            }
            out ← drawColumn data out colParams
          pos := pos + 4 + len
  pure out

/-- `FUZZTABLE` (`r_draw.c`). -/
def fuzzTableSize : Nat := 50

/-- `FUZZOFF` (`SCREENWIDTH`). -/
def fuzzOff : Int32 := Int32.ofNat screenWidth

/-- `fuzzoffset[FUZZTABLE]` (`r_draw.c`). -/
def fuzzoffset : Array Int32 := #[
  fuzzOff, -fuzzOff, fuzzOff, -fuzzOff, fuzzOff, fuzzOff, -fuzzOff,
  fuzzOff, fuzzOff, -fuzzOff, fuzzOff, fuzzOff, fuzzOff, -fuzzOff,
  fuzzOff, fuzzOff, fuzzOff, -fuzzOff, -fuzzOff, -fuzzOff, -fuzzOff,
  fuzzOff, -fuzzOff, -fuzzOff, fuzzOff, fuzzOff, fuzzOff, fuzzOff, -fuzzOff,
  fuzzOff, -fuzzOff, fuzzOff, fuzzOff, -fuzzOff, -fuzzOff, fuzzOff,
  fuzzOff, -fuzzOff, -fuzzOff, -fuzzOff, -fuzzOff, fuzzOff, fuzzOff,
  fuzzOff, fuzzOff, -fuzzOff, fuzzOff, fuzzOff, -fuzzOff, fuzzOff
]

#guard fuzzoffset.size == 50

structure FuzzColumnDrawParams where
  x : Int32
  yl : Int32
  yh : Int32
  viewheight : Int32
  viewwindowx : Int32 := 0
  viewwindowy : Int32 := 0

private def fuzzColumnRangeError (yl yh x : Int32) : String :=
  s!"R_DrawFuzzColumn: {yl} to {yh} at {x}"

private def stepFuzzpos (p : Nat) : Nat :=
  let n := p + 1
  if n == fuzzTableSize then 0 else n

private def fuzzSample (fb : Framebuffer) (destIdx : Nat) (off : Int32) : UInt8 :=
  let idx := (Int.ofNat destIdx) + off.toInt
  if idx < 0 then
    0
  else
    match fb.pixels[idx.toNat]? with
    | some b => b
    | none => 0

/-- `R_DrawFuzzColumn` — spectre/invisibility dither. `fuzzpos` persists across posts/frames. -/
def drawFuzzColumn (data : RenderData) (fb : Framebuffer) (p : FuzzColumnDrawParams)
    (fuzzpos : Nat) : Except String (Framebuffer × Nat) := do
  let mut yl := p.yl
  let mut yh := p.yh
  if yl == 0 then
    yl := 1
  if yh == p.viewheight - 1 then
    yh := p.viewheight - 2
  let count := yh - yl
  if count < 0 then
    pure (fb, fuzzpos)
  else if i32AsU32 p.x >= screenWidth.toUInt32 || yl < 0 || yh >= Int32.ofNat screenHeight then
    throw (fuzzColumnRangeError yl yh p.x)
  else do
    let cmBase ← colormapBase 6
    let x := i32ToNat p.x + i32ToNat p.viewwindowx
    let y0 := i32ToNat yl + i32ToNat p.viewwindowy
    let mut out := fb
    let mut y := y0
    let mut fp := fuzzpos
    let mut rem := count
    while true do
      let destIdx := y * screenWidth + x
      let off := fuzzoffset.getD fp 0
      let src := fuzzSample out destIdx off
      let pix := data.colormaps[cmBase + src.toNat]?.getD 0
      out := Framebuffer.setIdx out destIdx pix
      fp := stepFuzzpos fp
      if rem == 0 then
        break
      rem := rem - 1
      y := y + 1
    pure (out, fp)

/-- `R_DrawMaskedColumn` with `colfunc = fuzzcolfunc`. -/
def drawMaskedFuzzColumn (data : RenderData) (fb : Framebuffer) (p : MaskedColumnDrawParams)
    (viewheight : Int32) (fuzzpos : Nat) : Except String (Framebuffer × Nat) := do
  let xNat := i32ToNat p.x
  let ceilClip := p.mceilingclip.getD xNat 0
  let floorClip := p.mfloorclip.getD xNat (Int32.ofNat screenHeight)
  let mut out := fb
  let mut fp := fuzzpos
  let mut pos : Nat := p.postsBase
  let mut guard : Nat := p.posts.size + 8
  while guard > 0 do
    guard := guard - 1
    match p.posts[pos]? with
    | none => break
    | some topdelta =>
      if topdelta == 0xff then
        break
      else
        match p.posts[pos + 1]? with
        | none => throw "R_DrawMaskedColumn: truncated post"
        | some length =>
          let len := length.toNat
          let topscreen := p.sprtopscreen + p.spryscale * Int32.ofNat topdelta.toNat
          let bottomscreen := topscreen + p.spryscale * Int32.ofNat len
          let mut dcYl := (topscreen + FRACUNIT - 1) >>> 16
          let mut dcYh := (bottomscreen - 1) >>> 16
          if dcYh >= floorClip then
            dcYh := floorClip - 1
          if dcYl <= ceilClip then
            dcYl := ceilClip + 1
          if dcYl <= dcYh then
            let fuzzParams : FuzzColumnDrawParams := {
              x := p.x
              yl := dcYl
              yh := dcYh
              viewheight := viewheight
              viewwindowx := p.viewwindowx
              viewwindowy := p.viewwindowy
            }
            let (out', fp') ← drawFuzzColumn data out fuzzParams fp
            out := out'
            fp := fp'
          pos := pos + 4 + len
  pure (out, fp)

/-- `R_DrawSpan` — horizontal floor/ceiling span blit. Range-check is view-local. -/
def drawSpan (data : RenderData) (fb : Framebuffer) (p : SpanDrawParams) :
    Except String Framebuffer := do
  if p.x2 < p.x1 || p.x1 < 0 || p.x2 >= Int32.ofNat screenWidth || i32AsU32 p.y > screenHeight.toUInt32 then
    throw (spanRangeError p.x1 p.x2 p.y)
  else do
    let cmBase ← colormapBase p.colormapLevel
    let y := i32ToNat p.y + i32ToNat p.viewwindowy
    let px := spanLoop data.colormaps p.source cmBase p.sourceBase (packSpanStep p.xstep p.ystep)
      fb.pixels (y * screenWidth + i32ToNat p.x1 + i32ToNat p.viewwindowx)
      (packSpanPosition p.xfrac p.yfrac) (i32ToNat (p.x2 - p.x1) + 1)
    pure { pixels := px }

private def requireLump (wad : WadDirectory) (name : String) : Except String Nat :=
  match checkNumForName wad name with
  | none => throw s!"missing lump {name}"
  | some idx => pure idx

private def drawPatchStrip (wad : WadDirectory) (fb : Framebuffer) (lump : String)
    (limit step : Int32) (coord : Int32 → Int32 × Int32) : Except String Framebuffer := do
  let idx ← requireLump wad lump
  let mut out := fb
  let mut v : Int32 := 0
  while v < limit do
    let (px, py) := coord v
    out ← drawPatch wad out idx px py
    v := v + step
  pure out

private def tileFlatRow (fb : Framebuffer) (src : ByteArray) (y dest0 : Nat) : Framebuffer × Nat :=
  Id.run do
    let srcOff := (y &&& 63) <<< 6
    let mut out := fb
    let mut dest := dest0
    let mut tile : Nat := 0
    while tile < screenWidth / 64 do
      let mut b : Nat := 0
      while b < 64 do
        let pix := match src[srcOff + b]? with | some p => p | none => 0
        out := Framebuffer.setIdx out dest pix
        dest := dest + 1
        b := b + 1
      tile := tile + 1
    let rem := screenWidth &&& 63
    let mut r : Nat := 0
    while r < rem do
      let pix := match src[srcOff + r]? with | some p => p | none => 0
      out := Framebuffer.setIdx out dest pix
      dest := dest + 1
      r := r + 1
    (out, dest)

/--
`R_FillBackScreen`. Tiles `FLOOR7_2` (shareware) / `GRNROCK` (commercial) into a
320×168 background, then draws `brdr_*` patches. No-op when `scaledviewwidth`
is full screen width.
-/
def fillBackScreen (wad : WadDirectory) (scaledviewwidth viewheight viewwindowx viewwindowy : Int32)
    (commercial : Bool) : Except String (Option Framebuffer) := do
  if scaledviewwidth == Int32.ofNat screenWidth then
    return none
  let name := if commercial then "GRNROCK" else "FLOOR7_2"
  let srcIdx ← requireLump wad name
  let src ← lumpData wad srcIdx
  let mut bg := Framebuffer.initBlack
  let mut dest : Nat := 0
  let mut y : Nat := 0
  let bgH := screenHeight - sbarHeight
  while y < bgH do
    let (bg', dest') := tileFlatRow bg src y dest
    bg := bg'
    dest := dest'
    y := y + 1
  bg ← drawPatchStrip wad bg "brdr_t" scaledviewwidth 8 fun x => (viewwindowx + x, viewwindowy - 8)
  bg ← drawPatchStrip wad bg "brdr_b" scaledviewwidth 8 fun x =>
    (viewwindowx + x, viewwindowy + viewheight)
  bg ← drawPatchStrip wad bg "brdr_l" viewheight 8 fun y => (viewwindowx - 8, viewwindowy + y)
  bg ← drawPatchStrip wad bg "brdr_r" viewheight 8 fun y =>
    (viewwindowx + scaledviewwidth, viewwindowy + y)
  let tl ← requireLump wad "brdr_tl"
  bg ← drawPatch wad bg tl (viewwindowx - 8) (viewwindowy - 8)
  let tr ← requireLump wad "brdr_tr"
  bg ← drawPatch wad bg tr (viewwindowx + scaledviewwidth) (viewwindowy - 8)
  let bl ← requireLump wad "brdr_bl"
  bg ← drawPatch wad bg bl (viewwindowx - 8) (viewwindowy + viewheight)
  let br ← requireLump wad "brdr_br"
  bg ← drawPatch wad bg br (viewwindowx + scaledviewwidth) (viewwindowy + viewheight)
  pure (some bg)

/-- `R_VideoErase` — copy `count` pixels from background at `ofs` onto `fb`. -/
def videoErase (fb bg : Framebuffer) (ofs count : Nat) : Framebuffer :=
  -- One memcpy; clamped to both buffers like the per-pixel loop it replaces.
  let n := min count (min (bg.pixels.size - ofs) (fb.pixels.size - ofs))
  if ofs >= fb.pixels.size || ofs >= bg.pixels.size || n == 0 then fb
  else { pixels := bg.pixels.copySlice ofs fb.pixels ofs n }

/--
`R_DrawViewBorder`. Copies the three C bezel regions from `background_buffer`.
Skip when `scaledviewwidth` is full screen or there is no background.
-/
def drawViewBorder (fb : Framebuffer) (bg? : Option Framebuffer)
    (scaledviewwidth viewheight : Int32) : Framebuffer :=
  if scaledviewwidth == Int32.ofNat screenWidth then
    fb
  else
    match bg? with
    | none => fb
    | some bg =>
      Id.run do
        let screenW : Int32 := Int32.ofNat screenWidth
        let top := ((Int32.ofNat screenHeight - Int32.ofNat sbarHeight) - viewheight) / (2 : Int32)
        let side0 := (screenW - scaledviewwidth) / (2 : Int32)
        let topN := i32ToNat top
        let sideN := i32ToNat side0
        let countTop := topN * screenWidth + sideN
        let fb1 := videoErase fb bg 0 countTop
        let ofsBot := i32ToNat ((viewheight + top) * screenW - side0)
        let fb2 := videoErase fb1 bg ofsBot countTop
        let mut ofs := topN * screenWidth + screenWidth - sideN
        let side2 := sideN <<< 1
        let mut out := fb2
        let mut i : Nat := 1
        let vh := i32ToNat viewheight
        while i < vh do
          out := videoErase out bg ofs side2
          ofs := ofs + screenWidth
          i := i + 1
        pure out

end Doom.Render.Draw
