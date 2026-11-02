import Yt.Aac.Buf

/-! Test-harness helpers: ADTS framing (to get access units and an
AudioSpecificConfig out of FFmpeg's `-f adts` output), hex decoding and a
hash. Pure, so they also run in the ARM32 transcript. -/

namespace AacTest
open Yt.Aac

/-- ADTS frames of a stream: `(asc, access units)`. CRC-protected headers
are skipped over; multi-block frames are not expected from FFmpeg. -/
def adtsSplit (b : ByteArray) : ByteArray × Array ByteArray := Id.run do
  let mut i := 0
  let mut aus : Array ByteArray := #[]
  let mut asc := ByteArray.empty
  for _ in [0:1000000] do
    if i + 7 > b.size then break
    let b1 := (ld8 b (i + 1).toUSize).toNat
    let b2 := (ld8 b (i + 2).toUSize).toNat
    let b3 := (ld8 b (i + 3).toUSize).toNat
    let b4 := (ld8 b (i + 4).toUSize).toNat
    let b5 := (ld8 b (i + 5).toUSize).toNat
    if ld8 b i.toUSize != 0xFF || b1 / 16 != 15 then break
    let protAbsent := b1 % 2 == 1
    let profile := b2 / 64
    let sfi := (b2 / 4) % 16
    let cc := (b2 % 2) * 4 + b3 / 64
    let len := (b3 % 4) * 2048 + b4 * 8 + b5 / 32
    let hdr := if protAbsent then 7 else 9
    if len < hdr || i + len > b.size then break
    if asc.size == 0 then
      let aot := profile + 1
      let v := aot * 2048 + sfi * 128 + cc * 8
      asc := ByteArray.empty.push (v / 256).toUInt8 |>.push (v % 256).toUInt8
    aus := aus.push (b.extract (i + hdr) (i + len))
    i := i + len
  return (asc, aus)

private def nibble (c : UInt8) : UInt8 :=
  if c ≥ 97 then c - 87 else if c ≥ 65 then c - 55 else c - 48

/-- Bytes of a hex string. -/
def unhex (s : String) : ByteArray := Id.run do
  let h := s.toUTF8
  let mut out := ByteArray.emptyWithCapacity (h.size / 2)
  for i in [0:h.size / 2] do
    out := out.push (nibble (ld8 h (2 * i).toUSize) * 16 + nibble (ld8 h (2 * i + 1).toUSize))
  return out

/-- FNV-1a, 32-bit. -/
def fnv (h : UInt32) (b : ByteArray) : UInt32 := Id.run do
  let mut h := h
  for i in [0:b.size] do
    h := (h ^^^ (ld8 b i.toUSize).toUInt32) * 16777619
  return h

def fnvInit : UInt32 := 2166136261

def hex32 (v : UInt32) : String := Id.run do
  let mut s := ""
  for i in [0:8] do
    let d := ((v >>> (28 - 4 * i).toUInt32) &&& 15).toNat
    s := s.push (Char.ofNat (if d < 10 then 48 + d else 87 + d))
  return s

end AacTest
