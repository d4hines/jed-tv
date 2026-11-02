import ZrAbi.C

/- Terminal pointer, reference-count field and fixed-width integer operations
   consumed by compiler output. Object deletion remains scalar HeaderRc Lean.
   These declarations match the single-core, NDEBUG freestanding ABI. -/
namespace ZrAbi.ExtraPrimitives
open C

private def id := Expr.ident
private def u8 : CType := .named "uint8_t"
private def u16 : CType := .named "uint16_t"
private def u32 : CType := .named "uint32_t"
private def u64 : CType := .named "uint64_t"
private def i16 : CType := .named "int16_t"
private def i32 : CType := .named "int32_t"
private def i64 : CType := .named "int64_t"
private def int : CType := .named "int"
private def uint : CType := .named "unsigned"
private def obj : CType := .ptr (.named "lean_object")
private def ctor : CType := .ptr (.named "lean_ctor_object")
private def p (name : String) (type : CType) : Param := ⟨name,type⟩
private def ret (e : Expr) : Stmt := .return_ (some e)
private def value (name : String) (result : CType) (params : List Param) (e : Expr) : Decl :=
  .function .staticInline result name params [ret e]
private def binary (op : BinaryOp) (a b : Expr) : Expr := .binary op a b
private def rc : Expr := .arrow (id "o") "m_rc"

private def pointersAndRc : Header := [
  value "lean_ptr_addr" (.named "size_t") [p "a" obj] (.cast (.named "size_t") (id "a")),
  value "lean_to_ctor" ctor [p "o" obj] (.cast ctor (id "o")),
  value "lean_ctor_obj_cptr" (.ptr obj) [p "o" obj] (.arrow (.cast ctor (id "o")) "m_objs"),
  value "lean_get_rc_mt_addr" (.ptr (.atomic int)) [p "o" obj]
    (.cast (.ptr (.atomic int)) (.unary .addr rc)),
  value "lean_internal_get_rc" int [p "o" obj] rc,
  .function .staticInline (.named "void") "lean_internal_set_rc" [p "o" obj,p "rc" int]
    [.expr (binary .assign rc (id "rc"))],
  .function .staticInline (.named "void") "lean_internal_add_rc" [p "o" obj,p "add" int]
    [.expr (binary .assign rc (.cast int (binary .add (.cast uint rc) (.cast uint (id "add")))))],
  .function .staticInline (.named "void") "lean_internal_sub_rc" [p "o" obj,p "sub" int]
    [.expr (binary .subAssign rc (id "sub"))]
]

private def integers : Header := [
  value "lean_int16_neg" u16 [p "a" u16] (.unary .neg (id "a")),
  value "lean_int16_to_int32" u32 [p "a" u16] (.cast u32 (.cast i32 (.cast i16 (id "a")))),
  value "lean_int32_neg" u32 [p "a" u32] (.unary .neg (id "a")),
  value "lean_int32_complement" u32 [p "a" u32] (.cast u32 (.unary .bitNot (.cast i32 (id "a")))),
  value "lean_int32_to_int64" u64 [p "a" u32] (.cast u64 (.cast i64 (.cast i32 (id "a")))),
  value "lean_int64_dec_lt" u8 [p "a" u64,p "b" u64]
    (binary .lt (.cast i64 (id "a")) (.cast i64 (id "b")))
] ++
  [("add",BinaryOp.add),("sub",.sub)].map (fun (name,op) =>
    value ("lean_int32_" ++ name) u32 [p "a" u32,p "b" u32] (binary op (id "a") (id "b"))) ++
  [value "lean_int32_mul" u32 [p "a" u32,p "b" u32]
    (binary .mul (binary .mul (.num 1 "U") (id "a")) (id "b"))] ++
  [("land",BinaryOp.bitAnd),("lor",.bitOr)].map (fun (name,op) =>
    value ("lean_int32_" ++ name) u32 [p "a" u32,p "b" u32]
      (.cast u32 (binary op (.cast i32 (id "a")) (.cast i32 (id "b"))))) ++
  [("dec_eq",BinaryOp.eq),("dec_le",.le),("dec_lt",.lt)].map (fun (name,op) =>
    value ("lean_int32_" ++ name) u8 [p "a" u32,p "b" u32]
      (binary op (.cast i32 (id "a")) (.cast i32 (id "b"))))

/-- Add these declarations to primitiveNames as well as the final header. -/
def header : C.Header := pointersAndRc ++ integers

end ZrAbi.ExtraPrimitives
