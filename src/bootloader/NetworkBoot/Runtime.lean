import NetworkBoot.Plan
import Update.Manifest

namespace LeanBoot.NetworkBoot

/-- Like TV.Runtime.Effects: the controller owns ordering and policy; the
    adapter owns resources. A successful stop means DMA is quiescent. Copy
    must preserve the sealed source, and prepare must preserve both images.
    Only invoke may execute payload code. -/
structure Effects (m : Type → Type) where
  busy : m Bool
  reset : m Status
  start : m Status
  receive : PollBudget → m Status
  stop : m Status
  accepted : m ByteCount
  copy : ByteCount → ByteCount → m Status
  prepare : Executable → BootTags → m Status
  invoke : Executable → m Status

variable {m : Type → Type} [Monad m] [Memory.Reader m] [Update.Crypto m]

/-- Canonical parsing occurs before the machine starts. Its interpreter can
    only answer the current request; only the invoke stage executes code. -/
@[inline] def execute (effects : Effects m) (size : ByteCount) (tags : BootTags) : m Status :=
  Update.withManifest buffer.value size.value (pure ⟨43⟩) fun manifest =>
    Flow.run .execute fun stage => do
      match stage with
      | .policy | .signature | .sourceDigest =>
        return if (← Update.authenticationStep policy buffer.value size.value manifest stage)
          then .ok else ⟨43⟩
      | .entry => return if entryFits ⟨manifest.length⟩ ⟨manifest.entry⟩ then .ok else ⟨42⟩
      | .copy => return if (← effects.copy size ⟨manifest.length⟩).succeeded then .ok else ⟨44⟩
      | .copiedDigest =>
        return if (← Update.Crypto.digest image ⟨manifest.length⟩ expectedDigest) then .ok else ⟨43⟩
      | .prepare => withExecutable manifest (pure ⟨42⟩) fun executable => effects.prepare executable tags
      | .invoke => withExecutable manifest (pure ⟨42⟩) effects.invoke
      | _ => pure ⟨45⟩

/-- Once acquisition starts, every path attempts to stop the controller,
    including failed PHY/configuration startup. A busy controller is untouched.
    Failure to stop forbids consuming or executing the received image. -/
@[inline] def run (effects : Effects m) (tags : BootTags) (fuel : PollBudget) : m Status := do
  if (← effects.busy) then return ⟨29⟩
  let reset ← effects.reset
  if !reset.succeeded then return reset
  let started ← effects.start
  let result ← if started.succeeded then effects.receive fuel else pure started
  if !(← effects.stop).succeeded then return ⟨20⟩
  if !result.succeeded then return result
  let size ← effects.accepted
  if size.value == 0 then return .ok
  execute effects size tags

end LeanBoot.NetworkBoot
