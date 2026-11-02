import Yt.Crypto.Hash
import CryptoTest.Common
import CryptoTest.VecHmac
import CryptoTest.VecHkdf
open Yt.Crypto
namespace CryptoTest

def hashTests (o : ByteArray) : ByteArray := Id.run do
  let mut o := o
  let abc := ascii "abc"
  let m448 := ascii "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
  let m896 := ascii "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
  let mil := rep "aaaaaaaaaa" 100000
  o := check o "sha256 empty" (Sha256.hash ByteArray.empty) "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  o := check o "sha256 abc" (Sha256.hash abc) "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
  o := check o "sha256 448" (Sha256.hash m448) "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
  o := check o "sha256 896" (Sha256.hash m896) "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1"
  o := check o "sha256 million-a" (Sha256.hash mil) "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
  o := check o "sha384 empty" (Sha512.hash384 ByteArray.empty) "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b"
  o := check o "sha384 abc" (Sha512.hash384 abc) "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"
  o := check o "sha384 896" (Sha512.hash384 m896) "09330c33f71147e83d192fc782cd1b4753111b173b3b05d22fa08086e3b0f712fcc7c71a557e2db966c3e9fa91746039"
  o := check o "sha384 million-a" (Sha512.hash384 mil) "9d0e1809716474cb086e834e310a4a1ced149e9c00f248527972cec5704c2a5b07b8b3dc38ecc4ebae97ddd87f3d8985"
  o := check o "sha512 empty" (Sha512.hash ByteArray.empty) "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
  o := check o "sha512 abc" (Sha512.hash abc) "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
  o := check o "sha512 896" (Sha512.hash m896) "8e959b75dae313da8cf4f72814fc143f8f7779c6eb9f7fa17299aeadb6889018501d289e4900f7e4331b99dec4b5433ac7d329eeb6dd26545e96e55b874be909"
  o := check o "sha512 million-a" (Sha512.hash mil) "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973ebde0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b"
  -- every length 0..300 against a chained digest (padding boundaries)
  let mut acc := ByteArray.empty
  let mut m := ByteArray.empty
  for i in [0:300] do
    acc := Sha256.hash (acc ++ Sha256.hash m ++ Sha512.hash384 m ++ Sha512.hash m)
    m := m.push i.toUInt8
  -- expected value from Python hashlib
  o := check o "sha2 lengths 0..299 chain" acc "133d0c1487b48a0a22fb0b9e9abced1eb01b69dc2c5e688d7715634d8ce33ae7"
  o := suite o "hmac rfc4231" VecHmac.data fun v =>
    let alg := match fieldStr v 2 with | "sha256" => HashAlg.sha256 | "sha384" => .sha384 | _ => .sha512
    hmac alg (fieldHex v 3) (fieldHex v 4) == fieldHex v 5
  o := suite o "hkdf rfc5869" VecHkdf.data fun v =>
    let prk := hkdfExtract .sha256 (fieldHex v 3) (fieldHex v 2)
    prk == fieldHex v 6 && hkdfExpand .sha256 prk (fieldHex v 4) (fieldNat v 5) == fieldHex v 7
  o

end CryptoTest
