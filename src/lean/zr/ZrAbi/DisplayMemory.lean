import ZrAbi.C

/-! The compositor's native memory effects. These are ordinary typed loads,
stores and nonoverlapping copies, with no runtime or allocator dependency. -/
namespace ZrAbi.DisplayMemory
open C
private def t := CType.named
private def id := Expr.ident
private def ptr (ty : CType) (name : String) := Expr.cast (.ptr ty) (id name)
def header : Header := [
  .pragmaOnce, .include "stdint.h", .include "stddef.h",
  .function .staticInline (t "uint32_t") "tv_display_load" [⟨"address", t "size_t"⟩]
    [.return_ (some (.unary .deref (ptr (t "uint32_t") "address")))],
  .function .staticInline (t "uint8_t") "tv_display_store" [⟨"address",t "size_t"⟩,⟨"pixel",t "uint32_t"⟩]
    [.expr (.binary .assign (.unary .deref (ptr (t "uint32_t") "address")) (id "pixel")), .return_ (some (.num 0))],
  .function .staticInline (t "uint8_t") "tv_display_copy" [⟨"dst",t "size_t"⟩,⟨"src",t "size_t"⟩,⟨"pixels",t "uint32_t"⟩]
    [.expr (.call (id "__builtin_memcpy") [ptr (t "void") "dst",ptr (.const (t "void")) "src",
      .binary .mul (.cast (t "size_t") (id "pixels")) (.sizeof (t "uint32_t"))]),
     .return_ (some (.num 0))]]
end ZrAbi.DisplayMemory
