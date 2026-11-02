import ZrAbi.C

namespace ZrAbi.Fast
open C

private def bridge (name target : String) (params : List Param) (args : List Expr) : Decl :=
  .function .staticInline (.named "lean_obj_res") name params
    [.return_ (some (.cast (.ptr (.named "lean_object")) (.call (.ident target) args)))]

/- Pointer/scalar calling-convention bridges for the compiled scalar Lean
   push functions. All branching and mutation is in Low.Fast compiler output. -/
def bridges : Header := [
  .pragmaOnce,
  bridge "zr_byte_array_push_fast" "zr_inline_byte_array_push"
    [⟨"a",.named "lean_obj_arg"⟩, ⟨"b",.named "uint8_t"⟩]
    [.cast (.named "size_t") (.ident "a"), .ident "b"],
  bridge "zr_array_push_fast" "zr_inline_array_push"
    [⟨"a",.named "lean_obj_arg"⟩, ⟨"v",.named "lean_obj_arg"⟩]
    [.cast (.named "size_t") (.ident "a"), .cast (.named "size_t") (.ident "v")],
  .define "lean_byte_array_push" (.ident "zr_byte_array_push_fast"),
  .define "lean_array_push" (.ident "zr_array_push_fast")
]

def shim : Header := [.pragmaOnce,.include "zr_fastpath.generated.h"]

end ZrAbi.Fast
