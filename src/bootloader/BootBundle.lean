import Memory
import Fdt
import BootJournal
import Lz4

/- Exact-G13 bundle layout and integrity checks; no physical adapter here.
   The original kernel and the larger preparation payload are separate so
   the selector can keep using MBOOT's prepared initrd/FDT on fallback. -/
namespace LeanBoot.BootBundle
open Memory
variable {m : Type → Type} [Monad m] [Reader m] [Writer m]

def base : UInt32 := 0x40000000
def manifest : UInt32 := 0x40002000
def total : UInt32 := 15871916
def prefixSize : UInt32 := 0x20000
def restoreSize : UInt32 := 0x21000
def bootSize : UInt32 := 0xf24000
def vendorSize : UInt32 := 0x1156200
def initSize : UInt32 := 0x800000
def controlSize : UInt32 := 0xc40

def vendorBytes : UInt32 := 18128651
def genericBytes : UInt32 := 2262498
def vendorTail : UInt32 := 18132992
def metaSize : UInt32 := 56896

@[specialize] private partial def hashLoop (address bytes value : UInt32) : m UInt32 := do
  if bytes < 4 then
    let a ← if bytes > 0 then read8 address bytes 0 else pure 0
    let b ← if bytes > 1 then read8 address bytes 1 else pure 0
    let c ← if bytes > 2 then read8 address bytes 2 else pure 0
    return if bytes == 0 then value else
      (value ^^^ (a ||| (b <<< 8) ||| (c <<< 16))) * 16777619
  hashLoop (address + 4) (bytes - 4) ((value ^^^ (← Reader.word address)) * 16777619)

/-- Accidental-corruption check, not authentication. -/
@[specialize] def hash (address bytes : UInt32) : m UInt32 :=
  hashLoop address bytes 2166136261

@[specialize] def field (offset : UInt32) : m UInt32 :=
  readLE32 manifest 4096 offset >>= fun x => pure x.toUInt32

@[specialize] def headerValid : m Bool := do
  let size ← field 20
  let original ← field 16
  let metadata ← field 36
  let gateSize ← field 64
  let gateBytes ← field 68
  let metaCompressed ← field 40
  return (← field 0) == 0x3142474c && (← field 4) == 4 && (← field 8) == total &&
    (← field 60) == 0x3000 && gateSize > 0 && contains prefixSize 0x3000 gateSize &&
    gateBytes > 0 && gateBytes <= 0x40000 && original >= 0x3000 + gateSize &&
    size > 0 && contains prefixSize original size &&
    metadata >= original + size && metadata &&& 3 == 0 && metaCompressed > 0 &&
    contains prefixSize metadata metaCompressed && (← field 52) == vendorBytes + genericBytes &&
    (← field 56) == prefixSize &&
    (← BootJournal.crc manifest 76) == (← field 76)

@[specialize] def compressedValid : m Bool := do
  return (← hash (base + (← field 16)) (← field 20)) == (← field 24)

@[specialize] def kernelValid : m Bool := do
  return (← hash 0x20207000 bootSize) == (← field 32) &&
    (← readLE32 0x20207000 bootSize 4096) == 0xe28f9001 &&
    (← readLE32 0x20207000 bootSize 4100) == 0xe12fff19

@[specialize] def payloadValid : m Bool := do
  if (← hash 0x26000000 (← field 68)) != (← field 28) ||
      (← hash (base + (← field 36)) (← field 40)) != (← field 44) ||
      (← hash 0x24000000 (vendorBytes + genericBytes)) != (← field 48) then return false
  return (← Lz4.decode (base + (← field 36)) (← field 40) (base + prefixSize) metaSize) == 0

@[specialize] def inputs : m Unit := do
  let metadata := base + prefixSize
  let _ ← copy 0x24000000 vendorSize 0 metadata metaSize 0 4096
  let _ ← copy 0x24000000 vendorSize 4096 0x26500000 0x1400000 0 vendorBytes
  let _ ← fill 0x24000000 vendorSize (4096 + vendorBytes) (vendorTail - 4096 - vendorBytes) 0
  let _ ← copy 0x24000000 vendorSize vendorTail metadata metaSize 4096 (vendorSize - vendorTail)
  -- Only the header and generic ramdisk are consumed by Images.bootSection;
  -- do not write the unused multi-megabyte container tail.
  let _ ← copy 0x25400000 initSize 0 metadata metaSize 49664 4096
  let _ ← copy 0x25400000 initSize 4096 0x26500000 0x1400000 vendorBytes genericBytes
  let _ ← copy 0x26200000 65536 0 metadata metaSize 53760 controlSize
  pure ()

@[specialize] private def initrdName (address : UInt32) : m Bool := do
  return (← readLE32 address 16 0) == 0x756e696c &&
    (← readLE32 address 17 4) == 0x6e692c78 &&
    (← readLE32 address 16 8) == 0x64727469 &&
    (← readLE32 address 17 12) == 0x646e652d && (← read8 address 17 16) == 0

@[specialize] private partial def findEnd (fdt limit strings p : UInt32) : m UInt32 := do
  if !contains limit p 4 then return 0
  let token := (← readBE32 fdt limit p).toUInt32
  if token == 1 then
    let rec skip (q : UInt32) : m UInt32 := do
      if q >= limit then return 0
      if (← read8 fdt limit q) == 0 then return (q + 4) &&& 0xfffffffc
      skip (q + 1)
    let q ← skip (p + 4)
    if q == 0 then return 0
    findEnd fdt limit strings q
  else if token == 3 then
    if !contains limit p 12 then return 0
    let size := (← readBE32 fdt limit (p + 4)).toUInt32
    let name := (← readBE32 fdt limit (p + 8)).toUInt32
    if strings > limit || !contains limit (p + 12) size then return 0
    if size == 4 && contains (limit - strings) name 17 && (← initrdName (fdt + strings + name)) then
      return (← readBE32 fdt limit (p + 12)).toUInt32
    findEnd fdt limit strings ((p + 12 + size + 3) &&& 0xfffffffc)
  else if token == 2 || token == 4 then findEnd fdt limit strings (p + 4)
  else return 0

private def normalByte (index : UInt32) : UInt32 :=
  let word : UInt64 := if index < 8 then 0x6264696f72646e61
    else if index < 16 then 0x63726f662e746f6f
    else if index < 24 then 0x6c616d726f6e5f65
    else 0x746f6f625f
  ((word >>> ((index % 8) * 8).toUInt64) &&& 255).toUInt32

@[specialize] private partial def matchesKey (address index : UInt32) : m Bool := do
  if index == 29 then return true
  if (← Reader.byte (address + index)).toUInt32 != normalByte index then return false
  matchesKey address (index + 1)

@[specialize] private partial def spaces (address size offset : UInt32) : m UInt32 := do
  let c ← read8 address size offset
  if c == 32 || c == 9 then spaces address size (offset + 1) else pure offset

@[specialize] private def normalValue (address size offset : UInt32) : m Bool := do
  let p ← spaces address size offset
  if (← read8 address size p) != 61 then return false
  let p ← spaces address size (p + 1)
  let quoted := (← read8 address size p) == 34
  let p := if quoted then p + 1 else p
  if (← read8 address size p) != 49 then return false
  let p := p + 1
  if quoted && (← read8 address size p) != 34 then return false
  let p ← spaces address size (if quoted then p + 1 else p)
  let c ← read8 address size p
  return p == size || c == 10 || c == 0

@[specialize] private partial def normalFlag (address size offset : UInt32) : m Bool := do
  if !contains size offset 29 then return false
  if (offset == 0 || (← Reader.byte (address + offset - 1)) == 10) &&
      (← matchesKey (address + offset) 0) then
    return ← normalValue address size (offset + 29)
  normalFlag address size (offset + 1)

/-- Missing/malformed normal-boot flag means recovery/fallback, before any
    journal write or initrd modification. The original FDT is never edited. -/
@[specialize] def normalBoot (fdt : UInt32) : m Bool := do
  if fdt < 0x20000000 || fdt > 0x297e0000 || fdt &&& 3 != 0 then return false
  if (← Fdt.validate fdt 65536) != 0 then return false
  let limit := (← readBE32 fdt 65536 4).toUInt32
  let strings := (← readBE32 fdt 65536 12).toUInt32
  let start := (← readBE32 fdt 65536 8).toUInt32
  let stop ← findEnd fdt limit strings start
  if stop < 0x24000000 + 20 || stop > 0x25400000 then return false
  let trailer := stop - 20
  if (← readLE32 trailer 20 8) != 0x4f4f4223 ||
      (← readLE32 trailer 20 12) != 0x4e4f4354 ||
      (← readLE32 trailer 20 16) != 0x0a474946 then return false
  let size := (← readLE32 trailer 20 0).toUInt32
  if size < 32 || size > 32768 || size > trailer - 0x24000000 then return false
  normalFlag (trailer - size) size 0

end LeanBoot.BootBundle
