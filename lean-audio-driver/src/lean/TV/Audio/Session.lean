prelude
import Init.Control.Basic
import Init.Data.UInt
import Init.Data.SInt.Basic
import Init.Data.ByteArray.Basic
import Init.Data.Array.Basic
import Init.Data.Range
import Init.Data.Option.Basic
import Init.Data.ToString
import TV.Audio.Fault
import TV.Audio.Gain
import TV.Audio.Resample
import TV.Audio.Protocol

/- One app connection playing through the warm, already initialized MI
hardware. Everything here is a decision; every mechanism (socket, clock,
lease, ioctl) is a field of `Effects`, supplied by Rust on the TV and by a
mock in the checks. -/
namespace TV.Audio

/-- The speaker's gain as the hardware reports it. Restored exactly as read. -/
structure VolumeDb where
  integer : Int32
  thousandths : Int32
  deriving BEq, Repr, Inhabited

structure Effects (m : Type → Type) where
  /-- The next command byte; waits as long as it takes. `none` when the app
  closed the socket or the read failed. -/
  recvCommand : m (Option UInt8)
  /-- Exactly `n` bytes within 3 seconds, or `none`. -/
  recvExact : UInt32 → m (Option ByteArray)
  /-- All of the bytes within 3 seconds. -/
  send : ByteArray → m Bool
  /-- The global gain in millionths of full scale (clamped here, not there). -/
  gain : m UInt32
  /-- Monotonic milliseconds. -/
  now : m UInt64
  sleep : UInt32 → m Unit
  log : String → m Unit
  /-- The single-owner lease: 0 acquired, 1 owned elsewhere, 2 poisoned. -/
  acquire : m UInt8
  /-- Owned → free; a poisoned lease stays poisoned. -/
  release : m Unit
  poison : m Unit
  pcmOpen : m (Except Fault UInt32)
  pcmStart : UInt32 → m (Except Fault Unit)
  /-- handle, buffer, offset, length → bytes the hardware accepted. -/
  pcmWrite : UInt32 → ByteArray → UInt32 → UInt32 → m (Except Fault UInt32)
  /-- Bytes still queued in the hardware. -/
  pcmQueued : UInt32 → m (Except Fault UInt32)
  pcmStop : UInt32 → m (Except Fault Unit)
  pcmClose : UInt32 → m (Except Fault Unit)
  /-- Keep the device open for the rest of the process: its memory may still
  be referenced by the speaker. -/
  pcmRetain : UInt32 → m Unit
  /-- The existing speaker output's handle, unchecked. -/
  aoutSpeaker : m (Except Fault UInt32)
  aoutCount : UInt32 → m (Except Fault UInt32)
  /-- output, count → the connected input handles. -/
  aoutConnected : UInt32 → UInt32 → m (Except Fault (Array UInt32))
  aoutConnect : UInt32 → UInt32 → m (Except Fault Unit)
  aoutDisconnect : UInt32 → UInt32 → m (Except Fault Unit)
  aoutVolume : UInt32 → m (Except Fault VolumeDb)
  aoutSetVolume : UInt32 → VolumeDb → m (Except Fault Unit)

namespace Session
variable {m : Type → Type} [Monad m]

/-- A speaker output created by the stock HAL has this handle prefix. -/
@[inline] def validSpeaker (handle : UInt32) : Bool := handle >>> 24 == 0x17
def maxConnections : UInt32 := 32

def joined (failures : Array String) : String :=
  failures.foldl (fun acc f => acc ++ "; " ++ f) ""

/-- The speaker's input list is only trusted when the count did not change
while it was read, no handle is zero and none repeats. -/
def checkInputs (handles : Array UInt32) : Except Fault (Array UInt32) := Id.run do
  let mut seen : Array UInt32 := #[]
  for h in handles do
    if h == 0 then return .error (.invalid "AOUT snapshot contains zero handle")
    if seen.contains h then return .error (.invalid "duplicate AOUT input")
    seen := seen.push h
  return .ok seen

def readInputs (e : Effects m) (speaker : UInt32) : m (Except Fault (Option (Array UInt32))) := do
  let count ← match ← e.aoutCount speaker with
    | .error f => return .error f
    | .ok count => pure count
  if count > maxConnections then return .error (.invalid "AOUT connection count exceeds bound")
  let handles ← if count == 0 then pure (.ok #[]) else e.aoutConnected speaker count
  let handles ← match handles with
    | .error f => return .error f
    | .ok handles => pure handles
  match ← e.aoutCount speaker with
  | .error f => return .error f
  | .ok again =>
    if again > maxConnections then return .error (.invalid "AOUT connection count exceeds bound")
    if again != count then return .ok none
    return .ok (some handles)

/-- Up to `attempts` consistent reads of the speaker's inputs. -/
def snapshotAttempts (e : Effects m) (speaker : UInt32) : Nat → m (Except Fault (Array UInt32))
  | 0 => pure (.error (.busy "AOUT graph changed while taking snapshot"))
  | attempts + 1 => do
    match ← readInputs e speaker with
    | .error f => return .error f
    | .ok none => snapshotAttempts e speaker attempts
    | .ok (some handles) => return checkInputs handles

/-- Three attempts at a consistent read of the speaker's inputs. -/
def snapshot (e : Effects m) (speaker : UInt32) : m (Except Fault (Array UInt32)) :=
  snapshotAttempts e speaker 3

/-- Disconnect our input if it is still there, then verify it is gone. We only
ever attach to a speaker that had no inputs, so there is nothing else to put
back; inputs added by others meanwhile are never touched. A snapshot failure
before any write makes no write at all. -/
def restoreRoute (e : Effects m) (speaker pcm : UInt32) : m (Except Fault Unit) := do
  let current ← match ← snapshot e speaker with
    | .error f => return .error f
    | .ok current => pure current
  let mut failures : Array String := #[]
  if current.contains pcm then
    match ← e.aoutDisconnect speaker pcm with
    | .error f => failures := failures.push ("disconnect owned input: " ++ f.describe)
    | .ok () => pure ()
  match ← snapshot e speaker with
  | .ok after =>
    if after.contains pcm then failures := failures.push "verify restore: our input is still connected"
  | .error f => failures := failures.push ("verify restore snapshot: " ++ f.describe)
  if failures.isEmpty then return .ok ()
  return .error (.invalid ("AOUT restore incomplete" ++ joined failures))

inductive AttachFailure where
  /-- Nothing was changed. -/
  | refused (fault : Fault)
  /-- The connect failed or did not take; the graph was restored. -/
  | rolledBack (fault : Fault)
  /-- The restore failed too: our input may still be connected. -/
  | unresolved (fault rollback : Fault)

/-- `true`: we connected our PCM and cleanup must disconnect it. `false`: the
speaker already had exactly our PCM as input (a route to borrow, not own).
Any other input means someone else is playing, and we refuse. -/
def attach (e : Effects m) (speaker pcm : UInt32) : m (Except AttachFailure Bool) := do
  let before ← match ← snapshot e speaker with
    | .error f => return .error (.refused f)
    | .ok before => pure before
  if before.contains pcm then return .ok false
  if !before.isEmpty then
    return .error (.refused (.busy "speaker already has inputs; explicit preservation policy required"))
  let result : Except Fault Unit ← do
    match ← e.aoutConnect speaker pcm with
    | .error f => pure (.error f)
    | .ok () =>
      match ← snapshot e speaker with
      | .error f => pure (.error f)
      | .ok after =>
        if after.contains pcm then pure (.ok ())
        else pure (.error (.invalid "connect did not preserve requested graph"))
  match result with
  | .ok () => return .ok true
  | .error cause =>
    match ← restoreRoute e speaker pcm with
    | .ok () => return .error (.rolledBack cause)
    | .error rollback => return .error (.unresolved cause rollback)

/-- The only hardware attenuation we ever ask for: 0 dB, so that the global
software gain is the one thing that sets loudness. -/
def target : VolumeDb := ⟨0, 0⟩

def verifyVolume (e : Effects m) (speaker : UInt32) (expected : VolumeDb) : m (Except Fault Unit) := do
  match ← e.aoutVolume speaker with
  | .error f => return .error f
  | .ok actual =>
    if actual != expected then return .error (.invalid "speaker gain readback differs from what was set")
    return .ok ()

def restoreVolume (e : Effects m) (speaker : UInt32) (original : VolumeDb) : m (Except Fault Unit) := do
  match ← e.aoutSetVolume speaker original with
  | .error f => return .error f
  | .ok () => verifyVolume e speaker original

inductive VolumeFailure where
  | rolledBack (fault : Fault)
  | unresolved (fault rollback : Fault)

/-- Take the speaker to `target` with readback; the original is remembered
for cleanup. Nothing is written when the speaker is already there. -/
def takeVolume (e : Effects m) (speaker : UInt32) : m (Except VolumeFailure (VolumeDb × Bool)) := do
  let original ← match ← e.aoutVolume speaker with
    | .error f => return .error (.rolledBack f)
    | .ok original => pure original
  if original == target then return .ok (original, false)
  let result : Except Fault Unit ← do
    match ← e.aoutSetVolume speaker target with
    | .error f => pure (.error f)
    | .ok () => verifyVolume e speaker target
  match result with
  | .ok () => return .ok (original, true)
  | .error cause =>
    match ← restoreVolume e speaker original with
    | .ok () => return .error (.rolledBack cause)
    | .error rollback => return .error (.unresolved cause rollback)

/-- An open stream and everything cleanup has to undo. -/
structure Track where
  pcm : UInt32
  started : Bool
  speaker : UInt32
  /-- We connected our PCM to the speaker; cleanup must disconnect it. -/
  attached : Bool
  /-- The speaker's original gain, and whether we changed it. -/
  volume : Option (VolumeDb × Bool)
  /-- A route restore already failed: the PCM must then outlive this session. -/
  routeUnresolved : Bool
  converter : Resample.Converter
  rate : UInt32
  channels : UInt32
  /-- Input frames accepted from the app. -/
  inputWritten : UInt64
  /-- 48 kHz frames accepted by the hardware. -/
  outputWritten : UInt64

/-- Speaker gain first, then the route, then the PCM: stop (if started) and
close, each exactly once. A route that cannot be restored keeps the PCM open
and poisons the lease; any other failure also poisons it. The lease is
released only after the PCM was closed. -/
def cleanup (e : Effects m) (t : Track) : m (Except Fault Unit) := do
  let mut failures : Array String := #[]
  let mut unresolved := t.routeUnresolved
  if let some (original, changed) := t.volume then
    if changed then
      match ← restoreVolume e t.speaker original with
      | .error f => failures := failures.push ("speaker gain restore: " ++ f.describe)
      | .ok () => pure ()
  if t.attached then
    match ← restoreRoute e t.speaker t.pcm with
    | .error f =>
      unresolved := true
      failures := failures.push ("route restore: " ++ f.describe)
    | .ok () => pure ()
  if unresolved then
    e.poison
    e.pcmRetain t.pcm
    e.release
    return .error (.invalid ("direct audio cleanup failed; retained PCM " ++ hex t.pcm
      ++ "; explicit recovery required" ++ joined failures))
  if t.started then
    match ← e.pcmStop t.pcm with
    | .error f => failures := failures.push ("PCM stop: " ++ f.describe)
    | .ok () => pure ()
  match ← e.pcmClose t.pcm with
  | .error f => failures := failures.push ("PCM close: " ++ f.describe)
  | .ok () => pure ()
  if failures.isEmpty then
    e.release
    return .ok ()
  e.poison
  e.release
  return .error (.invalid ("direct audio cleanup failed" ++ joined failures))

/-- Undo a partly opened track and report why opening stopped. -/
def abandon (e : Effects m) (t : Track) (fault : Fault) : m (Except Fault Track) := do
  match ← cleanup e t with
  | .error f => e.log ("LEAN_AUDIO cleanup after failed open: " ++ f.describe)
  | .ok () => pure ()
  return .error fault

/-- Lease, PCM open, speaker lookup, route, speaker gain, PCM start. Every
failure after the lease undoes what was done, in cleanup order. -/
def openStream (e : Effects m) (rate channels : UInt32) : m (Except Fault Track) := do
  let some converter := Resample.Converter.new rate channels
    | return .error (.invalid "direct audio supports 44.1/48k mono/stereo")
  match ← e.acquire with
  | 0 => pure ()
  | 2 => return .error (.busy "direct PCM cleanup failed; explicit recovery required before reuse")
  | _ => return .error (.busy "direct PCM is already owned")
  let pcm ← match ← e.pcmOpen with
    | .error f => e.release; return .error f
    | .ok 0 => e.release; return .error (.invalid "MI PCM Open returned a zero handle")
    | .ok handle => pure handle
  let mut track : Track := {
    pcm, started := false, speaker := 0, attached := false, volume := none, routeUnresolved := false,
    converter, rate, channels, inputWritten := 0, outputWritten := 0 }
  let speaker ← match ← e.aoutSpeaker with
    | .error f => return ← abandon e track f
    | .ok handle =>
      if validSpeaker handle then pure handle
      else return ← abandon e track (.invalid "invalid existing AOUT handle")
  track := { track with speaker }
  match ← attach e speaker pcm with
  | .ok attached => track := { track with attached }
  | .error (.refused f) => return ← abandon e track f
  | .error (.rolledBack f) => return ← abandon e track (.invalid ("AOUT attach failed: " ++ f.describe))
  | .error (.unresolved f rollback) =>
    return ← abandon e { track with routeUnresolved := true }
      (.invalid ("AOUT attach failed: " ++ f.describe ++ "; rollback failed: " ++ rollback.describe))
  match ← takeVolume e speaker with
  | .ok volume => track := { track with volume := some volume }
  | .error (.rolledBack f) =>
    return ← abandon e track (.invalid ("speaker volume takeover failed: " ++ f.describe))
  | .error (.unresolved f rollback) =>
    e.poison
    return ← abandon e track (.invalid ("speaker volume takeover failed: " ++ f.describe
      ++ "; restore failed: " ++ rollback.describe))
  match ← e.pcmStart pcm with
  | .error f => return ← abandon e track f
  | .ok () => return .ok { track with started := true }

/-- Never hand the hardware more than 10 ms at a time. -/
def chunkBytes : UInt32 := 1920
def writeDeadlineMs : UInt64 := 5000

/-- Convert the app's frames and feed them to the hardware under one shared
deadline. Short and zero writes are retried; nothing is skipped. -/
def write (e : Effects m) (t : Track) (samples : ByteArray) : m (Except Fault Track) := do
  let some (converter, output) := t.converter.convert samples
    | return .error (.invalid "incomplete PCM frame")
  let deadline := (← e.now) + writeDeadlineMs
  let total := output.size.toUInt32
  let mut offset : UInt32 := 0
  let mut outputWritten := t.outputWritten
  -- Every chunk plus every 2 ms back-off the deadline allows; the clock is the real limit.
  for _ in [0:output.size / chunkBytes.toNat + 2600] do
    if offset >= total then break
    if (← e.now) >= deadline then return .error (.timeout "direct PCM write deadline")
    let len := min chunkBytes (total - offset)
    match ← e.pcmWrite t.pcm output offset len with
    | .error f => return .error f
    | .ok n =>
      if n > len || n % 4 != 0 then return .error (.invalid "invalid MI write count")
      offset := offset + n
      outputWritten := outputWritten + (n / 4).toUInt64
      if n == 0 then e.sleep 2
  if offset < total then return .error (.timeout "direct PCM write stalled")
  let frames := (samples.size / (t.channels.toNat * 2)).toUInt64
  return .ok { t with converter, outputWritten, inputWritten := t.inputWritten + frames }

/-- Input frames whose sound has left the hardware queue. -/
def played (e : Effects m) (t : Track) : m (Except Fault UInt64) := do
  match ← e.pcmQueued t.pcm with
  | .error f => return .error f
  | .ok queued =>
    let queuedFrames := (queued / 4).toUInt64
    if queued % 4 != 0 || queuedFrames > t.outputWritten then
      return .error (.invalid "inconsistent hardware PCM queue")
    return .ok (Resample.consumedInputFrames (t.outputWritten - queuedFrames) t.rate t.inputWritten)

def flushDeadlineMs : UInt64 := 5000

/-- Wait until everything written has played, polling every 5 ms for at most
5 seconds. `false` means the deadline passed. -/
def flush (e : Effects m) (t : Track) : m (Except Fault Bool) := do
  let deadline := (← e.now) + flushDeadlineMs
  for _ in [0:1100] do
    match ← played e t with
    | .error f => return .error f
    | .ok p =>
      if p >= t.inputWritten then return .ok true
      if (← e.now) >= deadline then return .ok false
      e.sleep 5
  return .ok false

def sendFault : Fault := .os 32

/-- End the session: clean up exactly once, log a cleanup failure, report `result`. -/
def finish (e : Effects m) (t : Track) (result : Except Fault Unit) : m (Except Fault Unit) := do
  match ← cleanup e t with
  | .error f => e.log ("LEAN_AUDIO cleanup: " ++ f.describe)
  | .ok () => pure ()
  return result

@[inline] def status (e : Effects m) (s : Int32) : m Bool := e.send (Protocol.statusBytes s)

/-- The command loop after a successful open. Reports progress every five
seconds of audio. -/
partial def serve (e : Effects m) (rate channels boost : UInt32) (t : Track) (nextReport : UInt64) : m (Except Fault Unit) := do
  let some command ← e.recvCommand | finish e t (.ok ())
  match command with
  | 87 => -- 'W'
    let some header ← e.recvExact 4 | finish e t (.error (.invalid "truncated block length"))
    let bytes := Protocol.readU32 header 0
    if !Protocol.validBlock channels bytes then
      let _ ← status e Protocol.einval
      return ← finish e t (.error (.invalid "invalid PCM block"))
    let some data ← e.recvExact bytes | finish e t (.error (.invalid "truncated PCM block"))
    let gain ← e.gain
    let scaled := Gain.scaleBlock boost gain data
    match ← write e t scaled.output with
    | .error f =>
      let _ ← status e Protocol.failure
      finish e t (.error f)
    | .ok t =>
      if !(← status e Protocol.ok) then return ← finish e t (.error sendFault)
      if t.inputWritten >= nextReport then
        match ← played e t with
        | .error f => finish e t (.error f)
        | .ok p =>
          e.log s!"LEAN_AUDIO written={t.inputWritten} played={p} source_peak={scaled.sourcePeak} output_peak={scaled.outputPeak} gain={gain}"
          serve e rate channels boost t (t.inputWritten + rate.toUInt64 * 5)
      else serve e rate channels boost t nextReport
  | 68 => -- 'D'
    match ← played e t with
    | .error f => finish e t (.error f)
    | .ok p =>
      if !(← e.send (Protocol.counterBytes t.inputWritten p)) then return ← finish e t (.error sendFault)
      serve e rate channels boost t nextReport
  | 82 => -- 'R'
    match ← cleanup e t with
    | .error f => e.log ("LEAN_AUDIO reset cleanup: " ++ f.describe)
    | .ok () => pure ()
    match ← openStream e rate channels with
    | .error f =>
      let _ ← status e Protocol.failure
      return .error f
    | .ok t =>
      if !(← status e Protocol.ok) then return ← finish e t (.error sendFault)
      serve e rate channels boost t 1
  | 70 => -- 'F'
    match ← flush e t with
    | .error f =>
      let _ ← status e Protocol.failure
      finish e t (.error f)
    | .ok drained =>
      if !(← status e (if drained then Protocol.ok else Protocol.failure)) then
        return ← finish e t (.error sendFault)
      serve e rate channels boost t nextReport
  | _ =>
    let _ ← status e Protocol.einval
    finish e t (.error (.invalid "unknown command"))

/-- One connection: the `O` handshake, then `serve`. The first byte must
arrive within the 3 second timeout. -/
def client (e : Effects m) : m (Except Fault Unit) := do
  let some first ← e.recvExact 1 | return .error (.invalid "closed before open")
  if first.get! 0 != 79 then -- 'O'
    let _ ← status e Protocol.einval
    return .error (.invalid "expected open")
  let some header ← e.recvExact 12 | return .error (.invalid "truncated open request")
  let rate := Protocol.readU32 header 0
  let channels := Protocol.readU32 header 4
  let boost := Protocol.readU32 header 8
  if !Protocol.validFormat rate channels boost then
    let _ ← status e Protocol.einval
    return .error (.invalid "invalid PCM format")
  match ← openStream e rate channels with
  | .error f =>
    let _ ← status e Protocol.failure
    return .error f
  | .ok t =>
    if !(← status e Protocol.ok) then return ← finish e t (.error sendFault)
    serve e rate channels boost t 1

end Session
end TV.Audio
