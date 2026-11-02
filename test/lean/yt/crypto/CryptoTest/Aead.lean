import Yt.Crypto.Gcm
import Yt.Crypto.ChaCha
import Yt.Crypto.Sha256
import CryptoTest.Common
import CryptoTest.VecAesGcm
import CryptoTest.VecChaCha
open Yt.Crypto
namespace CryptoTest

/-- Wycheproof AEAD vector: decrypt must succeed exactly for valid ones and
reproduce the message; encrypt must reproduce ct||tag. -/
def aeadVec (enc : ByteArray → ByteArray → ByteArray → ByteArray → ByteArray)
    (dec : ByteArray → ByteArray → ByteArray → ByteArray → Option ByteArray) (v : Vec) : Bool :=
  let key := fieldHex v 2
  let iv := fieldHex v 3
  let aad := fieldHex v 4
  let msg := fieldHex v 5
  let ct := fieldHex v 6 ++ fieldHex v 7
  match dec key iv aad ct with
  | some m => m == msg && enc key iv aad msg == ct
  | none => false

def aeadTests (o : ByteArray) : ByteArray := Id.run do
  let mut o := o
  o := check o "aes sbox" (slice Aes.sbox 0 4 ++ slice Aes.sbox 0x53 1 ++ slice Aes.sbox 252 4) "637c777bedb054bb16"
  o := check o "aes128 fips197" (Aes.encryptBlock (Aes.expand (ofHex "000102030405060708090a0b0c0d0e0f")) (ofHex "00112233445566778899aabbccddeeff"))
    "69c4e0d86a7b0430d8cdb78070b4c55a"
  o := check o "aes256 fips197" (Aes.encryptBlock (Aes.expand (ofHex "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")) (ofHex "00112233445566778899aabbccddeeff"))
    "8ea2b7ca516745bfeafc49904b496089"
  o := suite o "aes-gcm wycheproof" VecAesGcm.data
    (aeadVec (fun k n a p => Gcm.encrypt (Gcm.init k) n a p) (fun k n a c => Gcm.decrypt (Gcm.init k) n a c))
  o := check o "chacha20 block rfc8439" (ChaCha.block (ofHex "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f") (ofHex "000000090000004a00000000") 1).bytes
    "10f1e7e4d13b5915500fdd1fa32071c4c7d1f4c733c068030422aa9ac3d46c4ed2826446079faa0914c2d705d98b02a2b5129cd1de164eb9cbd083e8a2503c4e"
  o := check o "poly1305 rfc8439" (ChaCha.poly1305 (ofHex "85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b") (ascii "Cryptographic Forum Research Group"))
    "a8061dc1305136c6c22b8baf0c0127a9"
  o := suite o "chacha20-poly1305 wycheproof" VecChaCha.data (aeadVec ChaCha.encrypt ChaCha.decrypt)
  -- lengths 0..200 round trip and chained digest (partial blocks)
  let gk := Gcm.init (ofHex "feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308")
  let ck := ofHex "808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f"
  let nonce := ofHex "cafebabefacedbaddecaf888"
  let mut acc := ByteArray.empty
  let mut m := ByteArray.empty
  let mut rt := true
  for i in [0:200] do
    let g := Gcm.encrypt gk nonce (slice m 0 (i % 37)) m
    let c := ChaCha.encrypt ck nonce (slice m 0 (i % 37)) m
    rt := rt && Gcm.decrypt gk nonce (slice m 0 (i % 37)) g == some m && ChaCha.decrypt ck nonce (slice m 0 (i % 37)) c == some m
    acc := Sha256.hash (acc ++ g ++ c)
    m := m.push (i * 7).toUInt8
  o := checkB o "aead lengths 0..199 round trip" rt
  -- expected value from Python `cryptography` (AESGCM, ChaCha20Poly1305)
  o := check o "aead lengths 0..199 chain" acc "68128f7157db4e05127b8385b6c3c1919e4db51649a76f9447b828781653c0c8"
  o

end CryptoTest
