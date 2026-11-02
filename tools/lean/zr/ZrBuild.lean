import ZrPaths
import Lean.Data.Json
import Std.Data.HashMap

/-!
# zr-build: our runtime and Lean apps for the TV (32-bit ARM Linux)

    lake -d tools/lean/zr exe zr-build rt [OUT]
    lake -d tools/lean/zr exe zr-build app PKG MODULE ENTRY OUT
    lake -d tools/lean/zr exe zr-build link-check PKG [RT]
    lake -d tools/lean/zr exe zr-build transcript PKG [TIMEOUT_SECONDS]
    lake -d tools/lean/zr exe zr-build timed PKG FN STAGES [--keep DIR] [-- QEMU_ARGS...]
    lake -d tools/lean/zr exe zr-build conformance [--record]
    lake -d tools/lean/zr exe zr-build profile PKG FN STAGE [TOP] [--callers F,G] [--focus F] [--keep DIR]

`rt`: the runtime (`ZrLean.Low`), the app platform (`ZrTv`) and the H.264
kernels for 32-bit ARM, into OUT (default tools/lean/zr/.lake/rt, private to
the worktree: assets/ is shared by all worktrees):
- OUT/gen/rt/*.c, OUT/gen/tv/*.c: the Lean compiler's C, one file per module
  (`ZrLean`, and `ZrTv` for apps only), normalized by zr-cgen;
- OUT/gen/include: freestanding headers, lean/lean.h among them;
- OUT/gen/runtime.S: lean.h's out-of-line names as branch stubs, storage and
  the Linux system call; OUT/gen/tv.S: ZrTv's video routines;
- OUT/gen/h264/*.S: FFmpeg's H.264 NEON as written in the DSL
  (`ZrAsm.Ffmpeg`), of which the Lean decoder's kernels call five files;
- OUT/obj/*.o, OUT/obj/tv/*.o, OUT/obj/h264/*.o: all of it compiled.
No C is written by hand: it is the Lean compiler's, or the DSL's assembly.

`app`: Lean package PKG's MODULE and the modules it imports (only those),
with its exported `ENTRY : (argc : UInt32) → BaseIO UInt32`, a `main` and
the C library pieces apps need (`zr-asm-gen app`: `_start`, `memcpy`,
`memmove`, 64-bit division), linked with the runtime, ZrTv and the H.264
kernels, without a C library. `YT_STRIP=` (set, empty) keeps the symbols;
`YT_MAP=FILE` writes the linker map (tools/Composition.lean reads it).

`link-check`: can package PKG's code run on our runtime? Compiles all its C
(but not host-only `Host/` modules: the runtime provides those symbols) and
links it with the runtime without dead-code removal, then lists what is
missing: `l_*` (Lean's standard library: the runtime links no Init code; use
your own helper, or ask for the function to be added) and `lean_*` (runtime
functions not implemented yet).

`transcript`: same Lean, two runtimes. Package PKG exports
`zr_test_transcript (k : Nat) : ByteArray` (k is 0) and has a host
executable `transcript` that prints it. Its output under upstream Lean is
compared byte for byte with a 32-bit ARM binary's on our runtime, run as
PID 1 in QEMU (zr-qemu32): the package's and its path dependencies' C
(without host executables' `main` and `Host/` modules, which the runtime
build provides on ARM) linked with the whole runtime build, and a `main` in
the DSL (`zr-asm-gen transcript`), without a C library. Lean panics go to
stderr on the host and are not compared.

`timed`: instruction counts. A binary like the transcript's, but its DSL
entry (`zr-asm-gen timed`) calls the package's exported `FN` on each of
STAGES (comma-separated small Nats), reading the monotonic clock around each
call. Run under QEMU with QEMU_ARGS (`-icount shift=0`: a guest nanosecond
is one instruction; a TCG plugin to profile); prints each stage's time and
`FN`'s result (a checksum, for FNs that return `UInt32`). `--keep DIR` keeps
the unstripped binary (DIR/timed) for symbols.

`conformance`: the runtime conformance suite (test/lean/zr/conformance) on 32-bit
ARM: its transcript and Lean panic messages must be byte-identical to
upstream Lean's (its expected/). `--record` records those
instead: the suite's host executable (`conformance`, upstream Lean's
runtime), its standard output and its panic messages (standard error).
The binary: the runtime, ZrTv.Linux, the suite and its Linux entry
(ConformanceLinux.lean) with the apps' DSL main, run as PID 1 in QEMU.

`profile`: where the instructions go. The timed binary for FN STAGE runs
with a QEMU TCG plugin (profile/plugin.c here: C, against QEMU's plugin
API, built here for the host) that counts executed instructions per
translation block; they are attributed to the binary's functions by its
symbol table and the TOP (default 40) printed. `--callers` also counts
those functions' calls by caller (a traced caller shown with its own
caller), `--focus` lists one function's hottest blocks, `--keep` keeps the
binary, its symbols and the raw profile.

Tools run without `lake env`'s library paths, so zig loads its own LLVM.
-/

open System

/-! ### Running tools -/

def cleanEnv : Array (String × Option String) :=
  #["DYLD_LIBRARY_PATH", "LD_LIBRARY_PATH", "LEAN_PATH", "LEAN_SRC_PATH"].map (·, none)

/-- Run a tool; its standard output, or an error with all of its output. -/
def run (cmd : String) (args : Array String) (cwd : Option FilePath := none) : IO String := do
  let o ← IO.Process.output { cmd, args, cwd, env := cleanEnv }
  unless o.exitCode == 0 do
    throw <| IO.userError s!"{cmd} {" ".intercalate args.toList} failed:\n{o.stdout}{o.stderr}"
  return o.stdout

/-- Run `jobs`, `width` at a time; the first failure is rethrown. -/
def parallel (jobs : Array (IO Unit)) (width := 8) : IO Unit := do
  let mut i := 0
  while i < jobs.size do
    let tasks ← (jobs.extract i (i + width)).mapM fun (j : IO Unit) => IO.asTask j
    for t in tasks do
      match ← IO.wait t with
      | Except.ok _ => pure ()
      | Except.error e => throw e
    i := i + width

def target : Array String := #["-target", "arm-linux-musleabihf", "-mcpu=cortex_a55"]

/-- Compile the compiler's C against the runtime's generated headers. -/
def cc (gen src obj : FilePath) : IO Unit :=
  discard <| run "zig" (#["cc"] ++ target ++ #["-O2", "-ffreestanding", "-fno-builtin", "-nostdinc",
    "-fno-exceptions", "-ffunction-sections", "-fdata-sections", "-fno-unwind-tables",
    "-fno-asynchronous-unwind-tables", "-std=gnu11", "-Wno-everything", "-DNDEBUG",
    s!"-I{gen / "include"}", s!"-I{gen / "rt"}", "-include", "lean/zr_fastpath.h",
    "-c", src.toString, "-o", obj.toString])

def assemble (src obj : FilePath) : IO Unit :=
  discard <| run "zig" (#["cc"] ++ target ++ #["-c", src.toString, "-o", obj.toString])

/-! ### Files -/

/-- The files in `dir` with extension `ext`, sorted by name (byte order). -/
def filesIn (dir : FilePath) (ext : String) : IO (Array FilePath) := do
  unless ← dir.isDir do return #[]
  let es ← dir.readDir
  return ((es.filter (·.path.extension == some ext)).map (·.path)).qsort (·.toString < ·.toString)

def removeAll (dir : FilePath) (ext : String) : IO Unit := do
  for f in ← filesIn dir ext do IO.FS.removeFile f

/-- `p` as an absolute path (relative ones are relative to the caller's directory). -/
def absolute (p : FilePath) : IO FilePath := do
  if p.isAbsolute then return p
  return (← IO.currentDir) / p

/-- This tools package (tools/lean/zr): the executable is
tools/lean/zr/.lake/build/bin/zr-build. -/
def zrDir : IO FilePath := do
  let app ← IO.FS.realPath (← IO.appPath)
  let some zr := app.parent.bind (·.parent) |>.bind (·.parent) |>.bind (·.parent)
    | throw <| IO.userError s!"zr-build: unexpected location {app}"
  return zr

/-- The repository root, three levels above the tools package. -/
def repoRoot (zr : FilePath) : FilePath :=
  ((zr.parent.bind (·.parent)).bind (·.parent)).getD zr

/-- The runtime package (src/lean/zr): the Lean that runs on the TV. -/
def rtPkg (zr : FilePath) : FilePath := repoRoot zr / "src/lean/zr"


/-! ### The runtime -/

/-- Module `m` (a path below the runtime package, without `.lean`) as a C file name. -/
def cName (m : String) : String := m.replace "/" "_" ++ ".c"

/-- The compiler's C of every module under `pkg/lib`, as `dest/<module>.c`. -/
def copyIr (pkg dest : FilePath) (lib : String) : IO Unit := do
  IO.FS.createDirAll dest
  removeAll dest "c"
  for f in ← (pkg / lib).walkDir do
    if f.extension != some "lean" then continue
    let rel := (f.withExtension "").toString.drop (pkg.toString.length + 1) |>.toString
    IO.FS.writeBinFile (dest / cName rel) (← IO.FS.readBinFile ((← ZrPaths.buildDir pkg) / "ir" / (rel ++ ".c")))

/-- OUT/gen: the runtime's and ZrTv's C, headers and assembly (see above). -/
def generate (zr gen : FilePath) : IO Unit := do
  let pkg ← IO.FS.realPath (rtPkg zr)
  discard <| run "lake" #["build"] pkg                     -- the runtime's C
  discard <| run "lake" #["build"] zr                      -- the tools
  copyIr pkg (gen / "rt") "ZrLean"
  copyIr pkg (gen / "tv") "ZrTv"
  let bin (t : String) := (zr / ".lake/build/bin" / t).toString
  discard <| run (bin "zr-cgen") #["abi-normalize", gen.toString]
  discard <| run (bin "zr-cgen") #["header-inline", (gen / "rt").toString, (gen / "include/lean").toString]
  discard <| run (bin "zr-abi-gen") #[(gen / "include").toString, (gen / "include/lean/bindings.tsv").toString]
  discard <| run (bin "zr-cgen") #["fastpath", ((← ZrPaths.buildDir pkg) / "ir/ZrLean/Low/Fast.c").toString,
    (gen / "rt/zr_fastpath.generated.h").toString]
  discard <| run (bin "zr-asm-gen") #[gen.toString]
  discard <| run (bin "zr-cgen") #["check", gen.toString]

/-- The FFmpeg NEON files the Lean decoder's kernels (`ZrTv.H264Kernels`) call. -/
def h264Files : Array String := #["h264qpel_neon", "h264cmc_neon", "hpeldsp_neon", "h264dsp_neon", "h264idct_neon"]

def buildRt (zr out : FilePath) : IO Unit := do
  let gen := out / "gen"
  let obj := out / "obj"
  generate zr gen
  for d in [obj, obj / "tv", obj / "h264", gen / "h264"] do
    IO.FS.createDirAll d
    removeAll d "o"
  let tv ← filesIn (gen / "tv") "c"
  let rt ← filesIn (gen / "rt") "c"
  parallel <| tv.map (fun f => cc gen f (obj / "tv" / (f.fileStem.get! ++ ".o"))) ++
    rt.map (fun f => cc gen f (obj / (f.fileStem.get! ++ ".o")))
  assemble (gen / "runtime.S") (obj / "runtime.o")
  assemble (gen / "tv.S") (obj / "tv/tv_asm.o")
  discard <| run (zr / ".lake/build/bin/zr-asm-gen").toString #["ffmpeg-arm", (gen / "h264").toString]
  parallel <| h264Files.map fun f => assemble (gen / "h264" / (f ++ ".S")) (obj / "h264" / (f ++ ".o"))

/-- The runtime's objects in link order: runtime, ZrTv, H.264 kernels. -/
def rtObjects (rt : FilePath) : IO (Array FilePath) :=
  return (← filesIn (rt / "obj") "o") ++ (← filesIn (rt / "obj/tv") "o") ++ (← filesIn (rt / "obj/h264") "o")

/-- The worktree's runtime, built if missing. -/
def ensureRt (zr rt : FilePath) : IO Unit := do
  unless (← (rt / "obj/runtime.o").pathExists) && (← (rt / "obj/h264").isDir) do
    buildRt zr rt

/-! ### Apps -/

/-- The name of the module initializer exported by the compiler's C `src`. -/
def initializer (src : FilePath) : IO String := do
  let marker := "LEAN_EXPORT lean_object* initialize_"
  let some rest := ((← IO.FS.readFile src).splitOn marker)[1]?
    | throw <| IO.userError s!"zr-build: no module initializer in {src}"
  return "initialize_" ++ String.ofList (rest.toList.takeWhile fun c => c.isAlphanum || c == '_')

def buildApp (zr pkg : FilePath) (module entry : String) (out : FilePath) : IO Unit := do
  let rt := zr / ".lake/rt"
  ensureRt zr rt
  discard <| run "lake" #["build", "zr-cgen", "zr-asm-gen"] zr
  discard <| run "lake" #["build"] pkg
  let init ← initializer ((← ZrPaths.buildDir pkg) / "ir" / (module.replace "." "/" ++ ".c"))
  let files := ((← run (zr / ".lake/build/bin/zr-cgen").toString #["imports", pkg.toString, module]).splitOn "\n").filter
    (· != "")
  let work ← IO.FS.createTempDir
  try
    let objs := (List.range files.length).toArray.map fun i => work / s!"yt{i + 1}.o"
    parallel <| (files.toArray.zip objs).map fun (f, o) => cc (rt / "gen") f o
    if let some d := out.parent then IO.FS.createDirAll d
    discard <| run (zr / ".lake/build/bin/zr-asm-gen").toString #["app", (work / "main.S").toString, init, entry]
    assemble (work / "main.S") (work / "main.o")
    let strip := match ← IO.getEnv "YT_STRIP" with | none => #["-s"] | some "" => #[] | some s => #[s]
    let map ← IO.getEnv "YT_MAP"
    let linked ← run "zig" (#["cc"] ++ target ++ #["-static", "-nostdlib"] ++ strip ++
      (if map.isSome then #["-Wl,--print-map"] else #[]) ++ #["-Wl,--gc-sections"] ++
      (objs.qsort (·.toString < ·.toString) ++ #[work / "main.o"] ++ (← rtObjects rt)).map (·.toString) ++
      #["-o", out.toString])
    match map with
    | some f => IO.FS.writeFile f linked
    | none => IO.print linked
    IO.println s!"{out}: {(← out.metadata).byteSize} bytes ({module}: {files.length} modules, entry {entry})"
  finally
    IO.FS.removeDirAll work

/-! ### Link check -/

def linkCheck (zr pkg rt : FilePath) : IO UInt32 := do
  ensureRt zr rt
  discard <| run "lake" #["build"] pkg
  let ir := (← ZrPaths.buildDir pkg) / "ir"
  let files := ((← if ← ir.isDir then ir.walkDir else pure #[]).filter fun f =>
    f.extension == some "c" && !(f.components.contains "Host")).qsort (·.toString < ·.toString)
  let work ← IO.FS.createTempDir
  try
    let objs := (List.range files.size).toArray.map fun i => work / s!"pkg{i + 1}.o"
    parallel <| (files.zip objs).map fun (f, o) => cc (rt / "gen") f o
    IO.FS.writeFile (work / "main.c") "int main(void){return 0;}\n"
    -- The per-app main's zr_tv_args_* are linked only into apps; stand-ins
    -- here keep this check about Lean.
    let mut stubs : Array String := #[]
    for f in ← filesIn (rt / "gen/tv") "c" do
      for piece in ((← IO.FS.readFile f).splitOn "zr_tv_args_").drop 1 do
        let suffix := String.ofList (piece.toList.takeWhile fun c => c.isLower || c == '_')
        let name := "zr_tv_args_" ++ suffix
        if (piece.drop suffix.length).startsWith "(" && !stubs.contains name then stubs := stubs.push name
    IO.FS.writeFile (work / "tv_stubs.c") (String.join ((stubs.qsort (· < ·)).toList.map fun n => s!"int {n}(void)\{return 0;}\n"))
    assemble (work / "main.c") (work / "main.o")
    assemble (work / "tv_stubs.c") (work / "tv_stubs.o")
    let o ← IO.Process.output { cmd := "zig", env := cleanEnv, args := #["cc"] ++ target ++ #["-static"] ++
      ((← rtObjects rt) ++ #[work / "tv_stubs.o"] ++ objs ++ #[work / "main.o"]).map (·.toString) ++
      #["-o", (work / "a.out").toString] }
    if o.exitCode == 0 then
      IO.println s!"link-check: PASS ({files.size} C files link against the runtime for 32-bit ARM)"
      return 0
    let missing := ((o.stderr.splitOn "undefined symbol: ").drop 1).map fun s =>
      String.ofList (s.toList.takeWhile fun c => c.isAlphanum || c == '_')
    let missing := (missing.toArray.qsort (· < ·)).toList.eraseDups
    if missing.isEmpty then
      IO.println s!"link-check: link failed:\n{"\n".intercalate ((o.stderr.splitOn "\n").take 20)}"
    else
      IO.println s!"link-check: {files.size} C files; missing on the Lean runtime:"
      for m in missing do IO.println s!"  {m}"
    return 1
  finally
    IO.FS.removeDirAll work

/-! ### ARM transcripts -/

/-- The configured IR directories of `pkg` and its path dependencies
(lake-manifest.json): their compiled C goes into the binary. -/
def irDirs (pkg : FilePath) : IO (Array FilePath) := do
  let mut dirs := #[(← ZrPaths.buildDir pkg) / "ir"]
  let j ← IO.ofExcept (Lean.Json.parse (← IO.FS.readFile (pkg / "lake-manifest.json")))
  for p in ← IO.ofExcept (j.getObjValAs? (Array Lean.Json) "packages") do
    if p.getObjValAs? String "type" matches .ok "path" then
      let ir := (← ZrPaths.buildDir (pkg / (← IO.ofExcept (p.getObjValAs? String "dir")))) / "ir"
      if ← ir.isDir then dirs := dirs.push (← IO.FS.realPath ir)
  return dirs

/-- What lies between `begin` and the last `stop` in `s`, if both are there. -/
def between (s begin stop : String) : Option String := do
  let after ← (s.splitOn begin)[1]?
  let parts := after.splitOn stop
  guard (parts.length ≥ 2)
  return stop.intercalate parts.dropLast

/-- Build a 32-bit ARM test binary `bin` from package `pkg`: its and its path
dependencies' C (not host executables' `main`, not `Host/` modules: the
runtime build provides those on ARM), the whole runtime build, and a DSL
main, `zr-asm-gen MODE OUT INIT ARGS...`, where INIT initializes the module
that exports `fn`. Without a C library. -/
def armBinary (zr pkg : FilePath) (fn mode : String) (args : Array String) (bin : FilePath)
    (strip := true) : IO Unit := do
  let rt := zr / ".lake/rt"
  ensureRt zr rt
  discard <| run "lake" #["build", "zr-qemu32", "zr-asm-gen"] zr
  discard <| run "lake" #["build"] pkg
  let mut found : Array (FilePath × String) := #[]
  for d in ← irDirs pkg do
    for f in ← d.walkDir do
      if f.extension != some "c" || f.components.contains "Host" then continue
      let text ← IO.FS.readFile f
      unless text.startsWith "int main(" || (text.splitOn "\nint main(").length > 1 do
        found := found.push (f, text)
  let files := found.qsort (·.1.toString < ·.1.toString)
  -- the C that defines `fn`: `LEAN_EXPORT <type> fn(`
  let defines (t : String) := ((t.splitOn "LEAN_EXPORT ").drop 1).any fun p => (p.dropWhile (· != ' ')).startsWith s!" {fn}("
  let some (src, _) := files.find? (defines ·.2)
    | throw <| IO.userError s!"zr-build: no {fn} in {pkg}"
  let init ← initializer src
  let work ← IO.FS.createTempDir
  try
    let objs := (List.range files.size).toArray.map fun i => work / s!"pkg{i + 1}.o"
    parallel <| (files.zip objs).map fun ((f, _), o) => cc (rt / "gen") f o
    discard <| run (zr / ".lake/build/bin/zr-asm-gen").toString (#[mode, (work / "main.S").toString, init] ++ args)
    assemble (work / "main.S") (work / "main.o")
    discard <| run "zig" (#["cc"] ++ target ++ #["-static", "-nostdlib"] ++ (if strip then #["-s"] else #[]) ++
      #["-Wl,--gc-sections"] ++ ((← rtObjects rt) ++ objs ++ #[work / "main.o"]).map (·.toString) ++
      #["-o", bin.toString])
  finally
    IO.FS.removeDirAll work

def transcript (zr pkg : FilePath) (timeout : String) : IO UInt32 := do
  let host ← (do discard <| run "lake" #["build"] pkg; run "lake" #["exe", "transcript"] pkg)
  let work ← IO.FS.createTempDir
  try
    let bin := work / "transcript-arm32"
    armBinary zr pkg "zr_test_transcript" "transcript" #[] bin
    let console ← run (zr / ".lake/build/bin/zr-qemu32").toString #[bin.toString, timeout]
    let arm := between console "\n==lean-transcript==\n" "==lean-end==\n"
    if arm == some host && !host.isEmpty then
      IO.println s!"arm32-transcript: PASS ({host.utf8ByteSize} bytes identical; binary {(← bin.metadata).byteSize} bytes)"
      return 0
    IO.println "arm32-transcript: FAIL"
    match arm with
    | none => IO.println "  no transcript between the markers"
    | some a =>
      let (h, l) := (host.splitOn "\n", a.splitOn "\n")
      for i in [0:max h.length l.length] do
        if h[i]? != l[i]? then
          IO.println s!"  first difference, line {i + 1}:\n  host: {h[i]?.getD "(end)"}\n  arm:  {l[i]?.getD "(end)"}"
          break
    for line in ((console.splitOn "\n").filter (!·.startsWith "==lean")).reverse.take 5 |>.reverse do
      IO.println s!"  {line}"
    return 1
  finally
    IO.FS.removeDirAll work

/-- A hex word. -/
def hexWord (s : String) : Nat :=
  s.toList.foldl (fun a c => 16 * a + if c.isDigit then c.toNat - 48 else c.toLower.toNat - 87) 0

/-- `zr-build timed`: `fn` on each stage, timed in the guest (with QEMU's
`-icount shift=0`, nanoseconds are instructions). With `keep`, the binary
(unstripped, for symbols) stays there as `timed`. -/
def timed (zr pkg : FilePath) (fn : String) (stages : String) (keep : Option FilePath)
    (qemuArgs : Array String) : IO UInt32 := do
  let work ← IO.FS.createTempDir
  try
    let bin := (keep.getD work) / "timed"
    if let some k := keep then IO.FS.createDirAll k
    armBinary zr pkg fn "timed" #[fn, stages] bin (strip := keep.isNone)
    let console ← run (zr / ".lake/build/bin/zr-qemu32").toString
      (#[bin.toString, "1800"] ++ (if qemuArgs.isEmpty then #[] else #["--"] ++ qemuArgs))
    let some line := ((console.splitOn "\n==timed== ")[1]?).map (·.splitOn "\n" |>.head!)
      | IO.println "timed: no result"; return 1
    let w := ((line.splitOn " ").filter (· != "")).toArray.map hexWord
    let ss := stages.splitOn ","
    let ns (i : Nat) := w[i]! * 1000000000 + w[i + 1]!
    for i in [0:ss.length] do
      IO.println s!"{fn} stage {ss[i]!}: {ns (3 * i + 3) - ns (3 * i)} ns, result 0x{String.ofList (Nat.toDigits 16 w[3 * i + 2]!)}"
    return 0
  finally
    IO.FS.removeDirAll work

/-! ### Runtime conformance -/

def conformance (zr : FilePath) : IO UInt32 := do
  let pkg := repoRoot zr / "test/lean/zr/conformance"
  let work ← IO.FS.createTempDir
  try
    let gen := work / "gen"
    generate zr gen
    discard <| run "lake" #["build", "zr-qemu32"] zr
    discard <| run "lake" #["build"] pkg
    discard <| run "lake" #["build", "ConformanceLinux"] pkg
    let ir := (← ZrPaths.buildDir pkg) / "ir"
    let srcs := (← filesIn (gen / "rt") "c") ++
      #[gen / "tv/ZrTv_Linux.c", ir / "ZrLeanTest/Conformance.c", ir / "ConformanceLinux.c"]
    let objs := (List.range srcs.size).toArray.map fun i => work / s!"o{i + 1}.o"
    parallel <| (srcs.zip objs).map fun (f, o) => cc gen f o
    assemble (gen / "runtime.S") (work / "runtime.o")
    discard <| run (zr / ".lake/build/bin/zr-asm-gen").toString
      #["app", (work / "main.S").toString, ← initializer (ir / "ConformanceLinux.c"), "zr_conformance_main"]
    assemble (work / "main.S") (work / "main.o")
    let bin := work / "conformance-arm32"
    discard <| run "zig" (#["cc"] ++ target ++ #["-static", "-nostdlib", "-s", "-Wl,--gc-sections"] ++
      (objs ++ #[work / "runtime.o", work / "main.o"]).map (·.toString) ++ #["-o", bin.toString])
    let console ← run (zr / ".lake/build/bin/zr-qemu32").toString #[bin.toString, "180"]
    let got := [("expected.txt", between console "\n==lean-transcript==\n" "==lean-panics==\n"),
                ("expected-panics.txt", between console "==lean-panics==\n" "==lean-end==\n")]
    let mut ok := true
    for (name, out) in got do
      let want ← IO.FS.readFile (pkg / "expected" / name)
      if out == some want then IO.println s!"{name}: identical to upstream"
      else
        ok := false
        IO.println s!"{name}: DIFFERS"
        match out with
        | none => IO.println "  missing from the console"
        | some o =>
          let (w, g) := (want.splitOn "\n", o.splitOn "\n")
          for i in [0:max w.length g.length] do
            if w[i]? != g[i]? then
              IO.println s!"  line {i + 1}:\n  upstream: {w[i]?.getD "(end)"}\n  ours:     {g[i]?.getD "(end)"}"
              break
    IO.println s!"binary: {(← bin.metadata).byteSize} bytes"
    IO.println s!"conformance-arm32: {if ok then "PASS" else "FAIL"}"
    return if ok then 0 else 1
  finally
    IO.FS.removeDirAll work

def recordConformance (zr : FilePath) : IO UInt32 := do
  let pkg := repoRoot zr / "test/lean/zr/conformance"
  discard <| run "lake" #["build", "conformance"] pkg
  let o ← IO.Process.output { cmd := (pkg / ".lake/build/bin/conformance").toString,
                              env := #[("LEAN_BACKTRACE", some "0")] }
  IO.FS.writeFile (pkg / "expected/expected.txt") o.stdout
  IO.FS.writeFile (pkg / "expected/expected-panics.txt") o.stderr
  IO.println s!"recorded {(o.stdout.splitOn "\n").length - 1} lines, {(o.stderr.splitOn "\n").length - 1} panic lines"
  return 0

/-! ### Profiles -/

/-- An executable's code symbols (address, name) from its ELF symbol table:
defined, in executable sections, without ARM mapping symbols (`$a`, `$d`). -/
def codeSymbols (path : FilePath) : IO (Array (Nat × String)) := do
  let b ← IO.FS.readBinFile path
  let u8 (o : Nat) : Nat := (b.get! o).toNat
  let u16 (o : Nat) : Nat := u8 o + 256 * u8 (o + 1)
  let u32 (o : Nat) : Nat := u16 o + 65536 * u16 (o + 2)
  let cstr (o : Nat) : String := Id.run do
    let mut e := o
    while e < b.size && b.get! e != 0 do e := e + 1
    return String.fromUTF8! (b.extract o e)
  let (shoff, shentsize, shnum) := (u32 32, u16 46, u16 48)
  let sh (i k : Nat) : Nat := u32 (shoff + i * shentsize + 4 * k)   -- field k of section header i
  let mut out := #[]
  for i in [0:shnum] do
    if sh i 1 != 2 then continue                                    -- SHT_SYMTAB
    let (off, size, strs) := (sh i 4, sh i 5, sh (sh i 6) 4)
    for j in [0:size / 16] do
      let e := off + 16 * j
      let shndx := u16 (e + 14)
      let name := cstr (strs + u32 e)
      if shndx == 0 || shndx ≥ 0xff00 || name.isEmpty || name.startsWith "$" then continue
      if sh shndx 2 &&& 4 == 0 || u8 (e + 12) % 16 > 2 then continue  -- SHF_EXECINSTR; NOTYPE/OBJECT/FUNC
      out := out.push (u32 (e + 4), name)
  return out.qsort (·.1 < ·.1)

/-- The QEMU plugin, built for the host against the installed QEMU's and glib's headers. -/
def buildPlugin (zr out : FilePath) : IO Unit := do
  let path := ((← IO.getEnv "PATH").getD "").splitOn ":"
  let which (cmd : String) : IO (Option FilePath) := do
    for d in path do
      if ← (FilePath.mk d / cmd).pathExists then return some (← IO.FS.realPath (FilePath.mk d / cmd))
    return none
  let some qemu ← which "qemu-system-aarch64" | throw <| IO.userError "zr-build: no qemu-system-aarch64"
  let glib := match ← which "glib-compile-schemas" with
    | some g => (g.parent.bind (·.parent)).getD "/opt/homebrew/opt/glib"
    | none => "/opt/homebrew/opt/glib"
  let some qroot := qemu.parent.bind (·.parent) | throw <| IO.userError "zr-build: QEMU's prefix?"
  discard <| run "zig" #["cc", "-target", "aarch64-macos", "-O2", "-shared", "-fPIC", "-Wl,-undefined,dynamic_lookup",
    s!"-I{qroot / "include"}", s!"-I{glib / "include/glib-2.0"}", s!"-I{glib / "lib/glib-2.0/include"}",
    (zr / "profile/plugin.c").toString, "-o", out.toString]

def hexVal (s : String) : Nat :=
  s.toList.foldl (fun a c =>
    if c.isDigit then 16 * a + (c.toNat - 48)
    else if 'a' ≤ c.toLower && c.toLower ≤ 'f' then 16 * a + (c.toLower.toNat - 87) else a) 0

def decVal (s : String) : Nat := s.toList.foldl (fun a c => 10 * a + (c.toNat - 48)) 0

def padLeft (s : String) (n : Nat) : String := String.ofList (List.replicate (n - s.length) ' ') ++ s

/-- `c` as a percentage of `t`, one decimal. -/
def pct (c t : Nat) : String :=
  let p := if t == 0 then 0 else (1000 * c + t / 2) / t
  padLeft s!"{p / 10}.{p % 10}%" 6

/-- The last symbol at or below `pc` (`syms` sorted by address, nonempty). -/
def symbolAt (syms : Array (Nat × String)) (pc : Nat) : String := Id.run do
  let mut (lo, hi) := (0, syms.size)                                 -- syms[lo] ≤ pc < syms[hi]
  while hi - lo > 1 do
    let m := (lo + hi) / 2
    if syms[m]!.1 ≤ pc then lo := m else hi := m
  return syms[lo]!.2

def bump (m : Std.HashMap String Nat) (k : String) (c : Nat) : Std.HashMap String Nat :=
  m.insert k (m.getD k 0 + c)

def topOf (m : Std.HashMap String Nat) (n : Nat) : List (String × Nat) :=
  (m.toList.toArray.qsort fun a b => a.2 > b.2).toList.take n

/-- The plugin's lines `PC EXEC INSNS` (a block run EXEC times, of INSNS
instructions) and `c FN LR PREV N` (N calls of FN returning to LR; PREV the
previous traced call's return address), attributed to `syms`. -/
def summarize (syms : Array (Nat × String)) (prof : List String) (top : Nat) (focus : Option String) : IO Unit := do
  let words (l : String) := (l.splitOn " ").filter (· != "")
  let lo := syms[0]!.1
  let hi := syms[syms.size - 1]!.1 + 0x10000
  let traced := prof.filterMap fun l => match words l with
    | ["c", fn, _, _, _] => some (symbolAt syms (hexVal fn))
    | _ => none
  let mut cost : Std.HashMap String Nat := {}
  let mut calls : Std.HashMap String (Std.HashMap String Nat) := {}
  let mut (total, outside) := (0, 0)
  for l in prof do
    match words l with
    | ["c", fn, lr, prev, n] =>
      let callee := symbolAt syms (hexVal fn)
      let caller := symbolAt syms (hexVal lr - 1)
      let caller := if traced.contains caller then s!"{caller} <- {symbolAt syms (hexVal prev - 1)}" else caller
      calls := calls.insert callee (bump (calls.getD callee {}) caller (decVal n))
    | [pc, ex, n] =>
      let (a, c) := (hexVal pc, decVal ex * decVal n)
      if a < lo || hi ≤ a then outside := outside + c          -- the kernel, the zImage decompressor
      else
        cost := bump cost (symbolAt syms a) c
        total := total + c
    | _ => pure ()
  IO.println s!"{total} instructions in the binary (and {outside} outside it: kernel and boot)"
  for (name, c) in topOf cost top do
    IO.println s!"{pct c total} {padLeft (toString c) 12} {String.ofList (name.toList.take 110)}"
  if let some f := focus then
    let mut blocks : Array (Nat × Nat × Nat) := #[]
    for l in prof do
      if let [pc, ex, n] := words l then
        let a := hexVal pc
        if lo ≤ a && a < hi && symbolAt syms a == f then blocks := blocks.push (a, decVal ex, decVal n)
    IO.println s!"hottest blocks of {f} (address, runs, instructions, total):"
    for (a, ex, n) in (blocks.qsort fun x y => x.2.1 * x.2.2 > y.2.1 * y.2.2).toList.take 40 do
      IO.println s!"  {String.ofList (Nat.toDigits 16 a)} {padLeft (toString ex) 9} {padLeft (toString n) 4} {padLeft (toString (ex * n)) 10}"
  for (callee, m) in calls.toList do
    IO.println s!"callers of {callee} (calls):"
    for (name, n) in topOf m 16 do
      IO.println s!"  {padLeft (toString n) 10} {String.ofList (name.toList.take 150)}"

def profile (zr pkg : FilePath) (fn stage : String) (top : Nat) (callers : List String) (focus : Option String)
    (keep : Option FilePath) : IO UInt32 := do
  let work ← IO.FS.createTempDir
  try
    let dir := keep.getD work
    IO.FS.createDirAll dir
    let bin := dir / "timed"
    armBinary zr pkg fn "timed" #[fn, stage] bin (strip := false)
    buildPlugin zr (work / "plugin.dylib")
    let syms ← codeSymbols bin
    if syms.isEmpty then IO.println "profile: no symbols"; return 1
    IO.FS.writeFile (dir / "syms.txt") (String.join (syms.toList.map fun (a, n) => s!"{String.ofList (Nat.toDigits 16 a)} {n}\n"))
    let fns := callers.filterMap fun c => (syms.find? (·.2 == c)).map fun (a, _) => s!",fn={String.ofList (Nat.toDigits 16 a)}"
    discard <| run (zr / ".lake/build/bin/zr-qemu32").toString
      #[bin.toString, "1800", "--", "-plugin", s!"{work / "plugin.dylib"},out={dir / "profile.txt"}{String.join fns}"]
    summarize syms ((← IO.FS.readFile (dir / "profile.txt")).splitOn "\n") top focus
    if keep.isSome then IO.println s!"kept {dir}: timed (the binary), syms.txt, profile.txt"
    return 0
  finally
    IO.FS.removeDirAll work

def main (args : List String) : IO UInt32 := do
  let zr ← zrDir
  match args with
  | ["rt"] | ["rt", _] =>
    let out ← absolute (match args with | [_, o] => o | _ => (zr / ".lake/rt").toString)
    IO.FS.createDirAll out
    buildRt zr (← IO.FS.realPath out)
    IO.println out
    return 0
  | ["conformance"] => conformance zr
  | "profile" :: pkg :: fn :: stage :: rest =>
    let rec opts (top : Nat) (callers : List String) (focus : Option String) (keep : Option String) :
        List String → Except String (Nat × List String × Option String × Option String)
      | [] => .ok (top, callers, focus, keep)
      | "--callers" :: c :: r => opts top (c.splitOn ",") focus keep r
      | "--focus" :: f :: r => opts top callers (some f) keep r
      | "--keep" :: d :: r => opts top callers focus (some d) r
      | n :: r => if n.all Char.isDigit then opts n.toNat! callers focus keep r else .error s!"unexpected {n}"
    let (top, callers, focus, keep) ← IO.ofExcept (opts 40 [] none none rest)
    profile zr (← IO.FS.realPath pkg) fn stage top callers focus (← keep.mapM (absolute ·))
  | ["conformance", "--record"] => recordConformance zr
  | ["app", pkg, module, entry, out] =>
    buildApp zr (← IO.FS.realPath pkg) module entry (← absolute out)
    return 0
  | ["link-check", pkg] => linkCheck zr (← IO.FS.realPath pkg) (zr / ".lake/rt")
  | ["link-check", pkg, rt] => linkCheck zr (← IO.FS.realPath pkg) (← absolute rt)
  | "timed" :: pkg :: fn :: stages :: rest =>
    let (opts, qemu) := match rest.span (· != "--") with | (o, _ :: q) => (o, q) | (o, []) => (o, [])
    let keep ← match opts with
      | ["--keep", d] => pure (some (← absolute d))
      | [] => pure none
      | _ => throw <| IO.userError "zr-build timed PKG FN STAGES [--keep DIR] [-- QEMU_ARGS...]"
    timed zr (← IO.FS.realPath pkg) fn stages keep qemu.toArray
  | ["transcript", pkg] => transcript zr (← IO.FS.realPath pkg) "300"
  | ["transcript", pkg, timeout] => transcript zr (← IO.FS.realPath pkg) timeout
  | _ =>
    IO.eprintln "usage: zr-build rt [OUT] | app PKG MODULE ENTRY OUT | link-check PKG [RT] | transcript PKG [TIMEOUT] | timed PKG FN STAGES [--keep DIR] [-- QEMU_ARGS] | conformance [--record] | profile PKG FN STAGE [TOP] [--callers F,G] [--focus F] [--keep DIR]"
    return 2
