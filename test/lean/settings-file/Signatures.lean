import Lean
open Lean IR

unsafe def main : IO Unit := do
  initSearchPath (← findSysroot)
  enableInitializersExecution
  let env ← importModules #[{ module := `Init }] {} (loadExts := true)
  for name in [``String.toList, ``String.ofList] do
    match findEnvDecl env name with
    | some d =>
      let got := s!"{(d.params.map (fun p => (if p.borrow then "@&" else "") ++ toString p.ty)).toList} -> {d.resultType}"
      unless got == "[obj] -> tobj" && name == ``String.toList || got == "[tobj] -> obj" && name == ``String.ofList do
        throw (IO.userError s!"{name}: unexpected owned String/List signature {got}")
      IO.println s!"PASS: {name}: {got}"
    | none => throw (IO.userError s!"Missing upstream IR: {name}")
