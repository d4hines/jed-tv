import Slides.Parse
import Slides.Render
import SlideAssets
import Slides.Navigation
open Slides
@[extern "slides_open"] opaque platformOpen : IO UInt32
@[extern "slides_present"] opaque platformPresent (bytes : @& ByteArray) : IO UInt32
@[extern "slides_poll"] opaque platformPoll : IO UInt32
@[extern "slides_close"] opaque platformClose : IO Unit
private def showSlide (deck : Deck) (i : Nat) : IO Unit := do
  let frame := render deck i
  let status ← platformPresent frame.bytes
  if status != 0 then throw (IO.userError s!"present failed: errno {(0:UInt32)-status}")
  IO.println s!"slides: slide {i+1}/{deck.slides.size}: {plain deck.slides[i]!.title}"
  for note in deck.slides[i]!.notes do IO.println s!"slides: note: {note}"
  (← IO.getStdout).flush
private def run (deck : Deck) (start : Nat) : IO Unit := do
  let mut state : Navigation := { current := min (deck.slides.size-1) start }
  showSlide deck state.current
  while !state.quit do
    let event ← platformPoll
    if event &&& 0x80000000 != 0 && (event >>> 16 &&& 0x7fff) != 0 then
      let next := navigate state (event &&& 0xffff).toNat (← IO.monoMsNow) deck.slides.size
      if next.current != state.current then showSlide deck next.current
      state := next
    else IO.sleep 10
  IO.println "slides: quit"
def main (args : List String) : IO UInt32 := do
  try
    let parsed ← match parse source with
      | .ok d => pure d
      | .error error => throw (IO.userError error)
    let deck := { parsed with images := images }
    let start := args.findSome? fun a => if a.startsWith "slides.start=" then (a.drop 13).toString.toNat? else none
    let status ← platformOpen
    if status != 0 then throw (IO.userError s!"App API open failed: errno {(0:UInt32)-status}")
    try run deck ((start.getD 1)-1) finally platformClose
    return 0
  catch e => IO.eprintln s!"slides: {e}"; return 1
