import Doom.Render.Gfx.Patch
import Doom.Render.Types

/-!
# Doom.Render.Video

Screen buffer operations (`v_video.c` `V_DrawPatch`).
-/

namespace Doom.Render.Video

open Doom.Render.Gfx.Patch
open Doom.Render.Types
open Doom.Wad

/-- Draw patch at screen (x,y) into framebuffer; leftover/topoffset are signed 16-bit.
Posts are read in place from the WAD bytes. As in the decoder this replaces,
pixels below the patch height and off-screen pixels are skipped. -/
def drawPatch (wad : WadDirectory) (fb : Framebuffer) (lumpIdx : Nat) (x y : Int32) :
    Except String Framebuffer := do
  let some e := wad.entries[lumpIdx]? | throw s!"lump index {lumpIdx} out of range"
  let d := wad.data
  let base := e.filepos.toNat
  let size := e.size.toNat
  if base + size > d.size then throw s!"lump {lumpIdx} extends past the WAD"
  if size < 8 then throw "patch too small for header"
  let byte (o : Nat) : Nat := (d[base + o]?.getD 0).toNat
  let u16 (o : Nat) : Nat := byte o ||| (byte (o + 1) <<< 8)
  let width := u16 0
  let height := u16 2
  let x0 := x - (u16 4).toUInt16.toInt16.toInt32
  let y0 := y - (u16 6).toUInt16.toInt16.toInt32
  let mut px := fb.pixels
  for col in [0:width] do
    let o := 8 + col * 4
    if o + 4 > size then throw s!"EOF reading i32 at {o}"
    let colOff := byte o ||| (byte (o + 1) <<< 8) ||| (byte (o + 2) <<< 16) ||| (byte (o + 3) <<< 24)
    if colOff >= size then throw s!"patch column offset {colOff} out of range size={size}"
    let sx := x0 + Int32.ofNat col
    if sx < 0 || sx >= Int32.ofNat screenWidth then continue
    let ux := sx.toNatClampNeg
    let mut pos := colOff
    let mut guard := height + 4
    while guard > 0 do
      guard := guard - 1
      if pos >= size then throw s!"truncated patch column at offset {colOff}"
      let topdelta := byte pos
      if topdelta == 0xff then break
      if pos + 1 >= size then throw s!"truncated patch column at offset {colOff}"
      let len := byte (pos + 1)
      for row in [0:len] do
        let yy := topdelta + row
        if pos + 3 + row < size && yy < height then
          let sy := y0 + Int32.ofNat yy
          if sy >= 0 && sy < Int32.ofNat screenHeight then
            px := px.set! (sy.toNatClampNeg * screenWidth + ux) (d[base + pos + 3 + row]?.getD 0)
      pos := pos + 4 + len
  pure { pixels := px }

def fillRect (fb : Framebuffer) (x y w h : Nat) (color : UInt8) : Framebuffer :=
  Id.run do
    let mut out := fb
    let mut row : Nat := 0
    while row < h do
      let mut col : Nat := 0
      while col < w do
        let sx := x + col
        let sy := y + row
        if sx < screenWidth && sy < screenHeight then
          out := out.set sx sy color
        col := col + 1
      row := row + 1
    pure out

end Doom.Render.Video
