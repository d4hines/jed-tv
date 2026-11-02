import Types
import Lean.Elab.Tactic.Omega

/- Explicit protocol for effect completion. Neither parsing nor machine access
   is implemented here. A runner supplies one observation for the requested
   stage. An out-of-order completion fails closed, including duplicate events. -/
namespace LeanBoot.Flow

inductive Mode | authenticate | execute | install | journal
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq

inductive Stage
  | policy | signature | sourceDigest
  | entry | copy | copiedDigest | prepare | invoke
  | invalidate | flushInvalid | writeImage | flushImage | readImage
  | readbackDigest | flushReadback | writeHeader | flushHeader | readHeader | checkHeader
  | encodeRecord | writeRecord | readRecord | verifyRecord
  | complete | rejected
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq

structure Completion where
  stage : Stage
  passed : Bool
  deriving Repr, DecidableEq, BEq, ReflBEq, LawfulBEq

@[inline] def initial : Mode → Stage
  | .journal => .encodeRecord
  | _ => .policy

/-- The successful transitions are deliberately a plain table. -/
@[inline] def next (mode : Mode) (stage : Stage) : Stage :=
  match mode, stage with
  | .journal, .encodeRecord => .writeRecord
  | .journal, .writeRecord => .readRecord
  | .journal, .readRecord => .verifyRecord
  | .journal, .verifyRecord => .complete
  | .authenticate, .policy | .execute, .policy | .install, .policy => .signature
  | .authenticate, .signature | .execute, .signature | .install, .signature => .sourceDigest
  | .authenticate, .sourceDigest => .complete
  | .execute, .sourceDigest => .entry
  | .execute, .entry => .copy
  | .execute, .copy => .copiedDigest
  | .execute, .copiedDigest => .prepare
  | .execute, .prepare => .invoke
  | .execute, .invoke => .complete
  | .install, .sourceDigest => .invalidate
  | .install, .invalidate => .flushInvalid
  | .install, .flushInvalid => .writeImage
  | .install, .writeImage => .flushImage
  | .install, .flushImage => .readImage
  | .install, .readImage => .readbackDigest
  | .install, .readbackDigest => .flushReadback
  | .install, .flushReadback => .writeHeader
  | .install, .writeHeader => .flushHeader
  | .install, .flushHeader => .readHeader
  | .install, .readHeader => .checkHeader
  | .install, .checkHeader => .complete
  | _, .complete => .complete
  | _, _ => .rejected

@[inline] def step (mode : Mode) (state : Stage) (event : Completion) : Stage :=
  if state == .complete || state == .rejected then state
  else if event.stage == state && event.passed then next mode state else .rejected

/-- Total bounded interpreter. Only the current stage is issued; terminal
    states issue nothing. The budget covers every acyclic program below.
    Failure codes are preserved; 45 indicates exhausted/invalid control flow. -/
@[specialize] def runFrom {m : Type → Type} [Monad m]
    (perform : Stage → m Status) (mode : Mode) (fuel : UInt32) (state : Stage) : m Status := do
  if state == .complete then return .ok
  if state == .rejected then return ⟨45⟩
  if empty : fuel == 0 then return ⟨45⟩
  else
    let result ← perform state
    let following := step mode state ⟨state, result.succeeded⟩
    if following == .rejected then
      return if result.succeeded then ⟨45⟩ else result
    runFrom perform mode (fuel - 1) following
termination_by fuel.toNat
decreasing_by
  have positive : 0 < fuel.toNat := by
    have : fuel ≠ 0 := by simpa using empty
    by_cases zero : fuel.toNat = 0
    · exact False.elim (this (UInt32.toNat_inj.mp zero))
    · omega
  have one : (1 : UInt32) <= fuel := by change 1 <= fuel.toNat; omega
  rw [UInt32.toNat_sub_of_le fuel 1 one]
  change fuel.toNat - 1 < fuel.toNat
  omega

@[inline] def run {m : Type → Type} [Monad m]
    (mode : Mode) (perform : Stage → m Status) : m Status :=
  runFrom perform mode 32 (initial mode)

end LeanBoot.Flow
