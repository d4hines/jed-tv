import Doom.Audio.Opl
/-!
# AudioCheck.OplTrace (host test support)

Deterministic pseudo-random OPL register traces and a trace runner, used to
compare `Doom.Audio.Opl` sample-for-sample against the C reference
(`AudioCheck/c/opl_ref.c`). Trace text format: one `frame reg value` triple
per line (decimal), sorted by frame.
-/

namespace AudioCheck.OplTrace

open Doom.Audio.Opl

structure Rng where
  s : UInt64

def Rng.next (r : Rng) : Rng × UInt32 :=
  let s := r.s * 6364136223846793005 + 1442695040888963407
  (⟨s⟩, (s >>> 33).toUInt32)

/-- Uniform-ish value in `[0, n)`. -/
def Rng.below (r : Rng) (n : Nat) : Rng × Nat :=
  let (r, x) := r.next
  (r, x.toNat % n)

/-- Valid operator offsets within a bank. -/
def opOffsets : Array Nat := #[0, 1, 2, 3, 4, 5, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21]

/--
One random register write. `phase` 0 = OPL2 only, 1 = also rhythm mode,
2 = OPL3 (second bank, 4-op, stereo bits).
-/
def randomWrite (r : Rng) (phase : Nat) : Rng × Nat × Nat := Id.run do
  let (r, fam) := r.below 100
  let (r, opi) := r.below opOffsets.size
  let (r, ch) := r.below 9
  let (r, v) := r.next
  let v := v.toNat % 256
  let (r, bankR) := r.below 2
  let bank := if phase >= 2 then bankR * 0x100 else 0
  let op := opOffsets[opi]!
  let (r, b2) := r.next
  let b2 := b2.toNat
  if fam < 12 then return (r, bank + 0x20 + op, v)
  else if fam < 26 then return (r, bank + 0x40 + op, (v &&& 0xc0) ||| (b2 % 0x28))
  else if fam < 38 then return (r, bank + 0x60 + op, v ||| 0x40)
  else if fam < 48 then return (r, bank + 0x80 + op, v)
  else if fam < 56 then return (r, bank + 0xe0 + op, v &&& 7)
  else if fam < 68 then return (r, bank + 0xa0 + ch, v)
  else if fam < 84 then
    -- key on/off with frequency high bits and block
    let key := if b2 % 10 < 6 then 0x20 else 0
    return (r, bank + 0xb0 + ch, key ||| (v &&& 0x1f))
  else if fam < 92 then return (r, bank + 0xc0 + ch, v)
  else if fam < 95 then
    if phase >= 1 then return (r, 0xbd, v) else return (r, 0xbd, v &&& 0xc0)
  else if fam < 97 then return (r, 0x08, v &&& 0x40)
  else if fam < 99 then
    if phase >= 2 then return (r, 0x104, v &&& 0x3f) else return (r, 0x40 + op, v)
  else
    if phase >= 2 then return (r, 0x105, 1) else return (r, 0x01, v)

/-- A random trace of `frames` frames (three phases of increasing feature use). -/
def genTrace (frames : Nat) (seed : UInt64) (bursty : Bool) : Array (Nat × Nat × Nat) := Id.run do
  let mut r : Rng := ⟨seed⟩
  let mut out : Array (Nat × Nat × Nat) := #[]
  let mut f := 0
  while f < frames do
    let phase := if f * 3 < frames then 0 else if f * 3 < frames * 2 then 1 else 2
    if phase == 2 && f * 3 < frames * 2 + 3 then
      out := out.push (f, 0x105, 1)
    let (r1, p) := r.below 1000
    r := r1
    -- occasional large burst (exercises write-buffer overflow) when bursty
    if bursty && f == frames / 2 then
      for _ in [0:1100] do
        let (r2, (reg, v)) := randomWrite r phase
        r := r2
        out := out.push (f, reg, v)
    else if p < 30 then
      let (r2, k) := r.below 12
      r := r2
      for _ in [0:k + 1] do
        let (r3, (reg, v)) := randomWrite r phase
        r := r3
        out := out.push (f, reg, v)
    f := f + 1
  return out

def traceToString (t : Array (Nat × Nat × Nat)) : String := Id.run do
  let mut s := ""
  for (f, reg, v) in t do
    s := s ++ s!"{f} {reg} {v}\n"
  return s

def parseTrace (s : String) : Array (Nat × Nat × Nat) := Id.run do
  let mut out := #[]
  for line in s.splitOn "\n" do
    match (line.splitOn " ").filter (· ≠ "") with
    | [a, b, c] =>
      match a.toNat?, b.toNat?, c.toNat? with
      | some f, some r, some v => out := out.push (f, r, v)
      | _, _, _ => pure ()
    | _ => pure ()
  return out

/--
Run a trace through the Lean emulator. `stream = false`: unbuffered writes
and native-rate `generate`; `true`: buffered writes and the built-in
resampler (Chocolate's path). Returns raw s16le stereo.
-/
def runTrace (trace : Array (Nat × Nat × Nat)) (stream : Bool) (rate frames : Nat) : ByteArray := Id.run do
  let mut c := Chip.reset rate
  let mut out := ByteArray.emptyWithCapacity (frames * 4)
  let mut ti := 0
  for f in [0:frames] do
    while ti < trace.size && (trace[ti]!).1 == f do
      let (_, reg, v) := trace[ti]!
      c := if stream then c.writeRegBuffered reg.toUInt32 v.toUInt32 else c.writeRegNow reg.toUInt32 v.toUInt32
      ti := ti + 1
    if stream then
      let (c', o) := c.renderStream out 1
      c := c'
      out := o
    else
      c := c.generate
      out := pushS16 (pushS16 out c.cur0) c.cur1
  return out

end AudioCheck.OplTrace
