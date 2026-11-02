/-!
# The DSL's FFmpeg assembly is FFmpeg's, byte for byte

    lean --run test/lean/zr/ffmpeg/AsmCheck.lean          (from the repository root)

`ZrAsm.Ffmpeg` writes FFmpeg 9.0.1's 32-bit ARM H.264 assembly in Lean;
`zr-asm-gen ffmpeg-arm` prints it as assembler files, and `zr-build rt`
assembles them into the app. FFmpeg's originals are no longer in the tree:
they were vendored unmodified (src/ffh264/asm/arm) for FFmpeg's C decoder
and removed with it, so this check takes them from the repository's history
(the parent of the commit that deleted them; a shallow clone fails here).

Each original and its DSL counterpart are assembled as `zr-build rt` does
(`zig cc -target arm-linux-musleabihf -mcpu=cortex_a55 -c`), and each pair
of objects must have:
- the same ELF type, machine and header flags;
- the same allocated sections, `.ARM.attributes` and `.note.GNU-stack`:
  name, type, flags, alignment, entry size, size and bytes;
- the same relocations against those sections (offset, type, symbol,
  addend);
- the same symbol tables (name, value, size, type, binding, visibility,
  section), mapping symbols (`$a`, `$d`) included;
and the DSL files must assemble without diagnostics. Debug sections (zig
cc's DWARF line info names the source file) are the only ones not compared.
-/

/-! ### Running tools -/

/-- Run a tool, failing on a nonzero exit. Without `lake env`'s library
paths: zig loads its own LLVM, not Lean's. -/
def run (cmd : String) (args : Array String) (cwd : Option System.FilePath := none) :
    IO IO.Process.Output := do
  let env := #[("DYLD_LIBRARY_PATH", none), ("LD_LIBRARY_PATH", none)]
  let o ← IO.Process.output { cmd, args, cwd, env }
  unless o.exitCode == 0 do
    throw <| IO.userError s!"{cmd} {" ".intercalate args.toList} failed:\n{o.stderr}"
  return o

/-- Assemble `src` to `obj` as `zr-build rt` does; returns the diagnostics. -/
def assemble (src obj inc : System.FilePath) : IO String := do
  let o ← run "zig" #["cc", "-target", "arm-linux-musleabihf", "-mcpu=cortex_a55", "-I", inc.toString,
    "-c", src.toString, "-o", obj.toString]
  return o.stderr.trimAscii.toString

/-! ### The parts of a 32-bit little-endian ARM relocatable object compared -/

def u8 (b : ByteArray) (o : Nat) : Nat := (b.get! o).toNat
def u16 (b : ByteArray) (o : Nat) : Nat := u8 b o + 256 * u8 b (o + 1)
def u32 (b : ByteArray) (o : Nat) : Nat := u16 b o + 65536 * u16 b (o + 2)
def i32 (b : ByteArray) (o : Nat) : Int := let v := u32 b o; if v ≥ 2 ^ 31 then v - 2 ^ 32 else v

/-- The NUL-terminated string at `o`. -/
def cstr (b : ByteArray) (o : Nat) : String := Id.run do
  let mut e := o
  while e < b.size && b.get! e != 0 do e := e + 1
  return String.fromUTF8! (b.extract o e)

def SHF_ALLOC := 2
def SHT_SYMTAB := 2
def SHT_RELA := 4
def SHT_NOBITS := 8
def SHT_REL := 9
def SHT_ARM_ATTRIBUTES := 0x70000003
def STT_SECTION := 3
def STT_FILE := 4

structure Section where
  name : String
  type : Nat
  flags : Nat
  align : Nat
  entsize : Nat
  size : Nat
  link : Nat
  info : Nat
  data : ByteArray
  deriving Inhabited

structure Sym where
  name : String
  value : Nat
  size : Nat
  type : Nat
  bind : Nat
  vis : Nat
  sect : String
  deriving Inhabited

structure Elf where
  header : Nat × Nat × Nat   -- type, machine, flags
  sections : Array Section
  symbols : Array Sym
  /-- Per target section: its relocations, as sorted "offset type symbol addend" lines. -/
  relocs : List (String × Array String)

def hex (n : Nat) : String :=
  let d := String.ofList (Nat.toDigits 16 n)
  String.ofList (List.replicate (8 - d.length) '0') ++ d

def Elf.read (path : System.FilePath) : IO Elf := do
  let b ← IO.FS.readBinFile path
  unless b.size > 52 && u8 b 0 == 0x7f && u8 b 1 == 0x45 && u8 b 4 == 1 && u8 b 5 == 1 do
    throw <| IO.userError s!"{path}: not a 32-bit little-endian ELF file"
  let shoff := u32 b 32
  let shentsize := u16 b 46
  let shnum := u16 b 48
  let raw := (List.range shnum).toArray.map fun i => (List.range 10).toArray.map (u32 b <| shoff + i * shentsize + 4 * ·)
  let strtab := raw[u16 b 50]!
  let sections := raw.map fun h =>
    { name := cstr b (strtab[4]! + h[0]!), type := h[1]!, flags := h[2]!, size := h[5]!, link := h[6]!,
      info := h[7]!, align := h[8]!, entsize := h[9]!,
      data := if h[1]! == SHT_NOBITS then .empty else b.extract h[4]! (h[4]! + h[5]!) : Section }
  let secName (shndx : Nat) : String :=
    if shndx == 0 then "UND" else if shndx == 0xfff1 then "ABS" else sections[shndx]!.name
  let mut symbols : Array Sym := #[]
  for s in sections do
    if s.type != SHT_SYMTAB then continue
    let names := sections[s.link]!.data
    for j in [0:s.size / 16] do
      let o := j * 16
      let info := u8 s.data (o + 12)
      let shndx := u16 s.data (o + 14)
      let type := info % 16
      symbols := symbols.push
        { name := if type == STT_SECTION then "section:" ++ secName shndx else cstr names (u32 s.data o)
          value := u32 s.data (o + 4), size := u32 s.data (o + 8), type, bind := info / 16,
          vis := u8 s.data (o + 13) % 4, sect := secName shndx }
  let mut relocs := []
  for s in sections do
    if s.type != SHT_REL && s.type != SHT_RELA then continue
    let ent := if s.type == SHT_REL then 8 else 12
    let lines := (List.range (s.size / ent)).toArray.map fun j =>
      let o := j * ent
      let info := u32 s.data (o + 4)
      let sym := if info / 256 == 0 then "-" else symbols[info / 256]!.name
      let addend := if ent == 12 then toString (i32 s.data (o + 8)) else "-"
      s!"{hex (u32 s.data o)} {info % 256} {sym} {addend}"
    relocs := (sections[s.info]!.name, lines.qsort (· < ·)) :: relocs
  return { header := (u16 b 16, u16 b 18, u32 b 36), sections, symbols, relocs }

/-- The sections compared: allocated ones, the attributes, the stack note. -/
def Elf.compared (e : Elf) : Array Section :=
  (e.sections.filter fun s => s.flags &&& SHF_ALLOC != 0 || s.type == SHT_ARM_ATTRIBUTES ||
    s.name == ".note.GNU-stack").qsort (·.name < ·.name)

def Elf.relocsOf (e : Elf) (sect : String) : Array String :=
  ((e.relocs.find? (·.1 == sect)).map (·.2)).getD #[]

/-- The symbol table as sorted lines, without file symbols and unnamed ones,
and only symbols in compared sections or undefined/absolute. -/
def Elf.symtab (e : Elf) : Array String :=
  let keep := e.compared.map (·.name)
  ((e.symbols.filter fun s => s.type != STT_FILE && s.name != "" &&
      (keep.contains s.sect || s.sect == "UND" || s.sect == "ABS")).map fun s =>
    s!"{s.name} {s.value} {s.size} {s.type} {s.bind} {s.vis} {s.sect}").qsort (· < ·)

/-- The first index where `a` and `b` differ. -/
def firstDiff (a b : ByteArray) : Nat := Id.run do
  for i in [0:min a.size b.size] do
    if a.get! i != b.get! i then return i
  return min a.size b.size

/-- Differences between original `a` and DSL `b` (empty: identical), and a summary. -/
def compare (a b : Elf) : List String × String := Id.run do
  let mut bad := []
  if a.header != b.header then bad := s!"ELF header differs: {a.header} vs {b.header}" :: bad
  let (sa, sb) := (a.compared, b.compared)
  if sa.map (·.name) != sb.map (·.name) then
    bad := s!"sections differ: {sa.map (·.name)} vs {sb.map (·.name)}" :: bad
  for x in sa do
    let some y := sb.find? (·.name == x.name) | continue
    if (x.type, x.flags, x.align, x.entsize, x.size) != (y.type, y.flags, y.align, y.entsize, y.size) then
      bad := s!"{x.name}: type/flags/align/entsize/size {(x.type, x.flags, x.align, x.entsize, x.size)} vs {(y.type, y.flags, y.align, y.entsize, y.size)}" :: bad
    if x.data.data != y.data.data then
      bad := s!"{x.name}: bytes differ from offset 0x{hex (firstDiff x.data y.data)}" :: bad
    let (ra, rb) := (a.relocsOf x.name, b.relocsOf x.name)
    if ra != rb then
      let only := (ra.filter (!rb.contains ·)).toList.take 3 ++ (rb.filter (!ra.contains ·)).toList.take 3
      bad := s!"{x.name}: relocations differ ({ra.size} vs {rb.size}), e.g. {only}" :: bad
  let (ta, tb) := (a.symtab, b.symtab)
  if ta != tb then
    bad := s!"symbol tables differ: only original {(ta.filter (!tb.contains ·)).toList.take 4}, only DSL {(tb.filter (!ta.contains ·)).toList.take 4}" :: bad
  let funcs := (a.symbols.filter fun s => s.type == 2 && s.bind == 1).size
  let bytes := (sa.filter (·.flags &&& SHF_ALLOC != 0)).foldl (fun n s => n + s.size) 0
  let nrel := sa.foldl (fun n s => n + (a.relocsOf s.name).size) 0
  return (bad.reverse, s!"{funcs} global functions, {ta.size} symbols, {bytes} bytes, {nrel} relocations")

/-! ### The check -/

def main : IO UInt32 := do
  let root := (← run "git" #["rev-parse", "--show-toplevel"]).stdout.trimAscii.toString
  let orig := "src/ffh264/asm/arm"
  let gone := (← run "git" #["log", "-1", "--format=%H", "--diff-filter=D", "--", orig ++ "/SOURCES"] root).stdout.trimAscii.toString
  if gone.isEmpty then
    IO.eprintln s!"no commit deletes {orig} in this history"; return 1
  let tmp ← IO.FS.createTempDir
  try
    -- FFmpeg's originals as they were before their removal
    discard <| run "sh" #["-c", s!"git archive {gone}~1 {orig} | tar -x -C '{tmp}'"] root
    let origDir : System.FilePath := tmp / orig
    -- the DSL's files, as `zr-build rt` generates them
    let tools : System.FilePath := root / "tools/lean/zr"
    discard <| run "lake" #["build", "zr-asm-gen"] tools
    let dslDir : System.FilePath := tmp / "dsl"
    discard <| run (tools / ".lake/build/bin/zr-asm-gen").toString #["ffmpeg-arm", dslDir.toString]
    let words (s : String) := ((s.splitOn "\n").map (·.trimAscii.toString)).filter (· != "")
    let os := words (← IO.FS.readFile (origDir / "SOURCES"))
    let ds := words (← IO.FS.readFile (dslDir / "SOURCES"))
    let mut failed := false
    if os.map (fun p => (p.splitOn "/").getLast!) != ds then
      IO.println s!"SOURCES differ: {os} vs {ds}"; failed := true
    IO.println s!"FFmpeg's originals: {orig} at {String.ofList (gone.toList.take 10)}~1"
    let mut funcs := 0
    for (o, d) in os.zip ds do
      let stem := d.replace ".S" ""
      let ao := tmp / s!"{stem}.orig.o"
      let bo := tmp / s!"{stem}.dsl.o"
      discard <| assemble (origDir / o) ao origDir
      let diag ← assemble (dslDir / d) bo dslDir
      let a ← Elf.read ao
      let (bad, summary) := compare a (← Elf.read bo)
      let bad := if diag.isEmpty then bad else bad ++ [s!"assembler diagnostics for the DSL file:\n{diag}"]
      IO.println s!"  {if bad.isEmpty then "OK  " else "FAIL"} {stem}: {summary}"
      for m in bad.take 12 do IO.println s!"         {m}"
      failed := failed || !bad.isEmpty
      funcs := funcs + (a.symbols.filter fun s => s.type == 2 && s.bind == 1).size
    IO.println s!"ffmpeg-asm: {ds.length} files, {funcs} global functions: {if failed then "FAIL" else "identical objects"}"
    return if failed then 1 else 0
  finally
    IO.FS.removeDirAll tmp
