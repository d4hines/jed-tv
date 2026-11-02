import Probe
import Unsafe.Primitives

namespace LeanBoot

/-- All polling, timeout, byte selection and return decisions execute as
    ordinary compiled Lean. Tail recursion lowers to a bounded-stack C loop. -/
private partial def runProbe (state : UInt32) : Unsafe.HardwareM UInt32 := do
  let raw ← Unsafe.uartStatus
  let decision := step state ((raw >>> 5) &&& 3)
  let next := (decision >>> 32).toUInt32
  let action := decision.toUInt32
  if action == 0 then runProbe next
  else if action &&& 0x100 != 0 then
    let _ ← Unsafe.uartWrite action.toUInt8
    runProbe next
  else return action - 0x200

@[export leanboot_main]
def main : Unsafe.HardwareM UInt32 := runProbe 0

end LeanBoot
