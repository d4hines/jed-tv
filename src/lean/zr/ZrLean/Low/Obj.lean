/-
The Lean runtime's objects in ZrLow, part 1 of 4 (AGENTS.md, "The
runtime's objects"): the layout of lean.h's objects (as offsets the C compiler
computes, so they are right for the board's 32 bits and the host's 64), the
header, the sizes of the blocks the allocator hands back, and the few
primitives the rest shares: a signed compare, a copy, the fatal stop and the
cost model's counters.

An object is an address (`USize`); a field is a `Mem` access at an offset.
The header is `{int rc; u16 cs_sz; u8 other; u8 tag}`: the count at +0, the
tag at +7 and `other` (a constructor's field count, a scalar array's element
size, the linearity bit) at +6; test/leanrt/rt/src/layout.rs checks this
against the C compiler's bitfield layout. No allocation, no boxing, no
reference count of its own: checked by `zr-cgen check`.
-/
import ZrLean.Low.Alloc

namespace ZrLean.Low.Obj
open ZrLean.Low ZrLean.Low.Alloc

/-! ### Layout (`lean/lean.h`), as the C compiler computes it -/

@[never_extract, extern "zr_prim_low_ctor_objs"]
opaque ctorObjs (z : USize) : USize
@[never_extract, extern "zr_prim_low_arr_size_off"]
opaque arrSizeOff (z : USize) : USize
@[never_extract, extern "zr_prim_low_arr_cap_off"]
opaque arrCapOff (z : USize) : USize
/-- `sizeof(lean_array_object)`: the header, and where the elements start. -/
@[never_extract, extern "zr_prim_low_arr_data"]
opaque arrData (z : USize) : USize
@[never_extract, extern "zr_prim_low_sarr_size_off"]
opaque sarrSizeOff (z : USize) : USize
@[never_extract, extern "zr_prim_low_sarr_cap_off"]
opaque sarrCapOff (z : USize) : USize
@[never_extract, extern "zr_prim_low_sarr_data"]
opaque sarrData (z : USize) : USize
@[never_extract, extern "zr_prim_low_str_size_off"]
opaque strSizeOff (z : USize) : USize
@[never_extract, extern "zr_prim_low_str_cap_off"]
opaque strCapOff (z : USize) : USize
@[never_extract, extern "zr_prim_low_str_data"]
opaque strData (z : USize) : USize
@[never_extract, extern "zr_prim_low_clo_fixed_off"]
opaque cloFixedOff (z : USize) : USize
@[never_extract, extern "zr_prim_low_clo_objs"]
opaque cloObjs (z : USize) : USize

/- Tags (`lean.h`; zr_runtime.c asserts them) are written as literals, since a
constant def is a module constant (AGENTS.md): constructors up to 243,
closure 245, array 246, scalar array 248, string 249, big Nat 250, reference 253. The
linearity marker in `other` is 0x80 (`LEAN_LINEAR_MARK_MASK`). -/

@[inline] def tagOf (o : USize) : BaseIO UInt8 := ld8 (o + 7)
@[inline] def otherOf (o : USize) : BaseIO UInt8 := ld8 (o + 6)

/-- The header's second word `cs_sz | other << 16 | tag << 24` of an array
(tag 246, `other` 0) and of a scalar array (tag 248, `other` the element
size). Constants as C expressions: arithmetic on literals would be a closed
term (AGENTS.md). -/
@[never_extract, extern "zr_prim_low_array_header"]
opaque arrayHeader (z : USize) : UInt32
@[never_extract, extern "zr_prim_low_sarray_header"]
opaque sarrayHeader (elem : USize) : UInt32

/-- `lean_set_st_header`: count 1, then `word`, the second header word. -/
@[inline] def setHeader (o : USize) (word : UInt32) : BaseIO Unit := do
  st32 o 1
  st32 (o + 4) word

/-- `lean_is_exclusive`: a single-threaded object with count 1. -/
@[inline] def isExclusive (o : USize) : BaseIO Bool := do
  pure ((← ld32 o) == 1)

/-- A scalar array's element size (`other` without the linearity bit). -/
@[inline] def elemSize (o : USize) : BaseIO USize := do
  pure ((← otherOf o) &&& 0x7f).toUSize

/-! ### Sizes of blocks (`lean_*_byte_size`), for `dealloc` -/

@[inline] def arrayBytes (o : USize) : BaseIO USize := do
  pure (arrData 0 + wordBytes 0 * (← ldp (o + arrCapOff 0)))
@[inline] def sarrayBytes (o : USize) : BaseIO USize := do
  pure (sarrData 0 + (← elemSize o) * (← ldp (o + sarrCapOff 0)))
@[inline] def stringBytes (o : USize) : BaseIO USize := do
  pure (strData 0 + (← ldp (o + strCapOff 0)))
@[inline] def closureBytes (o : USize) : BaseIO USize := do
  pure (cloObjs 0 + wordBytes 0 * (← ld16 (o + cloFixedOff 0)).toUSize)

/-! ### Primitives -/

/-- Signed `a > b` on the bits of a count (`int m_rc`). -/
@[extern "zr_prim_low_sgt32"]
opaque sgt32 (a b : UInt32) : Bool

/-- The all-ones word (`SIZE_MAX`), as a function of a dummy argument (a
constant `0 - 1` would be a closed term, AGENTS.md). -/
@[never_extract, extern "zr_prim_low_size_max"]
opaque sizeMax (z : USize) : USize

/-- Overflow of the sum or product (the builtins `lean.h`'s `*_checked` use). -/
@[extern "zr_prim_low_mul_ovf"]
opaque mulOvf (a b : USize) : Bool

/-- Copy `n` bytes from `s` to `d` (the platform's `memcpy`; with the test
runtime's cost counters, a counted one). -/
@[extern "zr_prim_low_copy"]
opaque copy (d s n : USize) : BaseIO Unit

/-- `memmove`: the same, for regions that may overlap. -/
@[extern "zr_prim_low_move"]
opaque move (d s n : USize) : BaseIO Unit

/-- The test runtime's counter of a copy of a shared array (a no-op on the board). -/
@[extern "zr_prim_low_count_nonlinear"]
opaque countNonlinear (bytes : USize) : BaseIO Unit

/-- The test runtime's counter of heap bytes allocated (a no-op on the board). -/
@[extern "zr_prim_low_count_alloc"]
opaque countAllocBytes (bytes : USize) : BaseIO Unit

/-- Stop the program for reason `code` (`zr_low_fatal`, zr_runtime.c: 0 out of
memory, 1 freeing an object kind the runtime does not support, 2 the same
for `mark_persistent`, 3 integer overflow in a size). Does not return. -/
@[extern "zr_prim_low_fatal"]
opaque fatal (code : USize) : BaseIO USize

/-- The link of the deletion list, kept in the dead object's header: on 32
bits the count word, on 64 the low 48 bits of the header word (`other` and
`tag` stay, they are still needed to find the object's children and size). -/
@[inline] def setLink (o next : USize) : BaseIO Unit := do
  if wordBytes 0 > 4 then
    let mask := sizeMax 0 >>> 16
    stp o (((← ldp o) &&& ~~~mask) ||| (next &&& mask))
  else stp o next

/-- The deletion list link of `o`. -/
@[inline] def getLink (o : USize) : BaseIO USize := do
  let link ← ldp o
  if wordBytes 0 > 4 then pure (link &&& (sizeMax 0 >>> 16))
  else pure link

/-! ### Allocation and the checked sizes -/

/-- A block of the runtime's heap for an object of `size` bytes whose size
the runtime recomputes when freeing it (`lean_alloc_object`). -/
@[inline] def allocObj (size : USize) : BaseIO USize := do
  countAllocBytes size
  allocObject (theHeap 0) size

/-- `hdr + elem * cap`, stopping the program on overflow (`lean_usize_add_checked`
of `lean_usize_mul_checked`). -/
@[inline] def checkedBytes (hdr elem cap : USize) : BaseIO USize := do
  if mulOvf elem cap then fatal 3
  else
    let b := hdr + elem * cap
    if b < hdr then fatal 3 else pure b

/-- `lean_alloc_array size cap`: a fresh array object, count 1 (tag 246). -/
@[inline] def allocArray (size cap : USize) : BaseIO USize := do
  let o ← allocObj (← checkedBytes (arrData 0) (wordBytes 0) cap)
  setHeader o (arrayHeader 0)
  stp (o + arrSizeOff 0) size
  stp (o + arrCapOff 0) cap
  pure o

/-- `lean_alloc_sarray elem size cap` (`elem` below 128; tag 248). -/
@[inline] def allocSArray (elem size cap : USize) : BaseIO USize := do
  let o ← allocObj (← checkedBytes (sarrData 0) elem cap)
  setHeader o (sarrayHeader elem)
  stp (o + sarrSizeOff 0) size
  stp (o + sarrCapOff 0) cap
  pure o

/-- Copy the linearity mark of `src` to the new object `dst`. -/
@[inline] def copyLinearMark (src dst : USize) : BaseIO Unit := do
  if (← otherOf src) &&& 0x80 != 0 then
    st8 (dst + 6) ((← otherOf dst) ||| 0x80)

end ZrLean.Low.Obj
