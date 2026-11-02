import Unsafe.Bootstrap

namespace LeanBoot.BootstrapProbe

/-- Read-only characterization before adding persistent selection. This entry
    uses the verified MBOOT command stack, like MBOOT's own MMC command. -/
@[export leanboot_bootstrap_probe]
def probe : Unsafe.HardwareM UInt32 := do
  let cpsr ← Unsafe.readCpsr
  if cpsr &&& 0x3f != 0x13 then return 26
  if !(← Unsafe.Bootstrap.guard) then return 41
  let result ← Unsafe.Bootstrap.readSector 0x3217fe 0x26350000
  let _ ← Unsafe.liftMemory <| Unsafe.store32 0x26200100 result
  return result

end LeanBoot.BootstrapProbe
