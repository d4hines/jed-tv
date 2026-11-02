import ZrAbi.C

/- Typed pointer/word ABI casts and one explicit compile-time selector.
No AES/CCMP algorithm is authored as C expressions or templates. -/
namespace ZrAbi.CcmpPrimitives
open C
private def sz : CType := .named "size_t"
private def u8 : CType := .named "uint8_t"
private def u32 : CType := .named "uint32_t"
private def obj : CType := .ptr (.named "lean_object")
private def p (name : String) (type : CType) : Param := ⟨name,type⟩
private def id := Expr.ident

private def scalarParams : List Param :=
  [p "w" sz,p "tables" sz,p "dec" u8,p "a0" u32,p "a1" u32,p "a2" u32,p "a3" u32,
   p "buf" sz,p "u" sz,p "n" u32,p "i" u32,p "x0" u32,p "x1" u32,p "x2" u32,p "x3" u32]
private def bridgeParams : List Param :=
  scalarParams.map fun a => if ["w","tables","buf"].contains a.name then {a with type := obj} else a
private def castArgs : List Expr :=
  bridgeParams.map fun a => if ["w","tables","buf"].contains a.name then .cast sz (id a.name) else id a.name
private def rawParams : List Param := ["w","tables","data","state"].map (fun name => p name sz)
private def rawCall (target : String) : Decl :=
  .function .staticInline u8 "zr_prim_ccmp_raw" rawParams
    [.return_ (some (.call (id target) (rawParams.map fun a => id a.name)))]
private def enabled (value : Nat) : Decl :=
  .function .staticInline u8 "zr_ccmp_kernel_enabled" [p "unit" u8] [.return_ (some (.num value))]

def header : Header := [
  .prototype sz "zr_low_ccmp_blocks" scalarParams .extern_,
  .prototype u8 "zr_low_ccmp_blocks_reference" rawParams .extern_,
  .prototype u8 "zr_ccmp_blocks_arm" rawParams .extern_,
  .ifdef "__arm__" [rawCall "zr_ccmp_blocks_arm"] [rawCall "zr_low_ccmp_blocks_reference"],
  .ifdef "ZR_CCMP_BLOCK_KERNEL" [enabled 1] [enabled 0],
  .function .staticInline obj "zr_ccmp_blocks_kernel" bridgeParams
    [.return_ (some (.cast obj (.call (id "zr_low_ccmp_blocks") castArgs)))]
]
end ZrAbi.CcmpPrimitives
