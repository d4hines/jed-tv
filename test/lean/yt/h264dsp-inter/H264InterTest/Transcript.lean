import Yt.H264.Inter
import Yt.H264.Filter
import Yt.Json.Fmt

/-! The H.264 inter/deblocking transcript: deterministic pseudo-random cases
(xorshift32) through every function, size, fraction and flag, with a running
FNV-1a-32 hash of each function's output buffers. It uses the `…32`
(two's-complement) entry points, which link on our runtime today, so the
host (upstream Lean) and 32-bit ARM (our runtime, zr-build transcript) can be
compared byte for byte. The FFmpeg C harness in
assets/build/lean-h264-inter (`gen transcript`) replays the same cases
through FFmpeg's C and prints the same text. -/

namespace H264InterTest
open Yt.H264

@[inline] def step (s : UInt32) : UInt32 :=
  let s := s ^^^ (s <<< 13)
  let s := s ^^^ (s >>> 17)
  s ^^^ (s <<< 5)

/-- `n` pseudo-random bytes (the low byte of each step) and the new state. -/
def fill (s : UInt32) (n : Nat) : UInt32 × ByteArray := Id.run do
  let mut s := s
  let mut b := ByteArray.emptyWithCapacity n
  for _ in [0:n] do
    s := step s
    b := b.push s.toUInt8
  return (s, b)

def fnv (h : UInt32) (b : ByteArray) : UInt32 := Id.run do
  let mut h := h
  for x in b do
    h := (h ^^^ x.toUInt32) * 0x01000193
  return h

def fnvInit : UInt32 := 0x811c9dc5

def line (o : ByteArray) (name : String) (n : Nat) (h : UInt32) : ByteArray :=
  o ++ (name ++ " " ++ Yt.Fmt.dec n.toUInt64 ++ " " ++ Yt.Fmt.hex64 h.toUInt64 ++ "\n").toUTF8

def lumaSize (i : Nat) : Nat := if i == 0 then 16 else if i == 1 then 8 else 4
def chromaSize (i : Nat) : Nat := if i == 0 then 8 else if i == 1 then 4 else 2

/-- `r - k` as two's complement (a signed value from a draw `r`). -/
@[inline] def signed (r k : UInt32) : UInt32 := r - k

/-- Luma (40×24 picture, stride 48) and chroma (20×12, stride 24) cases:
every (w, h, dx, dy, avg) twice / once, every 8th position far outside. -/
def mcLines (o : ByteArray) (s0 : UInt32) : UInt32 × ByteArray := Id.run do
  let (s1, lref) := fill s0 (48 * 24)
  let (s2, cref) := fill s1 (24 * 12)
  let mut s := s2
  let mut h := fnvInit
  for k in [0:576] do
    let c := k % 288
    let far := k % 8 == 7
    s := step s
    let x := if far then signed (s % 4001) 2000 else signed (s % 100) 30
    s := step s
    let y := if far then signed (s % 4001) 2000 else signed (s % 84) 30
    let (s', dst) := fill s 400
    s := s'
    let dst := Inter.lumaMc32 dst 3 24 lref 48 40 24 x y (c / 9 % 4) (c / 36 % 4) (lumaSize (c % 3))
      (lumaSize (c / 3 % 3)) (c / 144 == 1)
    h := fnv h dst
  let mut o := line o "lumaMc" 576 h
  h := fnvInit
  for k in [0:1152] do
    let far := k % 8 == 7
    s := step s
    let x := if far then signed (s % 4001) 2000 else signed (s % 60) 20
    s := step s
    let y := if far then signed (s % 4001) 2000 else signed (s % 52) 20
    let (s', dst) := fill s 200
    s := s'
    let dst := Inter.chromaMc32 dst 5 16 cref 24 20 12 x y (k / 9 % 8) (k / 72 % 8) (chromaSize (k % 3))
      (chromaSize (k / 3 % 3)) (k / 576 == 1)
    h := fnv h dst
  o := line o "chromaMc" 1152 h
  return (s, o)

/-- Weighting: every (w, h, log2Denom) 4 times; biweight also implicit. -/
def weightLines (o : ByteArray) (s0 : UInt32) : UInt32 × ByteArray := Id.run do
  let mut s := s0
  let mut h := fnvInit
  for k in [0:512] do
    let c := k % 128
    s := step s
    let wt := signed (s % 256) 128
    s := step s
    let off := signed (s % 256) 128
    let (s', dst) := fill s 300
    s := s'
    h := fnv h (Inter.weight32 dst 2 18 (16 >>> (c % 4)) (16 >>> (c / 4 % 4)) (c / 16) wt off)
  let mut o := line o "weight" 512 h
  h := fnvInit
  for k in [0:576] do
    let c := k % 144
    let m := c / 16
    let mut ld := 5
    let mut w0 : UInt32 := 0
    let mut w1 : UInt32 := 0
    let mut off : UInt32 := 0
    if m < 8 then
      ld := m
      s := step s
      w0 := signed (s % 256) 128
      s := step s
      w1 := signed (s % 256) 128
      s := step s
      off := signed (s % 511) 256
    else
      s := step s
      w0 := signed (s % 193) 64
      w1 := 64 - w0
    let (s', dst) := fill s 300
    let (s'', src) := fill s' 320
    s := s''
    h := fnv h (Inter.biweight32 dst 2 18 src 1 19 (16 >>> (c % 4)) (16 >>> (c / 4 % 4)) ld w0 w1 off)
  o := line o "biweight" 576 h
  return (s, o)

/-- `base + d` clamped to a byte. -/
@[inline] def clipAdd (base d : UInt32) : UInt8 :=
  let v := base + d
  if v >>> 31 != 0 then 0 else if v > 255 then 255 else v.toUInt8

/-- Filters: each kind 1000 times, alternating vertical and horizontal
edges, with smooth content across the edge so the filters engage. -/
def filterLines (o : ByteArray) (s0 : UInt32) : UInt32 × ByteArray := Id.run do
  let mut s := s0
  let mut hs : Array UInt32 := #[fnvInit, fnvInit, fnvInit, fnvInit]
  for k in [0:4000] do
    let kind := k % 4
    let horiz := k / 4 % 2 == 1
    let lines := if kind < 2 then 16 else 8
    let (xs, ys, off) := if horiz then (20, 1, 80) else (1, 12, 4)
    s := step s
    let alpha := (s % 256).toNat
    s := step s
    let beta := (s % 24).toNat
    s := step s
    let t0 := signed (s % 32) 2
    s := step s
    let t1 := signed (s % 32) 2
    s := step s
    let t2 := signed (s % 32) 2
    s := step s
    let t3 := signed (s % 32) 2
    let (s', b0) := fill s 200
    s := step s'
    let base := s % 256
    s := step s
    let stp := signed (s % 41) 20
    s := step s
    let amp := s % 8
    let mut b := b0
    for l in [0:lines] do
      for j in [0:8] do
        s := step s
        let d := (if j ≥ 4 then stp else 0) + signed (s % (2 * amp + 1)) amp
        b := b.set! (off + l * ys + j * xs - 4 * xs) (clipAdd base d)
    let r := match kind with
      | 0 => Filter.luma32 b off xs ys alpha beta t0 t1 t2 t3
      | 1 => Filter.lumaIntra b off xs ys alpha beta
      | 2 => Filter.chroma32 b off xs ys alpha beta t0 t1 t2 t3
      | _ => Filter.chromaIntra b off xs ys alpha beta
    hs := hs.modify kind (fnv · r)
  let mut o := o
  o := line o "Filter.luma" 1000 hs[0]!
  o := line o "Filter.lumaIntra" 1000 hs[1]!
  o := line o "Filter.chroma" 1000 hs[2]!
  o := line o "Filter.chromaIntra" 1000 hs[3]!
  return (s, o)

@[export zr_test_transcript] def transcript (_k : Nat) : ByteArray :=
  let (s, o) := mcLines ByteArray.empty 0x12345678
  let (s, o) := weightLines o s
  let (_, o) := filterLines o s
  o

end H264InterTest
