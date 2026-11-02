prelude
import Init.Control.Basic
import Init.Data.UInt

/- Global volume policy. Units are millionths of full-scale PCM amplitude;
   the existing calibrated ceiling is 0.30, with twenty remote steps. -/
namespace TV.Volume

def ceiling : UInt32 := 300000
def step : UInt32 := 15000
def defaultGain : UInt32 := 75000

def clamp (gain : UInt32) : UInt32 := min gain ceiling

structure State where
  gain : UInt32
  saved : UInt32
  deriving BEq, Repr

def initial (gain : UInt32) : State :=
  let gain := clamp gain
  ⟨gain, if gain == 0 then defaultGain else gain⟩

inductive Event where
  | tick
  | external (gain : UInt32)
  | key (code value : UInt32)
  deriving BEq, Repr

structure Effects (m : Type → Type) where
  poll : m Event
  commit : UInt32 → m Bool
  publish : UInt32 → m Unit

/-- Releases, unrelated keys and repeated mute events have no effect. Volume
steps round to the calibrated grid before advancing, as the native controller did. -/
def proposal (state : State) (code value : UInt32) : Option State :=
  if value == 0 then none else
  let gain := clamp state.gain
  let slot := (gain + step / 2) / step
  if code == 115 then
    some { state with gain := min (slot + 1) 20 * step }
  else if code == 114 then
    some { state with gain := (if slot == 0 then 0 else slot - 1) * step }
  else if code == 113 && value == 1 then
    some (if gain == 0 then { state with gain := clamp state.saved }
          else ⟨0, gain⟩)
  else none

/-- Commit first. A failed atomic write changes neither remembered mute state nor
published output. External updates preserve the last explicit pre-mute level. -/
def handle [Monad m] (effects : Effects m) (state : State) (event : Event) : m State := do
  match event with
  | .tick => pure state
  | .external gain =>
    let gain := clamp gain
    effects.publish gain
    pure { state with gain }
  | .key code value =>
    match proposal state code value with
    | none => pure state
    | some next =>
      if ← effects.commit next.gain then
        effects.publish next.gain
        pure next
      else pure state

partial def run [Monad m] (effects : Effects m) (state : State) : m Unit := do
  let next ← handle effects state (← effects.poll)
  run effects next

end TV.Volume
