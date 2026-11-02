import ZrAbi.C
import ZrAbi.MemoryTypes

/- The primitive boundary of scalar Lean: typed loads, fields, layout queries
   and compiler effects. Allocation and object algorithms remain ordinary
   ZrLean.Low definitions, including the deletion-list link operations. -/
namespace ZrAbi.LowPrimitives
open C

private def sz : CType := .named "size_t"
private def u8 : CType := .named "uint8_t"
private def u16 : CType := .named "uint16_t"
private def u32 : CType := .named "uint32_t"
private def obj : CType := .ptr (.named "lean_object")
private def void : CType := .named "void"
private def id := Expr.ident
private def call (name : String) (args : List Expr) : Expr := .call (id name) args
private def p (name : String) (type : CType) : Param := ⟨name,type⟩
private def ret (e : Expr) : Stmt := .return_ (some e)
private def unit : Expr := call "lean_box" [.num 0]
private def fn (name : String) (result : CType) (params : List Param) (body : List Stmt) : Decl :=
  .function .staticInline result name params body
private def value (name : String) (result : CType) (params : List Param) (e : Expr) : Decl :=
  fn name result params [ret e]
private def action (name : String) (params : List Param) (e : Expr) : Decl :=
  fn name obj params [.expr e,ret unit]
private def mem (type : CType) (a : Expr) : Expr :=
  .unary .deref (.cast (.ptr (.volatile (MemoryTypes.rawType type))) (.cast (.named "uintptr_t") a))

private def memory : Header :=
  [("8",u8),("16",u16),("32",u32),("p",sz)].flatMap fun (suffix,type) => [
    value ("zr_prim_low_ld" ++ suffix) type [p "a" sz] (mem type (id "a")),
    action ("zr_prim_low_st" ++ suffix) [p "a" sz,p "v" type]
      (.binary .assign (mem type (id "a")) (.cast type (id "v")))
  ]

private def layouts : Header :=
  let sizes := [("ctor_objs","lean_ctor_object"),("arr_data","lean_array_object"),
    ("sarr_data","lean_sarray_object"),("str_data","lean_string_object"),
    ("clo_objs","lean_closure_object")]
  let offsets := [("arr_size_off","lean_array_object","m_size"),
    ("arr_cap_off","lean_array_object","m_capacity"),
    ("sarr_size_off","lean_sarray_object","m_size"),
    ("sarr_cap_off","lean_sarray_object","m_capacity"),
    ("str_size_off","lean_string_object","m_size"),
    ("str_cap_off","lean_string_object","m_capacity"),
    ("clo_fixed_off","lean_closure_object","m_num_fixed")]
  value "zr_prim_low_word_bytes" sz [p "z" sz] (.sizeof sz) ::
    sizes.map (fun (name,type) => value ("zr_prim_low_" ++ name) sz [p "z" sz] (.sizeof (.named type))) ++
    offsets.map (fun (name,type,field) => value ("zr_prim_low_" ++ name) sz [p "z" sz] (.offsetof (.named type) field))

private def objectPrimitives : Header := [
  value "zr_prim_low_array_header" u32 [p "z" sz] (.cast u32 (.num 0xF6000000 "U")),
  value "zr_prim_low_sarray_header" u32 [p "elem" sz]
    (.binary .bitOr (.cast u32 (.num 0xF8000000 "U"))
      (.binary .shl (.cast u32 (id "elem")) (.num 16))),
  value "zr_prim_low_sgt32" u8 [p "a" u32,p "b" u32]
    (.binary .gt (.cast (.named "int32_t") (id "a")) (.cast (.named "int32_t") (id "b"))),
  value "zr_prim_low_size_max" sz [p "z" sz] (.cast sz (.unary .neg (.num 1))),
  fn "zr_prim_low_mul_ovf" u8 [p "a" sz,p "b" sz] [
    .local sz "result" none,
    ret (.cast u8 (call "__builtin_mul_overflow" [id "a",id "b",.unary .addr (id "result")]))
  ],
  action "zr_prim_low_copy" [p "d" sz,p "s" sz,p "n" sz]
    (call "ZR_LOW_COPY" [id "d",id "s",id "n"]),
  action "zr_prim_low_move" [p "d" sz,p "s" sz,p "n" sz]
    (call "ZR_LOW_MOVE" [id "d",id "s",id "n"]),
  action "zr_prim_low_count_nonlinear" [p "bytes" sz]
    (call "ZR_LOW_COUNT_NONLINEAR" [id "bytes"]),
  action "zr_prim_low_count_alloc" [p "bytes" sz]
    (call "ZR_LOW_COUNT_ALLOC" [id "bytes"]),
  .prototype void "zr_low_fatal" [p "code" sz] .extern_ [.noreturn],
  fn "zr_prim_low_fatal" sz [p "code" sz] [.expr (call "zr_low_fatal" [id "code"]),ret (.num 0)]
]

private def heapField (type field : String) : Expr :=
  .arrow (.cast (.ptr (.named type)) (.cast (.named "uintptr_t") (id "a"))) field

private def fast : Header := [
  -- A known scalar-array byte field retains pointer/GEP provenance. Generic
  -- raw HeaderMem accesses remain may_alias for arbitrary object storage.
  value "zr_prim_sarray_byte_get" u8 [p "a" sz,p "i" sz]
    (.index (heapField "lean_sarray_object" "m_data") (id "i")),
  value "zr_prim_fast_likely" u8 [p "b" u8]
    (call "__builtin_expect" [.binary .ne (id "b") (.num 0),.num 1]),
  value "zr_prim_fast_load_count" u32 [p "a" sz] (heapField "lean_object" "m_rc"),
  value "zr_prim_fast_array_size" sz [p "a" sz] (heapField "lean_array_object" "m_size"),
  value "zr_prim_fast_array_cap" sz [p "a" sz] (heapField "lean_array_object" "m_capacity"),
  action "zr_prim_fast_array_set_size" [p "a" sz,p "n" sz]
    (.binary .assign (heapField "lean_array_object" "m_size") (id "n")),
  action "zr_prim_fast_array_set" [p "a" sz,p "i" sz,p "v" sz]
    (.binary .assign (.index (heapField "lean_array_object" "m_data") (id "i"))
      (.cast obj (.cast (.named "uintptr_t") (id "v")))),
  value "zr_prim_fast_byte_size" sz [p "a" sz] (heapField "lean_sarray_object" "m_size"),
  value "zr_prim_fast_byte_cap" sz [p "a" sz] (heapField "lean_sarray_object" "m_capacity"),
  action "zr_prim_fast_byte_set_size" [p "a" sz,p "n" sz]
    (.binary .assign (heapField "lean_sarray_object" "m_size") (id "n")),
  action "zr_prim_fast_byte_set" [p "a" sz,p "i" sz,p "v" u8]
    (.binary .assign (.index (heapField "lean_sarray_object" "m_data") (id "i")) (id "v"))
]

private def allocator : Header := [
  value "zr_prim_alloc_heap" sz [p "z" sz] (.cast sz (id "zr_the_heap")),
  .prototype (.ptr void) "zr_rt_mem_alloc" [p "size" sz,p "align" sz] .extern_,
  .prototype void "zr_rt_mem_free" [p "p" (.ptr void),p "size" sz,p "align" sz] .extern_,
  .prototype void "zr_low_oom" [] .extern_ [.noreturn],
  value "zr_prim_alloc_mem_alloc" sz [p "size" sz,p "align" sz]
    (.cast sz (call "zr_rt_mem_alloc" [id "size",id "align"])),
  action "zr_prim_alloc_mem_free" [p "p" sz,p "size" sz,p "align" sz]
    (call "zr_rt_mem_free" [.cast (.ptr void) (id "p"),id "size",id "align"]),
  fn "zr_prim_alloc_oom" sz [p "z" sz] [.expr (call "zr_low_oom" []),ret (.num 0)]
]

/-- Append after the object layouts, lean_box and extern heap declaration. -/
def header : C.Header := MemoryTypes.header ++ [.guard "ZR_LOW_PRIMITIVES_H" (memory ++ layouts ++ objectPrimitives ++ fast ++ allocator)]

end ZrAbi.LowPrimitives
