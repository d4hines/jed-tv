import Memory

/- Serialize the exact V013 unlocked/orange boot-parameter message. This is
   handoff metadata for the existing secure firmware, not image verification.
   The host still pins and verifies the input images and vbmeta digest. -/
namespace LeanBoot.VerifiedBoot

variable {m : Type → Type} [Monad m] [Memory.Reader m] [Memory.Writer m]
open Memory

@[specialize] private partial def hexDigest (b n i : UInt32) : m Bool := do
  if i == n then return true
  let c ← read8 b n i
  if !(c >= 48 && c <= 57 || c >= 97 && c <= 102) then return false
  hexDigest b n (i + 1)

@[inline] def validDigest (b n : UInt32) : m Bool := do
  if n != 64 || !addressable b n then return false
  hexDigest b n 0

@[specialize]
def packet (b capacity version digest size : UInt32) : m UInt32 := do
  if capacity < 132 || !addressable b capacity || size != 64 ||
      !addressable digest size then return 1
  if b < digest + size && digest < b + capacity then return 2
  if !(← validDigest digest size) then return 3
  let _ ← fill b capacity 0 132 0
  let _ ← writeLE32 b capacity 0 128
  let _ ← writeLE32 b capacity 4 0x6f6f626d -- mboo
  let _ ← writeLE32 b capacity 8 0x74 -- t
  let _ ← writeLE32 b capacity 12 0x69726576 -- veri
  let _ ← writeLE32 b capacity 16 0x64656966 -- fied
  let _ ← writeLE32 b capacity 20 0x6f6f622d -- -boo
  let _ ← writeLE32 b capacity 24 0x74 -- t
  let _ ← writeLE32 b capacity 40 version
  -- Empty public key, orange state, unlocked; never claim a verified lock.
  let _ ← writeLE32 b capacity 52 2
  let _ ← writeLE32 b capacity 60 size
  copy b capacity 64 digest size 0 size

end LeanBoot.VerifiedBoot
