import Slides.Parse
import Slides.Render
import Slides.Navigation
import SlideAssets
open Slides
private def check (b : Bool) (msg : String) : IO Unit :=
  unless b do throw (IO.userError msg)
private def good (s : String) : IO Deck :=
  match parse s with | .ok d => pure d | .error e => throw (IO.userError e)
def main (args : List String) : IO UInt32 := do
  check (font[32]! == #[0,0,0,0,0,0,0,0]) "space glyph must be blank"
  check (font[65]![0]! == 12) "font table must retain ASCII indexing"
  let deck ← good source
  for s in deck.slides do
    check (bodyHeight s 2 ≤ 450) s!"layout overflows: {plain s.title}"
    check ((plain s.title).length ≤ 140) "title exceeds two lines"
  if args.contains "--validate" then
    IO.println s!"PASS: {deck.slides.size} slides parsed and fit the screen"
    return 0
  check (deck.slides.size == 12) "preserve all 12 slides"
  check (plain deck.slides[0]!.title == "dande-tv") "deck title"
  for s in ["", "---", "# a\n---", "# a\n# b", "# a\n### b", "# a\n```\nmissing", "# a\n**open", "# a\n   - odd", "# a\n        - deep"] do
    check (match parse s with | .error _ => true | _ => false) s!"accepted malformed input: {s}"
  let numeric ← good "# Numbers\n\n123"
  check (numeric.slides[0]!.blocks == #[.para #[⟨"123",.plain⟩]]) "bare number is a paragraph"
  let windows ← good "# CRLF\r\n\r\nText\r\n"
  check (plain windows.slides[0]!.title == "CRLF") "CRLF input"
  let rich ← good "# rich\n\nPlain **bold** *italic* ***both*** `code` and \\*literal\\*.\n\n- first\n  continued\n  - nested\n\nAfter list.\n\n```rust\nx\ty\n```\n\n![image](test.png)\nNote: secret"
  check (rich.slides[0]!.notes == #["secret"]) "speaker notes"
  check (rich.slides[0]!.blocks.size == 6) "paragraph/list/code/image boundaries"
  check ((← good "# x\n\n``a ` tick``").slides.size == 1) "double backtick"
  check (wrap "hello world xxxxxxxxxx" 5 == #["hello", "world", "xxxxx", "xxxxx"]) "word wrapping"
  let a := navigate {} 106 2000 12
  check (a.current == 1) "next"
  check ((navigate a 105 2000 12).current == 0) "previous"
  check ((navigate {} 105 2000 12).current == 0) "previous boundary"
  check ((navigate {} 402 2000 12).current == 1) "Ch+ forward"
  check ((navigate (navigate {} 402 2000 12) 403 2000 12).current == 0) "Ch- back"
  let n := navigate (navigate {} 2 2000 12) 3 2300 12
  check (n.current == 11) "two digit jump"
  check ((navigate n 3 5000 12).current == 1) "digit timeout"
  for key in [1, 102, 158, 174, 223] do check ((navigate n key 5000 12).quit) "Back/Home exit"
  let c := render deck 0
  check (c.bytes.size == width*height*4) "pixel contract"
  let transparent : Image := ⟨"test.png",1,1,ByteArray.mk #[255,0,0,128]⟩
  let imageDeck : Deck := ⟨#[{title := #[⟨"IMAGE",.plain⟩],blocks := #[.image "caption" "test.png"]}],#[transparent]⟩
  let frame := render imageDeck 0
  let p := (182*width+639)*4
  check (frame.bytes[p+2]! > frame.bytes[p]!) "image alpha compositing"
  IO.println "PASS: 12-slide deck, parser errors/styles/code/images, layout bounds, pixel contract, remote navigation"
  return 0
