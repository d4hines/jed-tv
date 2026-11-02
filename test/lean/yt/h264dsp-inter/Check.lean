import Yt.Host.H264Kernels
import Yt.H264.Inter
import Yt.H264.Filter

/-! Host-only (upstream Lean runtime): compare `Yt.H264.Inter` and
`Yt.H264.Filter` byte for byte with FFmpeg 9.0.1's C on generated vectors.

    lake exe check [VECTORS]

`VECTORS` defaults to `assets/build/lean-h264-inter/vectors.bin` of this
worktree, written by `assets/build/lean-h264-inter/build.sh` (git-ignored):
FFmpeg's h264qpel, h264chroma, h264dsp and videodsp (emulated_edge_mc)
templates at BIT_DEPTH 8, compiled with clang, run on random inputs (the
record format is described in gen.c there). Each case runs the specified
`Int` entry point on the case's input buffer and compares the whole output
buffer, so bytes outside the block must be untouched too. Prints per-function
counts (cases, bytes, combinations covered) and exits 1 on any mismatch. -/

open Yt.H264

def rdU (b : ByteArray) (p : Nat) : UInt32 :=
  b[p]!.toUInt32 ||| (b[p + 1]!.toUInt32 <<< 8) ||| (b[p + 2]!.toUInt32 <<< 16) ||| (b[p + 3]!.toUInt32 <<< 24)

def rdN (b : ByteArray) (p : Nat) : Nat := (rdU b p).toNat

def rdI (b : ByteArray) (p : Nat) : Int :=
  let u := rdU b p
  if u ≥ 0x80000000 then Int.negSucc (0xFFFFFFFF - u).toNat else Int.ofNat u.toNat

structure Stat where
  name : String
  cases : Nat := 0
  bytes : Nat := 0
  bad : Nat := 0
  combos : Array Nat
  extra : Array Nat := #[0, 0, 0]
deriving Inhabited

def Stat.add (s : Stat) (combo : Nat) (n : Nat) (ok : Bool) : Stat :=
  { s with cases := s.cases + 1, bytes := s.bytes + n, bad := if ok then s.bad else s.bad + 1,
           combos := s.combos.modify combo (· + 1) }

def Stat.mark (s : Stat) (i : Nat) : Stat := { s with extra := s.extra.modify i (· + 1) }

def firstDiff (a b : ByteArray) : Option Nat := Id.run do
  if a.size != b.size then return some 0
  for i in [0:a.size] do
    if a[i]! != b[i]! then return some i
  return none

def lumaIdx (n : Nat) : Nat := if n == 16 then 0 else if n == 8 then 1 else 2
def chromaIdx (n : Nat) : Nat := if n == 8 then 0 else if n == 4 then 1 else 2
def wIdx (n : Nat) : Nat := if n == 16 then 0 else if n == 8 then 1 else if n == 4 then 2 else 3

/-- How far a block position is from needing no edge replication:
0 inside, 1 clamped within 64 samples, 2 farther. -/
def where_ (x : Int) (w n : Nat) (lo hi : Int) : Nat :=
  if x ≥ lo && x + w + hi ≤ n then 0
  else if x ≥ -64 && x ≤ n + 64 then 1 else 2

def main (args : List String) : IO UInt32 := do
  let path := args.headD "../../../../assets/build/lean-h264-inter/vectors.bin"
  let v ← IO.FS.readBinFile path
  if v.size < 8 || v.extract 0 8 != "H264IV01".toUTF8 then
    IO.eprintln s!"{path}: not a vector file"; return 1
  let mut st : Array Stat := #[
    { name := "lumaMc", combos := Array.replicate 288 0 },
    { name := "chromaMc", combos := Array.replicate 1152 0 },
    { name := "weight", combos := Array.replicate 128 0 },
    { name := "biweight", combos := Array.replicate 128 0 },
    { name := "Filter.luma", combos := Array.replicate 2 0 },
    { name := "Filter.lumaIntra", combos := Array.replicate 2 0 },
    { name := "Filter.chroma", combos := Array.replicate 2 0 },
    { name := "Filter.chromaIntra", combos := Array.replicate 2 0 }]
  let mut ref := ByteArray.empty
  let mut refW := 0
  let mut refH := 0
  let mut refStride := 0
  let mut p := 8
  let mut shown := 0
  let mut done := false
  while !done do
    let tag := rdN v p
    p := p + 4
    let f (k : Nat) := rdN v (p + 4 * k)
    let fi (k : Nat) := rdI v (p + 4 * k)
    let mut report : Option (Nat × String × ByteArray × ByteArray) := none
    match tag with
    | 1 =>
      refW := f 0; refH := f 1; refStride := f 2
      ref := v.extract (p + 12) (p + 12 + refStride * refH)
      p := p + 12 + refStride * refH
    | 2 | 3 =>
      let (x, y, dx, dy, w, h, avg) := (fi 0, fi 1, f 2, f 3, f 4, f 5, f 6 != 0)
      let (dstOff, dstStride, len) := (f 7, f 8, f 9)
      let din := v.extract (p + 40) (p + 40 + len)
      let want := v.extract (p + 40 + len) (p + 40 + 2 * len)
      p := p + 40 + 2 * len
      let luma := tag == 2
      let got :=
        if luma then Inter.lumaMc din dstOff dstStride ref refStride refW refH x y dx dy w h avg
        else Inter.chromaMc din dstOff dstStride ref refStride refW refH x y dx dy w h avg
      let ok := got == want
      let i := if luma then 0 else 1
      let combo :=
        if luma then lumaIdx w + 3 * lumaIdx h + 9 * dx + 36 * dy + 144 * (if avg then 1 else 0)
        else chromaIdx w + 3 * chromaIdx h + 9 * dx + 72 * dy + 576 * (if avg then 1 else 0)
      let (lo, hi) : Int × Nat := if luma then (2, 3) else (0, 1)
      let wx := where_ x w refW lo hi
      let wy := where_ y h refH lo hi
      st := st.modify i fun s => (s.add combo len ok).mark (max wx wy)
      if !ok then
        report := some (i, s!"x={x} y={y} dx={dx} dy={dy} w={w} h={h} avg={avg} ref={refW}x{refH}/{refStride} off={dstOff} stride={dstStride}", got, want)
    | 4 =>
      let (off, stride, w, h, ld, wt, o, len) := (f 0, f 1, f 2, f 3, f 4, fi 5, fi 6, f 7)
      let din := v.extract (p + 32) (p + 32 + len)
      let want := v.extract (p + 32 + len) (p + 32 + 2 * len)
      p := p + 32 + 2 * len
      let got := Inter.weight din off stride w h ld wt o
      let ok := got == want
      st := st.modify 2 fun s => s.add (wIdx w + 4 * wIdx h + 16 * ld) len ok
      if !ok then report := some (2, s!"w={w} h={h} log2Denom={ld} weight={wt} offset={o}", got, want)
    | 5 =>
      let (dstOff, dstStride, srcOff, srcStride, w, h, ld) := (f 0, f 1, f 2, f 3, f 4, f 5, f 6)
      let (w0, w1, o, dlen, slen) := (fi 7, fi 8, fi 9, f 10, f 11)
      let q := p + 48
      let din := v.extract q (q + dlen)
      let src := v.extract (q + dlen) (q + dlen + slen)
      let want := v.extract (q + dlen + slen) (q + 2 * dlen + slen)
      p := q + 2 * dlen + slen
      let got := Inter.biweight din dstOff dstStride src srcOff srcStride w h ld w0 w1 o
      let ok := got == want
      let implicit := ld == 5 && w0 + w1 == 64 && o == 0
      st := st.modify 3 fun s =>
        let s := s.add (wIdx w + 4 * wIdx h + 16 * ld) dlen ok
        if implicit then s.mark 0 else s
      if !ok then report := some (3, s!"w={w} h={h} log2Denom={ld} w0={w0} w1={w1} offset={o}", got, want)
    | 6 =>
      let (kind, off, xs, ys, alpha, beta) := (f 0, f 1, f 2, f 3, f 4, f 5)
      let tc0 : Array Int := #[fi 6, fi 7, fi 8, fi 9]
      let len := f 10
      let din := v.extract (p + 44) (p + 44 + len)
      let want := v.extract (p + 44 + len) (p + 44 + 2 * len)
      p := p + 44 + 2 * len
      let got := match kind with
        | 0 => Filter.luma din off xs ys alpha beta tc0
        | 1 => Filter.lumaIntra din off xs ys alpha beta
        | 2 => Filter.chroma din off xs ys alpha beta tc0
        | _ => Filter.chromaIntra din off xs ys alpha beta
      let ok := got == want
      let i := 4 + kind
      st := st.modify i fun s =>
        let s := s.add (if xs == 1 then 0 else 1) len ok
        if got != din then s.mark 0 else s
      if !ok then report := some (i, s!"off={off} xstride={xs} ystride={ys} alpha={alpha} beta={beta} tc0={tc0}", got, want)
    | _ => done := true
    if let some (i, what, got, want) := report then
      if shown < 20 then
        shown := shown + 1
        let d := (firstDiff got want).getD 0
        IO.println s!"MISMATCH {st[i]!.name} {what}: byte {d}: got {got[d]!} want {want[d]!}"
  let mut cases := 0
  let mut bad := 0
  for s in st do
    let covered := s.combos.foldl (fun n c => if c > 0 then n + 1 else n) 0
    let least := s.combos.foldl min (s.combos[0]!)
    let note :=
      if s.name == "lumaMc" || s.name == "chromaMc" then
        s!", positions inside/clamped/far {s.extra[0]!}/{s.extra[1]!}/{s.extra[2]!}"
      else if s.name == "biweight" then s!", implicit-weight cases {s.extra[0]!}"
      else if s.name.startsWith "Filter" then s!", cases that changed samples {s.extra[0]!}"
      else ""
    IO.println s!"{s.name}: {s.cases} cases, {s.bytes} bytes compared, {covered}/{s.combos.size} combinations (each ≥ {least}){note}, mismatches {s.bad}"
    cases := cases + s.cases
    bad := bad + s.bad
  IO.println s!"total: {cases} cases, {bad} mismatches"
  return if bad == 0 && cases > 0 then 0 else 1
