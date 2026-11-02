/-
H.264 NAL units and the bit reader for their headers (ITU-T H.264 7.2, 7.3).
Part of the Lean H.264 decoder, which follows FFmpeg 9.0.1's decoder
(LGPL-2.1-or-later) for exactness.
-/

namespace Yt.H264

/-- The RBSP of a NAL unit: the bytes after the header with the emulation
prevention bytes (`00 00 03` -> `00 00`) removed. -/
def unescape (nal : ByteArray) : ByteArray := Id.run do
  let mut out := ByteArray.emptyWithCapacity nal.size
  let mut zeros := 0
  for i in [1:nal.size] do
    let b := nal[i]!
    if zeros ≥ 2 && b == 3 then
      zeros := 0
    else
      out := out.push b
      zeros := if b == 0 then zeros + 1 else 0
  return out

/-- A position in an RBSP, in bits. -/
structure Bits where
  data : ByteArray
  pos : Nat := 0

abbrev BitM := StateT Bits (Except String)

def bitAt (data : ByteArray) (pos : Nat) : UInt32 :=
  if pos / 8 < data.size then ((data[pos / 8]!.toUInt32) >>> (7 - (pos % 8)).toUInt32) &&& 1 else 0

def bit : BitM UInt32 := do
  let s ← get
  if s.pos / 8 ≥ s.data.size then throw "H.264: read past the end of a header"
  set { s with pos := s.pos + 1 }
  return bitAt s.data s.pos

def flag : BitM Bool := return (← bit) == 1

/-- u(n), n ≤ 32. -/
def u (n : Nat) : BitM UInt32 := do
  let mut v : UInt32 := 0
  for _ in [0:n] do v := (v <<< 1) ||| (← bit)
  return v

/-- ue(v) (9.1). -/
def ue : BitM Nat := do
  let mut zeros := 0
  while (← bit) == 0 do
    zeros := zeros + 1
    if zeros > 31 then throw "H.264: bad Exp-Golomb code"
  return (1 <<< zeros) - 1 + (← u zeros).toNat

/-- se(v) (9.1.1). -/
def se : BitM Int := do
  let k ← ue
  return if k % 2 == 1 then Int.ofNat ((k + 1) / 2) else -Int.ofNat (k / 2)

/-- more_rbsp_data(): anything before the stop bit. -/
def moreRbspData : BitM Bool := do
  let s ← get
  -- the last 1 bit of the RBSP is the stop bit
  let mut last := s.data.size * 8
  while last > 0 && bitAt s.data (last - 1) == 0 do last := last - 1
  return s.pos + 1 < last

def bytePos : BitM Nat := return ((← get).pos + 7) / 8

end Yt.H264
