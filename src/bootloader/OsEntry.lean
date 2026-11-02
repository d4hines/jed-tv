import Unsafe.Ram
import Os
import Cpu
import VerifiedBoot
import EntryCheck

namespace LeanBoot.OsEntry

private partial def ready (mask fuel : UInt32) : Unsafe.HardwareM UInt32 := do
  if fuel == 0 then return 21
  if (← Unsafe.uartStatus) &&& mask == mask then return 0
  ready mask (fuel - 1)

private partial def emit (bytes : UInt64) : Unsafe.HardwareM UInt32 := do
  if bytes == 0 then return 0
  if (← ready 0x20 1048576) != 0 then return 21
  let _ ← Unsafe.uartWrite bytes.toUInt8
  emit (bytes >>> 8)

private partial def emitHex (value digits : UInt32) : Unsafe.HardwareM UInt32 := do
  if digits == 0 then return 0
  if (← ready 0x20 1048576) != 0 then return 21
  let nibble := (value >>> ((digits - 1) * 4)) &&& 15
  let _ ← Unsafe.uartWrite (nibble + (if nibble < 10 then 48 else 87)).toUInt8
  emitHex value (digits - 1)

@[export leanboot_os_main]
def main : Unsafe.HardwareM UInt32 := do
  -- Check the execution mode before any privileged register or MMIO access.
  let cpsr ← Unsafe.readCpsr
  if !EntryCheck.mode cpsr then return 26
  let sp ← Unsafe.readSp
  let sctlr ← Unsafe.readSctlr
  if !EntryCheck.g13 cpsr sp sctlr then return 26
  let _ ← emit 0x544f4f424e41454c -- LEANBOOT
  let _ ← emit 0x7065727020534f20 -- OS prep
  let _ ← emit 0x0a0d
  let mode ← Memory.readLE32 0x26200000 65536 8
  if mode > 1 then return 22
  Os.withPrepared false fun config => do
    let _ ← emit 0x544f4f424e41454c
    let _ ← emit 0x6572617065727020 -- prepare
    let _ ← emit 0x0a0d64 -- d\r\n
    if mode == 0 then return 0
    let _ ← Cpu.cleanCaches
    if (← Cpu.flushMiu) != 0 then return 20
    let status ← Unsafe.sendVerifiedBoot config.secureMessage
    let _ ← emit 0x544f4f424e41454c
    let _ ← emit 0x2065727563657320 -- secure
    let _ ← emitHex status 8
    let _ ← emit 0x0a0d
    if status != 0 then return 25
    let _ ← emit 0x544f4f424e41454c
    let _ ← emit 0x0a0d78756e694c20 -- Linux
    if (← ready 0x60 1048576) != 0 then return 21
    Cpu.enterLinux config.entry config.fdt

end LeanBoot.OsEntry
