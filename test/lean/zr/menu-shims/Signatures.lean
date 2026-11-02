import Lean
open Lean IR
def expected : List (Name × String) := [
  (`List.appendTR._redArg, "[tobj, tobj] -> tobj"),
  (`List.reverse._redArg, "[tobj] -> tobj"),
  (`List.range, "[tobj] -> tobj"),
  (`List.replicateTR._redArg, "[tobj, tobj] -> tobj"),
  (`List.zipIdxTR._redArg, "[tobj, @&tobj] -> tobj"),
  (`String.Slice.Pos.nextn, "[@&obj, tobj, tobj] -> tobj"),
  (`String.Slice.toString, "[@&obj] -> obj"),
  (`ByteArray.isEmpty, "[@&obj] -> u8")]

unsafe def main : IO Unit := do
  initSearchPath (← findSysroot)
  enableInitializersExecution
  let env ← importModules #[{ module := `Init }] {} (loadExts := true)
  for (n, want) in expected do
    match findEnvDecl env n with
    | some d =>
      let got := s!"{(d.params.map (fun p => (if p.borrow then "@&" else "") ++ toString p.ty)).toList} -> {d.resultType}"
      unless got == want do throw (IO.userError s!"{n}: expected {want}, got {got}")
    | none => throw (IO.userError s!"Missing upstream IR: {n}")
  IO.println "PASS: eight upstream ownership signatures"
