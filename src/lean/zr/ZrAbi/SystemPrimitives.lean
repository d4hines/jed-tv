import ZrAbi.C

/- Literal diagnostics and the zero-memory primitive, generated from Lean
   descriptions. Strings are immutable byte data, never Lean heap objects. -/
namespace ZrAbi.SystemPrimitives
open C

private def strings : List (String × String) := [
  ("zr_prim_fault_undefName", "UndefinedInstruction"),
  ("zr_prim_fault_svcName", "SoftwareInterrupt"),
  ("zr_prim_fault_prefetchName", "PrefetchAbort"),
  ("zr_prim_fault_dataName", "DataAbort"),
  ("zr_prim_fault_reservedName", "ReservedException"),
  ("zr_prim_fault_irqName", "IRQ"),
  ("zr_prim_fault_fiqName", "FIQ"),
  ("zr_prim_fault_unknownName", "?"),
  ("zr_prim_fault_corePrefix", " on core "),
  ("zr_prim_fault_ifsrPrefix", "\nIFSR: "),
  ("zr_prim_fault_ifarPrefix", "\nIFAR: "),
  ("zr_prim_fault_dfsrPrefix", "\nDFSR: "),
  ("zr_prim_fault_dfarPrefix", "\nDFAR: "),
  ("zr_prim_fwboot_secondsSuffix", "s] "),
  ("zr_prim_fwboot_targetSuffix", "): "),
  ("zr_prim_fwboot_panicPrefix", "panic at "),
  ("zr_prim_fwboot_colon", ": "),
  ("zr_prim_fwboot_runtimePath", "src/c/zr_runtime.c"),
  ("zr_prim_fwboot_warnName", "WARN"),
  ("zr_prim_fwboot_runtimeTarget", "leanrt::platform"),
  ("zr_prim_fwboot_leanPanic", "lean panic: "),
  ("zr_prim_fwboot_bootName", "zr_boot"),
  ("zr_prim_fwboot_initFailed", "lean module initialisation failed")
]

def header : Header :=
  (strings.map fun (name,value) => .function .staticInline (.named "size_t") name
    [⟨"unused",.named "size_t"⟩] [.return_ (some (.cast (.named "size_t") (.str value)))]) ++ [
    .function .staticInline (.ptr (.named "lean_object")) "zr_prim_platform_zero"
      [⟨"dst",.named "size_t"⟩, ⟨"len",.named "size_t"⟩]
      [.expr (.call (.ident "__builtin_memset") [
        .cast (.ptr (.named "void")) (.cast (.named "uintptr_t") (.ident "dst")),
        .num 0, .ident "len"]),
       .return_ (some (.call (.ident "lean_box") [.num 0]))]
  ]

end ZrAbi.SystemPrimitives
