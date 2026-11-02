import Yt.Json.Fmt

/-! The pixel arithmetic of libjpeg-turbo (3.1, its C code, which its SIMD
code matches bit for bit), so that our output equals `djpeg`'s:
`jpeg_idct_islow` (jidctint.c) with its post-IDCT range limit, and the
YCbCr→RGB conversion of jdcolor.c. All arithmetic is 32-bit two's
complement in `UInt32` (as `JLONG` on the TV), with arithmetic shifts
written out; signed `Int32` constants would compile to once-cells. -/
namespace Yt.Jpeg
open Yt.Fmt

/-- Arithmetic shift right of a two's-complement 32-bit value. -/
@[inline] def sar (x : UInt32) (n : UInt32) : UInt32 :=
  let s : UInt32 := (0 : UInt32) - (x >>> 31)
  ((x ^^^ s) >>> n) ^^^ s

/-- libjpeg's `DESCALE(x, n)`: round then shift (arithmetic). -/
@[inline] def descale (x : UInt32) (n : UInt32) : UInt32 :=
  sar (x + ((1 : UInt32) <<< (n - 1))) n

/-- The post-IDCT `range_limit[x & RANGE_MASK]` of jdmaster.c: `x` is a
centred sample (a 10-bit two's-complement value after masking). -/
@[inline] def rangeLimit (x : UInt32) : UInt8 :=
  let i := x &&& 1023
  if i < 128 then (i + 128).toUInt8
  else if i < 512 then 255
  else if i < 896 then 0
  else (i - 896).toUInt8

/-- Clamp a signed sample to 0..255 (jdmaster.c's "simple" range limit). -/
@[inline] def clamp255 (x : UInt32) : UInt8 :=
  if x >>> 31 != 0 then 0 else if x > 255 then 255 else x.toUInt8

/-- 8 values (one IDCT column or row). -/
structure V8 where
  (a0 a1 a2 a3 a4 a5 a6 a7 : UInt32)

-- FIX(x) constants of jidctint.c (CONST_BITS = 13).
@[inline] def k0298 : UInt32 := 2446
@[inline] def k0390 : UInt32 := 3196
@[inline] def k0541 : UInt32 := 4433
@[inline] def k0765 : UInt32 := 6270
@[inline] def k0899 : UInt32 := 7373
@[inline] def k1175 : UInt32 := 9633
@[inline] def k1501 : UInt32 := 12299
@[inline] def k1847 : UInt32 := 15137
@[inline] def k1961 : UInt32 := 16069
@[inline] def k2053 : UInt32 := 16819
@[inline] def k2562 : UInt32 := 20995
@[inline] def k3072 : UInt32 := 25172

/-- The 1-D kernel of jpeg_idct_islow on inputs `i0..i7` (dequantized
coefficients in pass 1, workspace values in pass 2), before the final
descale: outputs in the order 0..7 (tmp10+tmp3, tmp11+tmp2, tmp12+tmp1,
tmp13+tmp0, tmp13-tmp0, tmp12-tmp1, tmp11-tmp2, tmp10-tmp3). -/
@[inline] def kernel (i0 i1 i2 i3 i4 i5 i6 i7 : UInt32) : V8 :=
  -- Even part.
  let z2 := i2
  let z3 := i6
  let z1 := (z2 + z3) * k0541
  let tmp2 := z1 - z3 * k1847
  let tmp3 := z1 + z2 * k0765
  let tmp0 := (i0 + i4) <<< 13
  let tmp1 := (i0 - i4) <<< 13
  let tmp10 := tmp0 + tmp3
  let tmp13 := tmp0 - tmp3
  let tmp11 := tmp1 + tmp2
  let tmp12 := tmp1 - tmp2
  -- Odd part.
  let t0 := i7
  let t1 := i5
  let t2 := i3
  let t3 := i1
  let z1 := t0 + t3
  let z2 := t1 + t2
  let z3 := t0 + t2
  let z4 := t1 + t3
  let z5 := (z3 + z4) * k1175
  let t0 := t0 * k0298
  let t1 := t1 * k2053
  let t2 := t2 * k3072
  let t3 := t3 * k1501
  let z1 := (0 : UInt32) - z1 * k0899
  let z2 := (0 : UInt32) - z2 * k2562
  let z3 := (0 : UInt32) - z3 * k1961 + z5
  let z4 := (0 : UInt32) - z4 * k0390 + z5
  let t0 := t0 + z1 + z3
  let t1 := t1 + z2 + z4
  let t2 := t2 + z2 + z3
  let t3 := t3 + z1 + z4
  ⟨tmp10 + t3, tmp11 + t2, tmp12 + t1, tmp13 + t0, tmp13 - t0, tmp12 - t1, tmp11 - t2, tmp10 - t3⟩

/-- Pass 1 on one column: descale by CONST_BITS - PASS1_BITS = 11. With
all AC terms zero this is `dc << PASS1_BITS` (libjpeg's shortcut, same value). -/
@[inline] def col (i0 i1 i2 i3 i4 i5 i6 i7 : UInt32) : V8 :=
  if i1 == 0 && i2 == 0 && i3 == 0 && i4 == 0 && i5 == 0 && i6 == 0 && i7 == 0 then
    let v := i0 <<< 2
    ⟨v, v, v, v, v, v, v, v⟩
  else
  let r := kernel i0 i1 i2 i3 i4 i5 i6 i7
  ⟨descale r.a0 11, descale r.a1 11, descale r.a2 11, descale r.a3 11,
   descale r.a4 11, descale r.a5 11, descale r.a6 11, descale r.a7 11⟩

/-- Signed 16-bit value at byte offset `2i` (little-endian), sign-extended. -/
@[inline] def s16 (b : ByteArray) (i : Nat) : UInt32 :=
  let v := (byteAt b (2 * i)).toUInt32 ||| ((byteAt b (2 * i + 1)).toUInt32 <<< 8)
  if v &&& 0x8000 != 0 then v ||| 0xFFFF0000 else v

/-- Pass 2 on one row: descale by CONST_BITS + PASS1_BITS + 3 = 18, range
limit, and store 8 samples at `plane[o, o+8)`. -/
@[inline] def row (plane : ByteArray) (o : Nat) (i0 i1 i2 i3 i4 i5 i6 i7 : UInt32) : ByteArray :=
  let r := kernel i0 i1 i2 i3 i4 i5 i6 i7
  let p := plane.set! o (rangeLimit (descale r.a0 18))
  let p := p.set! (o + 1) (rangeLimit (descale r.a1 18))
  let p := p.set! (o + 2) (rangeLimit (descale r.a2 18))
  let p := p.set! (o + 3) (rangeLimit (descale r.a3 18))
  let p := p.set! (o + 4) (rangeLimit (descale r.a4 18))
  let p := p.set! (o + 5) (rangeLimit (descale r.a5 18))
  let p := p.set! (o + 6) (rangeLimit (descale r.a6 18))
  p.set! (o + 7) (rangeLimit (descale r.a7 18))

/-- jpeg_idct_islow of the block whose 64 coefficients (natural order,
int16) start at coefficient index `cb` of `c`, with the quantization table
`q` (64 int16, natural order), into `plane` at offset `o` with row stride
`stride`. libjpeg's shortcuts for all-zero columns and rows give the same
values as the full computation, so they are not needed for exactness. -/
@[inline] def dq (c : ByteArray) (cb : Nat) (q : ByteArray) (k : Nat) : UInt32 := s16 c (cb + k) * s16 q k

def idctBlock (c : ByteArray) (cb : Nat) (q : ByteArray) (plane : ByteArray) (o stride : Nat) : ByteArray :=
  let c0 := col (dq c cb q 0) (dq c cb q 8) (dq c cb q 16) (dq c cb q 24) (dq c cb q 32) (dq c cb q 40) (dq c cb q 48) (dq c cb q 56)
  let c1 := col (dq c cb q 1) (dq c cb q 9) (dq c cb q 17) (dq c cb q 25) (dq c cb q 33) (dq c cb q 41) (dq c cb q 49) (dq c cb q 57)
  let c2 := col (dq c cb q 2) (dq c cb q 10) (dq c cb q 18) (dq c cb q 26) (dq c cb q 34) (dq c cb q 42) (dq c cb q 50) (dq c cb q 58)
  let c3 := col (dq c cb q 3) (dq c cb q 11) (dq c cb q 19) (dq c cb q 27) (dq c cb q 35) (dq c cb q 43) (dq c cb q 51) (dq c cb q 59)
  let c4 := col (dq c cb q 4) (dq c cb q 12) (dq c cb q 20) (dq c cb q 28) (dq c cb q 36) (dq c cb q 44) (dq c cb q 52) (dq c cb q 60)
  let c5 := col (dq c cb q 5) (dq c cb q 13) (dq c cb q 21) (dq c cb q 29) (dq c cb q 37) (dq c cb q 45) (dq c cb q 53) (dq c cb q 61)
  let c6 := col (dq c cb q 6) (dq c cb q 14) (dq c cb q 22) (dq c cb q 30) (dq c cb q 38) (dq c cb q 46) (dq c cb q 54) (dq c cb q 62)
  let c7 := col (dq c cb q 7) (dq c cb q 15) (dq c cb q 23) (dq c cb q 31) (dq c cb q 39) (dq c cb q 47) (dq c cb q 55) (dq c cb q 63)
  let p := row plane o c0.a0 c1.a0 c2.a0 c3.a0 c4.a0 c5.a0 c6.a0 c7.a0
  let p := row p (o + stride) c0.a1 c1.a1 c2.a1 c3.a1 c4.a1 c5.a1 c6.a1 c7.a1
  let p := row p (o + 2 * stride) c0.a2 c1.a2 c2.a2 c3.a2 c4.a2 c5.a2 c6.a2 c7.a2
  let p := row p (o + 3 * stride) c0.a3 c1.a3 c2.a3 c3.a3 c4.a3 c5.a3 c6.a3 c7.a3
  let p := row p (o + 4 * stride) c0.a4 c1.a4 c2.a4 c3.a4 c4.a4 c5.a4 c6.a4 c7.a4
  let p := row p (o + 5 * stride) c0.a5 c1.a5 c2.a5 c3.a5 c4.a5 c5.a5 c6.a5 c7.a5
  let p := row p (o + 6 * stride) c0.a6 c1.a6 c2.a6 c3.a6 c4.a6 c5.a6 c6.a6 c7.a6
  row p (o + 7 * stride) c0.a7 c1.a7 c2.a7 c3.a7 c4.a7 c5.a7 c6.a7 c7.a7

/-- A block with only a DC coefficient: every sample is
`range_limit[DESCALE((dc * q0) << PASS1_BITS, PASS1_BITS + 3)]`. -/
def dcBlock (dcq : UInt32) (plane : ByteArray) (o stride : Nat) : ByteArray := Id.run do
  let v := rangeLimit (descale (dcq <<< 2) 5)
  let mut p := plane
  for r in [0:8] do
    let base := o + r * stride
    for k in [0:8] do
      p := p.set! (base + k) v
  return p

/-- libjpeg's YCbCr→RGB (jdcolor.c, SCALEBITS 16), stored at `out[o, o+3)`. -/
@[inline] def setRgb (out : ByteArray) (o : Nat) (y cb cr : UInt8) : ByteArray :=
  let y := y.toUInt32
  let x := cr.toUInt32 - 128
  let z := cb.toUInt32 - 128
  let r := y + sar (x * 91881 + 32768) 16
  let g := y + sar ((0 : UInt32) - x * 46802 + ((0 : UInt32) - z * 22554 + 32768)) 16
  let b := y + sar (z * 116130 + 32768) 16
  ((out.set! o (clamp255 r)).set! (o + 1) (clamp255 g)).set! (o + 2) (clamp255 b)

end Yt.Jpeg
