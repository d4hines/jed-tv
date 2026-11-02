prelude
import Init.System.IO

namespace MenuShimCheck

@[extern "zr_low_initialize_Init"] opaque bootstrap (builtin : UInt8) : BaseIO USize
@[extern "zr_tv_sys"] opaque sys (nr a0 a1 a2 a3 : USize) : BaseIO USize
@[extern "zr_tv_write"] opaque writeRaw (fd : USize) (b : @& ByteArray) (off len : USize) : BaseIO USize

@[extern "l_List_reverse___redArg"] opaque reverse (xs : List String) : List String
@[extern "l_List_appendTR___redArg"] opaque append (xs ys : List String) : List String
@[extern "l_List_range"] opaque range (n : Nat) : List Nat
@[extern "l_List_replicateTR___redArg"] opaque replicate (n : Nat) (value : String) : List String
@[extern "l_List_zipIdxTR___redArg"] opaque zipIdx (xs : List String) (start : @& Nat) : List (String × Nat)

@[extern "zr_low_header_string_cstr"] opaque stringData (s : @& String) : USize
@[extern "zr_low_shim_mk_string_from_bytes"] opaque fromBytes (p n : USize) : String

@[extern "l_ByteArray_isEmpty"] opaque bytesEmpty (b : @& ByteArray) : Bool

structure Slice where
  text : String
  first : Nat
  last : Nat

@[extern "l_String_Slice_Pos_nextn"] opaque nextn (slice : @& Slice) (pos count : Nat) : Nat
@[extern "l_String_Slice_toString"] opaque sliceText (slice : @& Slice) : String
@[extern "lean_string_memcmp"] opaque memcmp (left right : @& String) (lo ro n : @& Nat) : UInt8

private partial def say (text : String) : BaseIO Unit := do
  let bytes := (text ++ "\n").toUTF8
  let n ← writeRaw 1 bytes 0 bytes.size.toUSize
  if n == 0 then pure ()

private def ensure (yes : Bool) (name : String) : IO Unit :=
  unless yes do throw (IO.userError name)

private def checks : IO Unit := do
  let xs := ["a", "b", "c"]
  ensure (reverse xs == ["c", "b", "a"] && xs == ["a", "b", "c"]) "shared reverse"
  ensure (reverse ["a"] != ["b"]) "string-list inequality"
  ensure (reverse (reverse ["left", "right"]) == ["left", "right"]) "exclusive reverse"
  let left := ["one", "two"]
  let right := ["three"]
  ensure (append left right == ["one", "two", "three"] && left == ["one", "two"] && right == ["three"]) "append"
  ensure (append [] [] == [] && reverse [] == []) "empty lists"
  ensure (range 4 == [0, 1, 2, 3] && range 0 == []) "range"
  let held := "unchanged"
  ensure (replicate 3 held == [held, held, held] && held == "unchanged") "replicate ownership"
  ensure (replicate 0 held == []) "empty replicate"
  let large : Nat := 1099511627776
  ensure (zipIdx xs large == [("a", large), ("b", large + 1), ("c", large + 2)]) "zip big Nat"
  ensure (large == 1099511627776 && xs == ["a", "b", "c"]) "zip borrowed start/shared list"
  ensure (zipIdx [] large == []) "empty zip"
  ensure (bytesEmpty ByteArray.empty) "empty bytes"
  ensure (!bytesEmpty "x".toUTF8) "nonempty bytes"
  let nativeText := "é🙂"
  let copied := fromBytes (stringData nativeText) nativeText.utf8ByteSize.toUSize
  ensure (copied == nativeText && copied.length == 2) "native UTF-8 string length"
  ensure (fromBytes (stringData nativeText) 0 == "") "native empty string"
  let unicode : Slice := ⟨"aé🙂b", 1, 7⟩
  ensure (nextn unicode 0 0 == 0) "next zero"
  ensure (nextn unicode 0 1 == 2) "next two-byte character"
  ensure (nextn unicode 2 1 == 6) "next four-byte character"
  ensure (nextn unicode 0 large == 6) "next bounded at slice end"
  ensure (sliceText unicode == "é🙂") "Unicode slice"
  ensure (sliceText ⟨"abcd", 2, 2⟩ == "") "empty slice"
  ensure (memcmp "xxé🙂" "é🙂yy" 2 0 6 == 1) "memcmp offsets"
  ensure (memcmp "abc" "abd" 0 0 3 == 0) "memcmp unequal"
  ensure (memcmp "abc" "abc" 2 0 2 == 0) "memcmp bounds"
  ensure (memcmp "" "" 0 0 0 == 1) "memcmp empty"

private def repeatChecks : Nat → IO Unit
  | 0 => pure ()
  | n+1 => do checks; repeatChecks n

@[export zr_menu_shim_test_main]
def main (_argc : UInt32) : BaseIO UInt32 := do
  match ← (repeatChecks 200).toBaseIO with
  | .ok _ => say "PASS: menu shims, 5000 assertions"
  | .error (.userError text) => say ("FAIL: " ++ text)
  | .error _ => say "FAIL: unexpected error"
  let _ ← sys 36 0 0 0 0
  let _ ← sys 88 0xfee1dead 672274793 0x4321fedc 0
  pure 0

end MenuShimCheck
