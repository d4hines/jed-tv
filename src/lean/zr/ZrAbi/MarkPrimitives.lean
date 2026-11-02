import ZrAbi.C

/- Pointer ABI bridges only. Algorithms stay in Low.HeaderMarks; cold
providers keep their original declarations and may be test-only. -/
namespace ZrAbi.MarkPrimitives
open C

private def obj : CType := .ptr (.named "lean_object")
private def word : CType := .named "size_t"
private def byte : CType := .named "uint8_t"
private def arg (name : String) : Param := ⟨name, word⟩
private def pointer (name : String) : Expr := .cast obj (.ident name)
private def bridge (name target : String) (result : CType)
    (params : List Param) (arguments : List Expr) : Decl :=
  .function .staticInline result name params
    [.return_ (some (.cast result (.call (.ident target) arguments)))]

def header : C.Header := [
  bridge "zr_mark_string_copy_nonlinear" "lean_copy_string_nonlinear" word
    [arg "s", arg "capacity"] [pointer "s", .ident "capacity"],
  bridge "zr_mark_string_eq_cold" "lean_string_eq_cold" byte
    [arg "a", arg "b"] [pointer "a", pointer "b"],
  bridge "zr_mark_string_lt" "lean_string_lt" byte
    [arg "a", arg "b"] [pointer "a", pointer "b"],
  bridge "zr_mark_sarray_eq_cold" "lean_sarray_eq_cold" byte
    [arg "a", arg "b"] [pointer "a", pointer "b"]
]

end ZrAbi.MarkPrimitives
