import ZrLean.Low.Obj
import ZrLean.Low.HeaderMem
import ZrLean.Low.Rc

/- Constructor/header algorithms formerly implemented inline in lean.h.
   Object addresses have a scalar ABI so bootstrapping these operations does
   not allocate or depend on the very object operations being implemented. -/
namespace ZrLean.Low.HeaderObj
open ZrLean.Low

@[inline] def box (n : USize) : USize := (n <<< 1) ||| 1
@[inline] def unbox (o : USize) : USize := o >>> 1
@[inline] def isScalar (o : USize) : Bool := o &&& 1 == 1

@[export zr_low_header_obj_tag]
def objectTag (o : USize) : BaseIO UInt32 := do
  if isScalar o then pure (unbox o).toUInt32
  else pure (← HeaderMem.ld8 (o + 7)).toUInt32

@[export zr_low_header_ptr_tag]
def pointerTag (o : USize) : BaseIO UInt8 := HeaderMem.ld8 (o + 7)
@[export zr_low_header_ptr_other]
def pointerOther (o : USize) : BaseIO UInt32 := do
  pure (← HeaderMem.ld8 (o + 6)).toUInt32

@[inline] def addChecked (a b : USize) : BaseIO USize := do
  let r := a + b
  if r < a then Obj.fatal 3 else pure r

@[inline] def mulChecked (a b : USize) : BaseIO USize := do
  if Obj.mulOvf a b then Obj.fatal 3 else pure (a * b)

@[export zr_low_header_usize_add_checked]
def addCheckedFfi (a b : USize) : BaseIO USize := addChecked a b
@[export zr_low_header_usize_mul_checked]
def mulCheckedFfi (a b : USize) : BaseIO USize := mulChecked a b

@[inline] def setStHeader (o : USize) (tag other : UInt32) : BaseIO Unit := do
  HeaderMem.st32 o 1
  HeaderMem.st32 (o + 4) ((tag.toUInt8.toUInt32 <<< 24) ||| (other.toUInt8.toUInt32 <<< 16))

@[export zr_low_header_set_st_header]
def setStHeaderFfi (o : USize) (tag other : UInt32) : BaseIO Unit :=
  setStHeader o tag other

@[export zr_low_header_set_non_heap_header]
def setNonHeapHeader (o sz : USize) (tag other : UInt32) : BaseIO Unit := do
  HeaderMem.st32 o 0
  HeaderMem.st32 (o + 4) ((tag.toUInt8.toUInt32 <<< 24) |||
    (other.toUInt8.toUInt32 <<< 16) ||| sz.toUInt16.toUInt32)

@[export zr_low_header_set_non_heap_header_for_big]
def setNonHeapHeaderForBig (o : USize) (tag other : UInt32) : BaseIO Unit :=
  setNonHeapHeader o 1 tag other

@[export zr_low_header_align]
def align (v a : USize) : USize :=
  (v / a) * a + a * (if v % a != 0 then 1 else 0)

@[export zr_low_header_get_slot_idx]
def slotIndex (sz : UInt32) : UInt32 := sz / 8 - 1

@[extern "zr_low_alloc_small"]
private opaque allocSmallCall (sz : USize) : BaseIO USize

@[inline] def allocSmall (sz : UInt32) : BaseIO USize := do
  let rounded := ((sz + 7) &&& 0xfffffff8).toUSize
  Obj.countAllocBytes rounded
  allocSmallCall rounded

@[export zr_low_header_alloc_small_object]
def allocSmallFfi (sz : UInt32) : BaseIO USize := allocSmall sz

@[inline] def allocCtorMemory (sz : UInt32) : BaseIO USize := do
  let rounded := (sz + 7) &&& 0xfffffff8
  let o ← allocSmall sz
  if rounded > sz then
    HeaderMem.stp (o + rounded.toUSize - wordBytes 0) 0
  pure o

@[export zr_low_header_alloc_ctor_memory]
def allocCtorMemoryFfi (sz : UInt32) : BaseIO USize := allocCtorMemory sz

@[export zr_low_header_small_object_size]
def smallObjectSize (o : USize) : BaseIO UInt32 := do
  pure (← Alloc.sizeOf o).toUInt32

@[inline] def ctorData (o : USize) : USize := o + Obj.ctorObjs 0
@[inline] def ctorField (o i : USize) : USize := ctorData o + i * wordBytes 0

@[export zr_low_header_alloc_ctor]
def allocCtor (tag numObjs scalarBytes : UInt32) : BaseIO USize := do
  let fields ← mulChecked (wordBytes 0) numObjs.toUSize
  let size ← addChecked (← addChecked (Obj.ctorObjs 0) fields) scalarBytes.toUSize
  let o ← allocCtorMemory size.toUInt32
  setStHeader o tag numObjs
  pure o

@[export zr_low_header_ctor_get]
def ctorGet (o : USize) (i : UInt32) : BaseIO USize := HeaderMem.ldp (ctorField o i.toUSize)
@[export zr_low_header_ctor_set]
def ctorSet (o : USize) (i : UInt32) (v : USize) : BaseIO Unit := HeaderMem.stp (ctorField o i.toUSize) v
@[export zr_low_header_ctor_set_tag]
def ctorSetTag (o : USize) (tag : UInt8) : BaseIO Unit := HeaderMem.st8 (o + 7) tag

@[export zr_low_header_ctor_get_usize]
def ctorGetUSize (o : USize) (i : UInt32) : BaseIO USize := HeaderMem.ldp (ctorField o i.toUSize)
@[export zr_low_header_ctor_set_usize]
def ctorSetUSize (o : USize) (i : UInt32) (v : USize) : BaseIO Unit := HeaderMem.stp (ctorField o i.toUSize) v

@[export zr_low_header_ctor_get_uint8]
def ctorGetUInt8 (o : USize) (offset : UInt32) : BaseIO UInt8 := HeaderMem.ld8 (ctorData o + offset.toUSize)
@[export zr_low_header_ctor_get_uint16]
def ctorGetUInt16 (o : USize) (offset : UInt32) : BaseIO UInt16 := HeaderMem.ld16 (ctorData o + offset.toUSize)
@[export zr_low_header_ctor_get_uint32]
def ctorGetUInt32 (o : USize) (offset : UInt32) : BaseIO UInt32 := HeaderMem.ld32 (ctorData o + offset.toUSize)
@[export zr_low_header_ctor_get_uint64]
def ctorGetUInt64 (o : USize) (offset : UInt32) : BaseIO UInt64 := HeaderMem.ld64 (ctorData o + offset.toUSize)
@[export zr_low_header_ctor_get_float]
def ctorGetFloat (o : USize) (offset : UInt32) : BaseIO Float := HeaderMem.ldf64 (ctorData o + offset.toUSize)
@[export zr_low_header_ctor_get_float32]
def ctorGetFloat32 (o : USize) (offset : UInt32) : BaseIO Float32 := HeaderMem.ldf32 (ctorData o + offset.toUSize)

@[export zr_low_header_ctor_set_uint8]
def ctorSetUInt8 (o : USize) (offset : UInt32) (v : UInt8) : BaseIO Unit := HeaderMem.st8 (ctorData o + offset.toUSize) v
@[export zr_low_header_ctor_set_uint16]
def ctorSetUInt16 (o : USize) (offset : UInt32) (v : UInt16) : BaseIO Unit := HeaderMem.st16 (ctorData o + offset.toUSize) v
@[export zr_low_header_ctor_set_uint32]
def ctorSetUInt32 (o : USize) (offset : UInt32) (v : UInt32) : BaseIO Unit := HeaderMem.st32 (ctorData o + offset.toUSize) v
@[export zr_low_header_ctor_set_uint64]
def ctorSetUInt64 (o : USize) (offset : UInt32) (v : UInt64) : BaseIO Unit := HeaderMem.st64 (ctorData o + offset.toUSize) v
@[export zr_low_header_ctor_set_float]
def ctorSetFloat (o : USize) (offset : UInt32) (v : Float) : BaseIO Unit := HeaderMem.stf64 (ctorData o + offset.toUSize) v
@[export zr_low_header_ctor_set_float32]
def ctorSetFloat32 (o : USize) (offset : UInt32) (v : Float32) : BaseIO Unit := HeaderMem.stf32 (ctorData o + offset.toUSize) v

@[export zr_low_header_box_uint32]
def boxUInt32 (v : UInt32) : BaseIO USize := do
  if wordBytes 0 == 4 then
    let o ← allocCtor 0 0 4
    HeaderMem.st32 (ctorData o) v
    pure o
  else pure (box v.toUSize)
@[export zr_low_header_unbox_uint32]
def unboxUInt32 (o : USize) : BaseIO UInt32 := do
  if wordBytes 0 == 4 then HeaderMem.ld32 (ctorData o)
  else pure (unbox o).toUInt32

@[export zr_low_header_box_uint64]
def boxUInt64 (v : UInt64) : BaseIO USize := do
  let o ← allocCtor 0 0 8
  HeaderMem.st64 (ctorData o) v
  pure o
@[export zr_low_header_unbox_uint64]
def unboxUInt64 (o : USize) : BaseIO UInt64 := HeaderMem.ld64 (ctorData o)

@[export zr_low_header_box_usize]
def boxUSize (v : USize) : BaseIO USize := do
  let o ← allocCtor 0 0 (wordBytes 0).toUInt32
  HeaderMem.stp (ctorData o) v
  pure o
@[export zr_low_header_unbox_usize]
def unboxUSize (o : USize) : BaseIO USize := HeaderMem.ldp (ctorData o)

@[export zr_low_header_box_float]
def boxFloat (v : Float) : BaseIO USize := do
  let o ← allocCtor 0 0 8
  HeaderMem.stf64 (ctorData o) v
  pure o
@[export zr_low_header_unbox_float]
def unboxFloat (o : USize) : BaseIO Float := HeaderMem.ldf64 (ctorData o)
@[export zr_low_header_box_float32]
def boxFloat32 (v : Float32) : BaseIO USize := do
  let o ← allocCtor 0 0 4
  HeaderMem.stf32 (ctorData o) v
  pure o
@[export zr_low_header_unbox_float32]
def unboxFloat32 (o : USize) : BaseIO Float32 := HeaderMem.ldf32 (ctorData o)

/- Module initializers use these IO-result constructors even when the
   application itself has no general IO operations. Ownership is explicit. -/
@[export zr_low_header_io_result_is_ok]
def ioIsOk (r : USize) : BaseIO Bool := do pure ((← HeaderMem.ld8 (r + 7)) == 0)
@[export zr_low_header_io_result_is_error]
def ioIsError (r : USize) : BaseIO Bool := do pure ((← HeaderMem.ld8 (r + 7)) == 1)
@[export zr_low_header_io_result_get_value]
def ioGetValue (r : USize) : BaseIO USize := ctorGet r 0
@[export zr_low_header_io_result_get_error]
def ioGetError (r : USize) : BaseIO USize := ctorGet r 0
@[export zr_low_header_io_result_take_value]
def ioTakeValue (r : USize) : BaseIO USize := do
  let value ← ctorGet r 0
  Rc.inc value
  Rc.dec r
  pure value
@[export zr_low_header_io_result_mk_ok]
def ioMkOk (value : USize) : BaseIO USize := do
  let r ← allocCtor 0 1 0
  ctorSet r 0 value
  pure r
@[export zr_low_header_io_result_mk_error]
def ioMkError (error : USize) : BaseIO USize := do
  let r ← allocCtor 1 1 0
  ctorSet r 0 error
  pure r

end ZrLean.Low.HeaderObj
