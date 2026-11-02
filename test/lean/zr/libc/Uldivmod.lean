import ZrAsm.Sem
import ZrAsm.Libc

/-!
# `__aeabi_uldivmod` in the ARM model

Runs the very `Routine` that `zr-asm-gen app` writes into each app
(`ZrAsm.Libc.uldivmod`) in `ZrAsm.Sem` and checks, for edge cases (small
and full-width operands, denominators above the numerator, powers of two,
equal operands) and pseudo-random pairs of every pair of widths, that it
returns the quotient in r1:r0 and the remainder in r3:r2 as `Nat` division
gives them, with r4-r8 and sp restored.

    cd src/lean/zr && lake env lean --run ../../../test/lean/zr/libc/Uldivmod.lean
-/

open ZrAsm

def base : UInt32 := 0x10000
def retAddr : UInt32 := 0x7000000
def sp0 : UInt32 := 0x300000

def lo (x : Nat) : UInt32 := (x % 2^32).toUInt32
def hi (x : Nat) : UInt32 := (x / 2^32).toUInt32

def check (n d : Nat) : Option String := do
  let regs : List (Nat × UInt32) :=
    [(0, lo n), (1, hi n), (2, lo d), (3, hi d), (4, 44), (5, 55), (6, 66), (7, 77), (8, 88), (13, sp0)]
  let s0 : State :=
    { r := fun i => (regs.find? (·.1 == i.val)).map (·.2) |>.getD (if i.val == 14 then retAddr else 0)
      n := false, z := false, c := false, v := false, cpsrRest := 0x10
      mem := fun _ => 0, cpIn := fun _ => 0, vfpIn := fun _ => 0, log := [] }
  let some p := layout base Libc.uldivmod.body | return "layout failed"
  match run p 2000 (s0.setR 15 base) with
  | none => return "outside the model"
  | some (false, _) => return "did not finish"
  | some (true, s) =>
    let q := (s.r 0).toNat + 2^32 * (s.r 1).toNat
    let r := (s.r 2).toNat + 2^32 * (s.r 3).toNat
    unless q == n / d && r == n % d do return s!"got q {q} r {r}, want q {n / d} r {n % d}"
    unless s.r 4 == 44 && s.r 5 == 55 && s.r 6 == 66 && s.r 7 == 77 && s.r 8 == 88 && s.r 13 == sp0 &&
        s.r 15 == retAddr do
      return "registers not restored"
    none

/-- A 64-bit generator (splitmix64), for reproducible pseudo-random operands. -/
def mix (x : Nat) : Nat :=
  let x := (x + 0x9E3779B97F4A7C15) % 2^64
  let z := (x ^^^ (x >>> 30)) * 0xBF58476D1CE4E5B9 % 2^64
  let z := (z ^^^ (z >>> 27)) * 0x94D049BB133111EB % 2^64
  z ^^^ (z >>> 31)

def edges : List (Nat × Nat) :=
  let m := 2^64 - 1
  [(0, 1), (1, 1), (5, 7), (7, 5), (m, 1), (m, m), (m, m - 1), (m - 1, m), (m, 2^32), (m, 2^32 - 1),
   (m, 2^32 + 1), (2^63, 2^31), (2^63, 3), (2^32, 2^32), (2^32 - 1, 2^32), (123456789012345, 10),
   (10^19, 10), (10^19, 10^9), (2^40 + 12345, 2^20 + 7), (m, 2^63), (2^63 - 1, 2^63)]

/-- Pseudo-random pairs whose widths cover every combination of 1..64 bits
in steps of 7. -/
def randoms : List (Nat × Nat) := Id.run do
  let mut out := []
  let mut seed := 1
  for wn in [1:65:7] do
    for wd in [1:65:7] do
      seed := mix seed
      let n := seed % 2^wn
      seed := mix seed
      let d := (seed % 2^wd) + 1
      out := (n, d) :: out
  return out

def main : IO UInt32 := do
  let cases := edges ++ randoms
  let mut fails := 0
  for (n, d) in cases do
    if let some e := check n d then
      IO.println s!"uldivmod: FAIL {n} / {d}: {e}"
      fails := fails + 1
  IO.println s!"uldivmod: {cases.length - fails}/{cases.length} cases {if fails == 0 then "PASS" else "FAIL"}"
  return (if fails == 0 then 0 else 1)
