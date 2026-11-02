import Yt.Aac.Decoder
import AacFixture
import AacTest.Adts

/-! Decode-only workloads for instruction counting on 32-bit ARM (QEMU
`-icount`): the YouTube fixture segments (stereo CPE frames). `stage`
selects how far each frame is processed: 0 parse, 1 + inverse quantization
and stereo, 2 + TNS and IMDCT, 3 the full decoder. Returns a checksum. -/

namespace AacTest
open Yt.Aac

def ytFrames (_ : Unit) : Array (ByteArray × Array ByteArray) := Id.run do
  let mut aus := #[]
  for (name, hex) in AacFixture.streams do
    -- (String.startsWith needs lean_string_memcmp, which our runtime lacks)
    if name == "yt140" || name == "yt140b" || name == "yt140c" then aus := aus.push (adtsSplit (unhex hex))
  return aus

/-- One CPE frame up to `stage` (no END/FIL handling: enough for these streams). -/
def upTo (t : Tables) (au : ByteArray) (stage : Nat) : Nat := Id.run do
  let r : BitReader := { data := au, pos := 3 + 4 }
  let (common, r) := r.readBool
  let .ok (info, r) := parseIcsInfo r 4 | return 0
  let (msp, r) := r.readNat 2
  let r := if msp == 1 then r.skip (info.maxSfb * info.groupLen.size) else r
  let .ok (ics0, r) := parseIcs r 4 (if common then some info else none) t.books t.tns3 t.tns4 | return 0
  let .ok (ics1, _) := parseIcs r 4 (if common then some info else none) t.books t.tns3 t.tns4 | return 0
  if stage == 0 then return ics0.q.size + ics1.q.size
  let c := t.consts
  let b := max (channelBound c ics0) (channelBound c ics1)
  let e := expForBound b
  let (x0, rand) := dequantize c ics0 e 1
  let (x1, _) := dequantize c ics1 e rand
  let ms := ByteArray.mk (Array.replicate 512 1)
  let (x0, x1) := if msp == 2 then midSide ics0.info ms ics0.bandType ics1.bandType x0 x1 else (x0, x1)
  let x1 := intensity c ics1 ms (msp != 0) x0 x1
  if stage == 1 then return (ldI x0 5).toUInt32.toNat + (ldI x1 7).toUInt32.toNat
  let (x0, e0) := if ics0.tns.isEmpty then (x0, e) else tnsChannel ics0 4 x0 e
  let (_, o0) := synthesize t (Overlap.init ()) x0 e0 ics0.info
  let (_, o1) := synthesize t (Overlap.init ()) x1 e ics1.info
  return (ldI o0 100).toUInt32.toNat + (ldI o1 200).toUInt32.toNat

@[export aac_test_bench] def bench (stage : Nat) : Nat := Id.run do
  let t := sharedTables ()
  let mut total := 0
  for (asc, frames) in ytFrames () do
    if stage ≥ 3 then
      match Decoder.init asc with
      | .error _ => pure ()
      | .ok d0 =>
        let mut d := d0
        for au in frames do
          let (d1, r) := d.decode au
          d := d1
          match r with
          | .ok pcm => total := total + pcm.size
          | .error _ => pure ()
    else
      for au in frames do
        total := total + upTo t au stage
  return total

end AacTest
