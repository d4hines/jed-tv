prelude
import Init.System.IO
import TV.Audio.Queue

/-! One complete direct audio session. Lean owns protocol decisions, sample
conversion, queue submission and input/output accounting. Native effects only
own the session transport and the exclusive, initialized 48 kHz stereo device. -/
namespace TV.Audio.Direct
open TV.Audio

abbrev AudioM := EIO Failure

structure Transport where
  readTimeout : Option UInt32 → AudioM Unit
  writeTimeout : Option UInt32 → AudioM Unit
  read : UInt32 → AudioM ByteArray
  write : ByteArray → AudioM Unit
  gainBits : AudioM UInt32
  report : Progress → AudioM Unit

/-- `acquire` acquires one exclusive device configured for 48 kHz stereo s16.
The handle is nonzero. A failed open must clean up its partial resources.
`release` never throws: it attempts cleanup once, reports any failure through
native diagnostics and poisons the exclusive lease if ownership is uncertain.
`write` is one nonblocking ioctl, and `queuedBytes` is one queue observation.
No native effect resamples, loops over writes or advances counters. -/
structure Device where
  acquire : AudioM UInt32
  release : UInt32 → BaseIO Unit
  write : UInt32 → ByteArray → UInt32 → UInt32 → AudioM UInt32
  queuedBytes : UInt32 → AudioM UInt32
  clockNs : AudioM Word64
  sleepMs : UInt32 → AudioM Unit

structure Track where
  handle : UInt32
  converter : Converter
  inputWritten : Word64 := ⟨0, 0⟩
  outputWritten : Word64 := ⟨0, 0⟩

private def current (state : IO.Ref (Option Track)) : AudioM Track := do
  match ← state.get with
  | some track => return track
  | none => throw (.protocol "audio device is closed")

private def close (state : IO.Ref (Option Track)) (device : Device) : AudioM Unit := do
  let track ← state.get
  state.set none
  if let some track := track then device.release track.handle

private def openTrack (state : IO.Ref (Option Track)) (device : Device) (format : Format) : AudioM Unit := do
  if (← state.get).isSome then throw (.protocol "audio device is already open")
  let converter ← match Converter.create format.rate format.channels with
    | .ok value => pure value
    | .error error => throw error
  let handle ← device.acquire
  if handle == 0 then throw (.protocol "native audio returned an invalid handle")
  state.set (some { handle, converter })

private def queueEffects (state : IO.Ref (Option Track)) (device : Device) : QueueEffects AudioM where
  write bytes offset count := do
    device.write (← current state).handle bytes offset count
  accepted frames := do
    let track ← current state
    let next := Word64.ofUInt64 (track.outputWritten.toUInt64 + frames.toUInt64)
    state.set (some { track with outputWritten := next })
  clockNs := device.clockNs
  sleepMs := device.sleepMs

private def pcm (state : IO.Ref (Option Track)) (device : Device) (samples : ByteArray) : AudioM Unit := do
  let track ← current state
  let (converter, output) ← match track.converter.convert samples with
    | .ok value => pure value
    | .error error => throw error
  state.set (some { track with converter })
  submitPcm (queueEffects state device) output
  let track ← current state
  let frames := (samples.size / (converter.channels.toNat * 2)).toUInt64
  state.set (some { track with inputWritten := Word64.ofUInt64 (track.inputWritten.toUInt64 + frames) })

private def played (state : IO.Ref (Option Track)) (device : Device) : AudioM Word64 := do
  let track ← current state
  let queued ← device.queuedBytes track.handle
  match playedInputFrames queued track.outputWritten.toUInt64 track.inputWritten.toUInt64 track.converter.rate with
  | .ok value => pure (Word64.ofUInt64 value)
  | .error error => throw error

def effects (state : IO.Ref (Option Track)) (transport : Transport) (device : Device) : Effects AudioM where
  readTimeout := transport.readTimeout
  writeTimeout := transport.writeTimeout
  read := transport.read
  write := transport.write
  supports f := (f.rate == 44100 || f.rate == 48000) && (f.channels == 1 || f.channels == 2)
  openTrack := openTrack state device
  close := close state device
  pcm := pcm state device
  written := return (← current state).inputWritten
  played := played state device
  gainBits := transport.gainBits
  clockNs := device.clockNs
  sleepMs := device.sleepMs
  report := transport.report

/-- Call once for each accepted socket. Each session has isolated counters
and conversion phase; the native device enforces the global exclusive lease. -/
def run (transport : Transport) (device : Device) : AudioM Unit := do
  let state ← IO.mkRef none
  client (effects state transport device)

end TV.Audio.Direct
