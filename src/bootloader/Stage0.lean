import BootBundle
import Unsafe.Ram

namespace LeanBoot.Stage0
open Unsafe

private partial def halt : HardwareM UInt32 := do
  let _ ← barrier
  halt

/-- Only relocates and checks the resident gate. The signed-network verifier
    lives in that gate; this is not an independent secure-boot root. -/
@[export leanboot_bundle_copy]
def main (_zero _machine tags : UInt32) : HardwareM UInt32 := do
  if (← readCpsr) &&& 0xff != 0xd3 || (← readSctlr) &&& 0x1005 != 0 then return ← halt
  let _ ← Memory.copy BootBundle.base BootBundle.prefixSize 0
    0x20208000 BootBundle.prefixSize 0 BootBundle.prefixSize
  if !(← BootBundle.headerValid) then return ← halt
  let compressed := BootBundle.base + (← BootBundle.field 60)
  let size ← BootBundle.field 64
  let bytes ← BootBundle.field 68
  if (← BootBundle.hash compressed size) != (← BootBundle.field 72) then return ← halt
  if (← Lz4.decode compressed size 0x26000000 bytes) != 0 then return ← halt
  if (← BootBundle.hash 0x26000000 bytes) != (← BootBundle.field 28) then return ← halt
  let _ ← barrier
  qemuJump 0x26000000 tags

end LeanBoot.Stage0
