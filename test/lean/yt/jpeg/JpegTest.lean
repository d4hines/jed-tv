import Yt.Jpeg.Decode
import JpegTest.Fixtures

/-! The JPEG decoder against libjpeg-turbo's djpeg (ref.py): every test
image must decode to exactly the reference RGB (otherwise the number of
differing bytes and the largest difference are reported), and the
YouTube thumbnails' 288x162 tiles must equal browse.rs's scaling of the
reference. -/
namespace JpegTest
open Yt.Jpeg Yt.Fmt

def n (x : Nat) : String := toString x
def b64 (s : String) : ByteArray := (base64Decode s.toUTF8).getD ByteArray.empty

def compare (got want : ByteArray) : String := Id.run do
  if got == want then return "identical"
  let mut diff := 0
  let mut maxd := 0
  for i in [0:min got.size want.size] do
    let a := (byteAt got i).toNat
    let b := (byteAt want i).toNat
    if a != b then
      diff := diff + 1
      maxd := max maxd (if a > b then a - b else b - a)
  return "DIFFERS: " ++ n diff ++ " of " ++ n want.size ++ " bytes, max difference " ++ n maxd
    ++ (if got.size != want.size then " (sizes " ++ n got.size ++ " vs " ++ n want.size ++ ")" else "")

def case (name : String) (w h : Nat) (jpg rgb scaled : String) (k : Nat) : String :=
  let file := b64 jpg
  match decode (file.extract k file.size) with
  | .error e => "jpeg " ++ name ++ ": FAIL " ++ e ++ "\n"
  | .ok img =>
    let dims := if img.width == w && img.height == h then "" else " SIZE " ++ n img.width ++ "x" ++ n img.height
    let tile := if scaled.isEmpty then "" else
      "; 288x162 tile " ++ compare (scaleNearest img 288 162) (b64 scaled)
    let note := if name == "trunc-prog60" then
        " (known: for incompletely refined progressive data libjpeg also smooths across blocks; not implemented)"
      else ""
    "jpeg " ++ name ++ " " ++ n w ++ "x" ++ n h ++ " (" ++ n file.size ++ " bytes): " ++ dims
      ++ compare img.rgb (b64 rgb) ++ tile ++ note ++ "\n"

def errors : String := Id.run do
  let mut o := ""
  for (name, bytes) in #[("empty", ByteArray.empty), ("not a JPEG", "GIF89a".toUTF8),
      ("SOI only", ⟨#[0xFF, 0xD8]⟩), ("arithmetic (SOF9)", ⟨#[0xFF, 0xD8, 0xFF, 0xC9, 0, 11, 8, 0, 1, 0, 1, 1, 1, 0x11, 0]⟩)] do
    o := o ++ "jpeg error " ++ name ++ ": " ++ (match decode bytes with
      | .error e => "rejected (" ++ e ++ ")" | .ok _ => "FAIL accepted") ++ "\n"
  return o

@[export zr_test_transcript] def transcript (k : Nat) : ByteArray :=
  let o := Fixtures.all.foldl (fun o (name, w, h, jpg, rgb, scaled) => o ++ case name w h jpg rgb scaled k) ""
  (o ++ errors).toUTF8

end JpegTest
