import Update.Policy
import Types

/- Exact G13 software-image banks inside upgrade. The first MiB, final
   journal sectors, boot partitions, calibration and partition tables are
   outside this interface. Provisioning must reserve these extents first. -/
namespace LeanBoot.Update.Flash

structure SectorAddress where
  value : UInt32
  deriving Repr, DecidableEq

def firstSector : UInt32 := 0x309000
def bankSectors : UInt32 := 0x8000
def endSector : UInt32 := 0x319000
def capacity : UInt32 := 0x1000000 - 512
def component : UInt32 := 2
def source : CpuAddress := ⟨0x21200000⟩
def scratch : CpuAddress := ⟨0x26500000⟩
def scratchBytes : UInt32 := 0x1000000

@[inline] def policy : Policy := {
  board := 0x00130001, component, minimumVersion := 1, capacity }

@[inline] def sectorAllowed (bank sector : UInt32) : Bool :=
  bank < 2 && sector < bankSectors

@[inline] def sectorAddress (bank sector : UInt32) : SectorAddress :=
  ⟨firstSector + bank * bankSectors + sector⟩

/-- Only the fixed bank number and relative sector can cross this boundary.
    The packet cannot supply a physical address or a new partition map. -/
@[inline] def withSector {α : Type} (bank sector : UInt32)
    (reject : α) (accept : SectorAddress → α) : α :=
  if sectorAllowed bank sector then accept (sectorAddress bank sector) else reject

@[inline] def pointerAllowed (pointer : UInt32) : Bool :=
  pointer >= scratch.value && pointer <= scratch.value + scratchBytes - sectorSize &&
    pointer &&& 3 == 0

end LeanBoot.Update.Flash
