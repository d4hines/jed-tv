import TV.Audio.Fault
import TV.Audio.Gain
import TV.Audio.Resample
import TV.Audio.Protocol
import TV.Audio.Session
open TV.Audio

def samples (xs : List Int16) : ByteArray := xs.foldl Gain.pushInt16 ByteArray.empty

def fail (what : String) : IO α := throw (IO.userError what)

def ensure (ok : Bool) (name : String) : IO Unit :=
  unless ok do throw (IO.userError name)

/-! A mock of every mechanism, over a state monad. It records the ioctl trace
under the same names as the old Rust mock, so the ported cases keep their
expected traces. -/
structure Mock where
  input : ByteArray := ByteArray.empty      -- bytes the app sends
  cursor : Nat := 0
  output : ByteArray := ByteArray.empty     -- bytes we answer
  events : Array String := #[]              -- the ioctl trace
  pcmOut : ByteArray := ByteArray.empty     -- what reached the hardware
  inputs : Array UInt32 := #[]              -- the speaker's input graph
  gain : VolumeDb := ⟨-47, 0⟩
  queue : UInt32 := 0
  clock : UInt64 := 0
  owner : UInt8 := 0
  stops : Nat := 0
  closes : Nat := 0
  retained : Bool := false
  releasedBeforeClose : Bool := false
  globalGain : UInt32 := 0
  writeLimit : UInt32 := 16                 -- the hardware accepts at most this per write
  logs : Array String := #[]
  failOpen : Bool := false
  failStart : Bool := false
  failAttach : Bool := false
  partialAttach : Bool := false
  failDetach : Bool := false
  failClose : Bool := false
  failGainSet : Bool := false
  failGainRestore : Bool := false
  failWrite : Bool := false
  failLookup : Bool := false
  failQueue : Bool := false

abbrev M := StateM Mock

def event (name : String) : M Unit := modify fun s => { s with events := s.events.push name }
def vendor (request : UInt32) : Fault := .vendor request 0xffffffff

def effects : Effects M where
  recvCommand := do
    let s ← get
    if s.cursor < s.input.size then
      set { s with cursor := s.cursor + 1 }
      return some (s.input.get! s.cursor)
    return none
  recvExact := fun n => do
    let s ← get
    if s.cursor + n.toNat <= s.input.size then
      set { s with cursor := s.cursor + n.toNat }
      return some (s.input.extract s.cursor (s.cursor + n.toNat))
    set { s with cursor := s.input.size }
    return none
  send := fun b => do modify fun s => { s with output := s.output ++ b }; return true
  gain := return (← get).globalGain
  now := do let s ← get; set { s with clock := s.clock + 1 }; return s.clock
  sleep := fun ms => modify fun s => { s with clock := s.clock + ms.toUInt64 }
  log := fun text => modify fun s => { s with logs := s.logs.push text }
  acquire := do
    let s ← get
    if s.owner == 0 then
      set { s with owner := 1 }
      return 0
    return s.owner
  release := modify fun s => if s.owner == 1 then { s with owner := 0 } else s
  poison := modify fun s => { s with owner := 2 }
  pcmOpen := do
    event "pcm-open"
    if (← get).failOpen then return .error (vendor 0xc0803402)
    return .ok 0x81000000
  pcmStart := fun _ => do
    event "pcm-start"
    if (← get).failStart then return .error (vendor 0xc0083404)
    return .ok ()
  pcmWrite := fun _ bytes offset len => do
    event "pcm-write"
    let s ← get
    if s.failWrite then return .error (vendor 0xc0383408)
    let n := min len s.writeLimit
    set { s with pcmOut := s.pcmOut ++ bytes.extract offset.toNat (offset.toNat + n.toNat) }
    return .ok n
  pcmQueued := fun _ => do
    let s ← get
    if s.failQueue then return .error (vendor 0xc018340b)
    return .ok s.queue
  pcmStop := fun _ => do
    event "pcm-stop"
    modify fun s => { s with stops := s.stops + 1 }
    return .ok ()
  pcmClose := fun _ => do
    let s ← get
    if s.owner == 0 then set { s with releasedBeforeClose := true }
    event "pcm-close"
    modify fun s => { s with closes := s.closes + 1 }
    if (← get).failClose then return .error (vendor 0xc0043403)
    return .ok ()
  pcmRetain := fun _ => modify fun s => { s with retained := true }
  aoutSpeaker := do
    if (← get).failLookup then return .error (vendor 0xc0081116)
    return .ok 0x17000001
  aoutCount := fun _ => return .ok (← get).inputs.size.toUInt32
  aoutConnected := fun _ count => return .ok ((← get).inputs.extract 0 count.toNat)
  aoutConnect := fun _ input => do
    event "route-connect"
    let s ← get
    if !s.failAttach || s.partialAttach then set { s with inputs := s.inputs.push input }
    if s.failAttach then return .error (vendor 0xc00c111c)
    return .ok ()
  aoutDisconnect := fun _ input => do
    event "route-disconnect"
    let s ← get
    if s.failDetach then return .error (vendor 0xc008111d)
    set { s with inputs := s.inputs.filter (· != input) }
    return .ok ()
  aoutVolume := fun _ => return .ok (← get).gain
  aoutSetVolume := fun _ value => do
    let s ← get
    if value.integer == 0 then
      event "gain-set"
      if s.failGainSet then return .error (vendor 0x40101109)
    else
      event "gain-restore"
      if s.failGainRestore then return .error (vendor 0x40101109)
    modify fun s => { s with gain := value }
    return .ok ()

/-- Run a session action from a mock state. -/
def run (s : Mock) (act : M α) : α × Mock := act.run s

def pcmOpenHandle : UInt32 := 0x81000000

/-- The app's `O` request. -/
def openRequest (rate channels boost : UInt32) : ByteArray :=
  Protocol.pushU32 (Protocol.pushU32 (Protocol.pushU32 (ByteArray.mk #[79]) rate) channels) boost
/-- The app's `W` request. -/
def writeRequest (xs : List Int16) : ByteArray :=
  Protocol.pushU32 (ByteArray.mk #[87]) (xs.length * 2).toUInt32 ++ samples xs
def statusReply (s : Int32) : ByteArray := Protocol.statusBytes s
def countersReply (a b : UInt64) : ByteArray := Protocol.counterBytes a b

def ensureEvents (s : Mock) (expected : List String) (name : String) : IO Unit := do
  unless s.events.toList == expected do
    fail s!"{name}: events {s.events.toList} expected {expected}"

def resampled (rate channels : UInt32) (xs : List Int16) : IO ByteArray := do
  let some c := Resample.Converter.new rate channels | fail "resampled"
  let some (_, out) := c.convert (samples xs) | fail "resampled"
  return out

/-- The open/cleanup cases of the old direct-backend tests, by name. -/
def sessionCases : IO Unit := do
  let openClean (s : Mock) : (Except Fault Unit × Except Fault Unit) × Mock := run s do
    match ← Session.openStream effects 48000 2 with
    | .error f => return (.error f, .ok ())
    | .ok t => return (.ok (), ← Session.cleanup effects t)
  -- 1
  let ((o, c), s) := openClean {}
  ensure (o matches .ok _ && c matches .ok _) "1: open and cleanup succeed"
  ensureEvents s ["pcm-open", "route-connect", "gain-set", "pcm-start", "gain-restore", "route-disconnect", "pcm-stop", "pcm-close"] "1 route_restore_precedes_pcm_stop_and_close"
  ensure (s.inputs.isEmpty && s.owner == 0 && !s.releasedBeforeClose) "1: graph restored and lease released after close"
  -- 2
  let ((o, _), s) := openClean { failOpen := true }
  ensure (o matches .error _) "2: open fails"
  ensureEvents s ["pcm-open"] "2 pcm_open_failure_releases_lease_without_closing_unowned_stream"
  ensure (s.owner == 0) "2: lease released"
  -- 3
  let ((o, _), s) := openClean { failLookup := true }
  ensure (o matches .error _) "3: open fails"
  ensureEvents s ["pcm-open", "pcm-close"] "3 output_lookup_failure_closes_only_owned_pcm"
  ensure (s.owner == 0) "3: lease released"
  -- 4
  let ((o, _), s) := openClean { failAttach := true, partialAttach := true }
  ensure (o matches .error _) "4: open fails"
  ensureEvents s ["pcm-open", "route-connect", "route-disconnect", "pcm-close"] "4 attach_failure_rolls_back_before_closing_unstarted_pcm"
  ensure (s.inputs.isEmpty && s.owner == 0) "4: graph restored, lease released"
  -- 5
  let ((o, _), s) := openClean { failStart := true }
  ensure (o matches .error _) "5: open fails"
  ensureEvents s ["pcm-open", "route-connect", "gain-set", "pcm-start", "gain-restore", "route-disconnect", "pcm-close"] "5 start_failure_restores_route_before_close"
  ensure (s.owner == 0) "5: lease released"
  -- 6
  let (r, s) := run {} do
    let .ok t ← Session.openStream effects 48000 2 | return false
    modify fun s => { s with failWrite := true }
    let .error _ ← Session.write effects t (ByteArray.mk #[0, 0, 0, 0]) | return false
    let .ok () ← Session.cleanup effects t | return false
    return true
  ensure r "6: write fails, cleanup succeeds"
  ensureEvents s ["pcm-open", "route-connect", "gain-set", "pcm-start", "pcm-write", "gain-restore", "route-disconnect", "pcm-stop", "pcm-close"] "6 write_failure_drop_restores_route"
  ensure (s.owner == 0) "6: lease released"
  -- 7
  let (r, s) := run {} do
    let .ok t ← Session.openStream effects 48000 2 | return false
    modify fun s => { s with failDetach := true }
    let .error _ ← Session.cleanup effects t | return false
    return true
  ensure r "7: cleanup reports the failure"
  ensure (s.stops == 0 && s.closes == 0 && s.retained && s.owner == 2) "7 restore_failure_retains_pcm_lifetime_and_blocks_blind_reopen"
  let (o, s) := run s (Session.openStream effects 48000 2)
  ensure (o matches .error (.busy _)) "7: a blind reopen is refused"
  ensure (s.closes == 0) "7: and closes nothing"
  -- 8
  let ((o, _), s) := openClean { failAttach := true, partialAttach := true, failDetach := true }
  ensure (o matches .error _) "8: open fails"
  ensure (s.closes == 0 && s.stops == 0 && s.retained && s.owner == 2) "8 failed_attach_rollback_poison_is_preserved_after_pcm_cleanup"
  -- 9
  let (r, s) := run {} do
    let .ok t ← Session.openStream effects 48000 2 | return false
    modify fun s => { s with failClose := true }
    let .error _ ← Session.cleanup effects t | return false
    return true
  ensure r "9: cleanup reports the failure"
  ensure (s.inputs.isEmpty && s.owner == 2) "9 close_failure_blocks_reset_retry"
  -- 10
  let (r, s) := run {} do
    let .ok t ← Session.openStream effects 48000 2 | return false
    let .ok t ← Session.write effects t (samples [0, 0, 0, 0]) | return false
    if t.inputWritten != 2 then return false
    let .ok () ← Session.cleanup effects t | return false
    let .ok t ← Session.openStream effects 48000 2 | return false
    if t.inputWritten != 0 then return false
    let .ok () ← Session.cleanup effects t | return false
    return true
  ensure r "10: reset sequence"
  let close := s.events.toList.findIdx (· == "pcm-close")
  ensure (s.events.toList.getD (close + 1) "" == "pcm-open") "10 reset_releases_route_before_reopening_with_zero_counters"
  ensure (s.closes == 2 && s.inputs.isEmpty) "10: two closes, graph restored"
  -- 11
  let (r, s) := run {} do
    let .ok t ← Session.openStream effects 48000 2 | return false
    if (← get).gain.integer != 0 then return false
    let .ok () ← Session.cleanup effects t | return false
    return true
  ensure (r && s.gain.integer == -47) "11 default_backend_sets_scoped_gain_and_restores_it"
  -- 12
  let (r, s) := run {} do
    let .ok t ← Session.openStream effects 48000 2 | return false
    modify fun s => { s with failGainRestore := true }
    let .error _ ← Session.cleanup effects t | return false
    return true
  ensure (r && s.inputs.isEmpty && s.closes == 1 && s.owner == 2) "12 failed_gain_restore_poison_does_not_skip_route_cleanup"
  -- 13
  let ((o, _), s) := openClean { failGainSet := true }
  ensure (o matches .error _) "13: open fails"
  ensureEvents s ["pcm-open", "route-connect", "gain-set", "gain-restore", "route-disconnect", "pcm-close"] "13 gain_set_failure_restores_then_detaches_and_closes_before_start"
  ensure (s.gain.integer == -47 && s.owner == 0) "13: gain restored, lease released"
  -- 14
  let input : List Int16 := (List.range 4410).map fun (n : Nat) => Int16.ofInt ((n : Int) * 3 - 6000)
  let whole ← resampled 44100 1 input
  let (r, s) := run {} do
    let .ok t ← Session.openStream effects 44100 1 | return (0, 0, 0)
    let mut t := t
    let mut rest := input
    while !rest.isEmpty do
      let .ok next ← Session.write effects t (samples (rest.take 137)) | return (0, 0, 0)
      t := next
      rest := rest.drop 137
    let .ok played ← Session.played effects t | return (0, 0, 0)
    modify fun s => { s with queue := 400 }
    let .ok later ← Session.played effects t | return (0, 0, 0)
    let .ok () ← Session.cleanup effects t | return (0, 0, 0)
    return (t.inputWritten, played, later)
  ensure (r.1 == 4410 && r.2.1 == 4410) "14: all input accepted and played"
  ensure (s.pcmOut == whole && whole.size / 4 == 4800) "14 partial_hardware_writes_preserve_converted_samples_and_input_counters"
  ensure (r.2.2 == Resample.consumedInputFrames 4700 44100 4410) "14: queued bytes reduce the played count"
  ensure (s.stops == 1 && s.closes == 1 && s.owner == 0) "14: cleanup"
  -- 15
  let (r, _) := run { queue := 4 } do
    let .ok t ← Session.openStream effects 48000 2 | return false
    let .error _ ← Session.played effects t | return false
    return true
  ensure r "15 impossible_queue_is_not_a_fake_playback_counter"
  -- 16
  let (o, s) := run { owner := 1 } (Session.openStream effects 48000 2)
  ensure (o matches .error (.busy _)) "16 single_owner: a second owner is refused"
  ensure (s.events.isEmpty && s.owner == 1) "16: nothing touched, lease kept"
  let ((o, _), s) := openClean { owner := 0 }
  ensure (o matches .ok _ && s.owner == 0) "16 single_owner_and_lease_release"
  -- 17
  let ((o, c), s) := openClean { gain := ⟨0, 0⟩ }
  ensure (o matches .ok _ && c matches .ok _) "17: open and cleanup succeed"
  ensureEvents s ["pcm-open", "route-connect", "pcm-start", "route-disconnect", "pcm-stop", "pcm-close"] "17 volume_noop_does_not_set_hardware"
  -- 18
  let ((o, _), s) := openClean { inputs := #[0x81000005] }
  ensure (o matches .error (.busy _)) "18: refused"
  ensureEvents s ["pcm-open", "pcm-close"] "18 nonempty_default_is_refused_without_write"
  ensure (s.inputs == #[0x81000005]) "18: the other input is untouched"
  -- 19
  let ((o, c), s) := openClean { inputs := #[pcmOpenHandle] }
  ensure (o matches .ok _ && c matches .ok _) "19: borrowed open and cleanup succeed"
  ensureEvents s ["pcm-open", "gain-set", "pcm-start", "gain-restore", "pcm-stop", "pcm-close"] "19 borrowed_route_never_disconnects"
  ensure (s.inputs == #[pcmOpenHandle]) "19: the borrowed route stays"
  -- 20
  let (r, s) := run {} do
    let .ok t ← Session.openStream effects 48000 2 | return false
    modify fun s => { s with inputs := s.inputs.push 0x81000009 }
    let .ok () ← Session.cleanup effects t | return false
    return true
  ensure (r && s.inputs == #[0x81000009]) "20 cleanup_does_not_remove_foreign_new_connection"
  IO.println "PASS: audio session open/cleanup (20 cases)"

/-- The old protocol tests, through `Session.client`. -/
def protocolCases : IO Unit := do
  let play (s : Mock) : Except Fault Unit × Mock := run s (Session.client effects)
  -- 21
  let script := openRequest 44100 1 2 ++ writeRequest [10000, -10000] ++ ByteArray.mk #[68, 82, 68]
    ++ writeRequest [32767] ++ ByteArray.mk #[70, 68]
  let (r, s) := play { input := script, globalGain := 2500 }
  ensure (r matches .ok _) "21: clean session"
  let expected := statusReply 0 ++ statusReply 0 ++ countersReply 2 2 ++ statusReply 0 ++ countersReply 0 0
    ++ statusReply 0 ++ statusReply 0 ++ countersReply 1 1
  ensure (s.output == expected) "21 full_protocol_reset_counters_gain_and_fragmentation: replies"
  let first ← resampled 44100 1 [50, -50]
  let second ← resampled 44100 1 [81]
  ensure (s.pcmOut == first ++ second) "21: scaled samples reached the hardware (81, not Rust's rounded 82)"
  ensure (s.closes == 2 && s.owner == 0) "21: two opens, two closes"
  -- 22
  for (rate, channels, boost) in [(32000, 2, 1), (48000, 3, 1), (48000, 1, 0), (48000, 1, 5)] do
    let (r, s) := play { input := openRequest rate channels boost }
    ensure (r matches .error _ && s.output == statusReply (-22) && s.events.isEmpty) "22 unsupported_formats_are_einval_before_open"
  -- 23
  let (r, s) := play { input := ByteArray.mk #[68] }
  ensure (r matches .error _ && s.output == statusReply (-22)) "23: bad first command"
  let (r, s) := play { input := openRequest 48000 2 1 ++ ByteArray.mk #[63] }
  ensure (r matches .error _ && s.output == statusReply 0 ++ statusReply (-22) && s.closes == 1) "23 bad_first_command_and_unknown_command_are_einval"
  -- 24
  for n in [3, 1048577] do
    let (r, s) := play { input := openRequest 48000 2 1 ++ Protocol.pushU32 (ByteArray.mk #[87]) n }
    ensure (r matches .error _ && s.output == statusReply 0 ++ statusReply (-22) && s.closes == 1) "24 invalid_block_sizes_are_einval_and_close_owner"
  -- 25
  let (r, s) := play { input := openRequest 48000 2 1, failOpen := true }
  ensure (r matches .error _ && s.output == statusReply (-1)) "25: open failure"
  let (r, s) := play { input := openRequest 48000 2 1 ++ writeRequest [0, 0], failWrite := true }
  ensure (r matches .error _ && s.output == statusReply 0 ++ statusReply (-1) && s.closes == 1) "25 open_and_write_failures_return_abi_error"
  -- 26
  let (r, s) := play { input := openRequest 48000 2 1 ++ Protocol.pushU32 (ByteArray.mk #[87]) 4 ++ ByteArray.mk #[0] }
  ensure (r matches .error _ && s.output == statusReply 0 && s.closes == 1) "26 truncated_payload_closes_owner_without_write"
  ensure (!s.events.contains "pcm-write") "26: nothing written"
  -- 27
  let (r, s) := play { input := openRequest 48000 2 4 ++ writeRequest [32767, -32768] }
  ensure (r matches .ok _ && s.pcmOut.toList.all (· == 0) && s.pcmOut.size == 4) "27 zero_global_gain_is_silent"
  -- 28
  let (r, s) := play { input := openRequest 48000 2 1 ++ ByteArray.mk #[70], failQueue := true }
  ensure (r matches .error _ && s.output == statusReply 0 ++ statusReply (-1) && s.closes == 1) "28 drain_counter_failure_returns_status_and_closes_owner"
  -- 29
  let script := openRequest 44100 1 1 ++ Protocol.pushU32 (ByteArray.mk #[87]) 4 ++ ByteArray.mk #[0, 0, 0, 0] ++ ByteArray.mk #[68, 82, 68, 70]
  let (r, s) := play { input := script }
  ensure (r matches .ok _) "29: clean session"
  let expected := statusReply 0 ++ statusReply 0 ++ countersReply 2 2 ++ statusReply 0 ++ countersReply 0 0 ++ statusReply 0
  ensure (s.output == expected) "29: replies"
  let close := s.events.toList.findIdx (· == "pcm-close")
  ensure (s.closes == 2 && s.inputs.isEmpty && s.events.toList.getD (close + 1) "" == "pcm-open") "29 real_shared_protocol_reset_restores_route_before_next_open"
  ensure ((s.events.filter (· == "route-disconnect")).size == 2) "29: both routes restored"
  -- a flush that never drains answers -1 and keeps serving
  let (r, s) := play { input := openRequest 48000 2 1 ++ writeRequest [1, 1] ++ ByteArray.mk #[70, 68], queue := 4 }
  ensure (r matches .ok _ && s.output == statusReply 0 ++ statusReply 0 ++ statusReply (-1) ++ countersReply 1 0) "30 flush timeout keeps serving"
  IO.println "PASS: audio session protocol (10 cases)"

/-- Build a result word the way Rust does: tag in the low byte, value in the high word. -/
def word (tag : UInt64) (value : UInt32) : UInt64 := tag ||| (value.toUInt64 <<< 32)

def main : IO Unit := do
  ensure (outcome 0xc0803402 (word 0 0x81000000) == .ok 0x81000000) "ok word carries the value"
  ensure (outcome 7 (word 1 5) == .error (.vendor 7 5)) "tag 1 is the MI status"
  ensure (outcome 7 (word 2 11) == .error (.os 11)) "tag 2 is an errno"
  ensure ((Fault.vendor 0xc0803402 5).describe == "MI request 0xc0803402: status 5") "vendor text"
  IO.println "PASS: audio faults"

  -- The Rust protocol test's numbers: boost 2, gain 0.0025 (2500 millionths).
  ensure (Gain.scale 2 2500 10000 == 50 && Gain.scale 2 2500 (-10000) == -50) "boost then gain"
  ensure (Gain.scale 2 2500 32767 == 81) "clamp to 32767 before gain, truncate (Rust rounded to 82)"
  ensure (Gain.scale 4 0 32767 == 0 && Gain.scale 4 0 (-32768) == 0) "zero gain is silent"
  ensure (Gain.scale 4 300000 32767 == 9830 && Gain.scale 4 300000 (-32768) == -9830) "ceiling"
  ensure (Gain.scale 1 999999999 32767 == 9830) "gain above the ceiling is clamped"
  ensure (Gain.scale 1 1000000 100 == 30) "unit gain still passes through the ceiling"
  let block := Gain.scaleBlock 2 2500 (Gain.pushInt16 (Gain.pushInt16 ByteArray.empty 10000) (-10000))
  ensure (block.output == Gain.pushInt16 (Gain.pushInt16 ByteArray.empty 50) (-50)) "block scales each sample"
  ensure (block.sourcePeak == 10000 && block.outputPeak == 50) "peaks"
  ensure (Gain.readInt16 (Gain.pushInt16 ByteArray.empty (-2)) 0 == -2) "LE round trip"
  ensure ((Gain.pushInt16 ByteArray.empty (-2)).toList == [0xfe, 0xff]) "LE bytes"
  IO.println "PASS: audio gain"

  let some c := Resample.Converter.new 48000 2 | fail "48k stereo"
  let data := samples [-32768, 32767, -1, 1, 100, -100, 0, 0]
  let some (_, out) := c.convert data | fail "convert"
  ensure (out == data) "stereo 48k is bit-exact"
  let some m := Resample.Converter.new 48000 1 | fail "mono"
  let some (_, out) := m.convert (samples [-9, 0, 11]) | fail "convert"
  ensure (out == samples [-9, -9, 0, 0, 11, 11]) "mono is duplicated"
  for channels in [1, 2] do
    let seconds := 7
    let n := 44100 * channels * seconds
    let data := (List.range n).foldl (fun b (i : Nat) => Gain.pushInt16 b (Int16.ofInt (((i * 113) % 65536 : Nat) - 32768))) ByteArray.empty
    let some c := Resample.Converter.new 44100 channels.toUInt32 | fail "44.1k"
    let some (_, whole) := c.convert data | fail "whole"
    ensure (whole.size / 4 == 48000 * seconds) "44.1k input becomes the same duration of 48k"
    let mut split := c
    let mut output := ByteArray.empty
    let mut pos := 0
    let mut turn := 0
    while pos < data.size do
      let frames := (turn * 17) % 373 + 1
      let stop := min (pos + frames * channels * 2) data.size
      let some (next, piece) := split.convert (data.extract pos stop) | fail "piece"
      split := next; output := output ++ piece; pos := stop; turn := turn + 1
    ensure (output == whole) "chunk boundaries do not change the output"
  let some c := Resample.Converter.new 44100 1 | fail "44.1k mono"
  let mut conv := c
  let mut count : UInt64 := 0
  for n in [1:10001] do
    let some (next, piece) := conv.convert (samples [1]) | fail "one"
    conv := next; count := count + (piece.size / 4).toUInt64
    ensure (count == (n * 48000 / 44100).toUInt64) "cumulative floor count"
    ensure (Resample.consumedInputFrames count 44100 n.toUInt64 == n.toUInt64) "inverse"
    if count > 0 then ensure (Resample.consumedInputFrames (count - 1) 44100 n.toUInt64 < n.toUInt64) "strict inverse"
  ensure (Resample.consumedInputFrames 0 44100 100 == 0) "nothing consumed"
  let some c := Resample.Converter.new 44100 1 | fail "44.1k mono"
  let some (_, out) := c.convert (samples [-32768, 32767, -32768, 32767, 0, 32767]) | fail "bounded"
  for i in [0:out.size / 4] do
    ensure (Gain.readInt16 out (4 * i) == Gain.readInt16 out (4 * i + 2)) "mono output has equal channels"
  let some c := Resample.Converter.new 44100 2 | fail "44.1k stereo"
  let some (_, out) := c.convert (ByteArray.mk (Array.replicate 1764 0)) | fail "silence"
  ensure (out.toList.all (· == 0)) "silence stays silent"
  ensure ((Resample.Converter.new 32000 2).isNone && (Resample.Converter.new 48000 3).isNone) "unsupported formats"
  let some c := Resample.Converter.new 44100 2 | fail "44.1k stereo"
  ensure (c.convert (samples [7]) |>.isNone) "incomplete frame"
  IO.println "PASS: audio resampler"

  ensure (Protocol.validFormat 44100 1 2 && Protocol.validFormat 48000 2 1 && Protocol.validFormat 48000 2 4) "supported formats"
  ensure (!(Protocol.validFormat 32000 2 1) && !(Protocol.validFormat 48000 3 1) && !(Protocol.validFormat 48000 1 0) && !(Protocol.validFormat 48000 1 5)) "rejected formats"
  ensure (Protocol.validBlock 2 0 && Protocol.validBlock 2 4 && Protocol.validBlock 1 2 && Protocol.validBlock 2 1048576) "valid blocks"
  ensure (!(Protocol.validBlock 2 3) && !(Protocol.validBlock 2 1048577) && !(Protocol.validBlock 2 2)) "invalid blocks"
  ensure (Protocol.statusBytes (-22) == ByteArray.mk #[0xea, 0xff, 0xff, 0xff]) "EINVAL bytes"
  ensure (Protocol.counterBytes 2 1 == ByteArray.mk #[2,0,0,0,0,0,0,0, 1,0,0,0,0,0,0,0]) "counter bytes"
  ensure (Protocol.readU32 (ByteArray.mk #[0x44, 0xac, 0, 0]) 0 == 44100) "LE u32"
  ensure (Protocol.readU64 (Protocol.pushU64 ByteArray.empty 0x1122334455667788) 0 == 0x1122334455667788) "LE u64 round trip"
  IO.println "PASS: audio protocol"

  sessionCases
  protocolCases
