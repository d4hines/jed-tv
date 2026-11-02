import ZrLean.Low.HeaderArray
import ZrLean.Low.HeaderSupport

/-! Remaining array/string header decisions. Cold operations retain their
pointer-typed ABI through typed MarkPrimitives bridges. Marking consumes the
destination, preserves source borrowing, and uses existing nonlinear-copy
providers (including their configured linearity diagnostic policy). -/
namespace ZrLean.Low.HeaderMarks
open ZrLean.Low

@[extern "zr_mark_string_copy_nonlinear"]
opaque copyString (s capacity : USize) : BaseIO USize
@[extern "zr_mark_string_eq_cold"]
opaque stringEqCold (a b : USize) : BaseIO UInt8
@[extern "zr_mark_string_lt"]
opaque stringLtCold (a b : USize) : BaseIO UInt8
@[extern "zr_mark_sarray_eq_cold"]
opaque sarrayEqCold (a b : USize) : BaseIO UInt8

@[export zr_low_header_array_pop]
def arrayPop (a : USize) : BaseIO USize := do
  let r ← HeaderArray.ensureExclusiveArray a
  let size ← HeaderArray.arraySize r
  if size == 0 then return r
  let size := size - 1
  HeaderMem.stp (r + Obj.arrSizeOff 0) size
  Rc.dec (← HeaderMem.ldp (HeaderArray.arrayCptr r + size * wordBytes 0))
  pure r

@[export zr_low_header_array_mark_linear]
def arrayMarkLinear (a : USize) : BaseIO USize := do
  let r ← HeaderArray.ensureExclusiveArray a
  HeaderSupport.markLinearCore r
  pure r

@[export zr_low_header_array_propagate_mark]
def arrayPropagateMark (source dest : USize) : BaseIO USize := do
  if ← HeaderSupport.isMarkedLinearCore source then arrayMarkLinear dest else pure dest

@[export zr_low_header_sarray_mark_linear]
def sarrayMarkLinear (a : USize) : BaseIO USize := do
  let r ← HeaderArray.sarrayEnsureExclusive a
  HeaderSupport.markLinearCore r
  pure r

@[export zr_low_header_sarray_propagate_mark]
def sarrayPropagateMark (source dest : USize) : BaseIO USize := do
  if ← HeaderSupport.isMarkedLinearCore source then sarrayMarkLinear dest else pure dest

@[export zr_low_header_sarray_dec_eq]
def sarrayDecEq (a b : USize) : BaseIO UInt8 := do
  if a == b then return 1
  if (← HeaderArray.sarraySize a) != (← HeaderArray.sarraySize b) then return 0
  sarrayEqCold a b

@[inline] private def ensureExclusiveString (s : USize) : BaseIO USize := do
  if (← HeaderMem.ld32 s) == 1 then pure s
  else copyString s (← HeaderSupport.stringCapacity s)

@[export zr_low_header_string_mark_linear]
def stringMarkLinear (s : USize) : BaseIO USize := do
  let r ← ensureExclusiveString s
  HeaderSupport.markLinearCore r
  pure r

@[export zr_low_header_string_propagate_mark]
def stringPropagateMark (source dest : USize) : BaseIO USize := do
  if ← HeaderSupport.isMarkedLinearCore source then stringMarkLinear dest else pure dest

@[export zr_low_header_string_length]
def stringLength (s : USize) : BaseIO USize := do
  pure (((← HeaderSupport.stringLen s) <<< 1) ||| 1)

@[export zr_low_header_string_utf8_byte_size]
def stringUtf8ByteSize (s : USize) : BaseIO USize := do
  pure ((((← HeaderSupport.stringSize s) - 1) <<< 1) ||| 1)

@[export zr_low_header_string_dec_eq]
def stringDecEq (a b : USize) : BaseIO UInt8 := do
  if a == b then return 1
  if (← HeaderSupport.stringSize a) != (← HeaderSupport.stringSize b) then return 0
  stringEqCold a b

@[export zr_low_header_string_dec_lt]
def stringDecLt (a b : USize) : BaseIO UInt8 := stringLtCold a b

end ZrLean.Low.HeaderMarks
