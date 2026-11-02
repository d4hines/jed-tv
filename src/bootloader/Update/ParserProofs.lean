import Update.Manifest
import Ethernet.Descriptor
import Update.Receiver

namespace LeanBoot.Update
variable {m : Type → Type} [Monad m] [Memory.Reader m]

/-- Rejecting an invalid span is equality with the rejection action: no read,
    hash, signature check, write, or acceptance callback precedes it. -/
theorem short_manifest_rejects_without_reads {α : Type} (base size : UInt32)
    (reject : m α) (accept : Manifest → m α) (short : size < headerSize) :
    withManifest base size reject accept = reject := by
  simp [withManifest, short]

theorem wrapping_manifest_rejects_without_reads {α : Type} (base size : UInt32)
    (reject : m α) (accept : Manifest → m α) (wrap : Memory.addressable base size = false) :
    withManifest base size reject accept = reject := by
  simp [withManifest, wrap]

variable [LawfulMonad m]

theorem wrong_magic_never_accepts {α : Type} (base size : UInt32)
    (reject : m α) (accept : Manifest → m α) (word : UInt64)
    (read : Memory.readLE32 base size 0 = (pure word : m UInt64))
    (wrong : word ≠ 0x4f4f424c) : withManifest base size reject accept = reject := by
  unfold withManifest
  split
  · simp
  · simp [read, wrong]

theorem unsupported_format_never_accepts {α : Type} (base size : UInt32)
    (reject : m α) (accept : Manifest → m α) (format : UInt64)
    (magicLo : Memory.readLE32 base size 0 = (pure 0x4f4f424c : m UInt64))
    (magicHi : Memory.readLE32 base size 4 = (pure 0x31505554 : m UInt64))
    (read : Memory.readLE32 base size 8 = (pure format : m UInt64))
    (wrong : format ≠ 1) : withManifest base size reject accept = reject := by
  unfold withManifest
  split
  · simp
  · simp [magicLo, magicHi, read, wrong]

variable [Memory.Writer m]

omit [LawfulMonad m] in
theorem short_packet_rejects_without_effects (state buffer capacity packet size : UInt32)
    (short : size < 20) : Receiver.receive state buffer capacity packet size = (pure 0x100000000 : m UInt64) := by
  simp [Receiver.receive, short]

omit [LawfulMonad m] in
theorem oversized_packet_rejects_without_effects (state buffer capacity packet size : UInt32)
    (large : size > 1044) : Receiver.receive state buffer capacity packet size = (pure 0x100000000 : m UInt64) := by
  simp [Receiver.receive, large]

end LeanBoot.Update

namespace LeanBoot.Ethernet.Descriptor

theorem rejects_unknown_slot (index word : UInt32) (invalid : index >= 4) :
    validAddress index word = false := by
  have outside : ¬index < 4 := by
    change ¬index.toNat < 4
    change 4 <= index.toNat at invalid
    omega
  simp [validAddress, outside]

theorem payload_is_bounded (word : UInt32) :
    withPayload word True (fun n => n.value.toNat <= 1514) := by
  unfold withPayload
  dsimp only
  split
  · rename_i h
    simp only [Bool.and_eq_true, decide_eq_true_eq] at h
    have sub := UInt32.toNat_sub_of_le (word &&& 2047) 4 h.1
    change ((word &&& 2047) - 4).toNat = (word &&& 2047).toNat - 4 at sub
    have hi := h.2
    change (word &&& 2047).toNat <= 1518 at hi
    omega
  · trivial

end LeanBoot.Ethernet.Descriptor
