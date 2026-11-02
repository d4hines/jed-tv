import Yt.Crypto.Util
/-! Multi-precision modular arithmetic for the public-key code (X25519,
ECDSA P-256/P-384, RSA up to 4096 bits).

A number is an `Array Nat` of 30-bit limbs, least significant first. Limbs
below 2^30 are scalar (unboxed) `Nat`s on both 64-bit hosts and our
32-bit runtime (whose small-`Nat` limit is 2^31), so limb arrays never
allocate per element; arithmetic converts each limb to `UInt32`/`UInt64`
(a 30x30-bit product plus carries fits in 64 bits).

`Mod` holds an odd modulus with its Montgomery constants (R = 2^(30n)).
Multiplication is fused-loop Montgomery (CIOS) with a mask-selected final
subtraction; `addMod`/`subMod`/`cswap` select with masks too, so the field
operations used by X25519 have no data-dependent branches or indices.
Exponentiation (`powPublic`) and the conversions branch on public data
only. -/
namespace Yt.Crypto.Mont

abbrev Limbs := Array Nat

@[inline] def M : UInt64 := 0x3fffffff

@[inline] def lg (a : Limbs) (i : Nat) : UInt64 := (a[i]!).toUInt32.toUInt64
@[inline] def lg32 (a : Limbs) (i : Nat) : UInt32 := (a[i]!).toUInt32

def zero (n : Nat) : Limbs := Array.replicate n 0

/-- Big-endian bytes to `n` limbs (excess high bits are dropped). -/
def ofBytes (b : ByteArray) (n : Nat) : Limbs := Id.run do
  let mut r : Limbs := Array.mkEmpty n
  let mut acc : UInt64 := 0
  let mut bits : UInt64 := 0
  let mut k := b.size
  while r.size < n do
    if bits ≥ 30 || k == 0 then
      r := r.push (acc &&& M).toNat
      acc := acc >>> 30
      bits := if bits ≥ 30 then bits - 30 else 0
    else
      k := k - 1
      acc := acc ||| (b[k]!.toUInt64 <<< bits)
      bits := bits + 8
  r

/-- `len` big-endian bytes (high limbs beyond `len` bytes are dropped). -/
def toBytes (a : Limbs) (len : Nat) : ByteArray := Id.run do
  let mut r := zeros len
  let mut acc : UInt64 := 0
  let mut bits : UInt64 := 0
  let mut i := 0
  let mut k := len
  while k > 0 do
    if bits < 8 && i < a.size then
      acc := acc ||| (lg a i <<< bits)
      bits := bits + 30
      i := i + 1
    else
      k := k - 1
      r := r.set! k (acc &&& 0xff).toUInt8
      acc := acc >>> 8
      bits := if bits ≥ 8 then bits - 8 else 0
  r

/-- Number of limbs for `bits` bits. -/
def limbsFor (bits : Nat) : Nat := (bits + 29) / 30

def isZero (a : Limbs) : Bool := Id.run do
  let mut d : UInt32 := 0
  for x in a do d := d ||| x.toUInt32
  d == 0

/-- `a < b` (same length). -/
def lt (a b : Limbs) : Bool := Id.run do
  let mut borrow : UInt64 := 0
  for i in [0:a.size] do
    let s := lg a i + 0x40000000 - lg b i - borrow
    borrow := (1 : UInt64) - (s >>> 30)
  borrow == 1

def bitLen (a : Limbs) : Nat := Id.run do
  let mut r := 0
  for i in [0:a.size] do
    let x := a[i]!
    if x != 0 then r := 30 * i + Nat.log2 x + 1
  r

@[inline] def testBit (a : Limbs) (i : Nat) : Bool := (a[i / 30]! >>> (i % 30)) &&& 1 == 1

/-- `a - b` and the final borrow (same length). -/
def subB (a b : Limbs) : Limbs × UInt64 := Id.run do
  let mut r := a
  let mut borrow : UInt64 := 0
  for i in [0:a.size] do
    let s := lg a i + 0x40000000 - lg b i - borrow
    r := r.set! i (s &&& M).toNat
    borrow := (1 : UInt64) - (s >>> 30)
  (r, borrow)

/-- Limbwise `mask ? a : b` (mask all ones or zero). -/
def select (mask : UInt32) (a b : Limbs) : Limbs := Id.run do
  let mut r := b
  let nm := ~~~mask
  for i in [0:b.size] do
    r := r.set! i ((lg32 a i &&& mask) ||| (lg32 b i &&& nm)).toNat
  r

/-- An odd modulus and its Montgomery constants. -/
structure Mod where
  m : Limbs
  n : Nat
  /-- -m⁻¹ mod 2^30 -/
  m0inv : UInt64
  /-- R mod m (Montgomery 1) -/
  one : Limbs
  /-- R² mod m -/
  rr : Limbs
  /-- bytes in m -/
  byteLen : Nat
deriving Inhabited

/-- Reduce `t` (n+1 limbs, value < 2m) below `m`, without branching on it. -/
def reduceOnce (md : Mod) (t : Limbs) : Limbs := Id.run do
  let n := md.n
  let mut r : Limbs := Array.mkEmpty n
  let mut borrow : UInt64 := 0
  for i in [0:n] do
    let s := lg t i + 0x40000000 - lg md.m i - borrow
    r := r.push (s &&& M).toNat
    borrow := (1 : UInt64) - (s >>> 30)
  -- t - m is negative iff the top limb (0 or 1) does not cover the borrow
  let neg : UInt64 := ((lg t n - borrow) >>> 63) &&& 1
  -- mask = all ones when t < m (keep t)
  let mask : UInt32 := (0 : UInt32) - neg.toUInt32
  let mut out := r
  let nm := ~~~mask
  for i in [0:n] do
    out := out.set! i ((lg32 t i &&& mask) ||| (lg32 r i &&& nm)).toNat
  out

/-- Inner loop of Montgomery multiplication for limb `bi` of b and
quotient digit `q`: `t := (t + a·bi + q·m) / 2^30`. `bi` and `q` are
`UInt32` so that each product is visibly 32x32→64 bits (one `umull`/`umlal`
on 32-bit ARM rather than a 64x64 multiply). -/
partial def mulInner (a m : @& Limbs) (n : Nat) (bi q : UInt32) (j : Nat) (c : UInt64) (t : Limbs) : Limbs :=
  if j < n then
    let s := lg t j + lg a j * bi.toUInt64 + lg m j * q.toUInt64 + c
    mulInner a m n bi q (j + 1) (s >>> 30) (t.set! (j - 1) (s &&& M).toNat)
  else
    let s := lg t n + c
    (t.set! (n - 1) (s &&& M).toNat).set! n (s >>> 30).toNat

partial def mulOuter (a b m : @& Limbs) (n : Nat) (m0inv : UInt64) (i : Nat) (t : Limbs) : Limbs :=
  if i < n then
    let bi := lg32 b i
    let s := lg t 0 + lg a 0 * bi.toUInt64
    let q := (((s &&& M) * m0inv) &&& M).toUInt32
    let s := s + lg m 0 * q.toUInt64
    mulOuter a b m n m0inv (i + 1) (mulInner a m n bi q 1 (s >>> 30) t)
  else t

/-- Montgomery product a·b·R⁻¹ mod m (a, b < m). -/
def mul (md : Mod) (a b : Limbs) : Limbs :=
  reduceOnce md (mulOuter a b md.m md.n md.m0inv 0 (zero (md.n + 1)))

@[inline] def sqr (md : Mod) (a : Limbs) : Limbs := mul md a a

/-- (a + b) mod m. -/
def addMod (md : Mod) (a b : Limbs) : Limbs := Id.run do
  let n := md.n
  let mut t : Limbs := Array.mkEmpty (n + 1)
  let mut c : UInt64 := 0
  for i in [0:n] do
    let s := lg a i + lg b i + c
    t := t.push (s &&& M).toNat
    c := s >>> 30
  reduceOnce md (t.push c.toNat)

/-- (a - b) mod m. -/
def subMod (md : Mod) (a b : Limbs) : Limbs := Id.run do
  let n := md.n
  let (d, borrow) := subB a b
  -- add m back when it borrowed (mask selects m or 0)
  let mask : UInt32 := (0 : UInt32) - borrow.toUInt32
  let mut r := d
  let mut c : UInt64 := 0
  for i in [0:n] do
    let s := lg d i + (lg32 md.m i &&& mask).toUInt64 + c
    r := r.set! i (s &&& M).toNat
    c := s >>> 30
  r

/-- Swap `a` and `b` when `bit` is 1, without branching on it. -/
def cswap (bit : UInt32) (a b : Limbs) : Limbs × Limbs := Id.run do
  let mask : UInt32 := (0 : UInt32) - bit
  let mut x := a
  let mut y := b
  for i in [0:a.size] do
    let u := lg32 a i
    let v := lg32 b i
    let d := (u ^^^ v) &&& mask
    x := x.set! i (u ^^^ d).toNat
    y := y.set! i (v ^^^ d).toNat
  (x, y)

/-- Doubling mod m by shift and conditional subtraction (public data). -/
def dblMod (md : Mod) (a : Limbs) : Limbs := addMod md a a

/-- Set up a modulus (odd, at least 2 bits) given as limbs. -/
def Mod.ofLimbs (m : Limbs) : Mod := Id.run do
  let n := m.size
  -- Newton iteration for m0⁻¹ mod 2^30
  let m0 := lg m 0
  let mut inv : UInt64 := 1
  for _ in [0:5] do inv := (inv * (2 - m0 * inv)) &&& M
  let m0inv := (0x40000000 - inv) &&& M
  let base : Mod := ⟨m, n, m0inv, zero n, zero n, (bitLen m + 7) / 8⟩
  -- R mod m: double 2^(bits-1) until 2^(30n)
  let bl := bitLen m
  let mut x := zero n
  x := x.set! ((bl - 1) / 30) (1 <<< ((bl - 1) % 30))
  for _ in [0:30 * n - (bl - 1)] do x := dblMod base x
  let one := x
  -- R² mod m = Mont(2^(30n)) by square-and-double over the bits of 30n
  let e := 30 * n
  let mut y := one
  let withOne : Mod := { base with one := one }
  for k in [0:Nat.log2 e + 1] do
    let bit := (e >>> (Nat.log2 e - k)) &&& 1
    y := mul withOne y y
    if bit == 1 then y := dblMod withOne y
  { withOne with rr := y }

/-- Drop high zero limbs (keeping at least one). -/
def trim (l : Limbs) : Limbs := Id.run do
  let mut l := l
  while l.size > 1 && l[l.size - 1]! == 0 do l := l.pop
  l

/-- A modulus from big-endian bytes. -/
def Mod.ofBytes (b : ByteArray) : Mod :=
  Mod.ofLimbs (trim (Mont.ofBytes b (limbsFor (b.size * 8))))

@[inline] def toMont (md : Mod) (a : Limbs) : Limbs := mul md a md.rr
@[inline] def fromMont (md : Mod) (a : Limbs) : Limbs := mul md a ((zero md.n).set! 0 1)

/-- Big-endian bytes (any length, value < m) into Montgomery form; values
≥ m are reduced first by `reduceSlow`. -/
def reduceSlow (md : Mod) (a : Limbs) : Limbs := Id.run do
  -- a has md.n limbs; subtract m while a ≥ m (callers ensure a < 2^k·m for small k)
  let mut x := a
  let mut guard := 0
  while !(lt x md.m) && guard < 64 do
    x := (subB x md.m).1
    guard := guard + 1
  x

/-- `a^e mod m` in Montgomery form, `e` big-endian bytes (public). -/
def powPublic (md : Mod) (a : Limbs) (e : ByteArray) : Limbs := Id.run do
  let mut r := md.one
  for byte in e do
    for k in [0:8] do
      r := sqr md r
      if (byte >>> (7 - k).toUInt8) &&& 1 == 1 then r := mul md r a
  r

/-- a⁻¹ mod p for prime p, as a^(p-2) (Montgomery form in and out). The
exponent is public, so the operation sequence is fixed. -/
def invPrime (md : Mod) (a : Limbs) : Limbs :=
  let pm2 := (subB md.m ((zero md.n).set! 0 2)).1
  powPublic md a (toBytes pm2 md.byteLen)

end Yt.Crypto.Mont
