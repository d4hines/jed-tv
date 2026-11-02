/-!
Where the Lean app's bytes come from: the allocated (non-bss) input sections
of the final link, grouped by origin, from the map `zr-build app` writes with
YT_MAP=FILE.

    YT_MAP=/tmp/app.map lake -d tools/lean/zr exe zr-build app src/lean/yt Yt.App.Main yt_app_main OUT
    lake env lean --run tools/lean/yt/Composition.lean /tmp/app.map

Groups: Lean (the app's and the runtime's compiled modules), the DSL (our
assembly, including FFmpeg's NEON kernels as written in it) and the linker's
own sections. Anything else is reported as `other` with its file.
-/

def contains (s sub : String) : Bool := (s.splitOn sub).length > 1

/-- The whitespace-separated fields of `s`. -/
def fields (s : String) : List String :=
  let (acc, cur) := s.toList.foldl (fun (acc, cur) c =>
    if c.isWhitespace then (if cur.isEmpty then (acc, cur) else (acc ++ [String.ofList cur.reverse], []))
    else (acc, c :: cur)) (([] : List String), ([] : List Char))
  if cur.isEmpty then acc else acc ++ [String.ofList cur.reverse]

def hexVal (s : String) : Option Nat :=
  if s.isEmpty then none
  else s.toList.foldl (fun acc c => acc.bind fun a =>
    if c.isDigit then some (16 * a + (c.toNat - 48))
    else if 'a' ≤ c.toLower && c.toLower ≤ 'f' then some (16 * a + (c.toLower.toNat - 87)) else none) (some 0)

def group (base : String) : String :=
  if (base.startsWith "yt" && base.endsWith ".o" && (String.ofList ((base.toList.drop 2).take (base.length - 4))).all Char.isDigit &&
      base.length > 4) || base.startsWith "ZrLean_" || base.startsWith "ZrTv_" || base.startsWith "ZrAbi_" then "Lean"
  else if base == "main.o" || base == "runtime.o" || base == "tv_asm.o" then "DSL (ours)"
  -- FFmpeg's NEON kernels as written in the DSL (ZrAsm.Ffmpeg, zr-build rt's OUT/obj/h264)
  else if contains base "_neon.o" then "DSL (FFmpeg NEON)"
  else if base == "<internal>" then "linker (exidx, got)"
  else "other " ++ base

def pad (s : String) (n : Nat) (right : Bool) : String :=
  let sp := String.ofList (List.replicate (n - s.length) ' ')
  if right then sp ++ s else s ++ sp

def main (args : List String) : IO UInt32 := do
  let [mapPath] := args | IO.eprintln "usage: Composition MAP"; return 2
  let mut sizes : Array (String × Nat) := #[]
  for line in (← IO.FS.readFile mapPath).splitOn "\n" do
    let f := fields line
    if f.length < 5 || !contains f[4]! ":(" then continue
    let some size := hexVal f[2]! | continue
    let src := (f[4]!.splitOn ":(").headD ""
    let sect0 := ":(".intercalate ((f[4]!.splitOn ":(").drop 1)
    let sect := String.ofList (sect0.toList.reverse.dropWhile (· == ')')).reverse
    if sect.startsWith ".bss" || sect.startsWith ".tbss" then continue
    -- an archive member counts as its archive
    let arch := if src.endsWith ")" && contains src "(" then "(".intercalate ((src.splitOn "(").dropLast) else src
    let g := group (arch.splitOn "/").getLast!
    sizes := match sizes.findIdx? (·.1 == g) with
      | some i => sizes.modify i fun (k, v) => (k, v + size)
      | none => sizes.push (g, size)
  let total := sizes.foldl (fun a p => a + p.2) 0
  let sorted := sizes.qsort (fun a b => a.2 > b.2)
  for (g, s) in sorted do
    let pct := (1000 * s + total / 2) / total
    IO.println s!"{pad g 26 false} {pad (toString s) 9 true} {pad s!"{pct / 10}.{pct % 10}" 5 true}%"
  IO.println s!"{pad "total" 26 false} {pad (toString total) 9 true}"
  return 0
