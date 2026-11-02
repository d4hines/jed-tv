import Yt.Crypto.Aes
/-! AES-GCM (NIST SP 800-38D) with 96-bit nonces and 128-bit tags, the
form TLS 1.3 uses. GHASH uses Shoup's 4-bit tables (16 multiples of H per
key); like the AES tables, lookups depend on secret data (not
constant-time against cache-timing observers). -/
namespace Yt.Crypto.Gcm
open Yt.Crypto

/-- Expanded AES key plus the GHASH table for H = E(K, 0). -/
structure Key where
  aes : Aes.Key
  hh : Array UInt64
  hl : Array UInt64
deriving Inhabited

/-- Reduction constants for the 4-bit shift, pre-shifted into the top
16 bits. -/
def last4 : Array UInt64 := (#[0x0000, 0x1c20, 0x3840, 0x2460, 0x7080, 0x6ca0, 0x48c0, 0x54e0,
  0xe100, 0xfd20, 0xd940, 0xc560, 0x9180, 0x8da0, 0xa9c0, 0xb5e0] : Array UInt64).map (fun (x : UInt64) => x <<< (48 : UInt64))

def init (key : ByteArray) : Key := Id.run do
  let aes := Aes.expand key
  let h := Aes.encryptWords aes 0 0 0 0
  let mut vh : UInt64 := (h.w0.toUInt64 <<< 32) ||| h.w1.toUInt64
  let mut vl : UInt64 := (h.w2.toUInt64 <<< 32) ||| h.w3.toUInt64
  let mut hh : Array UInt64 := Array.replicate 16 0
  let mut hl : Array UInt64 := Array.replicate 16 0
  hh := hh.set! 8 vh
  hl := hl.set! 8 vl
  for i in [4, 2, 1] do
    let t : UInt64 := (vl &&& 1) * 0xe1000000
    vl := (vh <<< 63) ||| (vl >>> 1)
    vh := (vh >>> 1) ^^^ (t <<< 32)
    hh := hh.set! i vh
    hl := hl.set! i vl
  for i in [2, 4, 8] do
    for j in [1:i] do
      hh := hh.set! (i + j) (hh[i]! ^^^ hh[j]!)
      hl := hl.set! (i + j) (hl[i]! ^^^ hl[j]!)
  ⟨aes, hh, hl⟩

structure U128 where
  hi : UInt64
  lo : UInt64
deriving Inhabited

/-- Byte `i` (0 = most significant) of the 128-bit value `xh:xl`. -/
@[inline] def byteOf (xh xl : UInt64) (i : Nat) : UInt64 :=
  if i < 8 then (xh >>> (56 - 8 * i).toUInt64) &&& 0xff else (xl >>> (120 - 8 * i).toUInt64) &&& 0xff

/-- X·H: nibbles from the last byte to the first. -/
partial def mulLoop (hh hl : Array UInt64) (xh xl : UInt64) (i : Nat) (zh zl : UInt64) : U128 :=
  let x := byteOf xh xl i
  let lo := (x &&& 0xf).toNat
  let hi := (x >>> 4).toNat
  -- shift Z right by 4, add the low nibble's multiple
  let (zh, zl) := if i == 15 then (hh[lo]!, hl[lo]!) else
    let rem := (zl &&& 0xf).toNat
    let zl := (zh <<< 60) ||| (zl >>> 4)
    let zh := (zh >>> 4) ^^^ last4[rem]!
    (zh ^^^ hh[lo]!, zl ^^^ hl[lo]!)
  let rem := (zl &&& 0xf).toNat
  let zl := (zh <<< 60) ||| (zl >>> 4)
  let zh := (zh >>> 4) ^^^ last4[rem]!
  let zh := zh ^^^ hh[hi]!
  let zl := zl ^^^ hl[hi]!
  if i == 0 then ⟨zh, zl⟩ else mulLoop hh hl xh xl (i - 1) zh zl

/-- Absorb the bytes `b[off, off+len)` (zero-padded to whole blocks). -/
def absorb (k : Key) (x : U128) (b : ByteArray) (off len : Nat) : U128 := Id.run do
  let mut x := x
  let mut p := 0
  while p < len do
    let n := if len - p ≥ 16 then 16 else len - p
    let mut bh : UInt64 := 0
    let mut bl : UInt64 := 0
    if n == 16 then
      bh := getU64BE b (off + p)
      bl := getU64BE b (off + p + 8)
    else
      for j in [0:n] do
        let v := b[off + p + j]!.toUInt64
        if j < 8 then bh := bh ||| (v <<< (56 - 8 * j).toUInt64)
        else bl := bl ||| (v <<< (120 - 8 * j).toUInt64)
    x := mulLoop k.hh k.hl (x.hi ^^^ bh) (x.lo ^^^ bl) 15 0 0
    p := p + n
  x

/-- CTR-mode transform of `b[off, off+len)` starting at counter block
`n0 n1 n2 ctr`; appended to `out`. -/
def ctr (k : Key) (n0 n1 n2 : UInt32) (ctr0 : UInt32) (b : ByteArray) (off len : Nat)
    (out : ByteArray) : ByteArray := Id.run do
  let mut out := out
  let mut c := ctr0
  let mut p := 0
  while p < len do
    let ks := Aes.encryptWords k.aes n0 n1 n2 c
    c := c + 1
    if len - p ≥ 16 then
      out := pushU32BE out (getU32BE b (off + p) ^^^ ks.w0)
      out := pushU32BE out (getU32BE b (off + p + 4) ^^^ ks.w1)
      out := pushU32BE out (getU32BE b (off + p + 8) ^^^ ks.w2)
      out := pushU32BE out (getU32BE b (off + p + 12) ^^^ ks.w3)
      p := p + 16
    else
      let kb := pushU32BE (pushU32BE (pushU32BE (pushU32BE (ByteArray.emptyWithCapacity 16) ks.w0) ks.w1) ks.w2) ks.w3
      for j in [0:len - p] do out := out.push (b[off + p + j]! ^^^ kb[j]!)
      p := len
  out

/-- The tag over `aad` and the ciphertext `c[off, off+len)`. -/
def tag (k : Key) (n0 n1 n2 : UInt32) (aad : ByteArray) (c : ByteArray) (off len : Nat) : ByteArray :=
  let x := absorb k ⟨0, 0⟩ aad 0 aad.size
  let x := absorb k x c off len
  let x := mulLoop k.hh k.hl (x.hi ^^^ (aad.size * 8).toUInt64) (x.lo ^^^ (len * 8).toUInt64) 15 0 0
  let ek := Aes.encryptWords k.aes n0 n1 n2 1
  let t := ByteArray.emptyWithCapacity 16
  pushU64BE (pushU64BE t (x.hi ^^^ ((ek.w0.toUInt64 <<< 32) ||| ek.w1.toUInt64)))
    (x.lo ^^^ ((ek.w2.toUInt64 <<< 32) ||| ek.w3.toUInt64))

/-- Encrypt: ciphertext followed by the 16-byte tag. `nonce` is 12 bytes. -/
def encrypt (k : Key) (nonce aad pt : ByteArray) : ByteArray :=
  let n0 := getU32BE nonce 0
  let n1 := getU32BE nonce 4
  let n2 := getU32BE nonce 8
  let c := ctr k n0 n1 n2 2 pt 0 pt.size (ByteArray.emptyWithCapacity (pt.size + 16))
  c ++ tag k n0 n1 n2 aad c 0 c.size

/-- Decrypt `b[off, off+len)` (ciphertext then tag); `none` if the tag
does not verify. -/
def decryptAt (k : Key) (nonce aad : ByteArray) (b : ByteArray) (off len : Nat) : Option ByteArray :=
  if len < 16 || b.size < off + len then none else
  let n0 := getU32BE nonce 0
  let n1 := getU32BE nonce 4
  let n2 := getU32BE nonce 8
  let clen := len - 16
  let t := tag k n0 n1 n2 aad b off clen
  if ctEq t (slice b (off + clen) 16) then
    some (ctr k n0 n1 n2 2 b off clen (ByteArray.emptyWithCapacity clen))
  else none

def decrypt (k : Key) (nonce aad ct : ByteArray) : Option ByteArray := decryptAt k nonce aad ct 0 ct.size

end Yt.Crypto.Gcm
