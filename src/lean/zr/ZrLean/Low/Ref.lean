import ZrLean.Low.Rc
import ZrLean.Low.Runtime

/-! Single-executor ST/IO references. A reference owns one value pointer; get
duplicates that ownership, take moves it out, and put/swap move a value in.
There are deliberately no thread-sharing operations in this runtime. -/
namespace ZrLean.Low.Ref
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc

@[never_extract, extern "zr_prim_ref_empty_message"]
opaque emptyMessage (z : USize) : USize

private def requireValue (value : USize) : BaseIO USize := do
  if value == 0 then Runtime.internalPanic (emptyMessage 0) else pure value

@[export zr_low_st_mk_ref]
def mk (value : USize) : BaseIO USize := do
  let ref ← allocObj (ctorObjs 0 + wordBytes 0)
  setHeader ref 0xfd000000
  stp (ref + ctorObjs 0) value
  pure ref

@[export zr_low_st_ref_get]
def get (ref : USize) : BaseIO USize := do
  let value ← requireValue (← ldp (ref + ctorObjs 0))
  inc value
  pure value

@[export zr_low_st_ref_take]
def take (ref : USize) : BaseIO USize := do
  let value ← requireValue (← ldp (ref + ctorObjs 0))
  stp (ref + ctorObjs 0) 0
  pure value

@[export zr_low_st_ref_put]
def put (ref value : USize) : BaseIO USize := do
  let old ← ldp (ref + ctorObjs 0)
  stp (ref + ctorObjs 0) value
  if old != 0 then dec old
  pure 1

@[export zr_low_st_ref_swap]
def swap (ref value : USize) : BaseIO USize := do
  let old ← requireValue (← ldp (ref + ctorObjs 0))
  stp (ref + ctorObjs 0) value
  pure old

end ZrLean.Low.Ref
