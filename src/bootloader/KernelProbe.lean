import Unsafe.Bootstrap

namespace LeanBoot.KernelProbe

private partial def emit (bytes : UInt64) : Unsafe.HardwareM Unit := do
  if bytes != 0 then
    let _ ← Unsafe.uartWrite bytes.toUInt8
    emit (bytes >>> 8)

@[export leanboot_kernel_probe_main]
def main (_zero _machine fdt : UInt32) : Unsafe.HardwareM UInt32 := do
  let _ ← emit 0x544f4f424e41454c
  let _ ← emit 0x65626f7270204b20 -- K probe
  let cpsr ← Unsafe.readCpsr
  let sctlr ← Unsafe.readSctlr
  let _ ← Unsafe.liftMemory <| Unsafe.store32 0x26350100 cpsr
  let _ ← Unsafe.liftMemory <| Unsafe.store32 0x26350104 sctlr
  let status ← Unsafe.Bootstrap.readSector 0x3217fe 0x26350400
  let _ ← Unsafe.liftMemory <| Unsafe.store32 0x26350108 status
  let _ ← emit (if status == 0 then 0x0a0d4b4f20 else 0x0a0d44414220)
  -- Restore the eight original kernel bytes overwritten for this RAM trial.
  let _ ← Unsafe.liftMemory <| Unsafe.store32 0x20208000 0xe28f9001
  let _ ← Unsafe.liftMemory <| Unsafe.store32 0x20208004 0xe12fff19
  let _ ← Unsafe.barrier
  Unsafe.qemuJump 0x20208000 fdt

end LeanBoot.KernelProbe
