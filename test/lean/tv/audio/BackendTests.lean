import TV.Audio.Queue
import Init

namespace AudioBackendTests
instance [BEq α] [BEq β] : BEq (Except α β) where
  beq a b := match a, b with
    | .ok a, .ok b => a == b
    | .error a, .error b => a == b
    | _, _ => false

def failed (x : Except α β) : Bool := !x.isOk

open TV.Audio

private def check (p : Bool) (name : String) : IO Unit :=
  unless p do throw (IO.userError name)

private def get (value : Except Failure α) : IO α := match value with
  | .ok value => pure value
  | .error error => throw (IO.userError (reprStr error))

private def pcm (samples : Array Int16) : ByteArray := samples.foldl (fun (b : ByteArray) (value : Int16) =>
  b.push value.toUInt16.toUInt8 |>.push (value.toUInt16 >>> 8).toUInt8) .empty

private def converterTests : IO Unit := do
  let c ← get (Converter.create 48000 2)
  let input := pcm #[-32768, 32767, -1, 1, 100, -100, 0, 0]
  let (_, output) ← get (c.convert input)
  check (input == output) "48k stereo must be bit exact"
  let c ← get (Converter.create 48000 1)
  let (_, output) ← get (c.convert (pcm #[-9, 0, 11]))
  check (output == pcm #[-9, -9, 0, 0, 11, 11]) "mono channel duplication"
  check (failed (Converter.create 32000 2) && failed (Converter.create 48000 3))
    "unsupported converter format"
  let c ← get (Converter.create 44100 2)
  check (failed (c.convert (pcm #[7]))) "incomplete frame rejected"
  let mut count : UInt64 := 0
  let mut progress ← get (Converter.create 44100 1)
  for n in [1:10001] do
    let (next, output) ← get (progress.convert (pcm #[1]))
    progress := next
    count := count + (output.size / 4).toUInt64
    check (count == n.toUInt64 * 48000 / 44100) "cumulative rational phase drift"
    check (consumedInputFrames count 44100 n.toUInt64 == n.toUInt64) "inverse frame count"
    check (consumedInputFrames (count - 1) 44100 n.toUInt64 < n.toUInt64) "partial output counted complete"
  for channels in #[1, 2] do
    let mut input := ByteArray.empty
    for i in [0:44100 * channels * 2] do
      let value := ((i * 113) % 65536).toUInt16
      input := input.push value.toUInt8 |>.push (value >>> 8).toUInt8
    let c ← get (Converter.create 44100 channels.toUInt32)
    let (_, whole) ← get (c.convert input)
    check (whole.size / 4 == 96000) "two-second output frame count"
    let mut c := c
    let mut split := ByteArray.empty
    let mut offset := 0
    let mut turn := 0
    while offset < input.size do
      let endOffset := min input.size (offset + ((turn * 17) % 373 + 1) * channels * 2)
      let (next, part) ← get (c.convert (input.extract offset endOffset))
      c := next
      split := split ++ part
      offset := endOffset
      turn := turn + 1
    check (split == whole) "resampling depends on chunk boundaries"
  check (playedInputFrames 4 10 9 48000 == .ok 9) "queue count inverse"
  check (failed (playedInputFrames 3 10 9 48000) &&
    failed (playedInputFrames 44 10 9 48000)) "malformed queue count accepted"

structure QueueState where
  counts : Array UInt32 := #[]
  calls : Array (UInt32 × UInt32) := #[]
  accepted : Nat := 0
  now : Nat := 0
  timePerWrite : Nat := 0
  stalled : Bool := false
  sleeps : Nat := 0
  failAt : Nat := 0

private def queueEffects (ref : IO.Ref QueueState) : QueueEffects (EIO Failure) where
  write _ offset count := do
    let s ← ref.get
    let n := if s.stalled then 0 else s.counts[s.calls.size]?.getD count
    ref.set { s with calls := s.calls.push (offset, count), now := s.now + s.timePerWrite }
    if s.calls.size + 1 == s.failAt then throw (.native "write" 5)
    return n
  accepted frames := ref.modify fun s => { s with accepted := s.accepted + frames.toNat }
  clockNs := return Word64.ofUInt64 (← ref.get).now.toUInt64
  sleepMs duration := ref.modify fun s => { s with now := s.now + duration.toNat * 1000000, sleeps := s.sleeps + 1 }

private def queueRun (initial : QueueState) (data : ByteArray) : IO (Except Failure Unit × QueueState) := do
  let ref ← IO.mkRef initial
  let result ← (submitPcm (queueEffects ref) data).toBaseIO
  return (result, ← ref.get)

private def queueTests : IO Unit := do
  let data := ByteArray.mk (Array.replicate 4096 0)
  let (result, s) ← queueRun { counts := #[0, 4, 8, 1908, 0, 1920] } data
  check (result.isOk && s.accepted == 1024 && s.sleeps == 2) "fragmented/stalled queue did not finish"
  check (s.calls == #[(0,1920), (0,1920), (4,1920), (12,1920), (1920,1920), (1920,1920), (3840,256)])
    "partial-write offsets or 10ms ioctl bound changed"
  let (result, s) ← queueRun { stalled := true } data
  check (result == .error (.native "direct PCM write deadline" 110) &&
    s.sleeps == 2500 && s.now == 5000000000 && s.accepted == 0) s!"backpressure deadline mismatch: result={reprStr result} sleeps={s.sleeps} now={s.now} accepted={s.accepted}"
  let (result, s) ← queueRun { counts := #[4,4,4], timePerWrite := 2000000000 } data
  check (failed result && s.calls.size == 3 && s.accepted == 3) "positive short writes reset shared deadline"
  let (result, s) ← queueRun { counts := #[4], failAt := 2 } data
  check (result == .error (.native "write" 5) && s.accepted == 1) "partial submission lost on transport failure"
  for n in #[2, 1924] do
    let (result, s) ← queueRun { counts := #[n] } data
    check (failed result && s.accepted == 0) "invalid driver write count accepted"
  let (result, s) ← queueRun {} (ByteArray.mk #[0,0])
  check (failed result && s.calls.isEmpty) "unaligned input reached driver"
  let (result, s) ← queueRun {} .empty
  check (result.isOk && s.calls.isEmpty && s.accepted == 0) "empty buffer reached driver"

/-- The common fixture format also feeds the real Rust converter oracle. -/
def differential (input output : String) : IO Unit := do
  let data ← IO.FS.readBinFile input
  let mut c ← get (Converter.create (word32 data) (word32 (data.extract 4 8)))
  let chunks := (word32 (data.extract 8 12)).toNat
  let mut offset := 12
  let mut out := ByteArray.empty
  let mut inputFrames := 0
  let mut outputFrames := 0
  for _ in [0:chunks] do
    let count := (word32 (data.extract offset (offset + 4))).toNat
    offset := offset + 4
    let part := data.extract offset (offset + count)
    offset := offset + count
    inputFrames := inputFrames + count / (c.channels.toNat * 2)
    let (next, converted) ← get (c.convert part)
    c := next
    outputFrames := outputFrames + converted.size / 4
    out := out ++ bytes32 converted.size.toUInt32 ++ converted
  for frames in [0:outputFrames + 1] do
    out := out ++ bytes64 (consumedInputFrames frames.toUInt64 c.rate inputFrames.toUInt64)
  IO.FS.writeBinFile output out

def tests : IO Unit := do
  queueTests
  converterTests
  IO.println "PASS: Lean resampling, mono/stereo, queue bounds, partial writes, backpressure and consumed-frame accounting"

end AudioBackendTests
