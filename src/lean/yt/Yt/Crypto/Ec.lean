import Yt.Crypto.Mont
/-! ECDSA signature verification on NIST P-256 and P-384 (FIPS 186-4),
Jacobian coordinates with a = -3 and Montgomery-form field elements.
Verification handles only public data, so the scalar multiplication
(Shamir's trick over u1·G + u2·Q) branches freely on scalar bits. -/
namespace Yt.Crypto.Ec
open Yt.Crypto.Mont

structure Curve where
  p : Mod
  n : Mod
  /-- b in Montgomery form -/
  b : Limbs
  gx : Limbs
  gy : Limbs
  /-- bytes per coordinate -/
  size : Nat
deriving Inhabited

def mkCurve (p n b gx gy : String) (size : Nat) : Curve :=
  let pm := Mod.ofBytes (ofHex p)
  let nm := Mod.ofBytes (ofHex n)
  let tm (h : String) := toMont pm (Mont.ofBytes (ofHex h) pm.n)
  ⟨pm, nm, tm b, tm gx, tm gy, size⟩

def p256 : Curve := mkCurve
  "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff"
  "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551"
  "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b"
  "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296"
  "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5" 32

def p384 : Curve := mkCurve
  "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff"
  "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973"
  "b3312fa7e23ee7e4988e056be3f82d19181d9c6efe8141120314088f5013875ac656398d8a2ed19d2a85c8edd3ec2aef"
  "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7"
  "3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f" 48

/-- A point in Jacobian coordinates (Montgomery form); z = 0 is infinity. -/
structure Pt where
  x : Limbs
  y : Limbs
  z : Limbs
deriving Inhabited

def Pt.isInf (q : Pt) : Bool := isZero q.z

def dbl (c : Curve) (q : Pt) : Pt :=
  let f := c.p
  let delta := sqr f q.z
  let gamma := sqr f q.y
  let beta := mul f q.x gamma
  let t := mul f (subMod f q.x delta) (addMod f q.x delta)
  let alpha := addMod f t (addMod f t t)
  let beta4 := dblMod f (dblMod f beta)
  let x3 := subMod f (sqr f alpha) (dblMod f beta4)
  let z3 := subMod f (subMod f (sqr f (addMod f q.y q.z)) gamma) delta
  let g2 := sqr f gamma
  let g8 := dblMod f (dblMod f (dblMod f g2))
  let y3 := subMod f (mul f alpha (subMod f beta4 x3)) g8
  ⟨x3, y3, z3⟩

def add (c : Curve) (p q : Pt) : Pt :=
  if p.isInf then q else if q.isInf then p else
  let f := c.p
  let z1z1 := sqr f p.z
  let z2z2 := sqr f q.z
  let u1 := mul f p.x z2z2
  let u2 := mul f q.x z1z1
  let s1 := mul f (mul f p.y q.z) z2z2
  let s2 := mul f (mul f q.y p.z) z1z1
  let h := subMod f u2 u1
  let r := dblMod f (subMod f s2 s1)
  if isZero h then
    if isZero r then dbl c p else ⟨f.one, f.one, zero f.n⟩
  else
    let i := sqr f (dblMod f h)
    let j := mul f h i
    let v := mul f u1 i
    let x3 := subMod f (subMod f (sqr f r) j) (dblMod f v)
    let y3 := subMod f (mul f r (subMod f v x3)) (dblMod f (mul f s1 j))
    let z3 := mul f (subMod f (subMod f (sqr f (addMod f p.z q.z)) z1z1) z2z2) h
    ⟨x3, y3, z3⟩

/-- Is the affine point (x, y), Montgomery form, on the curve? -/
def onCurve (c : Curve) (x y : Limbs) : Bool :=
  let f := c.p
  let x3 := mul f (sqr f x) x
  let rhs := addMod f (subMod f x3 (addMod f x (addMod f x x))) c.b
  sqr f y == rhs

/-- u1·G + u2·Q (plain scalars, limbs of the order's size). -/
def shamir (c : Curve) (u1 u2 : Limbs) (q : Pt) : Pt := Id.run do
  let g : Pt := ⟨c.gx, c.gy, c.p.one⟩
  let gq := add c g q
  let top := max (bitLen u1) (bitLen u2)
  let mut r : Pt := ⟨c.p.one, c.p.one, zero c.p.n⟩
  for k in [0:top] do
    let i := top - 1 - k
    r := dbl c r
    let b1 := testBit u1 i
    let b2 := testBit u2 i
    if b1 && b2 then r := add c r gq
    else if b1 then r := add c r g
    else if b2 then r := add c r q
  r

/-- Verify an ECDSA signature `(r, s)` (big-endian integers) over the
message hash `h` with the uncompressed public key `04 ‖ X ‖ Y`. -/
def verify (c : Curve) (pub : ByteArray) (h r s : ByteArray) : Bool := Id.run do
  let sz := c.size
  if pub.size != 2 * sz + 1 || pub[0]! != 4 then return false
  let f := c.p
  let nm := c.n
  -- coordinates must be below p
  let xb := slice pub 1 sz
  let yb := slice pub (1 + sz) sz
  let xl := Mont.ofBytes xb f.n
  let yl := Mont.ofBytes yb f.n
  if !(lt xl f.m) || !(lt yl f.m) then return false
  let x := toMont f xl
  let y := toMont f yl
  if !(onCurve c x y) then return false
  -- 0 < r, s < n
  if r.size > sz || s.size > sz then return false
  let rl := Mont.ofBytes r nm.n
  let sl := Mont.ofBytes s nm.n
  if isZero rl || isZero sl || !(lt rl nm.m) || !(lt sl nm.m) then return false
  -- e: the leftmost bits of the hash, reduced mod n
  let hb := if h.size > sz then slice h 0 sz else h
  let e := reduceSlow nm (Mont.ofBytes hb nm.n)
  let w := invPrime nm (toMont nm sl)
  let u1 := fromMont nm (mul nm (toMont nm e) w)
  let u2 := fromMont nm (mul nm (toMont nm rl) w)
  let pt := shamir c u1 u2 ⟨x, y, f.one⟩
  if pt.isInf then return false
  let zi := invPrime f pt.z
  let ax := fromMont f (mul f pt.x (sqr f zi))
  -- x mod n (x < p < 2n for these curves)
  let v := reduceSlow nm (Mont.ofBytes (toBytes ax sz) nm.n)
  return v == rl

end Yt.Crypto.Ec
