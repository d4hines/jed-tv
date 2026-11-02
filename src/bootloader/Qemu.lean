import Unsafe.Ram
import Os
import Cpu

/- The virt target uses the same image parsers, preparation, FDT editor,
   bootconfig writer and cache-maintenance loops as the G13 target.
   Only the memory map, UART and terminal platform handoff differ. -/
namespace LeanBoot.Qemu

private partial def ready (fuel : UInt32) : Unsafe.HardwareM Bool := do
  if fuel == 0 then return false
  if (← Unsafe.readMmio32 0x09000018) &&& 0x20 == 0 then return true
  ready (fuel - 1)

private partial def emit (bytes : UInt64) : Unsafe.HardwareM UInt32 := do
  if bytes == 0 then return 0
  if !(← ready 1048576) then return 1
  let _ ← Unsafe.writeMmio32 0x09000000 (bytes.toUInt32 &&& 255)
  emit (bytes >>> 8)

private partial def halt : Unsafe.HardwareM UInt32 := do
  let _ ← Unsafe.barrier
  halt

@[export leanboot_qemu_after_cache_off]
def afterCacheOff (kernel fdt : UInt32) : Unsafe.HardwareM UInt32 := do
  let _ ← Cpu.cleanCaches
  Unsafe.qemuJump kernel fdt

@[export leanboot_qemu_main]
def main : Unsafe.HardwareM UInt32 := do
  let _ ← emit 0x544f4f424e41454c
  let _ ← emit 0x0a0d554d455120 -- QEMU
  let result ← Os.withPrepared true fun config => do
    let _ ← emit 0x544f4f424e41454c
    let _ ← emit 0x0a0d78756e694c20 -- Linux
    let sctlr ← Unsafe.readSctlr
    let _ ← Cpu.cleanCaches
    Unsafe.qemuTransition (sctlr &&& 0xffffeffa) config.entry config.fdt
  let _ ← emit 0x204c494146
  let _ ← emit (48 + result.toUInt64)
  let _ ← emit 0x0a0d
  halt

end LeanBoot.Qemu
