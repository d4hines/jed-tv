import CityJsb.Plant
import CityJsb.F.Plant
import JsbExact.F.Convert

/-! `lake exe jsbdiff [frames]`: the generic model at `Float` against the `Float` model.

From the same initial conditions (`CityJsb.init` and `CityJsb.F.init`), both models are stepped `frames` times
(default 20,000, that is 156 s at 1/128 s) with the same deterministic control schedule, and after every
frame the 19 observation values, every number of the state and every number of the diagnostics record are
compared by `Float.toBits`. The first mismatch stops the run with the frame and field. On success the line
`JSBDIFF PASS N frames bit-identical` is printed, then both models are timed. Exit code 1 on failure. -/

open JsbExact

namespace JsbDiff

/-- Frame length, JSBSim's 1/128 s. -/
def dt : Float := Gen.dt

@[inline] def clamp (lo hi x : Float) : Float := if x < lo then lo else if x > hi then hi else x

/-- Control commands for frame `k`: elevator, aileron, rudder in [-1, 1], throttle in [0, 1]. Sinusoids of
different periods, and in each 10 s window a step: a 0.4 s full deflection of one surface (both signs over
the 80 s cycle), or full or idle throttle for the whole window. Small enough that the aircraft keeps flying
from a sensible start, while every input reaches both ends of its range. Both models get these same numbers. -/
def controls (k : Nat) : Float × Float × Float × Float :=
  let t := Float.ofNat k * dt
  let twoPi := 2.0 * 3.141592653589793
  let window := (k / 1280) % 8
  let pulse := k % 1280 < 51
  let elevStep := if pulse && window == 1 then 1.0 else if pulse && window == 3 then -1.0 else 0.0
  let ailStep := if pulse && window == 2 then 1.0 else if pulse && window == 5 then -1.0 else 0.0
  let rudStep := if pulse && window == 4 then -1.0 else if pulse && window == 6 then 1.0 else 0.0
  let thrStep := if window == 0 then 1.0 else if window == 7 then -1.0 else 0.0
  let elev := clamp (-1.0) 1.0 (0.12 * Float.sin (twoPi * t / 7.0) + elevStep)
  let ail := clamp (-1.0) 1.0 (0.25 * Float.sin (twoPi * t / 4.5) + ailStep)
  let rud := clamp (-1.0) 1.0 (0.2 * Float.sin (twoPi * t / 3.0) + rudStep)
  let thr := clamp 0.0 1.0 (0.6 + 0.3 * Float.sin (twoPi * t / 11.0) + thrStep)
  (elev, ail, rud, thr)

/-- An initial condition (`CityJsb.init`'s arguments) and a brake command. The demo never brakes; one case
here does, so that the brake code of the gear model is compared too. -/
structure IC where
  name : String
  h : Float
  v : Float
  gam : Float
  psi : Float
  phi : Float
  alpha : Float
  brk : Float
  deriving Inhabited

def ics : Array IC := #[
  { name := "cruise at 1500 m", h := 1500, v := 40, gam := 0, psi := 0, phi := 0, alpha := 0.05, brk := 0 },
  { name := "climbing turn at 300 m", h := 300, v := 55, gam := 0.1, psi := 1.2, phi := 0.3, alpha := 0.03, brk := 0 },
  { name := "descent from 120 m", h := 120, v := 35, gam := -0.08, psi := -2.0, phi := -0.15, alpha := 0.06, brk := 0 },
  { name := "on the ground, braking", h := 1.2, v := 30, gam := 0, psi := 0.5, phi := 0, alpha := 0.03, brk := 0.4 }]

def input (e a r t brk : Float) : Input Float :=
  { elev := e, ail := a, rud := r, thr := t, mix := 0.9, brkL := brk, brkR := brk }
def inputF (e a r t brk : Float) : F.Input :=
  { elev := e, ail := a, rud := r, thr := t, mix := 0.9, brkL := brk, brkR := brk }

def hex (n : UInt64) : String := String.ofList (Nat.toDigits 16 n.toNat)

/-- The first entry whose bits differ, as `name: generic value vs F value`. -/
def firstDiff (a b : F.Fields) : Option String := Id.run do
  if a.size != b.size then return some s!"{a.size} fields vs {b.size}"
  for i in [0:a.size] do
    let (n, x) := a[i]!
    let (m, y) := b[i]!
    if n != m then return some s!"field order: {n} vs {m}"
    if x.toBits != y.toBits then
      return some s!"{n}: generic {x} (0x{hex x.toBits}) vs F {y} (0x{hex y.toBits})"
  return none

def obsFields (o : FloatArray) : F.Fields :=
  (List.range o.size).toArray.map fun i => (s!"obs[{i}]", o.get! i)

/-- Non-negative `x` with `d` decimals. -/
def fmt (x : Float) (d : Nat) : String :=
  let n := (x * Float.ofNat (10 ^ d)).round.toUInt64.toNat
  let fs := toString (n % 10 ^ d)
  s!"{n / 10 ^ d}.{"".pushn '0' (d - fs.length)}{fs}"

/-- Step both models `frames` times from `ic`, comparing after every frame. Without brakes, the exported
entry points (`CityJsb.initE`, `CityJsb.stepE`, what the C ABI calls) are stepped as well and compared. -/
def runDiff (ic : IC) (frames : Nat) : IO (Except String Nat) := do
  let mut sg := CityJsb.init ic.h ic.v ic.gam ic.psi ic.phi ic.alpha
  let mut sf := CityJsb.F.init ic.h ic.v ic.gam ic.psi ic.phi ic.alpha
  let mut se := CityJsb.initE ic.h ic.v ic.gam ic.psi ic.phi ic.alpha
  let exported := ic.brk == 0.0
  if let some d := firstDiff (F.State.ofGeneric sg).fields sf.fields then
    return .error s!"{ic.name}: initial state: {d}"
  if exported then
    if let some d := firstDiff sf.fields se.fields then
      return .error s!"{ic.name}: initial state of the exported init: {d}"
  let mut last : FloatArray := ⟨#[]⟩
  let mut wow := 0
  let mut firstWow : Option Nat := none
  for k in [0:frames] do
    let (e, a, r, t) := controls k
    let (sg', dg) := step sg (input e a r t ic.brk)
    let (sf', df) := F.step sf (inputF e a r t ic.brk)
    let og := CityJsb.obs sg' dg
    let of := CityJsb.F.obs sf' df
    if og.size != 19 || of.size != 19 then
      return .error s!"{ic.name} frame {k}: observation has {og.size}/{of.size} entries, not 19"
    if let some d := firstDiff (obsFields og) (obsFields of) then
      return .error s!"{ic.name} frame {k} observation {d}"
    if let some d := firstDiff (F.State.ofGeneric sg').fields sf'.fields then
      return .error s!"{ic.name} frame {k} state {d}"
    if let some d := firstDiff (F.Diag.ofGeneric dg).fields df.fields then
      return .error s!"{ic.name} frame {k} diagnostics {d}"
    if exported then
      let (se', oe) := CityJsb.stepE se e a r t
      if let some d := firstDiff (obsFields of) (obsFields oe) then
        return .error s!"{ic.name} frame {k} exported step observation {d}"
      if let some d := firstDiff sf'.fields se'.fields then
        return .error s!"{ic.name} frame {k} exported step state {d}"
      se := se'
    if sf'.gears.any (·.wow) then
      wow := wow + 1
      if firstWow.isNone then firstWow := some k
    sg := sg'
    sf := sf'
    last := of
  let nan := (List.range last.size).any fun i => (last.get! i).isNaN
  let contact := match firstWow with
    | some k => s!"wheel contact in {wow} frames from frame {k}"
    | none => "no wheel contact"
  IO.println s!"  {ic.name}: {frames} frames identical; final altitude {fmt (last.get! 2) 1} m, \
    airspeed {fmt (last.get! 6) 1} m/s, rpm {fmt (last.get! 18) 0}, {contact}, \
    NaN in final observation: {nan}"
  return .ok frames

/-- A steady cruise command for the timing runs, so that they measure in-flight steps (what the TV runs). -/
def cruise : Float × Float × Float × Float := (0.0, 0.0, 0.0, 0.6)

/-- Nanoseconds per step of the generic plant step (`CityJsb.stepObs`, with its observation), and the final
height above the terrain. -/
def timeGeneric (ic : IC) (frames : Nat) : IO (Float × Float) := do
  let mut s := CityJsb.init ic.h ic.v ic.gam ic.psi ic.phi ic.alpha
  let mut agl : Float := 0
  let (e, a, r, t) := cruise
  let t0 ← IO.monoNanosNow
  for _ in [0:frames] do
    let (s', o) := CityJsb.stepObs s e a r t
    s := s'
    agl := o.get! 16
  let t1 ← IO.monoNanosNow
  return (Float.ofNat (t1 - t0) / Float.ofNat frames, agl)

/-- Nanoseconds per step of the exported plant step (`CityJsb.stepE`, the `Float` model), and the final height
above the terrain. -/
def timeF (ic : IC) (frames : Nat) : IO (Float × Float) := do
  let mut s := CityJsb.initE ic.h ic.v ic.gam ic.psi ic.phi ic.alpha
  let mut agl : Float := 0
  let (e, a, r, t) := cruise
  let t0 ← IO.monoNanosNow
  for _ in [0:frames] do
    let (s', o) := CityJsb.stepE s e a r t
    s := s'
    agl := o.get! 16
  let t1 ← IO.monoNanosNow
  return (Float.ofNat (t1 - t0) / Float.ofNat frames, agl)

end JsbDiff

open JsbDiff in
def main (args : List String) : IO UInt32 := do
  let frames := (args.head? >>= String.toNat?).getD 20000
  IO.println s!"jsbdiff: {ics.size} initial conditions, {frames} frames each"
  let mut total := 0
  for ic in ics do
    match ← runDiff ic frames with
    | .error msg =>
      IO.println s!"JSBDIFF FAIL {msg}"
      return 1
    | .ok n => total := total + n
  IO.println s!"JSBDIFF PASS {total} frames bit-identical"
  let ic := ics[0]!
  let _ ← timeGeneric ic 2000
  let _ ← timeF ic 2000
  let (g, aglG) ← timeGeneric ic frames
  let (f, aglF) ← timeF ic frames
  IO.println s!"timing ({frames} frames each from {ic.name}, steady cruise command, final height above \
    terrain {fmt aglG 0}/{fmt aglF 0} m): generic {fmt (g / 1e6) 4} ms/step, F {fmt (f / 1e6) 4} ms/step, \
    generic/F = {fmt (g / f) 2}"
  return 0
