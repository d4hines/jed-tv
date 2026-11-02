/-
The standard-library shim: the few `Init` functions ordinary code reaches
without inlining them (`xs[i]!` on a `ByteArray`, `toString` on a `Nat`,
`Char.ofNat`, `ByteArray.extract`, `Array.reverse`, `Int.toNat`, `Int.pow`,
`>>>`/`<<<` on `Int`, `default` of `Array`/`Int`). Our runtime links no
`Init` code, so these are written here, in ZrLow style with explicit
reference counting, and runtime.S binds upstream's C names to them.

Ownership follows upstream's compiled signatures, which `ZrInitCheck.lean`
re-reads from the toolchain: a borrowed argument is neither consumed nor
returned without an `inc`; an owned one is consumed.
-/
import ZrLean.Low.Str
import ZrLean.Low.Arr
import ZrLean.Low.HeaderInt

-- Runtime code must not read module constants (`zr-cgen check`).
set_option compiler.extract_closed false

namespace ZrLean.Low.InitShim
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc

@[never_extract, extern "zr_prim_runtime_msg11"]
opaque outOfBoundsMsg (z : USize) : USize

/-- `outOfBounds._redArg (default : @& tobj) : tobj`: the panic of `xs[i]!`
on a type with no runtime-side panic; returns the `Inhabited` default. -/
@[export zr_low_shim_out_of_bounds]
def outOfBounds (default : USize) : BaseIO USize := do
  inc default
  let _ ← Runtime.leanPanic (outOfBoundsMsg 0) 59
  pure default

/-- `Char.ofNat (n : @& tobj) : u32`: `n` if it is a Unicode scalar value,
else 0. -/
@[export zr_low_shim_char_of_nat]
def charOfNat (n : USize) : BaseIO UInt32 := do
  if n &&& 1 == 0 then pure 0
  else
    let v := n >>> 1
    pure (if v < 0xd800 || (0xdfff < v && v < 0x110000) then v.toUInt32 else 0)

/-- Write the decimal digits of the 128-bit value `h:l` backwards, ending
before `p`; the address of the first digit. -/
partial def digits128 (p : USize) (l h : UInt64) : BaseIO USize := do
  let qh := h / 10
  let r1 := h % 10
  let a := (r1 <<< 32) ||| (l >>> 32)
  let qa := a / 10
  let b := ((a % 10) <<< 32) ||| (l &&& 0xffffffff)
  let ql := (qa <<< 32) ||| (b / 10)
  st8 (p - 1) (48 + b % 10).toUInt8
  if qh == 0 && ql == 0 then pure (p - 1) else digits128 (p - 1) ql qh

/-- `Nat.reprFast (n : tobj) : obj`: the decimal string of `n` (at most 39
digits within this runtime's 2^128 bound). -/
@[export zr_low_shim_nat_repr]
def natRepr (n : USize) : BaseIO USize := do
  let l ← BigNat.lo n
  let h ← BigNat.hi n
  dec n
  let s ← HeaderSupport.allocString 1 41 0
  let d := s + strData 0
  let start ← digits128 (d + 40) l h
  let k := d + 40 - start
  move d start k
  st8 (d + k) 0
  stp (s + strSizeOff 0) (k + 1)
  stp (s + Str.lenOff 0) k
  pure s

/-- `ByteArray.extract (a : @& obj) (b : tobj) (e : @& tobj) : obj`: bytes
`b` up to `e` of `a`, as `a.copySlice b empty 0 (e - b)`. -/
@[export zr_low_shim_byte_array_extract]
def byteArrayExtract (a b e : USize) : BaseIO USize := do
  let len ← BigNat.sub e b
  let r ← Arr.copySlice a b (← allocSArray 1 0 0) 1 len 1
  dec b
  dec len
  pure r

partial def swapLoop (d i j : USize) : BaseIO Unit := do
  if i < j then
    let x ← ldp (d + i * wordBytes 0)
    let y ← ldp (d + j * wordBytes 0)
    stp (d + i * wordBytes 0) y
    stp (d + j * wordBytes 0) x
    swapLoop d (i + 1) (j - 1)

/-- `Array.reverse._redArg (a : obj) : obj`. -/
@[export zr_low_shim_array_reverse]
def arrayReverse (a : USize) : BaseIO USize := do
  let n ← ldp (a + arrSizeOff 0)
  if n ≤ 1 then pure a
  else
    let r ← if ← isExclusive a then pure a else Arr.copyExpandArrayNonlinear a 0
    swapLoop (r + arrData 0) 0 (n - 1)
    pure r

/-- `Array.extract._redArg (as : @& obj) (start stop : tobj) : obj`: the
elements from `start` up to `min stop as.size`. -/
@[export zr_low_shim_array_extract]
def arrayExtract (a start stop : USize) : BaseIO USize := do
  let size ← ldp (a + arrSizeOff 0)
  let s := Arr.natToSizeSat start
  let e := Arr.natToSizeSat stop
  let e := if e > size then size else e
  let n := if s < e then e - s else 0
  let r ← allocArray n n
  Arr.incCopy (a + arrData 0 + s * wordBytes 0) (r + arrData 0) 0 n
  dec start
  dec stop
  pure r

/-- `Array.append._redArg (as : obj) (bs : @& obj) : obj`: `as ++ bs`, in
place when `as` is exclusive with room. -/
@[export zr_low_shim_array_append]
def arrayAppend (a b : USize) : BaseIO USize := do
  let n ← ldp (b + arrSizeOff 0)
  if n == 0 then pure a
  else
    let sz ← ldp (a + arrSizeOff 0)
    let cap ← ldp (a + arrCapOff 0)
    let r ← if (← isExclusive a) && cap ≥ sz + n then pure a
      else
        let r ← allocArray sz (sz + n)
        copyLinearMark a r
        if ← isExclusive a then
          copy (r + arrData 0) (a + arrData 0) (sz * wordBytes 0)
          deallocSized a (arrData 0 + wordBytes 0 * cap)
        else
          Arr.incCopy (a + arrData 0) (r + arrData 0) 0 sz
          decRef a
        pure r
    Arr.incCopy (b + arrData 0) (r + arrData 0 + sz * wordBytes 0) 0 n
    stp (r + arrSizeOff 0) (sz + n)
    pure r

/-- `Array.instInhabited._redArg : obj` (its one argument is erased): `#[]`. -/
@[export zr_low_shim_array_inhabited]
def arrayInhabited (_z : USize) : BaseIO USize := allocArray 0 0

/-! `Int` (representation and bounds: ZrLean.Low.BigInt; `Int.instInhabited`
is a constant word in runtime.S). -/

/-- `Int.toNat (x : @& tobj) : tobj`: 0 for a negative `x`. -/
@[export zr_low_shim_int_to_nat]
def intToNat (x : USize) : BaseIO USize := do
  if ← HeaderInt.decNonneg x then HeaderInt.natAbs x else pure 1

/-- `Int.pow (x n : @& tobj) : tobj`: `x.natAbs ^ n`, negated for a negative
`x` and an odd `n`. -/
@[export zr_low_shim_int_pow]
def intPow (x n : USize) : BaseIO USize := do
  let m ← HeaderInt.natAbs x
  let p ← BigNat.pow m n
  let neg := !(← HeaderInt.decNonneg x) && (← BigNat.lo n) &&& 1 != 0
  let r ← BigInt.ofMag neg (← BigNat.lo p) (← BigNat.hi p)
  dec p
  dec m
  pure r

/-- `Int.shiftRight (x s : @& tobj) : tobj`: `x >>> s`, rounding toward
minus infinity (an arithmetic shift). -/
@[export zr_low_shim_int_shift_right]
def intShiftRight (x s : USize) : BaseIO USize := do
  let huge := s &&& 1 == 0
  if x &&& 1 != 0 then
    let k : UInt64 := if huge || s >>> 1 ≥ 63 then 63 else (s >>> 1).toUInt64
    pure (BigInt.box (BigInt.sar (BigInt.unbox x) k))
  else
    let k : UInt64 := if huge || s >>> 1 ≥ 127 then 127 else (s >>> 1).toUInt64
    let l ← BigInt.lo x
    let h ← BigInt.hi x
    BigInt.ofWords (BigInt.sarLo l h k) (BigInt.sarHi h k)

/-- `Int.shiftLeft (x s : @& tobj) : tobj`: `x * 2^s`. Exact when shifting
back restores `x`; otherwise beyond the Int bound. -/
@[export zr_low_shim_int_shift_left]
def intShiftLeft (x s : USize) : BaseIO USize := do
  let l ← BigInt.lo x
  let h ← BigInt.hi x
  if (l ||| h) == 0 then pure 1
  else if s &&& 1 == 0 || s >>> 1 ≥ 128 then BigInt.overflow
  else
    let k := (s >>> 1).toUInt64
    let nl := BigNat.shlLo l k
    let nh := BigNat.shlHi l h k
    if BigInt.sarLo nl nh k == l && BigInt.sarHi nh k == h then BigInt.ofWords nl nh
    else BigInt.overflow

/-! ### Lists and UTF-8 slices used by the native menus -/

/-- Construct a list node, transferring both owned fields. -/
private def listCons (head tail : USize) : BaseIO USize := do
  let node ← HeaderObj.allocCtor 1 2 0
  stp (node + ctorObjs 0) head
  stp (node + ctorObjs 0 + wordBytes 0) tail
  pure node

/-- Reverse `xs` onto `tail`, consuming both. Reuse exclusively owned nodes;
shared nodes are copied without changing the shared list. -/
private partial def listReverseOnto (xs tail : USize) : BaseIO USize := do
  if xs == 1 then pure tail
  else
    let head ← ldp (xs + ctorObjs 0)
    let rest ← ldp (xs + ctorObjs 0 + wordBytes 0)
    if ← isExclusive xs then
      stp (xs + ctorObjs 0 + wordBytes 0) tail
      listReverseOnto rest xs
    else
      inc head
      inc rest
      dec xs
      listReverseOnto rest (← listCons head tail)

/-- `List.reverse._redArg (xs : obj) : obj`. -/
@[export zr_low_shim_list_reverse]
def listReverse (xs : USize) : BaseIO USize := listReverseOnto xs 1

/-- `List.appendTR._redArg (xs ys : obj) : obj`. -/
@[export zr_low_shim_list_append]
def listAppend (xs ys : USize) : BaseIO USize := do
  listReverseOnto (← listReverseOnto xs 1) ys

private partial def listRangeLoop (n acc : USize) : BaseIO USize := do
  if n == 1 then pure acc
  else
    let previous ← BigNat.sub n 3
    dec n
    inc previous
    listRangeLoop previous (← listCons previous acc)

/-- `List.range (n : tobj) : obj`. The descending counter builds the ascending
list directly and handles both immediate and heap Nats. -/
@[export zr_low_shim_list_range]
def listRange (n : USize) : BaseIO USize := listRangeLoop n 1

private partial def listReplicateLoop (n value acc : USize) : BaseIO USize := do
  if n == 1 then
    dec value
    pure acc
  else
    let previous ← BigNat.sub n 3
    dec n
    inc value
    listReplicateLoop previous value (← listCons value acc)

/-- `List.replicateTR._redArg (n : tobj) (value : obj) : obj`. -/
@[export zr_low_shim_list_replicate]
def listReplicate (n value : USize) : BaseIO USize := listReplicateLoop n value 1

private partial def listZipIdxLoop (xs index acc : USize) : BaseIO USize := do
  if xs == 1 then
    dec index
    listReverseOnto acc 1
  else
    let value ← ldp (xs + ctorObjs 0)
    let rest ← ldp (xs + ctorObjs 0 + wordBytes 0)
    inc value
    inc rest
    dec xs
    let next ← BigNat.add index 3
    let pair ← HeaderObj.allocCtor 0 2 0
    stp (pair + ctorObjs 0) value
    stp (pair + ctorObjs 0 + wordBytes 0) index
    listZipIdxLoop rest next (← listCons pair acc)

/-- `List.zipIdxTR._redArg (xs : obj) (index : @& tobj) : obj`. -/
@[export zr_low_shim_list_zip_idx]
def listZipIdx (xs index : USize) : BaseIO USize := do
  inc index
  listZipIdxLoop xs index 1

/-- Advance a byte offset by at most `n` UTF-8 scalar values. Inputs are valid
UTF-8 and offsets at scalar boundaries, as required by `String.Slice.Pos`. -/
private partial def sliceAdvance (data pos stop remaining : USize) : BaseIO USize := do
  if pos ≥ stop || remaining == 0 then pure pos
  else
    let lead ← ld8 (data + pos)
    let width : USize := if lead < 0x80 then 1 else if lead < 0xe0 then 2 else if lead < 0xf0 then 3 else 4
    sliceAdvance data (pos + width) stop (remaining - 1)

/-- `String.Slice.Pos.nextn (slice : @& obj) (pos n : tobj) : tobj`.
Positions are offsets from the slice start, not from the underlying string. -/
@[export zr_low_shim_string_slice_nextn]
def stringSliceNextN (slice pos n : USize) : BaseIO USize := do
  let text ← ldp (slice + ctorObjs 0)
  let start ← ldp (slice + ctorObjs 0 + wordBytes 0)
  let stop ← ldp (slice + ctorObjs 0 + 2 * wordBytes 0)
  let first := Arr.natToSizeSat start
  let last := Arr.natToSizeSat stop
  let relative := Arr.natToSizeSat pos
  let amount := Arr.natToSizeSat n
  let next ← sliceAdvance (text + strData 0) (first + relative) last amount
  dec pos
  dec n
  BigNat.ofWords (next - first).toUInt64 0

/-- `String.Slice.toString (slice : @& obj) : obj`. -/
@[export zr_low_shim_string_slice_to_string]
def stringSliceToString (slice : USize) : BaseIO USize := do
  let text ← ldp (slice + ctorObjs 0)
  let first := Arr.natToSizeSat (← ldp (slice + ctorObjs 0 + wordBytes 0))
  let last := Arr.natToSizeSat (← ldp (slice + ctorObjs 0 + 2 * wordBytes 0))
  let length := last - first
  let data := text + strData 0 + first
  let result ← HeaderSupport.allocString (length + 1) (length + 1) (← Str.utf8Len data 0 length 0)
  copy (result + strData 0) data length
  st8 (result + strData 0 + length) 0
  pure result

/-- `lean_string_memcmp`: equality of `length` bytes at the two borrowed
string offsets. All three offsets/counts are borrowed Nats. -/
@[export zr_low_shim_string_memcmp]
def stringMemcmp (left right leftStart rightStart length : USize) : BaseIO UInt8 := do
  let a := Arr.natToSizeSat leftStart
  let b := Arr.natToSizeSat rightStart
  let n := Arr.natToSizeSat length
  let leftSize := (← ldp (left + strSizeOff 0)) - 1
  let rightSize := (← ldp (right + strSizeOff 0)) - 1
  if a > leftSize || b > rightSize || n > leftSize - a || n > rightSize - b then return 0
  pure (if ← Str.bytesEq (left + strData 0 + a) (right + strData 0 + b) 0 n then 1 else 0)

/-- `lean_mk_string_unchecked (bytes : char const*) (size length : size_t)`.
The byte count excludes the terminating zero; input ownership is unchanged. -/
@[export zr_low_shim_mk_string_unchecked]
def mkStringUnchecked (bytes size length : USize) : BaseIO USize := do
  let result ← HeaderSupport.allocString (size + 1) (size + 1) length
  copy (result + strData 0) bytes size
  st8 (result + strData 0 + size) 0
  pure result

/-- Borrowed valid UTF-8 bytes from the native adapter's Rust `&str` boundary.
The code-point count excludes continuation bytes; no allocation is borrowed. -/
@[export zr_low_shim_mk_string_from_bytes]
def mkStringFromBytes (bytes size : USize) : BaseIO USize := do
  mkStringUnchecked bytes size (← Str.utf8Len bytes 0 size 0)

/-- `ByteArray.isEmpty (bytes : @& obj) : Bool`. -/
@[export zr_low_shim_byte_array_is_empty]
def byteArrayIsEmpty (bytes : USize) : BaseIO UInt8 := do
  pure (if (← ldp (bytes + sarrSizeOff 0)) == 0 then 1 else 0)

/-- The compiler-shared `List String` equality specialization used by menu
state. Both lists and their strings are borrowed. -/
@[export zr_low_shim_list_string_eq]
partial def listStringEq (left right : USize) : BaseIO UInt8 := do
  if left == right then return 1
  if left == 1 || right == 1 then return 0
  let a ← ldp (left + ctorObjs 0)
  let b ← ldp (right + ctorObjs 0)
  if (← ldp (a + strSizeOff 0)) != (← ldp (b + strSizeOff 0)) then return 0
  if (← Str.eqCold a b) == 0 then return 0
  listStringEq (← ldp (left + ctorObjs 0 + wordBytes 0)) (← ldp (right + ctorObjs 0 + wordBytes 0))

end ZrLean.Low.InitShim
