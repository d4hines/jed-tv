import NetworkBoot.Runtime
import Update.CryptoBounds
import Proofs

namespace LeanBoot.NetworkBoot

theorem entry_within_image (length : ByteCount) (offset : EntryOffset)
    (ok : entryFits length offset = true) :
    offset.value.toNat + 4 <= length.value.toNat := by
  have span : Memory.contains length.value offset.value 4 = true := by
    simp only [entryFits, Bool.and_eq_true] at ok
    exact ok.2
  simp only [Memory.contains, Bool.and_eq_true, decide_eq_true_eq] at span
  have sub := UInt32.toNat_sub_of_le length.value offset.value span.1
  have count := span.2
  change 4 <= (length.value - offset.value).toNat at count
  omega

theorem entry_within_reservation (length : ByteCount) (offset : EntryOffset)
    (ok : entryFits length offset = true) : length.value <= capacity.value := by
  simp only [entryFits, Bool.and_eq_true, decide_eq_true_eq] at ok
  exact ok.1.1

theorem entry_aligned (length : ByteCount) (offset : EntryOffset)
    (ok : entryFits length offset = true) : offset.value &&& 3 = 0 := by
  simp only [entryFits, Bool.and_eq_true, beq_iff_eq] at ok
  exact ok.1.2

theorem executable_address_within_image (executable : Executable) :
    image.value.toNat <= executable.address.value.toNat ∧
    executable.address.value.toNat + 4 <= image.value.toNat + executable.length.value.toNat := by
  have entry := entry_within_image executable.length executable.entry executable.fits
  have capacity := entry_within_reservation executable.length executable.entry executable.fits
  change executable.length.value.toNat <= 0x100000 at capacity
  simp only [Executable.address, image, UInt32.toNat_add]
  change 0x40600000 <= (0x40600000 + executable.entry.value.toNat) % 4294967296 ∧
    (0x40600000 + executable.entry.value.toNat) % 4294967296 + 4 <=
      0x40600000 + executable.length.value.toNat
  omega

theorem busy_leaves_resources_untouched {m : Type → Type} [Monad m] [LawfulMonad m]
    [Memory.Reader m] [Update.Crypto m] (effects : Effects m) (tags : BootTags) (fuel : PollBudget)
    (busy : effects.busy = pure true) : run effects tags fuel = pure ⟨29⟩ := by
  simp [run, busy]

end LeanBoot.NetworkBoot

namespace LeanBoot.Update.CryptoBounds

theorem rejected_signature_never_calls {m : Type → Type} [Monad m]
    (verify : CpuAddress → m Bool) (header : CpuAddress)
    (rejected : signatureInput header = false) :
    signature verify header = pure false := by
  simp [signature, rejected]

theorem rejected_digest_never_calls {m : Type → Type} [Monad m]
    (verify : CpuAddress → ByteCount → CpuAddress → m Bool)
    (input : CpuAddress) (length : ByteCount) (expected : CpuAddress)
    (rejected : digestInput input length expected = false) :
    digest verify input length expected = pure false := by
  simp [digest, rejected]

end LeanBoot.Update.CryptoBounds
