import Lean

/-! The standard-library shim (ZrLean/Low/InitShim.lean) handles reference
counts by hand, assuming upstream's compiled signatures. This re-reads them
from the toolchain and fails if any differs (after a Lean upgrade, say).
Run: `lake env lean --run ZrInitCheck.lean`. -/
open Lean IR

/-- (upstream declaration, its C name, the signature our code assumes). -/
def table : List (Name × String × String) :=
  [(`outOfBounds._redArg, "l_outOfBounds___redArg", "[@&tobj] -> tobj"),
   (``Char.ofNat, "l_Char_ofNat", "[@&tobj] -> u32"),
   (``Nat.reprFast, "l_Nat_reprFast", "[tobj] -> obj"),
   (``ByteArray.extract, "l_ByteArray_extract", "[@&obj, tobj, @&tobj] -> obj"),
   (`Array.reverse._redArg, "l_Array_reverse___redArg", "[obj] -> obj"),
   (`Array.extract._redArg, "l_Array_extract___redArg", "[@&obj, tobj, tobj] -> obj"),
   (`Array.append._redArg, "l_Array_append___redArg", "[obj, @&obj] -> obj"),
   (`Array.instInhabited._redArg, "l_Array_instInhabited___redArg", "[void] -> obj"),
   (``Int.toNat, "l_Int_toNat", "[@&tobj] -> tobj"),
   (``Int.pow, "l_Int_pow", "[@&tobj, @&tobj] -> tobj"),
   (``Int.shiftRight, "l_Int_shiftRight", "[@&tobj, @&tobj] -> tobj"),
   (``Int.shiftLeft, "l_Int_shiftLeft", "[@&tobj, @&tobj] -> tobj"),
   (``Int.instInhabited, "l_Int_instInhabited", "[] -> tobj")]

def sig (d : Decl) : String :=
  s!"{(d.params.map (fun p => (if p.borrow then "@&" else "") ++ toString p.ty)).toList} -> {d.resultType}"

unsafe def main : IO UInt32 := do
  initSearchPath (← findSysroot)
  enableInitializersExecution
  let env ← importModules #[{ module := `Init }] {} (loadExts := true)
  let mut bad := 0
  for (n, c, want) in table do
    match findEnvDecl env n with
    | some d =>
      let ok := n.mangle == c && sig d == want
      IO.println s!"{if ok then "ok " else "BAD"} {c}: upstream {sig d}, assumed {want}"
      if !ok then bad := bad + 1
    | none => IO.println s!"BAD {n}: no IR declaration upstream"; bad := bad + 1
  IO.println (if bad == 0 then "zrinit-check: PASS" else s!"zrinit-check: FAIL ({bad})")
  return if bad == 0 then 0 else 1
