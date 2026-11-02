import Update.Receiver.Plan
import Lean

namespace LeanBoot.Update.Receiver

theorem sealed_never_appends (s : State) (p : Packet) (capacity : ByteCount)
    (sealed : s.phase = .sealed) : (step s p capacity).action ≠ .append := by
  unfold step
  repeat' split
  all_goals simp_all [reply, malformed]

theorem retransmission_never_appends (s : State) (p : Packet) (capacity : ByteCount)
    (old : p.offset.value < s.received.value) : (step s p capacity).action ≠ .append := by
  unfold step
  repeat' split
  all_goals simp_all [reply, malformed]

theorem seal_requires_complete (s : State) (p : Packet) (capacity : ByteCount)
    (sealedResult : (step s p capacity).action = .seal) : s.received = s.total := by
  unfold step at sealedResult
  repeat' split at sealedResult
  all_goals simp_all [reply, malformed]

theorem wrong_session_cannot_write (s : State) (p : Packet) (capacity : ByteCount)
    (active : s.phase ≠ .idle) (different : p.session ≠ s.session) :
    (step s p capacity).next = s := by
  unfold step
  repeat' split
  all_goals simp_all [reply, malformed]

theorem rejection_preserves_state (s : State) (p : Packet) (capacity : ByteCount)
    (failed : (step s p capacity).status.code ≠ 0) : (step s p capacity).next = s := by
  unfold step at *
  repeat' split at *
  all_goals simp_all [reply, malformed, Status.ok]

theorem append_has_checked_chunk (s : State) (p : Packet) (capacity : ByteCount)
    (append : (step s p capacity).action = .append) :
    chunkFits s.total.value s.received.value p.offset.value p.count.value = true ∧
    ¬p.offset.value < s.received.value := by
  unfold step at append
  repeat' split at append
  all_goals simp_all [reply, malformed]

theorem append_cannot_exceed_total (s : State) (p : Packet) (capacity : ByteCount)
    (append : (step s p capacity).action = .append) :
    s.received.value.toNat + p.count.value.toNat <= s.total.value.toNat := by
  have checked := append_has_checked_chunk s p capacity append
  have fits := checked.1
  simp only [chunkFits, Memory.contains, Bool.and_eq_true, Bool.or_eq_true,
    beq_iff_eq, decide_eq_true_eq] at fits
  have offset : p.offset.value = s.received.value := by
    have le := fits.1.1.1.1.2
    have ge := checked.2
    apply UInt32.toNat_inj.mp
    change p.offset.value.toNat <= s.received.value.toNat at le
    change ¬ p.offset.value.toNat < s.received.value.toNat at ge
    omega
  rw [offset] at fits
  have ordered : s.received.value.toNat <= s.total.value.toNat := fits.1.1.1.1.1
  have bounded := fits.1.2.2
  have sub := UInt32.toNat_sub_of_le s.total.value s.received.value fits.1.1.1.1.1
  change p.count.value.toNat <= (s.total.value - s.received.value).toNat at bounded
  omega

theorem append_updates_exact_bytes (s : State) (p : Packet) (capacity : ByteCount)
    (append : (step s p capacity).action = .append) :
    (step s p capacity).next.received.value.toNat =
      s.received.value.toNat + p.count.value.toNat := by
  have bound := append_cannot_exceed_total s p capacity append
  have totalBound := s.total.value.toNat_lt
  have value : (step s p capacity).next.received.value = s.received.value + p.count.value := by
    unfold step at *
    repeat' split at *
    all_goals simp_all [reply, malformed]
  rw [value, UInt32.toNat_add]
  exact Nat.mod_eq_of_lt (by omega)

end LeanBoot.Update.Receiver
