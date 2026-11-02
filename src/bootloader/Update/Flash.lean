import Update.Install
import Update.FlashPlan

namespace LeanBoot.Update.Flash
variable {m : Type → Type} [Monad m] [Memory.Reader m] [Memory.Writer m]
  [Crypto m] [Storage m]

/-- Bit 32 reports an I/O error, separately from an invalid image. All 32-bit
    signed release numbers remain representable. A read fault aborts updates. -/
@[inline] def inspect (bank : UInt32) : m UInt64 :=
  inspectBank policy bank scratch.value scratchBytes (pure 0) (pure 0x100000000)
    (fun v _ entry => pure (if entry == 0 then v.toUInt64 else 0))

@[inline] def version (bank : UInt32) : m UInt32 := do
  let result ← inspect bank
  return if result >>> 32 != 0 then 0 else result.toUInt32

/-- Zero means no authenticated choice; 1 and 2 identify the two banks. -/
@[inline] def choice : m UInt32 := do
  let a ← inspect 0
  let b ← inspect 1
  if a >>> 32 != 0 || b >>> 32 != 0 then return 0
  let selected := selectBank a.toUInt32 b.toUInt32 (a != 0) (b != 0)
  return if selected < 2 then selected + 1 else 0

/-- First provisioning also requires a signed image. With no authenticated
    bank there is no active image to preserve; bank 0 is the fixed target.
    Equal authenticated versions remain ambiguous and reject. -/
@[inline] private def installCanonical (base size : UInt32) : m UInt32 := do
  let va ← inspect 0
  let vb ← inspect 1
  if va >>> 32 != 0 || vb >>> 32 != 0 then return 8
  let a := va.toUInt32
  let b := vb.toUInt32
  let active := selectBank a b (a != 0) (b != 0)
  if active < 2 then
    install policy active (if active == 0 then a else b) base size scratch.value scratchBytes
  else if a == 0 && b == 0 then
    install policy 1 0 base size scratch.value scratchBytes
  else pure 7

@[inline] def installImage (base size : UInt32) : m UInt32 :=
  withManifest base size (pure 2) fun manifest => do
    if manifest.entry != 0 then return 2
    installCanonical base size

end LeanBoot.Update.Flash
