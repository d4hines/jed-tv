import TV.Audio.Device

namespace AudioDeviceTests
open TV.Audio.Device

structure Trace where
  failures : Array Result := Array.replicate 10 .ok
  events : Array Nat := #[]

abbrev TestM := StateM Trace

def operation (n : Nat) : TestM Result := do
  modify fun t => {t with events := t.events.push n}
  return (← get).failures[n]!

def effects : Effects TestM where
  openPcm := operation 0
  openOutput := operation 1
  attach := operation 2
  setVolume := operation 3
  start := operation 4
  restoreVolume := operation 5
  restoreRoute := operation 6
  closePcm := operation 7
  retainPcm := discard (operation 8)
  dropOutput := operation 9

def check (ok : Bool) (message : String) : IO Unit :=
  unless ok do throw (IO.userError message)

def failure (n : Nat) (kind : Result := .failed) : Trace :=
  {failures := (Array.replicate 10 .ok).set! n kind}

def acquired : State := {owner := some 2, pcm := true, route := true, volume := true}

def tests : IO Unit := do
  let ((code, live), trace) := ((acquire effects 2).run {}).run {}
  check (code == 0 && live == acquired && trace.events == #[0,1,2,3,4]) "device acquire order"
  let ((code, same), trace) := ((acquire effects 3).run live).run {}
  check (code != 0 && same == live && trace.events.isEmpty) "second owner acquired device"
  let ((_, same), trace) := ((release effects 3).run live).run {}
  check (same == live && trace.events.isEmpty) "wrong owner released device"
  let ((_, free), trace) := ((release effects 2).run live).run {}
  check (free == {} && trace.events == #[5,6,7,9]) "release ordering"
  let ((_, same), trace) := ((release effects 2).run free).run {}
  check (same == free && trace.events.isEmpty) "release repeated cleanup"
  for (n, expected) in #[(0,#[0,9]), (1,#[0,1,7,9]), (2,#[0,1,2,7,9]),
      (3,#[0,1,2,3,6,7,9]), (4,#[0,1,2,3,4,5,6,7,9])] do
    let ((code, final), trace) := ((acquire effects 2).run {}).run (failure n)
    check (code != 0 && final == {} && trace.events == expected) s!"acquire rollback {n}"
  for n in #[2,3] do
    let ((code, final), trace) := ((acquire effects 2).run {}).run (failure n .uncertain)
    let expected := if n == 2 then #[0,1,2,8,9] else #[0,1,2,3,6,7,9]
    check (code != 0 && final.poisoned && final.owner.isNone && trace.events == expected)
      s!"uncertain acquisition {n}"
    let ((code, same), trace) := ((acquire effects 2).run final).run {}
    check (code != 0 && same == final && trace.events.isEmpty) "poisoned device retried"
  -- Exhaust every success/failure combination in cleanup. The route result
  -- decides whether PCM may be destroyed, independently of the gain result.
  for mask in List.range 16 do
    let faults := (Array.replicate 10 Result.ok).set! 5 (if mask % 2 == 1 then .failed else .ok)
      |>.set! 6 (if mask / 2 % 2 == 1 then .failed else .ok)
      |>.set! 7 (if mask / 4 % 2 == 1 then .failed else .ok)
      |>.set! 9 (if mask / 8 % 2 == 1 then .failed else .ok)
    let ((_, final), trace) := ((release effects 2).run acquired).run {failures := faults}
    let retained := mask / 2 % 2 == 1
    check (trace.events == #[5,6,if retained then 8 else 7,9]) s!"cleanup order {mask}"
    let failedClose := !retained && mask / 4 % 2 == 1
    check (final.poisoned == (mask % 2 == 1 || retained || failedClose || mask / 8 % 2 == 1)
      && final.owner.isNone) s!"cleanup poison {mask}"
  IO.println "PASS: Lean device ownership, acquisition failures, 16 cleanup combinations, retain/poison and no retry"

end AudioDeviceTests
