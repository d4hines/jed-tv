import Unsafe.UpdateCrypto
import Update.Install

/- Volatile, fixed-address storage for the G13 RAM trial. No block device,
   controller register or persistent storage operation is available here. -/
namespace LeanBoot.Unsafe.UpdateRam

abbrev capacity : UInt32 := 32768
abbrev bankSize : UInt32 := capacity + 512
abbrev scratch : UInt32 := 0x26240000
abbrev buffer : UInt32 := 0x26230000
abbrev state : UInt32 := 0x26200200
abbrev statistics : UInt32 := 0x26200140

@[inline] def policy : Update.Policy := {
  board := 0x00130001, component := 1, minimumVersion := 1, capacity }

@[inline] def bankAddress (bank : UInt32) : UInt32 := 0x26210000 + bank * 65536

private def transfer (writing : Bool) (bank sector pointer : UInt32) : MemoryM Bool := do
  if bank >= 2 || sector >= bankSize / 512 || pointer < scratch ||
      pointer > scratch + bankSize - 512 then return false
  let address := bankAddress bank + sector * 512
  let result ← if writing then Memory.copy address 512 0 pointer 512 0 512
    else Memory.copy pointer 512 0 address 512 0 512
  if writing && result == 0 then
    let count ← load32 statistics
    let _ ← store32 statistics (count + 1)
  return result == 0

@[always_inline] instance : Update.Storage HardwareM where
  read := fun b s p => liftMemory (transfer false b s p)
  write := fun b s p => liftMemory (transfer true b s p)
  flush := pure true -- RAM visibility only; never a persistence claim.

/-- Initial signed test fixture is supplied by the host before execution.
    This provisioning entry is not reachable from network requests. -/
@[export leanboot_ram_initialize]
def seedBanks : HardwareM UInt32 := do
  if (← liftMemory <| load32 0x26200000) != 0x55504431 then return 40
  let size ← liftMemory <| load32 0x26200004
  if size < 160 || size > 4096 then return 40
  let _ ← liftMemory <| Memory.fill state 40 0 40 0
  let _ ← liftMemory <| Memory.fill statistics 32 0 32 0
  let _ ← liftMemory <| Memory.fill (bankAddress 0) bankSize 0 bankSize 0
  let _ ← liftMemory <| Memory.fill (bankAddress 1) bankSize 0 bankSize 0
  Update.withAuthenticated policy 0x26201000 size (pure 41) fun manifest => do
    if manifest.version != 1 then return 41
    let _ ← liftMemory <| Memory.copy (bankAddress 0) bankSize 0 0x26201000 size 0 160
    let _ ← liftMemory <| Memory.copy (bankAddress 0) bankSize 512 0x26201000 size 160 manifest.length
    return 0

@[export leanboot_ram_version]
def version (bank : UInt32) : HardwareM UInt32 :=
  Update.withBank policy bank scratch bankSize (pure 0) (fun v _ _ => pure v)

end LeanBoot.Unsafe.UpdateRam
