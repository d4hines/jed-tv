import ZrAbi.C

/- Typed object/pointer ABI adaptation only. The native implementation is
the scalar Lean reference; ARM uses the Lean-authored read-only byte leaf. -/
namespace ZrAbi.Crc32Primitives
open C
private def sz : CType := .named "size_t"
private def u32 : CType := .named "uint32_t"
private def obj : CType := .ptr (.named "lean_object")
private def p (n : String) (t : CType) : Param := ⟨n,t⟩
private def args : List Param := [p "buf" obj, p "off" sz, p "len" sz]
private def id := Expr.ident
private def invoke (arm : Bool) : Decl :=
  let data := Expr.binary .add (.cast sz (.call (id "lean_sarray_cptr") [id "buf"])) (id "off")
  let call := if arm then Expr.call (id "zr_crc32_arm") [data, id "len"]
    else Expr.call (id "zr_crc32_reference") [id "buf",id "off",id "len"]
  let retain := if arm then [] else [Stmt.expr (.call (id "lean_inc") [id "buf"])]
  .function .staticInline u32 "zr_crc32_kernel" args (retain ++ [.return_ (some call)])

def header : Header := [
  .prototype u32 "zr_crc32_reference" args .extern_,
  .prototype u32 "zr_crc32_arm" [p "data" sz,p "len" sz] .extern_,
  .ifdef "__arm__" [invoke true] [invoke false]
]
end ZrAbi.Crc32Primitives
