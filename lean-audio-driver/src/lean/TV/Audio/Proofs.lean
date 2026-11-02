import Std.Tactic.BVDecide
import TV.Audio.Gain
import TV.Audio.Resample
import TV.Audio.Session

/- Proofs about the shipping audio driver, built by test/lean/tv/check-audio.py
   like TV/Proofs.lean; not linked into the TV. Three properties: the loudness
   ceiling, the resampler's exactness, and the cleanup order. Each theorem is
   about the definition the TV runs, not a copy. `bv_decide` and
   `native_decide` add their certificate axiom, as every TV/Proofs.lean theorem
   already does; the rest is propext, Classical.choice and Quot.sound. -/

namespace TV.Audio.Gain

/-- No sample the driver sends is louder than 0.3 of full scale: whatever the
app's boost, whatever the gain file says, every output lies in [-9830, 9830]. -/
theorem scale_never_exceeds_ceiling (boost gain : UInt32) (sample : Int16) :
    -9830 ≤ scale boost gain sample ∧ scale boost gain sample ≤ 9830 := by
  unfold scale ceiling unit
  bv_decide

/-- The bound is reached: full scale at the ceiling gain gives exactly 9830,
so no smaller bound would be true. -/
example : scale 1 300000 32767 = 9830 := by decide
example : scale 4 300000 (-32768) = -9830 := by decide

theorem zero_gain_is_silent (boost : UInt32) (sample : Int16) : scale boost 0 sample = 0 := by
  unfold scale ceiling unit
  bv_decide

/-- A gain file above the ceiling is treated as the ceiling. -/
theorem gain_above_ceiling_is_capped (boost gain : UInt32) (sample : Int16) (h : gain ≥ ceiling) :
    scale boost gain sample = scale boost ceiling sample := by
  by_cases hg : gain > ceiling
  · simp [scale, hg]
  · have : gain = ceiling := by unfold ceiling at *; bv_decide
    subst this; rfl

end TV.Audio.Gain

namespace TV.Audio.Resample

/-- With the phase below the rate, `step` is exact rational arithmetic: it
emits `(phase + 48000) / rate` frames and keeps `(phase + 48000) % rate`. -/
theorem step_is_rational_phase (rate phase : UInt32)
    (hr : rate = 44100 ∨ rate = 48000) (hp : phase < rate) :
    step rate phase = ((phase + 48000) / rate, (phase + 48000) % rate) := by
  rcases hr with rfl | rfl <;> simp only [step, outputRate]
  · by_cases h1 : phase + 48000 ≥ 44100 <;> by_cases h2 : phase + 48000 - 44100 ≥ 44100
      <;> simp only [h1, h2, ↓reduceIte, Prod.mk.injEq] <;> constructor <;> bv_decide
  · by_cases h1 : phase + 48000 ≥ 48000 <;> by_cases h2 : phase + 48000 - 48000 ≥ 48000
      <;> simp only [h1, h2, ↓reduceIte, Prod.mk.injEq] <;> constructor <;> bv_decide

/-- At phase zero nothing is interpolated: the current sample comes out as is. -/
theorem mix_at_phase_zero_is_identity (previous current : Int16) :
    mix previous current 0 = current := by
  unfold mix outputRate
  bv_decide (config := { timeout := 120 })

theorem toNat_bridge (phase rate : UInt32) (hr : rate = 44100 ∨ rate = 48000) (hp : phase < rate) :
    ((phase + 48000) / rate).toNat = (phase.toNat + 48000) / rate.toNat
    ∧ ((phase + 48000) % rate).toNat = (phase.toNat + 48000) % rate.toNat := by
  have hlt : phase.toNat < rate.toNat := UInt32.lt_iff_toNat_lt.mp hp
  rcases hr with rfl | rfl
    <;> simp only [UInt32.toNat_div, UInt32.toNat_mod, UInt32.toNat_add, UInt32.reduceToNat] at *
    <;> omega

/-- One input frame keeps the rate, moves the phase as `step` says, and
appends 4 bytes per emitted frame. -/
theorem feed_rate (c : Converter) (f : Frame) (out : ByteArray) : (feed c f out).1.rate = c.rate := by
  simp [feed]

theorem feed_phase (c : Converter) (f : Frame) (out : ByteArray)
    (hr : c.rate = 44100 ∨ c.rate = 48000) (hp : c.phase < c.rate) :
    (feed c f out).1.phase.toNat = (c.phase.toNat + 48000) % c.rate.toNat := by
  simp [feed, step_is_rational_phase c.rate c.phase hr hp, (toNat_bridge c.phase c.rate hr hp).2]

theorem feed_size (c : Converter) (f : Frame) (out : ByteArray)
    (hr : c.rate = 44100 ∨ c.rate = 48000) (hp : c.phase < c.rate) :
    (feed c f out).2.size = out.size + 4 * ((c.phase.toNat + 48000) / c.rate.toNat) := by
  have hs := step_is_rational_phase c.rate c.phase hr hp
  have hb := toNat_bridge c.phase c.rate hr hp
  have hlt := UInt32.lt_iff_toNat_lt.mp hp
  have hq : (c.phase + 48000) / c.rate = 0 ∨ (c.phase + 48000) / c.rate = 1
      ∨ (c.phase + 48000) / c.rate = 2 := by
    have : ((c.phase + 48000) / c.rate).toNat ≤ 2 := by
      rw [hb.1]; rcases hr with h | h <;> simp only [h, UInt32.reduceToNat] at * <;> omega
    rcases (show ((c.phase + 48000) / c.rate).toNat = 0 ∨ ((c.phase + 48000) / c.rate).toNat = 1
        ∨ ((c.phase + 48000) / c.rate).toNat = 2 by omega) with h | h | h
    · exact Or.inl (UInt32.toNat.inj (by simpa using h))
    · exact Or.inr (Or.inl (UInt32.toNat.inj (by simpa using h)))
    · exact Or.inr (Or.inr (UInt32.toNat.inj (by simpa using h)))
  rw [← hb.1]
  rcases hq with h | h | h <;> simp [feed, hs, h, emitFrame, Gain.pushInt16]

/-- After feeding any frames the phase is still below the rate, the output grew
by 4 bytes per `(phase + n * 48000) / rate` frames, and the phase is
`(phase + n * 48000) % rate`. -/
theorem convertFrames_facts (frames : List Frame) :
    ∀ (c : Converter) (out : ByteArray), (c.rate = 44100 ∨ c.rate = 48000) → c.phase < c.rate →
      (convertFrames c frames out).1.rate = c.rate
      ∧ (convertFrames c frames out).1.phase.toNat = (c.phase.toNat + frames.length * 48000) % c.rate.toNat
      ∧ (convertFrames c frames out).2.size
          = out.size + 4 * ((c.phase.toNat + frames.length * 48000) / c.rate.toNat) := by
  induction frames with
  | nil =>
    intro c out _ hp
    have := UInt32.lt_iff_toNat_lt.mp hp
    simp only [convertFrames, List.foldl_nil, List.length_nil, Nat.zero_mul, Nat.add_zero,
      Nat.mod_eq_of_lt this, Nat.div_eq_of_lt this, Nat.mul_zero, and_self]
  | cons f fs ih =>
    intro c out hr hp
    have hrate := feed_rate c f out
    have hphase := feed_phase c f out hr hp
    have hsize := feed_size c f out hr hp
    have hr' : (feed c f out).1.rate = 44100 ∨ (feed c f out).1.rate = 48000 := by rw [hrate]; exact hr
    have hp' : (feed c f out).1.phase < (feed c f out).1.rate := by
      rw [UInt32.lt_iff_toNat_lt, hphase, hrate]
      rcases hr with h | h <;> simp only [h, UInt32.reduceToNat] <;> omega
    have step : convertFrames c (f :: fs) out = convertFrames (feed c f out).1 fs (feed c f out).2 := by
      simp [convertFrames]
    obtain ⟨r, p, s⟩ := ih (feed c f out).1 (feed c f out).2 hr' hp'
    rw [step]
    refine ⟨by rw [r, hrate], ?_, ?_⟩
    · rw [p, hphase, hrate]
      rcases hr with h | h <;> simp only [h, UInt32.reduceToNat, List.length_cons] <;> omega
    · rw [s, hsize, hphase, hrate]
      rcases hr with h | h <;> simp only [h, UInt32.reduceToNat, List.length_cons] <;> omega

/-- `n` input frames into a fresh converter give exactly `n * 48000 / rate`
output frames: 7 seconds in, 7 seconds out. -/
theorem output_frames_match_rate (rate channels : UInt32) (c : Converter) (frames : List Frame)
    (h : Converter.new rate channels = some c) :
    (convertFrames c frames ByteArray.empty).2.size = 4 * (frames.length * 48000 / rate.toNat) := by
  have hc : c.rate = rate ∧ c.phase = 0 ∧ (rate = 44100 ∨ rate = 48000) := by
    unfold Converter.new at h
    split at h
    · rename_i hcond
      simp only [Option.some.injEq] at h
      subst h
      refine ⟨rfl, rfl, ?_⟩
      simp only [Bool.and_eq_true, Bool.or_eq_true, beq_iff_eq] at hcond
      exact hcond.1
    · exact absurd h (by simp)
  have facts := convertFrames_facts frames c ByteArray.empty (hc.1 ▸ hc.2.2)
    (by rw [hc.2.1]; rcases hc.2.2 with h | h <;> rw [hc.1, h] <;> decide)
  rw [facts.2.2, hc.2.1, hc.1]
  simp

/-- The hypothesis is satisfiable: 44.1 kHz mono is a converter. -/
example : Converter.new 44100 1 = some { rate := 44100, channels := 1, phase := 0, previous := none } := rfl

/-- At 48 kHz nothing is interpolated: each input frame comes out once, unchanged. -/
theorem feed_48k_is_identity (c : Converter) (frame : Frame) (out : ByteArray)
    (hr : c.rate = 48000) (hp : c.phase = 0) :
    feed c frame out = ({ c with phase := 0, previous := some frame },
      Gain.pushInt16 (Gain.pushInt16 out frame.1) frame.2) := by
  simp [feed, step, outputRate, emitFrame, hr, hp, mix_at_phase_zero_is_identity]

/-- Splitting the input into chunks never changes the output: the conversion
is a fold, so feeding `a` then `b` is feeding `a ++ b`. -/
theorem chunks_do_not_change_output (c : Converter) (a b : List Frame) (out : ByteArray) :
    (let (c', out') := convertFrames c a out; convertFrames c' b out')
      = convertFrames c (a ++ b) out := by
  unfold convertFrames
  simp [List.foldl_append]

/-- `consumedNat` undoes `n * 48000 / rate`: exactly `n` input frames are
reported consumed once their output has played, and fewer before. -/
theorem consumed_is_inverse (rate n : Nat) (hr : rate = 44100 ∨ rate = 48000) :
    consumedNat (n * 48000 / rate) rate n = n := by
  unfold consumedNat
  rcases hr with rfl | rfl <;> omega

theorem consumed_is_strict (rate n : Nat) (hr : rate = 44100 ∨ rate = 48000)
    (h : 0 < n * 48000 / rate) :
    consumedNat (n * 48000 / rate - 1) rate n < n := by
  unfold consumedNat
  rcases hr with rfl | rfl <;> omega

end TV.Audio.Resample

/-! The cleanup order is checked on every combination of what an open took and
which mechanism fails, by running the shipping `cleanup` under a mock that only
records what it asks for. -/
namespace TV.Audio.Session.Order

/-- What cleanup can ask the mechanisms to do. -/
inductive Op
  | gainRestore | routeDisconnect | pcmStop | pcmClose | retain | poison | release
  deriving DecidableEq, Repr

/-- Everything cleanup's behaviour depends on: what the open had taken, and
which mechanism fails. -/
structure Config where
  started : Bool
  attached : Bool
  changed : Bool
  unresolved : Bool
  failGainRestore : Bool
  failSnapshot : Bool
  failDetach : Bool
  failClose : Bool
  deriving Repr, DecidableEq

def Config.all : List Config :=
  let b := [false, true]
  b.flatMap fun a => b.flatMap fun b' => b.flatMap fun c => b.flatMap fun d =>
  b.flatMap fun e => b.flatMap fun f => b.flatMap fun g => b.map fun h => ⟨a, b', c, d, e, f, g, h⟩

structure Log where
  cfg : Config
  ops : List Op := []
  /-- Our PCM is an input of the speaker. -/
  connected : Bool
  /-- The speaker sits at 0 dB while we play. -/
  gain : VolumeDb := ⟨0, 0⟩

abbrev M := StateM Log
def record (op : Op) : M Unit := modify fun s => { s with ops := s.ops ++ [op] }
def fault : Fault := .os 5

/-- The mechanisms as cleanup sees them. Anything cleanup never calls is inert. -/
def mock : Effects M where
  recvCommand := pure none
  recvExact := fun _ => pure none
  send := fun _ => pure true
  gain := pure 0
  now := pure 0
  sleep := fun _ => pure ()
  log := fun _ => pure ()
  acquire := pure 0
  release := record .release
  poison := record .poison
  pcmOpen := pure (.ok 1)
  pcmStart := fun _ => pure (.ok ())
  pcmWrite := fun _ _ _ _ => pure (.ok 0)
  pcmQueued := fun _ => pure (.ok 0)
  pcmStop := fun _ => do record .pcmStop; pure (.ok ())
  pcmClose := fun _ => do
    record .pcmClose
    return if (← get).cfg.failClose then .error fault else .ok ()
  pcmRetain := fun _ => record .retain
  aoutSpeaker := pure (.ok 0x17000001)
  aoutCount := fun _ => do
    let s ← get
    return if s.cfg.failSnapshot then .error fault else .ok (if s.connected then 1 else 0)
  aoutConnected := fun _ _ => do
    let s ← get
    return if s.cfg.failSnapshot then .error fault else .ok (if s.connected then #[1] else #[])
  aoutConnect := fun _ _ => do modify fun s => { s with connected := true }; pure (.ok ())
  aoutDisconnect := fun _ _ => do
    record .routeDisconnect
    if (← get).cfg.failDetach then return .error fault
    modify fun s => { s with connected := false }
    return .ok ()
  aoutVolume := fun _ => return .ok (← get).gain
  aoutSetVolume := fun _ v => do
    record .gainRestore
    if (← get).cfg.failGainRestore then return .error fault
    modify fun s => { s with gain := v }
    return .ok ()

def original : VolumeDb := ⟨-47, 0⟩

/-- The track an open with these choices leaves behind. -/
def track (cfg : Config) : Track :=
  { pcm := 1, started := cfg.started, speaker := 0x17000001, attached := cfg.attached,
    volume := if cfg.changed then some (original, true) else none,
    routeUnresolved := cfg.unresolved,
    converter := { rate := 48000, channels := 2, phase := 0, previous := none },
    rate := 48000, channels := 2, inputWritten := 0, outputWritten := 0 }

/-- Run the shipping cleanup under the mock: its result and what it asked for. -/
def run (cfg : Config) : Except Fault Unit × List Op :=
  let (result, log) := (cleanup mock (track cfg)).run { cfg, connected := cfg.attached }
  (result, log.ops)

/-- `a` comes before `b` whenever both happen. -/
def before (ops : List Op) (a b : Op) : Bool :=
  match ops.idxOf? a, ops.idxOf? b with
  | some i, some j => i < j
  | _, _ => true

/-- The route stays unresolved when it already was, or when disconnecting it
fails or cannot be verified. -/
def routeStaysUnresolved (cfg : Config) : Bool :=
  cfg.unresolved || (cfg.attached && (cfg.failDetach || cfg.failSnapshot))

def failed (cfg : Config) : Bool :=
  routeStaysUnresolved cfg || (cfg.changed && cfg.failGainRestore)
    || (!routeStaysUnresolved cfg && cfg.failClose)

/-- The cleanup contract on one configuration: gain before route before PCM
stop before PCM close; the lease released exactly once, last; what was taken
is given back once; an unresolved route keeps the PCM open and retained,
otherwise the PCM is stopped (if started) and closed, once; any failure is
reported and poisons the lease, and success does neither. -/
def CleanupInOrder (cfg : Config) : Bool :=
  let (result, ops) := run cfg
  before ops .gainRestore .routeDisconnect && before ops .routeDisconnect .pcmStop
    && before ops .pcmStop .pcmClose && before ops .routeDisconnect .pcmClose
    && before ops .gainRestore .pcmClose
  && ops.count .release == 1 && ops.getLast? == some .release
  && ops.count .gainRestore == (if cfg.changed then 1 else 0)
  && ops.count .routeDisconnect == (if cfg.attached && !cfg.failSnapshot then 1 else 0)
  && (if routeStaysUnresolved cfg then
        ops.count .pcmClose == 0 && ops.count .pcmStop == 0 && ops.count .retain == 1
      else ops.count .pcmClose == 1 && ops.count .pcmStop == (if cfg.started then 1 else 0)
        && ops.count .retain == 0)
  && (if failed cfg then result matches .error _ && ops.count .poison == 1
      else result matches .ok _ && ops.count .poison == 0)

/-- All 256 configurations. The kernel cannot unfold core's `Array` loops, so
this runs the compiled code, like `bv_decide`'s certificate check. -/
theorem cleanup_in_order : Config.all.all CleanupInOrder = true := by native_decide

/-- Two of the traces, as examples of what the contract allows. -/
example : (run ⟨true, true, true, false, false, false, false, false⟩).2
    = [.gainRestore, .routeDisconnect, .pcmStop, .pcmClose, .release] := by native_decide
example : (run ⟨true, true, true, false, false, false, true, false⟩).2
    = [.gainRestore, .routeDisconnect, .poison, .retain, .release] := by native_decide

/-- The contract rejects a cleanup that closed the PCM after releasing the lease. -/
example : before [.pcmClose, .release] .pcmClose .release = true := rfl
example : ([.release, .pcmClose] : List Op).getLast? ≠ some .release := by decide

/-- The mechanisms of a TV whose lease is poisoned: `acquire` answers 2. -/
def poisoned : Effects M := { mock with acquire := pure 2 }

/-- After a failed cleanup the next open is refused, and nothing is touched. -/
theorem poisoned_lease_refuses_open (s : Log) :
    (openStream poisoned 48000 2).run s
      = (.error (.busy "direct PCM cleanup failed; explicit recovery required before reuse"), s) := rfl

/-- A lease owned by another connection is refused the same way. -/
theorem owned_lease_refuses_open (s : Log) :
    (openStream { mock with acquire := pure 1 } 48000 2).run s
      = (.error (.busy "direct PCM is already owned"), s) := rfl

/-- The format is checked before the lease: an unsupported rate never touches it. -/
theorem unsupported_format_refuses_open (s : Log) :
    (openStream poisoned 32000 2).run s
      = (.error (.invalid "direct audio supports 44.1/48k mono/stereo"), s) := rfl

end TV.Audio.Session.Order

#print axioms TV.Audio.Gain.scale_never_exceeds_ceiling
#print axioms TV.Audio.Resample.output_frames_match_rate
#print axioms TV.Audio.Resample.chunks_do_not_change_output
#print axioms TV.Audio.Session.Order.cleanup_in_order
#print axioms TV.Audio.Session.Order.poisoned_lease_refuses_open
