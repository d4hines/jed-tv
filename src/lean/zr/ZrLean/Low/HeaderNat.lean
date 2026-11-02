import ZrLean.Low.BigNat

/-!
Tagged Nat header fast paths, with raw USize bit patterns instead of Lean Nat.
The scalar path preserves pointer-width wrapping arithmetic and overflow
checks; big values delegate to the bounded runtime. In particular compiler
unsigned literals keep their cached persistent representation, while ordinary
integer conversions return an owned value. Helpers are inline so extracted
header bodies retain the original small-Nat fast paths without extra calls.
-/
namespace ZrLean.Low.HeaderNat
open ZrLean.Low

@[inline] def isScalar (a : USize) : Bool := a &&& 1 != 0
@[inline] def bothScalar (a b : USize) : Bool := isScalar a && isScalar b
@[inline] def box (a : USize) : USize := (a <<< 1) ||| 1

@[inline, export zr_low_header_usize_to_nat]
def usizeToNat (n : USize) : BaseIO USize := do
  if n.toUInt64 ≤ BigNat.maxSmall 0 then pure (box n)
  else BigNat.fromUSize n

@[export zr_low_header_unsigned_to_nat]
def unsignedToNat (n : UInt32) : BaseIO USize := do
  if n.toUInt64 ≤ BigNat.maxSmall 0 then pure (box n.toUSize)
  else BigNat.unsignedLiteral n

@[export zr_low_header_uint64_to_nat]
def uint64ToNat (n : UInt64) : BaseIO USize := do
  if n ≤ BigNat.maxSmall 0 then pure (box n.toUSize)
  else BigNat.fromUInt64 n

@[export zr_low_header_uint8_to_nat]
def uint8ToNat (n : UInt8) : BaseIO USize := usizeToNat n.toUSize
@[export zr_low_header_uint16_to_nat]
def uint16ToNat (n : UInt16) : BaseIO USize := usizeToNat n.toUSize
@[export zr_low_header_uint32_to_nat]
def uint32ToNat (n : UInt32) : BaseIO USize := usizeToNat n.toUSize

@[export zr_low_header_nat_add]
def add (a b : USize) : BaseIO USize := do
  if bothScalar a b then usizeToNat ((a >>> 1) + (b >>> 1))
  else BigNat.add a b

@[export zr_low_header_nat_sub]
def sub (a b : USize) : BaseIO USize := do
  if bothScalar a b then
    let n := a >>> 1; let m := b >>> 1
    pure (if n < m then 1 else box (n - m))
  else BigNat.sub a b

@[export zr_low_header_nat_mul]
def mul (a b : USize) : BaseIO USize := do
  if bothScalar a b then
    let n := a >>> 1
    if n == 0 then pure a
    else
      let m := b >>> 1
      let r := n * m
      if r.toUInt64 ≤ BigNat.maxSmall 0 && r / n == m then pure (box r)
      else BigNat.overflowMul n m
  else BigNat.mul a b

@[export zr_low_header_nat_div]
def div (a b : USize) : BaseIO USize := do
  if bothScalar a b then
    let m := b >>> 1
    pure (if m == 0 then 1 else box ((a >>> 1) / m))
  else BigNat.div a b

@[export zr_low_header_nat_mod]
def mod (a b : USize) : BaseIO USize := do
  if bothScalar a b then
    let m := b >>> 1
    pure (if m == 0 then a else box ((a >>> 1) % m))
  else BigNat.mod a b

@[inline, export zr_low_header_nat_eq]
def eq (a b : USize) : BaseIO Bool := do
  if bothScalar a b then pure (a == b) else BigNat.eq a b
@[export zr_low_header_nat_dec_eq]
def decEq (a b : USize) : BaseIO Bool := eq a b

@[inline, export zr_low_header_nat_le]
def le (a b : USize) : BaseIO Bool := do
  if bothScalar a b then pure (a ≤ b) else BigNat.lessEq a b
@[export zr_low_header_nat_dec_le]
def decLe (a b : USize) : BaseIO Bool := le a b

@[inline, export zr_low_header_nat_lt]
def lt (a b : USize) : BaseIO Bool := do
  if bothScalar a b then pure (a < b) else BigNat.less a b
@[export zr_low_header_nat_dec_lt]
def decLt (a b : USize) : BaseIO Bool := lt a b

@[export zr_low_header_nat_land]
def land (a b : USize) : BaseIO USize := do
  if bothScalar a b then pure (a &&& b) else BigNat.land a b
@[export zr_low_header_nat_lor]
def lor (a b : USize) : BaseIO USize := do
  if bothScalar a b then pure (a ||| b) else BigNat.lor a b

@[export zr_low_header_nat_shiftr]
def shiftr (a b : USize) : BaseIO USize := do
  if bothScalar a b then
    let s := b >>> 1
    pure (if s < wordBytes 0 * 8 then box ((a >>> 1) >>> s) else 1)
  else BigNat.shiftr a b

@[inline, export zr_low_header_uint8_of_nat]
def uint8OfNat (a : USize) : BaseIO UInt8 := do
  if isScalar a then pure (a >>> 1).toUInt8 else BigNat.toUInt8 a
@[inline, export zr_low_header_uint16_of_nat]
def uint16OfNat (a : USize) : BaseIO UInt16 := do
  if isScalar a then pure (a >>> 1).toUInt16 else BigNat.toUInt16 a
@[inline, export zr_low_header_uint32_of_nat]
def uint32OfNat (a : USize) : BaseIO UInt32 := do
  if isScalar a then pure (a >>> 1).toUInt32 else BigNat.toUInt32 a
@[export zr_low_header_uint64_of_nat]
def uint64OfNat (a : USize) : BaseIO UInt64 := do
  if isScalar a then pure (a >>> 1).toUInt64 else BigNat.toUInt64 a
@[export zr_low_header_usize_of_nat]
def usizeOfNat (a : USize) : BaseIO USize := do
  if isScalar a then pure (a >>> 1) else BigNat.toUSize a

/-- Signed conversions return the same truncated bit patterns in unsigned ABI
slots, including when the original helper first casts through int16/int32. -/
@[export zr_low_header_int16_of_nat]
def int16OfNat (a : USize) : BaseIO UInt16 := uint16OfNat a
@[export zr_low_header_int32_of_nat]
def int32OfNat (a : USize) : BaseIO UInt32 := uint32OfNat a

@[export zr_low_header_nat_lxor]
def lxor (a b : USize) : BaseIO USize := do
  if bothScalar a b then pure (box ((a >>> 1) ^^^ (b >>> 1)))
  else BigNat.xor a b

end ZrLean.Low.HeaderNat
