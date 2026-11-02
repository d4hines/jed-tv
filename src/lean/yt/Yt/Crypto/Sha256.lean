import Yt.Crypto.Util
/-! SHA-256 (FIPS 180-4). One-shot over a `ByteArray`. The 64 rounds run as
a self tail-recursive function whose state (8 working words plus a 16-word
rolling message schedule) lives in unboxed `UInt32` parameters, so the
compiled loop allocates nothing; one small object per block holds the
chaining value. -/
namespace Yt.Crypto.Sha256

def K : Array UInt32 := #[
  0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
  0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
  0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
  0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
  0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
  0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
  0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
  0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2]

/-- The chaining value. -/
structure H where
  a : UInt32
  b : UInt32
  c : UInt32
  d : UInt32
  e : UInt32
  f : UInt32
  g : UInt32
  h : UInt32
deriving Inhabited

def iv : H := ⟨0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19⟩

@[inline] def rotr (x : UInt32) (n : UInt32) : UInt32 := (x >>> n) ||| (x <<< (32 - n))
@[inline] def bs0 (x : UInt32) : UInt32 := rotr x 2 ^^^ rotr x 13 ^^^ rotr x 22
@[inline] def bs1 (x : UInt32) : UInt32 := rotr x 6 ^^^ rotr x 11 ^^^ rotr x 25
@[inline] def ss0 (x : UInt32) : UInt32 := rotr x 7 ^^^ rotr x 18 ^^^ (x >>> 3)
@[inline] def ss1 (x : UInt32) : UInt32 := rotr x 17 ^^^ rotr x 19 ^^^ (x >>> 10)

partial def rounds (i : Nat) (a b c d e f g h : UInt32)
    (w0 w1 w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14 w15 : UInt32) (s : H) : H :=
  if i < 64 then
    let t1 := h + bs1 e + ((e &&& f) ^^^ (~~~e &&& g)) + K[i]! + w0
    let t2 := bs0 a + ((a &&& b) ^^^ (a &&& c) ^^^ (b &&& c))
    let nw := ss1 w14 + w9 + ss0 w1 + w0
    rounds (i + 1) (t1 + t2) a b c (d + t1) e f g w1 w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14 w15 nw s
  else
    ⟨s.a + a, s.b + b, s.c + c, s.d + d, s.e + e, s.f + f, s.g + g, s.h + h⟩

/-- Compress the 64-byte block at `o`. -/
@[inline] def block (s : H) (m : ByteArray) (o : Nat) : H :=
  rounds 0 s.a s.b s.c s.d s.e s.f s.g s.h
    (getU32BE m o) (getU32BE m (o+4)) (getU32BE m (o+8)) (getU32BE m (o+12))
    (getU32BE m (o+16)) (getU32BE m (o+20)) (getU32BE m (o+24)) (getU32BE m (o+28))
    (getU32BE m (o+32)) (getU32BE m (o+36)) (getU32BE m (o+40)) (getU32BE m (o+44))
    (getU32BE m (o+48)) (getU32BE m (o+52)) (getU32BE m (o+56)) (getU32BE m (o+60)) s

/-- Compress the whole blocks of `m` from `o`; returns the state. -/
def blocks (s : H) (m : ByteArray) (o : Nat) : H := Id.run do
  let mut s := s
  let mut o := o
  while o + 64 ≤ m.size do
    s := block s m o
    o := o + 64
  s

/-- Padding for a message of `len` bytes whose last `len % 64` bytes are
`tail`: one or two final blocks. -/
def finalBlocks (tail : ByteArray) (len : Nat) : ByteArray := Id.run do
  let mut t := tail.push 0x80
  while t.size % 64 != 56 do t := t.push 0
  pushU64BE t (len * 8).toUInt64

def digestOf (s : H) : ByteArray :=
  let r := ByteArray.emptyWithCapacity 32
  let r := pushU32BE (pushU32BE (pushU32BE (pushU32BE r s.a) s.b) s.c) s.d
  pushU32BE (pushU32BE (pushU32BE (pushU32BE r s.e) s.f) s.g) s.h

/-- SHA-256 of `m`. -/
def hash (m : ByteArray) : ByteArray :=
  let full := m.size / 64 * 64
  let s := blocks iv m 0
  let s := blocks s (finalBlocks (slice m full (m.size - full)) m.size) 0
  digestOf s

end Yt.Crypto.Sha256
