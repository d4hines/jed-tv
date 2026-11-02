/-
ZrLow memory access (AGENTS.md): the primitives low-level Lean code
reads and writes memory with. Typed Lean declarations in
`ZrAbi.LowPrimitives` generate the inline load/store definitions; there is
no handwritten C expression, allocation or boxed value. Addresses are
`USize`s (pointer width: 32 bits on the board, 64 on the host tests), data
`UInt8/16/32` or a pointer-width word.

All are `BaseIO` actions: the compiler keeps their order and never
duplicates or drops one, and a function written in `BaseIO` over scalars
compiles to a plain C function over scalars (AGENTS.md, "ABI").
Every access is `volatile`, so the C compiler keeps them in order too.

Each access is a typed pointer cast and volatile dereference in the generated
interoperability header. None checks its address: the caller owns that.
-/
namespace ZrLean.Low

/-- The byte at `a`. -/
@[extern "zr_prim_low_ld8"]
opaque ld8 (a : USize) : BaseIO UInt8

/-- The little-endian halfword at `a` (aligned). -/
@[extern "zr_prim_low_ld16"]
opaque ld16 (a : USize) : BaseIO UInt16

/-- The word at `a` (aligned). -/
@[extern "zr_prim_low_ld32"]
opaque ld32 (a : USize) : BaseIO UInt32

/-- The pointer-width word at `a` (aligned): a link in a list of blocks. -/
@[extern "zr_prim_low_ldp"]
opaque ldp (a : USize) : BaseIO USize

/-- Store the byte `v` at `a`. -/
@[extern "zr_prim_low_st8"]
opaque st8 (a : USize) (v : UInt8) : BaseIO Unit

/-- Store the halfword `v` at `a` (aligned). -/
@[extern "zr_prim_low_st16"]
opaque st16 (a : USize) (v : UInt16) : BaseIO Unit

/-- Store the word `v` at `a` (aligned). -/
@[extern "zr_prim_low_st32"]
opaque st32 (a : USize) (v : UInt32) : BaseIO Unit

/-- Store the pointer-width word `v` at `a` (aligned). -/
@[extern "zr_prim_low_stp"]
opaque stp (a : USize) (v : USize) : BaseIO Unit

/-- The bytes of a pointer: 4 on the board, 8 on the 64-bit host tests (the
argument is ignored). A constant is written as a function of a dummy
argument, with `never_extract`, because Lean makes a closed term or an
extern constant into a module constant, initialised at startup through
`lean_once`, which ZrLow code must not read (`zr-cgen check`). The C
compiler folds the expression. -/
@[never_extract, extern "zr_prim_low_word_bytes"]
opaque wordBytes (z : USize) : USize

/-- `v` rounded up to a multiple of `a`, a power of two (wrapping, as the
C it replaces: `(v + a - 1) & ~(a - 1)`). -/
@[inline] def roundUp (v a : USize) : USize := (v + a - 1) &&& ~~~(a - 1)

end ZrLean.Low
