import Unsafe.Emac
import Unsafe.UpdateCrypto
import Unsafe.Flash
import NetworkBoot.Receiver
import NetworkBoot.Runtime

/- Physical adapter for the network controller. Owns the fixed RAM layout,
   MAC/DMA lifecycle, cache maintenance and the only payload-call boundary.
   See CONTRACTS.txt for assumptions that the arithmetic model cannot prove. -/
namespace LeanBoot.Unsafe.Network
open LeanBoot.NetworkBoot

private partial def emit (bytes : UInt64) (fuel : UInt32 := 1000000) : HardwareM Unit := do
  if bytes == 0 || fuel == 0 then return
  if (← uartStatus) &&& 32 == 0 then emit bytes (fuel - 1)
  else
    let _ ← uartWrite bytes.toUInt8
    emit (bytes >>> 8)

@[inline] private def prepare (executable : Executable) (tags : BootTags) : HardwareM Status := do
  let _ ← liftMemory <| store32 context.value 0x31424e4c -- LNB1 context
  let _ ← liftMemory <| store32 0x40301004 1
  let _ ← liftMemory <| store32 0x40301008 tags.address.value
  let _ ← liftMemory <| store32 0x4030100c 0x23000000
  let _ ← liftMemory <| store32 0x40301010 0x20208000
  let _ ← liftMemory <| store32 0x40301014 executable.version.value
  let _ ← liftMemory <| store32 0x40301018 executable.length.value
  let sctlr ← readSctlr
  let _ ← liftMemory <| store32 0x4030101c sctlr
  let _ ← Cpu.cleanCaches
  if (← Cpu.flushMiu) != 0 then return ⟨20⟩
  let _ ← emit 0x544f4f424e41454c
  let _ ← emit 0x0a0d54454e20 -- NET
  return .ok

@[extern "leanboot_call_network"]
private opaque rawCall (entry context : UInt32) : BaseIO UInt32

/-- The plan proves the first ARM instruction is within the image. The
    controller additionally authenticates, copies, hashes and stops DMA. -/
@[inline] private def invoke (executable : Executable) : HardwareM Status := fun _ => do
  return ⟨← rawCall executable.address.value context.value⟩

private def start : HardwareM UInt32 := do
  let _ ← Emac.powerOn
  let phy ← Emac.phyRead 0 1
  if phy == 0 || phy >= 65535 then return 30
  let _ ← Cpu.cleanCaches
  Emac.configure 0

@[inline] def effects : Effects HardwareM where
  busy := do return (← Emac.readReg 0) &&& 12 != 0
  reset := do return ⟨← liftMemory <| Memory.fill state.value 128 0 128 0⟩
  start := do return ⟨← start⟩
  receive := fun fuel => do
    let _ ← emit 0x544f4f424e41454c
    let _ ← emit 0x0a0d5449415720 -- WAIT
    let respond := fun rx length => do return (← Receiver.respond ⟨rx⟩ ⟨length⟩).value
    return ⟨← Emac.receiveUntil respond Receiver.finished fuel.value⟩
  stop := do return ⟨← Emac.stop⟩
  accepted := do return ⟨← liftMemory <| load32 (state.value + 64)⟩
  copy := fun size length => do
    return ⟨← liftMemory <| Memory.copy image.value capacity.value 0 buffer.value size.value 160 length.value⟩
  prepare := prepare
  invoke := invoke

end LeanBoot.Unsafe.Network
