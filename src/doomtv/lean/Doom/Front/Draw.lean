import Doom.Render.Hud
import Doom.Render.Types
import Doom.Render.Video
import Doom.Wad

/-!
# Doom.Front.Draw

Screen drawing shared by the intermission, finale and automap
(`v_video.c` `V_DrawPatch` / `V_DrawPatchFlipped`, `f_finale.c`
`F_DrawPatchCol`, the HU font). Patches are read in place from the WAD
bytes; the framebuffer is updated in place while uniquely referenced.
Out-of-screen pixels are clipped (Chocolate Doom would `I_Error` on a
`RANGECHECK` violation instead).
-/

namespace Doom.Front.Draw

open Doom.Render.Types
open Doom.Wad

/-- A patch lump's header fields (`patch_t`). -/
structure PatchInfo where
  lump : Nat
  width : Int32
  height : Int32
  leftoffset : Int32
  topoffset : Int32
  deriving Inhabited

private def byteAt (wad : WadDirectory) (i : Nat) : Nat := (wad.data[i]?.getD 0).toNat

private def i16At (wad : WadDirectory) (i : Nat) : Int32 :=
  (byteAt wad i ||| (byteAt wad (i + 1) <<< 8)).toUInt16.toInt16.toInt32

/-- Header of patch lump `lump` (zeros when it is too short). -/
def patchInfo (wad : WadDirectory) (lump : Nat) : PatchInfo :=
  match wad.entries[lump]? with
  | some e =>
    if e.size.toNat < 8 then { lump, width := 0, height := 0, leftoffset := 0, topoffset := 0 }
    else
      let b := e.filepos.toNat
      { lump, width := i16At wad b, height := i16At wad (b + 2),
        leftoffset := i16At wad (b + 4), topoffset := i16At wad (b + 6) }
  | none => { lump, width := 0, height := 0, leftoffset := 0, topoffset := 0 }

/-- `W_CacheLumpName` for a patch: its header, if the lump exists. -/
def patchNamed? (wad : WadDirectory) (name : String) : Option PatchInfo :=
  (checkNumForName wad name).map (patchInfo wad)

/-- `V_DrawPatch`. -/
def patch (wad : WadDirectory) (fb : Framebuffer) (p : PatchInfo) (x y : Int32) :
    Except String Framebuffer :=
  Doom.Render.Video.drawPatch wad fb p.lump x y

/-- `V_DrawPatch` by lump name; a missing lump draws nothing. -/
def named (wad : WadDirectory) (fb : Framebuffer) (name : String) (x y : Int32) :
    Except String Framebuffer :=
  match checkNumForName wad name with
  | some i => Doom.Render.Video.drawPatch wad fb i x y
  | none => pure fb

/-- Draw column `col` of patch `lump` at screen column `sx`, with the patch
origin at `y0`. Off-screen pixels are skipped. -/
@[inline] private def drawColumn (wad : WadDirectory) (px : ByteArray) (base size height : Nat)
    (col : Nat) (sx y0 : Int32) : ByteArray := Id.run do
  let d := wad.data
  let o := base + 8 + col * 4
  if col * 4 + 12 > size then return px
  let colOff := byteAt wad o ||| (byteAt wad (o + 1) <<< 8) ||| (byteAt wad (o + 2) <<< 16) |||
    (byteAt wad (o + 3) <<< 24)
  if colOff >= size || sx < 0 || sx >= 320 then return px
  let ux := sx.toNatClampNeg
  let mut px := px
  let mut pos := colOff
  let mut guard := height + 4
  while guard > 0 do
    guard := guard - 1
    if pos >= size then break
    let topdelta := byteAt wad (base + pos)
    if topdelta == 0xff then break
    if pos + 1 >= size then break
    let len := byteAt wad (base + pos + 1)
    for row in [0:len] do
      if pos + 3 + row < size then
        let sy := y0 + Int32.ofNat (topdelta + row)
        if sy >= 0 && sy < 200 then
          px := px.set! (sy.toNatClampNeg * 320 + ux) (d[base + pos + 3 + row]?.getD 0)
    pos := pos + 4 + len
  return px

/-- `V_DrawPatchFlipped`: the patch mirrored left to right. -/
def patchFlipped (wad : WadDirectory) (fb : Framebuffer) (p : PatchInfo) (x y : Int32) :
    Framebuffer := Id.run do
  let some e := wad.entries[p.lump]? | return fb
  let base := e.filepos.toNat
  let size := e.size.toNat
  if base + size > wad.data.size || size < 8 then return fb
  let w := p.width.toNatClampNeg
  let x0 := x - p.leftoffset
  let y0 := y - p.topoffset
  let mut px := fb.pixels
  for col in [0:w] do
    px := drawColumn wad px base size p.height.toNatClampNeg (w - 1 - col) (x0 + Int32.ofNat col) y0
  return { pixels := px }

/-- `F_DrawPatchCol`: column `col` of patch `p` at screen column `x`, from the
top of the screen (offsets ignored). -/
def patchColumn (wad : WadDirectory) (fb : Framebuffer) (p : PatchInfo) (x : Int32) (col : Nat) :
    Framebuffer := Id.run do
  let some e := wad.entries[p.lump]? | return fb
  let base := e.filepos.toNat
  let size := e.size.toNat
  if base + size > wad.data.size || size < 8 || col >= p.width.toNatClampNeg then return fb
  return { pixels := drawColumn wad fb.pixels base size p.height.toNatClampNeg col x 0 }

/-- Fill the whole screen with a 64x64 flat (`F_TextWrite` background). -/
def fillFlat (wad : WadDirectory) (fb : Framebuffer) (name : String) : Framebuffer := Id.run do
  let some i := checkNumForName wad name | return fb
  let some e := wad.entries[i]? | return fb
  let base := e.filepos.toNat
  if e.size.toNat < 4096 || base + 4096 > wad.data.size then return fb
  let d := wad.data
  let mut px := fb.pixels
  let mut o := 0
  for y in [0:200] do
    let row := base + ((y &&& 63) <<< 6)
    for x in [0:320] do
      px := px.set! o (d[row + (x &&& 63)]!)
      o := o + 1
  return { pixels := px }

/-- Fill rows `[0, h)` of width `w` with one colour (`AM_clearFB`). -/
def clearTop (fb : Framebuffer) (w h : Nat) (color : UInt8) : Framebuffer := Id.run do
  let mut px := fb.pixels
  let n := min (w * h) px.size
  for i in [0:n] do
    px := px.set! i color
  return { pixels := px }

/-- The HU font (`hu_font[]`, `STCFN033`..`STCFN095`) as lump numbers. -/
structure Font where
  /-- Patch for character `HU_FONTSTART + i`. -/
  glyphs : Array PatchInfo := #[]

def HU_FONTSTART : Nat := 33
def HU_FONTSIZE : Nat := 63

def loadFont (wad : WadDirectory) : Font :=
  { glyphs := (List.range HU_FONTSIZE).toArray.map fun i =>
      (patchNamed? wad (Doom.Render.Hud.stcfnName (HU_FONTSTART + i))).getD
        { lump := 0, width := 0, height := 0, leftoffset := 0, topoffset := 0 } }

/-- Glyph for byte `c` after `toupper`, if it is in the font. -/
def Font.glyph? (f : Font) (c : UInt8) : Option PatchInfo :=
  let u := if c >= 97 && c <= 122 then c.toNat - 32 else c.toNat
  if u < HU_FONTSTART then none
  else match f.glyphs[u - HU_FONTSTART]? with
    | some g => if g.width == 0 && g.lump == 0 then none else some g
    | none => none

end Doom.Front.Draw
