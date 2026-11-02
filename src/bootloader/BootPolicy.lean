import Init

namespace LeanBoot.BootPolicy

/- Persisted word format: 0 is unstarted, 1/2 are Lean starts, 4 is the
   one-start stock escape. Keep this format and the scalar export stable.
   Successful Home startup does not reset the counter. Standby resume without
   a fresh kernel handoff does not advance it. Proofs.lean checks this reducer
   against the previous bit-level implementation for every input word. -/
@[inline] def initial : UInt32 := 0
@[inline] def leanFirst : UInt32 := 1
@[inline] def leanSecond : UInt32 := 2
@[inline] def stock : UInt32 := 4

/-- Stock escape with the error bit set: do not commit this as a record. -/
@[inline] def rejected : UInt32 := 0x80000004

@[inline] def failed (state : UInt32) : Bool := state &&& 0x80000000 != 0

@[inline] def valid (state : UInt32) : Bool :=
  state == initial || state == leanFirst || state == leanSecond || state == stock

@[inline] def fallback (state : UInt32) : Bool :=
  !valid state || state == stock

/-- Event 0 inspects; event 1 is a full kernel start, including adb reboot.
    Other events and invalid states reject without a journal commit.
    Initial -> Lean first -> Lean second -> stock -> Lean first. -/
@[export leanboot_policy_step]
def step (state event : UInt32) : UInt32 :=
  if !valid state || event > 1 then rejected
  else if event == 0 then state
  else if state == leanSecond then stock
  else if state == leanFirst then leanSecond
  else leanFirst

end LeanBoot.BootPolicy
