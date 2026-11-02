import Update.Policy
import Update.Receiver
import Lean.Elab.Tactic.Omega

/- These prove the actual executable decision functions. Cryptographic
   correctness, storage semantics, driver sequencing and compiler correctness
   are separate obligations; Bool inputs are observations, not axioms. -/
namespace LeanBoot.Update

theorem admitted_requires_signature (p : Policy) (h : Manifest) (a v : UInt32)
    (s d : Bool) (ok : canInstall p h a v s d = true) : s = true := by
  simp [canInstall, authenticated] at ok
  exact ok.2.1.1

theorem admitted_requires_digest (p : Policy) (h : Manifest) (a v : UInt32)
    (s d : Bool) (ok : canInstall p h a v s d = true) : d = true := by
  simp [canInstall, authenticated] at ok
  exact ok.2.1.2

theorem admitted_requires_compatibility (p : Policy) (h : Manifest) (a v : UInt32)
    (s d : Bool) (ok : canInstall p h a v s d = true) : compatible p h = true := by
  simp [canInstall, authenticated] at ok
  exact ok.2.2

theorem admitted_advances_version (p : Policy) (h : Manifest) (a v : UInt32)
    (s d : Bool) (ok : canInstall p h a v s d = true) : v < h.version := by
  simp [canInstall] at ok
  exact ok.1.2

theorem unsigned_never_invokes_install {α : Type} (p : Policy) (h : Manifest)
    (a v : UInt32) (d : Bool) (reject : α) (accept : UInt32 → α) :
    withInstall p h a v false d reject accept = reject := by
  simp [withInstall, canInstall, authenticated]

theorem bad_digest_never_invokes_install {α : Type} (p : Policy) (h : Manifest)
    (a v : UInt32) (s : Bool) (reject : α) (accept : UInt32 → α) :
    withInstall p h a v s false reject accept = reject := by
  simp [withInstall, canInstall, authenticated]

theorem inactive_target (a : UInt32) (valid : a < 2) :
    targetBank a < 2 ∧ targetBank a ≠ a := by
  have cases : a = 0 ∨ a = 1 := by
    change a.toNat < 2 at valid
    have h := a.toNat_lt
    have : a.toNat = 0 ∨ a.toNat = 1 := by omega
    rcases this with z | o
    · left; exact UInt32.toNat_inj.mp z
    · right; exact UInt32.toNat_inj.mp o
  rcases cases with rfl | rfl <;> decide

theorem commit_requires_readback (a r f : Bool) (ok : canCommit a r f = true) :
    a = true ∧ r = true ∧ f = true := by
  simpa [canCommit, and_assoc] using ok

theorem select_a_authenticated (a b : UInt32) (va vb : Bool)
    (selected : selectBank a b va vb = 0) : va = true := by
  cases va <;> cases vb <;> simp_all [selectBank] <;> split at selected <;> simp_all

theorem select_b_authenticated (a b : UInt32) (va vb : Bool)
    (selected : selectBank a b va vb = 1) : vb = true := by
  cases va <;> cases vb <;> simp_all [selectBank] <;> split at selected <;> simp_all

theorem no_authenticated_bank_no_boot (a b : UInt32) :
    selectBank a b false false = 2 := by simp [selectBank]

theorem equal_versions_reject (a : UInt32) : selectBank a a true true = 2 := by
  simp [selectBank]

end LeanBoot.Update
