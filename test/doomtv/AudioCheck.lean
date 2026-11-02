import AudioCheck.OplTrace
import AudioCheck.Util
import AudioCheck.Scenes
import Doom.Audio.Music
import Doom.Audio.Sfx
import Doom.Audio.Genmidi
import Doom.Audio.Mus
import Doom.Audio.Midi
/-!
# audio-check (host test executable)

Subcommands (driven by `check-audio.sh`):
* `opl-trace OUT FRAMES SEED [bursty]` — random OPL register trace
* `opl-render native|stream RATE FRAMES TRACE OUT` — Lean OPL output for a trace
* `lump WAD NAME OUT` — extract a lump
* `music GENMIDI SONG RATE FRAMES BUFFER DRIVER OPL3 VOLUME LOOP OUT [WRITELOG]`
  — Lean music render with the same arguments as `music_ref`
* `parse-all WAD...` — parse every music, sfx and GENMIDI lump
* `render-music WAD LUMP SECONDS RATE OUT.wav` — full music path to WAV + checks
* `sfx-scene WAD RATE OUT.wav [music]` — scripted Sound+Mixer scene to WAV + checks
* `bench WAD` — real-time factors of the music, mixer and combined paths
-/

open AudioCheck.OplTrace AudioCheck.Util AudioCheck.Scenes Doom.Audio

def usage : String :=
  "usage: audio-check opl-trace|opl-render|lump|music|parse-all ... (see AudioCheck.lean)"

def driverOf : String → OplMusic.DriverVer
  | "0" => .doom1_1_666
  | "1" => .doom2_1_666
  | _ => .doom_1_9

def ioOk {α} (r : Except String α) (what : String) : IO α :=
  match r with
  | .ok a => pure a
  | .error e => throw (IO.userError s!"{what}: {e}")

/-- Render a song like `music_ref`: `frames` frames in buffers of `buffer` frames. -/
def renderSong (gm : Genmidi.Genmidi) (song : ByteArray) (rate frames buffer : Nat)
    (cfg : OplMusic.Config) (volume : Nat) (looping log : Bool) : Music.Player × ByteArray := Id.run do
  let mut p := Music.Player.init gm rate cfg volume log
  p := p.start song looping
  let mut out := ByteArray.emptyWithCapacity (frames * 4)
  let mut done := 0
  while done < frames do
    let n := min buffer (frames - done)
    let (p', b) := p.render n rate
    p := p'
    out := out ++ b
    done := done + n
  return (p, out)

def cmdParseAll (wads : List String) : IO UInt32 := do
  let mut failures := 0
  for path in wads do
    let wad ← Doom.Wad.loadFile path
    let mut nMidi := 0
    let mut nMus := 0
    let mut nSfx := 0
    let mut events := 0
    for name in lumpNames wad "D_" do
      let b ← loadLump wad name
      if Midi.isMidLump b then
        match Midi.parse b with
        | .ok f =>
          nMidi := nMidi + 1
          events := events + f.tracks.foldl (· + ·.size) 0
        | .error e => IO.eprintln s!"{path} {name}: MIDI error {e}"; failures := failures + 1
      else
        match Mus.parse b >>= (Mus.toMidi ·) with
        | .ok (f, _) =>
          nMus := nMus + 1
          events := events + f.tracks.foldl (· + ·.size) 0
          -- the stand-alone 140 Hz sequencer delivers every event exactly once per loop
          let song := (Mus.parse b).toOption.get!
          let loopTicks := song.events.foldl (· + ·.delay) 0
          let (seq, delivered, lastKind) := Id.run do
            let mut seq := Mus.Sequencer.start song true
            let mut n := 0
            let mut lastKind : UInt8 := 0xff
            for _ in [0:loopTicks + 1] do
              let (s', due) := seq.tick
              seq := s'
              n := n + due.size
              if h : 0 < due.size then lastKind := due[due.size - 1].kind
            return (seq, n, lastKind)
          let (_, again) := seq.tick
          let firstGroup := (song.events.toList.takeWhile (·.delay == 0)).length + 1
          if delivered != song.events.size || lastKind != Mus.musScoreEnd || again.size != min firstGroup song.events.size then
            IO.eprintln s!"{path} {name}: MUS sequencer delivered {delivered}/{song.events.size} events in {loopTicks + 1} ticks, restart group {again.size}/{firstGroup}"
            failures := failures + 1
        | .error e => IO.eprintln s!"{path} {name}: MUS error {e}"; failures := failures + 1
    for name in lumpNames wad "DS" do
      match Sfx.parse (← loadLump wad name) with
      | .ok _ => nSfx := nSfx + 1
      | .error e => IO.eprintln s!"{path} {name}: sfx error {e}"; failures := failures + 1
    let gmOk ← match lumpOpt wad "GENMIDI" with
      | some b => match Genmidi.parse b with
        | .ok g => pure s!"GENMIDI ok ({g.instrs.size} instruments, first '{g.names[0]!}')"
        | .error e => do failures := failures + 1; pure s!"GENMIDI error {e}"
      | none => pure "no GENMIDI"
    IO.println s!"{path}: {nMidi} MIDI + {nMus} MUS music lumps ({events} events), {nSfx} sfx, {gmOk}"
  IO.println s!"parse-all: {failures} failures"
  return if failures == 0 then 0 else 1

/-- Checks on a music render: duration, level, clipping, silence, spectrum. -/
def checkMusic (pcm : ByteArray) (rate seconds : Nat) : Array String := Id.run do
  let st := stats pcm rate
  let mut f : Array String := #[]
  if st.frames != rate * seconds then f := f.push s!"wrong duration: {st.frames} frames"
  if st.rmsL < 500 then f := f.push s!"too quiet (rms {st.rmsL})"
  if st.clipped * 10000 > st.frames then f := f.push s!"clipping: {st.clipped} samples"
  if st.silentWindows > 0.2 then f := f.push s!"too much silence ({st.silentWindows})"
  -- plausible spectrum: music has energy well below Nyquist but is not a pure low tone
  if st.hfRatio > 0.8 then f := f.push s!"implausibly noisy spectrum (hf ratio {st.hfRatio})"
  if st.zcr < 40 then f := f.push s!"implausibly dull spectrum (zcr {st.zcr}/s)"
  return f

def lookupOf (wad : Doom.Wad.WadDirectory) : String → Option ByteArray := lumpOpt wad

def report (what : String) (fails : Array String) : IO UInt32 := do
  if fails.isEmpty then
    IO.println s!"{what}: PASS"
    return 0
  else
    for e in fails do IO.println s!"{what}: FAIL {e}"
    return 1

def timeIt (f : Unit → α) : IO (α × Float) := do
  let t0 ← IO.monoNanosNow
  let a := f ()
  let t1 ← IO.monoNanosNow
  return (a, (t1 - t0).toFloat / 1e9)

def rtf (seconds secs : Float) : String := toString ((seconds / secs * 10).round / 10) ++ "x"

def cmdBench (wadPath : String) (phase : String := "all") : IO UInt32 := do
  let wad ← Doom.Wad.loadFile wadPath
  let rate := 48000
  let seconds := 60
  let buffer := 1024
  let gm ← ioOk (Genmidi.parse (← loadLump wad "GENMIDI")) "GENMIDI"
  let song ← loadLump wad (if (lumpOpt wad "D_E1M1").isSome then "D_E1M1" else "D_RUNNIN")
  -- music only
  if phase == "all" || phase == "music" then
   let ((_, pcm), t) ← timeIt fun _ => renderSong gm song rate (rate * seconds) buffer {} 64 true false
   IO.println s!"bench music (OPL2, {seconds} s, {buffer}-frame buffers): {t} s, {rtf seconds.toFloat t} real time ({pcm.size} bytes)"
  -- mixer only: 8 channels kept busy with long sounds
  let sys ← ioOk (Sound.System.create (lookupOf wad) { rate }) "sound"
  let long := #["brssit", "cybdth", "cybsit", "firsht", "rxplod", "bgdth1", "posit1", "barexp"]
  if phase == "all" || phase == "mixer" then
   let (s, t) ← timeIt fun _ => Id.run do
    let mut s := sys
    let mut total := 0
    for k in [0:seconds * 35] do
      if k % 35 == 0 then
        for j in [0:8] do
          s := s.startSound listener (some { id := 1000 + j, x := fu (100 + 40 * j), y := fu (30 * j) })
            ((Sound.sfxId? long[j]!).getD 1)
      let n := ticFrames rate k
      let (m, b) := s.mixer.mix n
      s := { s with mixer := m }
      total := total + b.size
    return (s, total)
   IO.println s!"bench mixer (8 busy channels, {seconds} s): {t} s, {rtf seconds.toFloat t} real time ({s.2} bytes)"
  -- everything: Sound.System with music and 8 channels, rendered per tic
  let sys := sys.changeMusic Sound.musE1M1 true
  let sys := if sys.musPlaying.isNone then sys.changeMusic Sound.musRunnin true else sys
  if phase == "all" || phase == "full" then
   let (s, t) ← timeIt fun _ => Id.run do
    let mut s := sys
    let mut total := 0
    for k in [0:seconds * 35] do
      let mut cmds : Array Sound.Cmd := #[]
      if k % 35 == 0 then
        for j in [0:8] do
          cmds := cmds.push (.start ((Sound.sfxId? long[j]!).getD 1) (some { id := 1000 + j, x := fu (100 + 40 * j), y := fu (30 * j) }))
      s := s.tick listener cmds
      let (s', b) := s.render (ticFrames rate k)
      s := s'
      total := total + b.size
    return (s, total)
   IO.println s!"bench full (music + 8 sfx channels + S_UpdateSounds, {seconds} s): {t} s, {rtf seconds.toFloat t} real time ({s.2} bytes)"
  return 0

def main (args : List String) : IO UInt32 := do
  match args with
  | ["opl-trace", out, frames, seed] =>
    IO.FS.writeFile out (traceToString (genTrace frames.toNat! seed.toNat!.toUInt64 false))
    return 0
  | ["opl-trace", out, frames, seed, "bursty"] =>
    IO.FS.writeFile out (traceToString (genTrace frames.toNat! seed.toNat!.toUInt64 true))
    return 0
  | ["opl-render", mode, rate, frames, trace, out] =>
    let t := parseTrace (← IO.FS.readFile trace)
    let t0 ← IO.monoNanosNow
    let raw := runTrace t (mode == "stream") rate.toNat! frames.toNat!
    let t1 ← IO.monoNanosNow
    IO.FS.writeBinFile out raw
    IO.eprintln s!"opl-render: {frames} frames in {(t1 - t0) / 1000000} ms"
    return 0
  | ["lump", wad, name, out] =>
    IO.FS.writeBinFile out (← loadLump (← Doom.Wad.loadFile wad) name)
    return 0
  | "music" :: gmPath :: songPath :: rate :: frames :: buffer :: drv :: opl3 :: vol :: loop :: out :: rest =>
    let gm ← ioOk (Genmidi.parse (← IO.FS.readBinFile gmPath)) "GENMIDI"
    let song ← IO.FS.readBinFile songPath
    let cfg : OplMusic.Config := { driver := driverOf drv, opl3 := opl3 == "1" }
    let t0 ← IO.monoNanosNow
    let (p, pcm) := renderSong gm song rate.toNat! frames.toNat! buffer.toNat! cfg vol.toNat! (loop == "1")
      (!rest.isEmpty)
    let t1 ← IO.monoNanosNow
    if let some e := p.lastError then IO.eprintln s!"music: song failed to load: {e}"
    IO.FS.writeBinFile out pcm
    if let [logPath] := rest then
      IO.FS.writeFile logPath (String.join (p.st.writeLog.toList.map fun (f, r, v) => s!"{f} {r} {v}\n"))
    let secs := frames.toNat!.toFloat / rate.toNat!.toFloat
    let ms := (t1 - t0).toFloat / 1e6
    IO.eprintln s!"music: {frames} frames in {ms.round} ms, {((secs * 1000 / ms) * 10).round / 10}x real time"
    return 0
  | "parse-all" :: wads => cmdParseAll wads
  | ["render-music", wadPath, name, secs, rate, out] =>
    let wad ← Doom.Wad.loadFile wadPath
    let gm ← ioOk (Genmidi.parse (← loadLump wad "GENMIDI")) "GENMIDI"
    let r := rate.toNat!
    let n := secs.toNat!
    let songBytes ← loadLump wad name
    let ((_, pcm), t) ← timeIt fun _ => renderSong gm songBytes r (r * n) 1024 {} 64 true false
    IO.FS.writeBinFile out (wav r pcm)
    IO.println s!"render-music {name}: {(stats pcm r).summary r}; {rtf n.toFloat t} real time -> {out}"
    report s!"render-music {name}" (checkMusic pcm r n)
  | "sfx-scene" :: wadPath :: rate :: out :: rest =>
    let wad ← Doom.Wad.loadFile wadPath
    let r := rate.toNat!
    let sys ← ioOk (Sound.System.create (lookupOf wad) { rate := r }) "sound"
    let music := if rest == ["music"] then some Sound.musE1M1 else none
    let tics := 7 * 35
    let (s, pcm) := runScript sys tics music
    IO.FS.writeBinFile out (wav r pcm)
    IO.println s!"sfx-scene: {(stats pcm r).summary r} -> {out}"
    let unit := unitChecks sys
    let scene := if music.isNone then checkScene pcm r s else
      (if (stats pcm r).frames == tics * r / 35 then #[] else #["wrong duration"])
    let a ← report "sound semantics" unit
    let b ← report (if music.isNone then "sfx-scene" else "game-scene") scene
    return a + b
  | ["bench", wadPath] => cmdBench wadPath
  | ["bench", wadPath, phase] => cmdBench wadPath phase
  | ["mus-events", lumpPath] =>
    let (f, _) ← ioOk (Mus.parse (← IO.FS.readBinFile lumpPath) >>= (Mus.toMidi ·)) "MUS"
    let mut t := 0
    for e in f.tracks[0]! do
      t := t + e.delta
      if e.kind == Midi.evMeta then IO.println s!"{t} {e.delta} meta {e.metaType}"
      else if e.kind == Midi.evProgramChange || e.kind == Midi.evChanAftertouch then
        IO.println s!"{t} {e.delta} 0x{String.ofList (Nat.toDigits 16 (e.kind.toNat ||| e.channel.toNat))} {e.p1}"
      else
        IO.println s!"{t} {e.delta} 0x{String.ofList (Nat.toDigits 16 (e.kind.toNat ||| e.channel.toNat))} {e.p1} {e.p2}"
    return 0
  | _ =>
    IO.eprintln usage
    return 2
