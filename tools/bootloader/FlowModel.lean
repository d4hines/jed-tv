import Flow.Proofs

/- Run the production interpreter with a failure at every individual effect.
   The independent program specification determines the allowed trace prefix. -/
open LeanBoot LeanBoot.Flow

def observe (failure : Option Stage) (stage : Stage) : StateM (List Stage) Status := do
  modify (· ++ [stage])
  return if failure == some stage then ⟨77⟩ else .ok

def main : IO Unit := do
  let mut failures := 0
  for mode in [Mode.authenticate, .execute, .install, .journal] do
    let expected := program mode
    let success := ((run mode (observe none)).run []).run
    unless success == (Status.ok, expected) do
      throw <| IO.userError s!"{repr mode}: successful trace {repr success}"
    for (stage, index) in expected.zipIdx do
      let result := ((run mode (observe (some stage))).run []).run
      unless result == (⟨77⟩, expected.take (index + 1)) do
        throw <| IO.userError s!"{repr mode}/{repr stage}: advanced after failure: {repr result}"
      failures := failures + 1
    for (state, fuel, status) in [(Stage.complete, (32 : UInt32), Status.ok),
        (.rejected, 32, ⟨45⟩), (initial mode, 0, ⟨45⟩)] do
      let result := ((runFrom (observe none) mode fuel state).run []).run
      unless result == (status, []) do
        throw <| IO.userError s!"{repr mode}/{repr state}: terminal/budget effects: {repr result}"
  IO.println s!"PASS: {failures} effect-failure prefixes, 4 successful flows, 12 terminal/budget cases"
