import Unsafe.Bootstrap
import Unsafe.UpdateCrypto
import Update.Flash

namespace LeanBoot.Unsafe.Flash

private def transfer (writing : Bool) (bank sector pointer : UInt32) : HardwareM Bool := do
  if !Update.Flash.sectorAllowed bank sector || !Update.Flash.pointerAllowed pointer then return false
  -- The deployed entry runs after MBOOT's kernel handoff with MMU/data cache
  -- off. Do not accept aliases or run eMMC writes from a cached RAM probe.
  if (← readSctlr) &&& 5 != 0 then return false
  if !(← Bootstrap.guard) then return false
  if writing then
    if (← liftMemory <| Memory.copy 0x26350800 512 0 pointer 512 0 512) != 0 then return false
  if (← Bootstrap.flashSector writing bank sector) != 0 then return false
  if !writing then
    if (← liftMemory <| Memory.copy pointer 512 0 0x26350800 512 0 512) != 0 then return false
  return true

@[always_inline] instance : Update.Storage HardwareM where
  read := transfer false
  write := transfer true
  flush := Bootstrap.flashFlush

-- Scalar entry points also used by the register-level offline harness. The
-- production link discards unused exports; no model substitutes the writer.
@[export leanboot_flash_install]
def install (base size : UInt32) : HardwareM UInt32 := Update.Flash.installImage base size

@[export leanboot_flash_version]
def version (bank : UInt32) : HardwareM UInt32 := fun permission => do
  if bank == 0 then Update.Flash.version (m := HardwareM) 0 permission
  else if bank == 1 then Update.Flash.version (m := HardwareM) 1 permission
  else pure 0

@[noinline, export leanboot_flash_choice]
def choice : HardwareM UInt32 := Update.Flash.choice

@[export leanboot_flash_transfer]
def transferSector (writing bank sector pointer : UInt32) : HardwareM UInt32 := do
  return if (← transfer (writing != 0) bank sector pointer) then 0 else 1

end LeanBoot.Unsafe.Flash
