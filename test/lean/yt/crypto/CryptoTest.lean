import CryptoTest.Hash
import CryptoTest.Aead
import CryptoTest.Pk
open Yt.Crypto
namespace CryptoTest

@[export zr_test_transcript] def transcript (k : Nat) : ByteArray := Id.run do
  let mut o := ByteArray.empty
  o := line o ("crypto transcript " ++ toString k)
  o := hashTests o
  o := aeadTests o
  o := pkTests o (1000 + k)
  o
end CryptoTest
