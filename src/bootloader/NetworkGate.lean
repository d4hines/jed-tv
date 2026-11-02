import BootGate
import NetworkBoot
import Update.Handoff

namespace LeanBoot.NetworkGate
@[export leanboot_gate_main]
def main (zero machine tags : UInt32) : Unsafe.HardwareM UInt32 :=
  -- About three seconds without traffic on the measured G13. This is a
  -- finite polling budget, not a hardware-clock deadline.
  BootGate.boot (fun tags => do
    let _ ← NetworkBoot.window tags 4000000
    Unsafe.Flash.choice) zero machine tags Update.Handoff.publish
end LeanBoot.NetworkGate
