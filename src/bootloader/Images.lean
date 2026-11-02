import Memory

/- Allocation-free readers for the exact G13 Android v4 layout. They return
   section offsets/sizes, never load destinations. No image code is executed.
   The source RAM must remain stable for the whole call.

   Result = offset << 32 | length. Error offset 0xffffffff, codes:
   1 invalid/truncated address range, 2 magic, 3 unsupported version/profile,
   4 header fields, 5 truncated/overflowing layout, 6 wrong boot/init role,
   7 unknown section selector, 8 unsupported vendor table, 9 FDT header.
   This is structural validation, NOT authentication or full FDT validation. -/
namespace LeanBoot.Images

variable {m : Type → Type} [Monad m] [Memory.Reader m] [Memory.Writer m]
open Memory

@[specialize] private def alignUp (value page : UInt64) : UInt64 :=
  (value + page - 1) &&& ~~~(page - 1)

@[specialize] private def packSection (offset length : UInt64) : UInt64 :=
  (offset <<< 32) ||| length

@[specialize] private partial def wildcardBoard (base size offset index : UInt32) : m Bool := do
  if index == 16 then return true
  let board ← readLE32 base size (offset + 44 + index * 4)
  if board != 0 then return false
  wildcardBoard base size offset (index + 1)

/-- role 0 = boot (kernel only), 1 = init_boot (generic ramdisk only).
    selector 0 = kernel, 1 = generic ramdisk, 2 = boot signature.
    All sections are validated before any one is returned. -/
@[specialize]
def bootSection (base size role selector : UInt32) : m UInt64 := do
  if !addressable base size || size < 1584 then return error 1
  let magic0 ← readLE32 base size 0
  let magic1 ← readLE32 base size 4
  if magic0 != 0x52444e41 || magic1 != 0x2144494f then return error 2
  let version ← readLE32 base size 40
  if version != 4 then return error 3
  let header ← readLE32 base size 20
  let reserved0 ← readLE32 base size 24
  let reserved1 ← readLE32 base size 28
  let reserved2 ← readLE32 base size 32
  let reserved3 ← readLE32 base size 36
  if header != 1584 || reserved0 != 0 || reserved1 != 0 ||
      reserved2 != 0 || reserved3 != 0 then return error 4
  let kernel ← readLE32 base size 8
  let ramdisk ← readLE32 base size 12
  let signature ← readLE32 base size 1580
  if role == 0 then
    if kernel == 0 || ramdisk != 0 then return error 6
  else if role == 1 then
    if kernel != 0 || ramdisk == 0 then return error 6
  else return error 6
  let ramdiskOffset := 4096 + alignUp kernel 4096
  let signatureOffset := ramdiskOffset + alignUp ramdisk 4096
  if signatureOffset + signature > size.toUInt64 then return error 5
  if selector == 0 then return packSection 4096 kernel
  else if selector == 1 then return packSection ramdiskOffset ramdisk
  else if selector == 2 then return packSection signatureOffset signature
  else return error 7

/-- G13 v4, 2048-byte pages, one complete platform fragment, no board selector.
    selector 0 = vendor ramdisk, 1 = wrapped DTB, 2 = bootconfig, 3 = table.
    Other layouts are explicitly unsupported, rather than guessed at. -/
@[specialize]
def vendorSection (base size selector : UInt32) : m UInt64 := do
  if !addressable base size || size < 2128 then return error 1
  let magic0 ← readLE32 base size 0
  let magic1 ← readLE32 base size 4
  if magic0 != 0x52444e56 || magic1 != 0x544f4f42 then return error 2
  let version ← readLE32 base size 8
  let page ← readLE32 base size 12
  if version != 4 || page != 2048 then return error 3
  let header ← readLE32 base size 2096
  if header != 2128 then return error 4
  let ramdisk ← readLE32 base size 24
  let dtb ← readLE32 base size 2100
  let table ← readLE32 base size 2112
  let count ← readLE32 base size 2116
  let entrySize ← readLE32 base size 2120
  let bootconfig ← readLE32 base size 2124
  if ramdisk == 0 || dtb == 0 then return error 4
  if table != 108 || count != 1 || entrySize != 108 then return error 8
  let dtbOffset := 4096 + alignUp ramdisk 2048
  let tableOffset := dtbOffset + alignUp dtb 2048
  let configOffset := tableOffset + alignUp table 2048
  if configOffset + bootconfig > size.toUInt64 then return error 5
  let fragmentSize ← readLE32 base size tableOffset.toUInt32
  let fragmentOffset ← readLE32 base size (tableOffset.toUInt32 + 4)
  let fragmentType ← readLE32 base size (tableOffset.toUInt32 + 8)
  if fragmentSize != ramdisk || fragmentOffset != 0 || fragmentType != 1 then
    return error 8
  -- Board-id selection is intentionally not implemented. Accept only wildcards.
  if !(← wildcardBoard base size tableOffset.toUInt32 0) then return error 8
  if selector == 0 then return packSection 4096 ramdisk
  else if selector == 1 then return packSection dtbOffset dtb
  else if selector == 2 then return packSection configOffset bootconfig
  else if selector == 3 then return packSection tableOffset table
  else return error 7

/-- Locate the single wildcard DTB in the G13 Android DT table. Validate the
    container and FDT magic/length, not the FDT token stream or its properties. -/
@[specialize]
def dtbSection (base size : UInt32) : m UInt64 := do
  if !addressable base size || size < 64 then return error 1
  let magic ← readBE32 base size 0
  if magic != 0xd7b7ab1e then return error 2
  let total ← readBE32 base size 4
  let header ← readBE32 base size 8
  let entrySize ← readBE32 base size 12
  let count ← readBE32 base size 16
  let entries ← readBE32 base size 20
  let page ← readBE32 base size 24
  let version ← readBE32 base size 28
  if header != 32 || entrySize != 32 || count != 1 || entries != 32 ||
      page != 2048 || version != 0 then return error 3
  let length ← readBE32 base size 32
  let offset ← readBE32 base size 36
  if total > size.toUInt64 || total < 104 || offset != 64 || length < 40 ||
      offset + length != total then return error 5
  let id ← readBE32 base size 40
  let revision ← readBE32 base size 44
  let custom0 ← readBE32 base size 48
  let custom1 ← readBE32 base size 52
  let custom2 ← readBE32 base size 56
  let custom3 ← readBE32 base size 60
  if id != 0 || revision != 0 || custom0 != 0 || custom1 != 0 ||
      custom2 != 0 || custom3 != 0 then return error 3
  let fdtMagic ← readBE32 base size 64
  let fdtLength ← readBE32 base size 68
  if fdtMagic != 0xd00dfeed || fdtLength != length then return error 9
  return packSection offset length

end LeanBoot.Images
