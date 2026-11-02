import H264Fixture
import H264Test.Run

/-! Decode-only workload for instruction counting on 32-bit ARM (QEMU
`-icount`): the YouTube fixture's first samples (1280x536). `stage` 0:
entropy decoding only, 1: + reconstruction, 2: + deblocking (the full
decoder). Returns a checksum of the output (every 64th luma sample, so
that hashing stays out of the counts). -/

namespace H264Test
open Yt.H264

def ytClip (_ : Unit) : ByteArray := Id.run do
  for (name, hex) in H264Fixture.clips do
    if name == "yt136" then return unhex hex
  return .empty

def sparse (h : UInt32) (b : ByteArray) (i : Nat) : UInt32 :=
  if i < b.size then sparse ((h ^^^ b[i]!.toUInt32) * 16777619) b (i + 64) else h
termination_by b.size - i

@[export h264_test_bench]
def bench (stage : Nat) : UInt32 :=
  let mp4 := ytClip ()
  match decodeMp4 mp4 (stage ≥ 1) (stage ≥ 2) fnvInit (fun h _ f => sparse h f.y 0) with
  | .ok (h, n) => h + n.toUInt32
  | .error _ => 0

end H264Test
