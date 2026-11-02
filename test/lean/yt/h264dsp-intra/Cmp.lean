import Yt.Host.H264Kernels
import Yt.H264.Intra
import Yt.H264.Transform

/-! Host-only: compare `Yt.H264.Intra` / `Yt.H264.Transform` with FFmpeg
9.0.1's C byte for byte. Reads reference records on stdin, produced by the
FFmpeg C harness (FFmpeg's `h264pred_template.c` through `ff_h264_pred_init`
and `h264idct_template.c`, at 8 bits) kept git-ignored under
`assets/build/lean-h264-intra/harness/` (`./build.sh`, then
`./ref SEED SCALE | lake exe h264dspcmp`). Each record carries the inputs
(plane before, block position, mode, flags, coefficients, qmul) and
FFmpeg's output (the whole plane after, or the DC outputs); the Lean
function is run on the same inputs and the results compared exactly
(whole plane, so writes outside the block are caught too).

Record (little-endian): `u8 kind, u8 mode, u8 flags, u8 ncoef, u32 stride,
u32 off, u32 size, i32 qmul, i16 coef[ncoef], u8 before[size],
u8 after[size], i16 out[ncoef if kind ≥ 8]`. Kinds: 0 pred4x4,
1 pred8x8l, 2 pred16x16, 3 pred8x8, 4 idct4Add, 5 idct8Add, 6 idct4DcAdd,
7 idct8DcAdd, 8 lumaDcDequantIdct, 9 chromaDcDequantIdct.

To exercise the documented input conventions, every third transform record
has multiples of 65536 added to its coefficients (16-bit wrap), every fourth
4x4/8x8/DC transform record drops its trailing zero coefficients (missing
ones count as 0) and every fifth DC record has 2^32 added to or subtracted
from qmul (32-bit wrap). -/

open Yt.H264

structure Rd where
  h : IO.FS.Stream
  buf : ByteArray
  pos : Nat

/-- Ensure `n` unread bytes (false at end of input). -/
partial def Rd.fill (r : Rd) (n : Nat) : IO (Rd × Bool) := do
  if r.buf.size - r.pos ≥ n then return (r, true)
  let more ← r.h.read (USize.ofNat (max n (1 <<< 22)))
  if more.isEmpty then return (r, false)
  Rd.fill { r with buf := r.buf.extract r.pos r.buf.size ++ more, pos := 0 } n

def u32 (b : ByteArray) (i : Nat) : Nat :=
  b[i]!.toNat ||| (b[i+1]!.toNat <<< 8) ||| (b[i+2]!.toNat <<< 16) ||| (b[i+3]!.toNat <<< 24)

def le16 (b : ByteArray) (i : Nat) : Int :=
  let v := b[i]!.toNat ||| (b[i+1]!.toNat <<< 8)
  if v ≥ 32768 then Int.ofNat v - 65536 else Int.ofNat v

def s32 (b : ByteArray) (i : Nat) : Int :=
  let v := u32 b i
  if v ≥ 2147483648 then Int.ofNat v - 4294967296 else Int.ofNat v

def kindName : Nat → String
  | 0 => "pred4x4" | 1 => "pred8x8l" | 2 => "pred16x16" | 3 => "pred8x8"
  | 4 => "idct4Add" | 5 => "idct8Add" | 6 => "idct4DcAdd" | 7 => "idct8DcAdd"
  | 8 => "lumaDcDequantIdct" | 9 => "chromaDcDequantIdct" | _ => "?"

/-- Per (kind, mode, flags) slot: cases, failures. -/
def slot (kind mode flags : Nat) : Nat := (kind * 16 + mode) * 4 + flags

def main : IO UInt32 := do
  let mut r : Rd := { h := ← IO.getStdin, buf := ByteArray.empty, pos := 0 }
  let mut cases : Array Nat := Array.replicate (10 * 16 * 4) 0
  let mut fails : Array Nat := Array.replicate (10 * 16 * 4) 0
  let mut wrapped := 0
  let mut changed := 0
  let mut shortened := 0
  let mut shown := 0
  let mut nrec := 0
  repeat
    let (r1, ok) ← r.fill 20
    r := r1
    if !ok then break
    let b := r.buf
    let p := r.pos
    let kind := b[p]!.toNat
    let mode := b[p+1]!.toNat
    let flags := b[p+2]!.toNat
    let nc := b[p+3]!.toNat
    let stride := u32 b (p+4)
    let off := u32 b (p+8)
    let size := u32 b (p+12)
    let qmul0 := s32 b (p+16)
    let nout := if kind ≥ 8 then nc else 0
    let total := 20 + 2 * nc + 2 * size + 2 * nout
    let (r2, ok2) ← r.fill total
    r := r2
    if !ok2 then
      IO.eprintln "truncated record"
      return 2
    let b := r.buf
    let p := r.pos
    -- Coefficients, perturbed by multiples of 65536 on every third transform record.
    let wrap := kind ≥ 4 && nrec % 3 == 1
    let mut c : Array Int := Array.emptyWithCapacity nc
    for i in [0:nc] do
      let k : Int := Int.ofNat ((nrec * 7 + i * 13) % 7) - 3
      c := c.push (le16 b (p + 20 + 2 * i) + (if wrap then k * 65536 else 0))
    -- Every fourth transform record drops its trailing zero coefficients
    -- (missing coefficients count as 0).
    if (kind == 4 || kind == 5 || kind ≥ 8) && nrec % 4 == 3 then
      while c.size > 0 && c.back! == 0 do
        c := c.pop
      shortened := shortened + 1
    let qmul : Int :=
      if kind < 8 || nrec % 5 != 2 then qmul0
      else if nrec % 2 == 0 then qmul0 + 4294967296 else qmul0 - 4294967296
    if wrap || qmul != qmul0 then wrapped := wrapped + 1
    let pb := p + 20 + 2 * nc
    let before := b.extract pb (pb + size)
    let after := b.extract (pb + size) (pb + 2 * size)
    if before.data != after.data then changed := changed + 1
    let ok : Bool ←
      if kind < 8 then do
        let res := match kind with
          | 0 => Intra.pred4x4 before off stride mode (flags &&& 1 != 0)
          | 1 => Intra.pred8x8l before off stride mode (flags &&& 2 != 0) (flags &&& 1 != 0)
          | 2 => Intra.pred16x16 before off stride mode
          | 3 => Intra.pred8x8 before off stride mode
          | 4 => Transform.idct4Add before off stride c
          | 5 => Transform.idct8Add before off stride c
          | 6 => Transform.idct4DcAdd before off stride c[0]!
          | _ => Transform.idct8DcAdd before off stride c[0]!
        if res.data == after.data then pure true
        else do
          if shown < 10 then
            shown := shown + 1
            let mut diffs := ""
            let mut nd := 0
            for i in [0:size] do
              if res[i]! != after[i]! then
                if nd < 6 then
                  let (x, y) := ((i - off) % stride, (i - off) / stride)
                  diffs := diffs ++ s!" @({x},{y}) lean={res[i]!} ff={after[i]!}"
                nd := nd + 1
            IO.eprintln (s!"MISMATCH record {nrec} {kindName kind} mode={mode} flags={flags} " ++
              s!"stride={stride} off={off}: {nd} bytes{diffs}")
          pure false
      else do
        let res := if kind == 8 then Transform.lumaDcDequantIdct c qmul
          else Transform.chromaDcDequantIdct c qmul
        let po := pb + 2 * size
        let mut exp : Array Int := Array.emptyWithCapacity nout
        for i in [0:nout] do
          exp := exp.push (le16 b (po + 2 * i))
        if res == exp then pure true
        else do
          if shown < 10 then
            shown := shown + 1
            IO.eprintln
              s!"MISMATCH record {nrec} {kindName kind} qmul={qmul} in={c} lean={res} ff={exp}"
          pure false
    let sl := slot kind mode flags
    cases := cases.modify sl (· + 1)
    if !ok then fails := fails.modify sl (· + 1)
    r := { r with pos := p + total }
    nrec := nrec + 1
  -- Summary: one line per kind, with the modes/flags covered.
  let mut allCases := 0
  let mut allFails := 0
  for kind in [0:10] do
    let mut kc := 0
    let mut kf := 0
    let mut slots := 0
    for mode in [0:16] do
      for f in [0:4] do
        let n := cases[slot kind mode f]!
        if n > 0 then slots := slots + 1
        kc := kc + n
        kf := kf + fails[slot kind mode f]!
    IO.println s!"{kindName kind}: {kc} cases over {slots} mode/flag combinations, {kf} mismatches"
    allCases := allCases + kc
    allFails := allFails + kf
  IO.println (s!"total: {allCases} cases ({wrapped} with out-of-16-bit Int inputs or " ++
    s!"qmul ± 2^32; {shortened} with trailing zeros dropped; FFmpeg changed the plane in " ++
    s!"{changed}), {allFails} mismatches")
  return if allFails == 0 && allCases > 0 then 0 else 1
