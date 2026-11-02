import Unsafe.Ram
import Cpu
import EmacConfig
import Ethernet
import Ethernet.Descriptor

/- G13-only, polled EMAC boundary. See CONTRACTS.txt. All device accesses are
   HardwareM; DMA-owned memory is not exposed to the deterministic core. -/
namespace LeanBoot.Unsafe.Emac

def readReg (offset : UInt32) : HardwareM UInt32 := do
  let lo ← readMmio32 (0x1f204000 + offset * 2)
  let hi ← readMmio32 (0x1f204004 + offset * 2)
  return (lo &&& (65535 : UInt32)) ||| (hi <<< (16 : UInt32))

def writeReg (offset value : UInt32) : HardwareM UInt32 := do
  let _ ← writeMmio32 (0x1f204000 + offset * 2) (value &&& 65535)
  writeMmio32 (0x1f204004 + offset * 2) (value >>> 16)

private partial def phyRecipe (index : UInt32) : HardwareM UInt32 := do
  let step := EmacConfig.phyStep index
  if step == 0 then return 0
  let _ ← writeMmio8 (step >>> 32).toUInt32 step.toUInt8
  phyRecipe (index + 1)

def powerOn : HardwareM UInt32 := do
  let _ ← phyRecipe 0
  let pads ← readMmio8 0x1f203d41
  let _ ← writeMmio8 0x1f203d41 (pads &&& 0x7f)
  let mode ← readMmio8 0x1f203dbd
  let _ ← writeMmio8 0x1f203dbd (mode &&& 0xfe)
  barrier

private partial def waitMask (offset mask expected fuel : UInt32) : HardwareM UInt32 := do
  if fuel == 0 then return 1
  if (← readReg offset) &&& mask == expected then return 0
  waitMask offset mask expected (fuel - 1)

/-- MDIO management read; 0x10000 is timeout. Restore the management enable. -/
def phyRead (address reg : UInt32) : HardwareM UInt32 := do
  let ctl ← readReg 0
  let _ ← writeReg 0 (ctl ||| 16)
  let _ ← writeReg 0x34 ((0x60020000 : UInt32) ||| ((address &&& 31) <<< 23) ||| ((reg &&& 31) <<< 18))
  let timeout ← waitMask 8 4 4 100000
  let value ← readReg 0x34
  let _ ← writeReg 0 ctl
  return if timeout == 0 then value &&& 65535 else 65536

def phyWrite (address reg value : UInt32) : HardwareM UInt32 := do
  let ctl ← readReg 0
  let _ ← writeReg 0 (ctl ||| 16)
  let _ ← writeReg 0x34 ((0x50020000 : UInt32) ||| ((address &&& 31) <<< 23) |||
    ((reg &&& 31) <<< 18) ||| (value &&& 65535))
  let status ← waitMask 8 4 4 100000
  let _ ← writeReg 0 ctl
  return status

private partial def link (address fuel : UInt32) : HardwareM UInt32 := do
  if fuel == 0 then return 32
  let status ← phyRead address 1
  if status == 65536 then return 31
  if status &&& 0x24 == 0x24 then return 0
  link address (fuel - 1)

/-- Four private 2 KiB RX buffers; a 64-byte descriptor line. CPU uses only
    the uncached alias, MAC addresses are relative to MIU0 at 0x20000000. -/
private def dmaBase : HardwareM UInt32 := do
  return if (← readSctlr) &&& 1 == 0 then 0x26400000 else 0x66400000

private partial def descriptors (base index : UInt32) : HardwareM UInt32 := do
  if index == 4 then return 0
  let _ ← liftMemory <| store32 (base + index * 8)
    (0x06401000 + index * 2048 + (if index == 3 then 2 else 0))
  let _ ← liftMemory <| store32 (base + 4 + index * 8) 0
  descriptors base (index + 1)

def configure (address : UInt32) : HardwareM UInt32 := do
  -- Same autonegotiation enable as exact V013 MBOOT 2749fbd8.
  if (← phyWrite address 0 0x1000) != 0 then return 31
  let state ← link address 200000
  if state != 0 then return state
  let advertised ← phyRead address 4
  let partner ← phyRead address 5
  if advertised > 65535 || partner > 65535 then return 31
  let common := advertised &&& partner
  let config : UInt32 := if common &&& 0x100 != 0 then 0x803 else
    if common &&& 0x80 != 0 then 0x801 else
    if common &&& 0x40 != 0 then 0x802 else 0x800
  let _ ← writeReg 0 0
  let _ ← writeReg 0x2c 0xffff
  let base ← dmaBase
  let _ ← descriptors base 0
  let _ ← liftMemory <| Memory.fill (base + 0x1000) 0x2000 0 0x2000 0
  let _ ← barrier
  if (← Cpu.flushMiu) != 0 then return 20
  let _ ← writeReg 0x100 0xf011
  let _ ← writeReg 0x104 1 -- software descriptors, checksum off, no IRQ delay
  let _ ← writeReg 0x1c 0x06401001
  let _ ← writeReg 0x38 0
  let _ ← writeReg 0x3c 0
  let _ ← writeReg 0x18 0x06400000
  let _ ← writeReg 0x98 0x2dc2ac18
  let _ ← writeReg 0x9c 0x471c
  let _ ← writeReg 4 config
  let _ ← writeReg 0x14 0x26 -- clear old TX completion/error status
  let _ ← writeReg 0x20 0xf
  let _ ← barrier
  writeReg 0 12

private def transmit (length : UInt32) : HardwareM UInt32 := do
  if (← waitMask 0x14 0x10 0x10 100000) != 0 then return 34
  let _ ← barrier
  if (← Cpu.flushMiu) != 0 then return 20
  let _ ← writeReg 0x14 0x26
  let _ ← writeReg 0x0c 0x06404000
  let _ ← writeReg 0x10 length
  if (← waitMask 0x14 0x20 0x20 1000000) != 0 then return 35
  return 0

def report (offset value : UInt32) : HardwareM UInt32 :=
  liftMemory <| store32 (0x26200100 + offset) value

/-- Packet, idle-poll and total-poll budgets bound the session. Replies renew
    the idle budget, so a large stop-and-wait upload can outlast the initial
    discovery window. No traffic can renew the total budget. Any UART byte stops it.
    The core only sees completed descriptors and a CPU-owned TX buffer. -/
@[specialize] private partial def receiveLoop (respond : UInt32 → UInt32 → HardwareM UInt32)
    (finished : HardwareM Bool) (base index idleLimit idle fuel received sent : UInt32) : HardwareM UInt32 := do
  if idle == 0 || fuel == 0 || received >= 65536 then return 0
  if (← finished) then return 0
  if (← uartStatus) &&& 1 != 0 then
    let _ ← readMmio8 0x1f201300
    return 0
  let desc := base + index * 8
  let address ← liftMemory <| load32 desc
  if !Ethernet.Descriptor.ready address then
    receiveLoop respond finished base index idleLimit (idle - 1) (fuel - 1) received sent
  else
    let _ ← barrier
    let raw ← liftMemory <| load32 (desc + 4)
    let expected := Ethernet.Descriptor.expected index
    if !Ethernet.Descriptor.validAddress index address then return 36
    let response ← Ethernet.Descriptor.withPayload raw (pure 0) fun length =>
      respond (base + 0x1000 + index * 2048) length.value
    let status ← if response != 0 then transmit response else pure 0
    let _ ← report 16 (received + 1)
    let _ ← report 20 (sent + (if response == 0 then 0 else 1))
    let _ ← report 24 raw
    let _ ← report 28 address
    let _ ← barrier
    let _ ← liftMemory <| store32 desc expected
    let _ ← barrier
    if status != 0 then return status
    if (← finished) then return 0
    receiveLoop respond finished base ((index + 1) &&& 3) idleLimit
      (if response == 0 then idle - 1 else idleLimit) (fuel - 1)
      (received + 1) (sent + (if response == 0 then 0 else 1))

@[inline] def receiveUntil (respond : UInt32 → UInt32 → HardwareM UInt32)
    (finished : HardwareM Bool) (fuel : UInt32) : HardwareM UInt32 := do
  let idle := min fuel 4000000
  receiveLoop respond finished (← dmaBase) 0 idle idle (idle * 40) 0 0

@[inline] def receiveWith (respond : UInt32 → UInt32 → HardwareM UInt32)
    (index fuel received sent : UInt32) : HardwareM UInt32 := do
  receiveLoop respond (pure false) (← dmaBase) index fuel fuel fuel received sent

def receive (index fuel received sent : UInt32) : HardwareM UInt32 :=
  receiveWith (fun rx length => liftMemory <|
    Ethernet.respond rx length 0x66404000 2048 0xc0a84d03 0x18acc22d1c47)
    index fuel received sent

def stop : HardwareM UInt32 := do
  let _ ← writeReg 0 0
  let _ ← writeReg 0x2c 0xffff
  let _ ← barrier
  Cpu.flushMiu

end LeanBoot.Unsafe.Emac
