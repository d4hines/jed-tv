import Yt.Crypto.Mont
/-! X25519 (RFC 7748): the Montgomery ladder over GF(2^255 - 19) with the
generic Montgomery field arithmetic of `Mont`. The ladder runs a fixed
255 steps with mask-based conditional swaps, and every field operation is
branch-free, so the private scalar does not steer branches or memory
indices (best effort: the compiled C is not audited instruction by
instruction). -/
namespace Yt.Crypto.X25519
open Yt.Crypto.Mont

/-- p = 2^255 - 19. -/
def p : Mod := Mod.ofBytes (ofHex "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed")

/-- (A - 2) / 4 = 121665 in Montgomery form. -/
def a24 : Limbs := toMont p (Mont.ofBytes (ofHex "01db41") p.n)

/-- Reverse a byte array (little-endian ↔ big-endian). -/
def rev (b : ByteArray) : ByteArray := Id.run do
  let mut r := ByteArray.emptyWithCapacity b.size
  for i in [0:b.size] do r := r.push b[b.size - 1 - i]!
  r

partial def ladder (k : ByteArray) (x1 : Limbs) (t : Nat) (swap : UInt32)
    (x2 z2 x3 z3 : Limbs) : Limbs × Limbs :=
  let md := p
  let kt : UInt32 := ((k[t / 8]! >>> (t % 8).toUInt8) &&& 1).toUInt32
  let sw := swap ^^^ kt
  let (x2, x3) := cswap sw x2 x3
  let (z2, z3) := cswap sw z2 z3
  let a := addMod md x2 z2
  let aa := sqr md a
  let b := subMod md x2 z2
  let bb := sqr md b
  let e := subMod md aa bb
  let c := addMod md x3 z3
  let d := subMod md x3 z3
  let da := mul md d a
  let cb := mul md c b
  let x3 := sqr md (addMod md da cb)
  let z3 := mul md x1 (sqr md (subMod md da cb))
  let x2 := mul md aa bb
  let z2 := mul md e (addMod md aa (mul md a24 e))
  if t == 0 then
    let (x2, _) := cswap kt x2 x3
    let (z2, _) := cswap kt z2 z3
    (x2, z2)
  else ladder k x1 (t - 1) kt x2 z2 x3 z3

/-- X25519(k, u): 32-byte little-endian scalar and u-coordinate (other
sizes give 32 zero bytes, which callers must reject like any all-zero
shared secret). -/
def x25519 (scalar u : ByteArray) : ByteArray :=
  if scalar.size != 32 || u.size != 32 then zeros 32 else
  let md := p
  -- clamp
  let k := scalar.set! 0 (scalar[0]! &&& 248) |>.set! 31 ((scalar[31]! &&& 127) ||| 64)
  -- decode u: mask bit 255, reduce mod p (public value)
  let ub := u.set! 31 (u[31]! &&& 127)
  let x1 := toMont md (reduceSlow md (Mont.ofBytes (rev ub) md.n))
  let (x2, z2) := ladder k x1 254 0 md.one (zero md.n) x1 md.one
  let r := fromMont md (mul md x2 (invPrime md z2))
  rev (toBytes r 32)

/-- The public key for a 32-byte private key. -/
def publicKey (scalar : ByteArray) : ByteArray :=
  x25519 scalar (ofHex "0900000000000000000000000000000000000000000000000000000000000000")

end Yt.Crypto.X25519
