import BootGate

namespace LeanBoot.GateEntry
@[export leanboot_gate_main]
def main (zero machine tags : UInt32) : Unsafe.HardwareM UInt32 :=
  BootGate.boot (fun _ => pure 0) zero machine tags
end LeanBoot.GateEntry
