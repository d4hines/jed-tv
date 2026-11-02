import Flow

namespace LeanBoot.Flow

/-- Declarative ordering specification, separate from the transition table. -/
def program : Mode → List Stage
  | .authenticate => [.policy, .signature, .sourceDigest]
  | .execute => [.policy, .signature, .sourceDigest, .entry, .copy, .copiedDigest, .prepare, .invoke]
  | .install => [.policy, .signature, .sourceDigest, .invalidate, .flushInvalid, .writeImage,
      .flushImage, .readImage, .readbackDigest, .flushReadback, .writeHeader,
      .flushHeader, .readHeader, .checkHeader]
  | .journal => [.encodeRecord, .writeRecord, .readRecord, .verifyRecord]

def required (mode : Mode) (state : Stage) : List Stage :=
  if state == .complete then program mode
  else if state ∈ program mode then (program mode).takeWhile (· != state)
  else []

def pending (mode : Mode) (state : Stage) : Nat :=
  ((program mode).dropWhile (· != state)).length

/-- Lists are chronological. Events may be malicious, repeated, or out of order. -/
def replay (mode : Mode) (state : Stage) : List Completion → Stage
  | [] => state
  | event :: rest => replay mode (step mode state event) rest

def Seen (stage : Stage) (events : List Completion) : Prop :=
  ∃ event ∈ events, event.stage = stage ∧ event.passed = true

theorem rejected_absorbing (mode : Mode) (event : Completion) :
    step mode .rejected event = .rejected := by simp [step]

theorem complete_absorbing (mode : Mode) (event : Completion) :
    step mode .complete event = .complete := by simp [step]

theorem wrong_stage_rejects (mode : Mode) (state : Stage) (event : Completion)
    (active : state ≠ .complete ∧ state ≠ .rejected) (wrong : event.stage ≠ state) :
    step mode state event = .rejected := by simp [step, active.1, active.2, wrong]

theorem failure_rejects (mode : Mode) (state : Stage) (event : Completion)
    (active : state ≠ .complete ∧ state ≠ .rejected) (failed : event.passed = false) :
    step mode state event = .rejected := by simp [step, active.1, active.2, failed]

private theorem required_next (mode : Mode) (state milestone : Stage)
    (h : milestone ∈ required mode (next mode state)) :
    milestone ∈ required mode state ∨ milestone = state := by
  cases mode <;> cases state <;> simp_all [required, next, program, or_assoc]

@[simp] theorem rejected_requires_nothing (mode : Mode) : required mode .rejected = [] := by
  cases mode <;> decide

theorem required_step (mode : Mode) (state milestone : Stage) (event : Completion)
    (h : milestone ∈ required mode (step mode state event)) :
    milestone ∈ required mode state ∨ (event.stage = milestone ∧ event.passed = true) := by
  by_cases terminal : state == .complete || state == .rejected
  · simp [step, terminal] at h
    exact Or.inl h
  · by_cases good : event.stage == state && event.passed
    · have advance : step mode state event = next mode state := by simp [step, terminal, good]
      rw [advance] at h
      rcases required_next mode state milestone h with old | now
      · exact Or.inl old
      · have parts : event.stage = state ∧ event.passed = true := by simpa using good
        exact Or.inr ⟨parts.1.trans now.symm, parts.2⟩
    · simp [step, terminal, good] at h

theorem replay_required (mode : Mode) (events : List Completion) (state milestone : Stage)
    (h : milestone ∈ required mode (replay mode state events)) :
    milestone ∈ required mode state ∨ Seen milestone events := by
  induction events generalizing state with
  | nil => exact Or.inl h
  | cons event rest ih =>
    rcases ih (step mode state event) h with before | seen
    · rcases required_step mode state milestone event before with old | now
      · exact Or.inl old
      · exact Or.inr ⟨event, by simp, now⟩
    · rcases seen with ⟨e, member, observed⟩
      exact Or.inr ⟨e, by simp [member], observed⟩

theorem initial_requires_nothing (mode : Mode) : required mode (initial mode) = [] := by
  cases mode <;> rfl

/-- Reaching a stage requires successful observations of every prerequisite,
    for arbitrary trace lengths, including adversarial event sequences. -/
theorem reached_requires (mode : Mode) (events : List Completion) (state milestone : Stage)
    (reached : replay mode (initial mode) events = state)
    (needed : milestone ∈ required mode state) : Seen milestone events := by
  rw [← reached] at needed
  rcases replay_required mode events (initial mode) milestone needed with empty | observed
  · simp [initial_requires_nothing] at empty
  · exact observed

theorem execution_requires_signature (events : List Completion)
    (h : replay .execute (initial .execute) events = .invoke) : Seen .signature events :=
  reached_requires .execute events .invoke .signature h (by decide)

theorem execution_requires_both_digests (events : List Completion)
    (h : replay .execute (initial .execute) events = .invoke) :
    Seen .sourceDigest events ∧ Seen .copiedDigest events :=
  ⟨reached_requires .execute events .invoke .sourceDigest h (by decide),
   reached_requires .execute events .invoke .copiedDigest h (by decide)⟩

theorem execution_requires_preparation (events : List Completion)
    (h : replay .execute (initial .execute) events = .invoke) : Seen .prepare events :=
  reached_requires .execute events .invoke .prepare h (by decide)

theorem header_commit_requires_readback_and_flush (events : List Completion)
    (h : replay .install (initial .install) events = .writeHeader) :
    Seen .signature events ∧ Seen .readbackDigest events ∧ Seen .flushReadback events :=
  ⟨reached_requires .install events .writeHeader .signature h (by decide),
   reached_requires .install events .writeHeader .readbackDigest h (by decide),
   reached_requires .install events .writeHeader .flushReadback h (by decide)⟩

theorem image_write_requires_durable_invalidation (events : List Completion)
    (h : replay .install (initial .install) events = .writeImage) : Seen .flushInvalid events :=
  reached_requires .install events .writeImage .flushInvalid h (by decide)

theorem journal_success_requires_readback (events : List Completion)
    (h : replay .journal (initial .journal) events = .complete) : Seen .verifyRecord events :=
  reached_requires .journal events .complete .verifyRecord h (by decide)

theorem success_strictly_progresses (mode : Mode) (state : Stage)
    (active : state ∈ program mode) : pending mode (next mode state) < pending mode state := by
  cases mode <;> cases state <;> simp_all [program, pending, next]

theorem all_success_finishes (mode : Mode) :
    replay mode (initial mode) ((program mode).map fun state => ⟨state, true⟩) = .complete := by
  cases mode <;> decide

theorem within_budget (mode : Mode) : (program mode).length < 32 := by
  cases mode <;> decide

theorem duplicate_completion_rejects (mode : Mode) (state : Stage)
    (active : state ∈ program mode) (notLast : next mode state ≠ .complete) :
    step mode (step mode state ⟨state, true⟩) ⟨state, true⟩ = .rejected := by
  cases mode <;> cases state <;> simp_all [program, step, next]

/-- These equations concern the real monadic runner, not just replay. -/
theorem terminal_runner_has_no_effects {m : Type → Type} [Monad m]
    (perform : Stage → m Status) (mode : Mode) (fuel : UInt32) :
    runFrom perform mode fuel .complete = pure .ok := by
  rw [runFrom]
  simp

theorem exhausted_runner_has_no_effects {m : Type → Type} [Monad m]
    (perform : Stage → m Status) (mode : Mode) (state : Stage)
    (active : state ≠ .complete ∧ state ≠ .rejected) :
    runFrom perform mode 0 state = pure ⟨45⟩ := by
  rw [runFrom]
  simp [active.1, active.2]

theorem failed_runner_never_advances {m : Type → Type} [Monad m] [LawfulMonad m]
    (perform : Stage → m Status) (mode : Mode) (fuel : UInt32) (state : Stage)
    (error : Status) (active : state ≠ .complete ∧ state ≠ .rejected)
    (available : fuel ≠ 0) (failed : error.succeeded = false)
    (observation : perform state = pure error) :
    runFrom perform mode fuel state = pure error := by
  rw [runFrom]
  simp [active.1, active.2, available, observation, step, failed]

def recordSuccess (stage : Stage) : StateM (List Stage) Status := do
  modify (· ++ [stage])
  return .ok

theorem successful_runner_follows_program (mode : Mode) :
    ((run mode recordSuccess).run []).run = (Status.ok, program mode) := by
  cases mode <;> dsimp only [run, initial]
  all_goals
    repeat (rw [runFrom]; dsimp [recordSuccess, step, next, Status.succeeded, Status.ok])
    rfl

end LeanBoot.Flow
