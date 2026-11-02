import TV.Volume.Native
open TV.Volume

structure Harness where
  writes : Array UInt32 := #[]
  shown : Array UInt32 := #[]
  succeed : Bool := true
  deriving Inhabited

def effects (r : IO.Ref Harness) : Effects IO where
  poll := pure .tick
  commit := fun gain => do
    r.modify fun h => { h with writes := h.writes.push gain }
    return (← r.get).succeed
  publish := fun gain => r.modify fun h => { h with shown := h.shown.push gain }

def ensure (ok : Bool) (name : String) : IO Unit :=
  unless ok do throw (IO.userError name)

def main : IO Unit := do
  let r ← IO.mkRef ({} : Harness)
  let e := effects r
  let mut state := initial 75000
  for _ in [:20] do state ← handle e state (.key 115 1)
  ensure (state.gain == 300000) "up saturates at calibrated ceiling"
  for _ in [:20] do state ← handle e state (.key 114 1)
  ensure (state.gain == 0) "each down press counts and saturates at zero"
  ensure ((← r.get).writes.size == 40 && (← r.get).shown.size == 40) "one write and publication per volume press"

  state ← handle e (initial 123456) (.key 113 1)
  ensure (state == ⟨0,123456⟩) "mute saves unsnapped external gain"
  r.set {}
  state ← handle e state (.key 113 2)
  state ← handle e state (.key 113 0)
  state ← handle e state (.key 42 1)
  ensure ((← r.get).writes.isEmpty && (← r.get).shown.isEmpty) "repeat mute, release and unrelated keys do nothing"
  state ← handle e state (.key 113 1)
  ensure (state.gain == 123456) "unmute restores exact saved gain"
  state ← handle e (initial 0) (.key 113 1)
  ensure (state.gain == defaultGain) "initial mute restores default"

  r.set { succeed := false }
  let before := initial 123456
  state ← handle e before (.key 113 1)
  ensure (state == before && (← r.get).shown.isEmpty) "failed write changes neither mute memory nor display"
  r.set {}
  state ← handle e ⟨0,123456⟩ (.external 999999)
  ensure (state == ⟨ceiling,123456⟩ && (← r.get).writes.isEmpty) "external update clamps without rewriting or losing mute memory"
  ensure ((← r.get).shown == #[ceiling]) "external gain publishes once"
  ensure ((proposal (initial 7500) 115 1).map (·.gain) == some 30000) "half-step rounds upward"
  ensure ((proposal (initial 7499) 115 1).map (·.gain) == some 15000) "below half-step rounds downward"

  for gain in [0,1,7499,7500,15000,75000,123456,292500,300000,4294967295] do
    for code in [113,114,115] do
      for value in [0,1,2] do
        if let some next := proposal (initial gain) code value then
          ensure (next.gain <= ceiling && next.saved <= ceiling) "all boundary cases stay within calibrated ceiling"
  ensure (TV.Volume.Native.decode ((123456 : UInt64) <<< 32 ||| 1) == .external 123456) "external ABI"
  ensure (TV.Volume.Native.decode ((115 : UInt64) <<< 32 ||| 514) == .key 115 2) "key ABI preserves repeat"
  ensure (TV.Volume.Native.decode 255 == .tick) "unknown poll tag inert"
  IO.println "PASS: Lean volume presses, limits, mute, external updates, write failure and scalar ABI"
