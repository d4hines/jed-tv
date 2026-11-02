import EntryCheck
import Unsafe.Emac

namespace LeanBoot.EthernetEntry

private partial def emit (bytes : UInt64) (fuel : UInt32 := 1000000) : Unsafe.HardwareM UInt32 := do
  if bytes == 0 then return 0
  if fuel == 0 then return 21
  if (← Unsafe.uartStatus) &&& 32 == 0 then emit bytes (fuel - 1)
  else
    let _ ← Unsafe.uartWrite bytes.toUInt8
    emit (bytes >>> 8)

private def session : Unsafe.HardwareM UInt32 := do
  let cpsr ← Unsafe.readCpsr
  if !EntryCheck.mode cpsr then return 26
  let sp ← Unsafe.readSp
  let sctlr ← Unsafe.readSctlr
  if !EntryCheck.g13 cpsr sp sctlr then return 26
  -- Recheck the observed mappings before touching DMA RAM. The bootstrap also
  -- owns and checks the physical reservation, helper hashes and exact target.
  if (← Unsafe.liftMemory <| Unsafe.load32 0x297f0990) != 0x26400c0e ||
      (← Unsafe.liftMemory <| Unsafe.load32 0x297f1990) != 0x26400c06 then return 27
  if (← Unsafe.liftMemory <| Unsafe.load32 0x26200000) != 0x45544831 then return 28
  let mode ← Unsafe.liftMemory <| Unsafe.load32 0x26200004
  if mode > 1 then return 28
  if (← Unsafe.Emac.readReg 0) &&& 12 != 0 then return 29
  let _ ← emit 0x544f4f424e41454c
  let _ ← emit 0x0a0d48544520 -- ETH
  let _ ← Unsafe.Emac.powerOn
  let id1 ← Unsafe.Emac.phyRead 0 2
  let id2 ← Unsafe.Emac.phyRead 0 3
  let _ ← Unsafe.Emac.report 0 id1
  let _ ← Unsafe.Emac.report 4 id2
  let status ← Unsafe.Emac.phyRead 0 1
  let _ ← Unsafe.Emac.report 8 status
  -- This integrated PHY reports zero ID words. BMSR supplies the presence and
  -- management-read check; target identity is established by the bootstrap.
  if id1 == 65536 || id2 == 65536 || status == 0 || status >= 65535 then return 30
  if mode == 0 then return 0
  -- Clear any old cache lines before using the uncached alias exclusively.
  let _ ← Cpu.cleanCaches
  let initialized ← Unsafe.Emac.configure 0
  let _ ← Unsafe.Emac.report 12 initialized
  let result ← if initialized == 0 then do
      let _ ← emit 0x594441455220 -- READY
      let _ ← emit 0x0a0d
      Unsafe.Emac.receive 0 100000000 0 0
    else pure initialized
  let stopped ← Unsafe.Emac.stop
  let _ ← Unsafe.Emac.report 32 result
  let _ ← Unsafe.Emac.report 36 stopped
  let _ ← emit 0x0a0d444e4520 -- END
  return if stopped != 0 then 20 else result

@[export leanboot_ethernet_main]
def main : Unsafe.HardwareM UInt32 := do
  let cpsr ← Unsafe.readCpsr
  if cpsr &&& 0x3f != 0x13 then return 26
  let saved ← Unsafe.maskInterrupts
  let result ← session
  let _ ← Unsafe.restoreInterrupts saved
  return result

end LeanBoot.EthernetEntry
