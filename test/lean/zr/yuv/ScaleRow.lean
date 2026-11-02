import ZrAsm.Sem
import ZrAsm.Yuv

/-!
# `zr_u32_scale_row` in the ARM model

Runs the very `Routine` that `zr-asm-gen` writes to the runtime's `tv.S`
(`ZrAsm.Yuv.scaleRow`) in `ZrAsm.Sem` and checks, for source and destination
widths around the video sizes (upscaling, downscaling, equal, odd ratios),
that `dst[x] = src[x * sw / w]` for the first `n` pixels, that nothing past
them is written, and that it returns with r4-r7 and sp as they were.

    cd src/lean/zr && lake env lean --run ../../../test/lean/zr/yuv/ScaleRow.lean
-/

open ZrAsm

def base : UInt32 := 0x10000
def retAddr : UInt32 := 0x7000000
def src : UInt32 := 0x100000
def dst : UInt32 := 0x200000
def sp0 : UInt32 := 0x300000

def pat (j : Nat) : UInt32 := (j.toUInt32 + 1) * 2654435761

/-- Source words, the two stack arguments at `sp0`, everything else zero. -/
def mem0 (sw q r : Nat) : UInt32 → UInt8 := fun a =>
  let word (v : UInt32) := ((v >>> (8 * (a &&& 3))) &&& 0xff).toUInt8
  if a ≥ src && a < src + 4 * sw.toUInt32 then word (pat ((a - src) / 4).toNat)
  else if a ≥ sp0 && a < sp0 + 4 then word q.toUInt32
  else if a ≥ sp0 + 4 && a < sp0 + 8 then word r.toUInt32
  else 0

def regs0 (n w : Nat) : List (Nat × UInt32) :=
  [(0, dst), (1, src), (2, n.toUInt32), (3, w.toUInt32), (4, 44), (5, 55), (6, 66), (7, 77), (13, sp0)]

def check (sw w n : Nat) : Option String := do
  let s0 : State :=
    { r := fun i => ((regs0 n w).find? (·.1 == i.val)).map (·.2) |>.getD (if i.val == 14 then retAddr else 0)
      n := false, z := false, c := false, v := false, cpsrRest := 0x10
      mem := mem0 sw (sw / w) (sw % w), cpIn := fun _ => 0, vfpIn := fun _ => 0, log := [] }
  let some p := layout base Yuv.scaleRow.body | return "layout failed"
  match run p (12 * n + 100) (s0.setR 15 base) with
  | none => return "outside the model"
  | some (false, _) => return "did not finish"
  | some (true, s) =>
    for x in [0:n] do
      let got := rd32 s.mem (dst + 4 * x.toUInt32)
      if got != pat (x * sw / w) then return s!"x = {x}: {got} != src[{x * sw / w}]"
    if rd32 s.mem (dst + 4 * n.toUInt32) != 0 then return "wrote past n"
    unless s.r 4 == 44 && s.r 5 == 55 && s.r 6 == 66 && s.r 7 == 77 && s.r 13 == sp0 && s.r 15 == retAddr do
      return "registers not restored"
    none

def cases : List (Nat × Nat × Nat) :=
  [(16, 16, 16), (640, 1280, 300), (854, 1280, 300), (1280, 854, 300), (1920, 1280, 300), (426, 1280, 300),
   (7, 5, 5), (5, 7, 7), (3, 1000, 1000), (1000, 3, 3), (1280, 1, 1), (1, 1280, 64), (1280, 720, 720)]

def main : IO UInt32 := do
  let mut fails := 0
  for (sw, w, n) in cases do
    if let some e := check sw w n then
      IO.println s!"scale-row: FAIL sw {sw} w {w} n {n}: {e}"
      fails := fails + 1
  IO.println s!"scale-row: {cases.length - fails}/{cases.length} cases {if fails == 0 then "PASS" else "FAIL"}"
  return (if fails == 0 then 0 else 1)
