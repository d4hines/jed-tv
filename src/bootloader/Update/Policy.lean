import Init

/- Value-only decisions used by the installer, receiver and boot selector. -/
namespace LeanBoot.Update

abbrev headerSize : UInt32 := 160
abbrev signedSize : UInt32 := 96
abbrev sectorSize : UInt32 := 512
abbrev maxPayload : UInt32 := 16 * 1024 * 1024

structure Manifest where
  board : UInt32
  component : UInt32
  version : UInt32
  length : UInt32
  entry : UInt32
  deriving Repr, DecidableEq

structure Policy where
  board : UInt32
  component : UInt32
  minimumVersion : UInt32
  capacity : UInt32
  deriving Repr

/-- No wrap is permitted for release versions. The floor is supplied by the
    trusted caller, never by the update package. This is not secure storage. -/
@[inline] def compatible (p : Policy) (m : Manifest) : Bool :=
  m.board == p.board && m.component == p.component &&
  m.version >= p.minimumVersion && m.version != 0 &&
  m.length != 0 && m.length <= maxPayload && m.length <= p.capacity &&
  m.entry < m.length && m.entry &&& 3 == 0

/-- Signing binds the complete manifest; hashing binds its image bytes. -/
@[inline] def authenticated (p : Policy) (m : Manifest) (signature digest : Bool) : Bool :=
  signature && digest && compatible p m

/-- The active bank is a locally authenticated selection, not a wire field. -/
@[inline] def targetBank (active : UInt32) : UInt32 := active ^^^ 1

@[inline] def canInstall (p : Policy) (m : Manifest) (active currentVersion : UInt32)
    (signature digest : Bool) : Bool :=
  active < 2 && m.version > currentVersion && authenticated p m signature digest

/-- Tail callbacks let the freestanding specialization avoid allocating a plan. -/
@[inline] def withInstall {α : Type} (p : Policy) (m : Manifest)
    (active currentVersion : UInt32) (signature digest : Bool)
    (reject : α) (accept : UInt32 → α) : α :=
  if canInstall p m active currentVersion signature digest then accept (targetBank active)
  else reject

/-- Both candidates have already passed full signature/hash verification.
    Equal versions are ambiguous and fail closed. 2 means no selection. -/
@[inline] def selectBank (versionA versionB : UInt32) (validA validB : Bool) : UInt32 :=
  if validA && validB then
    if versionA > versionB then 0 else if versionB > versionA then 1 else 2
  else if validA then 0 else if validB then 1 else 2

/-- A full final-sector write is safe only after staged data was checked. -/
@[inline] def canCommit (admitted readback flushed : Bool) : Bool :=
  admitted && readback && flushed

end LeanBoot.Update
