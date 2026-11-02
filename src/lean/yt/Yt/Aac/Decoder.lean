import Yt.Aac.Spectrum
import Yt.Aac.Imdct

/-! AAC-LC decoder: raw_data_block (ISO/IEC 14496-3 4.4.2.1) to interleaved
signed 16-bit PCM.

    let d ← Decoder.init asc          -- AudioSpecificConfig bytes (MP4 esds)
    let (d, pcm) := d.decode accessUnit
    -- pcm : Except DecodeError ByteArray, 1024 · d.channels · 2 bytes (s16le)
    let d := d.reset                  -- after a seek: drop the overlap

Every access unit yields 1024 samples per channel, including the encoder's
priming frames: trimming them (MP4 edit list) is the player's job. On error
the frame is not output and the state is unchanged.

Supported: mono and stereo AAC-LC (SCE, CPE; LFE parsed and dropped; DSE,
FIL and PCE skipped), all spectral codebooks, PNS (FFmpeg's noise generator,
so noise bands match it), M/S and intensity stereo, pulse data, TNS, sine and
KBD windows with window switching. HE-AAC is detected (in the
AudioSpecificConfig, or SBR fill elements in a frame) and reported as
`unsupported`, as are coupling channels, Main/SSR/LTP tools, 960-sample
frames and more than two channels.

The arithmetic is integer-only (our runtime has no `Float` operations):
block floating point over `Int32` words (`Spectrum`), a fixed-point
FFT-based IMDCT (`Imdct`) and Q12 time-domain overlap. On the YouTube
fixture the output is within 1 LSB of FFmpeg's float decoder rounded to
16 bits. -/

namespace Yt.Aac

structure Tables where
  books : Books
  consts : Consts
  long : Mdct
  short : Mdct
  win : Windows
  tns3 : ByteArray
  tns4 : ByteArray

/-- All constant tables, computed (from integers) on first use. -/
def sharedTables (_ : Unit) : Tables :=
  let (m, e) := Math.pow43 ()
  { books := books (), consts := { pow43M := m, pow43E := e, frac := Math.fracPow2 () },
    long := Mdct.make 2048, short := Mdct.make 256,
    win := { sineLong := Math.sineWindow 1024, sineShort := Math.sineWindow 128,
             kbdLong := Math.kbdWindow 1024 4, kbdShort := Math.kbdWindow 128 6 },
    tns3 := Math.tnsCoefs 3, tns4 := Math.tnsCoefs 4 }

/-- Counts of coding tools seen (for diagnostics and tests). -/
structure Stats where
  frames : Nat := 0
  /-- Channel-frames with eight short windows. -/
  short : Nat := 0
  /-- Channel-frames with KBD window shape. -/
  kbd : Nat := 0
  /-- Channel-frames with TNS / pulse data. -/
  tns : Nat := 0
  pulse : Nat := 0
  /-- Channel pairs using M/S. -/
  ms : Nat := 0
  /-- Bands coded with intensity stereo / noise substitution. -/
  intensity : Nat := 0
  noise : Nat := 0
  deriving Inhabited

private def countBands (ics : Ics) (lo hi : UInt8) : Nat := Id.run do
  let mut n := 0
  for g in [0:ics.info.groupLen.size] do
    for sfb in [0:ics.info.maxSfb] do
      let t := ld8 ics.bandType (g * 64 + sfb).toUSize
      if t ≥ lo && t ≤ hi then n := n + 1
  return n

def Stats.add (s : Stats) (ics : Ics) : Stats :=
  { s with short := s.short + (if ics.info.short then 1 else 0)
           kbd := s.kbd + (if ics.info.kbd then 1 else 0)
           tns := s.tns + (if ics.tns.isEmpty then 0 else 1)
           pulse := s.pulse + (if ics.pulses then 1 else 0)
           intensity := s.intensity + countBands ics 14 15
           noise := s.noise + countBands ics 13 13 }

structure Decoder where
  config : Config
  stats : Stats := {}
  tables : Tables
  /-- Overlap state per output channel. -/
  overlap : Array Overlap
  /-- PNS generator state (FFmpeg's seed). -/
  rand : UInt32

def Decoder.init (asc : ByteArray) : Except DecodeError Decoder := do
  let config ← parseAsc asc
  return { config, tables := sharedTables (), overlap := Array.replicate config.channels (Overlap.init ()),
           rand := 0x1f2e3d4c }

namespace Decoder
@[inline] def sampleRate (d : Decoder) : Nat := d.config.sampleRate
@[inline] def channels (d : Decoder) : Nat := d.config.channels
/-- Forget the overlap from previous frames (after a seek or a gap). -/
def reset (d : Decoder) : Decoder :=
  { d with overlap := Array.replicate d.config.channels (Overlap.init ()) }
end Decoder

/-! ### Synthesis -/

/-- IMDCT of words `[xo, xo + n)` (exponent `e`) into Q12 words of `buf` at
`xo`, normalising the block to [2^28, 2^29) on the way in. -/
def imdctBlock (m : Mdct) (x : ByteArray) (xo n : USize) (e : Int32) (z buf : ByteArray) :
    ByteArray × ByteArray × ByteArray :=
  let mx := maxAbs x xo (xo + n) 0
  if mx == 0 then (x, z, buf)
  else
    let s : Int32 := 28 - mx.log2.toInt32
    -- time value = output · 2^(e − s − 1); in Q12: shift by e − s + 11
    if s ≥ 0 then
      let (z, buf) := m.half x xo s.toUInt32 z buf xo (e - s + 11)
      (x, z, buf)
    else
      let x := shiftWords x xo (xo + n) s
      let (z, buf) := m.half x xo 0 z buf xo (e - s + 11)
      (x, z, buf)

/-- Frequency to time for one channel: returns the new overlap state and
1024 output words in Q12. -/
def synthesize (t : Tables) (ov : Overlap) (x : ByteArray) (e : Int32) (info : IcsInfo) :
    Overlap × ByteArray := Id.run do
  let mut buf := zeros32 1024
  let mut x := x
  if info.short then
    let mut z := zeros32 128
    for w in [0:8] do
      let (x1, z1, b1) := imdctBlock t.short x (w * 128).toUSize 128 e z buf
      x := x1
      z := z1
      buf := b1
  else
    let (_, _, b1) := imdctBlock t.long x 0 1024 e (zeros32 1024) buf
    buf := b1
  return overlapAdd t.win ov buf info.seq info.kbd

/-- Write Q12 words as s16le samples of channel `ch` of `nch`. -/
partial def writePcm (out pcm : ByteArray) (i : USize) (ch nch : USize) : ByteArray :=
  if i < 1024 then
    let v := ldI out i
    let s : Int32 := if v ≥ 134215680 then 32767 else if v < -134219776 then -32768
      else (v + 2048) >>> (12 : Int32)
    let u := s.toUInt32
    let idx := (i * nch + ch) * 2
    writePcm out (st8 (st8 pcm idx u.toUInt8) (idx + 1) (u >>> 8).toUInt8) (i + 1) ch nch
  else pcm

/-! ### Channel elements -/

structure Frame where
  r : BitReader
  pcm : ByteArray
  ch : Nat
  overlap : Array Overlap
  rand : UInt32
  stats : Stats

private def err (s : String) : Except DecodeError α := throw (.invalid s)

/-- TNS on a channel with exponent `e`: normalised to [2^28, 2^29) first for
precision (the filters can amplify quantization error strongly), with two
bits of headroom, then four more per retry if an output saturated. -/
def tnsChannel (ics : Ics) (rateIndex : Nat) (x : ByteArray) (e : Int32) : ByteArray × Int32 := Id.run do
  let mx := maxAbs x 0 1024 0
  if mx == 0 then return (x, e)
  let mut s : Int32 := 28 - mx.log2.toInt32
  for _ in [0:8] do
    let (y, sat) := applyTns ics rateIndex (shiftWords x 0 1024 s)
    if !sat then return (y, e - s)
    s := s - 4
  return (x, e)

/-- TNS, synthesis and output of one channel. -/
private def finishChannel (d : Decoder) (f : Frame) (ics : Ics) (x : ByteArray) (e : Int32) :
    Frame := Id.run do
  let t := d.tables
  let (x, e) := if ics.tns.isEmpty then (x, e) else tnsChannel ics d.config.rateIndex x e
  if f.ch < f.overlap.size then
    let ov := f.overlap[f.ch]!
    let overlap := f.overlap.set! f.ch (Overlap.init ())
    let (ov, out) := synthesize t ov x e ics.info
    let pcm := writePcm out f.pcm 0 f.ch.toUSize d.config.channels.toUSize
    return { f with pcm, overlap := overlap.set! f.ch ov, ch := f.ch + 1 }
  else return f

private def decodeSce (d : Decoder) (f : Frame) (lfe : Bool) : Except DecodeError Frame := do
  let t := d.tables
  let r := f.r.skip 4
  let (ics, r) ← parseIcs r d.config.rateIndex none t.books t.tns3 t.tns4
  if lfe && ics.info.short then err "LFE with short windows"
  let e := expForBound (channelBound t.consts ics)
  let (x, rand) := dequantize t.consts ics e f.rand
  let f := { f with r, rand, stats := f.stats.add ics }
  if lfe then return f
  if f.ch + 1 > d.config.channels then err "more channels than configured"
  return finishChannel d f ics x e

private def parseMsMask (r : BitReader) (info : IcsInfo) : Except DecodeError (Nat × ByteArray × BitReader) := do
  let (msp, r) := r.readNat 2
  if msp == 3 then err "reserved ms_mask_present"
  let mut ms := zeros (8 * 64)
  let mut r := r
  if msp != 0 then
    for g in [0:info.groupLen.size] do
      for sfb in [0:info.maxSfb] do
        let (b, r1) := if msp == 1 then r.read 1 else (1, r)
        r := r1
        ms := st8 ms (g * 64 + sfb).toUSize b.toUInt8
  return (msp, ms, r)

private def decodeCpe (d : Decoder) (f : Frame) : Except DecodeError Frame := do
  let t := d.tables
  let ri := d.config.rateIndex
  let r := f.r.skip 4
  let (common, r) := r.readBool
  let (info, msp, ms, r) ← if common then do
      let (info, r) ← parseIcsInfo r ri
      let (msp, ms, r) ← parseMsMask r info
      pure (some info, msp, ms, r)
    else pure (none, 0, zeros (8 * 64), r)
  let (ics0, r) ← parseIcs r ri info t.books t.tns3 t.tns4
  let (ics1, r) ← parseIcs r ri info t.books t.tns3 t.tns4
  if f.ch + 2 > d.config.channels then err "more channels than configured"
  -- One exponent for both channels, covering M/S and intensity bands.
  let c := t.consts
  let b0 := channelBound c ics0
  let b1 := channelBound c ics1
  let bi := intensityBound c ics0 ics1
  let e := expForBound (if b0 > b1 then (if b0 > bi then b0 else bi) else (if b1 > bi then b1 else bi))
  let (x0, rand) := dequantize c ics0 e f.rand
  let (x1, rand) := dequantize c ics1 e rand
  let (x0, x1) := if common && msp != 0 then midSide ics0.info ms ics0.bandType ics1.bandType x0 x1
    else (x0, x1)
  let x1 := intensity c ics1 ms (common && msp != 0) x0 x1
  let stats := (f.stats.add ics0).add ics1
  let stats := if common && msp != 0 then { stats with ms := stats.ms + 1 } else stats
  let f := { f with r, rand, stats }
  let f := finishChannel d f ics0 x0 e
  return finishChannel d f ics1 x1 e

/-- program_config_element: parsed only to be skipped. -/
private def skipPce (r : BitReader) : BitReader := Id.run do
  let r := r.skip (4 + 2 + 4)
  let (nf, r) := r.readNat 4
  let (ns, r) := r.readNat 4
  let (nb, r) := r.readNat 4
  let (nl, r) := r.readNat 2
  let (na, r) := r.readNat 3
  let (nc, r) := r.readNat 4
  let (mono, r) := r.readBool
  let r := if mono then r.skip 4 else r
  let (stereo, r) := r.readBool
  let r := if stereo then r.skip 4 else r
  let (matrix, r) := r.readBool
  let r := if matrix then r.skip 3 else r
  let r := r.skip (5 * (nf + ns + nb) + 4 * (nl + na) + 5 * nc)
  let r := r.byteAlign
  let (comment, r) := r.readNat 8
  return r.skip (8 * comment)

/-- One access unit (raw_data_block). -/
private def decodeFrame (d : Decoder) (au : ByteArray) : Except DecodeError Frame := do
  let nch := d.config.channels
  let mut f : Frame :=
    { r := { data := au, pos := 0 }, pcm := zeros (2048 * nch), ch := 0, overlap := d.overlap,
      rand := d.rand, stats := { d.stats with frames := d.stats.frames + 1 } }
  let mut ended := false
  for _ in [0:256] do
    let (id, r) := f.r.read 3
    f := { f with r }
    if id == 7 then
      ended := true
      break
    else if id == 0 then f ← decodeSce d f false
    else if id == 1 then f ← decodeCpe d f
    else if id == 2 then throw (.unsupported "coupling channel element")
    else if id == 3 then f ← decodeSce d f true
    else if id == 4 then
      let r := f.r.skip 4
      let (align, r) := r.readBool
      let (n, r) := r.readNat 8
      let (n, r) := if n == 255 then (let (m, r) := r.readNat 8; (n + m, r)) else (n, r)
      let r := if align then r.byteAlign else r
      f := { f with r := r.skip (8 * n) }
    else if id == 5 then f := { f with r := skipPce f.r }
    else
      let (n, r) := f.r.readNat 4
      let (n, r) := if n == 15 then (let (m, r) := r.readNat 8; (n + m - 1, r)) else (n, r)
      if n > 0 then
        let ext := peekBits r.data r.pos 4
        if ext == 0xD || ext == 0xE then throw (.unsupported "HE-AAC (SBR data in a fill element)")
      f := { f with r := r.skip (8 * n) }
    if f.r.overrun then err "access unit truncated"
  if !ended then err "missing END element"
  if f.ch != nch then err "missing channel element"
  return f

/-- Decode one access unit to interleaved s16le PCM (1024 samples per
channel). On error the decoder state is left unchanged. -/
def Decoder.decode (d : Decoder) (au : ByteArray) : Decoder × Except DecodeError ByteArray :=
  match decodeFrame d au with
  | .ok f => ({ d with overlap := f.overlap, rand := f.rand, stats := f.stats }, .ok f.pcm)
  | .error e => (d, .error e)

end Yt.Aac
