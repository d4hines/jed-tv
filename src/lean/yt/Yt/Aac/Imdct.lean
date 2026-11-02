import Yt.Aac.Math

/-! Fixed-point inverse MDCT and windowed overlap-add.

`Mdct.half` computes the middle half of the N-point IMDCT
(`h[n] = Σ_k X[k] cos(2π/N (n + N/4 + 1/2 + N/4)(k + 1/2))`, `n < N/2`, i.e.
outputs `N/4 .. 3N/4` of the spec's transform without its 2/N factor) by
rotating into an N/4-point complex FFT and back. Input coefficients must be
below 2^29 in magnitude; every FFT stage halves, so the result is
`h / (N/4)`, and with the 2/N factor the time signal is `result / 2`.
Twiddles are Q31; sums of two values below 2^30 stay in `Int32`.

The overlap-add follows FFmpeg's `imdct_and_windowing` exactly (same
window-sequence cases and buffer layout), so window-switching behaviour,
including on illegal sequences, matches the reference decoder. -/

namespace Yt.Aac

structure Mdct where
  /-- Input coefficients, N/2. -/
  n2 : USize
  /-- FFT points, N/4. -/
  n4 : USize
  /-- Words `2i, 2i+1`: cos, sin of 2π(i + 1/8)/N. -/
  tw : ByteArray
  /-- Words `2k, 2k+1`: cos, sin of 2πk/(N/4). -/
  fft : ByteArray
  /-- Bit reversal of FFT indices. -/
  rev : ByteArray

namespace Mdct

def make (bigN : Nat) : Mdct :=
  { n2 := (bigN / 2).toUSize, n4 := (bigN / 4).toUSize, tw := Math.imdctTwiddles bigN,
    fft := Math.fftTwiddles (bigN / 4), rev := Math.bitReverse (bigN / 4) }

@[inline] private def rnd31 (x : UInt64) : Int32 := lo32 ((x + 0x40000000) >>> 31)

/-- Pre-rotation into bit-reversed FFT order, inputs shifted left by `s`. -/
private partial def preRot (m : Mdct) (x : ByteArray) (xo : USize) (s : UInt32) (z : ByteArray)
    (k : USize) : ByteArray :=
  if k < m.n4 then
    let are := sx (ldI x (xo + m.n2 - 1 - 2 * k) <<< s.toInt32)
    let aim := sx (ldI x (xo + 2 * k) <<< s.toInt32)
    let c := sx (ldI m.tw (2 * k))
    let sn := sx (ldI m.tw (2 * k + 1))
    let j := (ld32 m.rev k).toUSize
    let z := stI z (2 * j) (rnd31 (are * c - aim * sn))
    let z := stI z (2 * j + 1) (rnd31 (are * sn + aim * c))
    preRot m x xo s z (k + 1)
  else z

/-- Radix-2 butterflies `(i, i+1)` for even `i` (the first stage, twiddle 1). -/
private partial def stage2 (z : ByteArray) (i n : USize) : ByteArray :=
  if i < n then
    let ar := ldI z (2 * i)
    let ai := ldI z (2 * i + 1)
    let br := ldI z (2 * i + 2)
    let bi := ldI z (2 * i + 3)
    let z := stI z (2 * i) ((ar + br + 1) >>> (1 : Int32))
    let z := stI z (2 * i + 1) ((ai + bi + 1) >>> (1 : Int32))
    let z := stI z (2 * i + 2) ((ar - br + 1) >>> (1 : Int32))
    let z := stI z (2 * i + 3) ((ai - bi + 1) >>> (1 : Int32))
    stage2 z (i + 2) n
  else z

@[inline] private def quarter (x : UInt64) : Int32 := lo32 (sar64 (x + 2) 2)

/-- Two radix-2 stages (half-sizes h and 2h) fused, on points
`i, i+h, i+2h, i+3h` for `i = j, j + 4h, …`, with `w1 = W^(j/2h)` and
`w2 = W^(j/4h)`; scaled by 1/4. -/
private partial def bfly4 (z : ByteArray) (i h n : USize) (w1r' w1i' w2r' w2i' : Int32) : ByteArray :=
  if i < n then
    -- Sign-extend here, not in the caller, so the products compile to smull.
    let w1r := sx w1r'
    let w1i := sx w1i'
    let w2r := sx w2r'
    let w2i := sx w2i'
    let i1 := i + h
    let i2 := i1 + h
    let i3 := i2 + h
    let a0r := ldI z (2 * i)
    let a0i := ldI z (2 * i + 1)
    let a1r := sx (ldI z (2 * i1))
    let a1i := sx (ldI z (2 * i1 + 1))
    let a2r := ldI z (2 * i2)
    let a2i := ldI z (2 * i2 + 1)
    let a3r := sx (ldI z (2 * i3))
    let a3i := sx (ldI z (2 * i3 + 1))
    let b1r := rnd31 (a1r * w1r - a1i * w1i)
    let b1i := rnd31 (a1r * w1i + a1i * w1r)
    let b3r := rnd31 (a3r * w1r - a3i * w1i)
    let b3i := rnd31 (a3r * w1i + a3i * w1r)
    let s0r := sx (a0r + b1r)
    let s0i := sx (a0i + b1i)
    let s1r := sx (a0r - b1r)
    let s1i := sx (a0i - b1i)
    let s2r := sx (a2r + b3r)
    let s2i := sx (a2i + b3i)
    -- i·(a2 − b3)
    let s3r := sx (b3i - a2i)
    let s3i := sx (a2r - b3r)
    let t2r := sx (rnd31 (s2r * w2r - s2i * w2i))
    let t2i := sx (rnd31 (s2r * w2i + s2i * w2r))
    let t3r := sx (rnd31 (s3r * w2r - s3i * w2i))
    let t3i := sx (rnd31 (s3r * w2i + s3i * w2r))
    let z := stI z (2 * i) (quarter (s0r + t2r))
    let z := stI z (2 * i + 1) (quarter (s0i + t2i))
    let z := stI z (2 * i2) (quarter (s0r - t2r))
    let z := stI z (2 * i2 + 1) (quarter (s0i - t2i))
    let z := stI z (2 * i1) (quarter (s1r + t3r))
    let z := stI z (2 * i1 + 1) (quarter (s1i + t3i))
    let z := stI z (2 * i3) (quarter (s1r - t3r))
    let z := stI z (2 * i3 + 1) (quarter (s1i - t3i))
    bfly4 z (i + 4 * h) h n w1r' w1i' w2r' w2i'
  else z

private partial def stage4 (m : Mdct) (z : ByteArray) (h j : USize) : ByteArray :=
  if j < h then
    let t1 := j * (m.n4 / (2 * h))
    let t2 := j * (m.n4 / (4 * h))
    let z := bfly4 z j h m.n4 (ldI m.fft (2 * t1)) (ldI m.fft (2 * t1 + 1))
      (ldI m.fft (2 * t2)) (ldI m.fft (2 * t2 + 1))
    stage4 m z h (j + 1)
  else z

private partial def stages4 (m : Mdct) (z : ByteArray) (h : USize) : ByteArray :=
  if h < m.n4 then stages4 m (stage4 m z h 0) (4 * h) else z

/-- In-place FFT (e^{+2πi/n}), bit-reversed input, natural-order output,
scaled by 1/n. -/
private def transform (m : Mdct) (z : ByteArray) : ByteArray :=
  -- An odd number of radix-2 stages starts with one plain stage.
  if m.n4.log2 % 2 == 1 then stages4 m (stage2 z 0 m.n4) 2 else stages4 m z 1

/-- Post-rotation; results shifted by `t` (see `shiftRound`) into `out` at `oo`. -/
private partial def postRot (m : Mdct) (z out : ByteArray) (oo : USize) (t : Int32) (k : USize) :
    ByteArray :=
  let n8 := m.n4 / 2
  if k < n8 then
    let a := n8 - 1 - k
    let b := n8 + k
    let zra := sx (ldI z (2 * a))
    let zia := sx (ldI z (2 * a + 1))
    let zrb := sx (ldI z (2 * b))
    let zib := sx (ldI z (2 * b + 1))
    let ca := sx (ldI m.tw (2 * a))
    let sa := sx (ldI m.tw (2 * a + 1))
    let cb := sx (ldI m.tw (2 * b))
    let sb := sx (ldI m.tw (2 * b + 1))
    let out := stI out (oo + 2 * a) (shiftRound (rnd31 (zra * ca - zia * sa)) t)
    let out := stI out (oo + 2 * a + 1) (shiftRound (rnd31 (0 - zib * cb - zrb * sb)) t)
    let out := stI out (oo + 2 * b) (shiftRound (rnd31 (zrb * cb - zib * sb)) t)
    let out := stI out (oo + 2 * b + 1) (shiftRound (rnd31 (0 - zia * ca - zra * sa)) t)
    postRot m z out oo t (k + 1)
  else out

/-- Half IMDCT of words `[xo, xo + N/2)` of `x`, inputs shifted left by `s`
(the caller guarantees they stay below 2^29), into words `[oo, oo + N/2)` of
`out`, scaled by `2^t / (N/4)`. `z` is scratch of at least N/2 words. -/
def half (m : Mdct) (x : ByteArray) (xo : USize) (s : UInt32) (z out : ByteArray) (oo : USize)
    (t : Int32) : ByteArray × ByteArray :=
  let z := transform m (preRot m x xo s z 0)
  (z, postRot m z out oo t 0)

end Mdct

/-- Largest magnitude among words `[i, j)`. -/
partial def maxAbs (x : ByteArray) (i j : USize) (m : UInt32) : UInt32 :=
  if i < j then
    let a := iabs (ldI x i)
    maxAbs x (i + 1) j (if a > m then a else m)
  else m

/-- Shift words `[i, j)` by `s` (see `shiftRound`). -/
partial def shiftWords (x : ByteArray) (i j : USize) (s : Int32) : ByteArray :=
  if i < j then shiftWords (stI x i (shiftRound (ldI x i) s)) (i + 1) j s else x

/-- FFmpeg's `vector_fmul_window` with a Q31 window of 2·len words:
`dst[d+k] = s0[k]·w[2len−1−k] − s1[len−1−k]·w[k]`,
`dst[d+2len−1−k] = s0[k]·w[k] + s1[len−1−k]·w[2len−1−k]`. -/
partial def fmulWindow (dst : ByteArray) (d : USize) (s0 : ByteArray) (o0 : USize)
    (s1 : ByteArray) (o1 : USize) (w : ByteArray) (len k : USize) : ByteArray :=
  if k < len then
    let a := sx (ldI s0 (o0 + k))
    let b := sx (ldI s1 (o1 + len - 1 - k))
    let wi := sx (ldI w k)
    let wj := sx (ldI w (2 * len - 1 - k))
    let dst := stI dst (d + k) (rshr64 (a * wj - b * wi) 31)
    let dst := stI dst (d + 2 * len - 1 - k) (rshr64 (a * wi + b * wj) 31)
    fmulWindow dst d s0 o0 s1 o1 w len (k + 1)
  else dst

/-- Window shapes, rising halves, Q31. -/
structure Windows where
  sineLong : ByteArray
  sineShort : ByteArray
  kbdLong : ByteArray
  kbdShort : ByteArray

/-- Per-channel overlap state. -/
structure Overlap where
  /-- 512 words, Q12: the second half of the previous frame. -/
  saved : ByteArray
  /-- window_sequence of the previous frame. -/
  prevSeq : UInt8
  /-- window_shape (KBD) of the previous frame. -/
  prevKbd : Bool
  deriving Inhabited

def Overlap.init (_ : Unit) : Overlap := { saved := zeros32 512, prevSeq := 0, prevKbd := false }

/-- Window sequences. -/
def onlyLong : UInt8 := 0
def longStart : UInt8 := 1
def eightShort : UInt8 := 2
def longStop : UInt8 := 3

/-- Overlap-add one frame. `buf` holds the half-IMDCT outputs in Q12 (1024
words: one long block, or eight 128-word short blocks); returns the new state
and 1024 output words in Q12. -/
def overlapAdd (w : Windows) (st : Overlap) (buf : ByteArray) (seq : UInt8) (kbd : Bool) :
    Overlap × ByteArray :=
  let lwinPrev := if st.prevKbd then w.kbdLong else w.sineLong
  let swinPrev := if st.prevKbd then w.kbdShort else w.sineShort
  let swin := if kbd then w.kbdShort else w.sineShort
  let saved := st.saved
  let out := zeros32 1024
  let longLong := (st.prevSeq == onlyLong || st.prevSeq == longStop) &&
    (seq == onlyLong || seq == longStart)
  let (out, temp) :=
    if longLong then (fmulWindow out 0 saved 0 buf 0 lwinPrev 512 0, ByteArray.empty)
    else
      let out := copy32 saved 0 out 0 448
      if seq == eightShort then
        let out := fmulWindow out 448 saved 448 buf 0 swinPrev 64 0
        let out := fmulWindow out 576 buf 64 buf 128 swin 64 0
        let out := fmulWindow out 704 buf 192 buf 256 swin 64 0
        let out := fmulWindow out 832 buf 320 buf 384 swin 64 0
        let temp := fmulWindow (zeros32 128) 0 buf 448 buf 512 swin 64 0
        (copy32 temp 0 out 960 64, temp)
      else
        let out := fmulWindow out 448 saved 448 buf 0 swinPrev 64 0
        (copy32 buf 64 out 576 448, ByteArray.empty)
  let saved :=
    if seq == eightShort then
      let saved := copy32 temp 64 saved 0 64
      let saved := fmulWindow saved 64 buf 576 buf 640 swin 64 0
      let saved := fmulWindow saved 192 buf 704 buf 768 swin 64 0
      let saved := fmulWindow saved 320 buf 832 buf 896 swin 64 0
      copy32 buf 960 saved 448 64
    else copy32 buf 512 saved 0 512
  ({ saved, prevSeq := seq, prevKbd := kbd }, out)

end Yt.Aac
