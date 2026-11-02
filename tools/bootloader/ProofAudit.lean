import Lean
import Proofs
import Update.Proofs
import NetworkBoot.Proofs
import Flow.Proofs
import Update.Receiver.Proofs
import Update.ParserProofs
import Update.FlashProofs
import Update.StorageModel

/- Audit every theorem in the proof module, including generated helper lemmas.
   Reject sorry, native evaluation assumptions, and other added axioms. -/
open Lean in
run_cmd do
  let env ← getEnv
  let modules := #[`Proofs, `Update.Proofs, `NetworkBoot.Proofs,
    `Flow, `Flow.Proofs, `Update.Receiver.Plan, `Update.Receiver.Proofs, `Update.ParserProofs,
    `Update.FlashProofs, `Update.StorageModel]
  let mut checkedModules := #[]
  for name in modules do
    let some index := env.getModuleIdx? name
      | throwError "{name} module not loaded"
    checkedModules := checkedModules.push index
  let allowed := #[`propext, `Classical.choice, `Quot.sound]
  let mut count : Nat := 0
  for (name, declaration) in env.constants.toList do
    if (env.getModuleIdxFor? name).any checkedModules.contains then
      if let .thmInfo _ := declaration then
        for assumption in ← collectAxioms name do
          unless allowed.contains assumption do
            throwError "{name} depends on disallowed axiom {assumption}"
        count := count + 1
  if count == 0 then
    throwError "No bootloader theorems found"
  logInfo m!"Checked {count} theorem declarations: only standard Lean logical axioms."
