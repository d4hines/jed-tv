import ZrLean.Low.BigInt

/-!
Int header fast paths, as HeaderNat does for Nat: raw USize words, the small
(scalar) case inline, anything else in BigInt. Representation and bounds:
BigInt. Arithmetic on two small Ints is exact in 64 bits; only a result
outside the small range allocates. Arguments are borrowed except where
lean.h's are owned (`nat_to_int`, `int_neg_succ_of_nat`, `int_to_nat`).
-/
namespace ZrLean.Low.HeaderInt
open ZrLean.Low ZrLean.Low.BigInt

@[inline] def isScalar (a : USize) : Bool := a &&& 1 != 0
@[inline] def bothScalar (a b : USize) : Bool := isScalar a && isScalar b

/-- `Int.ofNat` (owned): a small Nat below LEAN_MAX_SMALL_INT is the same word. -/
@[export zr_low_header_nat_to_int]
def natToInt (a : USize) : BaseIO USize := do
  if isScalar a then
    let n := (a >>> 1).toUInt64
    if fits n then pure a else BigInt.ofWords n 0
  else BigInt.ofNat a

/-- `Int.negSucc` (owned): `-(a + 1)`, which is `~a`. -/
@[export zr_low_header_int_neg_succ_of_nat]
def negSuccOfNat (a : USize) : BaseIO USize := do
  if isScalar a then ofInt64 (~~~(a >>> 1).toUInt64) else BigInt.negSuccOfNat a

@[export zr_low_header_int_neg]
def neg (a : USize) : BaseIO USize := do
  if isScalar a then ofInt64 (0 - unbox a) else BigInt.neg a

@[export zr_low_header_int_add]
def add (a b : USize) : BaseIO USize := do
  if bothScalar a b then ofInt64 (unbox a + unbox b) else BigInt.add a b

@[export zr_low_header_int_sub]
def sub (a b : USize) : BaseIO USize := do
  if bothScalar a b then ofInt64 (unbox a - unbox b) else BigInt.sub a b

@[export zr_low_header_int_mul]
def mul (a b : USize) : BaseIO USize := do
  if bothScalar a b then ofInt64 (unbox a * unbox b) else BigInt.mul a b

/-- Small `x` divided by small nonzero `y` in BigInt.divMod's `mode`, on 32-bit
magnitudes. Every result but the quotient 2^30 (of -2^30 by -1) is small. -/
@[inline] def smallDivMod (mode : UInt8) (x y : UInt64) : BaseIO USize :=
  let negX := isNeg x
  let negY := isNeg y
  let mx := (if negX then 0 - x else x).toUInt32
  let my := (if negY then 0 - y else y).toUInt32
  let q := (mx / my).toUInt64
  let r := (mx % my).toUInt64
  let away := negX && r != 0
  if mode == 0 then ofInt64 (if negX != negY then 0 - q else q)
  else if mode == 1 then pure (box (if negX then 0 - r else r))
  else if mode == 2 then
    let q := if away then q + 1 else q
    ofInt64 (if negX != negY then 0 - q else q)
  else pure (box (if away then my.toUInt64 - r else r))

@[inline] def divMod (a b : USize) (mode : UInt8) : BaseIO USize := do
  if bothScalar a b then
    let y := unbox b
    if y == 0 then pure (if mode &&& 1 != 0 then a else 1)
    else smallDivMod mode (unbox a) y
  else BigInt.divMod a b mode

@[export zr_low_header_int_div]
def div (a b : USize) : BaseIO USize := divMod a b 0
@[export zr_low_header_int_div_exact]
def divExact (a b : USize) : BaseIO USize := divMod a b 0
@[export zr_low_header_int_mod]
def mod (a b : USize) : BaseIO USize := divMod a b 1
@[export zr_low_header_int_ediv]
def ediv (a b : USize) : BaseIO USize := divMod a b 2
@[export zr_low_header_int_emod]
def emod (a b : USize) : BaseIO USize := divMod a b 3

@[inline, export zr_low_header_int_eq]
def eq (a b : USize) : BaseIO Bool := do
  if bothScalar a b then pure (a == b) else BigInt.eq a b
@[export zr_low_header_int_ne]
def ne (a b : USize) : BaseIO Bool := do pure (!(← eq a b))
@[export zr_low_header_int_dec_eq]
def decEq (a b : USize) : BaseIO Bool := eq a b

@[inline, export zr_low_header_int_le]
def le (a b : USize) : BaseIO Bool := do
  if bothScalar a b then pure (key a ≤ key b) else BigInt.lessEq a b
@[export zr_low_header_int_dec_le]
def decLe (a b : USize) : BaseIO Bool := le a b

@[inline, export zr_low_header_int_lt]
def lt (a b : USize) : BaseIO Bool := do
  if bothScalar a b then pure (key a < key b) else BigInt.less a b
@[export zr_low_header_int_dec_lt]
def decLt (a b : USize) : BaseIO Bool := lt a b

@[export zr_low_header_int_dec_nonneg]
def decNonneg (a : USize) : BaseIO Bool := do
  if isScalar a then pure (!isNeg (unbox a)) else pure (!isNeg (← BigNat.hi a))

/-- `Int.natAbs` (borrowed); a small Int's magnitude is a small Nat. -/
@[export zr_low_header_nat_abs]
def natAbs (a : USize) : BaseIO USize := do
  if isScalar a then
    let v := unbox a
    pure (BigNat.boxed (if isNeg v then 0 - v else v).toUSize)
  else BigInt.natAbs a

/-- `lean_int_to_nat` (owned), for a nonnegative Int; 0 for a negative one,
as `Int.toNat`. A small nonnegative Int is the same word as its Nat. -/
@[export zr_low_header_int_to_nat]
def intToNat (a : USize) : BaseIO USize := do
  if isScalar a then pure (if isNeg (unbox a) then 1 else a) else BigInt.toNat a

/-! Fixed-width signed integers (two's complement bits in unsigned slots, as
HeaderScalar): truncation of an Int, and sign extension to an Int. -/

@[export zr_low_header_int8_of_int]
def int8OfInt (a : USize) : BaseIO UInt8 := do pure (← BigInt.lo a).toUInt8
@[export zr_low_header_int16_of_int]
def int16OfInt (a : USize) : BaseIO UInt16 := do pure (← BigInt.lo a).toUInt16
@[export zr_low_header_int32_of_int]
def int32OfInt (a : USize) : BaseIO UInt32 := do pure (← BigInt.lo a).toUInt32
@[export zr_low_header_int64_of_int]
def int64OfInt (a : USize) : BaseIO UInt64 := BigInt.lo a
@[export zr_low_header_isize_of_int]
def isizeOfInt (a : USize) : BaseIO USize := do pure (← BigInt.lo a).toUSize

@[export zr_low_header_int8_to_int]
def int8ToInt (a : UInt8) : USize :=
  box (if a >>> 7 != 0 then a.toUInt64 ||| 0xffffffffffffff00 else a.toUInt64)
@[export zr_low_header_int16_to_int]
def int16ToInt (a : UInt16) : USize :=
  box (if a >>> 15 != 0 then a.toUInt64 ||| 0xffffffffffff0000 else a.toUInt64)
@[export zr_low_header_int32_to_int]
def int32ToInt (a : UInt32) : BaseIO USize :=
  ofInt64 (if a >>> 31 != 0 then a.toUInt64 ||| 0xffffffff00000000 else a.toUInt64)
@[export zr_low_header_int64_to_int_sint]
def int64ToInt (a : UInt64) : BaseIO USize := ofInt64 a
@[export zr_low_header_isize_to_int]
def isizeToInt (a : USize) : BaseIO USize :=
  ofInt64 (if wordBytes 0 == 4 && a >>> 31 != 0 then a.toUInt64 ||| 0xffffffff00000000 else a.toUInt64)

end ZrLean.Low.HeaderInt
