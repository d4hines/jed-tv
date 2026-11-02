import Yt.H264.Intra
import Yt.H264.Transform
import Yt.Json.Fmt

/-! The H.264 intra/transform DSP transcript: every function and mode on
pseudo-random planes and coefficients (xorshift32), one FNV-1a hash of the
results per function and mode, so the host (upstream Lean) and our runtime
on 32-bit ARM can be compared byte for byte (zr-build transcript).
Agreement with FFmpeg itself is checked on the host
by `h264dspcmp` (Cmp.lean). -/

namespace H264DspTest
open Yt.H264

@[inline] def next (s : UInt32) : UInt32 :=
  let s := s ^^^ (s <<< 13)
  let s := s ^^^ (s >>> 17)
  s ^^^ (s <<< 5)

def fnvInit : UInt32 := 2166136261

def fnv (h : UInt32) (b : ByteArray) : UInt32 := Id.run do
  let mut h := h
  for x in b do
    h := (h ^^^ x.toUInt32) * 16777619
  return h

def hex32 (v : UInt32) : String := Id.run do
  let mut s := ""
  for i in [0:8] do
    let d := ((v >>> (28 - 4 * i).toUInt32) &&& 15).toNat
    s := s.push (Char.ofNat (if d < 10 then 48 + d else 87 + d))
  return s

def line (o : ByteArray) (s : String) : ByteArray := o ++ (s ++ "\n").toUTF8

/-- A random plane of `n` samples: uniform, 0/255 only, or near-constant
(cycling with `style`). Returns the plane and the new generator state. -/
def randPlane (s : UInt32) (n style : Nat) : ByteArray × UInt32 := Id.run do
  let mut s := s
  let mut p := ByteArray.emptyWithCapacity n
  s := next s
  let c := s &&& 255
  for _ in [0:n] do
    s := next s
    let v : UInt32 := match style % 3 with
      | 0 => s &&& 255
      | 1 => if s &&& 1 == 0 then 0 else 255
      | _ => let w := c + (s &&& 7); if w > 255 then 255 else w
    p := p.push v.toUInt8
  return (p, s)

/-- A signed 16-bit value from 16 random bits. -/
def i16 (v : UInt32) : Int := Int.ofNat (v &&& 0xFFFF).toNat - 32768

/-- `n` random coefficients: small, full 16-bit, or sparse (by `style`). -/
def randCoefs (s : UInt32) (n style : Nat) : Array Int × UInt32 := Id.run do
  let mut s := s
  let mut c : Array Int := Array.emptyWithCapacity n
  for _ in [0:n] do
    s := next s
    let v : Int := match style % 3 with
      | 0 => Int.ofNat (s % 41).toNat - 20
      | 1 => i16 s
      | _ => if s % 7 == 0 then i16 (s >>> 8) else 0
    c := c.push v
  return (c, s)

/-- Hash of an `Array Int` as 32-bit little-endian words. -/
def fnvInts (h : UInt32) (a : Array Int) : UInt32 := Id.run do
  let mut b := ByteArray.emptyWithCapacity (4 * a.size)
  for x in a do
    let v := x.toInt32.toUInt32
    b := (b.push v.toUInt8).push (v >>> 8).toUInt8
    b := (b.push (v >>> 16).toUInt8).push (v >>> 24).toUInt8
  return fnv h b

/-- Kinds of plane operation exercised by `planeRun`. -/
inductive Op where
  | p4 (mode : Nat) (tr : Bool)
  | p8l (mode : Nat) (tl tr : Bool)
  | p16 (mode : Nat)
  | p8 (mode : Nat)
  | i4 | i8 | d4 | d8

/-- `n` cases of an operation on random planes; hash of every result. -/
def planeRun (op : Op) (seed : UInt32) (n : Nat) : UInt32 := Id.run do
  let mut s := seed
  let mut h := fnvInit
  for k in [0:n] do
    s := next s
    let stride := 26 + (s % 7).toNat
    let off := stride * (1 + ((s >>> 4) % 2).toNat) + 1 + ((s >>> 8) % 3).toNat
    let (p, s1) := randPlane s (stride * 21) k
    let (c, s2) := randCoefs s1 64 k
    s := s2
    let q := match op with
      | .p4 m tr => Intra.pred4x4 p off stride m tr
      | .p8l m tl tr => Intra.pred8x8l p off stride m tl tr
      | .p16 m => Intra.pred16x16 p off stride m
      | .p8 m => Intra.pred8x8 p off stride m
      | .i4 => Transform.idct4Add p off stride c
      | .i8 => Transform.idct8Add p off stride c
      | .d4 => Transform.idct4DcAdd p off stride c[0]!
      | .d8 => Transform.idct8DcAdd p off stride c[0]!
    h := fnv h q
  return h

def planeLine (o : ByteArray) (name : String) (op : Op) (seed : UInt32) (n : Nat) : ByteArray :=
  line o (name ++ " n=" ++ Yt.Fmt.dec n.toUInt64 ++ " fnv=" ++ hex32 (planeRun op seed n))

/-- `n` DC transforms (luma when `luma`), qmul cycling through FFmpeg-style
values and arbitrary 32-bit ones. -/
def dcRun (luma : Bool) (seed : UInt32) (n : Nat) : UInt32 := Id.run do
  let mut s := seed
  let mut h := fnvInit
  for k in [0:n] do
    let (c, s1) := randCoefs s (if luma then 16 else 4) k
    s := next s1
    let qp := (s % 52).toNat
    let qmul : Int := match k % 3 with
      | 0 => Int.ofNat ((#[10, 11, 13, 14, 16, 18][qp % 6]! * 16) <<< (qp / 6 + 2))
      | 1 => Int.ofNat (s >>> 12).toNat
      | _ => i16 s * 65536 + Int.ofNat (s &&& 0xFFFF).toNat
    let r := if luma then Transform.lumaDcDequantIdct c qmul
      else Transform.chromaDcDequantIdct c qmul
    h := fnvInts h r
  return h

def transcript (_ : Nat) : ByteArray := Id.run do
  let mut o := ByteArray.empty
  let n := 150
  for m in [0:12] do
    for tr in [false, true] do
      let name := "pred4x4 m=" ++ Yt.Fmt.dec m.toUInt64 ++ " tr=" ++ (if tr then "1" else "0")
      o := planeLine o name (.p4 m tr) (0x1234 + m.toUInt32) n
  for m in [0:12] do
    for f in [0:4] do
      o := planeLine o ("pred8x8l m=" ++ Yt.Fmt.dec m.toUInt64 ++ " f=" ++ Yt.Fmt.dec f.toUInt64)
        (.p8l m (f &&& 2 != 0) (f &&& 1 != 0)) (0x5678 + m.toUInt32) n
  for m in [0:7] do
    o := planeLine o ("pred16x16 m=" ++ Yt.Fmt.dec m.toUInt64) (.p16 m) (0x9abc + m.toUInt32) n
  for m in [0:11] do
    o := planeLine o ("pred8x8 m=" ++ Yt.Fmt.dec m.toUInt64) (.p8 m) (0xdef0 + m.toUInt32) n
  o := planeLine o "idct4Add" .i4 0x1111 1000
  o := planeLine o "idct8Add" .i8 0x2222 500
  o := planeLine o "idct4DcAdd" .d4 0x3333 300
  o := planeLine o "idct8DcAdd" .d8 0x4444 300
  o := line o ("lumaDcDequantIdct n=1000 fnv=" ++ hex32 (dcRun true 0x5555 1000))
  o := line o ("chromaDcDequantIdct n=1000 fnv=" ++ hex32 (dcRun false 0x6666 1000))
  return o

@[export zr_test_transcript] def transcriptExport (k : Nat) : ByteArray := transcript k

end H264DspTest
