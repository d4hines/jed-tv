import Yt.Mp4.Demux
import Yt.H264.Decoder
import Yt.H264.Recon
import Yt.H264.Loop

/-! Test-harness helpers: decode an MP4 held in memory with the Lean H.264
decoder, hex decoding and a hash. Pure, so they also run in the ARM32
transcript and benchmark. -/

namespace H264Test
open Yt Yt.H264

private def nibble (c : UInt8) : UInt8 :=
  if c ≥ 97 then c - 87 else if c ≥ 65 then c - 55 else c - 48

/-- Bytes of a hex string. -/
def unhex (s : String) : ByteArray := Id.run do
  let h := s.toUTF8
  let mut out := ByteArray.emptyWithCapacity (h.size / 2)
  for i in [0:h.size / 2] do
    out := out.push (nibble h[2 * i]! * 16 + nibble h[2 * i + 1]!)
  return out

/-- FNV-1a, 32-bit. -/
def fnv (h : UInt32) (b : ByteArray) : UInt32 := Id.run do
  let mut h := h
  for i in [0:b.size] do
    h := (h ^^^ b[i]!.toUInt32) * 16777619
  return h

def fnvInit : UInt32 := 2166136261

def hex32 (v : UInt32) : String := Id.run do
  let mut s := ""
  for i in [0:8] do
    let d := ((v >>> (28 - 4 * i).toUInt32) &&& 15).toNat
    s := s.push (Char.ofNat (if d < 10 then 48 + d else 87 + d))
  return s

/-- Decode every sample of an MP4, calling `each` on the frames in output
order (with the frame number), threading `acc`. -/
def decodeMp4 {α : Type} (buf : ByteArray) (recon deblock : Bool) (acc : α)
    (each : α → Nat → OutFrame → α) : Except String (α × Nat) := do
  let init ← Mp4.parseInit buf
  let .avc _ _ nls _ _ spsList ppsList _ := init.codec | throw "not H.264"
  let samples ← Mp4.parseFragments buf init
  let mut dec : Decoder := { recon, deblock }
  for s in spsList ++ ppsList do dec ← dec.decodeNal s
  let mut acc := acc
  let mut k := 0
  for i in [0:samples.size + 1] do
    if i < samples.size then
      let s := samples[i]!
      let nals ← Mp4.avcNals buf s.offset (s.offset + s.size.toNat) nls
      for (a, len) in nals do
        dec ← (dec.decodeNal (buf.extract a (a + len))).mapError (fun e => s!"sample {i}: {e}")
    else dec := dec.flushOutput
    for f in dec.out do
      acc := each acc k f
      k := k + 1
    dec := { dec with out := #[] }
  return (acc, k)

end H264Test
