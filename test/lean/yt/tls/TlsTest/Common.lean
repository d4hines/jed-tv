import Yt.Crypto.Util
open Yt.Crypto
namespace TlsTest

def line (o : ByteArray) (s : String) : ByteArray := o ++ (s ++ "\n").toUTF8

def check (o : ByteArray) (name : String) (got : ByteArray) (want : String) : ByteArray :=
  if got == ofHex want then line o (name ++ " ok")
  else line o (name ++ " FAIL got " ++ toHex got ++ " want " ++ want)

def checkB (o : ByteArray) (name : String) (ok : Bool) : ByteArray :=
  line o (name ++ (if ok then " ok" else " FAIL"))

/-- Lines of hex. -/
def hexLines (s : String) : Array ByteArray := Id.run do
  let mut out := #[]
  let mut cur := ByteArray.empty
  let mut hi : UInt8 := 0
  let mut odd := false
  for c in s.toUTF8 do
    if c == 10 then
      out := out.push cur
      cur := ByteArray.empty
    else
      let v := hexVal c
      if v != 255 then
        if odd then cur := cur.push ((hi <<< 4) ||| v) else hi := v
        odd := !odd
  if cur.size > 0 then out := out.push cur
  out

end TlsTest
