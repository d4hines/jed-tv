import Update.Flash
import Flow.Proofs

namespace LeanBoot.Update.Flash

theorem invalid_sector_never_invokes {α : Type} (bank sector : UInt32)
    (reject : α) (accept : SectorAddress → α)
    (invalid : sectorAllowed bank sector = false) :
    withSector bank sector reject accept = reject := by
  simp [withSector, invalid]

theorem sector_within_banks (bank sector : UInt32)
    (allowed : sectorAllowed bank sector = true) :
    firstSector.toNat <= (sectorAddress bank sector).value.toNat ∧
    (sectorAddress bank sector).value.toNat < endSector.toNat := by
  simp only [sectorAllowed, Bool.and_eq_true, decide_eq_true_eq] at allowed
  have b : bank.toNat < 2 := allowed.1
  have s : sector.toNat < 32768 := allowed.2
  have cases : bank = 0 ∨ bank = 1 := by
    have : bank.toNat = 0 ∨ bank.toNat = 1 := by omega
    rcases this with z | o
    · left; exact UInt32.toNat_inj.mp z
    · right; exact UInt32.toNat_inj.mp o
  rcases cases with rfl | rfl <;>
    simp only [sectorAddress, firstSector, bankSectors, endSector, UInt32.toNat_add,
      UInt32.toNat_mul] <;>
    simp only [UInt32.toNat_ofNat] <;> omega

theorem sector_excludes_journal (bank sector : UInt32)
    (allowed : sectorAllowed bank sector = true) :
    (sectorAddress bank sector).value ≠ 0x3217fe ∧
    (sectorAddress bank sector).value ≠ 0x3217ff := by
  have bounds := sector_within_banks bank sector allowed
  change _ ∧ (sectorAddress bank sector).value.toNat < 0x319000 at bounds
  constructor <;> intro equal <;> rw [equal] at bounds <;> contradiction

theorem first_storage_write_requires_authentication (events : List Flow.Completion)
    (reached : Flow.replay .install (Flow.initial .install) events = .invalidate) :
    Flow.Seen .signature events ∧ Flow.Seen .sourceDigest events :=
  ⟨Flow.reached_requires .install events .invalidate .signature reached (by decide),
   Flow.reached_requires .install events .invalidate .sourceDigest reached (by decide)⟩

theorem commit_requires_authentication_and_readback (events : List Flow.Completion)
    (reached : Flow.replay .install (Flow.initial .install) events = .writeHeader) :
    Flow.Seen .signature events ∧ Flow.Seen .readbackDigest events ∧ Flow.Seen .flushReadback events :=
  Flow.header_commit_requires_readback_and_flush events reached

end LeanBoot.Update.Flash

namespace LeanBoot.Update
variable {m : Type → Type} [Monad m] [LawfulMonad m]
  [Memory.Reader m] [Memory.Writer m] [Crypto m] [Storage m]

/-- Equality of effectful programs, not just a predicate on an abstract
    trace: a rejected verifier result makes the real installer return 2.
    No storage or memory writer is invoked, regardless of its implementation. -/
theorem rejected_signature_performs_no_writes (policy : Policy)
    (active current source size scratch capacity : UInt32) (manifest : Manifest)
    (space : ¬ capacity < (manifest.length + sectorSize - 1) / sectorSize * sectorSize)
    (rejected : Crypto.signature ⟨source⟩ = (pure false : m Bool)) :
    installManifest policy active current source size scratch capacity manifest =
      (pure 2 : m UInt32) := by
  unfold installManifest
  dsimp only
  unfold Flow.run
  rw [Flow.runFrom]
  dsimp [Flow.initial, authenticationStep]
  simp only [pure_bind, space, ↓reduceIte]
  split
  · simp [Flow.step, Status.succeeded]
  · simp only [pure_bind]
    dsimp [Flow.step, Flow.next, Status.succeeded, Status.ok]
    rw [Flow.runFrom]
    simp [rejected, Flow.step, Status.succeeded]

/-- Includes insufficient workspace: every outcome is a pure error, for
    arbitrary implementations of both writers and storage. -/
theorem rejected_signature_only_rejects (policy : Policy)
    (active current source size scratch capacity : UInt32) (manifest : Manifest)
    (rejected : Crypto.signature ⟨source⟩ = (pure false : m Bool)) :
    ∃ code : UInt32, (code = 1 ∨ code = 2) ∧
      installManifest policy active current source size scratch capacity manifest =
        (pure code : m UInt32) := by
  by_cases space : capacity < (manifest.length + sectorSize - 1) / sectorSize * sectorSize
  · unfold installManifest
    dsimp only
    unfold Flow.run
    rw [Flow.runFrom]
    dsimp [Flow.initial, authenticationStep]
    simp only [pure_bind, space, ↓reduceIte]
    split
    · exact ⟨2, Or.inr rfl, by simp [Flow.step, Status.succeeded]⟩
    · exact ⟨1, Or.inl rfl, by simp [Flow.step, Status.succeeded]⟩
  · exact ⟨2, Or.inr rfl,
      rejected_signature_performs_no_writes policy active current source size scratch capacity manifest space rejected⟩

end LeanBoot.Update

namespace LeanBoot.Update.Flash

/-- Exact arithmetic, including absence of UInt32 wrap, rather than just a
    bound on the union of the two banks. -/
theorem sector_exact (bank sector : UInt32)
    (allowed : sectorAllowed bank sector = true) :
    (sectorAddress bank sector).value.toNat = 0x309000 + bank.toNat * 0x8000 + sector.toNat := by
  simp only [sectorAllowed, Bool.and_eq_true, decide_eq_true_eq] at allowed
  have b : bank.toNat < 2 := allowed.1
  have s : sector.toNat < 32768 := allowed.2
  have cases : bank = 0 ∨ bank = 1 := by
    have : bank.toNat = 0 ∨ bank.toNat = 1 := by omega
    rcases this with z | o
    · exact Or.inl (UInt32.toNat_inj.mp z)
    · exact Or.inr (UInt32.toNat_inj.mp o)
  rcases cases with rfl | rfl <;>
    simp only [sectorAddress, firstSector, bankSectors, UInt32.toNat_add,
      UInt32.toNat_mul, UInt32.toNat_ofNat] <;> omega

theorem different_banks_disjoint (a b x y : UInt32)
    (ax : sectorAllowed a x = true) (byAllowed : sectorAllowed b y = true) (different : a ≠ b) :
    (sectorAddress a x).value ≠ (sectorAddress b y).value := by
  intro equal
  have e := congrArg UInt32.toNat equal
  rw [sector_exact a x ax, sector_exact b y byAllowed] at e
  simp only [sectorAllowed, Bool.and_eq_true, decide_eq_true_eq] at ax byAllowed
  have xBound : x.toNat < 32768 := ax.2
  have yBound : y.toNat < 32768 := byAllowed.2
  have distinct : a.toNat ≠ b.toNat := fun h => different (UInt32.toNat_inj.mp h)
  omega

end LeanBoot.Update.Flash

namespace LeanBoot.Update
variable {m : Type → Type} [Monad m] [LawfulMonad m]
  [Memory.Reader m] [Memory.Writer m] [Crypto m] [Storage m]

/-- Even a valid signature cannot authorize writing different payload bytes.
    This unfolds the actual installer's first three effectful stages. -/
theorem rejected_payload_performs_no_writes (policy : Policy)
    (active current source size scratch capacity : UInt32) (manifest : Manifest)
    (space : ¬ capacity < (manifest.length + sectorSize - 1) / sectorSize * sectorSize)
    (signature : Crypto.signature ⟨source⟩ = (pure true : m Bool))
    (digest : Crypto.digest ⟨source + headerSize⟩ ⟨manifest.length⟩ ⟨source + 40⟩ =
      (pure false : m Bool)) :
    installManifest policy active current source size scratch capacity manifest =
      (pure 2 : m UInt32) := by
  unfold installManifest
  dsimp only
  unfold Flow.run
  rw [Flow.runFrom]
  dsimp [Flow.initial, authenticationStep]
  simp only [pure_bind, space, ↓reduceIte]
  split
  · simp [Flow.step, Status.succeeded]
  · simp only [pure_bind]
    dsimp [Flow.step, Flow.next, Status.succeeded, Status.ok]
    rw [Flow.runFrom]
    simp only [signature, pure_bind, ↓reduceIte]
    dsimp [Flow.step, Flow.next, Status.succeeded, Status.ok]
    rw [Flow.runFrom]
    simp [digest, Flow.step, Status.succeeded]

end LeanBoot.Update
