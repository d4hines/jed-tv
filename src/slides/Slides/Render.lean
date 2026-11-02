import Slides.Deck
import Slides.Font
namespace Slides
abbrev Color := UInt32
def width : Nat := 1280
def height : Nat := 720
def white : Color := 0xf0f0f5
def grey : Color := 0x969aaa
def accent : Color := 0xe62832
structure Canvas where
  bytes : ByteArray
  deriving Inhabited
private def pixel (c : Canvas) (x y : Nat) (color : Color) : Canvas := Id.run do
  if x ≥ width || y ≥ height then return c
  let p := (y*width+x)*4
  let mut b := c.bytes
  b := b.set! p color.toUInt8
  b := b.set! (p+1) (color >>> 8).toUInt8
  b := b.set! (p+2) (color >>> 16).toUInt8
  b := b.set! (p+3) 255
  return ⟨b⟩
def rect (c : Canvas) (x y w h : Nat) (color : Color) : Canvas := Id.run do
  let mut c := c
  for yy in [y:min height (y+h)] do
    for xx in [x:min width (x+w)] do c := pixel c xx yy color
  return c
def background : Canvas := Id.run do
  let mut b := ByteArray.emptyWithCapacity (width*height*4)
  for y in [:height] do
    let r := 22 - 16*y/height
    let g := 26 - 20*y/height
    let blue := 44 - 32*y/height
    for _ in [:width] do b := b.push blue.toUInt8 |>.push g.toUInt8 |>.push r.toUInt8 |>.push 255
  return ⟨b⟩
def text (c : Canvas) (x y scale : Nat) (color : Color) (s : String) (style : Style := .plain) : Canvas := Id.run do
  let mut c := c
  let mut xx := x
  for ch in s.toList do
    let code := ch.toNat
    if code == 8226 then c := rect c (xx+2*scale) (y+2*scale) (3*scale) (3*scale) color
    else
      let glyph := font[if code < 128 then code else 63]!
      for row in [:8] do
        let bits := glyph[row]!
        let skew := if style == .italic || style == .boldItalic then (7-row)*scale/4 else 0
        for col in [:8] do
          if bits &&& ((1 : UInt8) <<< col.toUInt8) != 0 then
            c := rect c (xx+col*scale+skew) (y+row*scale) (scale + (if style == Style.bold || style == Style.boldItalic then 1 else 0)) scale color
    xx := xx+8*scale
  return c
/-- Wrap whole words; long code/URLs are split so nothing runs off the panel. -/
def wrap (s : String) (columns : Nat) : Array String := Id.run do
  let columns := max 1 columns
  let mut lines := #[]
  let mut line := ""
  for word in s.splitOn " " do
    if word.isEmpty then continue
    let mut word := word
    if !line.isEmpty && line.length+1+word.length > columns then lines := lines.push line; line := ""
    while word.length > columns do
      if !line.isEmpty then lines := lines.push line; line := ""
      lines := lines.push (word.take columns).toString
      word := (word.drop columns).toString
    if !word.isEmpty then line := if line.isEmpty then word else line ++ " " ++ word
  if !line.isEmpty then lines := lines.push line
  return if lines.isEmpty then #[""] else lines
private def blockLines (b : Block) (scale : Nat) : Nat :=
  match b with
  | .para ss | .subtitle ss => (wrap (plain ss) (1088/(8*scale))).size
  | .item d m ss => (wrap (plain ss) ((1088-d*32-max 40 ((m.length+1)*8*scale))/(8*scale))).size
  | .code _ ls => (ls.map fun s => max 1 ((s.length+67)/68)).foldl (·+·) 0
  | .image _ _ => 0
def bodyHeight (s : Slide) (scale : Nat) : Nat :=
  s.blocks.foldl (fun n b => n + match b with
    | .image _ _ => 252
    | .code _ _ => blockLines b scale * 20 + 28
    | _ => blockLines b scale * (8*scale+8) + 12) 0
private def styled (c : Canvas) (x y scale : Nat) (ss : Array Span) (columns : Nat) : Canvas := Id.run do
  -- Keep styles across wrapping by locating each character's source span.
  let mut chars : Array (Char × Style) := #[]
  for s in ss do
    for ch in s.text.toList do
      if ch != ' ' || (chars.back?.map (·.1)) != some ' ' then
        chars := chars.push (ch,s.style)
  let mut c := c
  let mut pos := 0
  let mut yy := y
  for line in wrap (plain ss) columns do
    while pos < chars.size && chars[pos]!.1 == ' ' do pos := pos+1
    let mut xx := x
    for ch in line.toList do
      let st := (chars[pos]?.getD (' ',Style.plain)).2
      if st == .code then c := rect c xx (yy-2) (8*scale) (8*scale+4) 0x202538
      c := text c xx yy scale (if st == .code then 0xf4ca88 else white) (String.singleton ch) st
      xx := xx+8*scale
      pos := pos+1
    yy := yy+8*scale+8
  return c
private def blit (c : Canvas) (img : Image) (x y maxW maxH : Nat) : Canvas := Id.run do
  if img.width == 0 || img.height == 0 || img.rgba.size != img.width*img.height*4 then return c
  let w := min img.width (min maxW (maxH*img.width/img.height))
  let h := img.height*w/img.width
  let mut c := c
  for yy in [:h] do
    for xx in [:w] do
      let src := ((yy*img.height/h)*img.width + xx*img.width/w)*4
      let dst := ((y+yy)*width+x+xx)*4
      let a := img.rgba[src+3]!.toNat
      for (offset,si) in #[(0,2),(1,1),(2,0)] do
        let v := (img.rgba[src+si]!.toNat*a+c.bytes[dst+offset]!.toNat*(255-a))/255
        c := ⟨c.bytes.set! (dst+offset) v.toUInt8⟩
  return c
/-- Full frame with the launcher's palette, 8x8 type, integer scales and square panels. -/
def render (deck : Deck) (index : Nat) : Canvas := Id.run do
  let s := deck.slides[index]!
  let mut c := background
  c := rect c 64 52 8 72 accent
  let titleScale := max 2 (min 5 (1120 / max 1 ((plain s.title).length*8)))
  let titleLines := wrap (plain s.title) (1120/(8*titleScale))
  for i in [:titleLines.size] do
    c := text c 96 (64+i*(8*titleScale+4)) titleScale white titleLines[i]!
  c := rect c 64 145 1152 2 0x34394d
  let scale := if bodyHeight s 3 ≤ 450 then 3 else 2
  let centered := s.blocks.size ≤ 3 && s.blocks.all (fun b => match b with | .para _ | .subtitle _ => true | _ => false)
  let mut y := if centered then 260 else 182
  for b in s.blocks do
    match b with
    | .subtitle ss | .para ss =>
      let lines := wrap (plain ss) (1088/(8*scale))
      if centered then
        for line in lines do
          c := text c ((width-line.length*8*scale)/2) y scale white line
          y := y+8*scale+8
      else
        c := styled c 96 y scale ss (1088/(8*scale))
        y := y+lines.size*(8*scale+8)
      y := y+12
    | .item d marker ss =>
      let x := 96+d*32
      c := text c x y scale accent marker
      let indent := max 40 ((marker.length+1)*8*scale)
      c := styled c (x+indent) y scale ss ((1088-d*32-indent)/(8*scale))
      y := y+blockLines b scale*(8*scale+8)+12
    | .code _ ls =>
      let h := blockLines b scale*20+16
      c := rect c 88 (y-8) 1104 h 0x202538
      c := rect c 88 (y-8) 4 h accent
      for line in ls do
        let mut rest := line
        repeat
          c := text c 104 y 2 0xf4ca88 (rest.take 68).toString
          y := y+20
          if rest.length ≤ 68 then break
          rest := (rest.drop 68).toString
      y := y+28
    | .image caption path =>
      if let some img := deck.images.find? (·.path == path) then
        let w := min img.width (min 1088 (220*img.width/max 1 img.height))
        c := blit c img ((width-w)/2) y 1088 220
      c := text c 96 (y+228) 2 grey caption
      y := y+252
  c := rect c 64 640 1152 2 0x34394d
  c := rect c 64 640 (1152*(index+1)/max 1 deck.slides.size) 2 accent
  c := text c 64 668 2 grey (plain deck.slides[0]!.title)
  let page := s!"{index+1} / {deck.slides.size}"
  c := text c (1216-page.length*16) 668 2 white page
  return c
end Slides
