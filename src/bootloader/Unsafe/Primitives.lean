import Unsafe.Effects

/- The shared runtime FFI surface. Raw BaseIO declarations are private;
   public operations carry a memory or hardware effect color. Assembly lives
   only in tools/bootloader/Unsafe/Arm.lean. See CONTRACTS.txt for physical preconditions. -/
namespace LeanBoot.Unsafe

@[extern "leanboot_load8"]
private opaque raw_load8 (address : UInt32) : BaseIO UInt8

@[inline] def load8 (address : UInt32) : MemoryM UInt8 :=
  fun _ => raw_load8 address

@[extern "leanboot_store8"]
private opaque raw_store8 (address : UInt32) (value : UInt8) : BaseIO UInt32

@[inline] def store8 (address : UInt32) (value : UInt8) : MemoryM UInt32 :=
  fun _ => raw_store8 address value

@[extern "leanboot_load32"]
private opaque raw_load32 (address : UInt32) : BaseIO UInt32

@[inline] def load32 (address : UInt32) : MemoryM UInt32 :=
  fun _ => raw_load32 address

@[extern "leanboot_store32"]
private opaque raw_store32 (address value : UInt32) : BaseIO UInt32

@[inline] def store32 (address value : UInt32) : MemoryM UInt32 :=
  fun _ => raw_store32 address value

@[extern "leanboot_load16"]
private opaque raw_load16 (address : UInt32) : BaseIO UInt32

@[inline] def load16 (address : UInt32) : HardwareM UInt32 :=
  fun _ => raw_load16 address

@[extern "leanboot_store16"]
private opaque raw_store16 (address value : UInt32) : BaseIO UInt32

@[inline] def store16 (address value : UInt32) : HardwareM UInt32 :=
  fun _ => raw_store16 address value

@[extern "leanboot_read_sctlr"]
private opaque raw_readSctlr : BaseIO UInt32

@[inline] def readSctlr : HardwareM UInt32 :=
  fun _ => raw_readSctlr

@[extern "leanboot_write_sctlr"]
private opaque raw_writeSctlr (value : UInt32) : BaseIO UInt32

/-- Only enable instruction fetch caching; leave MMU/data-cache state alone.
    Entry has already invalidated instructions after the code relocation. -/
def enableInstructionCache : HardwareM UInt32 := fun _ => do
  let value ← raw_readSctlr
  raw_writeSctlr (value ||| 0x1000)

@[extern "leanboot_read_cpsr"]
private opaque raw_readCpsr : BaseIO UInt32

@[inline] def readCpsr : HardwareM UInt32 :=
  fun _ => raw_readCpsr

@[extern "leanboot_mask_interrupts"]
private opaque raw_maskInterrupts : BaseIO UInt32

@[inline] def maskInterrupts : HardwareM UInt32 := fun _ => raw_maskInterrupts

@[extern "leanboot_restore_interrupts"]
private opaque raw_restoreInterrupts (saved : UInt32) : BaseIO UInt32

@[inline] def restoreInterrupts (saved : UInt32) : HardwareM UInt32 :=
  fun _ => raw_restoreInterrupts saved

@[extern "leanboot_read_sp"]
private opaque raw_readSp : BaseIO UInt32

@[inline] def readSp : HardwareM UInt32 :=
  fun _ => raw_readSp

@[extern "leanboot_read_clidr"]
private opaque raw_readClidr : BaseIO UInt32

@[inline] def readClidr : HardwareM UInt32 :=
  fun _ => raw_readClidr

@[extern "leanboot_read_ccsidr"]
private opaque raw_readCcsidr : BaseIO UInt32

@[inline] def readCcsidr : HardwareM UInt32 :=
  fun _ => raw_readCcsidr

@[extern "leanboot_select_cache"]
private opaque raw_selectCache (value : UInt32) : BaseIO UInt32

@[inline] def selectCache (value : UInt32) : HardwareM UInt32 :=
  fun _ => raw_selectCache value

@[extern "leanboot_clean_setway"]
private opaque raw_cleanSetWay (value : UInt32) : BaseIO UInt32

@[inline] def cleanSetWay (value : UInt32) : HardwareM UInt32 :=
  fun _ => raw_cleanSetWay value

@[extern "leanboot_barrier"]
private opaque raw_barrier : BaseIO UInt32

@[inline] def barrier : HardwareM UInt32 :=
  fun _ => raw_barrier

@[extern "leanboot_send_verified_boot"]
private opaque raw_sendVerifiedBoot (message : UInt32) : BaseIO UInt32

@[inline] def sendVerifiedBoot (message : UInt32) : HardwareM UInt32 :=
  fun _ => raw_sendVerifiedBoot message

@[extern "leanboot_begin_transition"]
private opaque raw_beginTransition (sctlr kernel fdt : UInt32) : BaseIO UInt32

@[inline] def beginTransition (sctlr kernel fdt : UInt32) : HardwareM UInt32 :=
  fun _ => raw_beginTransition sctlr kernel fdt

@[extern "leanboot_jump_linux"]
private opaque raw_jumpLinux (kernel fdt : UInt32) : BaseIO UInt32

@[inline] def jumpLinux (kernel fdt : UInt32) : HardwareM UInt32 :=
  fun _ => raw_jumpLinux kernel fdt

@[extern "leanboot_qemu_transition"]
private opaque raw_qemuTransition (sctlr kernel fdt : UInt32) : BaseIO UInt32

@[inline] def qemuTransition (sctlr kernel fdt : UInt32) : HardwareM UInt32 :=
  fun _ => raw_qemuTransition sctlr kernel fdt

@[extern "leanboot_qemu_jump"]
private opaque raw_qemuJump (kernel fdt : UInt32) : BaseIO UInt32

@[inline] def qemuJump (kernel fdt : UInt32) : HardwareM UInt32 :=
  fun _ => raw_qemuJump kernel fdt

/- MMIO uses the same instruction as RAM, but requires the hardware color. -/
@[inline] def readMmio8 (address : UInt32) : HardwareM UInt8 :=
  fun _ => raw_load8 address

@[inline] def writeMmio8 (address : UInt32) (value : UInt8) : HardwareM UInt32 :=
  fun _ => raw_store8 address value

@[inline] def readMmio32 (address : UInt32) : HardwareM UInt32 :=
  fun _ => raw_load32 address

@[inline] def writeMmio32 (address value : UInt32) : HardwareM UInt32 :=
  fun _ => raw_store32 address value

/-- UART policy is ordinary Lean; reuse the reviewed MMIO and barrier primitives.
    The shared barrier provides DSB plus ISB (the old UART wrapper used DSB). -/
@[inline] def uartStatus : HardwareM UInt32 := readMmio32 0x1f201328

@[inline] def uartWrite (value : UInt8) : HardwareM UInt32 := do
  let _ ← writeMmio8 0x1f201300 value
  barrier

end LeanBoot.Unsafe
