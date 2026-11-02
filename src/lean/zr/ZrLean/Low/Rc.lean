/-
Reference counting and freeing, part 3 of 4 (AGENTS.md, "The runtime's
objects"): `lean_dec_ref_cold`, the walk that frees a dead object and
everything only it kept alive (`lean_del_core`), `lean_free_object`, and the
increments and decrements the other ZrLow modules share. Same algorithm as the
C it replaced (src/c/zr_runtime.c as of d2c5b4c2, oracle in
test/leanrt/csrc/zr_rc_oracle.c).

Dead objects are freed without recursion: an object whose count drops to zero
goes on a to-do list threaded through its own header (`Obj.setLink`), and the
walk pops one at a time. Single core, so a thread-shared (negative) count needs
no atomics.
-/
import ZrLean.Low.RcCold

namespace ZrLean.Low.Rc
open ZrLean.Low ZrLean.Low.Alloc ZrLean.Low.Obj ZrLean.Low.RcCold

/-- A count no drop frees: persistent (0) or, as unsigned, at or below
`LEAN_RC_STICKY_DROP` (0xA0000000). -/
@[inline] def neverFreed (rc : UInt32) : Bool := rc ≤ 0xA0000000

/-- Drop one reference to `o` (not a scalar); queue it on `todo` if that was
the last. Returns the new head of the list. -/
@[inline] def decChild (o todo : USize) : BaseIO USize := do
  if o &&& 1 != 0 then pure todo
  else
    let rc ← ld32 o
    if sgt32 rc 1 then
      st32 o (rc - 1)
      pure todo
    else if rc == 1 then
      setLink o todo
      pure o
    else if neverFreed rc then pure todo
    else
      st32 o (rc + 1)
      if rc == 0xFFFFFFFF then
        setLink o todo
        pure o
      else pure todo

/-- Drop the `n` references at `base[i..]`. -/
partial def decFields (base i n todo : USize) : BaseIO USize := do
  if i < n then
    decFields base (i + 1) n (← decChild (← ldp (base + i * wordBytes 0)) todo)
  else pure todo

/-- Free a small object (`lean_free_small_object`: constructors, big Nats). -/
@[inline] def freeSmall (o : USize) : BaseIO Unit := Alloc.free (theHeap 0) o

/-- Free a block of `lean_alloc_object` of `size` bytes. -/
@[inline] def deallocSized (o size : USize) : BaseIO Unit := Alloc.dealloc (theHeap 0) o size

/-- Release the children of the dead object `o` and free it; the new to-do list. -/
def delCore (o todo : USize) : BaseIO USize := do
  let tag ← tagOf o
  if tag ≤ 243 then
    let todo ← decFields (o + ctorObjs 0) 0 (← otherOf o).toUSize todo
    freeSmall o
    pure todo
  else if tag == 245 then
    let todo ← decFields (o + cloObjs 0) 0 (← ld16 (o + cloFixedOff 0)).toUSize todo
    deallocSized o (← closureBytes o)
    pure todo
  else if tag == 246 then
    let todo ← decFields (o + arrData 0) 0 (← ldp (o + arrSizeOff 0)) todo
    deallocSized o (← arrayBytes o)
    pure todo
  else if tag == 248 then
    deallocSized o (← sarrayBytes o)
    pure todo
  else if tag == 249 then
    deallocSized o (← stringBytes o)
    pure todo
  else if tag == 250 then
    freeSmall o
    pure todo
  else if tag == 253 then
    let value ← ldp (o + ctorObjs 0)
    let todo ← if value == 0 then pure todo else decChild value todo
    freeSmall o
    pure todo
  else fatal 1

/-- Free `o` and, one after another, what its death releases. -/
partial def freeAll (o todo : USize) : BaseIO Unit := do
  let todo ← delCore o todo
  if todo != 0 then freeAll todo (← getLink todo)

/-- `lean_dec_ref_cold`: `lean_dec_ref` when the count is 1 or not
single-threaded. -/
def decRefCold (o : USize) : BaseIO Unit := do
  let rc ← ld32 o
  if rc != 1 then
    if neverFreed rc then return
    st32 o (rc + 1)
    if rc != 0xFFFFFFFF then return
  freeAll o 0

/-- `lean_dec_ref`. -/
@[inline] def decRef (o : USize) : BaseIO Unit := do
  let rc ← ld32 o
  if sgt32 rc 1 then st32 o (rc - 1)
  else if rc != 0 then decRefCold o

/-- `lean_dec`. -/
@[inline] def dec (o : USize) : BaseIO Unit := do
  if o &&& 1 == 0 then decRef o

/-- `lean_inc_ref_n`. -/
@[inline] def incRefN (o n : USize) : BaseIO Unit := do
  if n > 0x10000 then incRefHugeN o n
  else
    let rc ← ld32 o
    if sgt32 rc 0 then st32 o (rc + n.toUInt32)
    else if rc > 0x90000000 then st32 o (rc - n.toUInt32)

/-- `lean_inc`. -/
@[inline] def inc (o : USize) : BaseIO Unit := do
  if o &&& 1 == 0 then
    let rc ← ld32 o
    if sgt32 rc 0 then st32 o (rc + 1)
    else if rc > 0x90000000 then st32 o (rc - 1)

/-- `lean_free_object`: free the memory of the dead object `o` alone (its
children are not touched). -/
def freeObject (o : USize) : BaseIO Unit := do
  let tag ← tagOf o
  if tag == 246 then deallocSized o (← arrayBytes o)
  else if tag == 248 then deallocSized o (← sarrayBytes o)
  else if tag == 249 then deallocSized o (← stringBytes o)
  else if tag == 245 then deallocSized o (← closureBytes o)
  else freeSmall o

@[export zr_low_dec_ref_cold]
def decRefColdFfi (o : USize) : BaseIO UInt8 := do decRefCold o; pure 0

@[export zr_low_free_object]
def freeObjectFfi (o : USize) : BaseIO UInt8 := do freeObject o; pure 0

end ZrLean.Low.Rc
