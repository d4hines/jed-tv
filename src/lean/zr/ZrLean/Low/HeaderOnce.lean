import ZrLean.Low.Once
import ZrLean.Low.HeaderMem

/-!
Header once-cell fast paths. A published cell (state 1) reads its cached value
without touching its initializer. Other states delegate to the existing
single-core cold implementation, which publishes the value before the state
and makes object results persistent. Raw addresses and callbacks avoid any
runtime dependency while the runtime itself is starting.
-/
namespace ZrLean.Low.HeaderOnce
open ZrLean.Low

@[export zr_low_header_obj_once]
def obj (loc tok fn : USize) : BaseIO USize := do
  if (← HeaderMem.ld32 tok) == 1 then HeaderMem.ldp loc else Once.onceObj loc tok fn

@[export zr_low_header_uint8_once]
def uint8 (loc tok fn : USize) : BaseIO UInt8 := do
  if (← HeaderMem.ld32 tok) == 1 then HeaderMem.ld8 loc else Once.onceUInt8 loc tok fn

@[export zr_low_header_uint16_once]
def uint16 (loc tok fn : USize) : BaseIO UInt16 := do
  if (← HeaderMem.ld32 tok) == 1 then HeaderMem.ld16 loc else Once.onceUInt16 loc tok fn

@[export zr_low_header_uint32_once]
def uint32 (loc tok fn : USize) : BaseIO UInt32 := do
  if (← HeaderMem.ld32 tok) == 1 then HeaderMem.ld32 loc else Once.onceUInt32 loc tok fn

@[export zr_low_header_uint64_once]
def uint64 (loc tok fn : USize) : BaseIO UInt64 := do
  if (← HeaderMem.ld32 tok) == 1 then HeaderMem.ld64 loc else Once.onceUInt64 loc tok fn

@[export zr_low_header_usize_once]
def usize (loc tok fn : USize) : BaseIO USize := do
  if (← HeaderMem.ld32 tok) == 1 then HeaderMem.ldp loc else Once.onceUSize loc tok fn

@[export zr_low_header_float_once]
def float (loc tok fn : USize) : BaseIO Float := do
  if (← HeaderMem.ld32 tok) == 1 then HeaderMem.ldf64 loc else Once.onceFloat loc tok fn

@[export zr_low_header_float32_once]
def float32 (loc tok fn : USize) : BaseIO Float32 := do
  if (← HeaderMem.ld32 tok) == 1 then HeaderMem.ldf32 loc else Once.onceFloat32 loc tok fn

end ZrLean.Low.HeaderOnce
