import Lean.Elab.Tactic.Omega
import BootJournal
import Memory

-- Expose unsigned arithmetic to omega; casts and additions retain their widths.
local macro "word_arith" : tactic => `(tactic|
  (simp only [UInt32.lt_iff_toNat_lt, UInt32.le_iff_toNat_le,
     UInt64.lt_iff_toNat_lt, UInt64.le_iff_toNat_le,
     ← UInt32.toNat_inj, ← UInt64.toNat_inj] at *
   simp only [UInt32.toNat_add, UInt64.toNat_add, UInt32.toNat_toUInt64,
     UInt32.toNat_ofNat, UInt64.toNat_ofNat] at *
   simp only [UInt32.toNat, UInt64.toNat, UInt32.toBitVec_ofNat,
     UInt64.toBitVec_ofNat, BitVec.toNat_ofNat] at *
   omega))

/- Proofs about the executable definitions, kept out of the runtime import
   graph. Bit-vector theorems quantify over every input word, including wrap. -/
namespace LeanBoot.BootPolicy

theorem valid_states (state : UInt32) : valid state = true ↔
    state = initial ∨ state = leanFirst ∨ state = leanSecond ∨ state = stock := by
  simp [valid, or_assoc]

/-- The named states preserve the journal's installed bit encoding. -/
theorem valid_matches_encoding (state : UInt32) :
    valid state = (state &&& 0xfffffff8 == 0 && state &&& 3 < 3 &&
      (state &&& 4 == 0 || state &&& 3 == 0)) := by
  apply Bool.eq_iff_iff.mpr
  rw [valid_states]
  constructor
  · intro h
    rcases h with rfl | rfl | rfl | rfl <;> decide
  · intro h
    simp only [Bool.and_eq_true, Bool.or_eq_true, beq_iff_eq, decide_eq_true_eq] at h
    have high := congrArg UInt32.toBitVec h.1.1
    have splitWord := @BitVec.and_or_distrib_left 32 state.toBitVec 0xfffffff8 7
    simp only [UInt32.toBitVec_and, UInt32.toBitVec_ofNat] at high
    have low : state.toBitVec = state.toBitVec &&& 7 := by
      have ones : (0xfffffff8 ||| 7 : BitVec 32) = BitVec.allOnes 32 := by decide
      rw [ones, BitVec.and_allOnes] at splitWord
      change state.toBitVec = state.toBitVec &&& (0xfffffff8#32) |||
        state.toBitVec &&& (7#32) at splitWord
      rw [high, BitVec.zero_or] at splitWord
      exact splitWord
    have small : state.toNat < 8 := by
      have := congrArg BitVec.toNat low
      simp only [BitVec.toNat_and] at this
      change state.toNat = state.toNat &&& (2^3 - 1) at this
      rw [Nat.and_two_pow_sub_one_eq_mod] at this
      omega
    have cases : state = 0 ∨ state = 1 ∨ state = 2 ∨ state = 3 ∨
        state = 4 ∨ state = 5 ∨ state = 6 ∨ state = 7 := by word_arith
    rcases cases with rfl | rfl | rfl | rfl | rfl | rfl | rfl | rfl <;>
      simp_all [initial, leanFirst, leanSecond, stock] <;> contradiction

/-- The readable reducer has exactly the previous word-level behavior. -/
theorem step_matches_encoding (state event : UInt32) :
    step state event =
      (if !(state &&& 0xfffffff8 == 0 && state &&& 3 < 3 &&
          (state &&& 4 == 0 || state &&& 3 == 0)) || event > 1 then 0x80000004
       else if event == 0 then state
       else if state &&& 3 == 2 then 4
       else (state &&& 3) + 1) := by
  rw [← valid_matches_encoding]
  by_cases h : valid state = true
  · rcases (valid_states state).mp h with rfl | rfl | rfl | rfl <;>
      simp [step, valid, initial, leanFirst, leanSecond, stock, rejected,
        show (1 &&& 3 : UInt32) = 1 from rfl,
        show (2 &&& 3 : UInt32) = 2 from rfl,
        show (4 &&& 3 : UInt32) = 0 from rfl]
  · have : valid state = false := Bool.eq_false_iff.mpr h
    simp [step, this, rejected]

theorem fallback_matches_encoding (state : UInt32) :
    fallback state = (!valid state || state &&& 4 != 0) := by
  by_cases h : valid state = true
  · rcases (valid_states state).mp h with rfl | rfl | rfl | rfl <;> decide
  · have : valid state = false := Bool.eq_false_iff.mpr h
    simp [fallback, this]

theorem invalid_state_rejected (state event : UInt32) (h : valid state = false) :
    step state event = rejected := by
  simp [step, h]

theorem invalid_event_rejected (state event : UInt32) (h : event > 1) :
    step state event = rejected := by
  simp [step, h]

theorem inspection_preserves_state (state : UInt32) (h : valid state = true) :
    step state 0 = state := by
  simp [step, h]

theorem start_preserves_validity (state : UInt32) (h : valid state = true) :
    valid (step state 1) = true := by
  rcases (valid_states state).mp h with rfl | rfl | rfl | rfl <;> decide

theorem accepted_start_valid (state : UInt32) (h : failed (step state 1) = false) :
    valid (step state 1) = true := by
  by_cases good : valid state = true
  · exact start_preserves_validity state good
  · have bad : valid state = false := Bool.eq_false_iff.mpr good
    rw [invalid_state_rejected state 1 bad] at h
    contradiction

theorem rejected_selects_stock : fallback rejected = true := by decide

theorem initial_cycle :
    step initial 1 = leanFirst ∧
    step leanFirst 1 = leanSecond ∧
    step leanSecond 1 = stock ∧
    step stock 1 = leanFirst := by decide

/-- Once started, three full starts return to the same point in the cycle. -/
theorem three_starts_repeat (state : UInt32)
    (h : valid state = true) (started : state ≠ initial) :
    step (step (step state 1) 1) 1 = state := by
  rcases (valid_states state).mp h with rfl | rfl | rfl | rfl
  · exact False.elim (started rfl)
  all_goals decide

end LeanBoot.BootPolicy

namespace LeanBoot.BootJournal

theorem pack_sequence (seq state : UInt32) : sequence (pack seq state) = seq := by
  apply UInt32.eq_of_toBitVec_eq
  simp only [sequence, pack, UInt64.toBitVec_toUInt32, UInt64.toBitVec_shiftRight,
    UInt64.toBitVec_or, UInt64.toBitVec_shiftLeft, UInt32.toBitVec_toUInt64]
  change (((seq.toBitVec.setWidth 64 <<< (32 : Nat)) |||
    state.toBitVec.setWidth 64) >>> (32 : Nat)).setWidth 32 = seq.toBitVec
  rw [← BitVec.setWidth_append_eq_shiftLeft_setWidth_or]
  simp [BitVec.setWidth_ushiftRight_eq_extractLsb]
  exact BitVec.extractLsb'_append_eq_left

theorem pack_state (seq state : UInt32) : (pack seq state).toUInt32 = state := by
  apply UInt32.eq_of_toBitVec_eq
  simp only [pack, UInt64.toBitVec_toUInt32, UInt64.toBitVec_or,
    UInt64.toBitVec_shiftLeft, UInt32.toBitVec_toUInt64]
  change ((seq.toBitVec.setWidth 64 <<< (32 : Nat)) |||
    state.toBitVec.setWidth 64).setWidth 32 = state.toBitVec
  rw [← BitVec.setWidth_append_eq_shiftLeft_setWidth_or]
  simp
  exact BitVec.setWidth_append_eq_right

theorem selects_only_valid_a (a b : UInt64) (goodA goodB : Bool)
    (h : select a b goodA goodB = 0) : goodA = true := by
  cases goodA <;> cases goodB <;> simp_all [select]

theorem selects_only_valid_b (a b : UInt64) (goodA goodB : Bool)
    (h : select a b goodA goodB = 1) : goodB = true := by
  cases goodA <;> cases goodB <;> simp_all [select]

theorem no_valid_record_rejected (a b : UInt64) : select a b false false = 2 := by
  simp [select]

theorem missing_records_reject_plan {α : Type} (a b : UInt64)
    (reject : UInt32 → α) (accept : Commit → α) :
    withNext a b false false reject accept = reject BootPolicy.rejected := by
  simp [withNext, select]

/-- Sequence adjacency, including 0xffffffff -> 0, selects the newer record. -/
theorem newer_b_selected (a b : UInt64) (h : sequence a + 1 = sequence b) :
    select a b true true = 1 := by
  have different : a ≠ b := by
    intro same
    subst b
    word_arith
  simp [select, different, h]

/-- Every accepted plan targets the other slot and contains a valid state.
    Rejected plans make no claim about a storage write or callback. -/
theorem plan_targets_inactive_slot (a b : UInt64) (goodA goodB : Bool) :
    withNext a b goodA goodB (fun _ => True) (fun plan =>
      plan.source < 2 ∧ plan.target < 2 ∧ plan.target ≠ plan.source ∧
      BootPolicy.valid plan.state = true) := by
  unfold withNext
  dsimp only
  split
  · trivial
  · rename_i bounded
    generalize oldEq : (if (select a b goodA goodB == 0) = true then a else b) = old
    split
    · trivial
    · rename_i accepted
      have good := BootPolicy.accepted_start_valid _ (Bool.eq_false_iff.mpr accepted)
      have slots : select a b goodA goodB = 0 ∨ select a b goodA goodB = 1 := by word_arith
      rcases slots with slot | slot <;> simp_all [Commit.target]

end LeanBoot.BootJournal

namespace LeanBoot.Memory

/-- Subtract-before-compare is exactly a widened, non-wrapping range check. -/
theorem contains_iff_widened (size offset count : UInt32) :
    contains size offset count = true ↔
      offset.toUInt64 + count.toUInt64 ≤ size.toUInt64 := by
  simp only [contains, Bool.and_eq_true, decide_eq_true_eq]
  constructor
  · rintro ⟨ho, hc⟩
    have sub := UInt32.toNat_sub_of_le size offset ho
    word_arith
  · intro h
    have ho : offset ≤ size := by word_arith
    have sub := UInt32.toNat_sub_of_le size offset ho
    exact ⟨ho, by word_arith⟩

/-- Every addressed byte stays in the supplied span without UInt32 wrap. -/
theorem checked_address_in_range (base size offset count index : UInt32)
    (region : addressable base size = true)
    (range : contains size offset count = true) (inside : index < count) :
    (base + offset + index).toUInt64 =
      base.toUInt64 + offset.toUInt64 + index.toUInt64 ∧
    (base + offset + index).toUInt64 < base.toUInt64 + size.toUInt64 := by
  have span : base.toNat + size.toNat ≤ 0xffffffff := by
    simp only [addressable, decide_eq_true_eq] at region
    have sub := UInt32.toNat_sub_of_le 0xffffffff base (by word_arith)
    word_arith
  have slice := (contains_iff_widened size offset count).mp range
  constructor <;> word_arith

/-- Rejection is the pure result for any memory interpreter: no read occurs. -/
theorem read8_rejects_out_of_bounds {m : Type → Type} [Monad m] [Reader m]
    (base size offset : UInt32) (h : contains size offset 1 = false) :
    read8 (m := m) base size offset = pure 256 := by
  simp [read8, h]

/-- Rejection is the pure result for any memory interpreter: no write occurs. -/
theorem write8_rejects_out_of_bounds {m : Type → Type} [Monad m] [Writer m]
    (base size offset value : UInt32) (h : contains size offset 1 = false) :
    write8 (m := m) base size offset value = pure 1 := by
  simp [write8, h]

end LeanBoot.Memory
