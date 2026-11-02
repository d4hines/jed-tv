import ZrAbi.C
import ZrAbi.MemoryTypes

/- Terminal operations for the Lean-to-C backend. These are typed primitive
   operations, not C source templates. Division and shift preconditions are
   established by the scalar Lean header algorithms before these raw terminal
   operations are used. Device accesses are emitted separately and volatile. -/
namespace ZrAbi.RuntimePrimitives
open C

private def id := Expr.ident
private def t (s : String) := CType.named s
private def param (s : String) (ty : CType) : Param := ⟨s, ty⟩
private def bin (op : BinaryOp) (a b : Expr) := Expr.binary op a b
private def fn (result : CType) (name : String) (params : List Param) (value : Expr) : Decl :=
  .function .staticInline result name params [.return_ (some value)]

def unsignedTypes : List (String × CType × Nat) := [
  ("uint8", t "uint8_t", 8), ("uint16", t "uint16_t", 16),
  ("uint32", t "uint32_t", 32), ("uint64", t "uint64_t", 64),
  ("usize", t "size_t", 0)
]

private def binaryOps : List (String × BinaryOp) := [
  ("add", .add), ("sub", .sub), ("mul", .mul), ("div", .div), ("mod", .mod),
  ("land", .bitAnd), ("lor", .bitOr), ("xor", .bitXor),
  ("shift_left", .shl), ("shift_right", .shr)
]
private def compareOps : List (String × BinaryOp) := [("dec_eq", .eq), ("dec_lt", .lt), ("dec_le", .le)]

private def unsignedOne (name : String) (ty : CType) (bits : Nat) : Header := Id.run do
  let args := [param "a1" ty, param "a2" ty]
  -- C promotes uint8/uint16 to signed int. Widen before arithmetic so the
  -- terminal operation retains unsigned wrapping semantics, including mul.
  let arithTy := if bits != 0 && bits < 32 then t "uint32_t" else ty
  let a := Expr.cast arithTy (id "a1")
  let b := Expr.cast arithTy (id "a2")
  let mut out := []
  for (suffix, op) in binaryOps do
    let value := Expr.cast ty (bin op a b)
    out := out ++ [fn ty ("zr_prim_" ++ name ++ "_" ++ suffix) args value]
    if suffix != "div" && suffix != "mod" && suffix != "shift_left" && suffix != "shift_right" then
      out := out ++ [fn ty ("lean_" ++ name ++ "_" ++ suffix) args value]
  for (suffix, op) in compareOps do
    let value := Expr.cast (t "uint8_t") (bin op a b)
    out := out ++ [fn (t "uint8_t") ("zr_prim_" ++ name ++ "_" ++ suffix) args value,
      fn (t "uint8_t") ("lean_" ++ name ++ "_" ++ suffix) args value]
  for (suffix, op) in [("complement", UnaryOp.bitNot), ("neg", .neg)] do
    let value := Expr.cast ty (.unary op (.cast arithTy (id "a")))
    out := out ++ [fn ty ("zr_prim_" ++ name ++ "_" ++ suffix) [param "a" ty] value,
      fn ty ("lean_" ++ name ++ "_" ++ suffix) [param "a" ty] value]
  for (target, targetTy, _) in unsignedTypes do
    if target != name then
      let value := Expr.cast targetTy (id "a")
      out := out ++ [fn targetTy ("zr_prim_" ++ name ++ "_to_" ++ target) [param "a" ty] value,
        fn targetTy ("lean_" ++ name ++ "_to_" ++ target) [param "a" ty] value]
  return out

def unsigned : Header := unsignedTypes.flatMap fun (name, ty, bits) => unsignedOne name ty bits

/-- IEEE binary32 machine operations needed by PCM conversion. Integer
conversion truncates toward zero, saturates outside Int16 and maps NaN to zero,
matching Lean's scalar ABI without undefined out-of-range C conversions. -/
def float32 : Header :=
  let f := t "float"
  let u8 := t "uint8_t"
  let u16 := t "uint16_t"
  let a := id "a"
  let b := id "b"
  [fn f "lean_int16_to_float32" [param "a" u16] (.cast f (.cast (t "int16_t") a)),
   fn f "lean_uint32_to_float32" [param "a" (t "uint32_t")] (.cast f a),
   fn f "lean_float32_mul" [param "a" f, param "b" f] (bin .mul a b),
   fn f "lean_float32_negate" [param "a" f] (.unary .neg a),
   fn u8 "lean_float32_decLt" [param "a" f, param "b" f] (.cast u8 (bin .lt a b)),
   fn u16 "lean_float32_to_int16" [param "a" f]
     (.cond (bin .ne a a) (.num 0)
       (.cond (bin .gt a (.cast f (.unary .neg (.num 32769))))
         (.cond (bin .lt a (.cast f (.num 32768))) (.cast u16 (.cast (t "int16_t") a)) (.num 32767))
         (.num 32768))),
   .function .staticInline f "lean_float32_of_bits" [param "a" (t "uint32_t")]
     [.local f "result",
      .expr (.call (id "__builtin_memcpy") [.unary .addr (id "result"), .unary .addr a, .sizeof f]),
      .return_ (some (id "result"))]]

def tags : Header := [
  fn (t "uint8_t") "lean_is_scalar" [param "o" (.ptr (t "lean_object"))]
    (.cast (t "uint8_t") (bin .eq (bin .bitAnd (.cast (t "size_t") (id "o")) (.num 1)) (.num 1))),
  fn (.ptr (t "lean_object")) "lean_box" [param "n" (t "size_t")]
    (.cast (.ptr (t "lean_object")) (bin .bitOr (bin .shl (id "n") (.num 1)) (.num 1))),
  fn (t "size_t") "lean_unbox" [param "o" (.ptr (t "lean_object"))]
    (bin .shr (.cast (t "size_t") (id "o")) (.num 1)),
  fn (t "uint8_t") "lean_strict_or" [param "a" (t "uint8_t"), param "b" (t "uint8_t")]
    (bin .or (id "a") (id "b")),
  fn (t "uint8_t") "lean_strict_and" [param "a" (t "uint8_t"), param "b" (t "uint8_t")]
    (bin .and (id "a") (id "b"))
]

private def heapTypes : List (String × CType) := [
  ("8", t "uint8_t"), ("16", t "uint16_t"), ("32", t "uint32_t"),
  ("64", t "uint64_t"), ("p", t "size_t"), ("f64", t "double"), ("f32", t "float")
]

def heapMemory : Header := heapTypes.flatMap fun (suffix, ty) =>
  let address := Expr.cast (.ptr (MemoryTypes.rawType ty)) (.cast (t "uintptr_t") (id "a"))
  [ fn ty ("zr_header_ld" ++ suffix) [param "a" (t "size_t")] (.unary .deref address),
    .function .staticInline (.ptr (t "lean_object")) ("zr_header_st" ++ suffix)
      [param "a" (t "size_t"), param "v" ty]
      [.expr (bin .assign (.unary .deref address) (id "v")),
       .return_ (some (.call (id "lean_box") [.num 0]))]
  ]

def messageData : Header := [
  fn (t "size_t") "zr_header_sarray_bad_elem_message" [param "unused" (t "uint8_t")]
    (.cast (t "size_t") (.str "scalar array element size is larger than 2^7"))
]

def header : Header := MemoryTypes.header ++ tags ++ unsigned ++ float32 ++ heapMemory ++ messageData

end ZrAbi.RuntimePrimitives
