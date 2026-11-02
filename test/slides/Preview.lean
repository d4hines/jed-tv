import Slides.Parse
import Slides.Render
import SlideAssets
open Slides
def main (args : List String) : IO UInt32 := do
  let source ← if let some file := args[1]? then IO.FS.readFile (System.FilePath.mk file) else pure Slides.source
  let .ok deck := parse source | throw (IO.userError "invalid deck")
  let deck := { deck with images := images }
  let dir := args.headD "preview"
  IO.FS.createDirAll dir
  for i in [:deck.slides.size] do
    let c := render deck i
    let mut rgb := ByteArray.emptyWithCapacity (width*height*3)
    for p in [:width*height] do rgb := rgb.push c.bytes[p*4+2]! |>.push c.bytes[p*4+1]! |>.push c.bytes[p*4]!
    let path := s!"{dir}/slide-{i+1}.ppm"
    IO.FS.writeBinFile path (s!"P6\n{width} {height}\n255\n".toUTF8 ++ rgb)
    IO.println path
  return 0
