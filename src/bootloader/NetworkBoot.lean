import Unsafe.Network

namespace LeanBoot.NetworkBoot

/-- Caller owns the reserved RAM and has admitted the G13 CPU state. The poll
    budget bounds no-upload work; it is not a calibrated wall-clock timer. -/
@[export leanboot_network_window]
def window (tags fuel : UInt32) : Unsafe.HardwareM UInt32 :=
  fun permission => do
    let result ← run Unsafe.Network.effects ⟨⟨tags⟩⟩ ⟨fuel⟩ permission
    return result.code

end LeanBoot.NetworkBoot
