prelude
import Init.System.IO

/-! Exclusive ownership and cleanup of the warm speaker path. The native
adapter owns handles; this module alone chooses operation order and reuse.
There is no cold initialization or retry after uncertain cleanup. -/
namespace TV.Audio.Device

inductive Result where
  | ok | failed | uncertain
  deriving BEq, Inhabited

structure Effects (m : Type → Type) where
  openPcm : m Result
  openOutput : m Result
  attach : m Result
  setVolume : m Result
  start : m Result
  restoreVolume : m Result
  restoreRoute : m Result
  closePcm : m Result
  retainPcm : m Unit
  dropOutput : m Result

structure State where
  owner : Option UInt32 := none
  pcm : Bool := false
  route : Bool := false
  volume : Bool := false
  retain : Bool := false
  poisoned : Bool := false
  deriving BEq, Inhabited

def State.owns (s : State) (slot : UInt32) : Bool :=
  s.owner == some slot && !s.poisoned

/-- Attempt each restoration once. A route that may still reference PCM
prevents PCM destruction. Other cleanup failures also prohibit reuse. -/
def cleanup [Monad m] (e : Effects m) (s : State) : m State := do
  let mut poisoned := s.poisoned
  if s.volume then
    poisoned := (← e.restoreVolume) != .ok || poisoned
  let mut retain := s.retain
  if s.route then
    if (← e.restoreRoute) != .ok then
      poisoned := true
      retain := true
  if s.pcm then
    if retain then
      e.retainPcm
      poisoned := true
    else
      poisoned := (← e.closePcm) != .ok || poisoned
  poisoned := (← e.dropOutput) != .ok || poisoned
  return { poisoned }

private def reject [Monad m] (e : Effects m) (s : State) : m (UInt32 × State) := do
  return (0x80000005, ← cleanup e s)

/-- One finite acquisition attempt. Native failures do not trigger a retry;
the owner is claimed before the first operation and released after cleanup. -/
def acquire [Monad m] (e : Effects m) (slot : UInt32) : StateT State m UInt32 := do
  let s ← get
  if s.poisoned || s.owner.isSome then return 0x80000010
  let mut s : State := { owner := some slot }
  if (← e.openPcm) != .ok then
    let (code, next) ← liftM (reject e s)
    set next
    return code
  s := {s with pcm := true}
  if (← e.openOutput) != .ok then
    let (code, next) ← liftM (reject e s)
    set next
    return code
  let route ← e.attach
  if route != .ok then
    s := {s with retain := route == .uncertain, poisoned := route == .uncertain}
    let (code, next) ← liftM (reject e s)
    set next
    return code
  s := {s with route := true}
  let volume ← e.setVolume
  if volume != .ok then
    s := {s with poisoned := volume == .uncertain}
    let (code, next) ← liftM (reject e s)
    set next
    return code
  s := {s with volume := true}
  if (← e.start) != .ok then
    let (code, next) ← liftM (reject e s)
    set next
    return code
  set s
  return 0

def release [Monad m] (e : Effects m) (slot : UInt32) : StateT State m Unit := do
  let s ← get
  if s.owner == some slot then set (← liftM (cleanup e s))

end TV.Audio.Device
