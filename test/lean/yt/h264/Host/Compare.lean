import Yt.Host.H264Kernels
import Yt.Mp4.Demux
import Yt.H264.Decoder
import Yt.H264.Recon
import Yt.H264.Loop

/-! Host-only test tools (upstream Lean, FFmpeg from PATH; ARM builds skip
`Host/`): the frame-by-frame comparison of the Lean decoder with FFmpeg. -/

namespace Host
open Yt.H264

/-- Read exactly `n` bytes (fewer at the end of the stream). -/
partial def readExact (h : IO.FS.Handle) (n : Nat) (acc : ByteArray := .empty) : IO ByteArray := do
  if acc.size ≥ n then return acc
  let chunk ← h.read (n - acc.size).toUSize
  if chunk.isEmpty then return acc else readExact h n (acc ++ chunk)

/-- The first differing sample of two planes of width `w` (none if equal). -/
def firstDiff (a b : ByteArray) (w : Nat) : Option (Nat × Nat × UInt8 × UInt8) := Id.run do
  for i in [0:min a.size b.size] do
    if a[i]! != b[i]! then return some (i % w, i / w, a[i]!, b[i]!)
  return none

def countDiff (a b : ByteArray) : Nat := Id.run do
  let mut n := 0
  for i in [0:min a.size b.size] do
    if a[i]! != b[i]! then n := n + 1
  return n

/-- The report for frame `k` (none if identical). -/
def compareFrame (k : Nat) (f : OutFrame) (ref : ByteArray) : Option String := Id.run do
  let ys := f.width * f.height
  let cs := ys / 4
  let planes := #[("Y", f.y, ref.extract 0 ys, f.width, 16), ("Cb", f.cb, ref.extract ys (ys + cs), f.width / 2, 8),
                  ("Cr", f.cr, ref.extract (ys + cs) (ys + 2 * cs), f.width / 2, 8)]
  for (name, mine, theirs, w, mb) in planes do
    if let some (x, y, a, b) := firstDiff theirs mine w then
      return some s!"frame {k}: first mismatch {name} ({x},{y}) MB ({x / mb},{y / mb}): ffmpeg {a} lean {b}; {countDiff theirs mine} samples differ"
  return none

/-- The number of bytes left in `h` (read and dropped). -/
partial def drain (h : IO.FS.Handle) (n : Nat) : IO Nat := do
  let b ← h.read 65536
  if b.isEmpty then pure n else drain h (n + b.size)

/-- Decode the first `n` samples of `fixture` with the Lean decoder and compare
every output frame with FFmpeg's on exactly the same samples (stream-copied
to an Annex B file that ffmpeg decodes to the end, as the Lean decoder
flushes; `-skip_loop_filter all` for `noDeblock`). The report of the first
difference (its first differing sample and macroblock), or none, and the
number of identical frames. -/
def compareMp4 (fixture : String) (n : Nat) (noDeblock : Bool) : IO (Option String × Nat) := do
  let nStr := toString n
  let es := s!"{fixture}.first{nStr}.h264.tmp"
  let cut ← IO.Process.output { cmd := "ffmpeg", args := #["-v", "error", "-y", "-i", fixture, "-map", "0:v:0",
    "-c", "copy", "-frames:v", nStr, "-bsf:v", "h264_mp4toannexb", "-f", "h264", es] }
  if cut.exitCode != 0 then return (some cut.stderr, 0)
  let ffArgs := #["-v", "error", "-threads", "1"] ++ (if noDeblock then #["-skip_loop_filter", "all"] else #[]) ++
    #["-i", es, "-f", "rawvideo", "-pix_fmt", "yuv420p", "-"]
  let child ← IO.Process.spawn { cmd := "ffmpeg", args := ffArgs, stdout := .piped, stderr := .inherit }
  let buf ← IO.FS.readBinFile fixture
  let init ← IO.ofExcept (Yt.Mp4.parseInit buf)
  let .avc _ _ nls _ _ spsList ppsList _ := init.codec | throw (IO.userError "not H.264")
  let samples ← IO.ofExcept (Yt.Mp4.parseFragments buf init)
  let mut dec : Decoder := { deblock := !noDeblock }
  for s in spsList ++ ppsList do dec ← IO.ofExcept (dec.decodeNal s)
  let n := min n samples.size
  let mut same := 0
  let mut bad : Option String := none
  for i in [0:n + 1] do
    if bad.isSome then break
    if i < n then
      let s := samples[i]!
      let nals ← IO.ofExcept (Yt.Mp4.avcNals buf s.offset (s.offset + s.size.toNat) nls)
      for (a, len) in nals do
        match dec.decodeNal (buf.extract a (a + len)) with
        | .ok d => dec := d
        | .error e => bad := some s!"sample {i}: {e}"
    else dec := dec.flushOutput
    for f in dec.out do
      if bad.isNone then
        let size := f.width * f.height * 3 / 2
        let ref ← readExact child.stdout size
        if ref.size < size then bad := some s!"ffmpeg ended after {same} frames"
        else match compareFrame same f ref with
          | some r => bad := some r
          | none => same := same + 1
    dec := { dec with out := #[] }
  let rest ← drain child.stdout 0
  let _ ← child.wait
  IO.FS.removeFile es
  match bad with
  | some r => return (some r, same)
  | none => return (if rest == 0 then none else some s!"ffmpeg has more frames than the Lean decoder ({same})", same)

end Host
