import Images
import Fdt
import Bootconfig
import VerifiedBoot
import Plan

/- Deterministic preparation over caller-supplied memory. This module has no
   physical-memory instance, UART, secure call, CPU operation or kernel jump.
   Inspection needs only Reader. Applying a validated plan also needs Writer.
   The continuation is the sole exit to the caller's next stage. -/
namespace LeanBoot.Os
open Memory

variable {m : Type → Type} [Monad m] [Reader m]

@[inline] private def field (control offset : UInt32) : m UInt32 := do
  return (← readLE32 control 65536 offset).toUInt32

@[inline] private def applyPlan [Writer m] (r : Plan.Request) (p : Plan.BootPlan)
    (accept : Plan.KernelConfig → m UInt32) : m UInt32 := do
  let b := Plan.bias r.qemu
  -- Finish the bounded FDT work before modifying the source vendor ramdisk.
  if (← copy p.kernel.fdt 65536 0 p.dtbSource p.dtbSize 0 p.dtbSize) != 0 then return 9
  if (← Fdt.patch p.kernel.fdt 65536 p.recipeSource p.recipeSize) != 0 then return 10
  if (← Fdt.reserve p.kernel.fdt 65536 p.kernel.initrd p.kernel.initrdSize) != 0 then return 14
  if !r.qemu && (← VerifiedBoot.packet p.kernel.secureMessage 256 p.osVersion p.digestSource 64) != 0 then return 24
  if (← copy p.kernel.initrd 0x1400000 0 (b + 0x24000000) r.vendorSize p.vendorOffset p.vendorSize) != 0 then return 11
  if (← copy p.kernel.initrd 0x1400000 p.vendorSize (b + 0x25400000) r.initSize p.genericOffset p.genericSize) != 0 then return 12
  let final ← Bootconfig.append p.kernel.initrd 0x1400000 (p.vendorSize + p.genericSize) p.configSource p.configSize
  if final != p.kernel.initrdSize then return 13
  accept p.kernel

/-- Inspect input only, then deliver the pure plan to a continuation. A caller
    with Reader but no Writer can use this entry without permitting mutation. -/
@[specialize] def inspect (qemu : Bool) (accept : Plan.Request → Plan.BootPlan → m UInt32) : m UInt32 := do
  let b := Plan.bias qemu
  let control := b + 0x26200000
  let bootBase := b + 0x20207000
  let vendorBase := b + 0x24000000
  let initBase := b + 0x25400000
  if (← field control 0) != 0x534f424c || (← field control 4) != 1 || (← field control 12) != 64 then return 1
  let r : Plan.Request := {
    qemu := qemu
    bootSize := ← field control 16, vendorSize := ← field control 20, initSize := ← field control 24
    recipeOffset := ← field control 28, recipeSize := ← field control 32
    configOffset := ← field control 36, configSize := ← field control 40
    finalSize := ← field control 44, digestOffset := ← field control 48, digestSize := ← field control 52 }
  let bad := Plan.validRequest r
  if bad != 0 then return bad
  let kernel ← Images.bootSection bootBase r.bootSize 0 0
  if failed kernel then return 4
  if !qemu then
    if (← readLE32 (bootBase + 4096) kernel.toUInt32 0) != 0xe28f9001 ||
        (← readLE32 (bootBase + 4096) kernel.toUInt32 4) != 0xe12fff19 then return 5
  let generic ← Images.bootSection initBase r.initSize 1 1
  if failed generic then return 6
  let vendor ← Images.vendorSection vendorBase r.vendorSize 0
  let wrapped ← Images.vendorSection vendorBase r.vendorSize 1
  if failed vendor || failed wrapped then return 7
  let wrappedBase := vendorBase + (wrapped >>> 32).toUInt32
  let dtb ← Images.dtbSection wrappedBase wrapped.toUInt32
  if failed dtb then return 8
  let last ← read8 (control + r.configOffset) r.configSize (r.configSize - 1)
  let version := (← readLE32 bootBase r.bootSize 16).toUInt32
  if !qemu && !(← VerifiedBoot.validDigest (control + r.digestOffset) r.digestSize) then return 24
  Plan.withPlan r kernel generic vendor wrapped dtb last version pure (accept r)

/-- Both preparation targets use the same memory-only core. The callback gets
    a value configuration after all preparation succeeded; it is never stored. -/
@[inline] def withPrepared [Writer m] (qemu : Bool) (accept : Plan.KernelConfig → m UInt32) : m UInt32 :=
  inspect qemu fun r p => applyPlan r p accept

end LeanBoot.Os
