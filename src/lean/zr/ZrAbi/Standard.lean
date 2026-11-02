import ZrAbi.C

/- Compiler-owned standard headers supply C's scalar types and atomics.
   These are only the project's freestanding memory declarations and Lean
   runtime configuration, represented as typed data rather than C bodies. -/
namespace ZrAbi.Standard
open C

private def void : CType := .named "void"
private def ptr : CType := .ptr void
private def cptr : CType := .ptr (.const void)
private def size : CType := .named "__SIZE_TYPE__"
private def uintptr : CType := .named "__UINTPTR_TYPE__"
private def param (name : String) (type : CType) : Param := ⟨name,type⟩
private def id := Expr.ident
private def call (name : String) (args : List Expr) : Expr := .call (id name) args

def stringHeader : Header := [
  .guard "ZR_STRING_H" [
    .include "stddef.h",
    .prototype ptr "memcpy" [param "dst" ptr,param "src" cptr,param "n" (.named "size_t")],
    .prototype ptr "memmove" [param "dst" ptr,param "src" cptr,param "n" (.named "size_t")],
    .prototype ptr "memset" [param "dst" ptr,param "c" (.named "int"),param "n" (.named "size_t")],
    .prototype (.named "int") "memcmp" [param "a" cptr,param "b" cptr,param "n" (.named "size_t")]
  ]
]

private def copyArgs : List Expr := [
  .cast ptr (.cast uintptr (id "d")),
  .cast cptr (.cast uintptr (id "s")),
  id "n"
]

private def countMacros (counted : Bool) : List Decl :=
  if counted then [
    .prototype void "zr_low_copy" [param "d" ptr,param "s" cptr,param "n" size],
    .prototype void "zr_low_move" [param "d" ptr,param "s" cptr,param "n" size],
    .prototype void "zr_low_count_alloc" [param "bytes" size],
    .prototype void "zr_low_count_nonlinear" [param "bytes" size],
    .macro "ZR_LOW_COPY" ["d","s","n"] (call "zr_low_copy" copyArgs),
    .macro "ZR_LOW_MOVE" ["d","s","n"] (call "zr_low_move" copyArgs),
    .macro "ZR_LOW_COUNT_ALLOC" ["n"] (call "zr_low_count_alloc" [id "n"]),
    .macro "ZR_LOW_COUNT_NONLINEAR" ["n"] (call "zr_low_count_nonlinear" [id "n"])
  ] else [
    .macro "ZR_LOW_COPY" ["d","s","n"] (.cast void (call "__builtin_memcpy" copyArgs)),
    .macro "ZR_LOW_MOVE" ["d","s","n"] (.cast void (call "__builtin_memmove" copyArgs)),
    .macro "ZR_LOW_COUNT_ALLOC" ["n"] (.cast void (id "n")),
    .macro "ZR_LOW_COUNT_NONLINEAR" ["n"] (.cast void (id "n"))
  ]

def configHeader : Header := [
  .pragmaOnce,
  .define "LEAN_IS_STAGE0" (.num 0),
  .define "LEAN_VERSION_MAJOR" (.num 4),
  .define "LEAN_VERSION_MINOR" (.num 35),
  .define "LEAN_VERSION_PATCH" (.num 0),
  .define "LEAN_VERSION_IS_RELEASE" (.num 1),
  .define "LEAN_SPECIAL_VERSION_DESC" (.str ""),
  .define "LEAN_VERSION_STRING" (.str "4.35.0"),
  .define "LEAN_PLATFORM_TARGET" (.str "armv7-none-eabihf"),
  .define "LEAN_MANUAL_ROOT" (.str "https://lean-lang.org/doc/reference/4.35.0/"),
  .flag "LEAN_SMALL_ALLOCATOR",
  .define "malloc" (id "lean_no_malloc"),
  .define "free" (id "lean_no_free"),
  .define "free_sized" (id "lean_no_free_sized"),
  .define "alloca" (id "__builtin_alloca"),
  .ifdef "ZR_RT_COUNT_COPIES" (countMacros true) (countMacros false)
]

def write (directory : System.FilePath) : IO Unit := do
  IO.FS.createDirAll (directory / "lean")
  IO.FS.writeFile (directory / "string.h") (renderHeader stringHeader)
  IO.FS.writeFile (directory / "lean" / "config.h") (renderHeader configHeader)

end ZrAbi.Standard
