import ZrLean.Low.RcCold

/-!
Single-threaded closed terms: compute once, publish the value before the state,
and make object values persistent. The lock word is untouched. The callback
and memory accesses are scalar ABI primitives, so none of these paths depends
on the runtime it is initializing.
-/
namespace ZrLean.Low.Once
open ZrLean.Low ZrLean.Low.RcCold

@[extern "zr_prim_once_call_uint8"]
opaque callUInt8 (fn : USize) : BaseIO UInt8

@[export zr_low_uint8_once_cold]
def onceUInt8 (loc tok fn : USize) : BaseIO UInt8 := do
  if (← ld32 tok) != 1 then
    st8 loc (← callUInt8 fn)
    st32 tok 1
  ld8 loc

@[extern "zr_prim_once_call_uint16"]
opaque callUInt16 (fn : USize) : BaseIO UInt16

@[export zr_low_uint16_once_cold]
def onceUInt16 (loc tok fn : USize) : BaseIO UInt16 := do
  if (← ld32 tok) != 1 then
    st16 loc (← callUInt16 fn)
    st32 tok 1
  ld16 loc

@[extern "zr_prim_once_call_uint32"]
opaque callUInt32 (fn : USize) : BaseIO UInt32

@[export zr_low_uint32_once_cold]
def onceUInt32 (loc tok fn : USize) : BaseIO UInt32 := do
  if (← ld32 tok) != 1 then
    st32 loc (← callUInt32 fn)
    st32 tok 1
  ld32 loc

@[extern "zr_prim_once_ld64"]
opaque ld64 (a : USize) : BaseIO UInt64
@[extern "zr_prim_once_st64"]
opaque st64 (a : USize) (v : UInt64) : BaseIO Unit
@[extern "zr_prim_once_call_uint64"]
opaque callUInt64 (fn : USize) : BaseIO UInt64

@[export zr_low_uint64_once_cold]
def onceUInt64 (loc tok fn : USize) : BaseIO UInt64 := do
  if (← ld32 tok) != 1 then
    st64 loc (← callUInt64 fn)
    st32 tok 1
  ld64 loc

@[extern "zr_prim_once_call_usize"]
opaque callUSize (fn : USize) : BaseIO USize

@[export zr_low_usize_once_cold]
def onceUSize (loc tok fn : USize) : BaseIO USize := do
  if (← ld32 tok) != 1 then
    stp loc (← callUSize fn)
    st32 tok 1
  ldp loc

@[extern "zr_prim_once_ld_float"]
opaque ldFloat (a : USize) : BaseIO Float
@[extern "zr_prim_once_st_float"]
opaque stFloat (a : USize) (v : Float) : BaseIO Unit
@[extern "zr_prim_once_call_float"]
opaque callFloat (fn : USize) : BaseIO Float

@[export zr_low_float_once_cold]
def onceFloat (loc tok fn : USize) : BaseIO Float := do
  if (← ld32 tok) != 1 then
    stFloat loc (← callFloat fn)
    st32 tok 1
  ldFloat loc

@[extern "zr_prim_once_ld_float32"]
opaque ldFloat32 (a : USize) : BaseIO Float32
@[extern "zr_prim_once_st_float32"]
opaque stFloat32 (a : USize) (v : Float32) : BaseIO Unit
@[extern "zr_prim_once_call_float32"]
opaque callFloat32 (fn : USize) : BaseIO Float32

@[export zr_low_float32_once_cold]
def onceFloat32 (loc tok fn : USize) : BaseIO Float32 := do
  if (← ld32 tok) != 1 then
    stFloat32 loc (← callFloat32 fn)
    st32 tok 1
  ldFloat32 loc

@[extern "zr_prim_once_call_obj"]
opaque callObj (fn : USize) : BaseIO USize

@[export zr_low_obj_once_cold]
def onceObj (loc tok fn : USize) : BaseIO USize := do
  if (← ld32 tok) != 1 then
    let o ← callObj fn
    stp loc o
    markPersistent o
    st32 tok 1
  ldp loc

end ZrLean.Low.Once
