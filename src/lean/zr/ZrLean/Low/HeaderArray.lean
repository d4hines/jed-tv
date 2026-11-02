import ZrLean.Low.Arr
import ZrLean.Low.Runtime
import ZrLean.Low.HeaderMem

/-! Header array/closure algorithms over scalar pointers. HeaderMem accesses
CPU-owned heap fields without volatile semantics; DMA/MMIO primitives stay
separate. Bounds fallbacks, consumption, reference counts and copy-on-write
use the same runtime cold paths as the original ABI. -/
namespace ZrLean.Low.HeaderArray
open ZrLean.Low

@[never_extract, extern "zr_header_sarray_bad_elem_message"]
opaque badElemMessage (dummy : UInt8) : USize

@[inline] private def exclusive (a : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld32 a) == 1)

@[export zr_low_header_to_array]
def toArray (a : USize) : USize := a
@[export zr_low_header_to_sarray]
def toSArray (a : USize) : USize := a
@[export zr_low_header_to_closure]
def toClosure (a : USize) : USize := a

@[export zr_low_header_alloc_array]
def allocArray (size capacity : USize) : BaseIO USize :=
  Obj.allocArray size capacity

@[export zr_low_header_array_size]
def arraySize (a : USize) : BaseIO USize := HeaderMem.ldp (a + Obj.arrSizeOff 0)

@[export zr_low_header_array_cptr]
def arrayCptr (a : USize) : USize := a + Obj.arrData 0

@[export zr_low_header_array_get_core]
def arrayGetCore (a i : USize) : BaseIO USize :=
  HeaderMem.ldp (arrayCptr a + i * wordBytes 0)

@[export zr_low_header_array_get_size]
def arrayGetSize (a : USize) : BaseIO USize := do
  pure (((← arraySize a) <<< 1) ||| 1)

@[export zr_low_header_mk_empty_array_with_capacity]
def mkEmptyArrayWithCapacity (capacity : USize) : BaseIO USize := do
  if capacity &&& 1 == 0 then Obj.fatal 0
  else allocArray 0 (capacity >>> 1)

@[export zr_low_header_array_uget]
def arrayUget (a i : USize) : BaseIO USize := do
  let r ← arrayGetCore a i
  Rc.inc r
  pure r

@[export zr_low_header_array_uget_borrowed]
def arrayUgetBorrowed (a i : USize) : BaseIO USize := arrayGetCore a i

@[export zr_low_header_array_fget]
def arrayFget (a i : USize) : BaseIO USize := arrayUget a (i >>> 1)

@[export zr_low_header_array_fget_borrowed]
def arrayFgetBorrowed (a i : USize) : BaseIO USize := arrayGetCore a (i >>> 1)

@[export zr_low_header_array_get]
def arrayGet (default a i : USize) : BaseIO USize := do
  if i &&& 1 != 0 then
    let index := i >>> 1
    if index < (← arraySize a) then return ← arrayUget a index
  Rc.inc default
  Runtime.getPanic default

@[export zr_low_header_array_get_borrowed]
def arrayGetBorrowed (default a i : USize) : BaseIO USize := do
  if i &&& 1 != 0 then
    let index := i >>> 1
    if index < (← arraySize a) then return ← arrayGetCore a index
  Rc.inc default
  Runtime.getPanic default

@[export zr_low_header_ensure_exclusive_array]
def ensureExclusiveArray (a : USize) : BaseIO USize := do
  if ← exclusive a then pure a else Arr.copyExpandArrayNonlinear a 0

@[export zr_low_header_array_uset]
def arrayUset (a i v : USize) : BaseIO USize := do
  let r ← ensureExclusiveArray a
  let slot := arrayCptr r + i * wordBytes 0
  Rc.dec (← HeaderMem.ldp slot)
  HeaderMem.stp slot v
  pure r

@[export zr_low_header_array_fset]
def arrayFset (a i v : USize) : BaseIO USize := arrayUset a (i >>> 1) v

@[export zr_low_header_array_set]
def arraySet (a i v : USize) : BaseIO USize := do
  if i &&& 1 != 0 then
    let index := i >>> 1
    if index < (← arraySize a) then return ← arrayUset a index v
  Runtime.setPanic a v

@[export zr_low_header_alloc_sarray]
def allocSArray (elem : UInt32) (size capacity : USize) : BaseIO USize := do
  if elem >= 128 then Runtime.internalPanic (badElemMessage 0)
  else Obj.allocSArray elem.toUSize size capacity

@[export zr_low_header_sarray_size]
def sarraySize (a : USize) : BaseIO USize := HeaderMem.ldp (a + Obj.sarrSizeOff 0)

@[export zr_low_header_sarray_capacity]
def sarrayCapacity (a : USize) : BaseIO USize := HeaderMem.ldp (a + Obj.sarrCapOff 0)

@[export zr_low_header_sarray_cptr]
def sarrayCptr (a : USize) : USize := a + Obj.sarrData 0

@[export zr_low_header_sarray_ensure_exclusive]
def sarrayEnsureExclusive (a : USize) : BaseIO USize := do
  if ← exclusive a then pure a else Arr.copySArrayNonlinear a (← sarrayCapacity a)

@[export zr_low_header_mk_empty_byte_array]
def mkEmptyByteArray (capacity : USize) : BaseIO USize := do
  if capacity &&& 1 == 0 then Obj.fatal 0
  else allocSArray 1 0 (capacity >>> 1)

@[export zr_low_header_byte_array_size]
def byteArraySize (a : USize) : BaseIO USize := do
  pure (((← sarraySize a) <<< 1) ||| 1)

/-- The terminal byte-field access retains the scalar-array pointer's
provenance, permitting adjacent byte reads to combine into a word load.
The caller already establishes the unchecked index's bounds. -/
@[extern "zr_prim_sarray_byte_get"]
private opaque scalarArrayByte (a i : USize) : BaseIO UInt8

@[export zr_low_header_byte_array_uget]
def byteArrayUget (a i : USize) : BaseIO UInt8 := scalarArrayByte a i

@[export zr_low_header_byte_array_get]
def byteArrayGet (a i : USize) : BaseIO UInt8 := do
  if i &&& 1 != 0 then
    let index := i >>> 1
    if index < (← sarraySize a) then return ← byteArrayUget a index
  pure 0

@[export zr_low_header_byte_array_fget]
def byteArrayFget (a i : USize) : BaseIO UInt8 := byteArrayUget a (i >>> 1)

@[export zr_low_header_byte_array_uset]
def byteArrayUset (a i : USize) (v : UInt8) : BaseIO USize := do
  let r ← sarrayEnsureExclusive a
  HeaderMem.st8 (sarrayCptr r + i) v
  pure r

@[export zr_low_header_byte_array_set]
def byteArraySet (a i : USize) (v : UInt8) : BaseIO USize := do
  if i &&& 1 != 0 then
    let index := i >>> 1
    if index < (← sarraySize a) then return ← byteArrayUset a index v
  pure a

@[export zr_low_header_alloc_closure]
def allocClosure (funPtr : USize) (arity fixed : UInt32) : BaseIO USize := do
  let o ← Obj.allocObj (← Obj.checkedBytes (Obj.cloObjs 0) (wordBytes 0) fixed.toUSize)
  HeaderMem.st32 o 1
  HeaderMem.st32 (o + 4) 0xf5000000
  HeaderMem.stp (o + Obj.cloFixedOff 0 - 2 - wordBytes 0) funPtr
  HeaderMem.st16 (o + Obj.cloFixedOff 0 - 2) arity.toUInt16
  HeaderMem.st16 (o + Obj.cloFixedOff 0) fixed.toUInt16
  pure o

@[export zr_low_header_closure_set]
def closureSet (o : USize) (i : UInt32) (a : USize) : BaseIO Unit :=
  HeaderMem.stp (o + Obj.cloObjs 0 + i.toUSize * wordBytes 0) a

end ZrLean.Low.HeaderArray
