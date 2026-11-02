import Update.Manifest

namespace LeanBoot.Update
open Memory
variable {m : Type → Type} [Monad m] [Reader m] [Writer m] [Crypto m] [Storage m]

-- Results: 0 installed, 1 invalid input/workspace, 2 authentication/policy,
-- 3 invalidation, 4 image write, 5 readback/hash, 6 commit/durability/readback.
-- An error at commit can leave a valid new bank; callers must rescan it.

@[specialize] private partial def writeImage
    (source scratch bank length offset : UInt32) : m Bool := do
  if offset >= length then return true
  let count := min sectorSize (length - offset)
  if (← fill scratch sectorSize 0 sectorSize 0) != 0 ||
      (← copy scratch sectorSize 0 source length offset count) != 0 then return false
  if !(← Storage.write bank (1 + offset / sectorSize) scratch) then return false
  writeImage source scratch bank length (offset + sectorSize)

@[specialize] private partial def readImage
    (scratch capacity bank length offset : UInt32) : m Bool := do
  if offset >= length then return true
  if !contains capacity offset sectorSize then return false
  if !(← Storage.read bank (1 + offset / sectorSize) (scratch + offset)) then return false
  readImage scratch capacity bank length (offset + sectorSize)

@[specialize] private partial def equalBytes (a b count : UInt32) : m Bool := do
  if count == 0 then return true
  if (← Reader.byte a) != (← Reader.byte b) then return false
  equalBytes (a + 1) (b + 1) (count - 1)

/-- Run the install protocol for a parsed header. Authentication is still
    performed here, before any write; this is not an unchecked flash API. -/
@[inline] def installManifest (policy : Policy) (active currentVersion : UInt32)
    (source size scratch capacity : UInt32) (manifest : Manifest) : m UInt32 := do
  let rounded := (manifest.length + sectorSize - 1) / sectorSize * sectorSize
  let bank := targetBank active
  let result ← Flow.run .install fun stage => do
    match stage with
    | .policy =>
      if !(← authenticationStep policy source size manifest stage) ||
          !canInstall policy manifest active currentVersion true true then return ⟨2⟩
      return if capacity < rounded then ⟨1⟩ else .ok
    | .signature | .sourceDigest =>
      return if (← authenticationStep policy source size manifest stage) then .ok else ⟨2⟩
    | .invalidate =>
      if (← fill scratch capacity 0 sectorSize 0) != 0 then return ⟨3⟩
      return if (← Storage.write bank 0 scratch) then .ok else ⟨3⟩
    | .flushInvalid => return if (← Storage.flush) then .ok else ⟨3⟩
    | .writeImage =>
      return if (← writeImage (source + headerSize) scratch bank manifest.length 0) then .ok else ⟨4⟩
    | .flushImage => return if (← Storage.flush) then .ok else ⟨4⟩
    | .readImage =>
      return if (← readImage scratch capacity bank manifest.length 0) then .ok else ⟨5⟩
    | .readbackDigest =>
      return if (← Crypto.digest ⟨scratch⟩ ⟨manifest.length⟩ ⟨source + 40⟩) then .ok else ⟨5⟩
    | .flushReadback => return if (← Storage.flush) then .ok else ⟨5⟩
    | .writeHeader =>
      if (← fill scratch capacity 0 sectorSize 0) != 0 ||
          (← copy scratch capacity 0 source size 0 headerSize) != 0 then return ⟨6⟩
      return if (← Storage.write bank 0 scratch) then .ok else ⟨6⟩
    | .flushHeader => return if (← Storage.flush) then .ok else ⟨6⟩
    | .readHeader =>
      if (← fill scratch capacity 0 sectorSize 0) != 0 then return ⟨6⟩
      return if (← Storage.read bank 0 scratch) then .ok else ⟨6⟩
    | .checkHeader => return if (← equalBytes source scratch headerSize) then .ok else ⟨6⟩
    | _ => pure ⟨45⟩
  return result.code

/-- The authenticated active bank is never a write target. Invalidate the
    other header durably, write its image, verify storage readback, then write
    the signed header last. Input/scratch ownership excludes DMA mutation. -/
@[inline] def install (policy : Policy) (active currentVersion : UInt32)
    (source size scratch capacity : UInt32) : m UInt32 := do
  if !addressable source size || !addressable scratch capacity ||
      capacity < sectorSize ||
      (source < scratch + capacity && scratch < source + size) then return 1
  withManifest source size (pure 2) fun manifest => do
    installManifest policy active currentVersion source size scratch capacity manifest

/-- Boot must re-authenticate the stored bytes. A signed header alone is not
    sufficient. The workspace contains header sector followed by image
    on success; the callback supplies a verified version/entry to the caller. -/
@[inline] def inspectBank {α : Type} (policy : Policy) (bank scratch capacity : UInt32)
    (reject ioError : m α) (accept : UInt32 → UInt32 → UInt32 → m α) : m α := do
  if bank >= 2 || capacity < sectorSize || !addressable scratch capacity then return ← reject
  if !(← Storage.read bank 0 scratch) then return ← ioError
  withManifest scratch headerSize reject fun manifest => do
    if !compatible policy manifest ||
        capacity < sectorSize + (manifest.length + sectorSize - 1) / sectorSize * sectorSize then
      return ← reject
    if !(← Crypto.signature ⟨scratch⟩) then return ← reject
    if !(← readImage (scratch + sectorSize) (capacity - sectorSize) bank manifest.length 0) then
      return ← ioError
    if !(← Crypto.digest ⟨scratch + sectorSize⟩ ⟨manifest.length⟩ ⟨scratch + 40⟩) then return ← reject
    accept manifest.version manifest.length manifest.entry

@[inline] def withBank {α : Type} (policy : Policy) (bank scratch capacity : UInt32)
    (reject : m α) (accept : UInt32 → UInt32 → UInt32 → m α) : m α :=
  inspectBank policy bank scratch capacity reject reject accept

end LeanBoot.Update

namespace LeanBoot.Update
open Memory
variable {m : Type → Type} [Monad m] [Reader m] [Writer m] [Crypto m] [Storage m]

/-- Ordinary updates require an existing authenticated bootable bank. Factory
    provisioning is deliberately not an unsigned network command. Determine
    the active bank locally; the sender cannot choose which bank to overwrite. -/
@[inline] def installNext (policy : Policy)
    (source size scratch capacity : UInt32) : m UInt32 := do
  if !addressable source size || !addressable scratch capacity ||
      (source < scratch + capacity && scratch < source + size) then return 1
  let a ← withBank policy 0 scratch capacity (pure 0) (fun version _ _ => pure version)
  let b ← withBank policy 1 scratch capacity (pure 0) (fun version _ _ => pure version)
  let active := selectBank a b (a != 0) (b != 0)
  if active > 1 then return 7
  install policy active (if active == 0 then a else b) source size scratch capacity

end LeanBoot.Update
