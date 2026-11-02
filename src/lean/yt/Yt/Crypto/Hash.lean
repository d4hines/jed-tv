import Yt.Crypto.Sha256
import Yt.Crypto.Sha512
/-! The hash functions as a value (for HMAC/HKDF and TLS cipher suites),
plus HMAC (RFC 2104) and HKDF (RFC 5869). -/
namespace Yt.Crypto

inductive HashAlg where
  | sha256 | sha384 | sha512
deriving BEq, Inhabited

namespace HashAlg

def hash : HashAlg → ByteArray → ByteArray
  | .sha256, m => Sha256.hash m
  | .sha384, m => Sha512.hash384 m
  | .sha512, m => Sha512.hash m

def outLen : HashAlg → Nat
  | .sha256 => 32 | .sha384 => 48 | .sha512 => 64

def blockLen : HashAlg → Nat
  | .sha256 => 64 | .sha384 => 128 | .sha512 => 128

end HashAlg

/-- HMAC(key, msg). -/
def hmac (h : HashAlg) (key msg : ByteArray) : ByteArray := Id.run do
  let bl := h.blockLen
  let k := if key.size > bl then h.hash key else key
  let mut ipad := ByteArray.emptyWithCapacity (bl + msg.size)
  let mut opad := ByteArray.emptyWithCapacity (bl + h.outLen)
  for i in [0:bl] do
    let x := getD k i
    ipad := ipad.push (x ^^^ 0x36)
    opad := opad.push (x ^^^ 0x5c)
  h.hash (opad ++ h.hash (ipad ++ msg))

/-- HKDF-Extract(salt, ikm); an empty salt means `outLen` zeros. -/
def hkdfExtract (h : HashAlg) (salt ikm : ByteArray) : ByteArray :=
  hmac h (if salt.size == 0 then zeros h.outLen else salt) ikm

/-- HKDF-Expand(prk, info, len), `len ≤ 255 * outLen`. -/
def hkdfExpand (h : HashAlg) (prk info : ByteArray) (len : Nat) : ByteArray := Id.run do
  let mut out := ByteArray.emptyWithCapacity (len + h.outLen)
  let mut t := ByteArray.empty
  let mut i : Nat := 1
  while out.size < len do
    t := hmac h prk ((t ++ info).push i.toUInt8)
    out := out ++ t
    i := i + 1
  slice out 0 len

end Yt.Crypto
