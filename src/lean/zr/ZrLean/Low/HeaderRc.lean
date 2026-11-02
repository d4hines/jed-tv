import ZrLean.Low.Rc
import ZrLean.Low.HeaderMem

/-!
The runtime header's reference-count fast paths, authored as scalar Lean.
Arguments are raw addresses, never owned Lean objects. These functions can run
before module initialization: they use no closed terms, allocation or boxing.
The generated ABI header may inline their compiler-generated scalar bodies;
the existing cold runtime retains object destruction and huge increments.

As elsewhere in ZrLow, thread-shared counts use ordinary scalar loads/stores:
firmware runs on one core without concurrent Lean threads. Sticky ranges and
32-bit wrapping still match the upstream header, including huge increments.
-/
namespace ZrLean.Low.HeaderRc
open ZrLean.Low ZrLean.Low.Obj

@[export zr_low_header_is_mt]
def isMt (o : USize) : BaseIO Bool := do
  pure (sgt32 0 (← HeaderMem.ld32 o))

@[export zr_low_header_is_st]
def isSt (o : USize) : BaseIO Bool := do
  pure (sgt32 (← HeaderMem.ld32 o) 0)

@[export zr_low_header_is_persistent]
def isPersistent (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld32 o) == 0)

@[export zr_low_header_has_rc]
def hasRc (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld32 o) != 0)

@[export zr_low_header_is_unstuck_mt]
def isUnstuckMt (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld32 o) > 0x90000000)

/-- Called after ruling out a positive single-threaded count. -/
@[export zr_low_header_is_never_freed]
def isNeverFreed (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld32 o) ≤ 0xA0000000)

@[export zr_low_header_is_exclusive]
def isExclusive (o : USize) : BaseIO Bool := do
  pure ((← HeaderMem.ld32 o) == 1)

@[export zr_low_header_is_exclusive_obj]
def isExclusiveObj (o : USize) : BaseIO UInt8 := do
  pure (if (← HeaderMem.ld32 o) == 1 then 1 else 0)

@[export zr_low_header_is_shared]
def isShared (o : USize) : BaseIO Bool := do
  pure (sgt32 (← HeaderMem.ld32 o) 1)

@[inline, export zr_low_header_inc_ref_n]
def incRefN (o n : USize) : BaseIO Unit := do
  if n > 0x10000 then RcCold.incRefHugeN o n
  else
    let rc ← HeaderMem.ld32 o
    if sgt32 rc 0 then HeaderMem.st32 o (rc + n.toUInt32)
    else if rc > 0x90000000 then HeaderMem.st32 o (rc - n.toUInt32)

@[inline, export zr_low_header_inc_ref]
def incRef (o : USize) : BaseIO Unit := do
  let rc ← HeaderMem.ld32 o
  if sgt32 rc 0 then HeaderMem.st32 o (rc + 1)
  else if rc > 0x90000000 then HeaderMem.st32 o (rc - 1)

@[export zr_low_header_inc_n]
def incN (o n : USize) : BaseIO Unit := do
  if o &&& 1 == 0 then incRefN o n

@[export zr_low_header_inc]
def inc (o : USize) : BaseIO Unit := do
  if o &&& 1 == 0 then incRef o

@[inline, export zr_low_header_dec_ref]
def decRef (o : USize) : BaseIO Unit := do
  let rc ← HeaderMem.ld32 o
  if sgt32 rc 1 then HeaderMem.st32 o (rc - 1)
  else if rc != 0 then Rc.decRefCold o

@[inline, export zr_low_header_dec]
def dec (o : USize) : BaseIO Unit := do
  if o &&& 1 == 0 then decRef o

/-- Explicit scalar recursion avoids `while`'s erased typeclass wrapper. -/
partial def decKnownFields (o : USize) (objs i : UInt32) : BaseIO Unit := do
  if i < objs then
    dec (← HeaderMem.ldp (o + ctorObjs 0 + i.toUSize * wordBytes 0))
    decKnownFields o objs (i + 1)

/-- Compiler-known constructor field count: release fields in order before
freeing the unique parent; otherwise drop its shared reference normally. -/
@[export zr_low_header_dec_ref_known]
def decRefKnown (o : USize) (objs : UInt32) : BaseIO Unit := do
  if (← HeaderMem.ld32 o) == 1 then
    decKnownFields o objs 0
    Rc.freeObject o
  else
    decRef o

/-- Delete storage without releasing children, as required after a reset or
field transfer. A tagged scalar has no allocation to delete. -/
@[export zr_low_header_del_object]
def delObject (o : USize) : BaseIO Unit := do
  if o &&& 1 == 0 then Rc.freeObject o

end ZrLean.Low.HeaderRc
