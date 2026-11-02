import Yt.Aac.Decoder
import AacTest.Adts

/-! Host-only robustness check: `aacfuzz IN.adts` decodes every frame with
1, 5 and 9 flipped bits, then 20000 random byte strings, on one decoder that
keeps running, and reports outcomes. It must finish without panics. -/
open Yt.Aac AacTest

def main (args : List String) : IO Unit := do
  let data ← IO.FS.readBinFile args[0]!
  let (asc, aus) := adtsSplit data
  let .ok d0 := Decoder.init asc | throw (IO.userError "init")
  let mut d := d0
  let mut rnd : UInt32 := 777
  let mut ok := 0
  let mut bad := 0
  for round in [0:3] do
    for au in aus do
      let mut au := au
      for _ in [0:round * 4 + 1] do
        rnd := rnd * 1103515245 + 12345
        let bit := (rnd >>> 4).toNat % (8 * au.size)
        au := st8 au (bit / 8).toUSize ((ld8 au (bit / 8).toUSize) ^^^ ((1 : UInt8) <<< (bit % 8).toUInt8))
      let (d1, r) := d.decode au
      d := d1
      match r with
      | .ok _ => ok := ok + 1
      | .error _ => bad := bad + 1
  -- pure garbage
  for i in [0:20000] do
    let mut g := ByteArray.empty
    for _ in [0:(i % 700) + 1] do
      rnd := rnd * 1103515245 + 12345
      g := g.push (rnd >>> 24).toUInt8
    let (d1, r) := d.decode g
    d := d1
    match r with
    | .ok _ => ok := ok + 1
    | .error _ => bad := bad + 1
  IO.println s!"ok {ok} errors {bad}"
