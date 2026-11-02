import ZrAbi.RuntimeDecls
import ZrAbi.RuntimePrimitives
import ZrAbi.LowPrimitives
import ZrAbi.SystemPrimitives
import ZrAbi.RuntimeLowPrimitives
import ZrAbi.ExtraPrimitives
import ZrAbi.CcmpPrimitives
import ZrAbi.Crc32Primitives
import ZrAbi.MarkPrimitives

/- Lean-authored object layout, ABI declarations and terminal primitive
   lowering for the freestanding runtime. Scalar Lean implementations are
   compiled and inlined into the generated header by the build pipeline. -/
namespace ZrAbi.Runtime
open C RuntimeDecls

private def id := Expr.ident
private def t := CType.named
private def num (n : Nat) := Expr.num n
private def bin := Expr.binary
private def call (name : String) (args : List Expr) := Expr.call (id name) args

private def constants : Header :=
  ([ ("LEAN_CLOSURE_MAX_ARGS",16), ("LEAN_OBJECT_SIZE_DELTA",8),
     ("LEAN_MAX_SMALL_OBJECT_SIZE",4096), ("LEAN_MAX_CTOR_FIELDS",256),
     ("LEAN_MAX_CTOR_SCALARS_SIZE",1024), ("LEAN_LINEAR_MARK_MASK",128),
     ("LeanMaxCtorTag",243), ("LeanPromise",244), ("LeanClosure",245),
     ("LeanArray",246), ("LeanStructArray",247), ("LeanScalarArray",248),
     ("LeanString",249), ("LeanMPZ",250), ("LeanThunk",251), ("LeanTask",252),
     ("LeanRef",253), ("LeanExternal",254), ("LeanReserved",255),
     ("LEAN_TASK_STATE_WAITING",0), ("LEAN_TASK_STATE_RUNNING",1), ("LEAN_TASK_STATE_FINISHED",2)
   ].map fun (name,n) => .define name (num n)) ++ [
    .ifdef "LEAN_MAX_SMALL_NAT" [] [.define "LEAN_MAX_SMALL_NAT" (bin .shr (id "SIZE_MAX") (num 1))],
    .define "LEAN_MAX_SMALL_INT" (.cond (bin .eq (.sizeof (.ptr (t "void"))) (num 8))
      (id "INT_MAX") (bin .shr (id "INT_MAX") (num 1))),
    .define "LEAN_MIN_SMALL_INT" (.cond (bin .eq (.sizeof (.ptr (t "void"))) (num 8))
      (id "INT_MIN") (bin .shr (id "INT_MIN") (num 1))),
    .define "LEAN_RC_INC_MAX" (.cast (t "size_t") (num 65536)),
    .define "LEAN_RC_STICKY" (bin .add (id "INT_MIN") (num 268435456)),
    .define "LEAN_RC_STICKY_DROP" (bin .add (id "INT_MIN") (num 536870912)),
    .define "LEAN_RC_STUCK_ST" (bin .add (id "INT_MIN") (.cast (t "int") (id "LEAN_RC_INC_MAX"))),
    .define "LEAN_ONCE_CELL_INITIALIZER" (.initializer [(some "state",num 0),(some "lock",num 0)])
  ]

private def packedBytes (ty : CType) (names : List String) : Expr :=
  ((names.zipIdx).map fun (name,i) => bin .shl (.cast ty (id name)) (num (8*i))).foldl
    (bin .bitOr) (num 0)

private def scalarLiteral : Header := Id.run do
  let names := (List.range 8).map fun i => "b" ++ toString (i+1)
  let objectPtr := CType.ptr (t "lean_object")
  return [.if_ (bin .eq (id "__SIZEOF_POINTER__") (num 4))
    [.macro "LEAN_SCALAR_PTR_LITERAL" names (.splice [
      .cast objectPtr (packedBytes (t "uint32_t") (names.take 4)),
      .cast objectPtr (packedBytes (t "uint32_t") (names.drop 4))])]
    [.macro "LEAN_SCALAR_PTR_LITERAL" names (.cast objectPtr (packedBytes (t "uint64_t") names))]]

private def macros : Header := [
  .flag "LEAN_EXPORT", .flag "LEAN_USING_STD",
  .attributeMacro "LEAN_NORETURN" [.noreturn],
  .attributeMacro "LEAN_ATTR_MALLOC" [.malloc],
  .ifdef "NDEBUG" [.attributeMacro "LEAN_ALWAYS_INLINE" [.alwaysInline]] [.flag "LEAN_ALWAYS_INLINE"],
  .macro "LEAN_LIKELY" ["x"] (call "__builtin_expect" [id "x",num 1]),
  .macro "LEAN_UNLIKELY" ["x"] (call "__builtin_expect" [id "x",num 0]),
  .macro "LEAN_ALLOCA" ["s"] (call "__builtin_alloca" [id "s"]),
  .macro "LEAN_BYTE" ["v","i"] (.unary .deref (bin .add
    (.cast (.ptr (t "uint8_t")) (.unary .addr (id "v"))) (id "i"))),
  .macro "LEAN_CASSERT" ["p"] (call "_Static_assert" [id "p",.str "Lean runtime ABI assertion"]),
  .ifdef "assert" [] [.ifdef "NDEBUG"
    [.macro "assert" ["p"] (.cast (t "void") (num 0))]
    [.macro "assert" ["p"] (.cond (id "p") (.cast (t "void") (num 0))
      (call "lean_notify_assert" [id "__FILE__",id "__LINE__",.stringize "p"]))]]
]

private partial def declNames : Decl → List String
  | .function _ _ name _ _ => [name]
  | .guard _ body => body.flatMap declNames
  | .ifdef _ yes no | .if_ _ yes no => (yes ++ no).flatMap declNames
  | _ => []

def primitiveNames : List String := (RuntimePrimitives.header ++ ExtraPrimitives.header ++ MarkPrimitives.header).flatMap declNames

abbrev Binding := String × Nat

private def isPointer : CType → Bool
  | .ptr _ | .fnPtr _ _ => true
  | .const a | .volatile a => isPointer a
  | .named name => ["lean_obj_arg","b_lean_obj_arg","u_lean_obj_arg","lean_obj_res","b_lean_obj_res",
      "lean_external_finalize_proc","lean_external_foreach_proc"].contains name
  | _ => false

private def aliases (bindings : List Binding) : Header := bindings.filterMap fun (name,arity) => do
  let sig ← functions.find? (·.name == name)
  let target := "zr_inline_zr_low_header_" ++ (name.drop 5).toString
  let args := sig.params.map fun p =>
    if isPointer p.type then Expr.cast (t "size_t") (id p.name) else id p.name
  let args := if args.isEmpty && arity == 1 then [num 0] else args
  let body := if sig.result matches .named "void" then
    [Stmt.expr (call target args), .return_ none]
  else [.return_ (some (.cast sig.result (call target args)))]
  return .function .staticInline sig.result name sig.params body

/-- The pinned historical header forces these small scalar/RC/Nat facades
to inline in release builds. This is compiler metadata, not an algorithm. -/
def alwaysInlineNames : List String := [
  "lean_is_scalar", "lean_dec_ref", "lean_inc", "lean_dec",
  "lean_nat_add", "lean_nat_sub", "lean_nat_mul",
  "lean_nat_eq", "lean_nat_dec_eq", "lean_nat_le", "lean_nat_dec_le",
  "lean_nat_lt", "lean_nat_dec_lt"
]

def header (bindings : List Binding) : Header := Id.run do
  let implemented := primitiveNames ++ bindings.map Prod.fst
  let prototypes := functions.map fun f =>
    let storage := if implemented.contains f.name then Storage.staticInline else .plain
    let attrs := if ["lean_internal_panic","lean_internal_panic_out_of_memory","lean_internal_panic_unreachable",
      "lean_internal_panic_rc_overflow","lean_internal_panic_overflow"].contains f.name then [Attribute.noreturn] else []
    let ordinary := Decl.prototype f.result f.name f.params storage attrs
    if alwaysInlineNames.contains f.name then
      .ifdef "NDEBUG" [Decl.prototype f.result f.name f.params storage (attrs ++ [.alwaysInline])] [ordinary]
    else ordinary
  return [.pragmaOnce] ++
    (["stddef.h","stdbool.h","stdint.h","limits.h","float.h","stdatomic.h","string.h","lean/config.h"].map .include) ++
    macros ++ constants ++ RuntimeDecls.layout ++ scalarLiteral ++ prototypes ++
    [.variable .extern_ (.array (t "size_t") none) "zr_the_heap"] ++
    RuntimePrimitives.header ++ ExtraPrimitives.header ++ LowPrimitives.header ++ SystemPrimitives.header ++ RuntimeLowPrimitives.header ++ MarkPrimitives.header ++ CcmpPrimitives.header ++ Crc32Primitives.header ++
    [.include "zr_inline.generated.h" false] ++ aliases bindings

end ZrAbi.Runtime
