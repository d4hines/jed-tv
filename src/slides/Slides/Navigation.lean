import Init
namespace Slides
structure Navigation where
  current : Nat := 0
  digits : Nat := 0
  digitAt : Nat := 0
  quit : Bool := false
  deriving BEq, Repr, Inhabited
/-- Linux keycodes from App API v1. Home is normally intercepted by PID 1.
On the TCL remote: OK, Right, Down and Ch+ go forward; Up, Left and Ch- go
back; Back leaves. Digits (keyboard) jump to a slide. -/
def navigate (s : Navigation) (key now count : Nat) : Navigation := Id.run do
  if count == 0 then return s
  if [1, 14, 102, 172, 158, 174, 223].contains key then return { s with quit := true }
  if [28, 96, 106, 108, 109, 402, 163, 164, 57, 352, 353].contains key then return { s with current := min (count-1) (s.current+1) }
  if [103, 104, 105, 403, 165].contains key then return { s with current := s.current-1 }
  if key ≥ 2 && key ≤ 11 then
    let d := if key == 11 then 0 else key-1
    let n := if now-s.digitAt < 1500 && s.digits*10+d ≤ count then s.digits*10+d else d
    if n > 0 then return { s with current := min count n - 1, digits := n, digitAt := now }
  return s
end Slides
