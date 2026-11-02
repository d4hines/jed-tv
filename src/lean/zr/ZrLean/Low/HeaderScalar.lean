/-!
Guarded fixed-width arithmetic for the generated ABI header. These are Lean
algorithms, exported under private bootstrap names. The header extractor lowers
terminal fixed-width operations to typed primitive syntax; aliases for lean_*
are emitted only after extraction, avoiding calls back into these guards.
Signed Int32 inputs/results are their UInt32 two's-complement bit patterns.
USize width is selected using word overflow, never a boxed platform constant.
The zero probe depends on the argument so Lean cannot extract a closed term;
clang folds that scalar identity separately for each compilation target.
-/
namespace ZrLean.Low.HeaderScalar

@[never_extract, export zr_low_header_uint8_mod]
def uint8Mod (a b : UInt8) : UInt8 :=
  if b == 0 then a else a % b

@[never_extract, export zr_low_header_uint8_shift_left]
def uint8ShiftLeft (a b : UInt8) : UInt8 :=
  a <<< (b % 8)

@[never_extract, export zr_low_header_uint8_shift_right]
def uint8ShiftRight (a b : UInt8) : UInt8 :=
  a >>> (b % 8)

@[never_extract, export zr_low_header_uint16_div]
def uint16Div (a b : UInt16) : UInt16 :=
  if b == 0 then 0 else a / b

@[never_extract, export zr_low_header_uint16_mod]
def uint16Mod (a b : UInt16) : UInt16 :=
  if b == 0 then a else a % b

@[never_extract, export zr_low_header_uint16_shift_left]
def uint16ShiftLeft (a b : UInt16) : UInt16 :=
  a <<< (b % 16)

@[never_extract, export zr_low_header_uint16_shift_right]
def uint16ShiftRight (a b : UInt16) : UInt16 :=
  a >>> (b % 16)

@[never_extract, export zr_low_header_uint32_div]
def uint32Div (a b : UInt32) : UInt32 :=
  if b == 0 then 0 else a / b

@[never_extract, export zr_low_header_uint32_mod]
def uint32Mod (a b : UInt32) : UInt32 :=
  if b == 0 then a else a % b

@[never_extract, export zr_low_header_uint32_shift_left]
def uint32ShiftLeft (a b : UInt32) : UInt32 :=
  a <<< (b % 32)

@[never_extract, export zr_low_header_uint32_shift_right]
def uint32ShiftRight (a b : UInt32) : UInt32 :=
  a >>> (b % 32)

@[never_extract, export zr_low_header_uint64_div]
def uint64Div (a b : UInt64) : UInt64 :=
  if b == 0 then 0 else a / b

@[never_extract, export zr_low_header_uint64_mod]
def uint64Mod (a b : UInt64) : UInt64 :=
  if b == 0 then a else a % b

@[never_extract, export zr_low_header_uint64_shift_left]
def uint64ShiftLeft (a b : UInt64) : UInt64 :=
  a <<< (b % 64)

@[never_extract, export zr_low_header_uint64_shift_right]
def uint64ShiftRight (a b : UInt64) : UInt64 :=
  a >>> (b % 64)

@[never_extract, export zr_low_header_usize_div]
def usizeDiv (a b : USize) : USize :=
  if b == 0 then 0 else a / b

@[never_extract, export zr_low_header_usize_mod]
def usizeMod (a b : USize) : USize :=
  if b == 0 then a else a % b

@[never_extract, export zr_low_header_usize_shift_left]
def usizeShiftLeft (a b : USize) : USize :=
  let probe := (a ^^^ a) + 0xffffffff
  let bits : USize := if probe + 1 == 0 then 32 else 64
  a <<< (b % bits)

@[never_extract, export zr_low_header_usize_shift_right]
def usizeShiftRight (a b : USize) : USize :=
  let probe := (a ^^^ a) + 0xffffffff
  let bits : USize := if probe + 1 == 0 then 32 else 64
  a >>> (b % bits)

private partial def logLoop (a r : UInt64) : UInt64 :=
  if a < 2 then r else logLoop (a / 2) (r + 1)

@[never_extract, export zr_low_header_uint64_log2]
def uint64Log2 (a : UInt64) : UInt64 := logLoop a 0

@[never_extract, export zr_low_header_int32_div]
def int32Div (a b : UInt32) : UInt32 :=
  if b == 0 then 0
  else
    let negativeA := a >>> 31 != 0
    let negativeB := b >>> 31 != 0
    let magnitudeA := if negativeA then 0 - a else a
    let magnitudeB := if negativeB then 0 - b else b
    let q := magnitudeA / magnitudeB
    if negativeA != negativeB then 0 - q else q

@[never_extract, export zr_low_header_int32_shift_left]
def int32ShiftLeft (a b : UInt32) : UInt32 :=
  a <<< (b % 32)

@[never_extract, export zr_low_header_int32_shift_right]
def int32ShiftRight (a b : UInt32) : UInt32 :=
  let shift := b % 32
  if shift == 0 then a
  else
    let logical := a >>> shift
    if a >>> 31 == 0 then logical
    else logical ||| (0xffffffff <<< (32 - shift))


@[never_extract, export zr_low_header_uint8_div]
def uint8Div (a b : UInt8) : UInt8 :=
  if b == 0 then 0 else a / b

@[never_extract, export zr_low_header_uint8_log2]
def uint8Log2 (a : UInt8) : UInt8 := (logLoop a.toUInt64 0).toUInt8

@[never_extract, export zr_low_header_uint16_log2]
def uint16Log2 (a : UInt16) : UInt16 := (logLoop a.toUInt64 0).toUInt16

@[never_extract, export zr_low_header_uint32_log2]
def uint32Log2 (a : UInt32) : UInt32 := (logLoop a.toUInt64 0).toUInt32

@[never_extract, export zr_low_header_usize_log2]
def usizeLog2 (a : USize) : USize := (logLoop a.toUInt64 0).toUSize

end ZrLean.Low.HeaderScalar
