import Memory
import Images
import Fdt
import Bootconfig
import VerifiedBoot
import Os
import Unsafe.Ram
import Ethernet
import BootJournal
import Unsafe.Bootstrap
import BootBundle
import Lz4

namespace LeanBoot.Harness

@[export leanboot_journal_encode]
def journal_encode (base seq state : UInt32) : BaseIO UInt32 :=
  Unsafe.runMemory <| BootJournal.encode base seq state

@[export leanboot_journal_decode]
def journal_decode (base : UInt32) : BaseIO UInt64 :=
  Unsafe.runMemory <| BootJournal.decode base

@[export leanboot_journal_advance]
def journal_advance : BaseIO UInt32 := Unsafe.runHardware BootJournal.advance

@[export leanboot_bundle_normal]
def bundle_normal (fdt : UInt32) : BaseIO Bool := Unsafe.runMemory <| BootBundle.normalBoot fdt

@[export leanboot_bundle_hash]
def bundle_hash (address size : UInt32) : BaseIO UInt32 := Unsafe.runMemory <| BootBundle.hash address size

@[export leanboot_lz4_decode]
def lz4_decode (src size dst expected : UInt32) : BaseIO UInt32 :=
  Unsafe.runMemory <| Lz4.decode src size dst expected

@[export leanboot_ethernet_respond]
def ethernet_respond (rx length tx capacity : UInt32) : BaseIO UInt32 :=
  Unsafe.runMemory <| Ethernet.respond rx length tx capacity 0xc0a84d03 0x18acc22d1c47

@[export leanboot_os_prepare]
def prepare : BaseIO UInt32 := Unsafe.runMemory <| Os.withPrepared false fun _ => pure 0

@[export leanboot_copy]
def memory_copy (dst dstSize dstOffset src srcSize srcOffset count : UInt32) : BaseIO UInt32 := Unsafe.runMemory <| Memory.copy dst dstSize dstOffset src srcSize srcOffset count

@[export leanboot_read_le32]
def memory_readLE32 (base size offset : UInt32) : BaseIO UInt64 := Unsafe.runMemory <| Memory.readLE32 base size offset

@[export leanboot_boot_section]
def images_bootSection (base size role selector : UInt32) : BaseIO UInt64 := Unsafe.runMemory <| Images.bootSection base size role selector

@[export leanboot_vendor_section]
def images_vendorSection (base size selector : UInt32) : BaseIO UInt64 := Unsafe.runMemory <| Images.vendorSection base size selector

@[export leanboot_dtb_section]
def images_dtbSection (base size : UInt32) : BaseIO UInt64 := Unsafe.runMemory <| Images.dtbSection base size

@[export leanboot_fdt_validate]
def fdt_validate (b cap : UInt32) : BaseIO UInt32 := Unsafe.runMemory <| Fdt.validate b cap

@[export leanboot_fdt_patch]
def fdt_patch (b cap recipe recipeSize : UInt32) : BaseIO UInt32 := Unsafe.runMemory <| Fdt.patch b cap recipe recipeSize

@[export leanboot_fdt_reserve]
def fdt_reserve (b cap start size : UInt32) : BaseIO UInt32 := Unsafe.runMemory <| Fdt.reserve b cap start size

@[export leanboot_bootconfig_append]
def bootconfig_append (b capacity used config size : UInt32) : BaseIO UInt32 := Unsafe.runMemory <| Bootconfig.append b capacity used config size

@[export leanboot_verified_boot_packet]
def verifiedboot_packet (b capacity version digest size : UInt32) : BaseIO UInt32 := Unsafe.runMemory <| VerifiedBoot.packet b capacity version digest size

end LeanBoot.Harness
