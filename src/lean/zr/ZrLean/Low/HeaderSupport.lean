import ZrLean.Low.HeaderObj
import ZrLean.Low.HeaderRc
import ZrLean.Low.Apply

/-! Scalar runtime-header support used by fixture oracles as well as firmware.
Addresses are raw words; borrowed getters do not adjust reference counts and
raw setters transfer fields without releasing their previous contents. Byte
sizes include object headers, using capacity for allocation size and live
size for data size. All memory is CPU-owned, never DMA or MMIO. -/
namespace ZrLean.Low.HeaderSupport
open ZrLean.Low

@[export zr_low_header_to_string]
def toStringObject (o : USize) : USize := o

@[inline, export zr_low_header_string_capacity]
def stringCapacity (o : USize) : BaseIO USize := HeaderMem.ldp (o + Obj.strCapOff 0)
@[inline, export zr_low_header_string_size]
def stringSize (o : USize) : BaseIO USize := HeaderMem.ldp (o + Obj.strSizeOff 0)
@[export zr_low_header_string_len]
def stringLen (o : USize) : BaseIO USize := HeaderMem.ldp (o + Obj.strCapOff 0 + wordBytes 0)
@[export zr_low_header_string_cstr]
def stringCstr (o : USize) : USize := o + Obj.strData 0
@[export zr_low_header_string_byte_size]
def stringByteSize (o : USize) : BaseIO USize := do pure (Obj.strData 0 + (← stringCapacity o))
@[export zr_low_header_string_data_byte_size]
def stringDataByteSize (o : USize) : BaseIO USize := do pure (Obj.strData 0 + (← stringSize o))

@[export zr_low_header_alloc_string]
def allocString (size capacity length : USize) : BaseIO USize := do
  let o ← Obj.allocObj (← HeaderObj.addChecked (Obj.strData 0) capacity)
  HeaderMem.st32 o 1
  HeaderMem.st32 (o + 4) 0xf9000000
  HeaderMem.stp (o + Obj.strSizeOff 0) size
  HeaderMem.stp (o + Obj.strCapOff 0) capacity
  HeaderMem.stp (o + Obj.strCapOff 0 + wordBytes 0) length
  pure o

@[inline, export zr_low_header_array_capacity]
def arrayCapacity (o : USize) : BaseIO USize := HeaderMem.ldp (o + Obj.arrCapOff 0)
@[export zr_low_header_array_byte_size]
def arrayByteSize (o : USize) : BaseIO USize := do
  pure (Obj.arrData 0 + wordBytes 0 * (← arrayCapacity o))
@[export zr_low_header_array_data_byte_size]
def arrayDataByteSize (o : USize) : BaseIO USize := do
  pure (Obj.arrData 0 + wordBytes 0 * (← HeaderMem.ldp (o + Obj.arrSizeOff 0)))
@[export zr_low_header_array_set_size]
def arraySetSize (o size : USize) : BaseIO Unit := HeaderMem.stp (o + Obj.arrSizeOff 0) size
@[export zr_low_header_array_set_core]
def arraySetCore (o i v : USize) : BaseIO Unit := HeaderMem.stp (o + Obj.arrData 0 + i * wordBytes 0) v

@[inline, export zr_low_header_sarray_elem_size]
def sarrayElemSize (o : USize) : BaseIO UInt32 := do
  pure ((← HeaderMem.ld8 (o + 6)) &&& 0x7f).toUInt32
@[export zr_low_header_sarray_byte_size]
def sarrayByteSize (o : USize) : BaseIO USize := do
  pure (Obj.sarrData 0 + (← sarrayElemSize o).toUSize * (← HeaderMem.ldp (o + Obj.sarrCapOff 0)))
@[export zr_low_header_sarray_data_byte_size]
def sarrayDataByteSize (o : USize) : BaseIO USize := do
  pure (Obj.sarrData 0 + (← sarrayElemSize o).toUSize * (← HeaderMem.ldp (o + Obj.sarrSizeOff 0)))
@[export zr_low_header_sarray_set_size]
def sarraySetSize (o size : USize) : BaseIO Unit := HeaderMem.stp (o + Obj.sarrSizeOff 0) size

@[export zr_low_header_closure_fun]
def closureFun (o : USize) : BaseIO USize := HeaderMem.ldp (o + Apply.funOff 0)
@[export zr_low_header_closure_arity]
def closureArity (o : USize) : BaseIO UInt32 := do
  pure (← HeaderMem.ld16 (o + Apply.arityOff 0)).toUInt32
@[inline, export zr_low_header_closure_num_fixed]
def closureNumFixed (o : USize) : BaseIO UInt32 := do
  pure (← HeaderMem.ld16 (o + Obj.cloFixedOff 0)).toUInt32
@[export zr_low_header_closure_arg_cptr]
def closureArgCptr (o : USize) : USize := o + Obj.cloObjs 0
@[export zr_low_header_closure_get]
def closureGet (o : USize) (i : UInt32) : BaseIO USize :=
  HeaderMem.ldp (o + Obj.cloObjs 0 + i.toUSize * wordBytes 0)
@[inline, export zr_low_header_closure_byte_size]
def closureByteSize (o : USize) : BaseIO USize := do
  pure (Obj.cloObjs 0 + wordBytes 0 * (← closureNumFixed o).toUSize)
@[export zr_low_header_closure_data_byte_size]
def closureDataByteSize (o : USize) : BaseIO USize := closureByteSize o

@[inline, export zr_low_header_ctor_num_objs]
def ctorNumObjs (o : USize) : BaseIO UInt32 := do pure (← HeaderMem.ld8 (o + 6)).toUInt32
@[export zr_low_header_ctor_scalar_cptr]
def ctorScalarCptr (o : USize) : BaseIO USize := do
  pure (o + Obj.ctorObjs 0 + (← ctorNumObjs o).toUSize * wordBytes 0)
@[export zr_low_header_ctor_release]
def ctorRelease (o : USize) (i : UInt32) : BaseIO Unit := do
  let field := o + Obj.ctorObjs 0 + i.toUSize * wordBytes 0
  HeaderRc.dec (← HeaderMem.ldp field)
  HeaderMem.stp field 1

@[inline, export zr_low_header_is_marked_linear_core]
def isMarkedLinearCore (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 6)) &&& 0x80 != 0)
@[inline, export zr_low_header_mark_linear_core]
def markLinearCore (o : USize) : BaseIO Unit := do
  HeaderMem.st8 (o + 6) ((← HeaderMem.ld8 (o + 6)) ||| 0x80)

@[export zr_low_header_is_ctor]
def isCtor (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) ≤ 243)

@[export zr_low_header_is_closure]
def isClosure (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 245)

@[export zr_low_header_is_array]
def isArray (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 246)

@[export zr_low_header_is_sarray]
def isSArray (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 248)

@[export zr_low_header_is_string]
def isString (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 249)

@[export zr_low_header_is_mpz]
def isMpz (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 250)

@[export zr_low_header_is_thunk]
def isThunk (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 251)

@[export zr_low_header_is_task]
def isTask (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 252)

@[export zr_low_header_is_promise]
def isPromise (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 244)

@[export zr_low_header_is_external]
def isExternal (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 254)

@[export zr_low_header_is_ref]
def isRef (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld8 (o + 7)) == 253)

@[export zr_low_header_array_is_marked_linear]
def arrayIsMarkedLinear (o : USize) : BaseIO Bool := isMarkedLinearCore o
@[export zr_low_header_array_mark_linear_core]
def arrayMarkLinearCore (o : USize) : BaseIO Unit := markLinearCore o

@[export zr_low_header_sarray_is_marked_linear]
def sarrayIsMarkedLinear (o : USize) : BaseIO Bool := isMarkedLinearCore o
@[export zr_low_header_sarray_mark_linear_core]
def sarrayMarkLinearCore (o : USize) : BaseIO Unit := markLinearCore o

@[export zr_low_header_string_is_marked_linear]
def stringIsMarkedLinear (o : USize) : BaseIO Bool := isMarkedLinearCore o
@[export zr_low_header_string_mark_linear_core]
def stringMarkLinearCore (o : USize) : BaseIO Unit := markLinearCore o

end ZrLean.Low.HeaderSupport
