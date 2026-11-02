import Unsafe.Primitives

namespace LeanBoot.Cpu

private partial def leadingZeros (value count : UInt32) : UInt32 :=
  if count == 32 || value &&& 0x80000000 != 0 then count
  else leadingZeros (value <<< 1) (count + 1)

private partial def sets (level way shift count : UInt32) : Unsafe.HardwareM UInt32 := do
  let _ ← Unsafe.cleanSetWay (level ||| way ||| (count <<< shift))
  if count == 0 then return 0
  sets level way shift (count - 1)

private partial def ways (level wayShift lineShift maxSet count : UInt32) : Unsafe.HardwareM UInt32 := do
  let _ ← sets level (if count == 0 then 0 else count <<< wayShift) lineShift maxSet
  if count == 0 then return 0
  ways level wayShift lineShift maxSet (count - 1)

private partial def levels (clidr level limit : UInt32) : Unsafe.HardwareM UInt32 := do
  if level == limit then return 0
  let kind := (clidr >>> (level * 3)) &&& 7
  if kind >= 2 && kind <= 4 then
    let _ ← Unsafe.selectCache (level * 2)
    let size ← Unsafe.readCcsidr
    let maxWay := (size >>> 3) &&& 0x3ff
    let _ ← ways (level * 2) (leadingZeros maxWay 0) ((size &&& 7) + 4)
      ((size >>> 13) &&& 0x7fff) maxWay
    levels clidr (level + 1) limit
  else levels clidr (level + 1) limit

@[export leanboot_clean_caches]
def cleanCaches : Unsafe.HardwareM UInt32 := do
  let _ ← Unsafe.barrier
  let clidr ← Unsafe.readClidr
  let _ ← levels clidr 0 ((clidr >>> 24) &&& 7)
  let _ ← Unsafe.selectCache 0
  Unsafe.barrier

private partial def pipeWait (fuel : UInt32) : Unsafe.HardwareM UInt32 := do
  if fuel == 0 then return 1
  if (← Unsafe.load16 0x1f203140) &&& 0x1000 != 0 then return 0
  pipeWait (fuel - 1)

@[export leanboot_flush_miu]
def flushMiu : Unsafe.HardwareM UInt32 := do
  let value ← Unsafe.load16 0x1f203114
  let _ ← Unsafe.store16 0x1f203114 (value &&& 0xfffe)
  let value ← Unsafe.load16 0x1f203114
  let _ ← Unsafe.store16 0x1f203114 (value ||| 1)
  let result ← pipeWait 1048576
  let value ← Unsafe.load16 0x1f203114
  let _ ← Unsafe.store16 0x1f203114 (value &&& 0xfffe)
  let _ ← Unsafe.barrier
  return result

private partial def halt : Unsafe.HardwareM UInt32 := do
  let _ ← Unsafe.barrier
  halt

/-- Runs on a fresh stack with MMU and caches off. Re-cleaning removes dirty
    lines created by the first Lean cleanup's own stack accesses. The new stack
    was cleaned by that first pass and was never subsequently touched. -/
@[export leanboot_after_cache_off]
def afterCacheOff (kernel fdt : UInt32) : Unsafe.HardwareM UInt32 := do
  let _ ← cleanCaches
  if (← flushMiu) != 0 then halt else Unsafe.jumpLinux kernel fdt

/-- Terminal path. Preconditions: SVC mode, one active CPU, identity mappings,
    DMA idle, 0x26300000..0x26310000 is private RAM, verified images prepared. -/
@[export leanboot_enter_linux]
def enterLinux (kernel fdt : UInt32) : Unsafe.HardwareM UInt32 := do
  let sctlr ← Unsafe.readSctlr
  let _ ← cleanCaches
  if (← flushMiu) != 0 then return 20
  -- Match exact MBOOT's interrupt-controller mask at 0x27615988..0x276159a4.
  let _ ← Unsafe.writeMmio32 0x1f203290 0xffff
  let _ ← Unsafe.writeMmio32 0x1f203294 0xffff
  let _ ← Unsafe.writeMmio32 0x1f2032d0 0xffff
  let _ ← Unsafe.writeMmio32 0x1f2032d4 0xffff
  let _ ← Unsafe.writeMmio32 0x1f203298 0xffff
  let _ ← Unsafe.writeMmio32 0x1f20329c 0xffff
  let _ ← Unsafe.writeMmio32 0x1f2032d8 0xffff
  let _ ← Unsafe.writeMmio32 0x1f2032dc 0xffff
  Unsafe.beginTransition (sctlr &&& 0xffffeffa) kernel fdt

end LeanBoot.Cpu
