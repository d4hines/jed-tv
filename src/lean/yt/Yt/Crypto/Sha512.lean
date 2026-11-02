import Yt.Crypto.Util
/-! SHA-512 and SHA-384 (FIPS 180-4), same structure as `Sha256`: the 80
rounds are a self tail-recursive function over unboxed `UInt64`
parameters (pairs of registers on 32-bit ARM). -/
namespace Yt.Crypto.Sha512

def K : Array UInt64 := #[
  0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc, 0x3956c25bf348b538,
  0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 0xd807aa98a3030242, 0x12835b0145706fbe,
  0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2, 0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235,
  0xc19bf174cf692694, 0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
  0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5, 0x983e5152ee66dfab,
  0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725,
  0x06ca6351e003826f, 0x142929670a0e6e70, 0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed,
  0x53380d139d95b3df, 0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
  0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218,
  0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8, 0x19a4c116b8d2d0c8, 0x1e376c085141ab53,
  0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373,
  0x682e6ff3d6b2b8a3, 0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
  0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b, 0xca273eceea26619c,
  0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba, 0x0a637dc5a2c898a6,
  0x113f9804bef90dae, 0x1b710b35131c471b, 0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc,
  0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817]

structure H where
  a : UInt64
  b : UInt64
  c : UInt64
  d : UInt64
  e : UInt64
  f : UInt64
  g : UInt64
  h : UInt64
deriving Inhabited

def iv512 : H := ⟨0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
  0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179⟩
def iv384 : H := ⟨0xcbbb9d5dc1059ed8, 0x629a292a367cd507, 0x9159015a3070dd17, 0x152fecd8f70e5939,
  0x67332667ffc00b31, 0x8eb44a8768581511, 0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4⟩

@[inline] def rotr (x : UInt64) (n : UInt64) : UInt64 := (x >>> n) ||| (x <<< (64 - n))
@[inline] def bs0 (x : UInt64) : UInt64 := rotr x 28 ^^^ rotr x 34 ^^^ rotr x 39
@[inline] def bs1 (x : UInt64) : UInt64 := rotr x 14 ^^^ rotr x 18 ^^^ rotr x 41
@[inline] def ss0 (x : UInt64) : UInt64 := rotr x 1 ^^^ rotr x 8 ^^^ (x >>> 7)
@[inline] def ss1 (x : UInt64) : UInt64 := rotr x 19 ^^^ rotr x 61 ^^^ (x >>> 6)

partial def rounds (i : Nat) (a b c d e f g h : UInt64)
    (w0 w1 w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14 w15 : UInt64) (s : H) : H :=
  if i < 80 then
    let t1 := h + bs1 e + ((e &&& f) ^^^ (~~~e &&& g)) + K[i]! + w0
    let t2 := bs0 a + ((a &&& b) ^^^ (a &&& c) ^^^ (b &&& c))
    let nw := ss1 w14 + w9 + ss0 w1 + w0
    rounds (i + 1) (t1 + t2) a b c (d + t1) e f g w1 w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13 w14 w15 nw s
  else
    ⟨s.a + a, s.b + b, s.c + c, s.d + d, s.e + e, s.f + f, s.g + g, s.h + h⟩

@[inline] def block (s : H) (m : ByteArray) (o : Nat) : H :=
  rounds 0 s.a s.b s.c s.d s.e s.f s.g s.h
    (getU64BE m o) (getU64BE m (o+8)) (getU64BE m (o+16)) (getU64BE m (o+24))
    (getU64BE m (o+32)) (getU64BE m (o+40)) (getU64BE m (o+48)) (getU64BE m (o+56))
    (getU64BE m (o+64)) (getU64BE m (o+72)) (getU64BE m (o+80)) (getU64BE m (o+88))
    (getU64BE m (o+96)) (getU64BE m (o+104)) (getU64BE m (o+112)) (getU64BE m (o+120)) s

def blocks (s : H) (m : ByteArray) (o : Nat) : H := Id.run do
  let mut s := s
  let mut o := o
  while o + 128 ≤ m.size do
    s := block s m o
    o := o + 128
  s

def finalBlocks (tail : ByteArray) (len : Nat) : ByteArray := Id.run do
  let mut t := tail.push 0x80
  while t.size % 128 != 112 do t := t.push 0
  pushU64BE (pushU64BE t (len >>> 61).toUInt64) (len * 8).toUInt64

def digestOf (s : H) (words : Nat) : ByteArray := Id.run do
  let r := ByteArray.emptyWithCapacity 64
  let r := pushU64BE (pushU64BE (pushU64BE (pushU64BE r s.a) s.b) s.c) s.d
  let r := pushU64BE (pushU64BE r s.e) s.f
  if words == 6 then r else pushU64BE (pushU64BE r s.g) s.h

def run (iv : H) (words : Nat) (m : ByteArray) : ByteArray :=
  let full := m.size / 128 * 128
  let s := blocks iv m 0
  let s := blocks s (finalBlocks (slice m full (m.size - full)) m.size) 0
  digestOf s words

/-- SHA-512 of `m`. -/
def hash (m : ByteArray) : ByteArray := run iv512 8 m
/-- SHA-384 of `m`. -/
def hash384 (m : ByteArray) : ByteArray := run iv384 6 m

end Yt.Crypto.Sha512
