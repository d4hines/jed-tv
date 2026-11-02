import Unsafe.Network
import Update.Handoff

namespace LeanBoot.Unsafe.FlashHarness
@[export leanboot_flash_frame]
def frame (address size : UInt32) : HardwareM UInt32 := do
  return (← NetworkBoot.Receiver.respond ⟨address⟩ ⟨size⟩).value

@[export leanboot_flash_handoff]
def handoff (choice fdt : UInt32) : HardwareM UInt32 := Update.Handoff.publish choice fdt
end LeanBoot.Unsafe.FlashHarness
