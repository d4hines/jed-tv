/-
Arrays, scalar arrays (ByteArray) and `String.toUTF8`, part 4 of 4
(AGENTS.md, "The runtime's objects"): the out-of-line functions `lean.h`'s
inline `Array.push`, `ByteArray.push`, `Array.uset` and the like call when the
array has to grow or is shared, with the exports for the C forwarders in
src/c/zr_runtime.c. Same algorithms, order of operations and object layout as
the C they replace (oracle: test/leanrt/csrc/zr_rc_oracle.c), including the
test runtime's cost counters.
-/
import ZrLean.Low.Rc
import ZrLean.Low.Fast

namespace ZrLean.Low.Arr
open ZrLean.Low ZrLean.Low.Alloc ZrLean.Low.Obj ZrLean.Low.Rc

/-! ### Array -/

/-- Copy the `n` elements at `src` to `dst`, taking a reference on each. -/
partial def incCopy (src dst i n : USize) : BaseIO Unit := do
  if i < n then
    let x ← ldp (src + i * wordBytes 0)
    inc x
    stp (dst + i * wordBytes 0) x
    incCopy src dst (i + 1) n

/-- `lean_copy_expand_array`: a copy of `a` (consumed) with capacity
`(cap + 1) * 2` if `expand` is not 0. The functions here are the exports (C
`size_t`/`uint8_t` ABIs; the flags are the C `bool`s, and `copySlice`'s last
argument is a byte, matching lean.h including when passed on the stack). -/
@[export zr_low_copy_expand_array]
def copyExpandArray (a : USize) (expand : UInt8) : BaseIO USize := do
  let sz ← ldp (a + arrSizeOff 0)
  let cap0 ← ldp (a + arrCapOff 0)
  let cap := if expand != 0 then (cap0 + 1) * 2 else cap0
  let r ← allocArray sz cap
  copyLinearMark a r
  if ← isExclusive a then
    -- move the elements: no inc/dec
    copy (r + arrData 0) (a + arrData 0) (sz * wordBytes 0)
    deallocSized a (arrData 0 + wordBytes 0 * cap0)
  else
    incCopy (a + arrData 0) (r + arrData 0) 0 sz
    decRef a
  pure r

/-- `lean_copy_expand_array_nonlinear`: the copy of a shared array. -/
@[export zr_low_copy_expand_array_nonlinear]
def copyExpandArrayNonlinear (a : USize) (expand : UInt8) : BaseIO USize := do
  countNonlinear ((← ldp (a + arrSizeOff 0)) * wordBytes 0)
  copyExpandArray a expand

/-- `lean_array_push`: `Array.push`, in place when `a` is exclusive with room. -/
@[export zr_low_array_push_slow]
def arrayPushSlow (a v : USize) : BaseIO USize := do
  let sz ← ldp (a + arrSizeOff 0)
  let cap ← ldp (a + arrCapOff 0)
  let r ← if ← isExclusive a then
      (if cap > sz then pure a else copyExpandArray a 1)
    else copyExpandArrayNonlinear a (if cap < 2 * sz + 1 then 1 else 0)
  let rsz ← ldp (r + arrSizeOff 0)
  stp (r + arrData 0 + rsz * wordBytes 0) v
  stp (r + arrSizeOff 0) (rsz + 1)
  pure r

@[export zr_low_array_push]
def arrayPush (a v : USize) : BaseIO USize := Fast.arrayPush a v

/-- Fill `r[i..n]` with `v`. -/
partial def fill (r v i n : USize) : BaseIO Unit := do
  if i < n then
    stp (r + arrData 0 + i * wordBytes 0) v
    fill r v (i + 1) n

/-- `lean_mk_array`: `Array.replicate n v` (`n` a boxed Nat). -/
@[export zr_low_mk_array]
def mkArray (n v : USize) : BaseIO USize := do
  -- a boxed Nat (> 2^31 - 1 on 32-bit) cannot be an element count here
  if n &&& 1 == 0 then fatal 0
  else
    let sz := n >>> 1
    let r ← allocArray sz sz
    fill r v 0 sz
    if sz == 0 then dec v
    else if sz > 1 then
      if v &&& 1 == 0 then incRefN v (sz - 1)
    pure r

/-! ### Scalar arrays -/

/-- `lean_copy_sarray`: a copy of the scalar array `a` (consumed) with
capacity `cap`. -/
@[export zr_low_copy_sarray]
def copySArray (a cap : USize) : BaseIO USize := do
  let esz ← elemSize a
  let sz ← ldp (a + sarrSizeOff 0)
  let r ← allocSArray esz sz cap
  copyLinearMark a r
  copy (r + sarrData 0) (a + sarrData 0) (esz * sz)
  dec a
  pure r

/-- `lean_copy_sarray_nonlinear`: the same for a shared array. -/
@[export zr_low_copy_sarray_nonlinear]
def copySArrayNonlinear (a cap : USize) : BaseIO USize := do
  countNonlinear ((← ldp (a + sarrSizeOff 0)) * (← elemSize a))
  copySArray a cap

/-- `lean_sarray_ensure_exclusive`. -/
@[inline] def sarrayEnsureExclusive (a : USize) : BaseIO USize := do
  if ← isExclusive a then pure a
  else copySArrayNonlinear a (← ldp (a + sarrCapOff 0))

/-- `lean_sarray_ensure_capacity`: copy `a` unless it holds `minCap` elements. -/
@[export zr_low_sarray_ensure_capacity]
def sarrayEnsureCapacity (a minCap : USize) (exact : UInt8) : BaseIO USize := do
  if minCap ≤ (← ldp (a + sarrCapOff 0)) then pure a
  else
    let cap := if exact != 0 then minCap else minCap * 2
    if ← isExclusive a then copySArray a cap
    else copySArrayNonlinear a cap

/-- `lean_byte_array_push`. -/
@[export zr_low_byte_array_push_slow]
def byteArrayPushSlow (a : USize) (b : UInt8) : BaseIO USize := do
  let r ← sarrayEnsureExclusive (← sarrayEnsureCapacity a ((← ldp (a + sarrSizeOff 0)) + 1) 0)
  let sz ← ldp (r + sarrSizeOff 0)
  st8 (r + sarrData 0 + sz) b
  stp (r + sarrSizeOff 0) (sz + 1)
  pure r

@[export zr_low_byte_array_push]
def byteArrayPush (a : USize) (b : UInt8) : BaseIO USize := Fast.byteArrayPush a b

/-- Store `src[i..n]` (boxed bytes) as bytes at `dst`. -/
partial def unboxBytes (src dst i n : USize) : BaseIO Unit := do
  if i < n then
    st8 (dst + i) ((← ldp (src + i * wordBytes 0)) >>> 1).toUInt8
    unboxBytes src dst (i + 1) n

/-- `lean_byte_array_mk`: `ByteArray.mk` from an `Array UInt8` (consumed). -/
@[export zr_low_byte_array_mk]
def byteArrayMk (a : USize) : BaseIO USize := do
  let sz ← ldp (a + arrSizeOff 0)
  let r ← allocSArray 1 sz sz
  unboxBytes (a + arrData 0) (r + sarrData 0) 0 sz
  dec a
  pure r

/-- A Nat argument as a size, saturating (a boxed Nat exceeds any size). -/
@[inline] def natToSizeSat (n : USize) : USize :=
  if n &&& 1 != 0 then n >>> 1 else sizeMax 0

/-- `lean_byte_array_copy_slice`: `ByteArray.copySlice src srcOff dest destOff
len exact` (`dest` consumed). -/
@[export zr_low_byte_array_copy_slice]
def copySlice (src oSrcOff dest oDestOff oLen : USize) (exact : UInt8) : BaseIO USize := do
  let ssz ← ldp (src + sarrSizeOff 0)
  let dsz ← ldp (dest + sarrSizeOff 0)
  let srcOff := natToSizeSat oSrcOff
  if srcOff > ssz then return dest
  let len := natToSizeSat oLen
  let len := if len > ssz - srcOff then ssz - srcOff else len
  let destOff := natToSizeSat oDestOff
  let destOff := if destOff > dsz then dsz else destOff
  let newDsz := if destOff + len > dsz then destOff + len else dsz
  let r ← sarrayEnsureExclusive (← sarrayEnsureCapacity dest newDsz (if exact != 0 then 1 else 0))
  stp (r + sarrSizeOff 0) newDsz
  let from_ := src + sarrData 0 + srcOff
  let to := r + sarrData 0 + destOff
  -- r is normally distinct from src (exclusive while the caller still holds
  -- src); if a caller passed one unshared array as both, the copy may overlap
  if r == src then move to from_ len else copy to from_ len
  pure r

/-! ### Strings -/

/-- `lean_string_to_utf8`: `String.toUTF8` (`s` borrowed). -/
@[export zr_low_string_to_utf8]
def stringToUtf8 (s : USize) : BaseIO USize := do
  let n := (← ldp (s + strSizeOff 0)) - 1
  let r ← allocSArray 1 n n
  copy (r + sarrData 0) (s + strData 0) n
  pure r

end ZrLean.Low.Arr
