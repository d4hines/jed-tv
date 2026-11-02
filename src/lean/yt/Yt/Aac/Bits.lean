import Yt.Aac.Buf

/-! MSB-first bit reading over a `ByteArray`. Positions are bit offsets;
bits past the end read as 0 (callers check for overrun at the end). -/

namespace Yt.Aac

/-- The 32 bits starting at bit `pos`. -/
@[inline] def peek32 (a : ByteArray) (pos : USize) : UInt32 :=
  let b := pos >>> 3
  let w : UInt64 :=
    ((ld8 a b).toUInt64 <<< 32) ||| ((ld8 a (b + 1)).toUInt64 <<< 24) |||
    ((ld8 a (b + 2)).toUInt64 <<< 16) ||| ((ld8 a (b + 3)).toUInt64 <<< 8) |||
    (ld8 a (b + 4)).toUInt64
  ((w <<< (pos &&& 7).toUInt64) >>> 8).toUInt32

/-- The `n` bits starting at `pos`, `1 ≤ n ≤ 32`. -/
@[inline] def peekBits (a : ByteArray) (pos : USize) (n : UInt32) : UInt32 :=
  peek32 a pos >>> (32 - n)

/-- A bit reader for the non-critical parts of the syntax. -/
structure BitReader where
  data : ByteArray
  pos : USize

namespace BitReader

/-- Read `n ≤ 32` bits (0 bits reads 0). -/
@[inline] def read (r : BitReader) (n : UInt32) : UInt32 × BitReader :=
  if n == 0 then (0, r) else (peekBits r.data r.pos n, { r with pos := r.pos + n.toUSize })

@[inline] def readNat (r : BitReader) (n : UInt32) : Nat × BitReader :=
  let (v, r) := r.read n
  (v.toNat, r)

@[inline] def readBool (r : BitReader) : Bool × BitReader :=
  let (v, r) := r.read 1
  (v == 1, r)

@[inline] def skip (r : BitReader) (n : Nat) : BitReader := { r with pos := r.pos + n.toUSize }

@[inline] def byteAlign (r : BitReader) : BitReader := { r with pos := (r.pos + 7) &&& ~~~7 }

/-- Bits consumed so far. -/
@[inline] def bitPos (r : BitReader) : Nat := r.pos.toNat

/-- Has the reader gone past the end of the data? -/
@[inline] def overrun (r : BitReader) : Bool := r.pos.toNat > 8 * r.data.size

@[inline] def bitsLeft (r : BitReader) : Nat := 8 * r.data.size - r.pos.toNat

end BitReader

end Yt.Aac
