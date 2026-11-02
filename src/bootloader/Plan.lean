import Init

/- Pure boot policy. These records contain values, never live pointers with
   methods, callbacks to firmware, or access to memory/CPU/UART effects.
   withPlan is continuation-passing so the freestanding specialization can
   scalarize the records instead of allocating a boxed result. -/
namespace LeanBoot.Plan

structure Request where
  qemu : Bool
  bootSize : UInt32
  vendorSize : UInt32
  initSize : UInt32
  recipeOffset : UInt32
  recipeSize : UInt32
  configOffset : UInt32
  configSize : UInt32
  finalSize : UInt32
  digestOffset : UInt32
  digestSize : UInt32

structure KernelConfig where
  entry : UInt32
  fdt : UInt32
  initrd : UInt32
  initrdSize : UInt32
  secureMessage : UInt32

structure BootPlan where
  kernel : KernelConfig
  dtbSource : UInt32
  dtbSize : UInt32
  vendorOffset : UInt32
  vendorSize : UInt32
  genericOffset : UInt32
  genericSize : UInt32
  configSource : UInt32
  configSize : UInt32
  recipeSource : UInt32
  recipeSize : UInt32
  digestSource : UInt32
  osVersion : UInt32

@[inline] def bias (qemu : Bool) : UInt32 := if qemu then 0x20000000 else 0

@[inline] def contains (size offset count : UInt32) : Bool :=
  offset <= size && count <= size - offset

@[inline] def validRequest (r : Request) : UInt32 := Id.run do
  if r.bootSize > 0x2000000 || r.vendorSize > 0x1400000 || r.initSize > 0x800000 then return 2
  if !r.qemu && (r.bootSize != 15876096 || r.vendorSize != 18178560 || r.initSize != 8388608) then return 2
  if r.recipeOffset != 64 || r.recipeSize > 32768 || r.configOffset != 64 + r.recipeSize ||
      r.configSize == 0 || r.configSize > 4096 || !contains 65536 r.configOffset r.configSize then return 3
  if !r.qemu && (r.digestSize != 64 || !contains 65536 r.digestOffset r.digestSize ||
      r.configOffset.toUInt64 + r.configSize.toUInt64 > r.digestOffset.toUInt64) then return 24
  return 0

@[inline] private def offset (span : UInt64) : UInt32 := (span >>> 32).toUInt32
@[inline] private def length (span : UInt64) : UInt32 := span.toUInt32
@[inline] private def sectionFits (size : UInt32) (span : UInt64) : Bool :=
  length span != 0 && contains size (offset span) (length span)

/-- Linux bootconfig includes at least one padding byte, an aligned size and
    checksum, then the twelve-byte magic. Calculate in 64 bits before narrowing. -/
@[inline] def initrdLength (vendor generic config : UInt32) : UInt64 :=
  ((vendor.toUInt64 + generic.toUInt64 + config.toUInt64 + 4) &&& 0xfffffffffffffffc) + 20

/-- The last byte and OS version are values read by the inspector. No effects
    occur here. Either continuation is invoked exactly once, in tail position. -/
@[inline] def withPlan {α : Type} (r : Request)
    (kernel generic vendor wrapped dtb : UInt64) (lastConfigByte osVersion : UInt32)
    (reject : UInt32 → α) (accept : BootPlan → α) : α := Id.run do
  let bad := validRequest r
  if bad != 0 then return reject bad
  if !sectionFits r.bootSize kernel || (!r.qemu && kernel != 0x100000f22fac) then return reject 4
  if !sectionFits r.initSize generic || (!r.qemu && length generic != 2262498) then return reject 6
  if !sectionFits r.vendorSize vendor || !sectionFits r.vendorSize wrapped ||
      (!r.qemu && length vendor != 18128651) then return reject 7
  if !sectionFits (length wrapped) dtb || length dtb > 65536 then return reject 8
  let final := initrdLength (length vendor) (length generic) r.configSize
  if lastConfigByte != 10 || final > 0x1400000 || final != r.finalSize.toUInt64 then return reject 13
  if !r.qemu && osVersion != 0x1c000196 then return reject 24
  let b := bias r.qemu
  return accept {
    kernel := {
      entry := b + 0x20207000 + offset kernel
      fdt := b + 0x23000000
      initrd := b + 0x24000000
      initrdSize := final.toUInt32
      secureMessage := if r.qemu then 0 else 0x26320000 }
    dtbSource := b + 0x24000000 + offset wrapped + offset dtb
    dtbSize := length dtb
    vendorOffset := offset vendor, vendorSize := length vendor
    genericOffset := offset generic, genericSize := length generic
    configSource := b + 0x26200000 + r.configOffset, configSize := r.configSize
    recipeSource := b + 0x26200000 + r.recipeOffset, recipeSize := r.recipeSize
    digestSource := b + 0x26200000 + r.digestOffset, osVersion := osVersion }

end LeanBoot.Plan
