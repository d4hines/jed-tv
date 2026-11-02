import Yt.Host.H264Kernels
import Yt.Mp4.Demux
import Yt.H264.Decoder
import Yt.H264.Recon
import Yt.H264.Loop

open Yt Yt.H264

/-- Write the frames output so far (cropped yuv420p) and drop them. -/
def drain (o : Option IO.FS.Stream) (dec : Decoder) : IO (Decoder × Nat) := do
  if let some o := o then
    for f in dec.out do
      o.write f.y; o.write f.cb; o.write f.cr
    o.flush
  return ({ dec with out := #[] }, dec.out.size)

/-- Decode the first `n` samples of an MP4 with the Lean decoder and write
the frames (cropped yuv420p, output order) to `out` (`-`: standard output)
as they are output. `--no-deblock` skips the loop filter; `--parse` decodes
the syntax only (no reconstruction, nothing written; the tail-bits count
still checks that every slice ends exactly at its data). -/
def main (args : List String) : IO UInt32 := do
  let noDeblock := args.contains "--no-deblock"
  let parse := args.contains "--parse"
  let args := args.filter fun a => a != "--no-deblock" && a != "--parse"
  let [file, nStr, out] := args
    | IO.eprintln "usage: h264dec FILE.mp4 SAMPLES OUT.yuv|- [--no-deblock] [--parse]"; return 2
  let buf ← IO.FS.readBinFile file
  let init ← IO.ofExcept (Mp4.parseInit buf)
  let .avc _ _ nls _ _ spsList ppsList _ := init.codec | throw (IO.userError "not H.264")
  let samples ← IO.ofExcept (Mp4.parseFragments buf init)
  let mut dec : Decoder := { recon := !parse, deblock := !(noDeblock || parse) }
  for s in spsList ++ ppsList do dec ← IO.ofExcept (dec.decodeNal s)
  let o ← if parse then pure none else if out == "-" then some <$> IO.getStdout
    else (some ∘ IO.FS.Stream.ofHandle) <$> IO.FS.Handle.mk out .write
  let n := min nStr.toNat! samples.size
  let t0 ← IO.monoMsNow
  let mut frames := 0
  for i in [0:n] do
    let s := samples[i]!
    let nals ← IO.ofExcept (Mp4.avcNals buf s.offset (s.offset + s.size.toNat) nls)
    for (a, len) in nals do
      match dec.decodeNal (buf.extract a (a + len)) with
      | .ok d => dec := d
      | .error e => IO.eprintln s!"sample {i}: {e}"; return 1
    let (d, k) ← drain o dec
    dec := d
    frames := frames + k
  let (d, k) ← drain o dec.flushOutput
  frames := frames + k
  let t1 ← IO.monoMsNow
  IO.eprintln s!"{n} samples, {frames} frames in {t1 - t0} ms; max tail bits after end_of_slice {d.maxTailBits}"
  return 0
