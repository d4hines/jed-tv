import EntryCheck
import NetworkBoot

namespace LeanBoot.NetworkBootEntry
open Unsafe

@[export leanboot_ethernet_main]
def main : HardwareM UInt32 := do
  if (← readCpsr) &&& 0x3f != 0x13 then return 26
  let saved ← maskInterrupts
  let cpsr ← readCpsr
  let sp ← readSp
  let sctlr ← readSctlr
  let result ← if !EntryCheck.g13 cpsr sp sctlr then pure 26
    else NetworkBoot.window 0 300000000
  let _ ← restoreInterrupts saved
  return result

end LeanBoot.NetworkBootEntry
