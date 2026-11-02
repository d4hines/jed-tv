# Lean audio driver: implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Replace the Rust logic of the G06 PID1 direct audio driver (`src/audio/`) with Lean modules that own every decision, leaving Rust only the raw ioctls, sockets, clock and lease atomic.

**Architecture:** A Lean program over an `Effects m` record (one field per mechanism), compiled to C and linked into PID1 exactly like `TV/Volume.lean`. Rust implements each field as an `@[extern]`; a Lean mock over `StateM` implements it in tests and records the ioctl trace. Spec: `research/LEAN_AUDIO_DRIVER.txt`.

**Tech Stack:** Lean 4.35.0 (core only, `prelude` style), Lake package `src/lean` (buildDir `assets/build/lake/tv`), Rust 1.92 crates `src/audio` (`g06-audio`) and `src/pid1` (`hello-pid1`, feature `repl`), zig cc for ARM32, one C marshalling file compiled with the Lean include directory.

## Global Constraints

- Lean toolchain `leanprover/lean4:v4.35.0`; no Mathlib, no new Lake dependencies.
- Lean modules start with `prelude` and import only the `Init` modules they need (as `TV/Volume.lean` does); generated C must not call `initialize_Init(` (checked by the Python gate).
- Gain unit: millionths of full scale, ceiling 300000 (`TV.Volume.ceiling`). Maximum output sample magnitude 9830.
- Socket ABI byte-identical to today: commands `O W D R F`, statuses `0 / -1 / -22` as little-endian `i32`, counters as little-endian `u64`.
- No `partial def` in anything the three future proofs talk about (`Gain.scale`, `Resample.convert`, `Session.open`, `Session.cleanup`, `Session.write`). The connection loop may be `partial`.
- Build one Lake module per edit: `cd src/lean && lake build TV.Audio.<Module>`. Never run two `lake` processes at once.
- Lean on PATH: `export PATH=/nix/store/qw8qrxx59fzagy3a4y4khcq37iw3yzx9-lean4-4.35.0/bin:$PATH` (or `nix develop .#bootloader -c ...`).
- Commit after every task on branch `lean-audio`; never force-push.

---

## File structure

| Path | Responsibility |
|---|---|
| `src/lean/TV/Audio/Fault.lean` | `Fault`: why a mechanism failed; `describe` for logs; `outcome` decoding of Rust result words |
| `src/lean/TV/Audio/Gain.lean` | `scale`: one sample through boost and global gain; `scaleBlock`: a block of LE i16 bytes plus peaks |
| `src/lean/TV/Audio/Resample.lean` | `Converter`: 44.1k/48k mono/stereo to 48k stereo; `consumedInputFrames` |
| `src/lean/TV/Audio/Protocol.lean` | Pure decisions of the socket protocol: validation, status bytes, counters |
| `src/lean/TV/Audio/Session.lean` | `Effects m`; open sequence, write, played, flush, cleanup, connection loop |
| `src/lean/TV/Audio/Native.lean` | `@[extern]` declarations, the `IO` `Effects`, `@[export tv_audio_client]` |
| `src/lean/native/audio.c` | ByteArray/String marshalling between the generated C and Rust |
| `test/lean/tv/audio/Check.lean` | Mock `Effects` over `StateM`, ported unit cases, `#guard` checks |
| `test/lean/tv/check-audio.py` | Builds the modules, runs `Check.lean`, checks the generated C's initializers |
| `src/audio/mi-pcm.rs` | Wire structs and one function per PCM ioctl (reduced) |
| `src/audio/aout-route.rs` | Wire structs and one function per AOUT ioctl (reduced) |
| `src/audio/lib.rs` | Exports only `mi_pcm` and `aout_route` |
| `src/pid1/src/audio_service.rs` | Listener, per-connection thread, lease atomic, every extern |
| `tools/pid1/build.rs` | Compiles the new Lean modules and `native/audio.c` into `libpid1lean.a` |

---

### Task 1: Fault and result-word decoding

**Files:**
- Create: `src/lean/TV/Audio/Fault.lean`
- Create: `test/lean/tv/audio/Check.lean` (first cases)
- Create: `test/lean/tv/check-audio.py`

**Interfaces:**
- Produces:
  ```lean
  inductive TV.Audio.Fault
    | vendor (request status : UInt32) | os (errno : UInt32)
    | invalid (what : String) | timeout (what : String) | busy (what : String)
  def TV.Audio.Fault.describe : Fault → String
  def TV.Audio.outcome (request : UInt32) (word : UInt64) : Except Fault UInt32
  ```
  Result word: bits 0..7 tag (0 ok, 1 vendor status, 2 errno), bits 32..63 value.

- [x] **Step 1: Write the failing check**

`test/lean/tv/audio/Check.lean`:
```lean
import TV.Audio.Fault
open TV.Audio

def ensure (ok : Bool) (name : String) : IO Unit :=
  unless ok do throw (IO.userError name)

def main : IO Unit := do
  ensure (outcome 0xc0803402 (0x81000000 <<< 32) == .ok 0x81000000) "ok word carries the value"
  ensure (outcome 7 (1 ||| (5 <<< 32)) == .error (.vendor 7 5)) "tag 1 is the MI status"
  ensure (outcome 7 (2 ||| (11 <<< 32)) == .error (.os 11)) "tag 2 is an errno"
  ensure ((Fault.vendor 0xc0803402 5).describe == "MI request 0xc0803402: status 5") "vendor text"
  IO.println "PASS: audio faults"
```

`test/lean/tv/check-audio.py`:
```python
#!/usr/bin/env python3
"""Check PID1's Lean audio driver against a mock of its mechanisms."""
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
LEAN = ROOT / "src/lean"
subprocess.run(["lake", "build", "TV.Audio.Native"], cwd=LEAN, check=True)
subprocess.run(["lake", "env", "lean", "--run", str(ROOT / "test/lean/tv/audio/Check.lean")], cwd=LEAN, check=True)
for module in ("Audio/Fault", "Audio/Gain", "Audio/Resample", "Audio/Protocol", "Audio/Session", "Audio/Native"):
    code = (ROOT / "assets/build/lake/tv/ir/TV" / (module + ".c")).read_text()
    assert "initialize_Init(" not in code and "initialize_Init_Data_Random(" not in code, module
print("PASS: audio avoids full Init and random initializers")
```
Until Task 6 exists, run the module list with only the modules built so far (edit the tuple as modules appear; the final tuple is the one above).

- [x] **Step 2: Run it to verify it fails**

Run: `cd src/lean && lake build TV.Audio.Fault`
Expected: error, no such module.

- [x] **Step 3: Write the module**

`src/lean/TV/Audio/Fault.lean`:
```lean
prelude
import Init.Data.UInt
import Init.Data.ToString.Basic
import Init.Data.Repr

/-! Why a mechanism (an ioctl, a socket read, the clock) did not do what the
driver asked. Rust reports each outcome as one word so that no Rust code has
to interpret it. -/
namespace TV.Audio

inductive Fault where
  /-- The MI call completed but returned a nonzero status. -/
  | vendor (request status : UInt32)
  /-- The system call itself failed. -/
  | os (errno : UInt32)
  /-- The mechanism answered, but with a value the driver refuses to trust. -/
  | invalid (what : String)
  | timeout (what : String)
  /-- The hardware is owned by someone else right now. -/
  | busy (what : String)
  deriving BEq, Repr, Inhabited

def hex (value : UInt32) : String :=
  let digits := "0123456789abcdef".toList.toArray
  let rec go (n : Nat) (v : UInt32) (acc : List Char) : List Char :=
    match n with
    | 0 => acc
    | n + 1 => go n (v >>> 4) (digits[(v &&& 15).toNat]! :: acc)
  "0x" ++ String.mk (go 8 value [])

def Fault.describe : Fault → String
  | .vendor request status => s!"MI request {hex request}: status {status}"
  | .os errno => s!"errno {errno}"
  | .invalid what => what
  | .timeout what => s!"{what}: timed out"
  | .busy what => what

/-- Bits 0..7 of a result word are the tag (0 success, 1 the MI call's nonzero
status, anything else an errno); bits 32..63 carry the value. -/
def outcome (request : UInt32) (word : UInt64) : Except Fault UInt32 :=
  match (word &&& 0xff).toUInt32 with
  | 0 => .ok (word >>> 32).toUInt32
  | 1 => .error (.vendor request (word >>> 32).toUInt32)
  | _ => .error (.os (word >>> 32).toUInt32)

instance : BEq (Except Fault UInt32) where
  beq a b := match a, b with
    | .ok x, .ok y => x == y
    | .error x, .error y => x == y
    | _, _ => false

end TV.Audio
```
If `hex` fights the `prelude` imports, replace it with `toString value` and adjust the expected text in the check; the exact format is not part of the ABI.

- [x] **Step 4: Build and run the check**

Run: `cd src/lean && lake build TV.Audio.Fault && lake env lean --run ../../test/lean/tv/audio/Check.lean`
Expected: `PASS: audio faults`

- [x] **Step 5: Commit**

```bash
git add src/lean/TV/Audio/Fault.lean test/lean/tv/audio/Check.lean test/lean/tv/check-audio.py
git commit -m "Lean audio: faults and result-word decoding"
```

---

### Task 2: Gain scaling

**Files:**
- Create: `src/lean/TV/Audio/Gain.lean`
- Modify: `test/lean/tv/audio/Check.lean`

**Interfaces:**
- Produces:
  ```lean
  def TV.Audio.Gain.ceiling : UInt32 := 300000
  def TV.Audio.Gain.maxOutput : Int64 := 9830
  def TV.Audio.Gain.scale (boost gain : UInt32) (sample : Int16) : Int16
  structure TV.Audio.Gain.Scaled where output : ByteArray; sourcePeak : UInt32; outputPeak : UInt32
  def TV.Audio.Gain.scaleBlock (boost gain : UInt32) (pcm : ByteArray) : Scaled
  def TV.Audio.Gain.readInt16 (bytes : ByteArray) (i : Nat) : Int16   -- little-endian at byte index i
  def TV.Audio.Gain.pushInt16 (bytes : ByteArray) (v : Int16) : ByteArray
  ```

- [x] **Step 1: Add failing checks**

Append to `Check.lean` (and `import TV.Audio.Gain`):
```lean
  -- The Rust protocol test: boost 2, gain 0.0025 (2500 millionths).
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
  IO.println "PASS: audio gain"
```

- [x] **Step 2: Run to verify it fails**

Run: `cd src/lean && lake build TV.Audio.Gain`
Expected: no such module.

- [x] **Step 3: Write the module**

`src/lean/TV/Audio/Gain.lean`:
```lean
prelude
import Init.Data.UInt
import Init.Data.SInt.Basic
import Init.Data.ByteArray.Basic

/-! Global gain for everything the TV plays. A sample is first boosted (the
app asks for 1..4), clamped to full scale, then attenuated by the global gain.
The gain unit is millionths of full scale, the same as TV.Volume; the
calibrated ceiling is 0.30, so no output sample can exceed 9830 = 32767 * 0.3. -/
namespace TV.Audio.Gain

def ceiling : UInt32 := 300000
def unit : Int64 := 1000000
/-- The largest magnitude `scale` can produce: floor(32767 * 300000 / 1000000). -/
def maxOutput : Int64 := 9830

def readInt16 (bytes : ByteArray) (i : Nat) : Int16 :=
  ((bytes.get! i).toUInt16 ||| ((bytes.get! (i + 1)).toUInt16 <<< 8)).toInt16

def pushInt16 (bytes : ByteArray) (v : Int16) : ByteArray :=
  let raw := v.toUInt16
  (bytes.push raw.toUInt8).push (raw >>> 8).toUInt8

/-- `min(max(sample * boost, -32767), 32767) * min(gain, ceiling) / 1000000`,
division truncating toward zero. -/
def scale (boost gain : UInt32) (sample : Int16) : Int16 :=
  let boosted : Int64 := sample.toInt64 * boost.toInt64
  let clamped : Int64 := if boosted < -32767 then -32767 else if boosted > 32767 then 32767 else boosted
  let gain : Int64 := (if gain > ceiling then ceiling else gain).toInt64
  (clamped * gain / unit).toInt16

structure Scaled where
  output : ByteArray
  sourcePeak : UInt32
  outputPeak : UInt32

def magnitude (v : Int16) : UInt32 :=
  if v < 0 then (0 - v.toInt32).toUInt32 else v.toInt32.toUInt32

/-- Every pair of bytes is one little-endian sample; a trailing odd byte is dropped. -/
def scaleBlock (boost gain : UInt32) (pcm : ByteArray) : Scaled := Id.run do
  let mut out := ByteArray.emptyWithCapacity pcm.size
  let mut sourcePeak : UInt32 := 0
  let mut outputPeak : UInt32 := 0
  for i in [0:pcm.size / 2] do
    let sample := readInt16 pcm (2 * i)
    let scaled := scale boost gain sample
    sourcePeak := max sourcePeak (magnitude sample)
    outputPeak := max outputPeak (magnitude scaled)
    out := pushInt16 out scaled
  return { output := out, sourcePeak, outputPeak }

end TV.Audio.Gain
```
Names to confirm against 4.35 if the build complains: `UInt16.toInt16`, `Int16.toUInt16`, `Int16.toInt64`, `Int64.toInt16`, `UInt32.toInt64`, `Int32.toUInt32`, `ByteArray.emptyWithCapacity` (older name `ByteArray.mkEmpty`).

- [x] **Step 4: Build and run**

Run: `cd src/lean && lake build TV.Audio.Gain && lake env lean --run ../../test/lean/tv/audio/Check.lean`
Expected: `PASS: audio faults`, `PASS: audio gain`.

- [x] **Step 5: Commit**

```bash
git add src/lean/TV/Audio/Gain.lean test/lean/tv/audio/Check.lean
git commit -m "Lean audio: integer gain scaling with the 0.3 ceiling"
```

---

### Task 3: Resampler

**Files:**
- Create: `src/lean/TV/Audio/Resample.lean`
- Modify: `test/lean/tv/audio/Check.lean`

**Interfaces:**
- Consumes: `Gain.readInt16`, `Gain.pushInt16`.
- Produces:
  ```lean
  structure TV.Audio.Resample.Converter where rate channels phase : UInt32; previous : Option (Int16 × Int16)
  def TV.Audio.Resample.Converter.new (rate channels : UInt32) : Option Converter   -- none unless rate ∈ {44100, 48000}, channels ∈ {1, 2}
  def TV.Audio.Resample.Converter.convert (c : Converter) (input : ByteArray) : Option (Converter × ByteArray)  -- none on an incomplete frame
  def TV.Audio.Resample.consumedInputFrames (output : UInt64) (rate : UInt32) (written : UInt64) : UInt64
  ```

- [x] **Step 1: Add failing checks** (port of `pcm-converter.rs` tests, plus the Rust `partial_hardware_writes...` expected count)

```lean
  -- helpers
  let samples (xs : List Int16) : ByteArray := xs.foldl Gain.pushInt16 ByteArray.empty
  let some c := Resample.Converter.new 48000 2 | throw (IO.userError "48k stereo")
  let data := samples [-32768, 32767, -1, 1, 100, -100, 0, 0]
  let some (_, out) := c.convert data | throw (IO.userError "convert")
  ensure (out == data) "stereo 48k is bit-exact"
  let some m := Resample.Converter.new 48000 1 | throw (IO.userError "mono")
  let some (_, out) := m.convert (samples [-9, 0, 11]) | throw (IO.userError "convert")
  ensure (out == samples [-9, -9, 0, 0, 11, 11]) "mono is duplicated"
  for channels in [1, 2] do
    let n := 44100 * channels * 7
    let data := (List.range n).foldl (fun b i => Gain.pushInt16 b (Int16.ofInt ((i * 113) % 65536 - 32768))) ByteArray.empty
    let some c := Resample.Converter.new 44100 channels.toUInt32 | throw (IO.userError "44.1k")
    let some (_, whole) := c.convert data | throw (IO.userError "whole")
    ensure (whole.size / 4 == 48000 * 7) "7 s of 44.1k becomes 7 s of 48k"
    let mut split := c
    let mut output := ByteArray.empty
    let mut at := 0
    let mut turn := 0
    while at < data.size do
      let frames := (turn * 17) % 373 + 1
      let stop := min (at + frames * channels * 2) data.size
      let some (next, piece) := split.convert (data.extract at stop) | throw (IO.userError "piece")
      split := next; output := output ++ piece; at := stop; turn := turn + 1
    ensure (output == whole) "chunk boundaries do not change the output"
  let some c := Resample.Converter.new 44100 1 | throw (IO.userError "44.1k mono")
  let mut conv := c
  let mut count : UInt64 := 0
  for n in [1:10001] do
    let some (next, piece) := conv.convert (samples [1]) | throw (IO.userError "one")
    conv := next; count := count + (piece.size / 4).toUInt64
    ensure (count == (n * 48000 / 44100).toUInt64) "cumulative floor count"
    ensure (Resample.consumedInputFrames count 44100 n.toUInt64 == n.toUInt64) "inverse"
    if count > 0 then ensure (Resample.consumedInputFrames (count - 1) 44100 n.toUInt64 < n.toUInt64) "strict inverse"
  ensure (Resample.consumedInputFrames 0 44100 100 == 0) "nothing consumed"
  let some c := Resample.Converter.new 44100 1 | throw (IO.userError "44.1k mono")
  let some (_, out) := c.convert (samples [-32768, 32767, -32768, 32767, 0, 32767]) | throw (IO.userError "bounded")
  for i in [0:out.size / 4] do
    ensure (Gain.readInt16 out (4 * i) == Gain.readInt16 out (4 * i + 2)) "mono output has equal channels"
  let some c := Resample.Converter.new 44100 2 | throw (IO.userError "44.1k stereo")
  let some (_, out) := c.convert (ByteArray.mk (Array.replicate 1764 0)) | throw (IO.userError "silence")
  ensure (out.toList.all (· == 0)) "silence stays silent"
  ensure ((Resample.Converter.new 32000 2).isNone && (Resample.Converter.new 48000 3).isNone) "unsupported formats"
  let some c := Resample.Converter.new 44100 2 | throw (IO.userError "44.1k stereo")
  ensure (c.convert (samples [7]).isNone) "incomplete frame"
  IO.println "PASS: audio resampler"
```

- [x] **Step 2: Run to verify it fails**: `lake build TV.Audio.Resample` → no such module.

- [x] **Step 3: Write the module**

`src/lean/TV/Audio/Resample.lean`:
```lean
prelude
import Init.Data.UInt
import Init.Data.SInt.Basic
import Init.Data.ByteArray.Basic
import TV.Audio.Gain

/-! The hardware takes 48 kHz stereo only. Apps send 44.1 or 48 kHz, mono or
stereo. Conversion is linear interpolation with an exact rational phase: for
every input frame the phase advances by 48000 and one output frame is produced
each time it passes `rate`. The last input frame is kept so that splitting the
input into chunks never changes the output. -/
namespace TV.Audio.Resample

def outputRate : UInt32 := 48000

structure Converter where
  rate : UInt32
  channels : UInt32
  phase : UInt32
  previous : Option (Int16 × Int16)
  deriving Repr

def Converter.new (rate channels : UInt32) : Option Converter :=
  if (rate == 44100 || rate == 48000) && (channels == 1 || channels == 2)
  then some { rate, channels, phase := 0, previous := none } else none

/-- `(previous * phase + current * (48000 - phase)) / 48000` for one channel,
truncating toward zero as the Rust driver did. Both products fit in 32 bits. -/
def mix (previous current : Int16) (phase : UInt32) : Int16 :=
  let p := previous.toInt32 * phase.toInt32
  let c := current.toInt32 * (outputRate - phase).toInt32
  ((p + c) / outputRate.toInt32).toInt16

/-- Emit the output frames one input frame generates. The phase never reaches
twice the rate, so two steps always suffice. -/
def emit (c : Converter) (previous current : Int16 × Int16) (out : ByteArray) : Converter × ByteArray := Id.run do
  let mut phase := c.phase + outputRate
  let mut out := out
  for _ in [0:2] do
    if phase >= c.rate then
      phase := phase - c.rate
      out := Gain.pushInt16 (Gain.pushInt16 out (mix previous.1 current.1 phase)) (mix previous.2 current.2 phase)
  return ({ c with phase }, out)

def Converter.convert (c : Converter) (input : ByteArray) : Option (Converter × ByteArray) :=
  let frameBytes := c.channels.toNat * 2
  if input.size % frameBytes != 0 then none else Id.run do
    let mut state := c
    let mut out := ByteArray.emptyWithCapacity ((input.size / frameBytes + 1) * 8)
    for i in [0:input.size / frameBytes] do
      let left := Gain.readInt16 input (i * frameBytes)
      let right := Gain.readInt16 input (i * frameBytes + frameBytes - 2)
      let current := (left, right)
      let previous := state.previous.getD current
      let (next, more) := emit state previous current out
      state := { next with previous := some current }
      out := more
    return some (state, out)

/-- Number of whole input frames whose output has all been consumed: the
inverse of `floor(input * 48000 / rate)`, saturated at the accepted input. -/
def consumedInputFrames (output : UInt64) (rate : UInt32) (written : UInt64) : UInt64 :=
  let frames := ((output.toNat + 1) * rate.toNat - 1) / outputRate.toNat
  (min frames written.toNat).toUInt64

end TV.Audio.Resample
```

- [x] **Step 4: Build and run** — expected `PASS: audio resampler`. The 7-second cases take a few seconds in the interpreter; if over a minute, run the check compiled (`lake env lean -c` is not needed; `lake exe` is not set up) and instead reduce the count to 2 seconds in both the whole and split cases.

- [x] **Step 5: Commit**: `git commit -m "Lean audio: 48 kHz stereo resampler with exact phase"`

---

### Task 4: Protocol decisions

**Files:**
- Create: `src/lean/TV/Audio/Protocol.lean`
- Modify: `test/lean/tv/audio/Check.lean`

**Interfaces:**
- Produces:
  ```lean
  def TV.Audio.Protocol.ok : Int32 := 0
  def TV.Audio.Protocol.failure : Int32 := -1
  def TV.Audio.Protocol.einval : Int32 := -22
  def TV.Audio.Protocol.maxBlockBytes : UInt32 := 1048576
  def TV.Audio.Protocol.validFormat (rate channels boost : UInt32) : Bool
  def TV.Audio.Protocol.validBlock (channels bytes : UInt32) : Bool
  def TV.Audio.Protocol.readU32 (bytes : ByteArray) (i : Nat) : UInt32           -- little-endian
  def TV.Audio.Protocol.statusBytes (status : Int32) : ByteArray                 -- 4 bytes LE
  def TV.Audio.Protocol.counterBytes (written played : UInt64) : ByteArray       -- 16 bytes LE
  ```

- [x] **Step 1: Failing checks**

```lean
  ensure (Protocol.validFormat 44100 1 2 && Protocol.validFormat 48000 2 1 && Protocol.validFormat 48000 2 4) "supported formats"
  ensure (!(Protocol.validFormat 32000 2 1) && !(Protocol.validFormat 48000 3 1) && !(Protocol.validFormat 48000 1 0) && !(Protocol.validFormat 48000 1 5)) "rejected formats"
  ensure (Protocol.validBlock 2 0 && Protocol.validBlock 2 4 && Protocol.validBlock 1 2 && Protocol.validBlock 2 1048576) "valid blocks"
  ensure (!(Protocol.validBlock 2 3) && !(Protocol.validBlock 2 1048577) && !(Protocol.validBlock 2 2)) "invalid blocks"
  ensure (Protocol.statusBytes (-22) == ByteArray.mk #[0xea, 0xff, 0xff, 0xff]) "EINVAL bytes"
  ensure (Protocol.counterBytes 2 1 == ByteArray.mk #[2,0,0,0,0,0,0,0, 1,0,0,0,0,0,0,0]) "counter bytes"
  ensure (Protocol.readU32 (ByteArray.mk #[0x44, 0xac, 0, 0]) 0 == 44100) "LE u32"
  IO.println "PASS: audio protocol"
```

- [x] **Step 2: Verify it fails**, **Step 3: Write the module**

```lean
prelude
import Init.Data.UInt
import Init.Data.SInt.Basic
import Init.Data.ByteArray.Basic

/-! The socket protocol apps use to play sound: `O rate channels boost`, then
`W bytes data...`, `D` (counters), `R` (reset), `F` (flush). Each decision is a
pure function so that the Rust side only moves bytes. -/
namespace TV.Audio.Protocol

def ok : Int32 := 0
def failure : Int32 := -1
def einval : Int32 := -22
def maxBlockBytes : UInt32 := 1048576

/-- Rates and channel counts the direct hardware path supports. -/
def supports (rate channels : UInt32) : Bool :=
  (rate == 44100 || rate == 48000) && (channels == 1 || channels == 2)

def validFormat (rate channels boost : UInt32) : Bool :=
  8000 <= rate && rate <= 96000 && 1 <= channels && channels <= 2 && 1 <= boost && boost <= 4
    && supports rate channels

/-- A block is at most 1 MiB and holds whole frames. -/
def validBlock (channels bytes : UInt32) : Bool :=
  bytes <= maxBlockBytes && bytes % (channels * 2) == 0

def readU32 (bytes : ByteArray) (i : Nat) : UInt32 :=
  (bytes.get! i).toUInt32 ||| ((bytes.get! (i + 1)).toUInt32 <<< 8)
    ||| ((bytes.get! (i + 2)).toUInt32 <<< 16) ||| ((bytes.get! (i + 3)).toUInt32 <<< 24)

def pushU32 (bytes : ByteArray) (v : UInt32) : ByteArray :=
  (((bytes.push v.toUInt8).push (v >>> 8).toUInt8).push (v >>> 16).toUInt8).push (v >>> 24).toUInt8

def pushU64 (bytes : ByteArray) (v : UInt64) : ByteArray :=
  pushU32 (pushU32 bytes v.toUInt32) (v >>> 32).toUInt32

def statusBytes (status : Int32) : ByteArray := pushU32 ByteArray.empty status.toUInt32
def counterBytes (written played : UInt64) : ByteArray := pushU64 (pushU64 ByteArray.empty written) played

end TV.Audio.Protocol
```

- [x] **Step 4: Build and run** — `PASS: audio protocol`. **Step 5: Commit** `"Lean audio: protocol decisions"`.

---

### Task 5: Session (open, write, played, flush, cleanup, connection loop) with the mock

This is the heart. The mock and the ported cases are written first.

**Files:**
- Create: `src/lean/TV/Audio/Session.lean`
- Modify: `test/lean/tv/audio/Check.lean` (mock + cases)

**Interfaces:**
- Consumes: everything above.
- Produces:
  ```lean
  structure TV.Audio.VolumeDb where integer thousandths : Int32   -- raw hardware pair
  structure TV.Audio.Effects (m : Type → Type) where
    recvCommand : m (Option UInt8)              -- blocks without timeout; none on EOF or error
    recvExact   : UInt32 → m (Option ByteArray) -- n bytes within 3 s; none on short read
    send        : ByteArray → m Bool
    gain        : m UInt32                      -- millionths, clamped by Lean
    now         : m UInt64                      -- monotonic ms
    sleep       : UInt32 → m Unit
    log         : String → m Unit
    acquire     : m UInt8                       -- 0 acquired, 1 owned elsewhere, 2 poisoned
    release     : m Unit                        -- 1 → 0 only
    poison      : m Unit                        -- → 2
    pcmOpen     : m (Except Fault UInt32)
    pcmStart    : UInt32 → m (Except Fault Unit)
    pcmWrite    : UInt32 → ByteArray → UInt32 → UInt32 → m (Except Fault UInt32)  -- handle, buffer, offset, len → bytes accepted
    pcmQueued   : UInt32 → m (Except Fault UInt32)
    pcmStop     : UInt32 → m (Except Fault Unit)
    pcmClose    : UInt32 → m (Except Fault Unit)
    pcmRetain   : UInt32 → m Unit               -- keep the device open forever (route unresolved)
    aoutSpeaker : m (Except Fault UInt32)       -- raw GET_HANDLE value; Lean checks the 0x17 prefix
    aoutCount   : UInt32 → m (Except Fault UInt32)
    aoutConnected : UInt32 → UInt32 → m (Except Fault (Array UInt32))
    aoutConnect : UInt32 → UInt32 → m (Except Fault Unit)
    aoutDisconnect : UInt32 → UInt32 → m (Except Fault Unit)
    aoutVolume  : UInt32 → m (Except Fault VolumeDb)
    aoutSetVolume : UInt32 → VolumeDb → m (Except Fault Unit)
  structure TV.Audio.Session.Track where ...       -- see module
  def TV.Audio.Session.open [Monad m] (e : Effects m) (rate channels : UInt32) : m (Except Fault Track)
  def TV.Audio.Session.write [Monad m] (e : Effects m) (t : Track) (samples : ByteArray) : m (Except Fault Track)
  def TV.Audio.Session.played [Monad m] (e : Effects m) (t : Track) : m (Except Fault UInt64)
  def TV.Audio.Session.cleanup [Monad m] (e : Effects m) (t : Track) : m (Except Fault Unit)
  def TV.Audio.Session.client [Monad m] (e : Effects m) : m (Except Fault Unit)
  ```

- [x] **Step 1: Write the mock and the cases**

Mock state (in `Check.lean`):
```lean
structure VolumeDb' := TV.Audio.VolumeDb
structure Mock where
  input : ByteArray := ByteArray.empty      -- bytes the app sends
  cursor : Nat := 0
  output : ByteArray := ByteArray.empty     -- bytes we send back
  events : Array String := #[]              -- ioctl trace, same names as the Rust mock
  pcmOut : ByteArray := ByteArray.empty     -- what reached the hardware
  inputs : Array UInt32 := #[]              -- AOUT graph
  gain : VolumeDb := ⟨-47, 0⟩
  queue : UInt32 := 0
  clock : UInt64 := 0
  owner : UInt8 := 0
  stops : Nat := 0
  closes : Nat := 0
  globalGain : UInt32 := 0
  writeLimit : UInt32 := 16                 -- the Rust mock accepts at most 16 bytes per write
  failOpen failStart failAttach partialAttach failDetach failClose : Bool := false
  failGainSet failGainRestore failWrite failLookup failQueue : Bool := false
abbrev M := StateM Mock
def effects : Effects M := { ... }   -- each field pushes its event name exactly as the Rust mock does:
  -- pcmOpen: "pcm-open", handle 0x81000000; pcmStart: "pcm-start"; aoutVolume: value gain;
  -- aoutSetVolume: "gain-set" when integer == 0 else "gain-restore"; aoutSpeaker: 0x17000001;
  -- aoutCount: inputs.size; aoutConnected: inputs; aoutConnect: "route-connect" (push unless failAttach && !partialAttach);
  -- aoutDisconnect: "route-disconnect"; pcmWrite: "pcm-write", accept min len writeLimit;
  -- pcmQueued: queue; pcmStop: "pcm-stop"; pcmClose: "pcm-close" (assert owner != 0: "lease released before Close");
  -- recvCommand/recvExact read from input; send appends to output; now returns clock and advances it 1 ms;
  -- sleep advances clock; acquire/release/poison act on owner.
def run (m : Mock) (act : M α) : α × Mock := act.run m
```
Cases (each a `def caseN : IO Unit` or inline), expected exactly as the Rust tests:
1. `route_restore_precedes_pcm_stop_and_close`: open 48000 2, cleanup → events `#["pcm-open","route-connect","gain-set","pcm-start","gain-restore","route-disconnect","pcm-stop","pcm-close"]`, inputs empty, owner 0.
2. `pcm_open_failure_releases_lease_without_closing_unowned_stream`: failOpen → open errors; events `["pcm-open"]`; owner 0.
3. `output_lookup_failure_closes_only_owned_pcm`: failLookup → events `["pcm-open","pcm-close"]`; owner 0.
4. `attach_failure_rolls_back_before_closing_unstarted_pcm`: failAttach+partialAttach → `["pcm-open","route-connect","route-disconnect","pcm-close"]`, inputs empty, owner 0.
5. `start_failure_restores_route_before_close`: failStart → `["pcm-open","route-connect","gain-set","pcm-start","gain-restore","route-disconnect","pcm-close"]`, owner 0.
6. `write_failure_drop_restores_route`: open, failWrite, write 4 zero bytes errors, cleanup → `[...,"pcm-start","pcm-write","gain-restore","route-disconnect","pcm-stop","pcm-close"]`, owner 0.
7. `restore_failure_retains_pcm_lifetime_and_blocks_blind_reopen`: open, failDetach, cleanup errors; stops 0, closes 0, owner 2; a second open errors and closes stays 0.
8. `failed_attach_rollback_poison_is_preserved_after_pcm_cleanup`: failAttach+partialAttach+failDetach → open errors; closes 0, stops 0, owner 2.
9. `close_failure_blocks_reset_retry`: open, failClose, cleanup errors, inputs empty, owner 2.
10. `reset_releases_route_before_reopening_with_zero_counters`: open, write 4 bytes → written 2; cleanup; open → written 0; cleanup; the event after the first "pcm-close" is "pcm-open"; closes 2; inputs empty.
11. `default_backend_sets_scoped_gain_and_restores_it`: after open gain.integer = 0; after cleanup = -47.
12. `failed_gain_restore_poison_does_not_skip_route_cleanup`: open, failGainRestore, cleanup errors; inputs empty; closes 1; owner 2.
13. `gain_set_failure_restores_then_detaches_and_closes_before_start`: failGainSet → `["pcm-open","route-connect","gain-set","gain-restore","route-disconnect","pcm-close"]`, gain.integer -47, owner 0.
14. `partial_hardware_writes_preserve_converted_samples_and_input_counters`: open 44100 1 (writeLimit 16); write 4410 samples `(n*3-6000)` in chunks of 137 samples; written 4410; played 4410; pcmOut equals the converter's whole output of 4800 frames; with queue 400: played = consumedInputFrames 4700 44100 4410; cleanup → stops 1, closes 1, owner 0.
15. `impossible_queue_is_not_a_fake_playback_counter`: queue 4 with nothing written → played errors.
16. `single_owner_and_lease_release`: acquire twice → second is busy; release → acquire ok (exercise through `open` with owner preset to 1 → `.busy`).
17. `volume_noop_does_not_set_hardware`: gain preset ⟨0,0⟩ → no "gain-set"/"gain-restore" events.
18. `nonempty_default_is_refused_without_write`: inputs preset `#[0x81000005]` → open errors; no "route-connect"; `["pcm-open","pcm-close"]`.
19. `borrowed_route_never_disconnects`: inputs preset `#[0x81000000]` (our own handle) → open succeeds with no "route-connect"; cleanup has no "route-disconnect".
20. `cleanup_reconnects_missing_original` and `cleanup_does_not_remove_foreign_new_connection`: exercised by editing `inputs` between open and cleanup (remove an original → one "route-connect" during cleanup; add a foreign handle → it survives).
Protocol cases (through `client`), with `input` set to the exact byte script and `output` compared to the exact bytes:
21. `full_protocol_reset_counters_gain_and_fragmentation`: globalGain 2500; script `O 44100 1 2`, `W [10000,-10000]`, `D R D`, `W [32767]`, `F D` → output = `0,0, (2,2), 0, (0,0), 0, 0, (1,1)` as statuses/counters; pcmOut after both opens holds the scaled samples 50,-50 then 81 (each duplicated to stereo and resampled; compare against `Converter` applied to `[50,-50]` and `[81]`); closes 2.
22. `unsupported_formats_are_einval_before_open`: each of (32000,2,1),(48000,3,1),(48000,1,0),(48000,1,5) → output `-22`, no "pcm-open".
23. `bad_first_command_and_unknown_command_are_einval`: script `D` → `-22`; script `O 48000 2 1` then `?` → `0, -22`, closes 1.
24. `invalid_block_sizes_are_einval_and_close_owner`: `W 3` and `W 1048577` → `0, -22`, closes 1.
25. `open_and_write_failures_return_abi_error`: failOpen → `-1`; failWrite with `W [0,0]` → `0, -1`, closes 1.
26. `truncated_payload_closes_owner_without_write`: `W 4` then one byte → output `0` only, closes 1, no "pcm-write".
27. `zero_global_gain_is_silent`: boost 4, `W [32767,-32768]`, gain 0 → pcmOut all zero.
28. `drain_counter_failure_returns_status_and_closes_owner`: failQueue, `F` → `0, -1`, closes 1.
29. `real_shared_protocol_reset_restores_route_before_next_open`: `O 44100 1 1`, `W 4 zero bytes`, `D R D F` → output `0,0,(2,2),0,(0,0),0`; closes 2; inputs empty; event after first "pcm-close" is "pcm-open"; two "route-disconnect".
Print `PASS: audio session (29 cases)`.

- [x] **Step 2: Run to verify it fails** (`lake build TV.Audio.Session` → no module).

- [x] **Step 3: Write the module**

`src/lean/TV/Audio/Session.lean` — the full text is the implementation; its shape:
```lean
prelude
import Init.Control.Basic
import Init.Data.UInt
import Init.Data.SInt.Basic
import Init.Data.ByteArray.Basic
import Init.Data.Array.Basic
import TV.Audio.Fault
import TV.Audio.Gain
import TV.Audio.Resample
import TV.Audio.Protocol

namespace TV.Audio

structure VolumeDb where
  integer : Int32
  thousandths : Int32
  deriving BEq, Repr, Inhabited

structure Effects (m : Type → Type) where
  ... (exactly the fields listed in Interfaces)

namespace Session

/-- MI request numbers, for messages only; the mechanisms know their own. -/
def pcmOpenRequest : UInt32 := 0xc0803402
...

/-- The speaker output we attach to, as the stock HAL created it. -/
def validSpeaker (handle : UInt32) : Bool := handle >>> 24 == 0x17

/-- Read the speaker's input list twice; accept it only when the count did not
change in between (three attempts), no handle is zero and none repeats. -/
def snapshot (e : Effects m) (speaker : UInt32) : m (Except Fault (Array UInt32))

structure Route where
  before : Array UInt32     -- inputs when we arrived
  attached : Bool           -- we connected our PCM and must disconnect it

structure Track where
  pcm : UInt32
  started : Bool
  speaker : UInt32
  route : Option Route
  volume : Option (VolumeDb × Bool)   -- original, changed
  unresolvedRoute : Option (Array UInt32)
  converter : Resample.Converter
  rate : UInt32
  channels : UInt32
  inputWritten : UInt64
  outputWritten : UInt64

/-- Disconnect our input if present, reconnect any original input that is
missing, then verify. Inputs added by others are never touched. -/
def restoreGraph (e : Effects m) (speaker : UInt32) (before : Array UInt32) (input : UInt32) : m (Except Fault Unit)

/-- Attach (or borrow) the route under the RequireEmpty policy; on a failed
or destructive connect, roll back. -/
def attach (e : Effects m) (speaker pcm : UInt32) : m (Except (Fault × Bool) Route)   -- Bool: rollback failed

/-- Take the speaker to 0 dB with readback; roll back on failure. -/
def takeVolume (e : Effects m) (speaker : UInt32) : m (Except (Fault × Bool) (VolumeDb × Bool))

def target : VolumeDb := ⟨0, 0⟩

/-- Volume restore, then route restore, then PCM stop and close. A route that
cannot be restored keeps the PCM alive and poisons the lease. Any failure
poisons the lease. The lease is released only after the PCM is closed. -/
def cleanup (e : Effects m) (t : Track) : m (Except Fault Unit)

/-- lease → PCM open → speaker → route → volume → start. Every failure after
the lease undoes what was done, in cleanup order. -/
def open (e : Effects m) (rate channels : UInt32) : m (Except Fault Track)

def chunkBytes : UInt32 := 1920
def writeDeadlineMs : UInt64 := 5000

def write (e : Effects m) (t : Track) (samples : ByteArray) : m (Except Fault Track)
def played (e : Effects m) (t : Track) : m (Except Fault UInt64)
def flush (e : Effects m) (t : Track) : m (Except Fault Bool)   -- false on the 5 s timeout
partial def serve (e : Effects m) (t : Track) (nextReport : UInt64) : m (Except Fault Unit)   -- the command loop
def client (e : Effects m) : m (Except Fault Unit)   -- O handshake, serve, cleanup on every exit

end Session
end TV.Audio
```
Behaviour to preserve, from `direct-backend.rs`, `protocol.rs`, `aout-route.rs`:
- `open`: `Converter.new` first (invalid → `.invalid "direct audio supports 44.1/48k mono/stereo"`, nothing touched); `acquire` 2 → `.busy "direct PCM cleanup failed; explicit recovery required before reuse"`, 1 → `.busy "direct PCM is already owned"`; `pcmOpen` failure → `release`, error; zero handle → `.invalid`, release (no close); `aoutSpeaker` failure or bad prefix → cleanup of a track with no route/volume, error; attach failure with rollback failed → `poison`, `unresolvedRoute := before`, cleanup (retains PCM), error; attach failure otherwise → cleanup, error; `takeVolume` failure with rollback failed → `poison`; then cleanup; `pcmStart` failure → cleanup, error.
- `write`: convert (incomplete frame → `.invalid "incomplete PCM frame"`); deadline `now + 5000`; chunks of 1920 bytes; `pcmWrite` result must be `≤ len` and a multiple of 4 else `.invalid "invalid MI write count"`; `outputWritten += n / 4`; `n = 0` → `sleep 2`; past the deadline → `.timeout "direct PCM write deadline"`; a fuel of `samples.size / 1920 + 2600` bounds the loop; on success `inputWritten += frames`.
- `played`: `pcmQueued`; `queued % 4 ≠ 0 ∨ queued / 4 > outputWritten` → `.invalid "inconsistent hardware PCM queue"`; else `consumedInputFrames (outputWritten - queued / 4) rate inputWritten`.
- `client`: first byte must be `O` else send `-22`, `.invalid "expected open"`; EOF before it → `.invalid "closed before open"`; read 12 bytes (`rate channels boost`); invalid → send `-22`; open failure → send `-1`; success → send `0`. Loop: EOF → cleanup, ok. `W`: 4 bytes length; invalid → send `-22`, error; payload short → error (no status); `gain ← e.gain`, `scaleBlock boost gain`; write failure → send `-1`; else send `0`; when `inputWritten ≥ nextReport`, `log s!"LEAN_AUDIO written={..} played={..} source_peak={..} output_peak={..} gain={..}"` and `nextReport := inputWritten + rate * 5` (a `played` failure here is an error). `D`: send `counterBytes written played` (a `played` failure after... send written first, then error, as Rust did; simpler and acceptable: compute `played` first and send both only on success — note this in the module as the one deliberate difference). `R`: cleanup (log a failure, do not stop), open again (failure → send `-1`, error), `nextReport := 1`, send `0`. `F`: until `played ≥ written` or 5 s (`sleep 5` between polls): `played` failure → send `-1`, error; done → send `0`; timeout → send `-1`, continue serving. Unknown → send `-22`, error.
- Every exit of `client` after a successful open runs `cleanup` once and logs its failure.

- [x] **Step 4: Build and run** — `PASS: audio session (29 cases)`. Iterate until the traces match exactly.

- [x] **Step 5: Commit**: `"Lean audio: session open, write, cleanup and the connection loop, with the mock trace tests"`

---

### Task 6: Native boundary (Lean side) and the C marshalling file

**Files:**
- Create: `src/lean/TV/Audio/Native.lean`
- Create: `src/lean/native/audio.c`
- Modify: `src/lean/TV/Pid1.lean` (add `import TV.Audio.Native`)
- Modify: `test/lean/tv/check-audio.py` (final module tuple; compile `audio.c` with the host compiler as a syntax check)

**Interfaces:**
- Produces, for Rust (all on the connection's thread; C ABI):
  ```c
  // scalar, straight to Rust; return a result word (Fault.outcome) boxed via tv_audio_io_u64
  lean_obj_res tv_audio_recv_command(lean_obj_arg w);            // tag 0 + byte<<32, tag 2 on EOF
  lean_obj_res tv_audio_gain(lean_obj_arg w);                    // millionths
  lean_obj_res tv_audio_now(lean_obj_arg w);                     // ms
  lean_obj_res tv_audio_sleep(uint32_t ms, lean_obj_arg w);
  lean_obj_res tv_audio_acquire(lean_obj_arg w); tv_audio_release(w); tv_audio_poison(w);
  lean_obj_res tv_audio_pcm_open(w); tv_audio_pcm_start(uint32_t h, w); tv_audio_pcm_queued(h, w);
  lean_obj_res tv_audio_pcm_stop(h, w); tv_audio_pcm_close(h, w); tv_audio_pcm_retain(h, w);
  lean_obj_res tv_audio_aout_speaker(w); tv_audio_aout_count(uint32_t out, w);
  lean_obj_res tv_audio_aout_connect(out, in, w); tv_audio_aout_disconnect(out, in, w);
  lean_obj_res tv_audio_aout_set_volume(uint32_t out, uint32_t integer, uint32_t thousandths, w);
  // via src/lean/native/audio.c, which calls these plain Rust functions:
  uint64_t tv_audio_recv_bytes(uint8_t *buffer, uint32_t len);          // 0 ok
  uint64_t tv_audio_send_bytes(const uint8_t *buffer, uint32_t len);    // 0 ok
  void     tv_audio_log_bytes(const uint8_t *text, uint32_t len);
  uint64_t tv_audio_pcm_write_bytes(uint32_t handle, const uint8_t *buffer, uint32_t len);   // result word
  uint64_t tv_audio_aout_connected_into(uint32_t output, uint32_t count, uint32_t *handles); // result word
  uint64_t tv_audio_aout_volume_into(uint32_t output, int32_t *pair);                        // result word
  ```
- Lean exports: `tv_audio_client : IO UInt64` (0 clean, 1 failed), `tv_audio_io_u64 (v : UInt64) : IO UInt64`, `tv_audio_io_unit (_ : UInt32) : IO Unit`.

- [x] **Step 1: Write `Native.lean`**

```lean
prelude
import Init.System.IO
import TV.Audio.Session

namespace TV.Audio.Native
open TV.Audio.Session

@[extern "tv_audio_recv_command"] opaque recvCommandWord : IO UInt64
@[extern "tv_audio_recv_exact"] opaque recvExactRaw (len : UInt32) : IO ByteArray
@[extern "tv_audio_send"] opaque sendRaw (bytes : @& ByteArray) : IO UInt64
@[extern "tv_audio_gain"] opaque gainWord : IO UInt64
@[extern "tv_audio_now"] opaque nowWord : IO UInt64
@[extern "tv_audio_sleep"] opaque sleepMs (ms : UInt32) : IO Unit
@[extern "tv_audio_log"] opaque logRaw (message : @& String) : IO Unit
@[extern "tv_audio_acquire"] opaque acquireWord : IO UInt64
@[extern "tv_audio_release"] opaque releaseLease : IO Unit
@[extern "tv_audio_poison"] opaque poisonLease : IO Unit
@[extern "tv_audio_pcm_open"] opaque pcmOpenWord : IO UInt64
@[extern "tv_audio_pcm_start"] opaque pcmStartWord (handle : UInt32) : IO UInt64
@[extern "tv_audio_pcm_write"] opaque pcmWriteWord (handle : UInt32) (bytes : @& ByteArray) (offset len : UInt32) : IO UInt64
@[extern "tv_audio_pcm_queued"] opaque pcmQueuedWord (handle : UInt32) : IO UInt64
@[extern "tv_audio_pcm_stop"] opaque pcmStopWord (handle : UInt32) : IO UInt64
@[extern "tv_audio_pcm_close"] opaque pcmCloseWord (handle : UInt32) : IO UInt64
@[extern "tv_audio_pcm_retain"] opaque pcmRetain (handle : UInt32) : IO Unit
@[extern "tv_audio_aout_speaker"] opaque aoutSpeakerWord : IO UInt64
@[extern "tv_audio_aout_count"] opaque aoutCountWord (output : UInt32) : IO UInt64
@[extern "tv_audio_aout_connected"] opaque aoutConnectedRaw (output count : UInt32) : IO ByteArray  -- 8-byte result word, then LE handles
@[extern "tv_audio_aout_connect"] opaque aoutConnectWord (output input : UInt32) : IO UInt64
@[extern "tv_audio_aout_disconnect"] opaque aoutDisconnectWord (output input : UInt32) : IO UInt64
@[extern "tv_audio_aout_volume"] opaque aoutVolumeRaw (output : UInt32) : IO ByteArray  -- 8-byte result word, then integer, thousandths
@[extern "tv_audio_aout_set_volume"] opaque aoutSetVolumeWord (output integer thousandths : UInt32) : IO UInt64

def unit (request : UInt32) (word : UInt64) : Except Fault Unit := (outcome request word).map fun _ => ()

def effects : Effects IO where
  recvCommand := do
    let word ← recvCommandWord
    return if word &&& 0xff == 0 then some (word >>> 32).toUInt8 else none
  recvExact := fun len => do
    let bytes ← recvExactRaw len
    return if bytes.size == len.toNat then some bytes else none
  send := fun bytes => return (← sendRaw bytes) == 0
  gain := return (← gainWord).toUInt32
  now := nowWord
  sleep := sleepMs
  log := logRaw
  acquire := return (← acquireWord).toUInt8
  release := releaseLease
  poison := poisonLease
  pcmOpen := return outcome pcmOpenRequest (← pcmOpenWord)
  pcmStart := fun h => return unit pcmStartRequest (← pcmStartWord h)
  pcmWrite := fun h b off len => return outcome pcmWriteRequest (← pcmWriteWord h b off len)
  pcmQueued := fun h => return outcome pcmAttrRequest (← pcmQueuedWord h)
  pcmStop := fun h => return unit pcmStopRequest (← pcmStopWord h)
  pcmClose := fun h => return unit pcmCloseRequest (← pcmCloseWord h)
  pcmRetain
  aoutSpeaker := return outcome aoutHandleRequest (← aoutSpeakerWord)
  aoutCount := fun o => return outcome aoutCountRequest (← aoutCountWord o)
  aoutConnected := fun o n => do
    let raw ← aoutConnectedRaw o n
    if raw.size != 8 + 4 * n.toNat then return .error (.invalid "AOUT connection list is short")
    match outcome aoutConnectedRequest (Protocol.readU64 raw 0) with
    | .error f => return .error f
    | .ok _ => return .ok ((Array.range n.toNat).map fun i => Protocol.readU32 raw (8 + 4 * i))
  aoutConnect := fun o i => return unit aoutConnectRequest (← aoutConnectWord o i)
  aoutDisconnect := fun o i => return unit aoutDisconnectRequest (← aoutDisconnectWord o i)
  aoutVolume := fun o => do
    let raw ← aoutVolumeRaw o
    if raw.size != 16 then return .error (.invalid "AOUT volume reply is short")
    match outcome aoutVolumeRequest (Protocol.readU64 raw 0) with
    | .error f => return .error f
    | .ok _ => return .ok ⟨(Protocol.readU32 raw 8).toInt32, (Protocol.readU32 raw 12).toInt32⟩
  aoutSetVolume := fun o v => return unit aoutSetVolumeRequest (← aoutSetVolumeWord o v.integer.toUInt32 v.thousandths.toUInt32)

@[export tv_audio_io_u64] def ioU64 (value : UInt64) : IO UInt64 := pure value
@[export tv_audio_io_unit] def ioUnit (_ : UInt32) : IO Unit := pure ()

/-- One connection, on its own thread. Returns 0 when the app closed the
socket normally, 1 when the session ended with a fault (already logged). -/
@[export tv_audio_client]
def client : IO UInt64 := do
  match ← Session.client effects with
  | .ok () => return 0
  | .error fault => logRaw s!"LEAN_AUDIO client: {fault.describe}"; return 1

end TV.Audio.Native
```
Add `Protocol.readU64` to `Protocol.lean` (two `readU32`s).

- [x] **Step 2: Write `src/lean/native/audio.c`**

```c
// ByteArray and String marshalling for the Lean audio driver's mechanisms.
// The I/O itself is Rust (src/pid1/src/audio_service.rs); this file only
// moves bytes between Lean objects and plain pointers, using lean.h's own
// accessors so that no object layout is duplicated elsewhere.
#include <lean/lean.h>
#include <stdint.h>

uint64_t tv_audio_recv_bytes(uint8_t *buffer, uint32_t len);
uint64_t tv_audio_send_bytes(const uint8_t *buffer, uint32_t len);
void tv_audio_log_bytes(const uint8_t *text, uint32_t len);
uint64_t tv_audio_pcm_write_bytes(uint32_t handle, const uint8_t *buffer, uint32_t len);
uint64_t tv_audio_aout_connected_into(uint32_t output, uint32_t count, uint32_t *handles);
uint64_t tv_audio_aout_volume_into(uint32_t output, int32_t *pair);

static lean_obj_res word_prefix(uint64_t word, size_t extra) {
    lean_obj_res bytes = lean_alloc_sarray(1, 8 + extra, 8 + extra);
    uint8_t *data = lean_sarray_cptr(bytes);
    for (int i = 0; i < 8; i++) data[i] = (uint8_t)(word >> (8 * i));
    return bytes;
}

/* Exactly len bytes, or fewer when the socket closed or timed out. */
lean_obj_res tv_audio_recv_exact(uint32_t len, lean_obj_arg world) {
    (void)world;
    lean_obj_res bytes = lean_alloc_sarray(1, len, len);
    if (len != 0 && tv_audio_recv_bytes(lean_sarray_cptr(bytes), len) != 0) {
        lean_dec(bytes);
        bytes = lean_alloc_sarray(1, 0, 0);
    }
    return lean_io_result_mk_ok(bytes);
}

lean_obj_res tv_audio_send(b_lean_obj_arg bytes, lean_obj_arg world) {
    (void)world;
    uint64_t status = tv_audio_send_bytes(lean_sarray_cptr(bytes), (uint32_t)lean_sarray_size(bytes));
    return lean_io_result_mk_ok(lean_box_uint64(status));
}

lean_obj_res tv_audio_log(b_lean_obj_arg message, lean_obj_arg world) {
    (void)world;
    tv_audio_log_bytes((const uint8_t *)lean_string_cstr(message), (uint32_t)lean_string_size(message) - 1);
    return lean_io_result_mk_ok(lean_box(0));
}

lean_obj_res tv_audio_pcm_write(uint32_t handle, b_lean_obj_arg bytes, uint32_t offset, uint32_t len, lean_obj_arg world) {
    (void)world;
    size_t size = lean_sarray_size(bytes);
    uint64_t word;
    if ((size_t)offset > size || (size_t)len > size - offset) word = 2 | ((uint64_t)22 << 32); /* EINVAL: a Lean bug, not hardware */
    else word = tv_audio_pcm_write_bytes(handle, lean_sarray_cptr(bytes) + offset, len);
    return lean_io_result_mk_ok(lean_box_uint64(word));
}

lean_obj_res tv_audio_aout_connected(uint32_t output, uint32_t count, lean_obj_arg world) {
    (void)world;
    lean_obj_res bytes = word_prefix(0, 4 * (size_t)count);
    uint32_t *handles = (uint32_t *)(lean_sarray_cptr(bytes) + 8);
    uint64_t word = tv_audio_aout_connected_into(output, count, handles);
    uint8_t *data = lean_sarray_cptr(bytes);
    for (int i = 0; i < 8; i++) data[i] = (uint8_t)(word >> (8 * i));
    return lean_io_result_mk_ok(bytes);
}

lean_obj_res tv_audio_aout_volume(uint32_t output, lean_obj_arg world) {
    (void)world;
    int32_t pair[2] = {0, 0};
    uint64_t word = tv_audio_aout_volume_into(output, pair);
    lean_obj_res bytes = word_prefix(word, 8);
    uint8_t *data = lean_sarray_cptr(bytes) + 8;
    for (int i = 0; i < 4; i++) { data[i] = (uint8_t)((uint32_t)pair[0] >> (8 * i)); data[4 + i] = (uint8_t)((uint32_t)pair[1] >> (8 * i)); }
    return lean_io_result_mk_ok(bytes);
}
```
Handles are written in native byte order by Rust and read little-endian by Lean; the TV is little-endian (ARM32 LE), and the check only compiles this file.

- [x] **Step 3: Import in `Pid1.lean`** — after `import TV.Volume.Native` add `import TV.Audio.Native`.

- [x] **Step 4: Extend `check-audio.py`** to also run `cc -fsyntax-only -I$(lean --print-prefix)/include src/lean/native/audio.c`.

- [x] **Step 5: Build and run**: `lake build TV.Audio.Native && lake build TV.Pid1 && python3 test/lean/tv/check-audio.py` → all PASS lines. Also `python3 test/lean/tv/check-volume.py` still passes.

- [x] **Step 6: Commit**: `"Lean audio: native boundary and ByteArray marshalling"`

---

### Task 7: Rust wire layer (reduce `mi-pcm.rs` and `aout-route.rs`)

**Files:**
- Modify: `src/audio/mi-pcm.rs`, `src/audio/aout-route.rs`, `src/audio/lib.rs`
- Modify: `test/unit/audio/mi-pcm-tests.rs`, `test/unit/audio/aout-route-tests.rs` (keep only wire-layout cases)
- Delete: `src/audio/pcm-converter.rs`, `src/audio/direct-backend.rs`, `src/audio/direct-bridge.rs`, `test/unit/audio/direct-backend-tests.rs`, `test/unit/audio/protocol-tests.rs`, `tools/audio/build-direct.sh`, `tools/audio/check-direct.sh`
- Move: `src/audio/{direct_trial,aout-status,mi-silent-probe,volume-probe,pcm-probe-cleanup}.rs` → `research/audio-probes/` plus `research/audio-probes/README.txt` naming commit `7268406` as the driver they include.

**Interfaces:**
- Produces (`g06_audio::mi_pcm`):
  ```rust
  pub struct BufferInfo { capacity_bytes, queued_bytes, free_bytes: u32 }
  pub struct VendorError { pub request: u32, pub status: i32 }
  pub trait Transport { fn transact(&self, request: u32, wire: &mut [u8]) -> io::Result<()>; }
  pub struct DeviceTransport(File);  impl DeviceTransport { pub fn open_pcm() -> io::Result<Self>; pub fn open_aout() -> io::Result<Self>; pub fn from_file(File) -> Self }
  pub const OPEN: u32 = 0xc0803402; START = 0xc0083404; GET_ATTR = 0xc018340b; WRITE = 0xc0383408; STOP = 0xc0043405; CLOSE = 0xc0043403;
  pub fn open(t: &impl Transport) -> io::Result<u32>;                 // raw handle, may be 0
  pub fn start(t: &impl Transport, handle: u32) -> io::Result<()>;
  pub fn write(t: &impl Transport, handle: u32, pcm: &[u8]) -> io::Result<u32>;   // raw written count
  pub fn buffer_info(t: &impl Transport, handle: u32) -> io::Result<BufferInfo>;
  pub fn stop(t: &impl Transport, handle: u32) -> io::Result<()>;
  pub fn close(t: &impl Transport, handle: u32) -> io::Result<()>;
  ```
- Produces (`g06_audio::aout_route`):
  ```rust
  pub struct VolumeDb { pub integer: i32, pub thousandths: i32 }
  pub const GET_HANDLE, GET_COUNT, GET_CONNECTED, CONNECT, DISCONNECT, GET_VOLUME = 0xc010110a, SET_VOLUME = 0x40101109;
  pub fn speaker(t) -> io::Result<u32>;                 // raw GET_HANDLE value
  pub fn count(t, output: u32) -> io::Result<u32>;      // raw
  pub fn connected(t, output: u32, count: u32) -> io::Result<Vec<u32>>;
  pub fn connect(t, output: u32, input: u32) -> io::Result<()>;
  pub fn disconnect(t, output: u32, input: u32) -> io::Result<()>;
  pub fn volume_db(t, output: u32) -> io::Result<VolumeDb>;
  pub fn set_volume_db(t, output: u32, value: VolumeDb) -> io::Result<()>;
  ```
  All checks (zero handle, 0x17 prefix, count bound, duplicates, write count) move to Lean; Rust returns raw values and only maps `VendorError`/errno.

- [x] **Step 1: Trim the tests first.** In `mi-pcm-tests.rs` keep `buffer_query_reads_exact_output_shape` and a new `wire_layouts_are_exact` (the `const` asserts already cover sizes; the test calls `open`/`write` on a mock and checks the bytes at offsets 68, 72, 80, 124 for open and 8, 16, 48 for write). In `aout-route-tests.rs` keep `typed_status_queries_use_exact_getter_layouts_only` reduced to `volume_db`/`set_volume_db`/`connect`/`connected` layouts. Delete every ownership/rollback case (they are now Lean cases in Task 5).
- [x] **Step 2: `cargo test -p g06-audio` in `src/`** → compile errors (functions missing).
- [x] **Step 3: Rewrite the two Rust files** to the interfaces above; `lib.rs` exports only `mi_pcm` and `aout_route`. Delete the listed files; move the probes; write the README line.
- [x] **Step 4: `cd src && cargo test -p g06-audio`** → the kept cases pass. `cargo check -p hello-pid1` will fail until Task 8 (it still references `direct_backend`); that is expected here.
- [x] **Step 5: Commit**: `"Reduce the Rust audio crate to MI wire structs and raw ioctls"`

---

### Task 8: PID1 audio service (Rust externs) and the build

**Files:**
- Create: `src/pid1/src/audio_service.rs`
- Modify: `src/pid1/src/main.rs` (`mod audio_service;` under `repl`), `src/pid1/src/direct_services.rs` (`start_audio` calls `audio_service::start`), `tools/pid1/build.rs`

**Interfaces:**
- Consumes: Task 6's C ABI list, `g06_audio::{mi_pcm, aout_route}`, `tvinit::lean_runtime::{ThreadGuard, consume_io_u64}`.
- Produces: `pub(super) fn start(state: Arc<State>, listener: TcpListener)`.

- [x] **Step 1: Write `audio_service.rs`** (modelled on `volume_control.rs`):
```rust
//! Linux and MI mechanisms for the Lean-owned direct audio driver. Every
//! decision is Lean's (src/lean/TV/Audio); this file moves bytes, runs ioctls,
//! keeps the clock and the single-owner lease, and reports outcomes as words.
use super::State;
use g06_audio::{aout_route, mi_pcm::{self, DeviceTransport, VendorError}};
use std::{cell::RefCell, io::{self, Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, atomic::{AtomicU8, AtomicUsize, Ordering}}, thread, time::{Duration, Instant}};
use tvinit::lean_runtime::{self, Object};

static OWNER: AtomicU8 = AtomicU8::new(0);   // 0 free, 1 owned, 2 poisoned
static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

struct Connection { socket: TcpStream, pcm: Option<DeviceTransport>, aout: Option<DeviceTransport>, state: Arc<State> }
thread_local! { static CONNECTION: RefCell<Option<Connection>> = const { RefCell::new(None) }; }
extern "C" { fn tv_audio_client() -> Object; fn tv_audio_io_u64(v: u64) -> Object; fn tv_audio_io_unit(u: u32) -> Object; }

fn with<R>(f: impl FnOnce(&mut Connection) -> R) -> R { CONNECTION.with(|c| f(c.borrow_mut().as_mut().expect("audio effect outside a connection"))) }
fn word(result: io::Result<u32>) -> u64 { match result {
    Ok(v) => (v as u64) << 32,
    Err(e) => match e.get_ref().and_then(|e| e.downcast_ref::<VendorError>()) {
        Some(v) => 1 | ((v.status as u32 as u64) << 32),
        None => 2 | ((e.raw_os_error().unwrap_or(5) as u32 as u64) << 32) } } }

pub(super) fn start(state: Arc<State>, listener: TcpListener) {
    START.get_or_init(Instant::now);
    thread::spawn(move || {
        let active = Arc::new(AtomicUsize::new(0));
        state.event("LEAN_AUDIO ready 127.0.0.1:8877 direct MI PCM, warm hardware, 48k stereo output");
        for connection in listener.incoming() {
            let Ok(socket) = connection else { continue };
            let _ = socket.set_nodelay(true);
            if active.fetch_add(1, Ordering::Relaxed) >= 4 { active.fetch_sub(1, Ordering::Relaxed); continue; }
            let (state, active) = (state.clone(), active.clone());
            thread::spawn(move || {
                let _thread = lean_runtime::ThreadGuard::attach();
                CONNECTION.with(|c| *c.borrow_mut() = Some(Connection { socket, pcm: None, aout: None, state: state.clone() }));
                let result = unsafe { lean_runtime::consume_io_u64(tv_audio_client()) };
                if let Err(error) = result { state.event(format!("LEAN_AUDIO client: Lean IO failed: {error}")); }
                CONNECTION.with(|c| { c.borrow_mut().take(); });
                active.fetch_sub(1, Ordering::Relaxed);
            });
        }
    });
}
```
Then one `#[no_mangle] extern "C"` per entry in Task 6's list. Socket: `tv_audio_recv_command` sets read timeout `None`, reads one byte (`Ok(0)` or error → tag 2); `tv_audio_recv_bytes` sets timeout 3 s and `read_exact`; `tv_audio_send_bytes` sets write timeout 3 s and `write_all`. `tv_audio_gain` reads `$TV_RUNTIME_DIR/app-volume` (default `/tmp`) as the existing `read_gain` did and returns `(gain * 1_000_000.0).round() as u64`, 0 when unreadable. `tv_audio_now` = ms since `START`. `tv_audio_pcm_open` opens `DeviceTransport::open_pcm()`, runs `mi_pcm::open`, keeps the transport on success. `tv_audio_pcm_close` runs the ioctl and drops the transport whatever the status. `tv_audio_pcm_retain` `std::mem::forget`s the transport. `tv_audio_aout_speaker` opens `open_aout()` (replacing any previous) and runs `aout_route::speaker`. Volume/connected write into the pointers the C file passes.

- [x] **Step 2: `direct_services.rs`**: `start_audio` binds the listener as today, then calls `audio_service::start(state, listener)`; remove the `g06_audio::direct_backend` use. `main.rs`: add `#[cfg(feature = "repl")] mod audio_service;`.
- [x] **Step 3: `tools/pid1/build.rs`**: add the six `Audio/*.lean` names to `rerun-if-changed`, `lake build TV.Audio.Native` next to `TV.Volume.Native`, add the pairs `("AudioFault","Audio/Fault.c")`, `("AudioGain","Audio/Gain.c")`, `("AudioResample","Audio/Resample.c")`, `("AudioProtocol","Audio/Protocol.c")`, `("AudioSession","Audio/Session.c")`, `("AudioNative","Audio/Native.c")` to the compile loop, and compile `src/lean/native/audio.c` with the same zig command into the archive.
- [x] **Step 4: Host type-check**: `cd src/pid1 && cargo check --features repl` fails on the target assertion in build.rs; instead run `cargo check` (no feature) to confirm the crate still builds without `repl`, then `cargo check --target armv7-unknown-linux-musleabihf --features repl` after `rustup target add armv7-unknown-linux-musleabihf --toolchain 1.92.0` with `LEAN_ARM=/nix/store/36lhdx71qvj8zx3yyh3pqxm9nphnb0nd-lean-4.35.0-runtime-armv7-unknown-linux-musleabihf` and `zig` from `/nix/store/hna5xx0f2igssp4xaix69avnf6azfdk7-zig-0.17.0/bin` on PATH, plus dummy `HELLO_BUSYBOX`/`HELLO_AMP_PLAN`/`HELLO_ANDROID_PROPERTIES` files. Expected: build.rs compiles the Lean C and the C file; `cargo check` passes.
- [x] **Step 5: ARM link**: `cargo build --release --features repl --target armv7-unknown-linux-musleabihf` with the same environment (`CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER` as `tools/pid1/linker-armv7` does today). Expected: links; `nm -u` on the binary shows no `tv_audio_*` or `lean_` undefined symbols. If the Rust std for the target cannot be fetched, record this as the step the owner runs via `python3 tools/pid1/build-repl.py`.
- [x] **Step 6: Commit**: `"PID1: run the Lean audio driver; Rust keeps the sockets, clock, lease and ioctls"`

---

### Task 9: Gates and documentation

**Files:**
- Modify: `nix/lean-check.nix` (add `python3 work/test/lean/tv/check-audio.py | tee -a result.txt`), `tools/audio/README.txt` (top paragraph), `src/lean/README.md` (one paragraph: what `TV/Audio` owns, how to check), `src/README.md` if it lists the audio files, `AGENTS.md` (one dated status line under the G06 entries).
- [x] **Step 1**: Make the edits. In `tools/audio/README.txt` replace the first paragraph with: "Current PID1 audio is the Lean driver in src/lean/TV/Audio (gain, resampling, protocol, open/cleanup) over the Rust mechanisms in src/pid1/src/audio_service.rs and the MI wire layer in src/audio/{mi-pcm,aout-route}.rs. Host check: python3 test/lean/tv/check-audio.py (part of nix run .#lean-check). The G13 AAudio bridge (bridge.rs, protocol.rs) is unchanged."
- [x] **Step 2**: Run `nix run .#lean-check` (or the three Python checks directly) → all PASS.
- [x] **Step 3**: Commit: `"Document the Lean audio driver and add its check to the Lean gate"`.

---

### Task 10: Hardware check (owner)

Not automatable here. From the installed G06 session: `python3 tools/pid1/build-repl.py`, boot, `python3 tools/legacy-doom/run-native.py --sound`, play the one-second pistol test at the current level. Expected: audible at the same quiet level as before; PID1 log shows `LEAN_AUDIO written=... output_peak<=9830`. On any doubt about loudness, stop: the ceiling theorem (future) is the only thing that will make this check unnecessary.

## Self-review

- Spec coverage: boundary (Tasks 5–8), modules (1–6), Rust changes (7–8), behaviour change (Task 2 check `81`), testing (1–5, 9), hardware gate (10), G13 unchanged (7). Proof hooks are deliberately not tasks.
- Placeholders: Task 5 step 3 shows the module's shape and the exact behaviours rather than the full text; the full text is the implementation and the 29 traces define it. Task 8 shows the service skeleton and the rule for each extern.
- Type consistency: `Effects` field names and types are used identically in Tasks 5, 6 and the mock; result-word tags match Task 1 and Task 8's `word`.
