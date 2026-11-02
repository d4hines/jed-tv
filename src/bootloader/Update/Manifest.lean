import Memory
import Flow
import Update.Policy
import Update.Access

namespace LeanBoot.Update
open Memory
variable {m : Type → Type} [Monad m] [Reader m]

@[specialize] private def reservedZero (base : UInt32) : m Bool := do
  let a ← readLE32 base headerSize 72
  let b ← readLE32 base headerSize 76
  let c ← readLE32 base headerSize 80
  let d ← readLE32 base headerSize 84
  let e ← readLE32 base headerSize 88
  let f ← readLE32 base headerSize 92
  return a ||| b ||| c ||| d ||| e ||| f == 0

/-- Fixed canonical encoding: no variable-length allocations or executable
    install scripts. Unknown versions, codecs, algorithms and reserved bits
    reject before invoking the callback. -/
@[inline] def withManifest {α : Type} (base size : UInt32)
    (reject : m α) (accept : Manifest → m α) : m α := do
  if size < headerSize || !addressable base size then return ← reject
  if (← readLE32 base size 0) != 0x4f4f424c then return ← reject
  if (← readLE32 base size 4) != 0x31505554 then return ← reject
  if (← readLE32 base size 8) != 1 then return ← reject
  if (← readLE32 base size 32) != 0 then return ← reject
  if (← readLE32 base size 36) != 1 then return ← reject
  if !(← reservedZero base) then return ← reject
  let board := (← readLE32 base size 12).toUInt32
  let component := (← readLE32 base size 16).toUInt32
  let version := (← readLE32 base size 20).toUInt32
  let length := (← readLE32 base size 24).toUInt32
  let entry := (← readLE32 base size 28).toUInt32
  accept { board, component, version, length, entry }

/-- Parsing has already validated the canonical header. These are the three
    independent observations requested by the authentication machine. -/
@[inline] def authenticationStep [Crypto m] (policy : Policy)
    (base size : UInt32) (manifest : Manifest) (stage : Flow.Stage) : m Bool :=
  match stage with
  | .policy => pure (manifest.length == size - headerSize && compatible policy manifest)
  | .signature => Crypto.signature ⟨base⟩
  | .sourceDigest => Crypto.digest ⟨base + headerSize⟩ ⟨manifest.length⟩ ⟨base + 40⟩
  | _ => pure false

/-- Verify exact length as well as the signature and image. Trailing bytes
    cannot form a second unsigned payload. The input must remain stable. -/
@[inline] def withAuthenticated {α : Type} [Crypto m]
    (policy : Policy) (base size : UInt32) (reject : m α)
    (accept : Manifest → m α) : m α :=
  withManifest base size reject fun manifest => do
    let result ← Flow.run .authenticate fun stage => do
      return if (← authenticationStep policy base size manifest stage) then .ok else ⟨2⟩
    if result.succeeded then accept manifest else reject

end LeanBoot.Update
