import Lean
open Lean IR
unsafe def main : IO Unit := do
  initSearchPath (← findSysroot)
  enableInitializersExecution
  let env ← importModules #[{module := `Init}] {} (loadExts := true)
  for (name, expected) in [
    (``String.intercalate, "[@&obj, tobj] -> obj"),
    (``Int.repr, "[@&tobj] -> obj"),
    (``String.Pos.Raw.get', "[@&obj, @&tobj, ◾] -> u32"),
    (``String.Pos.Raw.next', "[@&obj, @&tobj, ◾] -> tagged")] do
    match findEnvDecl env name with
    | some d =>
      let signature := s!"{(d.params.map (fun p => (if p.borrow then "@&" else "") ++ toString p.ty)).toList} -> {d.resultType}"
      unless signature == expected do throw (IO.userError s!"{name}: {signature}, expected {expected}")
      IO.println s!"PASS {name}: {signature}"
    | none => throw (IO.userError s!"No upstream IR for {name}")
