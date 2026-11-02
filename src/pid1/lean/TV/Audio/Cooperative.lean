prelude
import Init.System.IO
import TV.Audio.Converter
import TV.Audio.Device

/-! Finite audio work for PID1's single Lean executor. Each tick performs at
most 16 transitions per client; no read, write, sleep or drain loop blocks.
Native code implements individual nonblocking socket/device operations. -/
namespace TV.Audio.Cooperative
open TV.Audio

structure ReadResult where
  data : ByteArray
  status : UInt32 -- 0=data, 1=would block, 2=EOF, high bit=errno

structure Effects (m : Type → Type) where
  accept : UInt32 → m UInt32 -- slot -> 1=accepted, 0=would block, high bit=errno
  read : UInt32 → UInt32 → m ReadResult
  write : UInt32 → ByteArray → UInt32 → UInt32 → m UInt32
  close : UInt32 → m Unit -- idempotent socket + any remaining device cleanup
  acquire : UInt32 → m UInt32 -- 0=success, high bit=errno
  release : UInt32 → m Unit -- exclusive lease is poisoned on uncertain cleanup
  pcm : UInt32 → ByteArray → UInt32 → UInt32 → m UInt32
  queued : UInt32 → m UInt32 -- queued bytes, or high bit=errno
  gain : m UInt32 -- finite calibrated IEEE binary32 gain
  report : UInt32 → Progress → m Unit

inductive Phase where
  | opening | command | length | data (bytes : Nat)
  | emit (source : ByteArray) (offset : Nat) (output : ByteArray) (sent : Nat) (gain : UInt32)
  | report (gain : UInt32)
  | drain
  | closing

structure Client where
  slot : UInt32
  phase : Phase := .opening
  format : Format := ⟨48000,2,1⟩
  converter : Converter := ⟨48000,2,0,none⟩
  buffer : ByteArray := .empty
  reply : ByteArray := .empty
  replyOffset : Nat := 0
  deadline : Word64 := ⟨0,0⟩
  replyDeadline : Word64 := ⟨0,0⟩
  retryAt : Word64 := ⟨0,0⟩
  written : Word64 := ⟨0,0⟩
  outputWritten : Word64 := ⟨0,0⟩
  nextReport : Word64 := ⟨1,0⟩
  sourcePeak : Nat := 0
  outputPeak : Nat := 0

abbrev Server := Array (Option Client)

def failed (word : UInt32) : Bool := word &&& 0x80000000 != 0
private def time (word : Word64) : Nat := word.toUInt64.toNat
private def later (now : Word64) (ms : UInt64) : Word64 := Word64.ofUInt64 (now.toUInt64 + ms)
private def response (c : Client) (bytes : ByteArray) (now : Word64) : Client :=
  { c with reply := bytes, replyOffset := 0, replyDeadline := later now 3000 }
private def status (c : Client) (code : UInt32) (now : Word64) : Client := response c (bytes32 code) now
private def error (c : Client) (code : UInt32) (now : Word64) : Client :=
  status { c with phase := .closing } code now

private def readMore [Monad m] (e : Effects m) (now : Word64) (c : Client)
    (wanted : Nat) : m (Client × Bool) := do
  if time now ≥ time c.deadline then return ({c with phase := .closing}, true)
  let r ← e.read c.slot (min 65536 (wanted - c.buffer.size)).toUInt32
  if r.status == 1 then return (c, false)
  if r.status != 0 || r.data.isEmpty || r.data.size > wanted - c.buffer.size then
    return ({c with phase := .closing}, true)
  return ({c with buffer := c.buffer ++ r.data, deadline := later now 3000}, true)

private def played [Monad m] (e : Effects m) (c : Client) : m (Option Word64) := do
  let queued ← e.queued c.slot
  if failed queued || queued % 4 != 0 || queued.toUInt64 / 4 > c.outputWritten.toUInt64 then
    return none
  return some (Word64.ofUInt64 (consumedInputFrames (c.outputWritten.toUInt64 - queued.toUInt64 / 4)
    c.format.rate c.written.toUInt64))

/-- One transition or one nonblocking native operation; Bool says more work
can progress immediately. False yields this client until a future tick. -/
def advance [Monad m] (e : Effects m) (now : Word64) (c : Client) : m (Option Client × Bool) := do
  if c.replyOffset < c.reply.size then
    if time now ≥ time c.replyDeadline then
      e.close c.slot
      return (none, false)
    let n ← e.write c.slot c.reply c.replyOffset.toUInt32 (c.reply.size - c.replyOffset).toUInt32
    if n == 0 then return (some c, false)
    if failed n || n.toNat > c.reply.size - c.replyOffset then
      e.close c.slot
      return (none, false)
    return (some {c with replyOffset := c.replyOffset + n.toNat}, true)
  if time now < time c.retryAt then return (some c, false)
  match c.phase with
  | .closing =>
    e.close c.slot
    return (none, false)
  | .opening =>
    if !c.buffer.isEmpty && c.buffer[0]! != 79 then return (some (error c 0xffffffea now), true)
    if c.buffer.size < 13 then
      let (c, more) ← readMore e now c 13
      return (some c, more)
    if c.buffer[0]! != 79 then return (some (error c 0xffffffea now), true)
    let f : Format := ⟨word32 (c.buffer.extract 1 5), word32 (c.buffer.extract 5 9), word32 (c.buffer.extract 9 13)⟩
    match Converter.create f.rate f.channels with
    | .error _ => return (some (error c 0xffffffea now), true)
    | .ok converter =>
      if !f.valid then return (some (error c 0xffffffea now), true)
      if failed (← e.acquire c.slot) then return (some (error c 0xffffffff now), true)
      return (some (status {c with format := f, converter, buffer := .empty, phase := .command} 0 now), true)
  | .command =>
    let r ← e.read c.slot 1
    if r.status == 1 then return (some c, false)
    if r.status != 0 || r.data.size != 1 then
      e.close c.slot
      return (none, false)
    let c := {c with buffer := .empty, deadline := later now 3000}
    match r.data[0]! with
    | 87 => return (some {c with phase := .length}, true)
    | 68 =>
      match ← played e c with
      | some n => return (some (response c (bytes64 c.written.toUInt64 ++ bytes64 n.toUInt64) now), true)
      | none => return (some (response {c with phase := .closing} (bytes64 c.written.toUInt64) now), true)
    | 82 =>
      e.release c.slot
      if failed (← e.acquire c.slot) then return (some (error c 0xffffffff now), true)
      let converter := {c.converter with phase := 0, previous := none}
      return (some (status {c with converter, written := ⟨0,0⟩, outputWritten := ⟨0,0⟩, nextReport := ⟨1,0⟩} 0 now), true)
    | 70 => return (some {c with phase := .drain, deadline := later now 5000}, true)
    | _ => return (some (error c 0xffffffea now), true)
  | .length =>
    if c.buffer.size < 4 then
      let (c, more) ← readMore e now c 4
      return (some c, more)
    let bytes := (word32 c.buffer).toNat
    if bytes > 1048576 || bytes % (c.format.channels.toNat * 2) != 0 then
      return (some (error c 0xffffffea now), true)
    return (some {c with buffer := .empty, phase := .data bytes}, true)
  | .data bytes =>
    if c.buffer.size < bytes then
      let (c, more) ← readMore e now c bytes
      return (some c, more)
    let gain ← e.gain
    return (some {c with phase := .emit c.buffer 0 .empty 0 gain, buffer := .empty, deadline := later now 5000}, true)
  | .emit source offset output sent gain =>
    if sent < output.size then
      if time now ≥ time c.deadline then return (some (error c 0xffffffff now), true)
      let count := min 1920 (output.size - sent)
      let n ← e.pcm c.slot output sent.toUInt32 count.toUInt32
      if failed n || n.toNat > count || n % 4 != 0 then return (some (error c 0xffffffff now), true)
      if n == 0 then return (some {c with retryAt := later now 2}, false)
      return (some {c with phase := .emit source offset output (sent + n.toNat) gain, outputWritten := Word64.ofUInt64 (c.outputWritten.toUInt64 + n.toUInt64 / 4)}, true)
    if offset < source.size then
      let endOffset := min source.size (offset + 8192)
      let mixed := mix (source.extract offset endOffset) c.format.boost gain
      match c.converter.convert mixed.bytes with
      | .error _ => return (some (error c 0xffffffff now), true)
      | .ok (converter, bytes) =>
        return (some {c with converter, phase := .emit source endOffset bytes 0 gain, sourcePeak := max c.sourcePeak mixed.sourcePeak, outputPeak := max c.outputPeak mixed.outputPeak}, true)
    let written := c.written.toUInt64 + (source.size / (c.format.channels.toNat * 2)).toUInt64
    let phase := if written ≥ c.nextReport.toUInt64 then Phase.report gain else .command
    return (some (status {c with phase, written := Word64.ofUInt64 written} 0 now), true)
  | .report gain =>
    match ← played e c with
    | none => return (some {c with phase := .closing}, true)
    | some n =>
      e.report c.slot ⟨c.written, n, c.sourcePeak, c.outputPeak, gain⟩
      return (some {c with phase := .command, sourcePeak := 0, outputPeak := 0, nextReport := Word64.ofUInt64 (c.written.toUInt64 + c.format.rate.toUInt64 * 5)}, true)
  | .drain =>
    match ← played e c with
    | none => return (some (error c 0xffffffff now), true)
    | some n =>
      if n.toUInt64 ≥ c.written.toUInt64 then return (some (status {c with phase := .command} 0 now), true)
      if time now ≥ time c.deadline then return (some (status {c with phase := .command} 0xffffffff now), true)
      return (some {c with retryAt := later now 5}, false)

private def service [Monad m] (e : Effects m) (now : Word64) : Nat → Client → m (Option Client)
  | 0, c => pure (some c)
  | fuel + 1, c => do
    let (next, more) ← advance e now c
    if !more then return next
    match next with
    | none => pure none
    | some c => service e now fuel c

private def tickSlots [Monad m] (e : Effects m) (now : Word64) : Nat → Nat → Server → m Server
  | 0, _, server => pure server
  | fuel + 1, slot, server => do
    let server ← match server[slot]! with
      | some c => do pure (server.set! slot (← service e now 16 c))
      | none => do
        if (← e.accept slot.toUInt32) == 1 then
          pure (server.set! slot (some {slot := slot.toUInt32, deadline := later now 3000}))
        else pure server
    tickSlots e now fuel (slot + 1) server

def tick [Monad m] (e : Effects m) (now : Word64) (server : Server) : m Server :=
  tickSlots e now 4 0 server

@[extern "tv_audio_server_accept"] opaque nativeAccept (slot : UInt32) : IO UInt32
@[extern "tv_audio_server_read"] opaque nativeRead (slot count : UInt32) : IO ReadResult
@[extern "tv_audio_server_write"] opaque nativeWrite (slot : UInt32) (bytes : @& ByteArray) (offset count : UInt32) : IO UInt32
@[extern "tv_audio_server_close"] opaque nativeClose (slot : UInt32) : IO Unit
@[extern "tv_audio_server_pcm"] opaque nativePcm (slot : UInt32) (bytes : @& ByteArray) (offset count : UInt32) : IO UInt32
@[extern "tv_audio_server_queued"] opaque nativeQueued (slot : UInt32) : IO UInt32
@[extern "tv_audio_server_gain"] opaque nativeGain : IO UInt32
@[extern "tv_audio_server_now"] opaque nativeNow : IO Word64
@[extern "tv_audio_server_report"] opaque nativeReport (slot low high playedLow playedHigh sourcePeak outputPeak gain : UInt32) : IO Unit

@[extern "tv_audio_device_step"] private opaque nativeDeviceStep (command : UInt32) : IO UInt32
private def deviceResult (code : UInt32) : Device.Result :=
  if code == 0 then .ok else if code == 2 then .uncertain else .failed
private def nativeDeviceEffects : Device.Effects IO where
  openPcm := return deviceResult (← nativeDeviceStep 0)
  openOutput := return deviceResult (← nativeDeviceStep 1)
  attach := return deviceResult (← nativeDeviceStep 2)
  setVolume := return deviceResult (← nativeDeviceStep 3)
  start := return deviceResult (← nativeDeviceStep 4)
  restoreVolume := return deviceResult (← nativeDeviceStep 5)
  restoreRoute := return deviceResult (← nativeDeviceStep 6)
  closePcm := return deviceResult (← nativeDeviceStep 7)
  retainPcm := discard (nativeDeviceStep 8)
  dropOutput := return deviceResult (← nativeDeviceStep 9)

abbrev NativeM := StateT Device.State IO

def nativeEffects : Effects NativeM where
  accept slot := liftM (nativeAccept slot)
  read slot count := liftM (nativeRead slot count)
  write slot bytes offset count := liftM (nativeWrite slot bytes offset count)
  close slot := do
    Device.release nativeDeviceEffects slot
    liftM (nativeClose slot)
  acquire := Device.acquire nativeDeviceEffects
  release := Device.release nativeDeviceEffects
  pcm slot bytes offset count := do
    if !(← get).owns slot then return 0x80000009
    liftM (nativePcm slot bytes offset count)
  queued slot := do
    if !(← get).owns slot then return 0x80000009
    liftM (nativeQueued slot)
  gain := liftM nativeGain
  report slot p := liftM <|
    nativeReport slot p.written.low p.written.high p.played.low p.played.high p.sourcePeak.toUInt32 p.outputPeak.toUInt32 p.gainBits

structure NativeState where
  clients : Server := Array.replicate 4 none
  device : Device.State := {}

@[export tv_audio_initial] def initial (_ : UInt32) : NativeState := {}
@[export tv_audio_tick] def runTick (state : NativeState) : IO NativeState := do
  let now ← nativeNow
  let (clients, device) ← (tick nativeEffects now state.clients).run state.device
  return {clients, device}
@[export tv_audio_io_u32] def ioU32 (n : UInt32) : IO UInt32 := pure n
@[export tv_audio_io_unit] def ioUnit (_ : UInt32) : IO Unit := pure ()
@[export tv_audio_io_now] def ioNow (low high : UInt32) : IO Word64 := pure ⟨low,high⟩
@[export tv_audio_io_read] def ioRead (bytes : ByteArray) (status : UInt32) : IO ReadResult := pure ⟨bytes,status⟩
@[export tv_audio_alloc_bytes] def allocateBytes (size : UInt32) : ByteArray :=
  ByteArray.mk (Array.replicate size.toNat 0)

end TV.Audio.Cooperative
