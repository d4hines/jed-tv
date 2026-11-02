import Yt.Aac.Decoder
import AacTest.Adts

/-! Host-only tool (upstream Lean runtime; uses IO and Float):

    aacdec IN.adts OUT.s16 [REF.s16 [REF.f32]]

decodes every ADTS frame with `Yt.Aac`, writes interleaved s16le, reports
decode speed, and compares per channel with FFmpeg's s16 output (max error,
differing samples, SNR) and with its float output (SNR before rounding). -/

open Yt.Aac AacTest

def s16At (b : ByteArray) (i : Nat) : Float :=
  let v := b[2 * i]!.toNat + 256 * b[2 * i + 1]!.toNat
  if v ≥ 32768 then Float.ofNat v - 65536.0 else Float.ofNat v

def f32At (b : ByteArray) (i : Nat) : Float :=
  let u := b[4 * i]!.toUInt32 ||| (b[4 * i + 1]!.toUInt32 <<< 8) ||| (b[4 * i + 2]!.toUInt32 <<< 16) |||
    (b[4 * i + 3]!.toUInt32 <<< 24)
  (Float32.ofBits u).toFloat

def db (sig noise : Float) : String :=
  if noise == 0.0 then "inf" else toString ((10.0 * Float.log10 (sig / noise) * 100.0).round / 100.0)

def compare (label : String) (mine : ByteArray) (nch : Nat) (n : Nat) (ref : Nat → Float) (scale : Float) : IO Unit := do
  for c in [0:nch] do
    let mut maxe := 0.0
    let mut ndiff := 0
    let mut sig := 0.0
    let mut noise := 0.0
    for k in [0:n] do
      let i := k * nch + c
      let r := max (-32768.0) (min 32767.0 (ref i * scale))
      let e := s16At mine i - r
      let ae := e.abs
      if ae > maxe then maxe := ae
      if ae > 0.5 then ndiff := ndiff + 1
      sig := sig + r * r
      noise := noise + e * e
    IO.println s!"  {label} ch{c}: max|err| {maxe} LSB, samples off by >0.5 LSB: {ndiff}/{n}, SNR {db sig noise} dB"

def main (args : List String) : IO UInt32 := do
  match args with
  | inp :: outp :: rest =>
    let data ← IO.FS.readBinFile inp
    let (asc, aus) := adtsSplit data
    let t0 ← IO.monoNanosNow
    match Decoder.init asc with
    | .error e => IO.eprintln e.message; return 1
    | .ok d0 =>
      let t1 ← IO.monoNanosNow
      let mut d := d0
      let mut pcm := ByteArray.emptyWithCapacity (aus.size * 2048 * d0.config.channels)
      let mut errors := 0
      for au in aus do
        let (d1, r) := d.decode au
        d := d1
        match r with
        | .ok p => pcm := pcm ++ p
        | .error e =>
          errors := errors + 1
          if errors ≤ 5 then IO.eprintln e.message
          pcm := pcm ++ ByteArray.mk (Array.replicate (2048 * d0.config.channels) 0)
      let t2 ← IO.monoNanosNow
      IO.FS.writeBinFile outp pcm
      let nch := d.config.channels
      let n := pcm.size / (2 * nch)
      let secs := Float.ofNat n / Float.ofNat d.config.sampleRate
      let dt := Float.ofNat (t2 - t1) / 1e9
      IO.println s!"{inp}: {aus.size} frames, {nch} ch, {d.config.sampleRate} Hz, {secs} s audio, errors {errors}"
      IO.println s!"  tools: frames={d.stats.frames} short={d.stats.short} kbd={d.stats.kbd} tns={d.stats.tns} pulse={d.stats.pulse} ms={d.stats.ms} is={d.stats.intensity} pns={d.stats.noise}"
      IO.println s!"  tables+init {Float.ofNat (t1 - t0) / 1e6} ms; decode {dt} s = {secs / dt}x realtime"
      match rest with
      | refS16 :: more =>
        let r ← IO.FS.readBinFile refS16
        let m := min n (r.size / (2 * nch))
        if r.size / (2 * nch) != n then IO.println s!"  length differs: ours {n}, reference {r.size / (2 * nch)} samples"
        compare "vs ffmpeg s16" pcm nch m (s16At r) 1.0
        match more with
        | refF32 :: _ =>
          let f ← IO.FS.readBinFile refF32
          let m := min n (f.size / (4 * nch))
          compare "vs ffmpeg float" pcm nch m (f32At f) 32768.0
          -- FFmpeg's own rounding, for scale.
          let mut sig := 0.0
          let mut noise := 0.0
          for i in [0:m * nch] do
            let x := max (-32768.0) (min 32767.0 (f32At f i * 32768.0))
            let e := s16At r i - x
            sig := sig + x * x
            noise := noise + e * e
          IO.println s!"  (ffmpeg s16 vs its own float: SNR {db sig noise} dB)"
        | [] => pure ()
      | [] => pure ()
      return 0
  | _ => IO.eprintln "usage: aacdec IN.adts OUT.s16 [REF.s16 [REF.f32]]"; return 2
