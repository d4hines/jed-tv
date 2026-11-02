import Yt.Crypto.Util
/-! ChaCha20, Poly1305 and the ChaCha20-Poly1305 AEAD (RFC 8439). Pure
32-bit arithmetic with no secret-dependent branches or table indices
(constant-time as far as the compiled C is). The ChaCha rounds and the
Poly1305 block loop are self tail-recursive over unboxed `UInt32`
parameters. -/
namespace Yt.Crypto.ChaCha

@[inline] def rotl (x : UInt32) (n : UInt32) : UInt32 := (x <<< n) ||| (x >>> (32 - n))

structure W16 where
  x0 : UInt32
  x1 : UInt32
  x2 : UInt32
  x3 : UInt32
  x4 : UInt32
  x5 : UInt32
  x6 : UInt32
  x7 : UInt32
  x8 : UInt32
  x9 : UInt32
  x10 : UInt32
  x11 : UInt32
  x12 : UInt32
  x13 : UInt32
  x14 : UInt32
  x15 : UInt32
deriving Inhabited

/-- `n` double rounds, then the feed-forward addition of the input `s`. -/
partial def doubleRounds (n : Nat) (s : W16)
    (x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 x10 x11 x12 x13 x14 x15 : UInt32) : W16 :=
  if n == 0 then
    ⟨x0 + s.x0, x1 + s.x1, x2 + s.x2, x3 + s.x3, x4 + s.x4, x5 + s.x5, x6 + s.x6, x7 + s.x7,
     x8 + s.x8, x9 + s.x9, x10 + s.x10, x11 + s.x11, x12 + s.x12, x13 + s.x13, x14 + s.x14, x15 + s.x15⟩
  else
    -- columns
    let x0 := x0 + x4; let x12 := rotl (x12 ^^^ x0) 16; let x8 := x8 + x12; let x4 := rotl (x4 ^^^ x8) 12
    let x0 := x0 + x4; let x12 := rotl (x12 ^^^ x0) 8; let x8 := x8 + x12; let x4 := rotl (x4 ^^^ x8) 7
    let x1 := x1 + x5; let x13 := rotl (x13 ^^^ x1) 16; let x9 := x9 + x13; let x5 := rotl (x5 ^^^ x9) 12
    let x1 := x1 + x5; let x13 := rotl (x13 ^^^ x1) 8; let x9 := x9 + x13; let x5 := rotl (x5 ^^^ x9) 7
    let x2 := x2 + x6; let x14 := rotl (x14 ^^^ x2) 16; let x10 := x10 + x14; let x6 := rotl (x6 ^^^ x10) 12
    let x2 := x2 + x6; let x14 := rotl (x14 ^^^ x2) 8; let x10 := x10 + x14; let x6 := rotl (x6 ^^^ x10) 7
    let x3 := x3 + x7; let x15 := rotl (x15 ^^^ x3) 16; let x11 := x11 + x15; let x7 := rotl (x7 ^^^ x11) 12
    let x3 := x3 + x7; let x15 := rotl (x15 ^^^ x3) 8; let x11 := x11 + x15; let x7 := rotl (x7 ^^^ x11) 7
    -- diagonals
    let x0 := x0 + x5; let x15 := rotl (x15 ^^^ x0) 16; let x10 := x10 + x15; let x5 := rotl (x5 ^^^ x10) 12
    let x0 := x0 + x5; let x15 := rotl (x15 ^^^ x0) 8; let x10 := x10 + x15; let x5 := rotl (x5 ^^^ x10) 7
    let x1 := x1 + x6; let x12 := rotl (x12 ^^^ x1) 16; let x11 := x11 + x12; let x6 := rotl (x6 ^^^ x11) 12
    let x1 := x1 + x6; let x12 := rotl (x12 ^^^ x1) 8; let x11 := x11 + x12; let x6 := rotl (x6 ^^^ x11) 7
    let x2 := x2 + x7; let x13 := rotl (x13 ^^^ x2) 16; let x8 := x8 + x13; let x7 := rotl (x7 ^^^ x8) 12
    let x2 := x2 + x7; let x13 := rotl (x13 ^^^ x2) 8; let x8 := x8 + x13; let x7 := rotl (x7 ^^^ x8) 7
    let x3 := x3 + x4; let x14 := rotl (x14 ^^^ x3) 16; let x9 := x9 + x14; let x4 := rotl (x4 ^^^ x9) 12
    let x3 := x3 + x4; let x14 := rotl (x14 ^^^ x3) 8; let x9 := x9 + x14; let x4 := rotl (x4 ^^^ x9) 7
    doubleRounds (n - 1) s x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 x10 x11 x12 x13 x14 x15

/-- The 16 output words of block `ctr` for a 32-byte key and 12-byte nonce. -/
def block (key nonce : ByteArray) (ctr : UInt32) : W16 :=
  let s : W16 := ⟨0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
    getU32LE key 0, getU32LE key 4, getU32LE key 8, getU32LE key 12,
    getU32LE key 16, getU32LE key 20, getU32LE key 24, getU32LE key 28,
    ctr, getU32LE nonce 0, getU32LE nonce 4, getU32LE nonce 8⟩
  doubleRounds 10 s s.x0 s.x1 s.x2 s.x3 s.x4 s.x5 s.x6 s.x7 s.x8 s.x9 s.x10 s.x11 s.x12 s.x13 s.x14 s.x15

def W16.bytes (w : W16) : ByteArray :=
  let b := ByteArray.emptyWithCapacity 64
  let b := pushU32LE (pushU32LE (pushU32LE (pushU32LE b w.x0) w.x1) w.x2) w.x3
  let b := pushU32LE (pushU32LE (pushU32LE (pushU32LE b w.x4) w.x5) w.x6) w.x7
  let b := pushU32LE (pushU32LE (pushU32LE (pushU32LE b w.x8) w.x9) w.x10) w.x11
  pushU32LE (pushU32LE (pushU32LE (pushU32LE b w.x12) w.x13) w.x14) w.x15

@[inline] def xw (out : ByteArray) (b : ByteArray) (o : Nat) (k : UInt32) : ByteArray :=
  pushU32LE out (getU32LE b o ^^^ k)

/-- ChaCha20 over `b[off, off+len)` from block counter `ctr`, appended to `out`. -/
def xorStream (key nonce : ByteArray) (ctr : UInt32) (b : ByteArray) (off len : Nat)
    (out : ByteArray) : ByteArray := Id.run do
  let mut out := out
  let mut c := ctr
  let mut p := 0
  while p < len do
    let w := block key nonce c
    c := c + 1
    let o := off + p
    if len - p ≥ 64 then
      out := xw (xw (xw (xw out b o w.x0) b (o+4) w.x1) b (o+8) w.x2) b (o+12) w.x3
      out := xw (xw (xw (xw out b (o+16) w.x4) b (o+20) w.x5) b (o+24) w.x6) b (o+28) w.x7
      out := xw (xw (xw (xw out b (o+32) w.x8) b (o+36) w.x9) b (o+40) w.x10) b (o+44) w.x11
      out := xw (xw (xw (xw out b (o+48) w.x12) b (o+52) w.x13) b (o+56) w.x14) b (o+60) w.x15
      p := p + 64
    else
      let ks := w.bytes
      for j in [0:len - p] do out := out.push (b[o + j]! ^^^ ks[j]!)
      p := len
  out

/-! Poly1305 with five 26-bit limbs (the 32-bit "donna" layout). -/

structure PolyKey where
  r0 : UInt32
  r1 : UInt32
  r2 : UInt32
  r3 : UInt32
  r4 : UInt32
  p0 : UInt32
  p1 : UInt32
  p2 : UInt32
  p3 : UInt32
deriving Inhabited

structure Acc where
  h0 : UInt32
  h1 : UInt32
  h2 : UInt32
  h3 : UInt32
  h4 : UInt32
deriving Inhabited

def polyKey (k : ByteArray) : PolyKey :=
  ⟨getU32LE k 0 &&& 0x3ffffff, (getU32LE k 3 >>> 2) &&& 0x3ffff03, (getU32LE k 6 >>> 4) &&& 0x3ffc0ff,
   (getU32LE k 9 >>> 6) &&& 0x3f03fff, (getU32LE k 12 >>> 8) &&& 0x00fffff,
   getU32LE k 16, getU32LE k 20, getU32LE k 24, getU32LE k 28⟩

@[inline] def m64 (a b : UInt32) : UInt64 := a.toUInt64 * b.toUInt64

/-- Absorb `n` whole 16-byte blocks of `b` from `o`, each with the 2^128
bit `hibit` (1 <<< 24 normally). -/
partial def polyBlocks (r0 r1 r2 r3 r4 : UInt32) (hibit : UInt32) (b : ByteArray) (o n : Nat)
    (h0 h1 h2 h3 h4 : UInt32) : Acc :=
  if n == 0 then ⟨h0, h1, h2, h3, h4⟩ else
  let s1 := r1 * 5
  let s2 := r2 * 5
  let s3 := r3 * 5
  let s4 := r4 * 5
  let h0 := h0 + (getU32LE b o &&& 0x3ffffff)
  let h1 := h1 + ((getU32LE b (o+3) >>> 2) &&& 0x3ffffff)
  let h2 := h2 + ((getU32LE b (o+6) >>> 4) &&& 0x3ffffff)
  let h3 := h3 + ((getU32LE b (o+9) >>> 6) &&& 0x3ffffff)
  let h4 := h4 + ((getU32LE b (o+12) >>> 8) ||| hibit)
  let d0 := m64 h0 r0 + m64 h1 s4 + m64 h2 s3 + m64 h3 s2 + m64 h4 s1
  let d1 := m64 h0 r1 + m64 h1 r0 + m64 h2 s4 + m64 h3 s3 + m64 h4 s2
  let d2 := m64 h0 r2 + m64 h1 r1 + m64 h2 r0 + m64 h3 s4 + m64 h4 s3
  let d3 := m64 h0 r3 + m64 h1 r2 + m64 h2 r1 + m64 h3 r0 + m64 h4 s4
  let d4 := m64 h0 r4 + m64 h1 r3 + m64 h2 r2 + m64 h3 r1 + m64 h4 r0
  let c := d0 >>> 26
  let h0 := d0.toUInt32 &&& 0x3ffffff
  let d1 := d1 + c
  let c := d1 >>> 26
  let h1 := d1.toUInt32 &&& 0x3ffffff
  let d2 := d2 + c
  let c := d2 >>> 26
  let h2 := d2.toUInt32 &&& 0x3ffffff
  let d3 := d3 + c
  let c := d3 >>> 26
  let h3 := d3.toUInt32 &&& 0x3ffffff
  let d4 := d4 + c
  let c := (d4 >>> 26).toUInt32
  let h4 := d4.toUInt32 &&& 0x3ffffff
  let h0 := h0 + c * 5
  let c := h0 >>> 26
  let h0 := h0 &&& 0x3ffffff
  let h1 := h1 + c
  polyBlocks r0 r1 r2 r3 r4 hibit b (o + 16) (n - 1) h0 h1 h2 h3 h4

def absorb (k : PolyKey) (a : Acc) (b : ByteArray) (o n : Nat) (hibit : UInt32 := 0x1000000) : Acc :=
  polyBlocks k.r0 k.r1 k.r2 k.r3 k.r4 hibit b o n a.h0 a.h1 a.h2 a.h3 a.h4

/-- Absorb `b[off, off+len)` zero-padded to whole blocks (the AEAD layout). -/
def absorbPadded (k : PolyKey) (a : Acc) (b : ByteArray) (off len : Nat) : Acc :=
  let full := len / 16
  let a := absorb k a b off full
  if len % 16 == 0 then a else
    absorb k a (slice b (off + full * 16) (len % 16) ++ zeros (16 - len % 16)) 0 1

/-- Final reduction mod 2^130 - 5 and addition of the pad. -/
def finish (k : PolyKey) (a : Acc) : ByteArray :=
  let c := a.h1 >>> 26
  let h1 := a.h1 &&& 0x3ffffff
  let h2 := a.h2 + c
  let c := h2 >>> 26
  let h2 := h2 &&& 0x3ffffff
  let h3 := a.h3 + c
  let c := h3 >>> 26
  let h3 := h3 &&& 0x3ffffff
  let h4 := a.h4 + c
  let c := h4 >>> 26
  let h4 := h4 &&& 0x3ffffff
  let h0 := a.h0 + c * 5
  let c := h0 >>> 26
  let h0 := h0 &&& 0x3ffffff
  let h1 := h1 + c
  -- g = h + 5 - 2^130; select g when it does not borrow
  let g0 := h0 + 5
  let c := g0 >>> 26
  let g0 := g0 &&& 0x3ffffff
  let g1 := h1 + c
  let c := g1 >>> 26
  let g1 := g1 &&& 0x3ffffff
  let g2 := h2 + c
  let c := g2 >>> 26
  let g2 := g2 &&& 0x3ffffff
  let g3 := h3 + c
  let c := g3 >>> 26
  let g3 := g3 &&& 0x3ffffff
  let g4 : UInt32 := h4 + c - 0x4000000
  let mask : UInt32 := (g4 >>> 31) - 1
  let nm := ~~~mask
  let h0 := (h0 &&& nm) ||| (g0 &&& mask)
  let h1 := (h1 &&& nm) ||| (g1 &&& mask)
  let h2 := (h2 &&& nm) ||| (g2 &&& mask)
  let h3 := (h3 &&& nm) ||| (g3 &&& mask)
  let h4 := (h4 &&& nm) ||| (g4 &&& mask)
  let w0 := h0 ||| (h1 <<< 26)
  let w1 := (h1 >>> 6) ||| (h2 <<< 20)
  let w2 := (h2 >>> 12) ||| (h3 <<< 14)
  let w3 := (h3 >>> 18) ||| (h4 <<< 8)
  let f0 := w0.toUInt64 + k.p0.toUInt64
  let f1 := w1.toUInt64 + k.p1.toUInt64 + (f0 >>> 32)
  let f2 := w2.toUInt64 + k.p2.toUInt64 + (f1 >>> 32)
  let f3 := w3.toUInt64 + k.p3.toUInt64 + (f2 >>> 32)
  pushU32LE (pushU32LE (pushU32LE (pushU32LE (ByteArray.emptyWithCapacity 16) f0.toUInt32) f1.toUInt32) f2.toUInt32) f3.toUInt32

/-- Poly1305 of a whole message (RFC 8439 section 2.5). -/
def poly1305 (key msg : ByteArray) : ByteArray :=
  let k := polyKey key
  let full := msg.size / 16
  let a := absorb k ⟨0, 0, 0, 0, 0⟩ msg 0 full
  let r := msg.size % 16
  let a := if r == 0 then a else
    absorb k a ((slice msg (full * 16) r).push 1 ++ zeros (15 - r)) 0 1 0
  finish k a

def tag (key nonce aad : ByteArray) (c : ByteArray) (off len : Nat) : ByteArray :=
  let k := polyKey (block key nonce 0).bytes
  let a := absorbPadded k ⟨0, 0, 0, 0, 0⟩ aad 0 aad.size
  let a := absorbPadded k a c off len
  let lens := pushU32LE (pushU32LE (pushU32LE (pushU32LE (ByteArray.emptyWithCapacity 16)
    aad.size.toUInt32) (aad.size >>> 32).toUInt32) len.toUInt32) (len >>> 32).toUInt32
  finish k (absorb k a lens 0 1)

/-- AEAD encrypt: ciphertext followed by the 16-byte tag. -/
def encrypt (key nonce aad pt : ByteArray) : ByteArray :=
  let c := xorStream key nonce 1 pt 0 pt.size (ByteArray.emptyWithCapacity (pt.size + 16))
  c ++ tag key nonce aad c 0 c.size

/-- AEAD decrypt of `b[off, off+len)` (ciphertext then tag). -/
def decryptAt (key nonce aad : ByteArray) (b : ByteArray) (off len : Nat) : Option ByteArray :=
  if len < 16 || b.size < off + len then none else
  let clen := len - 16
  if ctEq (tag key nonce aad b off clen) (slice b (off + clen) 16) then
    some (xorStream key nonce 1 b off clen (ByteArray.emptyWithCapacity clen))
  else none

def decrypt (key nonce aad ct : ByteArray) : Option ByteArray := decryptAt key nonce aad ct 0 ct.size

end Yt.Crypto.ChaCha
