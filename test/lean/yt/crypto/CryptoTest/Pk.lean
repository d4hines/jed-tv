import Yt.Crypto.X25519
import Yt.Crypto.Ec
import Yt.Crypto.Rsa
import Yt.X509.Der
import CryptoTest.Common
import CryptoTest.VecX25519
import CryptoTest.VecP256
import CryptoTest.VecP384
import CryptoTest.VecRsa2048
import CryptoTest.VecRsa4096
import CryptoTest.VecPss2048
import CryptoTest.VecPss4096
open Yt.Crypto
namespace CryptoTest

def ecdsaVec (c : Ec.Curve) (alg : HashAlg) (v : Vec) : Bool :=
  match Yt.X509.Der.ecdsaSig (fieldHex v 4) with
  | some (r, s) => Ec.verify c (fieldHex v 2) (alg.hash (fieldHex v 3)) r s
  | none => false

def rsaKey (v : Vec) : Rsa.PublicKey := ⟨fieldHex v 2, fieldHex v 3⟩

def pkTests (o : ByteArray) (iterations : Nat) : ByteArray := Id.run do
  let mut o := o
  o := check o "x25519 rfc7748 5.2 #1" (X25519.x25519 (ofHex "a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4")
    (ofHex "e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c")) "c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552"
  o := check o "x25519 rfc7748 5.2 #2" (X25519.x25519 (ofHex "4b66e9d4d1b4673c5ad22691957d6af5c11b6421e0ea01d42ca4169e7918ba0d")
    (ofHex "e5210f12786811d3f4b7959d0538ae2c31dbe7106fc03c3efc4cd549c715a493")) "95cbde9476e8907d7aade45cb4b873f88b595a68799fa152e6f8f7647aac7957"
  let a := ofHex "77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a"
  let b := ofHex "5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb"
  o := check o "x25519 rfc7748 6.1 alice" (X25519.publicKey a) "8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a"
  o := check o "x25519 rfc7748 6.1 bob" (X25519.publicKey b) "de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f"
  o := check o "x25519 rfc7748 6.1 shared" (X25519.x25519 a (X25519.publicKey b)) "4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742"
  -- the iterated test: k, u := X25519(k, u), k
  let mut k := ofHex "0900000000000000000000000000000000000000000000000000000000000000"
  let mut u := k
  for i in [0:iterations] do
    let r := X25519.x25519 k u
    u := k
    k := r
    if i == 0 then o := check o "x25519 rfc7748 iterated 1" k "422c8e7a6227d7bca1350b3e2bb7279f7897b87bb6854b783c60e80311ae3079"
  if iterations == 1000 then
    o := check o "x25519 rfc7748 iterated 1000" k "684cf59ba83309552800ef566f2f4d3c1c3887c49360e3875f2eb94d99532c51"
  o := suite o "x25519 wycheproof" VecX25519.data fun v =>
    X25519.x25519 (fieldHex v 2) (fieldHex v 3) == fieldHex v 4
  o := suite o "ecdsa p256 sha256 wycheproof" VecP256.data (ecdsaVec Ec.p256 .sha256)
  o := suite o "ecdsa p384 sha384 wycheproof" VecP384.data (ecdsaVec Ec.p384 .sha384)
  o := suite o "rsa pkcs1 2048 sha256 wycheproof" VecRsa2048.data fun v =>
    Rsa.verifyPkcs1 (rsaKey v) .sha256 (Sha256.hash (fieldHex v 4)) (fieldHex v 5)
  o := suite o "rsa pkcs1 4096 sha384 wycheproof" VecRsa4096.data fun v =>
    Rsa.verifyPkcs1 (rsaKey v) .sha384 (Sha512.hash384 (fieldHex v 4)) (fieldHex v 5)
  o := suite o "rsa pss 2048 sha256 wycheproof" VecPss2048.data fun v =>
    Rsa.verifyPss (rsaKey v) .sha256 (Sha256.hash (fieldHex v 4)) (fieldHex v 5)
  o := suite o "rsa pss 4096 sha512 wycheproof" VecPss4096.data fun v =>
    Rsa.verifyPss (rsaKey v) .sha512 (Sha512.hash (fieldHex v 4)) (fieldHex v 5)
  o

end CryptoTest
