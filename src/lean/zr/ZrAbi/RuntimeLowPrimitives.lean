import ZrAbi.C
import ZrAbi.MemoryTypes

/- Typed scalar interoperability for the runtime. Callback types, data
addresses, literal data, field layouts and compiler effects are described
here; runtime algorithms remain ordinary ZrLean.Low definitions. -/
namespace ZrAbi.RuntimeLowPrimitives
open C

private def sz : CType := .named "size_t"
private def up : CType := .named "uintptr_t"
private def u8 : CType := .named "uint8_t"
private def u16 : CType := .named "uint16_t"
private def u32 : CType := .named "uint32_t"
private def u64 : CType := .named "uint64_t"
private def obj : CType := .ptr (.named "lean_object")
private def void : CType := .named "void"
private def text : CType := .ptr (.const (.named "char"))
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
private def asPtr (type : CType) (e : Expr) : Expr := .cast type (.cast up e)
private def mem (type : CType) (a : Expr) : Expr :=
  .unary .deref (asPtr (.ptr (.volatile (MemoryTypes.rawType type))) a)
private def callback (result : CType) (args : List CType) (values : List Expr) : Expr :=
  .call (asPtr (.fnPtr result args) (id "fn")) values

private def globals : Header := [
  .variable .extern_ (.array u32 (some 3584)) "zr_telemetry_counters" none,
  value "zr_prim_telemetry_base" sz [p "z" sz] (.cast sz (id "zr_telemetry_counters")),
  .prototype obj "zr_pmu_tele_init" [],
  .prototype u32 "zr_pmu_tele_cycles" [],
  .prototype sz "zr_telemetry_identity" [],
  .variable .extern_ (.array sz (some 2)) "zr_runtime_state" none,
  .variable .extern_ obj "l_ByteArray_empty" none,
  .variable .extern_ (.array sz (some 3)) "zr_nat_literals" none,
  .variable .extern_ (.array sz (some 64)) "zr_mark_stack" none,
  .variable .extern_ sz "zr_fw_heap_head" none,
  value "zr_prim_runtime_state" sz [p "z" sz] (.cast sz (id "zr_runtime_state")),
  value "zr_prim_runtime_empty_cell" sz [p "z" sz] (.cast sz (.unary .addr (id "l_ByteArray_empty"))),
  value "zr_prim_bignat_literals" sz [p "z" sz] (.cast sz (id "zr_nat_literals")),
  value "zr_prim_rccold_mark_stack" sz [p "z" sz] (.cast sz (id "zr_mark_stack")),
  value "zr_prim_heap_cell" sz [p "z" sz] (.cast sz (.unary .addr (id "zr_fw_heap_head")))
]

private def constants : Header := [
  value "zr_prim_runtime_ok_header" u32 [p "z" sz] (.cast u32 (.num 0x00010000 "U")),
  value "zr_prim_runtime_boot_one" sz [p "z" sz] (.num 1),
  value "zr_prim_runtime_boot_zero" sz [p "z" sz] (.num 0),
  value "zr_prim_bignat_max_small" u64 [p "z" sz] (.cast u64 (id "LEAN_MAX_SMALL_NAT")),
  value "zr_prim_bignat_mpz_header" u32 [p "z" sz] (.cast u32 (.num 0xFA000000 "U")),
  value "zr_prim_bignat_object_bytes" sz [p "z" sz] (.binary .add (.sizeof (.named "lean_object")) (.num 16)),
  value "zr_prim_apply_fun_off" sz [p "z" sz] (.offsetof (.named "lean_closure_object") "m_fun"),
  value "zr_prim_apply_arity_off" sz [p "z" sz] (.offsetof (.named "lean_closure_object") "m_arity"),
  value "zr_prim_apply_closure_header" u32 [p "z" sz] (.cast u32 (.num 0xF5000000 "U")),
  value "zr_prim_heap_grain" sz [p "z" sz] (.binary .mul (.num 2) (.sizeof sz)),
  value "zr_prim_div_clz" u32 [p "v" u32] (call "__builtin_clz" [id "v"])
] ++ [8,16,32,64].map (fun bits =>
  value ("zr_prim_bignat_bits_int" ++ toString bits) (.named ("uint" ++ toString bits ++ "_t"))
    [p "v" u64] (.cast (.named ("uint" ++ toString bits ++ "_t"))
      (.cast (.named ("int" ++ toString bits ++ "_t")) (id "v"))))

private def once : Header :=
  [("uint8",u8),("uint16",u16),("uint32",u32),("uint64",u64),
   ("usize",sz),("float",.named "double"),("float32",.named "float")].map (fun (name,type) =>
    value ("zr_prim_once_call_" ++ name) type [p "fn" sz] (callback type [] [])) ++
  [value "zr_prim_once_call_obj" sz [p "fn" sz] (.cast sz (callback obj [] []))] ++
  [("64",u64),("_float",.named "double"),("_float32",.named "float")].flatMap (fun (suffix,type) => [
    value ("zr_prim_once_ld" ++ suffix) type [p "a" sz] (mem type (id "a")),
    action ("zr_prim_once_st" ++ suffix) [p "a" sz,p "v" type]
      (.binary .assign (mem type (id "a")) (id "v"))
  ])

private def apply : Header :=
  -- Allocate in the generated caller's frame, never a helper's frame.
  -- `zr-cgen abi-normalize` removes only the redundant compiler-emitted prototype
  -- for this functionlike macro; all other generated code stays unchanged.
  [.macro "zr_prim_apply_stack" ["bytes"] (.cast sz (call "__builtin_alloca" [id "bytes"]))] ++
  (List.range 16).map (fun index =>
    let arity := index + 1
    let names := (List.range arity).map (fun i => "a" ++ toString (i + 1))
    value ("zr_prim_apply_invoke" ++ toString arity) sz
      (p "fn" sz :: names.map (fun name => p name sz))
      (.cast sz (callback obj (List.replicate arity obj) (names.map (fun name => asPtr obj (id name)))))) ++
  [value "zr_prim_apply_invoke_array" sz [p "fn" sz,p "args" sz]
    (.cast sz (callback obj [.ptr obj] [asPtr (.ptr obj) (id "args")]))]

private def runtimeCalls : Header := [
  .prototype void "zr_rt_fatal" [p "msg" text,p "len" sz] .extern_ [.noreturn],
  .prototype void "zr_rt_report_panic" [p "msg" text,p "len" sz] .extern_,
  fn "zr_prim_runtime_halt" sz [p "msg" sz,p "len" sz]
    [.expr (call "zr_rt_fatal" [asPtr text (id "msg"),id "len"]),ret (.num 0)],
  action "zr_prim_runtime_report" [p "msg" sz,p "len" sz]
    (call "zr_rt_report_panic" [asPtr text (id "msg"),id "len"]),
  .prototype void "zr_trace_begin" [p "tag" text] .extern_ [.weak],
  .prototype void "zr_trace_end" [] .extern_ [.weak],
  .prototype void "zr_trace_ev" [p "kind" text,p "a" u32,p "v" u32] .extern_ [.weak],
  value "zr_prim_runtime_trace_begin_fn" sz [p "z" sz] (.cast sz (id "zr_trace_begin")),
  value "zr_prim_runtime_trace_end_fn" sz [p "z" sz] (.cast sz (id "zr_trace_end")),
  value "zr_prim_runtime_trace_ev_fn" sz [p "z" sz] (.cast sz (id "zr_trace_ev")),
  action "zr_prim_runtime_call_begin" [p "fn" sz,p "tag" sz]
    (callback void [text] [asPtr text (id "tag")]),
  action "zr_prim_runtime_call_end" [p "fn" sz] (callback void [] []),
  action "zr_prim_runtime_call_ev" [p "fn" sz,p "kind" sz,p "a" u32,p "v" u32]
    (callback void [text,u32,u32] [asPtr text (id "kind"),id "a",id "v"])
]

private def strings : Header :=
  [
    ("zr_prim_ref_empty_message","lean: empty reference read"),
    ("zr_prim_runtime_msg0","lean: out of memory"),
    ("zr_prim_runtime_msg1","lean: freeing an object kind this runtime does not support"),
    ("zr_prim_runtime_msg2","lean: mark_persistent on an object kind this runtime does not support"),
    ("zr_prim_runtime_msg3","lean: integer overflow in runtime computation"),
    ("zr_prim_runtime_msg4","lean: Nat >= 2^128 (this runtime's big-Nat bound)"),
    ("zr_prim_runtime_msg5","Nat.shiftl exponent is too big"),
    ("zr_prim_runtime_msg6","Nat.pow exponent is too big"),
    ("zr_prim_runtime_msg7","lean: malformed Nat literal"),
    ("zr_prim_runtime_msg8","lean: closure of arity 0"),
    ("zr_prim_runtime_msg9","lean: closure arity too large"),
    ("zr_prim_runtime_msg10","Error: index out of bounds"),
    ("zr_prim_runtime_msg11","PANIC at outOfBounds Init.GetElem:18:2: index out of bounds"),
    ("zr_prim_runtime_msg12","lean: Int outside [-2^127, 2^127) (this runtime's big-Int bound)"),
    ("zr_prim_runtime_op0","probe"),
    ("zr_prim_runtime_op1","init"),
    ("zr_prim_runtime_op2","configure_openwifi"),
    ("zr_prim_runtime_op3","set_lo"),
    ("zr_prim_runtime_op4","tx_quad_cal"),
    ("zr_prim_runtime_op5","set_tx_atten"),
    ("zr_prim_runtime_op6","get_tx_atten"),
    ("zr_prim_runtime_op7","set_rx_gain"),
    ("zr_prim_runtime_op8","rssi_mdb"),
    ("zr_prim_runtime_op9","ctrl_outs"),
    ("zr_prim_runtime_op10","spi_read"),
    ("zr_prim_runtime_op11","spi_write"),
    ("zr_prim_runtime_op12","tx_mute"),
    ("zr_prim_runtime_op13","dac_datasel"),
    ("zr_prim_runtime_op14","set_rx_gain_mode"),
    ("zr_prim_runtime_op15","get_rx_gain"),
    ("zr_prim_runtime_op16","bist_tx_tone"),
    ("zr_prim_runtime_op17","bist_loopback"),
    ("zr_prim_runtime_op18","set_rx_lo"),
    ("zr_prim_runtime_op19","RT")
  ].map (fun (name,text) => value name sz [p "z" sz] (.cast sz (.str text)))

/-- Append after runtime object types and scalar boxing declarations. -/
def header : C.Header :=
  MemoryTypes.header ++ [.guard "ZR_RUNTIME_LOW_PRIMITIVES_H" (globals ++ constants ++ once ++ apply ++ runtimeCalls ++ strings)]

end ZrAbi.RuntimeLowPrimitives
