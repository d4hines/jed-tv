import Fdt

namespace LeanBoot.Update.Handoff
open Memory
variable {m : Type → Type} [Monad m] [Reader m] [Writer m]

/-- Publish only the locally authenticated selection. The OS does not choose
    a bank by trusting unsigned version fields. Recipe lives in the reserved
    control workspace; the image/readback buffers may already be reused. -/
@[inline] def publish (choice fdt : UInt32) : m UInt32 := do
  if choice > 2 then return 1
  let recipe : UInt32 := 0x26350a00
  let _ ← fill recipe 64 0 64 0
  let _ ← writeBE32 recipe 64 0 1
  let _ ← writeBE32 recipe 64 4 7  -- /chosen
  let _ ← writeBE32 recipe 64 8 15 -- dande,apps-bank
  let _ ← writeBE32 recipe 64 12 2 -- ASCII 0, 1 or -; then NUL
  let _ ← writeLE32 recipe 64 16 0x6f68632f
  let _ ← writeLE32 recipe 64 20 0x646e6573
  let _ ← writeLE32 recipe 64 24 0x65646e61
  let _ ← writeLE32 recipe 64 28 0x7070612c
  let _ ← writeLE32 recipe 64 32 0x61622d73
  let _ ← writeLE32 recipe 64 36 0x00006b6e
  let _ ← write8 recipe 64 38 (if choice == 0 then 45 else choice + 47)
  Fdt.patch fdt 65536 recipe 40

end LeanBoot.Update.Handoff
