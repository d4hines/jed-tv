import ZrLean.Low.Apply

/-! Runtime startup, diagnostics and weak radio trace hooks. No closed terms
or high-level Lean values may occur here: these functions bootstrap Init. -/
namespace ZrLean.Low.Runtime
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc

@[never_extract, extern "zr_prim_runtime_state"]
opaque state (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_empty_cell"]
opaque emptyCell (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_ok_header"]
opaque okHeader (z : USize) : UInt32

@[extern "zr_prim_runtime_halt"]
opaque halt (msg len : USize) : BaseIO USize
@[extern "zr_prim_runtime_report"]
opaque report (msg len : USize) : BaseIO Unit

@[never_extract, extern "zr_prim_runtime_msg0"]
opaque msg0 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg1"]
opaque msg1 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg2"]
opaque msg2 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg3"]
opaque msg3 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg4"]
opaque msg4 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg5"]
opaque msg5 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg6"]
opaque msg6 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg7"]
opaque msg7 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg8"]
opaque msg8 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg9"]
opaque msg9 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg10"]
opaque msg10 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_msg12"]
opaque msg12 (z : USize) : USize

@[export zr_low_runtime_fatal]
def runtimeFatal (code : USize) : BaseIO USize := do
  if code == 0 then halt (msg0 0) 19
  else if code == 1 then halt (msg1 0) 58
  else if code == 2 then halt (msg2 0) 69
  else if code == 4 then halt (msg4 0) 49
  else if code == 5 then halt (msg5 0) 30
  else if code == 6 then halt (msg6 0) 27
  else if code == 7 then halt (msg7 0) 27
  else if code == 8 then halt (msg8 0) 24
  else if code == 9 then halt (msg9 0) 29
  else if code == 10 then halt (msg12 0) 64
  else halt (msg3 0) 45

@[export zr_low_runtime_oom]
def oom (_z : USize) : BaseIO USize := runtimeFatal 0
@[export zr_low_runtime_overflow]
def overflow (_z : USize) : BaseIO USize := runtimeFatal 3

partial def strlen (msg n : USize) : BaseIO USize := do
  if (← ld8 (msg + n)) == 0 then pure n else strlen msg (n + 1)

@[export zr_low_internal_panic]
def internalPanic (msg : USize) : BaseIO USize := do halt msg (← strlen msg 0)

@[export zr_low_lean_panic]
def leanPanic (msg len : USize) : BaseIO UInt8 := do
  stp (state 0) ((← ldp (state 0)) + 1)
  report msg len
  pure 0

@[export zr_low_panic_count]
def panicCount (_z : USize) : BaseIO USize := ldp (state 0)

@[export zr_low_array_get_panic]
def getPanic (default : USize) : BaseIO USize := do
  let _ ← leanPanic (msg10 0) 26
  pure default

@[export zr_low_array_set_panic]
def setPanic (a v : USize) : BaseIO USize := do
  dec v
  let _ ← leanPanic (msg10 0) 26
  pure a

@[export zr_low_runtime_module_init]
def moduleInit (_z : UInt8) : BaseIO UInt8 := pure 0

@[never_extract, extern "zr_prim_runtime_boot_one"]
opaque bootOne (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_boot_zero"]
opaque bootZero (z : USize) : USize

@[export zr_low_initialize_Init]
def bootstrap (_builtin : UInt8) : BaseIO USize := do
  let cell := state 0 + wordBytes 0
  if (← ldp cell) == 0 then
    stp cell 1
    let e ← allocSArray (bootOne 0) (bootZero 0) (bootZero 0)
    RcCold.markPersistent e
    stp (emptyCell 0) e
  -- IO.ok (): constructor tag 0, one object field holding scalar Unit.
  let r ← allocObj (ctorObjs 0 + wordBytes 0)
  setHeader r (okHeader 0)
  stp (r + ctorObjs 0) 1
  pure r

@[never_extract, extern "zr_prim_runtime_trace_begin_fn"]
opaque traceBeginFn (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_trace_end_fn"]
opaque traceEndFn (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_trace_ev_fn"]
opaque traceEvFn (z : USize) : USize
@[extern "zr_prim_runtime_call_begin"]
opaque callBegin (fn tag : USize) : BaseIO Unit
@[extern "zr_prim_runtime_call_end"]
opaque callEnd (fn : USize) : BaseIO Unit
@[extern "zr_prim_runtime_call_ev"]
opaque callEv (fn kind : USize) (a v : UInt32) : BaseIO Unit

@[never_extract, extern "zr_prim_runtime_op0"]
opaque op0 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op1"]
opaque op1 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op2"]
opaque op2 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op3"]
opaque op3 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op4"]
opaque op4 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op5"]
opaque op5 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op6"]
opaque op6 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op7"]
opaque op7 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op8"]
opaque op8 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op9"]
opaque op9 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op10"]
opaque op10 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op11"]
opaque op11 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op12"]
opaque op12 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op13"]
opaque op13 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op14"]
opaque op14 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op15"]
opaque op15 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op16"]
opaque op16 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op17"]
opaque op17 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op18"]
opaque op18 (z : USize) : USize
@[never_extract, extern "zr_prim_runtime_op19"]
opaque op19 (z : USize) : USize

def opName (op : UInt32) : USize :=
  if op == 0 then op0 0
  else if op == 1 then op1 0
  else if op == 2 then op2 0
  else if op == 3 then op3 0
  else if op == 4 then op4 0
  else if op == 5 then op5 0
  else if op == 6 then op6 0
  else if op == 7 then op7 0
  else if op == 8 then op8 0
  else if op == 9 then op9 0
  else if op == 10 then op10 0
  else if op == 11 then op11 0
  else if op == 12 then op12 0
  else if op == 13 then op13 0
  else if op == 14 then op14 0
  else if op == 15 then op15 0
  else if op == 16 then op16 0
  else if op == 17 then op17 0
  else if op == 18 then op18 0
  else 0

@[export zr_lean_trace_ev]
def traceEv (kind : UInt16) (a v : UInt32) : BaseIO Unit := do
  let fn := traceEvFn 0
  if fn != 0 then
    let p ← Apply.stack 2
    st8 p kind.toUInt8
    st8 (p + 1) (kind >>> 8).toUInt8
    callEv fn p a v

@[export zr_lean_trace_begin]
def traceBegin (op : UInt32) : BaseIO Unit := do
  let fn := traceBeginFn 0
  if fn != 0 && op < 19 then callBegin fn (opName op)

@[export zr_lean_trace_end]
def traceEnd (ret : UInt32) : BaseIO Unit := do
  let ev := traceEvFn 0
  if ev != 0 then callEv ev (op19 0) 0 ret
  let finish := traceEndFn 0
  if finish != 0 then callEnd finish

end ZrLean.Low.Runtime
