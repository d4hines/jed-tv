/-!
# Inputs for the Lean app's test packages (git-ignored, under assets/build)

    lean --run tools/lean/yt/Fixtures.lean KIND [--out DIR]

- `http`: test/http's: the latest live capture's search and thumbnail
  requests and replies, as base64 (the transcript runs in QEMU without a
  filesystem, so inputs are compiled in) -> assets/build/lean-formats/http
- `json`: test/json's: the Stage 0 /player replies and the live search
  reply, and their canonical forms by the independent oracle
  test/json/ref.py (Python's json) -> assets/build/lean-formats/json
- `jpeg`: test/jpeg's: test JPEGs and libjpeg-turbo's decodes
  (test/jpeg/ref.py, in a Nix shell), then test/jpeg/JpegTest/Fixtures.lean
  (gen-lean.lean) -> assets/build/lean-formats/jpeg
- `mp4`: test/mp4's: the two Stage 0 files as base64 and the dumps of the
  Rust demuxer the Lean one was ported from (src/mp4's dump example, built
  from the parent of the commit that removed it) -> assets/build/lean-formats/mp4
- `aac`: test/aac's: the YouTube fixture (itag 140) as ADTS and FFmpeg's
  decodes of it, synthetic streams from FFmpeg's AAC encoder (PNS,
  intensity, M/S, TNS, short windows, other sample rates; and Apple's
  encoder, HE-AAC included, where FFmpeg has it) and
  fixture/AacFixture.lean, the first frames of each as hex -> assets/build/lean-aac.
  References use `-cpuflags 0` (FFmpeg's C code): its default ARM64 NEON
  float -> s16 path floors instead of rounding; that output is kept as
  ref/NAME.neon.s16.

`--out DIR` writes into DIR instead (to compare with the current inputs;
jpeg's Lean file then goes there too). Inputs are taken in byte order of
their names, whatever the locale.
-/

open System

def cleanEnv : Array (String × Option String) :=
  #["DYLD_LIBRARY_PATH", "LD_LIBRARY_PATH", "LEAN_PATH", "LEAN_SRC_PATH"].map (·, none)

/-- Run a tool; its standard output, or an error with all of its output. -/
def run (cmd : String) (args : Array String) (cwd : Option FilePath := none) : IO String := do
  let o ← IO.Process.output { cmd, args, cwd, env := cleanEnv }
  unless o.exitCode == 0 do
    throw <| IO.userError s!"{cmd} {" ".intercalate args.toList} failed:\n{o.stdout}{o.stderr}"
  return o.stdout

def base64 (b : ByteArray) : String := Id.run do
  let abc := "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".toList.toArray
  let mut out : Array Char := Array.emptyWithCapacity ((b.size + 2) / 3 * 4)
  let mut i := 0
  while i < b.size do
    let n := min 3 (b.size - i)
    let w := (b.get! i).toNat <<< 16 ||| (if n > 1 then (b.get! (i + 1)).toNat <<< 8 else 0) |||
      (if n > 2 then (b.get! (i + 2)).toNat else 0)
    out := out.push abc[w >>> 18]! |>.push abc[(w >>> 12) &&& 63]!
    out := out.push (if n > 1 then abc[(w >>> 6) &&& 63]! else '=') |>.push (if n > 2 then abc[w &&& 63]! else '=')
    i := i + 3
  return String.ofList out.toList

def hexOf (b : ByteArray) : String :=
  let d := "0123456789abcdef".toList.toArray
  String.ofList (b.toList.flatMap fun x => [d[x.toNat / 16]!, d[x.toNat % 16]!])

/-- Entries of `dir` whose names satisfy `p`, sorted by name (byte order). -/
def entries (dir : FilePath) (p : String → Bool) : IO (Array FilePath) := do
  let es ← dir.readDir
  return ((es.filter (p ·.fileName)).map (·.path)).qsort (·.toString < ·.toString)

def rel (root p : FilePath) : String := (p.toString.drop (root.toString.length + 1)).toString

/-- The newest live capture (assets/build/lean-formats/live-*). -/
def latestLive (root : FilePath) : IO FilePath := do
  let some d := (← entries (root / "assets/build/lean-formats") (·.startsWith "live-")).back?
    | throw <| IO.userError "no assets/build/lean-formats/live-* capture"
  return d

def http (root out : FilePath) : IO Unit := do
  let live ← latestLive root
  for f in ["search.request", "search.response", "search.json", "thumb.request", "thumb.response", "mqdefault.jpg"] do
    IO.FS.writeFile (out / (f ++ ".b64")) (base64 (← IO.FS.readBinFile (live / f)))
  IO.FS.writeFile (out / "SOURCE.txt") s!"from {rel root live}\n"

def json (root out : FilePath) : IO Unit := do
  let mut players : Array FilePath := #[]
  for d in ← entries (root / "assets/yt-stage0") (·.startsWith "minimal-") do
    players := players ++ (← entries d fun n => n.startsWith "player." && n.endsWith ".json")
  let mut sources := ""
  let mut copies := #[]
  for i in [0:players.size] do
    let dst := out / s!"player{i + 1}.json"
    IO.FS.writeBinFile dst (← IO.FS.readBinFile players[i]!)
    copies := copies.push dst
    sources := sources ++ s!"player{i + 1}.json <- {rel root players[i]!}\n"
  let live ← latestLive root
  IO.FS.writeBinFile (out / "search.json") (← IO.FS.readBinFile (live / "search.json"))
  sources := sources ++ s!"search.json <- {rel root live}/search.json\n"
  IO.FS.writeFile (out / "SOURCES.txt") sources
  IO.print (← run "python3" (#["-I", (root / "test/lean/yt/json/ref.py").toString, out.toString] ++
    (copies.push (out / "search.json")).map (·.toString)))
  IO.print sources

def jpeg (root out : FilePath) (lean : FilePath) : IO Unit := do
  let here := root / "test/lean/yt/jpeg"
  let tiles ← entries (root / "assets/build/tiles/src") (·.endsWith ".jpg")
  let live ← latestLive root
  discard <| run "nix" (#["shell", "nixpkgs#libjpeg_turbo.bin", "nixpkgs#python3", "-c", "python3", "-I",
    (here / "ref.py").toString, out.toString] ++ tiles.map (·.toString) ++ #[(live / "mqdefault.jpg").toString])
  IO.print (← IO.FS.readFile (out / "manifest.txt"))
  discard <| run "lake" #["env", "lean", "--run", "gen-lean.lean", out.toString, lean.toString] here

def mp4 (root out : FilePath) : IO Unit := do
  let fx := root / "assets/yt-stage0/record-PVGeM40dABA-visionos-20261031T200633Z/fixtures"
  let gone := (← run "git" #["-C", root.toString, "log", "-1", "--format=%H", "--diff-filter=D", "--",
    "src/mp4/examples/dump.rs"]).trimAscii.toString
  let tmp ← IO.FS.createTempDir
  try
    discard <| run "sh" #["-c", s!"git -C '{root}' archive {gone}~1 src/mp4 | tar -x -C '{tmp}'"]
    let o ← IO.Process.output { cmd := "cargo", args := #["build", "--release", "-q", "--manifest-path",
      (tmp / "src/mp4/Cargo.toml").toString, "--example", "dump"], env := cleanEnv.push ("RUSTUP_TOOLCHAIN", some "1.92.0") }
    unless o.exitCode == 0 do throw <| IO.userError s!"building the Rust dump failed:\n{o.stderr}"
    for (f, stem) in [("136.mp4", "136"), ("140.m4a", "140")] do
      IO.FS.writeFile (out / (f ++ ".b64")) (base64 (← IO.FS.readBinFile (fx / f)))
      IO.FS.writeFile (out / (stem ++ ".dump")) (← run (tmp / "src/mp4/target/release/examples/dump").toString #[(fx / f).toString])
  finally
    IO.FS.removeDirAll tmp

/-- `n` ADTS frames of `b` from frame `start`. -/
def adtsFrames (b : ByteArray) (start n : Nat) : Except String ByteArray := do
  let mut (i, k, out) := (0, 0, ByteArray.empty)
  while i + 7 ≤ b.size && k < start + n do
    unless b.get! i == 0xFF && b.get! (i + 1) &&& 0xF0 == 0xF0 do throw s!"no ADTS sync at byte {i}"
    let len := ((b.get! (i + 3)).toNat &&& 3) <<< 11 ||| (b.get! (i + 4)).toNat <<< 3 ||| (b.get! (i + 5)).toNat >>> 5
    if k ≥ start then out := out ++ b.extract i (i + len)
    i := i + len
    k := k + 1
  return out

def aac (root out : FilePath) : IO Unit := do
  let ref := out / "ref"
  IO.FS.createDirAll ref
  IO.FS.createDirAll (out / "fixture")
  let ff (args : Array String) : IO Unit := discard <| run "ffmpeg" (#["-hide_banner", "-loglevel", "error", "-y"] ++ args)
  let adts (name : String) := (ref / (name ++ ".adts")).toString
  let refs (name : String) : IO Unit := do
    ff #["-cpuflags", "0", "-i", adts name, "-f", "s16le", "-c:a", "pcm_s16le", (ref / (name ++ ".ref.s16")).toString]
    ff #["-cpuflags", "0", "-i", adts name, "-f", "f32le", "-c:a", "pcm_f32le", (ref / (name ++ ".ref.f32")).toString]
    ff #["-i", adts name, "-f", "s16le", "-c:a", "pcm_s16le", (ref / (name ++ ".neon.s16")).toString]
  ff #["-i", (root / "assets/yt-stage0/record-PVGeM40dABA-visionos-20261031T200633Z/fixtures/140.m4a").toString,
       "-c:a", "copy", "-f", "adts", adts "140"]
  refs "140"
  -- Synthetic: decaying tones (transients force short windows), panned
  -- noise, a stereo image that suits intensity and M/S coding.
  let l := "0.45*sin(2*PI*440*t)*exp(-12*mod(t,0.37))+0.3*(random(0)-0.5)*exp(-3*mod(t,1.1))+0.2*sin(2*PI*3000*t+3*sin(2*PI*5*t))"
  let r := "0.40*sin(2*PI*440*t)*exp(-12*mod(t,0.37))+0.25*(random(1)-0.5)+0.2*sin(2*PI*3000*t+3*sin(2*PI*5*t))"
  for (name, rate, ch, kbps, secs) in [("s44", "44100", 2, "64k", "6"), ("s48", "48000", 2, "96k", "4"),
      ("s32", "32000", 2, "48k", "4"), ("s22m", "22050", 1, "32k", "4"), ("s16", "16000", 2, "24k", "4"),
      ("s8m", "8000", 1, "16k", "4"), ("s96", "96000", 2, "128k", "2")] do
    let exprs := if ch == 1 then l else l ++ "|" ++ r
    ff #["-f", "lavfi", "-i", s!"aevalsrc=exprs='{exprs}':s={rate}:d={secs}", "-c:a", "aac", "-aac_coder", "twoloop",
         "-aac_pns", "1", "-aac_is", "1", "-aac_ms", "1", "-aac_tns", "1", "-b:a", kbps, "-f", "adts", adts name]
    refs name
  -- Apple's encoder (macOS FFmpeg builds only) on 30 s of the YouTube audio:
  -- a second encoder's choices (many short windows, intensity), and HE-AAC
  -- in ADTS (implicit SBR).
  if ((← run "ffmpeg" #["-hide_banner", "-encoders"]).splitOn "aac_at").length > 1 then
    let pcm := #["-f", "s16le", "-ar", "44100", "-ac", "2", "-i", (ref / "140.ref.s16").toString]
    ff (pcm ++ #["-t", "30", "-c:a", "aac_at", "-b:a", "128k", "-f", "adts", adts "at128"]); refs "at128"
    ff (pcm ++ #["-t", "20", "-c:a", "aac_at", "-b:a", "48k", "-f", "adts", adts "at48"]); refs "at48"
    ff (pcm ++ #["-t", "10", "-ac", "1", "-ar", "22050", "-c:a", "aac_at", "-b:a", "24k", "-f", "adts", adts "at24m"])
    refs "at24m"
    ff (pcm ++ #["-t", "10", "-c:a", "aac_at", "-profile:a", "4", "-b:a", "48k", "-f", "adts", adts "athe"])
  -- The transcript's input: the first frames of each stream (the yt140
  -- segments were picked for the tools they contain: intensity, PNS, short
  -- windows, TNS).
  let sets := [("yt140", "140", 0, 160), ("yt140b", "140", 2000, 300), ("yt140c", "140", 6000, 300),
    ("s44", "s44", 0, 120), ("s48", "s48", 0, 60), ("s32", "s32", 0, 40), ("s22m", "s22m", 0, 40),
    ("s16", "s16", 0, 30), ("s8m", "s8m", 0, 30), ("s96", "s96", 0, 40), ("at48", "at48", 500, 60), ("athe", "athe", 0, 3)]
  let mut lines := #[]
  for (name, file, start, n) in sets do
    unless ← FilePath.pathExists (adts file) do continue
    let frames ← IO.ofExcept (adtsFrames (← IO.FS.readBinFile (adts file)) start n)
    lines := lines.push s!"  (\"{name}\", \"{hexOf frames}\")"
  IO.FS.writeFile (out / "fixture/AacFixture.lean")
    ("/-! Generated by tools/lean/yt/Fixtures.lean (aac): ADTS frames as hex. -/\n" ++
     "def AacFixture.streams : Array (String × String) := #[\n" ++ ",\n".intercalate lines.toList ++ "\n]\n")

def main (args : List String) : IO UInt32 := do
  let root : FilePath := (← run "git" #["rev-parse", "--show-toplevel"]).trimAscii.toString
  let (kind, outArg) ← match args with
    | [k] => pure (k, none)
    | [k, "--out", o] => pure (k, some (o : FilePath))
    | _ => IO.eprintln "usage: Fixtures.lean http|json|jpeg|mp4|aac [--out DIR]"; return 2
  let default := match kind with
    | "aac" => root / "assets/build/lean-aac"
    | k => root / "assets/build/lean-formats" / k
  let cwd ← IO.currentDir
  let out := match outArg with
    | some o => if o.isAbsolute then o else cwd / o
    | none => default
  IO.FS.createDirAll out
  match kind with
  | "http" => http root out
  | "json" => json root out
  | "jpeg" => jpeg root out (if outArg.isSome then out / "Fixtures.lean" else root / "test/lean/yt/jpeg/JpegTest/Fixtures.lean")
  | "mp4" => mp4 root out
  | "aac" => aac root out
  | k => IO.eprintln s!"unknown kind {k}"; return 2
  IO.println s!"{kind}: {out}"
  return 0
