import H264Fixture
import H264Test.Run

/-! The H.264 transcript: decode the embedded clips and print a hash of each
output frame's planes, so the host (upstream Lean) and our runtime on 32-bit
ARM can be compared byte for byte. -/

namespace H264Test
open Yt.H264

def line (o : ByteArray) (s : String) : ByteArray := o ++ (s ++ "\n").toUTF8

def runClip (o : ByteArray) (name : String) (mp4 : ByteArray) : ByteArray :=
  let each (o : ByteArray) (k : Nat) (f : OutFrame) : ByteArray :=
    line o ("  " ++ name ++ " " ++ toString k ++ " " ++ toString f.width ++ "x" ++ toString f.height ++
      " y=" ++ hex32 (fnv fnvInit f.y) ++ " cb=" ++ hex32 (fnv fnvInit f.cb) ++ " cr=" ++ hex32 (fnv fnvInit f.cr))
  match decodeMp4 mp4 true true o each with
  | .ok (o, n) => line o (name ++ ": " ++ toString n ++ " frames")
  | .error e => line o (name ++ ": error: " ++ e)

@[export zr_test_transcript]
def transcript (_ : Nat) : ByteArray := Id.run do
  let mut o := ByteArray.empty
  for (name, hex) in H264Fixture.clips do
    o := runClip o name (unhex hex)
  return o

end H264Test
