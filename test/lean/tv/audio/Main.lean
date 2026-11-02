import TV.Audio.Protocol
import TV.Audio.Direct
import BackendTests
import DeviceTests
import Init

open TV.Audio

namespace AudioTests

instance : Inhabited Progress := ⟨⟨⟨0, 0⟩, ⟨0, 0⟩, 0, 0, 0⟩⟩

structure Harness where
  input : ByteArray := .empty
  position : Nat := 0
  maxRead : Nat := 1
  output : ByteArray := .empty
  blocks : Array ByteArray := #[]
  format : Format := ⟨48000, 2, 1⟩
  gain : UInt32 := 0x3b23d70a -- 0.0025f
  opens : Nat := 0
  closes : Nat := 0
  live : Bool := false
  written : Nat := 0
  holdPlayed : Bool := false
  playedCalls : Nat := 0
  failOpen : Nat := 0
  failWrite : Bool := false
  failPlayed : Nat := 0
  accepted : Bool := true
  now : Nat := 0
  sleeps : Nat := 0
  reports : Array Progress := #[]
  readTimeouts : Array (Option UInt32) := #[]
  writeTimeouts : Array (Option UInt32) := #[]
  deriving Inhabited

abbrev TestM := EIO Failure

def nativeError (name : String) : Failure := .native name 5

def effects (r : IO.Ref Harness) : Effects TestM where
  readTimeout t := r.modify fun s => { s with readTimeouts := s.readTimeouts.push t }
  writeTimeout t := r.modify fun s => { s with writeTimeouts := s.writeTimeouts.push t }
  read n := do
    let s ← r.get
    let n := min n.toNat (min s.maxRead (s.input.size - s.position))
    r.set { s with position := s.position + n }
    return s.input.extract s.position (s.position + n)
  write b := r.modify fun s => { s with output := s.output ++ b }
  supports f := (f.rate == 44100 || f.rate == 48000) && (f.channels == 1 || f.channels == 2)
  openTrack f := do
    let s ← r.get
    let opens := s.opens + 1
    r.set { s with opens }
    if opens == s.failOpen then throw (nativeError "open")
    r.modify fun s => { s with live := true, format := f, written := 0 }
  close := r.modify fun s => if s.live then { s with live := false, closes := s.closes + 1 } else s
  pcm b := do
    let s ← r.get
    if s.failWrite then throw (nativeError "write")
    r.set { s with blocks := s.blocks.push b, written := s.written + b.size / (s.format.channels.toNat * 2) }
  written := return Word64.ofUInt64 (← r.get).written.toUInt64
  played := do
    let s ← r.get
    let calls := s.playedCalls + 1
    r.set { s with playedCalls := calls }
    if calls == s.failPlayed then throw (nativeError "played")
    return Word64.ofUInt64 (if s.holdPlayed then 0 else s.written.toUInt64)
  gainBits := return (← r.get).gain
  clockNs := return Word64.ofUInt64 (← r.get).now.toUInt64
  sleepMs t := r.modify fun s => { s with now := s.now + t.toNat * 1000000, sleeps := s.sleeps + 1 }
  report p := r.modify fun s => { s with reports := s.reports.push p }

def openBytes (rate : UInt32 := 44100) (channels : UInt32 := 1) (boost : UInt32 := 2) : ByteArray :=
  ⟨#[79]⟩ ++ bytes32 rate ++ bytes32 channels ++ bytes32 boost

def samples (values : Array Int16) : ByteArray := values.foldl (fun (b : ByteArray) (s : Int16) =>
  let u := s.toUInt16
  b.push u.toUInt8 |>.push (u >>> 8).toUInt8) .empty

def writeBytes (values : Array Int16) : ByteArray :=
  let data := samples values
  ⟨#[87]⟩ ++ bytes32 data.size.toUInt32 ++ data

def counters (n : UInt64) : ByteArray := bytes64 n ++ bytes64 n

def okStatus : ByteArray := bytes32 0

def badStatus : ByteArray := bytes32 0xffffffea

def failedStatus : ByteArray := bytes32 0xffffffff

def run (initial : Harness) : IO (Except Failure Unit × Harness) := do
  let r ← IO.mkRef initial
  let result ← (client (effects r)).toBaseIO
  return (result, ← r.get)

def check (condition : Bool) (description : String) : IO Unit :=
  unless condition do throw (IO.userError description)

def success (result : Except Failure Unit) : Bool := match result with
  | .ok _ => true
  | .error _ => false

def testProtocol : IO Unit := do
  let stream := openBytes ++ writeBytes #[10000, -10000] ++ ⟨#[68, 82, 68]⟩ ++
    writeBytes #[32767] ++ ⟨#[70, 68]⟩
  let (result, s) ← run { input := stream }
  check (success result) "full protocol failed"
  let expected := okStatus ++ okStatus ++ counters 2 ++ okStatus ++ counters 0 ++
    okStatus ++ okStatus ++ counters 1
  check (s.output == expected) "full protocol response mismatch"
  check (s.blocks == #[samples #[50, -50], samples #[82]]) "boost/gain sample mismatch"
  check (s.opens == 2 && s.closes == 2 && !s.live) "reset and EOF must close exactly once per live track"
  check (s.readTimeouts[0]! == some 3000 && s.readTimeouts.back! == none &&
    s.writeTimeouts == #[some 3000]) "idle/payload deadlines changed"
  check (s.reports.size == 2 && s.reports[0]!.sourcePeak == 10000 &&
    s.reports[0]!.outputPeak == 50 && s.reports[1]!.sourcePeak == 32767) "progress/peak mismatch"

def testInvalid : IO Unit := do
  for data in #[⟨#[68]⟩, openBytes 7999, openBytes 96001, openBytes 48000 0,
      openBytes 48000 3, openBytes 48000 1 0, openBytes 48000 1 5, openBytes 32000] do
    let (result, s) ← run { input := data }
    check (!success result && s.output == badStatus && s.opens == 0 && s.closes == 0)
      "invalid format must fail before backend open"
  for count in #[3, 1048577] do
    let (result, s) ← run { input := openBytes ++ ⟨#[87]⟩ ++ bytes32 count }
    check (!success result && s.output == okStatus ++ badStatus && s.closes == 1)
      "bad block size accepted"
  let (result, s) ← run { input := openBytes ++ ⟨#[63]⟩ }
  check (!success result && s.output == okStatus ++ badStatus && s.closes == 1)
    "unknown command accepted"
  for data in #[ByteArray.empty, ⟨#[79]⟩, openBytes ++ ⟨#[87]⟩ ++ bytes32 4 ++ samples #[1]] do
    let (result, s) ← run { input := data }
    check (!success result) "truncated input accepted"
    check (!s.live) "truncated input leaked track"

def testFailures : IO Unit := do
  let (result, s) ← run { input := openBytes, failOpen := 1 }
  check (!success result && s.output == failedStatus && s.closes == 0) "open failure"
  let (result, s) ← run { input := openBytes ++ ⟨#[82]⟩, failOpen := 2 }
  check (!success result && s.output == okStatus ++ failedStatus && s.closes == 1) "reset failure"
  let (result, s) ← run { input := openBytes ++ writeBytes #[5], failWrite := true }
  check (!success result && s.output == okStatus ++ failedStatus && s.closes == 1) "write failure"
  let (result, s) ← run { input := openBytes ++ ⟨#[68]⟩, failPlayed := 1 }
  check (!success result && s.output == okStatus ++ bytes64 0 && s.closes == 1)
    "counter failure must preserve already-written first counter"
  let (result, s) ← run { input := openBytes ++ ⟨#[70]⟩, failPlayed := 1 }
  check (!success result && s.output == okStatus ++ failedStatus && s.closes == 1) "flush failure"
  let (result, s) ← run { input := openBytes ++ writeBytes #[5], failPlayed := 1 }
  check (!success result && s.output == okStatus ++ okStatus && s.closes == 1)
    "progress read error must not invent extra protocol status"

def testFlushAndMute : IO Unit := do
  let (result, s) ← run { input := openBytes ++ writeBytes #[1000] ++ ⟨#[70, 68]⟩, holdPlayed := true }
  check (success result && s.output == okStatus ++ okStatus ++ failedStatus ++ bytes64 1 ++ bytes64 0)
    "flush timeout must leave session usable"
  check (s.sleeps == 1000 && s.now == 5000000000 && s.closes == 1) "flush deadline/poll interval mismatch"
  let (result, s) ← run { input := openBytes ++ writeBytes #[-32768, 32767], gain := 0 }
  check (success result && s.blocks == #[samples #[0, 0]]) "mute must write silent frames"
  let (result, s) ← run { input := openBytes ++ writeBytes #[], gain := 0 }
  check (success result && s.blocks == #[.empty] && s.reports.isEmpty) "empty write behavior changed"

structure DirectState where
  opens : Nat := 0
  releases : Nat := 0
  live : Bool := false
  output : ByteArray := .empty
  calls : Nat := 0
  stallOnce : Bool := true
  failWrite : Nat := 0
  now : Nat := 0
  sleeps : Nat := 0

def directRun (stream : ByteArray) (initial : DirectState := {}) :
    IO (Except Failure Unit × Harness × DirectState) := do
  let wire ← IO.mkRef ({ input := stream } : Harness)
  let hw ← IO.mkRef initial
  let base := effects wire
  let transport : TV.Audio.Direct.Transport := {
    readTimeout := base.readTimeout, writeTimeout := base.writeTimeout,
    read := base.read, write := base.write, gainBits := base.gainBits, report := base.report }
  let device : TV.Audio.Direct.Device := {
    acquire := do
      let s ← hw.get
      if s.live then throw (.native "device busy" 16)
      hw.set { s with opens := s.opens + 1, live := true }
      return (s.opens + 1).toUInt32
    release := fun _ => hw.modify fun s => { s with live := false, releases := s.releases + 1 }
    write := fun _ data offset count => do
      let s ← hw.get
      hw.set { s with calls := s.calls + 1, stallOnce := false }
      if s.calls + 1 == s.failWrite then throw (.native "write" 5)
      if s.stallOnce then return 0
      let count := min count 4
      hw.modify fun s => { s with output := s.output ++ data.extract offset.toNat (offset + count).toNat }
      return count
    queuedBytes := fun _ => pure 0
    clockNs := return Word64.ofUInt64 (← hw.get).now.toUInt64
    sleepMs := fun ms => hw.modify fun s => { s with now := s.now + ms.toNat * 1000000, sleeps := s.sleeps + 1 }
  }
  let result ← (TV.Audio.Direct.run transport device).toBaseIO
  return (result, ← wire.get, ← hw.get)

def testDirect : IO Unit := do
  let stream := openBytes ++ writeBytes #[10000, -10000] ++ ⟨#[68, 82, 68]⟩ ++
    writeBytes #[32767] ++ ⟨#[70, 68]⟩
  let (result, wire, hw) ← directRun stream
  let expected := okStatus ++ okStatus ++ counters 2 ++ okStatus ++ counters 0 ++
    okStatus ++ okStatus ++ counters 1
  check (success result && wire.output == expected) "direct pipeline wire/counter mismatch"
  check (hw.output == samples #[50,50,-33,-33,82,82]) "direct pipeline PCM conversion mismatch"
  check (hw.opens == 2 && hw.releases == 2 && !hw.live && hw.sleeps == 1 && hw.calls == 4)
    "direct pipeline must reset phase, handle partial writes and release both tracks"
  let (result, wire, hw) ← directRun (openBytes ++ writeBytes #[10000,-10000]) { failWrite := 3 }
  check (!success result && wire.output == okStatus ++ failedStatus && hw.output == samples #[50,50])
    "partial device failure was acknowledged as a complete PCM block"
  check (hw.releases == 1 && !hw.live && wire.reports.isEmpty) "partial failure leaked device"
  IO.println "PASS: Lean direct stream joins protocol, gain, converter, queue, reset and partial-failure cleanup"

def tests : IO Unit := do
  testProtocol
  testInvalid
  testFailures
  testFlushAndMute
  testDirect
  AudioBackendTests.tests
  AudioDeviceTests.tests
  IO.println "PASS: Lean audio protocol, fragmented input, reset, counters, failures, drain, gain and mute"

def differential (input output : String) : IO Unit := do
  let data ← IO.FS.readBinFile input
  let (result, s) ← run { input := data.extract 4 data.size, gain := word32 data, maxRead := 65536 }
  let mut out := ByteArray.mk #[if success result then 0 else 1]
  out := out ++ bytes32 s.opens.toUInt32 ++ bytes32 s.closes.toUInt32
  out := out ++ bytes32 s.output.size.toUInt32 ++ s.output ++ bytes32 s.blocks.size.toUInt32
  for block in s.blocks do
    out := out ++ bytes32 block.size.toUInt32 ++ block
  IO.FS.writeBinFile output out

end AudioTests

def main (args : List String) : IO Unit := match args with
  | ["convert", input, output] => AudioBackendTests.differential input output
  | [input, output] => AudioTests.differential input output
  | [] => AudioTests.tests
  | _ => throw (IO.userError "usage: audioTests [input.bin output.bin]")
