import Memory

namespace LeanBoot.Bootconfig

variable {m : Type → Type} [Monad m] [Memory.Reader m] [Memory.Writer m]
open Memory

@[specialize] private partial def checksum (b n i sum : UInt32) : m UInt32 := do
  if i == n then return sum
  checksum b n (i + 1) (sum + (← read8 b n i))

/-- Linux bootconfig trailer. Return final initrd size, or zero on error.
    Source is a separately owned UTF-8 config ending in newline. -/
@[specialize]
def append (b capacity used config size : UInt32) : m UInt32 := do
  if !addressable b capacity || !addressable config size || size == 0 || size > 32768 then return 0
  let final64 := ((used.toUInt64 + size.toUInt64 + 4) &&& 0xfffffffffffffffc) + 20
  if final64 > capacity.toUInt64 then return 0
  let final := final64.toUInt32
  let padded := final - 20 - used
  if b < config + size && config < b + capacity then return 0
  if (← read8 config size (size - 1)) != 10 then return 0
  let sum ← checksum config size 0 0
  if (← copy b capacity used config size 0 size) != 0 then return 0
  let _ ← fill b capacity (used + size) (padded - size) 0
  let tail := used + padded
  let _ ← writeLE32 b capacity tail padded
  let _ ← writeLE32 b capacity (tail + 4) sum
  -- "#BOOTCONFIG\n", represented as three LE words.
  let _ ← writeLE32 b capacity (tail + 8) 0x4f4f4223
  let _ ← writeLE32 b capacity (tail + 12) 0x4e4f4354
  let _ ← writeLE32 b capacity (tail + 16) 0x0a474946
  return final

end LeanBoot.Bootconfig
