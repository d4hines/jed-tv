import BootBundle
import Unsafe.Bootstrap
import Os
import Cpu

namespace LeanBoot.BootGate
open Unsafe

private partial def ready (fuel : UInt32) : HardwareM Bool := do
  if fuel == 0 then return false
  if (← uartStatus) &&& 0x20 != 0 then return true
  ready (fuel - 1)

private partial def emit (bytes : UInt64) : HardwareM Unit := do
  if bytes != 0 && (← ready 1048576) then
    let _ ← uartWrite bytes.toUInt8
    emit (bytes >>> 8)

private partial def halt : HardwareM UInt32 := do
  let _ ← barrier
  halt

private def admitted : HardwareM Bool := do
  let cpsr ← readCpsr
  if cpsr &&& 0xff != 0xd3 then return false
  return (← readSctlr) &&& 0x1005 == 0

@[inline] private def stock (finish : UInt32 → HardwareM UInt32) (fdt : UInt32) : HardwareM UInt32 := do
  if (← finish 0x23000000) != 0 then return ← halt
  let _ ← emit 0x544f4f424e41454c
  let _ ← emit 0x206b636f747320 -- stock
  let _ ← emit 0x0a0d
  let _ ← barrier
  if (← Cpu.flushMiu) != 0 then return ← halt
  qemuJump 0x20208000 fdt

@[inline] def boot (beforePrepare : UInt32 → HardwareM UInt32)
    (_zero _machine fdt : UInt32)
    (onHandoff : UInt32 → UInt32 → HardwareM UInt32 := fun _ _ => pure 0) : HardwareM UInt32 := do
  if !(← admitted) then return ← halt
  let _ ← enableInstructionCache
  let _ ← emit 0x544f4f424e41454c
  let _ ← emit 0x4554414720 -- GATE
  let _ ← emit 0x0a0d
  -- Recovery transport must be reachable before inspecting the normal OS,
  -- including on the third-start path and when normal images are corrupt.
  -- The resident Stage0/gate itself must still have loaded successfully.
  let selection ← beforePrepare fdt
  if !(← BootBundle.headerValid) || !(← BootBundle.compressedValid) then
    let _ ← emit 0x0a0d444142
    return ← halt
  if (← Lz4.decode (BootBundle.base + (← BootBundle.field 16)) (← BootBundle.field 20)
      0x20207000 BootBundle.restoreSize) != 0 then
    let _ ← emit 0x0a0d444142 -- BAD
    return ← halt
  -- MBOOT loads only the real kernel bytes, not the final page padding.
  let _ ← Memory.fill 0x20207000 BootBundle.bootSize (4096 + BootBundle.total)
    (BootBundle.bootSize - 4096 - BootBundle.total) 0
  if !(← BootBundle.kernelValid) then return ← halt
  -- Recovery and malformed normal-boot metadata bypass the journal entirely.
  -- Exact V013 passes legacy tags in r2; the real kernel DTB is at 23000000.
  -- Preserve r2 opaquely for stock, but inspect the DTB actually used by Linux.
  let normal ← BootBundle.normalBoot 0x23000000
  let state ← if normal && (← Bootstrap.guard) then BootJournal.advance
    else pure BootPolicy.rejected
  let _ ← emit 0x544f4f424e41454c
  let _ ← emit 0x206563696f686320 -- choice
  let _ ← emit (if normal then 0x4e else 0x52)
  let _ ← emit (if BootPolicy.failed state then 0x45 else (state + 48).toUInt64)
  let _ ← emit 0x0a0d
  let payload ← if !BootPolicy.fallback state then BootBundle.payloadValid else pure false
  -- Preserve MBOOT's initrd before the larger loader is allowed to modify it.
  -- These buffers are used only after the final eMMC transaction.
  if payload then
    let _ ← Memory.copy 0x26500000 0x1400000 0 0x24000000 0x1400000 0 0x1400000
    let _ ← Memory.copy 0x26410000 65536 0 0x23000000 65536 0 65536
    BootBundle.inputs
  if !payload then return ← stock (onHandoff selection) fdt
  let _ ← emit 0x544f4f424e41454c
  let _ ← emit 0x7065727020534f20
  let _ ← emit 0x0a0d
  -- MBOOT already completed the secure-monitor handoff. Reuse the shared
  -- memory-only OS preparation, then jump without repeating those SMCs.
  let result ← Os.withPrepared false fun config => do
    if (← onHandoff selection config.fdt) != 0 then return ← halt
    let _ ← emit 0x544f4f424e41454c
    let _ ← emit 0x0a0d78756e694c20
    let _ ← barrier
    if (← Cpu.flushMiu) != 0 then return ← halt
    qemuJump config.entry config.fdt
  let _ ← emit 0x0a0d525245 -- ERR
  let _ ← Memory.copy 0x24000000 0x1400000 0 0x26500000 0x1400000 0 0x1400000
  let _ ← Memory.copy 0x23000000 65536 0 0x26410000 65536 0 65536
  let _ := result
  stock (onHandoff selection) fdt

end LeanBoot.BootGate
