import Init

/- Nominal wrappers, not aliases or implicit coercions. These describe units
   and roles, not ownership of physical memory. Explicit conversions belong
   at wire/ABI boundaries; checked plans carry additional validity proofs. -/
namespace LeanBoot

/-- CPU-visible address, which may be a virtual alias while the MMU is on. -/
structure CpuAddress where
  value : UInt32
  deriving Repr, DecidableEq

structure ByteCount where
  value : UInt32
  deriving Repr, DecidableEq

structure EntryOffset where
  value : UInt32
  deriving Repr, DecidableEq

structure ReleaseVersion where
  value : UInt32
  deriving Repr, DecidableEq

structure PollBudget where
  value : UInt32
  deriving Repr, DecidableEq

/-- Opaque boot argument retained for the stock kernel fallback. -/
structure BootTags where
  address : CpuAddress
  deriving Repr, DecidableEq

/-- Return codes cross the scalar ABI, but are not lengths or addresses. -/
structure Status where
  code : UInt32
  deriving Repr, DecidableEq

namespace Status
def ok : Status := ⟨0⟩
@[inline] def succeeded (status : Status) : Bool := status.code == 0
end Status

end LeanBoot
