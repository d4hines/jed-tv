import MemoryAccess

/- Checked arithmetic and byte decoding are ordinary Lean. The caller still
   has to supply a stable RAM region it owns: arithmetic cannot establish RAM
   ownership, MMU permissions, or absence of concurrent modification. -/
namespace LeanBoot.Memory

variable {m : Type → Type} [Monad m] [Memory.Reader m] [Memory.Writer m]

@[inline] def contains (size offset count : UInt32) : Bool :=
  offset <= size && count <= size - offset

/-- Conservatively exclude regions whose exclusive end wraps the address word. -/
@[inline] def addressable (base size : UInt32) : Bool :=
  size <= 0xffffffff - base

/-- Error results have an impossible section offset, followed by an error code. -/
@[inline] def error (code : UInt32) : UInt64 :=
  0xffffffff00000000 ||| code.toUInt64

@[inline] def failed (value : UInt64) : Bool := value >>> 32 == 0xffffffff

/-- Checked byte load; 256 denotes out of bounds. -/
@[specialize] def read8 (base size offset : UInt32) : m UInt32 := do
  if !addressable base size || !contains size offset 1 then return 256
  return (← Reader.byte (base + offset)).toUInt32

@[specialize] def write8 (base size offset value : UInt32) : m UInt32 := do
  if !addressable base size || !contains size offset 1 then return 1
  let _ ← Writer.byte (base + offset) value.toUInt8
  return 0

@[specialize] private partial def copyForward (dst src count : UInt32) : m UInt32 := do
  if count == 0 then return 0
  if count >= 4 && (dst ||| src) &&& 3 == 0 then
    let word ← Reader.word src
    let _ ← Writer.word dst word
    copyForward (dst + 4) (src + 4) (count - 4)
  else
    let byte ← Reader.byte src
    let _ ← Writer.byte dst byte
    copyForward (dst + 1) (src + 1) (count - 1)

@[specialize] private partial def copyBackward (dst src count : UInt32) : m UInt32 := do
  if count == 0 then return 0
  if count >= 4 && (dst ||| src ||| count) &&& 3 == 0 then
    let word ← Reader.word (src + count - 4)
    let _ ← Writer.word (dst + count - 4) word
    copyBackward dst src (count - 4)
  else
    let byte ← Reader.byte (src + count - 1)
    let _ ← Writer.byte (dst + count - 1) byte
    copyBackward dst src (count - 1)

/-- Bounded memmove. Caller owns both RAM spans; overlap is supported. -/
@[specialize]
def copy (dst dstSize dstOffset src srcSize srcOffset count : UInt32) : m UInt32 := do
  if !addressable dst dstSize || !addressable src srcSize ||
      !contains dstSize dstOffset count || !contains srcSize srcOffset count then return 1
  let d := dst + dstOffset
  let s := src + srcOffset
  if d > s && d - s < count then copyBackward d s count else copyForward d s count

@[specialize] private partial def fillLoop (dst count value : UInt32) : m UInt32 := do
  if count == 0 then return 0
  let _ ← Writer.byte dst value.toUInt8
  fillLoop (dst + 1) (count - 1) value

@[specialize] def fill (dst size offset count value : UInt32) : m UInt32 := do
  if !addressable dst size || !contains size offset count then return 1
  fillLoop (dst + offset) count value

@[specialize] def writeLE32 (base size offset value : UInt32) : m UInt32 := do
  if !addressable base size || !contains size offset 4 then return 1
  let _ ← Writer.byte (base + offset) value.toUInt8
  let _ ← Writer.byte (base + offset + 1) (value >>> 8).toUInt8
  let _ ← Writer.byte (base + offset + 2) (value >>> 16).toUInt8
  let _ ← Writer.byte (base + offset + 3) (value >>> 24).toUInt8
  return 0

@[inline] def swap32 (v : UInt32) : UInt32 :=
  let a : UInt32 := (v &&& 255) <<< 24
  let b : UInt32 := (v &&& 65280) <<< 8
  let c : UInt32 := (v >>> 8) &&& 65280
  let d : UInt32 := (v >>> 24) &&& 255
  a ||| b ||| c ||| d

@[specialize] def writeBE32 (base size offset value : UInt32) : m UInt32 :=
  writeLE32 base size offset (swap32 value)

/-- Checked unaligned little-endian read. Out-of-range calls perform no loads
    and return error 1; valid results fit in the low 32 bits. -/
@[specialize]
def readLE32 (base size offset : UInt32) : m UInt64 := do
  if !addressable base size || !contains size offset 4 then return error 1
  let address := base + offset
  let a ← Reader.byte address
  let b ← Reader.byte (address + 1)
  let c ← Reader.byte (address + 2)
  let d ← Reader.byte (address + 3)
  return a.toUInt64 ||| (b.toUInt64 <<< 8) ||| (c.toUInt64 <<< 16) ||| (d.toUInt64 <<< 24)

@[specialize] def readBE32 (base size offset : UInt32) : m UInt64 := do
  let v ← readLE32 base size offset
  if failed v then return v
  let a : UInt64 := (v &&& 255) <<< 24
  let b : UInt64 := (v &&& 65280) <<< 8
  let c : UInt64 := (v >>> 8) &&& 65280
  let d : UInt64 := (v >>> 24) &&& 255
  return a ||| b ||| c ||| d

end LeanBoot.Memory
