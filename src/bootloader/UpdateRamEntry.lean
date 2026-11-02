import EntryCheck
import Unsafe.Emac
import Unsafe.UpdateRam
import Update.Server

/- Returnable Ethernet updater experiment. Storage is exclusively RAM; the
   regular installed boot path never imports this module. -/
namespace LeanBoot.UpdateRamEntry
open Unsafe

private partial def emit (bytes : UInt64) (fuel : UInt32 := 1000000) : HardwareM UInt32 := do
  if bytes == 0 then return 0
  if fuel == 0 then return 21
  if (← uartStatus) &&& 32 == 0 then emit bytes (fuel - 1)
  else
    let _ ← uartWrite bytes.toUInt8
    emit (bytes >>> 8)

private partial def hex (value : UInt32) (digits : UInt32 := 8) : HardwareM UInt32 := do
  if digits == 0 then return 0
  let digit := (value >>> ((digits - 1) * 4)) &&& 15
  let _ ← emit (if digit < 10 then digit + 48 else digit + 87).toUInt64
  hex value (digits - 1)

private def selection (status : UInt32) : HardwareM UInt32 := do
  let a ← UpdateRam.version 0
  let b ← UpdateRam.version 1
  let chosen := Update.selectBank a b (a != 0) (b != 0)
  let version := if chosen == 0 then a else if chosen == 1 then b else 0
  let _ ← liftMemory <| store32 (UpdateRam.statistics + 4) status
  let _ ← liftMemory <| store32 (UpdateRam.statistics + 8) chosen
  let _ ← liftMemory <| store32 (UpdateRam.statistics + 12) version
  let _ ← emit 0x544f4f424e41454c -- LEANBOOT
  let _ ← emit 0x2050445520 -- UPD
  let _ ← hex status
  let _ ← emit 0x3d6b6e616220 -- bank=
  let _ ← hex chosen
  let _ ← emit 0x3d7620 -- v=
  let _ ← hex version
  emit 0x0a0d

@[export leanboot_ram_respond]
def respond (rx length : UInt32) : HardwareM UInt32 := do
  let tx : UInt32 := 0x66404000
  let n ← liftMemory <| Ethernet.respond rx length tx 2048 0xc0a84d03 0x18acc22d1c47
  if n == 0 then return 0
  if (← liftMemory <| Memory.read8 tx 2048 23) != 17 then return n
  let hi ← liftMemory <| Memory.read8 rx length 38
  let lo ← liftMemory <| Memory.read8 rx length 39
  let size := (hi <<< 8 ||| lo) - 8
  let out ← Update.Server.respond UpdateRam.policy UpdateRam.state UpdateRam.buffer
    (UpdateRam.capacity + 160) (rx + 42) size (tx + 42) UpdateRam.scratch UpdateRam.bankSize
  if out == 0 then return 0
  if (← liftMemory <| Memory.readLE32 (tx + 42) out 8) == 3 then
    let status := (← liftMemory <| Memory.readLE32 (tx + 42) out 12).toUInt32
    let _ ← selection status
  liftMemory <| Ethernet.finishUdp tx 2048 out

private def session : HardwareM UInt32 := do
  let cpsr ← readCpsr
  if !EntryCheck.mode cpsr then return 26
  let sp ← readSp
  let sctlr ← readSctlr
  if !EntryCheck.g13 cpsr sp sctlr then return 26
  -- Identity/cacheability of the code, private data/stack and DMA mappings.
  if (← liftMemory <| load32 0x297f0980) != 0x26000c0e ||
      (← liftMemory <| load32 0x297f0988) != 0x26200c0e ||
      (← liftMemory <| load32 0x297f098c) != 0x26300c0e ||
      (← liftMemory <| load32 0x297f0990) != 0x26400c0e ||
      (← liftMemory <| load32 0x297f1990) != 0x26400c06 then return 27
  if (← Emac.readReg 0) &&& 12 != 0 then return 29
  let initialized ← UpdateRam.seedBanks
  if initialized != 0 then return initialized
  let _ ← selection 0
  let _ ← Emac.powerOn
  let status ← Emac.phyRead 0 1
  if status == 0 || status >= 65535 then return 30
  let _ ← Cpu.cleanCaches
  let configured ← Emac.configure 0
  let _ ← Emac.report 12 configured
  let result ← if configured == 0 then do
      let _ ← emit 0x544f4f424e41454c
      let _ ← emit 0x204d415220 -- RAM
      let _ ← emit 0x0a0d5944414552 -- READY
      Emac.receiveWith respond 0 300000000 0 0
    else pure configured
  let stopped ← Emac.stop
  let _ ← Emac.report 32 result
  let _ ← Emac.report 36 stopped
  let _ ← Cpu.cleanCaches
  let _ ← emit 0x0a0d444e45204d4152 -- RAM END
  return if stopped != 0 then 20 else result

-- Reuse the existing DSL private-stack/return entry unchanged.
@[export leanboot_ethernet_main]
def main : HardwareM UInt32 := do
  if (← readCpsr) &&& 0x3f != 0x13 then return 26
  let saved ← maskInterrupts
  let result ← session
  let _ ← restoreInterrupts saved
  return result

end LeanBoot.UpdateRamEntry
