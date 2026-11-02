import Unsafe.Primitives
import Unsafe.Ram
import BootJournal
import Cpu
import Update.FlashPlan

/- Exact-V013, bounded FCIE/DMA bootstrap. MBOOT establishes the card, bus
   timing and pads. All commands, polling and data movement here are Lean;
   no resident MBOOT function is called. Journal and application writers have
   separate fixed sector allowlists.
   Register semantics: exact V013 disassembly and MediaTek's BSD-licensed
   mt9221 eMMC_reg_v5.h. See CONTRACTS.txt for provenance and limitations. -/
namespace LeanBoot.Unsafe.Bootstrap

private def reg (index : UInt32) : HardwareM UInt32 :=
  load16 (0x1f222600 + index * 4)

private def put (index value : UInt32) : HardwareM UInt32 :=
  store16 (0x1f222600 + index * 4) value

private partial def wait (index mask expected fuel : UInt32) : HardwareM Bool := do
  if fuel == 0 then return false
  if (← reg index) &&& mask == expected then return true
  wait index mask expected (fuel - 1)

private def clear : HardwareM Bool := do
  let _ ← put 0 0x37
  let _ ← put 0x0d 0x3f
  let _ ← barrier
  return (← wait 0 0x37 0 1024) && (← wait 0x0d 0x3f 0 1024)

private def command (index argument ctrl mode : UInt32) : HardwareM UInt32 := do
  if !(← clear) then return 41
  let _ ← put 0x0a 0x0505
  let _ ← put 0x0b mode
  -- CMD FIFO reads expose the response unless CHK_CMD selects the command.
  let _ ← put 0x0c 0x80
  let a : UInt32 := ((argument >>> 24) <<< 8) ||| 0x40 ||| index
  let b : UInt32 := (argument &&& (0xff00 : UInt32)) ||| ((argument >>> 16) &&& (255 : UInt32))
  let c : UInt32 := argument &&& 255
  let _ ← put 0x20 a
  let _ ← put 0x21 b
  let _ ← put 0x22 c
  let _ ← barrier
  if (← reg 0x20) != a || (← reg 0x21) != b || (← reg 0x22) != c then return 42
  let _ ← put 0x0c 0
  if !(← wait 0x0d 0x40 0 1000000) then return 43
  let _ ← put 0x0c ctrl
  let _ ← put 0x0c (ctrl ||| 0x40)
  let _ ← barrier
  if !(← wait 0 2 2 1000000) then return 44
  if (← reg 0x0d) &&& 0x18 != 0 then return 45
  if (← reg 0x20) &&& 255 != index then return 46
  -- R1 error bits, excluding current state, ready-for-data and APP_CMD.
  let r1 : UInt32 := (((← reg 0x20) >>> 8) <<< 24) |||
    (((← reg 0x21) &&& 255) <<< 16) ||| (((← reg 0x21) >>> 8) <<< 8) |||
    ((← reg 0x22) &&& 255)
  if r1 &&& 0xfdffa080 != 0 then return 47
  return 0

private def transfer (index sector buffer : UInt32) : HardwareM UInt32 := do
  let write := index == 24
  let mode ← reg 0x0b
  let priority ← reg 2
  let interrupts ← reg 1
  -- Require the initialized 8-bit card and a quiescent controller.
  if mode &&& 6 != 4 || (← reg 0x0c) &&& 0x40 != 0 ||
      (← reg 7) &&& 1 != 1 || (← reg 0x3f) &&& 1 != 1 ||
      (← liftMemory <| load32 0x28fa384c) != 0x294a34c0 ||
      (← liftMemory <| load8 0x294a3735) != 1 then return 51
  let cached := (← readSctlr) &&& 4 != 0
  if cached then let _ ← Cpu.cleanCaches
  if (← Cpu.flushMiu) != 0 then return 56
  let _ ← put 1 0
  let _ ← put 2 (priority &&& 0xfff3)
  let dma := buffer - 0x20000000
  let _ ← put 3 (dma &&& 0xffff)
  let _ ← put 4 (dma >>> 16)
  let _ ← put 8 1
  let _ ← put 9 512
  let _ ← put 5 512
  let _ ← put 6 0
  let currentMode := (mode &&& 0xffef) ||| 1
  let status ← command index sector (if write then 6 else 14) currentMode
  let status ← if status != 0 then pure status else if write then do
      if !(← wait 0x0d 0x40 0 1000000) then pure 43 else do
        let _ ← put 0x0c 0x18
        let _ ← put 0x0c 0x58
        pure 0
    else pure 0
  let status ← if status != 0 then pure status else do
    if !(← wait 0 1 1 1000000) then pure 52
    else if (← reg 0x0d) &&& 0x3f != 0 then pure 53
    else if !(← wait 0x0d 0x40 0 1000000) then pure 54
    -- Errors may become visible while the card is finishing its busy phase.
    else if (← reg 0x0d) &&& 0x3f != 0 then pure 53
    else pure 0
  -- Preserve an error snapshot and stop the job before returning to the caller.
  let _ ← liftMemory <| store32 0x26350f00 status
  let _ ← liftMemory <| store32 0x26350f04 (← reg 0x0d)
  let _ ← liftMemory <| store32 0x26350f08 (← reg 0)
  let _ ← put 0x0c 0
  if status != 0 then
    -- Hold the controller in reset on an incomplete DMA transaction. Linux
    -- will initialize it; no later transfer may reuse a timed-out engine.
    let _ ← put 7 0
    let _ ← put 0x3f 0
    let _ ← wait 0x3f 14 14 1000000
  let _ ← put 0x0b (mode &&& 0xfffe)
  let _ ← put 2 priority
  let _ ← put 1 interrupts
  let _ ← barrier
  if cached then let _ ← Cpu.cleanCaches
  return status

private def allowed (sector buffer : UInt32) : Bool :=
  (sector == 0x3217fe || sector == 0x3217ff) &&
  buffer >= 0x26350000 && buffer <= 0x26350a00 && buffer &&& 63 == 0

@[export leanboot_read_sector]
def readSector (sector buffer : UInt32) : HardwareM UInt32 := do
  if !allowed sector buffer then return 40
  transfer 17 sector buffer

@[export leanboot_write_journal]
def writeJournal (sector buffer : UInt32) : HardwareM UInt32 := do
  if !allowed sector buffer then return 40
  -- Read live EXT_CSD, not MBOOT's cached initial snapshot. Never change the
  -- card's partition selection, cache policy or sector size implicitly.
  let status ← transfer 8 0 0x26350c00
  if status != 0 then return status
  if (← liftMemory <| load8 0x26350cb3) &&& 7 != 0 ||
      (← liftMemory <| load8 0x26350c21) != 0 ||
      (← liftMemory <| load8 0x26350c3d) != 0 then return 55
  transfer 24 sector buffer

instance : BootJournal.Blocks HardwareM where
  read := fun slot buffer =>
    if slot > 1 then pure 40 else readSector (0x3217fe + slot) buffer
  write := fun slot buffer =>
    if slot > 1 then pure 40 else writeJournal (0x3217fe + slot) buffer

/-- The update path uses one fixed DMA bounce buffer. The original journal
    API retains its old two-sector restriction. Card settings are checked
    before every write; no erase, partition switch or cache-policy command. -/
def flashSector (writing : Bool) (bank sector : UInt32) : HardwareM UInt32 :=
  Update.Flash.withSector bank sector (pure 40) fun address => do
    -- Reads must use the same user partition that Linux later mounts. Checking
    -- only writes could authenticate a bank in another hardware partition.
    let status ← transfer 8 0 0x26350c00
    if status != 0 then return status
    if (← liftMemory <| load8 0x26350cb3) &&& 7 != 0 ||
        (← liftMemory <| load8 0x26350c21) != 0 ||
        (← liftMemory <| load8 0x26350c3d) != 0 then return 55
    transfer (if writing then 24 else 17) address.value 0x26350800

/-- With the card write cache disabled, CMD24 completion plus DAT0 ready is
    the durability contract. Drain MIU writes and recheck controller idle.
    Physical power-loss behavior remains a hardware validation obligation. -/
def flashFlush : HardwareM Bool := do
  if (← reg 7) &&& 1 != 1 || (← reg 0x3f) &&& 1 != 1 ||
      (← reg 0x0c) &&& 0x40 != 0 then return false
  if (← Cpu.flushMiu) != 0 then return false
  return (← wait 0x0d 0x40 0 1000000) && ((← reg 0x0d) &&& 0x3f == 0)

/-- A short identity guard complements the full installed MBOOT hash checked
    by the host. Relocation is fixed for this board/build. -/
def guard : HardwareM Bool := do
  return (← liftMemory <| load32 0x28d8030c) == 0xe1a0c00d &&
    (← liftMemory <| load32 0x28d80310) == 0xe92ddef0 &&
    (← liftMemory <| load32 0x28d8031c) == 0xe59f93e8 &&
    (← liftMemory <| load32 0x28d7dfc4) == 0xe1a0c00d &&
    (← liftMemory <| load32 0x28d7dfcc) == 0xe92ddef0

end LeanBoot.Unsafe.Bootstrap
