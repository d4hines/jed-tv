prelude
import Init.System.IO
import TV.Volume

namespace TV.Volume.Native

@[extern "tv_volume_initial"] opaque initialWord : IO UInt64
@[extern "tv_volume_ready"] opaque readyWord : IO UInt64
@[extern "tv_volume_poll"] opaque pollWord : IO UInt64
@[extern "tv_volume_commit"] opaque commitWord (gain : UInt32) : IO UInt64
@[extern "tv_volume_publish"] opaque publish (gain : UInt32) : IO Unit

/-- Poll tag 0 is idle, 1 is external gain (high word), 2 is an evdev
key (code in high word, value in bits 8..31). Unknown tags are inert. -/
def decode (word : UInt64) : Event :=
  match (word &&& 255).toUInt32 with
  | 1 => .external (word >>> 32).toUInt32
  | 2 => .key (word >>> 32).toUInt32 ((word >>> 8) &&& 0xffffff).toUInt32
  | _ => .tick

def effects : Effects IO where
  poll := return decode (← pollWord)
  commit := fun gain => return (← commitWord gain) != 0
  publish

def packState (state : State) : UInt64 :=
  state.gain.toUInt64 ||| (state.saved.toUInt64 <<< 32)

def unpackState (word : UInt64) : State :=
  ⟨word.toUInt32, (word >>> 32).toUInt32⟩

/-- Initialize on the supervisor's thread. An all-ones word means the native
    setup failed; no valid calibrated state has this representation. -/
@[export tv_volume_start]
def start : IO UInt64 := do
  let state := initial (← initialWord).toUInt32
  if (← readyWord) == 0 then return 0xffffffffffffffff
  return packState state

/-- One finite volume step. PID 1 calls this while polling Core, including
    while its launcher is idle; no second thread enters the Lean runtime. -/
@[export tv_volume_tick]
def tick (word : UInt64) : IO UInt64 := do
  let state ← handle effects (unpackState word) (← effects.poll)
  return packState state

end TV.Volume.Native
