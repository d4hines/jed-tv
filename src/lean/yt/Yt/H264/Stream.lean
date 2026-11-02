import Yt.H264.Decoder
import Yt.Mp4.Demux

/-!
The Lean H.264 decoder driven by MP4 samples, as a player uses it: samples
in (AVC length-prefixed NAL units with a presentation time), pictures out in
presentation order. Pure: the player keeps the stream in its state.
-/

namespace Yt.H264

structure Stream where
  dec : Decoder := {}
  /-- Bytes of each NAL unit's length prefix (avcC). -/
  nls : UInt8
  /-- The picture last returned by `receive`. -/
  frame : Option OutFrame := none
  flushed : Bool := false

/-- A stream for an avcC record's parameter sets (`nls`: its NAL length size). -/
def Stream.open (nls : UInt8) (params : Array ByteArray) : Except String Stream := do
  let mut dec : Decoder := {}
  for p in params do dec ← dec.decodeNal p
  return { dec, nls }

/-- Decode one sample, bytes `off ..< off + len` of `buf`, presented at `pts`;
`skipNonRef`: drop it if it is a non-reference picture (to catch up). -/
def Stream.send (s : Stream) (buf : ByteArray) (off len : Nat) (pts : UInt64) (skipNonRef := false) :
    Except String Stream := do
  let nals ← match Mp4.avcNals buf off (off + len) s.nls with
    | .ok n => pure n
    | .error e => throw s!"H.264 sample: {e}"
  let mut dec := { s.dec with pts, skipNonRef }
  for (a, n) in nals do dec ← dec.decodeNal (buf.extract a (a + n))
  return { s with dec }

/-- End of input: the remaining pictures become available. -/
def Stream.flush (s : Stream) : Stream := { s with dec := s.dec.flushOutput, flushed := true }

/-- The next picture in presentation order into `frame` (true), or false:
more input needed, or the end after `flush`. -/
def Stream.receive (s : Stream) : Stream × Bool :=
  match s.dec.out[0]? with
  | some f => ({ s with frame := some f, dec := { s.dec with out := s.dec.out.extract 1 s.dec.out.size } }, true)
  | none => (s, false)

end Yt.H264
