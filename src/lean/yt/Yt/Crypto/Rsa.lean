import Yt.Crypto.Mont
import Yt.Crypto.Hash
/-! RSA signature verification (RFC 8017): RSASSA-PKCS1-v1_5 and
RSASSA-PSS with MGF1 (salt length = hash length, as TLS 1.3 requires).
Only public data is involved. -/
namespace Yt.Crypto.Rsa
open Yt.Crypto.Mont

/-- A public key: modulus and exponent, big-endian magnitudes. -/
structure PublicKey where
  n : ByteArray
  e : ByteArray
deriving Inhabited

/-- Strip leading zero bytes. -/
def strip (b : ByteArray) : ByteArray := Id.run do
  let mut i := 0
  while i < b.size && b[i]! == 0 do i := i + 1
  slice b i (b.size - i)

def PublicKey.bits (k : PublicKey) : Nat :=
  let n := strip k.n
  if n.size == 0 then 0 else 8 * (n.size - 1) + Nat.log2 n[0]!.toNat + 1

/-- s^e mod n as `k` bytes (k = modulus length), or `none` if the
signature is not exactly k bytes or not below n. -/
def encrypt (key : PublicKey) (sig : ByteArray) : Option ByteArray := Id.run do
  let nb := strip key.n
  let k := nb.size
  if k == 0 || nb[k-1]! &&& 1 == 0 || sig.size != k then return none
  let md := Mod.ofBytes nb
  let s := Mont.ofBytes sig md.n
  if !(lt s md.m) then return none
  let e := strip key.e
  if e.size == 0 then return none
  let r := fromMont md (powPublic md (toMont md s) e)
  return some (toBytes r k)

/-- The DER DigestInfo prefixes for SHA-256/384/512. -/
def digestInfoPrefix : HashAlg → ByteArray
  | .sha256 => ofHex "3031300d060960864801650304020105000420"
  | .sha384 => ofHex "3041300d060960864801650304020205000430"
  | .sha512 => ofHex "3051300d060960864801650304020305000440"

/-- RSASSA-PKCS1-v1_5 verification of `sig` over a message with hash `h`
(computed with `alg`). -/
def verifyPkcs1 (key : PublicKey) (alg : HashAlg) (h sig : ByteArray) : Bool :=
  match encrypt key sig with
  | none => false
  | some em => Id.run do
    let t := digestInfoPrefix alg ++ h
    let k := em.size
    if k < t.size + 11 then return false
    let mut want := ByteArray.emptyWithCapacity k
    want := want.push 0 |>.push 1
    for _ in [0:k - t.size - 3] do want := want.push 0xff
    want := want.push 0 ++ t
    return want == em

/-- MGF1 mask of `len` bytes. -/
def mgf1 (alg : HashAlg) (seed : ByteArray) (len : Nat) : ByteArray := Id.run do
  let mut out := ByteArray.emptyWithCapacity (len + alg.outLen)
  let mut c : UInt32 := 0
  while out.size < len do
    out := out ++ alg.hash (pushU32BE seed c)
    c := c + 1
  slice out 0 len

/-- RSASSA-PSS verification (MGF1 with the same hash, salt length
`saltLen`, default the hash length). -/
def verifyPss (key : PublicKey) (alg : HashAlg) (mHash sig : ByteArray) (saltLen : Nat := alg.outLen) : Bool :=
  match encrypt key sig with
  | none => false
  | some m => Id.run do
    let modBits := key.bits
    let emBits := modBits - 1
    let emLen := (emBits + 7) / 8
    -- m has k bytes; EM is its last emLen bytes, the rest must be zero
    let k := m.size
    for i in [0:k - emLen] do
      if m[i]! != 0 then return false
    let em := slice m (k - emLen) emLen
    let hLen := alg.outLen
    if emLen < hLen + saltLen + 2 then return false
    if em[emLen - 1]! != 0xbc then return false
    let dbLen := emLen - hLen - 1
    let masked := slice em 0 dbLen
    let hh := slice em dbLen hLen
    let topBits := 8 * emLen - emBits
    let topMask : UInt8 := (0xff : UInt8) >>> topBits.toUInt8
    if masked[0]! &&& ~~~topMask != 0 then return false
    let db := xorBytes masked (mgf1 alg hh dbLen)
    let db := db.set! 0 (db[0]! &&& topMask)
    let psLen := emLen - hLen - saltLen - 2
    for i in [0:psLen] do
      if db[i]! != 0 then return false
    if db[psLen]! != 1 then return false
    let salt := slice db (dbLen - saltLen) saltLen
    let h' := alg.hash (zeros 8 ++ mHash ++ salt)
    return h' == hh

end Yt.Crypto.Rsa
