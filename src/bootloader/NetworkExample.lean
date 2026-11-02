import Unsafe.Ram

/- Minimal returnable Lean -> C development payload for the network ABI. -/
namespace LeanBoot.NetworkExample
open Unsafe

private partial def emit (bytes : UInt64) (fuel : UInt32 := 1000000) : HardwareM UInt32 := do
  if bytes == 0 then return 0
  if fuel == 0 then return 21
  if (← uartStatus) &&& 32 == 0 then emit bytes (fuel - 1)
  else
    let _ ← uartWrite bytes.toUInt8
    emit (bytes >>> 8)

@[export leanboot_network_example]
def main (context : UInt32) : HardwareM UInt32 := do
  if context != 0x40301000 then return 1
  if (← liftMemory <| load32 context) != 0x31424e4c then return 2
  let _ ← emit 0x204b524f5754454e -- NETWORK
  let _ ← emit 0x0a0d4e41454c -- LEAN
  let _ ← liftMemory <| store32 (context + 32) 0x1ea4
  return 0

end LeanBoot.NetworkExample
