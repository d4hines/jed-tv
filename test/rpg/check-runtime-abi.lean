import Lean
open Lean IR
private def expected : List (Name × String) := [
  (`List.get?Internal._redArg, "[@&tobj, tobj] -> tobj"),
  (`List.getD._redArg, "[@&tobj, tobj, @&tobj] -> tobj"),
  (`List.lengthTR._redArg, "[@&tobj] -> tobj"),
  (`List.drop._redArg, "[tobj, @&tobj] -> tobj"),
  (`List.isEmpty._redArg, "[@&tobj] -> u8"),
  (`Option.instDecidableEq._redArg, "[obj, tobj, tobj] -> u8"),
  (`instDecidableEqNat._boxed, "[tobj, tobj] -> tagged"),
  (`List.modifyTR._redArg, "[tobj, tobj, tobj] -> tobj"),
  (`List.MergeSort.Internal.mergeSortTR₂._redArg, "[tobj, obj] -> tobj"),
  (``String.intercalate, "[@&obj, tobj] -> obj"),
  (``Int.repr, "[@&tobj] -> obj")]
unsafe def main : IO Unit := do
  initSearchPath (← findSysroot)
  enableInitializersExecution
  let env ← importModules #[{ module := `Init }] {} (loadExts := true)
  for (n, want) in expected do
    let some d := findEnvDecl env n | throw (IO.userError s!"No upstream IR for {n}")
    let got := s!"{(d.params.map (fun p => (if p.borrow then "@&" else "") ++ toString p.ty)).toList} -> {d.resultType}"
    unless got == want do throw (IO.userError s!"{n}: expected {want}; got {got}")
  IO.println s!"PASS {expected.length} RPG runtime ownership/ABI contracts"
