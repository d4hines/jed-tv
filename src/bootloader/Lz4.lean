import Memory

/- Raw LZ4 block decoder. All lengths and offsets are checked before access;
   back references copy forward so overlap has the format's copyMatch semantics. -/
namespace LeanBoot.Lz4
open Memory
variable {m : Type → Type} [Monad m] [Reader m] [Writer m]

@[specialize] private partial def length (src size pos value : UInt32) : m UInt64 := do
  if pos >= size then return error 1
  let byte ← read8 src size pos
  if value > 0x7fffffff - byte then return error 1
  let value := value + byte
  if byte == 255 then length src size (pos + 1) value
  else return ((pos + 1).toUInt64 <<< 32) ||| value.toUInt64

@[specialize] private partial def copyMatch (dst distance count : UInt32) : m Unit := do
  if count != 0 then
    if distance >= 4 && (dst ||| distance) &&& 3 == 0 && count >= 4 then
      let value ← Reader.word (dst - distance)
      let _ ← Writer.word dst value
      copyMatch (dst + 4) distance (count - 4)
    else
      let value ← Reader.byte (dst - distance)
      let _ ← Writer.byte dst value
      copyMatch (dst + 1) distance (count - 1)

@[specialize] private partial def loop (src size dst capacity pos used : UInt32) : m UInt32 := do
  if pos >= size then return 1
  let token ← read8 src size pos
  let pos := pos + 1
  let literals ← if token >>> 4 == 15 then length src size pos 15
    else pure ((pos.toUInt64 <<< 32) ||| (token >>> 4).toUInt64)
  if failed literals then return 1
  let pos := (literals >>> 32).toUInt32
  let count := literals.toUInt32
  if !contains size pos count || !contains capacity used count then return 1
  if (← copy dst capacity used src size pos count) != 0 then return 1
  let pos := pos + count
  let used := used + count
  if pos == size then return if used == capacity then 0 else 1
  if !contains size pos 2 then return 1
  let distance := (← read8 src size pos) ||| ((← read8 src size (pos + 1)) <<< 8)
  if distance == 0 || distance > used then return 1
  let pos := pos + 2
  let backref ← if token &&& 15 == 15 then length src size pos 19
    else pure ((pos.toUInt64 <<< 32) ||| ((token &&& 15) + 4).toUInt64)
  if failed backref then return 1
  let pos := (backref >>> 32).toUInt32
  let count := backref.toUInt32
  if !contains capacity used count then return 1
  copyMatch (dst + used) distance count
  loop src size dst capacity pos (used + count)

@[specialize] def decode (src size dst expected : UInt32) : m UInt32 := do
  if size == 0 || !addressable src size || !addressable dst expected ||
      (src < dst + expected && dst < src + size) then return 1
  loop src size dst expected 0 0

end LeanBoot.Lz4
