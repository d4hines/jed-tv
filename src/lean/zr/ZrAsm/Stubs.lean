import ZrAsm.Dsl

/-!
# Branch stubs

`name: b target`, one instruction: how a symbol the C side (or the compiler)
calls is bound to a Lean implementation under another name, without a
wrapper function: the `lean.h` externs, `memcpy`, the `__aeabi_*` helpers.
Each stub is its own section so `--gc-sections` drops it when unused. A
tail jump keeps the caller's arguments and link register, so the target
must have the same calling convention as the name it stands for.
-/

namespace ZrAsm
open Asm

/-- `name: b target`. -/
def stub (name target : String) : Routine := routine name do b target

/-- The stubs for `(name, target)` pairs. -/
def stubs (l : List (String × String)) : List Routine := l.map fun (n, t) => stub n t

end ZrAsm
