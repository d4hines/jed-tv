prelude
import Init.Control.Except
import Init.Data.ByteArray.Basic
import Init.Data.SInt.Float32
import Init.Data.Range.Basic

/-! The existing O/W/D/R/F PCM stream, interpreted in Lean. Linux sockets,
backend handles and clocks remain effects; no device operation is hidden here.
PCM on this boundary is interleaved signed 16-bit little-endian, at the input
rate/channels. The device backend still converts it to 48 kHz stereo. -/
namespace TV.Audio

/-- Scalar halves keep effect results portable to ARM32 without a boxed
64-bit scalar. Arithmetic stays UInt64 after decoding. -/
structure Word64 where
  low : UInt32
  high : UInt32
  deriving BEq, Repr, Inhabited

def Word64.ofUInt64 (value : UInt64) : Word64 :=
  ⟨value.toUInt32, (value >>> 32).toUInt32⟩

def Word64.toUInt64 (value : Word64) : UInt64 :=
  value.low.toUInt64 ||| (value.high.toUInt64 <<< 32)

structure Format where
  rate : UInt32
  channels : UInt32
  boost : UInt32
  deriving BEq, Repr

def Format.valid (f : Format) : Bool :=
  8000 ≤ f.rate && f.rate ≤ 96000 && 1 ≤ f.channels && f.channels ≤ 2 &&
  1 ≤ f.boost && f.boost ≤ 4

inductive Failure where
  | eof
  | protocol (message : String)
  | native (operation : String) (errno : UInt32)
  deriving BEq, Repr

structure Progress where
  written : Word64
  played : Word64
  sourcePeak : Nat
  outputPeak : Nat
  gainBits : UInt32
  deriving BEq, Repr

/-- Native operations only. read returns at most the requested bytes; empty
means EOF. Writes consume the whole byte array or throw. close is idempotent
and must not throw, so a failed reset/open also gets safe cleanup. gainBits is
an IEEE binary32 value already bounded by the owner's calibrated [0,0.30]
policy. Backend counters count whole input frames, not samples or bytes. -/
structure Effects (m : Type → Type) where
  readTimeout : Option UInt32 → m Unit
  writeTimeout : Option UInt32 → m Unit
  read : UInt32 → m ByteArray
  write : ByteArray → m Unit
  supports : Format → Bool
  openTrack : Format → m Unit
  close : m Unit
  pcm : ByteArray → m Unit
  written : m Word64
  played : m Word64
  gainBits : m UInt32
  clockNs : m Word64
  sleepMs : UInt32 → m Unit
  report : Progress → m Unit

def bytes32 (value : UInt32) : ByteArray :=
  ⟨#[value.toUInt8, (value >>> 8).toUInt8, (value >>> 16).toUInt8, (value >>> 24).toUInt8]⟩

def bytes64 (value : UInt64) : ByteArray := Id.run do
  let mut out := ByteArray.empty
  for i in [0:8] do out := out.push ((value >>> (i * 8).toUInt64).toUInt8)
  return out

def word32 (b : ByteArray) : UInt32 :=
  b[0]!.toUInt32 ||| (b[1]!.toUInt32 <<< 8) |||
  (b[2]!.toUInt32 <<< 16) ||| (b[3]!.toUInt32 <<< 24)

structure Mixed where
  bytes : ByteArray
  sourcePeak : Nat
  outputPeak : Nat

/-- Same operation order as the Rust processor: binary32 boost, saturate to
±32767, binary32 gain, round half away from zero, then signed-16 output.
In particular, amplification is clamped BEFORE master attenuation. -/
def mix (data : ByteArray) (boost gainBits : UInt32) : Mixed := Id.run do
  let gain := Float32.ofBits gainBits
  let limit := (32767 : UInt32).toFloat32
  let mut out := ByteArray.empty
  let mut sourcePeak := 0
  let mut outputPeak := 0
  for i in [0:data.size / 2] do
    let raw := data[2*i]!.toUInt16 ||| (data[2*i+1]!.toUInt16 <<< 8)
    let sample := raw.toInt16
    let amplified := sample.toFloat32 * boost.toFloat32
    let bounded := if amplified < -limit then -limit
      else if amplified > limit then limit else amplified
    let result := (bounded * gain).round.toInt16
    let bits := result.toUInt16
    out := out.push bits.toUInt8 |>.push (bits >>> 8).toUInt8
    sourcePeak := max sourcePeak sample.toInt.natAbs
    outputPeak := max outputPeak result.toInt.natAbs
  return ⟨out, sourcePeak, outputPeak⟩

partial def readExact [Monad m] [MonadExceptOf Failure m] (e : Effects m)
    (count : Nat) (soFar : ByteArray := .empty) : m ByteArray := do
  if soFar.size == count then return soFar
  let chunk ← e.read (count - soFar.size).toUInt32
  if chunk.isEmpty then throw .eof
  if chunk.size > count - soFar.size then throw (.protocol "read exceeded requested bound")
  readExact e count (soFar ++ chunk)

def status [Monad m] (e : Effects m) (value : UInt32) : m Unit := e.write (bytes32 value)
def invalid [Monad m] [MonadExceptOf Failure m] {α : Type} (e : Effects m)
    (message : String) : m α := do
  status e 0xffffffea -- -EINVAL
  throw (.protocol message)

def openFormat [Monad m] [MonadExceptOf Failure m] (e : Effects m) : m Format := do
  e.readTimeout (some 3000)
  e.writeTimeout (some 3000)
  if (← readExact e 1)[0]! != 79 then invalid e "expected open"
  let rate := word32 (← readExact e 4)
  let channels := word32 (← readExact e 4)
  let boost := word32 (← readExact e 4)
  let f := Format.mk rate channels boost
  unless f.valid && e.supports f do invalid e "invalid PCM format"
  return f

def openTrack [Monad m] [MonadExceptOf Failure m] (e : Effects m) (f : Format) : m Unit := do
  try e.openTrack f catch error =>
    status e 0xffffffff
    throw error

partial def drain [Monad m] [MonadExceptOf Failure m] (e : Effects m)
    (deadline : Nat) : m Unit := do
  let playedWord ← try e.played catch error =>
    status e 0xffffffff
    throw error
  let played := playedWord.toUInt64
  if played ≥ (← e.written).toUInt64 then status e 0
  else if (← e.clockNs).toUInt64.toNat ≥ deadline then status e 0xffffffff
  else
    e.sleepMs 5
    drain e deadline

structure Counters where
  nextReport : Word64 := ⟨1, 0⟩
  sourcePeak : Nat := 0
  outputPeak : Nat := 0

partial def commands [Monad m] [MonadExceptOf Failure m] (e : Effects m)
    (f : Format) (c : Counters := {}) : m Unit := do
  -- Idle/paused clients have no deadline; command payloads remain bounded.
  e.readTimeout none
  let command ← e.read 1
  if command.isEmpty then return
  if command.size != 1 then throw (.protocol "read exceeded requested bound")
  e.readTimeout (some 3000)
  let mut c := c
  match command[0]! with
  | 87 => -- W: byte length, then PCM
    let count := word32 (← readExact e 4)
    if count > 1048576 || count % (f.channels * 2) != 0 then invalid e "invalid PCM block"
    let data ← readExact e count.toNat
    let gain ← e.gainBits
    let mixed := mix data f.boost gain
    c := { c with sourcePeak := max c.sourcePeak mixed.sourcePeak, outputPeak := max c.outputPeak mixed.outputPeak }
    try e.pcm mixed.bytes catch error =>
      status e 0xffffffff
      throw error
    status e 0
    let written := (← e.written).toUInt64
    if written ≥ c.nextReport.toUInt64 then
      let played := (← e.played).toUInt64
      e.report ⟨Word64.ofUInt64 written, Word64.ofUInt64 played, c.sourcePeak, c.outputPeak, gain⟩
      c := { nextReport := Word64.ofUInt64 (written + f.rate.toUInt64 * 5) }
  | 68 => -- D: accepted/consumed input-frame counters
    e.write (bytes64 (← e.written).toUInt64)
    e.write (bytes64 (← e.played).toUInt64)
  | 82 => -- R: drop the previous track before opening its replacement
    e.close
    openTrack e f
    c := { c with nextReport := ⟨1, 0⟩ }
    status e 0
  | 70 => -- F: bounded drain; timeout reports failure but keeps the session open
    drain e ((← e.clockNs).toUInt64.toNat + 5000000000)
  | _ => invalid e "unknown command"
  commands e f c

/-- Run one complete stream, releasing the backend on EOF and every failure
once opening succeeded. All protocol control lives here; native code only
implements Effects. -/
def client [Monad m] [MonadExceptOf Failure m] [MonadFinally m] (e : Effects m) : m Unit := do
  let f ← openFormat e
  openTrack e f
  try
    status e 0
    commands e f
  finally e.close

end TV.Audio
