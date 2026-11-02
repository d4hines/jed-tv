import Types
import Update.Policy
import Memory
import Update.FlashPlan

namespace LeanBoot.NetworkBoot

def state : CpuAddress := ⟨0x40300000⟩
def buffer : CpuAddress := Update.Flash.source
def workspace : CpuAddress := ⟨0x40310000⟩
def image : CpuAddress := ⟨0x40600000⟩
def capacity : ByteCount := ⟨0x100000⟩
def context : CpuAddress := ⟨0x40301000⟩
def expectedDigest : CpuAddress := ⟨buffer.value + 40⟩

@[inline] def policy : Update.Policy := {
  board := 0x00130001, component := 1, minimumVersion := 1, capacity := capacity.value }

/-- The first complete ARM instruction must lie in the authenticated image,
    not just somewhere in the larger executable reservation. -/
@[inline] def entryFits (length : ByteCount) (offset : EntryOffset) : Bool :=
  length.value <= capacity.value && offset.value &&& 3 == 0 &&
    Memory.contains length.value offset.value 4

/-- Arithmetic evidence only: authentication and exclusive memory ownership
    are separate requirements. The proof erases before code generation. -/
structure Executable where
  length : ByteCount
  entry : EntryOffset
  version : ReleaseVersion
  fits : entryFits length entry = true

/-- Decode scalar wire fields once, and admit a plan only after checking it. -/
@[inline] def withExecutable {α : Type} (manifest : Update.Manifest)
    (reject : α) (accept : Executable → α) : α :=
  let length : ByteCount := ⟨manifest.length⟩
  let entry : EntryOffset := ⟨manifest.entry⟩
  if h : entryFits length entry then
    accept { length, entry, version := ⟨manifest.version⟩, fits := h }
  else reject

@[inline] def Executable.address (executable : Executable) : CpuAddress :=
  ⟨image.value + executable.entry.value⟩

end LeanBoot.NetworkBoot
