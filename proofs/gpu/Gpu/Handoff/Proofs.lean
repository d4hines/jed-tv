import Gpu.Handoff.Spec

/-!
# Renderer-to-presenter frame handoff: proofs

Main results:
* `no_torn_frames` (D1): under any schedule, if the reader accepts, the
  fields it read are exactly the fields of one frame the writer
  completed, namely script entry `frameIndex c1`.
* `freshness` (D2): that frame is the newest one complete at the moment
  of the reader's first counter read, and `c1` is twice the number of
  frames complete at that moment.
* `counter_identifies_frame` (D2): two accepted counter values name the
  same frame only if they are equal.
* Negative examples on a concrete torn schedule: the real check rejects
  it; the check without the second counter read accepts it.

The proof is by an invariant (`Inv`) that holds in the initial state and
is kept by every writer step and every reader step.
-/

namespace Gpu.Handoff

/-- `1` while a write is in progress, `0` otherwise. -/
def phaseBit : WriterPhase → Nat
  | .idle => 0
  | .writing _ => 1

/-- The frame the writer is currently writing (script entry `done`). -/
def currentFrame (w : Writer) : Frame := w.frames.getD w.done zeroFrame

/-- The newest completed frame (script entry `done - 1`; meaningful when `done > 0`). -/
def lastFrame (w : Writer) : Frame := w.frames.getD (w.done - 1) zeroFrame

/-- The invariant.  In plain words:
* the counter is twice the number of completed frames, plus one while a
  write is in progress;
* the writer never runs past its script;
* while idle (and at least one frame done) the record holds the newest
  completed frame; while writing field `i`, fields below `i` already
  hold the new frame;
* while the reader is reading: `c1` is at most the current counter, and
  if the counter still equals `c1` and `c1` is even and non-zero, every
  field read so far is from the frame `c1` names;
* once finished: `c1` is at most the counter, and if the check passes
  (`c1 = c2`, `c1` even and non-zero), all `n` fields read are from the
  frame `c1` names. -/
structure Inv (s : System) : Prop where
  seq_counts : s.shared.seq = 2 * s.writer.done + phaseBit s.writer.phase
  done_le : s.writer.done ≤ s.writer.frames.length
  writing_bounds : ∀ i, s.writer.phase = .writing i →
    s.writer.done < s.writer.frames.length ∧ i ≤ s.fieldCount
  idle_fields : s.writer.phase = .idle → 0 < s.writer.done →
    ∀ j < s.fieldCount, s.shared.fields j = lastFrame s.writer j
  writing_fields : ∀ i, s.writer.phase = .writing i →
    ∀ j < i, s.shared.fields j = currentFrame s.writer j
  reading_inv : ∀ c1 got, s.reader = .reading c1 got →
    c1 ≤ s.shared.seq ∧ got.length ≤ s.fieldCount ∧
    (s.shared.seq = c1 → c1 % 2 = 0 → c1 ≠ 0 →
      got = fieldsOf got.length (s.writer.frames.getD (frameIndex c1) zeroFrame))
  finished_inv : ∀ c1 got c2, s.reader = .finished c1 got c2 →
    c1 ≤ s.shared.seq ∧
    (c1 = c2 → c1 % 2 = 0 → c1 ≠ 0 →
      got = fieldsOf s.fieldCount (s.writer.frames.getD (frameIndex c1) zeroFrame))

/-! ## Facts about the pieces -/

/-- Listing one more field appends that field's value. -/
theorem fieldsOf_succ (n : Nat) (f : Frame) : fieldsOf (n + 1) f = fieldsOf n f ++ [f n] := by
  simp [fieldsOf, List.range_succ]

/-- Listing no fields gives the empty list. -/
theorem fieldsOf_zero (f : Frame) : fieldsOf 0 f = [] := rfl

/-- `fieldsOf n f` has `n` entries. -/
theorem fieldsOf_length (n : Nat) (f : Frame) : (fieldsOf n f).length = n := by
  simp [fieldsOf]

/-- The script and the field count never change. -/
theorem step_frames (s : System) (who : Actor) :
    (step s who).writer.frames = s.writer.frames ∧ (step s who).fieldCount = s.fieldCount := by
  cases who with
  | writer =>
    simp only [step, writerStep]
    cases s.writer.phase <;> simp only <;> split <;> simp
  | reader =>
    simp only [step, readerStep]
    cases s.reader <;> simp only <;> (try split) <;> simp

/-- The script and the field count are the same after any schedule. -/
theorem run_frames (sched : List Actor) (s : System) :
    (run sched s).writer.frames = s.writer.frames ∧ (run sched s).fieldCount = s.fieldCount := by
  induction sched generalizing s with
  | nil => exact ⟨rfl, rfl⟩
  | cons who rest ih =>
    simp only [run, List.foldl_cons]
    have := ih (step s who)
    simp only [run] at this
    rw [this.1, this.2]
    exact step_frames s who

/-- From the initial state, the script is unchanged by any schedule. -/
theorem run_init_frames (sched : List Actor) (n : Nat) (frames : List Frame) :
    (run sched (init n frames)).writer.frames = frames :=
  (run_frames sched (init n frames)).1

/-- From the initial state, the field count is unchanged by any schedule. -/
theorem run_init_fieldCount (sched : List Actor) (n : Nat) (frames : List Frame) :
    (run sched (init n frames)).fieldCount = n :=
  (run_frames sched (init n frames)).2

/-- Once the reader is finished, nothing changes it. -/
theorem run_finished (sched : List Actor) (s : System) (c1 : Nat) (got : List Nat) (c2 : Nat)
    (h : s.reader = .finished c1 got c2) : (run sched s).reader = .finished c1 got c2 := by
  induction sched generalizing s with
  | nil => exact h
  | cons who rest ih =>
    simp only [run, List.foldl_cons]
    apply ih
    cases who with
    | writer =>
      simp only [step, writerStep]
      cases s.writer.phase <;> simp only <;> split <;> simp [h]
    | reader => simp [step, readerStep, h]

/-- The reader's `c1`, once read, never changes. -/
theorem step_c1 (s : System) (who : Actor) (c1 : Nat) (got : List Nat)
    (h : s.reader = .reading c1 got) :
    (∃ got', (step s who).reader = .reading c1 got') ∨
    (∃ got' c2, (step s who).reader = .finished c1 got' c2) := by
  cases who with
  | writer =>
    simp only [step, writerStep]
    cases s.writer.phase <;> simp only <;> split <;> simp [h]
  | reader =>
    simp only [step, readerStep, h]
    split
    · exact Or.inl ⟨_, rfl⟩
    · exact Or.inr ⟨_, _, rfl⟩

/-- The reader's `c1`, once read, is the same at the end of any schedule. -/
theorem run_c1 (sched : List Actor) (s : System) (c1 : Nat) (got : List Nat)
    (h : s.reader = .reading c1 got) :
    (∃ got', (run sched s).reader = .reading c1 got') ∨
    (∃ got' c2, (run sched s).reader = .finished c1 got' c2) := by
  induction sched generalizing s got with
  | nil => exact Or.inl ⟨got, h⟩
  | cons who rest ih =>
    simp only [run, List.foldl_cons]
    rcases step_c1 s who c1 got h with ⟨got', h'⟩ | ⟨got', c2, h'⟩
    · exact ih _ _ h'
    · exact Or.inr ⟨got', c2, run_finished rest _ c1 got' c2 h'⟩

/-! ## The invariant holds at the start and is kept by every step -/

/-- The initial state satisfies the invariant. -/
theorem inv_init (n : Nat) (frames : List Frame) : Inv (init n frames) where
  seq_counts := rfl
  done_le := Nat.zero_le _
  writing_bounds := by intro i h; cases h
  idle_fields := by intro _ h; cases h
  writing_fields := by intro i h; cases h
  reading_inv := by intro _ _ h; cases h
  finished_inv := by intro _ _ _ h; cases h

/-- A writer step keeps the invariant. -/
theorem inv_writerStep (s : System) (hinv : Inv s) : Inv (writerStep s) := by
  obtain ⟨hseq, hdone, hwb, hidle, hwf, hread, hfin⟩ := hinv
  unfold writerStep
  cases hph : s.writer.phase with
  | idle =>
    simp only [hph, phaseBit] at hseq
    simp only
    split
    · -- begin a write: counter becomes odd
      rename_i hlt
      refine ⟨?_, hdone, ?_, ?_, ?_, ?_, ?_⟩
      · simp only [phaseBit]; omega
      · intro i h
        simp only [WriterPhase.writing.injEq] at h
        subst h
        exact ⟨hlt, Nat.zero_le _⟩
      · intro h; cases h
      · intro i h j hj
        simp only [WriterPhase.writing.injEq] at h
        subst h
        exact absurd hj (Nat.not_lt_zero _)
      · intro c1 got h
        have ⟨h1, h2, _⟩ := hread c1 got h
        refine ⟨by simp; omega, h2, ?_⟩
        intro h'
        simp at h'
        omega
      · intro c1 got c2 h
        have ⟨h1, h2⟩ := hfin c1 got c2 h
        exact ⟨by simp; omega, h2⟩
    · exact ⟨by simp [hseq, hph, phaseBit], hdone, hwb, hidle, hwf, hread, hfin⟩
  | writing i =>
    simp only [hph, phaseBit] at hseq
    have ⟨hlt, hin⟩ := hwb i hph
    have hwfi := hwf i hph
    simp only
    split
    · -- write field `i`
      rename_i hi
      refine ⟨?_, hdone, ?_, ?_, ?_, ?_, ?_⟩
      · simp only [phaseBit]; omega
      · intro i' h
        simp only [WriterPhase.writing.injEq] at h
        subst h
        exact ⟨hlt, hi⟩
      · intro h; cases h
      · intro i' h j hj
        simp only [WriterPhase.writing.injEq] at h
        subst h
        simp only [currentFrame]
        by_cases hji : j = i
        · simp [hji]
        · simp only [hji, ite_false]
          exact hwfi j (by omega)
      · intro c1 got h
        have ⟨h1, h2, _⟩ := hread c1 got h
        refine ⟨h1, h2, ?_⟩
        intro h' heven
        simp at h'
        omega
      · intro c1 got c2 h
        have ⟨h1, h2⟩ := hfin c1 got c2 h
        exact ⟨h1, h2⟩
    · -- all fields written: counter becomes even, frame complete
      rename_i hi
      have hin' : i = s.fieldCount := by omega
      refine ⟨?_, hlt, ?_, ?_, ?_, ?_, ?_⟩
      · simp only [phaseBit]; omega
      · intro i' h; cases h
      · intro _ _ j hj
        dsimp only at hj
        simp only [lastFrame, Nat.add_sub_cancel]
        exact hwfi j (by omega)
      · intro i' h; cases h
      · intro c1 got h
        have ⟨h1, h2, _⟩ := hread c1 got h
        refine ⟨by simp; omega, h2, ?_⟩
        intro h'
        simp at h'
        omega
      · intro c1 got c2 h
        have ⟨h1, h2⟩ := hfin c1 got c2 h
        exact ⟨by simp; omega, h2⟩

/-- A reader step keeps the invariant. -/
theorem inv_readerStep (s : System) (hinv : Inv s) : Inv (readerStep s) := by
  obtain ⟨hseq, hdone, hwb, hidle, hwf, hread, hfin⟩ := hinv
  unfold readerStep
  cases hr : s.reader with
  | start =>
    simp only
    refine ⟨hseq, hdone, hwb, hidle, hwf, ?_, ?_⟩
    · intro c1 got h
      simp only [ReaderPhase.reading.injEq] at h
      obtain ⟨rfl, rfl⟩ := h
      exact ⟨Nat.le_refl _, Nat.zero_le _, fun _ _ _ => rfl⟩
    · intro _ _ _ h; cases h
  | reading c1 got =>
    have ⟨h1, h2, h3⟩ := hread c1 got hr
    simp only
    split
    · -- read one more field
      rename_i hlen
      refine ⟨hseq, hdone, hwb, hidle, hwf, ?_, ?_⟩
      · intro c1' got' h
        simp only [ReaderPhase.reading.injEq] at h
        obtain ⟨rfl, rfl⟩ := h
        refine ⟨h1, by simp; omega, ?_⟩
        intro hc heven hne
        dsimp only at hc
        have hgot := h3 hc heven hne
        -- the counter is even and non-zero, so the writer is idle with a frame done
        have hbit : phaseBit s.writer.phase = 0 := by
          cases hph : s.writer.phase <;> simp [hph, phaseBit] at hseq ⊢ <;> omega
        have hidle' : s.writer.phase = .idle := by
          cases hph : s.writer.phase <;> simp [hph, phaseBit] at hbit ⊢
        rw [hbit] at hseq
        have hpos : 0 < s.writer.done := by omega
        have hfield := hidle hidle' hpos got.length hlen
        have hidx : frameIndex c1 = s.writer.done - 1 := by
          unfold frameIndex; omega
        rw [List.length_append, List.length_singleton, fieldsOf_succ, hfield, lastFrame, ← hidx]
        exact congrArg (fun l => l ++ [s.writer.frames.getD (frameIndex c1) zeroFrame got.length]) hgot
      · intro _ _ _ h; cases h
    · -- all fields read: second counter read
      rename_i hlen
      refine ⟨hseq, hdone, hwb, hidle, hwf, ?_, ?_⟩
      · intro _ _ h; cases h
      · intro c1' got' c2' h
        simp only [ReaderPhase.finished.injEq] at h
        obtain ⟨rfl, rfl, rfl⟩ := h
        refine ⟨h1, ?_⟩
        intro hc heven hne
        have hlen' : got.length = s.fieldCount := by omega
        rw [← hlen']
        exact h3 hc.symm heven hne
  | finished c1 got c2 =>
    simp only
    exact ⟨hseq, hdone, hwb, hidle, hwf, hread, hfin⟩

/-- Any schedule keeps the invariant. -/
theorem inv_run (sched : List Actor) (s : System) (hinv : Inv s) : Inv (run sched s) := by
  induction sched generalizing s with
  | nil => exact hinv
  | cons who rest ih =>
    simp only [run, List.foldl_cons]
    apply ih
    cases who with
    | writer => exact inv_writerStep s hinv
    | reader => exact inv_readerStep s hinv

/-- What the acceptance check means, as three plain facts. -/
theorem accepts_iff (c1 : Nat) (got : List Nat) (c2 : Nat) :
    accepts (.finished c1 got c2) = true ↔ c1 % 2 = 0 ∧ c1 ≠ 0 ∧ c1 = c2 := by
  simp [accepts, and_assoc]

/-! ## D1: no torn frames -/

/-- **D1.** Under any schedule, if the reader finishes and its check
accepts, then the frame it read is a frame the writer has completed:
script entry `frameIndex c1` (which is below the number of completed
frames, hence inside the script), with all `n` fields from that one frame. -/
theorem no_torn_frames (n : Nat) (frames : List Frame) (sched : List Actor)
    (c1 : Nat) (got : List Nat) (c2 : Nat)
    (hfin : (run sched (init n frames)).reader = .finished c1 got c2)
    (hacc : accepts (.finished c1 got c2) = true) :
    frameIndex c1 < (run sched (init n frames)).writer.done ∧
    frameIndex c1 < frames.length ∧
    got = fieldsOf n (frames.getD (frameIndex c1) zeroFrame) := by
  have hinv := inv_run sched _ (inv_init n frames)
  have hframes := run_init_frames sched n frames
  have hn := run_init_fieldCount sched n frames
  obtain ⟨heven, hne, heq⟩ := (accepts_iff c1 got c2).mp hacc
  have ⟨hle, hgot⟩ := hinv.finished_inv c1 got c2 hfin
  have hseq := hinv.seq_counts
  have hdone := hinv.done_le
  rw [hframes] at hdone hgot
  rw [hn] at hgot
  refine ⟨?_, ?_, hgot heq heven hne⟩ <;> unfold frameIndex <;>
    cases hph : (run sched (init n frames)).writer.phase <;> simp [hph, phaseBit] at hseq <;> omega

/-! ## D2: freshness -/

/-- D2 for a state `s1` that satisfies the invariant and whose reader has
not started: if the reader steps now (its first counter read) and later
accepts, `c1` is twice the number of frames done in `s1`, and the frame
it read is the newest one done in `s1`. -/
theorem freshness_from (s1 : System) (hinv1 : Inv s1) (hstart : s1.reader = .start)
    (after : List Actor) (c1 : Nat) (got : List Nat) (c2 : Nat)
    (hfin : (run after (step s1 .reader)).reader = .finished c1 got c2)
    (hacc : accepts (.finished c1 got c2) = true) :
    c1 = 2 * s1.writer.done ∧ frameIndex c1 = s1.writer.done - 1 := by
  have hread : (step s1 .reader).reader = .reading s1.shared.seq [] := by
    simp [step, readerStep, hstart]
  rcases run_c1 after _ _ _ hread with ⟨got', h'⟩ | ⟨got', c2', h'⟩
  · rw [h'] at hfin; cases hfin
  · rw [h'] at hfin
    cases hfin
    obtain ⟨heven, -, -⟩ := (accepts_iff _ _ _).mp hacc
    have hseq := hinv1.seq_counts
    have hbit : phaseBit s1.writer.phase = 0 := by
      cases hph : s1.writer.phase <;> simp [hph, phaseBit] at hseq ⊢ <;> omega
    rw [hbit] at hseq
    refine ⟨hseq, ?_⟩
    unfold frameIndex
    omega

/-- **D2.** Split the schedule at the reader's first counter read: after
`before` the reader has not started, and the next step is the reader's.
If the reader later accepts, then `c1` is twice the number of frames
complete at that moment, and the frame it read is the newest complete
one at that moment (script entry `done - 1`). -/
theorem freshness (n : Nat) (frames : List Frame) (before after : List Actor)
    (hstart : (run before (init n frames)).reader = .start)
    (c1 : Nat) (got : List Nat) (c2 : Nat)
    (hfin : (run (before ++ .reader :: after) (init n frames)).reader = .finished c1 got c2)
    (hacc : accepts (.finished c1 got c2) = true) :
    c1 = 2 * (run before (init n frames)).writer.done ∧
    got = fieldsOf n (frames.getD ((run before (init n frames)).writer.done - 1) zeroFrame) := by
  have ⟨_, _, hgot⟩ := no_torn_frames n frames _ c1 got c2 hfin hacc
  have hrun : run (before ++ .reader :: after) (init n frames)
      = run after (step (run before (init n frames)) .reader) := by
    simp only [run, List.foldl_append, List.foldl_cons]
  rw [hrun] at hfin
  have ⟨hc1, hidx⟩ := freshness_from _ (inv_run before _ (inv_init n frames)) hstart after
    c1 got c2 hfin hacc
  refine ⟨hc1, ?_⟩
  rw [hgot, hidx]

/-- **D2 (identification).** Two accepted counter values that name the
same frame are the same value: the counter identifies the frame. -/
theorem counter_identifies_frame (c c' : Nat) (hc : c % 2 = 0) (hc' : c' % 2 = 0)
    (h0 : c ≠ 0) (h0' : c' ≠ 0) (h : frameIndex c = frameIndex c') : c = c' := by
  unfold frameIndex at h
  omega

/-! ## Hypotheses are satisfiable: concrete instances -/

/-- A clean read after one publish: the reader accepts and gets frame 1. -/
example : (run (publishOne ++ readOne) exInit).reader = .finished 2 [10, 11] 2 := by decide

/-- D1 on that schedule, every hypothesis discharged. -/
example : frameIndex 2 < (run (publishOne ++ readOne) exInit).writer.done ∧
    frameIndex 2 < [frame1, frame2].length ∧
    [10, 11] = fieldsOf 2 ([frame1, frame2].getD (frameIndex 2) zeroFrame) :=
  no_torn_frames 2 [frame1, frame2] (publishOne ++ readOne) 2 [10, 11] 2 (by decide) (by decide)

/-- D2 on that schedule: the reader started after `publishOne`, when one frame was done. -/
example : 2 = 2 * (run publishOne exInit).writer.done ∧
    [10, 11] = fieldsOf 2 ([frame1, frame2].getD ((run publishOne exInit).writer.done - 1) zeroFrame) :=
  freshness 2 [frame1, frame2] publishOne [.reader, .reader, .reader] (by decide)
    2 [10, 11] 2 (by decide) (by decide)

/-- Counter identification on concrete values. -/
example : (4 : Nat) = 4 :=
  counter_identifies_frame 4 4 (by decide) (by decide) (by decide) (by decide) rfl

/-- Counters 2 and 4 name different frames (0 and 1). -/
example : frameIndex 2 ≠ frameIndex 4 := by decide

/-! ## The spec rejects bad behaviour -/

/-- **Negative example.** On `tornSchedule` the reader's fields are
`[10, 21]`: field 0 from frame 1, field 1 from frame 2.  That is no frame
the writer ever published. -/
theorem torn_is_mixed :
    (run tornSchedule exInit).reader = .finished 2 [10, 21] 3 ∧
    [10, 21] ≠ fieldsOf 2 frame1 ∧ [10, 21] ≠ fieldsOf 2 frame2 := by
  decide

/-- The real check rejects the torn read (`c1 = 2` but `c2 = 3`). -/
theorem torn_rejected : accepts (run tornSchedule exInit).reader = false := by
  decide

/-- **Without the second counter read** the torn read would be accepted:
`c1 = 2` is even and non-zero, which is all the naive check looks at. -/
theorem torn_accepted_by_naive : acceptsNaive (run tornSchedule exInit).reader = true := by
  decide

/-- A read that starts while a write is in progress: the writer has
written field 0 of frame 1 but not field 1 when the reader reads. -/
def midWriteSchedule : List Actor :=
  [.writer, .writer] ++ [.reader, .reader, .reader] ++ [.writer, .writer] ++ [.reader]

/-- The reader sees `c1 = 1` (odd) and the half-written fields `[10, 0]`;
the check rejects it. -/
theorem mid_write_rejected :
    (run midWriteSchedule exInit).reader = .finished 1 [10, 0] 2 ∧
    accepts (run midWriteSchedule exInit).reader = false := by
  decide

end Gpu.Handoff
