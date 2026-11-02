import ZrLean.Low.Obj

/-!
# Inline runtime pushes, authored in scalar Lean

`zr-cgen fastpath` extracts the generated C bodies into static inline
functions. Only typed field loads/stores and pointer ABI casts use C
primitives; every branch, capacity check and update is scalar Lean.
These nonvolatile accesses are for CPU-owned runtime heap objects, never
MMIO or DMA. Unlike Low.Mem's volatile primitives, they let clang retain
unchanged fields in registers, as the prior C inline functions did.
-/
namespace ZrLean.Low.Fast

/-- A compiler branch hint, with no change to the scalar Boolean. -/
@[never_extract, extern "zr_prim_fast_likely"]
private opaque likely (b : Bool) : Bool

@[extern "zr_prim_fast_load_count"]
private opaque loadCount (a : USize) : BaseIO UInt32
@[extern "zr_prim_fast_array_size"]
private opaque arraySize (a : USize) : BaseIO USize
@[extern "zr_prim_fast_array_cap"]
private opaque arrayCap (a : USize) : BaseIO USize
@[extern "zr_prim_fast_array_set_size"]
private opaque arraySetSize (a n : USize) : BaseIO Unit
@[extern "zr_prim_fast_array_set"]
private opaque arraySet (a i v : USize) : BaseIO Unit
@[extern "zr_prim_fast_byte_size"]
private opaque byteSize (a : USize) : BaseIO USize
@[extern "zr_prim_fast_byte_cap"]
private opaque byteCap (a : USize) : BaseIO USize
@[extern "zr_prim_fast_byte_set_size"]
private opaque byteSetSize (a n : USize) : BaseIO Unit
@[extern "zr_prim_fast_byte_set"]
private opaque byteSet (a i : USize) (v : UInt8) : BaseIO Unit

@[extern "zr_low_array_push_slow"]
opaque arrayPushSlow (a v : USize) : BaseIO USize
@[extern "zr_low_byte_array_push_slow"]
opaque byteArrayPushSlow (a : USize) (b : UInt8) : BaseIO USize

@[export zr_low_array_push_fast]
def arrayPush (a v : USize) : BaseIO USize := do
  if likely ((← loadCount a) == 1) then
    let sz ← arraySize a
    let cap ← arrayCap a
    if likely (sz < cap) then
      arraySet a sz v
      arraySetSize a (sz + 1)
      return a
  arrayPushSlow a v

@[export zr_low_byte_array_push_fast]
def byteArrayPush (a : USize) (b : UInt8) : BaseIO USize := do
  if likely ((← loadCount a) == 1) then
    let sz ← byteSize a
    let cap ← byteCap a
    if likely (sz < cap) then
      byteSet a sz b
      byteSetSize a (sz + 1)
      return a
  byteArrayPushSlow a b

end ZrLean.Low.Fast
