import Doom.Front.Strings
import Doom.Wad

/-!
# Doom.Front.Deh

BEX `[STRINGS]` replacements from an IWAD's `DEHACKED` lump, as Chocolate Doom
loads them for Freedoom and FreeDM (`d_main.c` `LoadIwadDeh`,
`deh_main.c` `DEH_ParseContext`, `deh_io.c` `DEH_ReadLine`,
`deh_bexstr.c`). Only the string section is interpreted: level names, cast
names, finale texts and flats, automap messages. Other sections are skipped
with the same section/blank-line structure so their lines are never taken for
string assignments.

`DEH_String(original)` becomes `Strings.get mnemonic original`: the lookup is
by BEX mnemonic instead of by original text, which is equivalent for the
strings used here (their originals are distinct).
-/

namespace Doom.Front.Deh

open Doom.Wad

/-- Mnemonic → replacement, in file order (the last assignment wins). -/
structure Strings where
  entries : Array (String × String) := #[]
  deriving Inhabited, Repr

/-- `DEH_String`: the replacement for `key`, or `default`. -/
def Strings.get (s : Strings) (key default : String) : String := Id.run do
  let mut r := default
  for (k, v) in s.entries do
    if k == key then r := v
  return r

/-- `DEH_String` of a front-end string by mnemonic, with its English default. -/
def Strings.str (s : Strings) (key : String) : String :=
  let tables := Doom.Front.Strings.mapnamesDoom ++ Doom.Front.Strings.mapnamesCommercial ++
    Doom.Front.Strings.castNames ++ Doom.Front.Strings.finaleTexts ++ Doom.Front.Strings.automapStrings
  s.get key ((tables.find? (·.1 == key)).map (·.2) |>.getD "")

/-- The automap level title (`HU_Start`'s `w_title`): `HU_TITLE` for Doom,
`HU_TITLE2` for Doom II (empty past MAP32, as in Doom 1.9). -/
def Strings.levelTitle (s : Strings) (commercial : Bool) (episode map : Int32) : String :=
  if commercial then
    if map >= 1 && map <= 32 then s.str s!"HUSTR_{map}" else ""
  else
    match Doom.Front.Strings.mapnamesDoom[((episode - 1) * 9 + map - 1).toNatClampNeg]? with
    | some (k, d) => if episode >= 1 && map >= 1 then s.get k d else ""
    | none => ""

/-- Every string the front end may look up (`deh_bexstr.c` table subset). -/
private def known : Array String :=
  (Strings.mapnamesDoom ++ Strings.mapnamesCommercial ++ Strings.castNames ++
    Strings.finaleTexts ++ Strings.automapStrings).map (·.1) ++
  #["BGFLATE1", "BGFLATE2", "BGFLATE3", "BGFLATE4", "BGFLAT06", "BGFLAT11",
    "BGFLAT20", "BGFLAT30", "BGFLAT15", "BGFLAT31"]

private def isSpace (c : UInt8) : Bool :=
  c == 32 || c == 9 || c == 10 || c == 11 || c == 12 || c == 13

/-- Byte cursor over the lump with `DEH_GetChar`'s CRLF folding. -/
private structure Reader where
  data : ByteArray
  pos : Nat := 0

/-- `DEH_GetChar`: next byte, `none` at the end; CR LF reads as LF. -/
private def Reader.getChar (r : Reader) : Option UInt8 × Reader :=
  if h : r.pos < r.data.size then
    let c := r.data[r.pos]
    if c == 13 && r.data[r.pos + 1]? == some 10 then (some 10, { r with pos := r.pos + 2 })
    else (some c, { r with pos := r.pos + 1 })
  else (none, r)

/-- `DEH_ReadLine(context, extended)`. `none` at end of file. -/
private def Reader.readLine (r0 : Reader) (extended : Bool) : Option String × Reader := Id.run do
  let mut r := r0
  let mut buf : ByteArray := ByteArray.empty
  let mut escaped := false
  let mut fuel := r0.data.size + 1
  while fuel > 0 do
    fuel := fuel - 1
    let (c0, r1) := r.getChar
    r := r1
    if c0.isNone && buf.size == 0 then return (none, r)
    let mut c := c0
    if extended && c == some 92 then  -- '\\'
      let (c1, r2) := r.getChar
      r := r2
      c := c1
      if c == some 110 then  -- "\n" escape: internal linefeed
        buf := buf.push 10
        continue
      if c == some 10 then  -- backslash at end of line: continuation
        escaped := true
        continue
    match c with
    | some ch =>
      if escaped && isSpace ch && ch != 10 then continue
      escaped := false
      if ch == 10 then break
      if ch != 0 then buf := buf.push ch
    | none => break
  return (some (String.fromUTF8? buf |>.getD (String.ofList (buf.toList.map (Char.ofNat ·.toNat)))), r)

/-- `CleanString`: trim leading and trailing whitespace. -/
private def clean (s : String) : String := Id.run do
  let b := s.toUTF8
  let mut i := 0
  while i < b.size && isSpace b[i]! do i := i + 1
  let mut j := b.size
  while j > i && isSpace b[j - 1]! do j := j - 1
  return String.fromUTF8? (b.extract i j) |>.getD s

private def sectionNames : Array String :=
  #["ammo", "cheat", "frame", "misc", "pointer", "sound", "text", "thing", "weapon", "[strings]"]

private def firstToken (s : String) : String :=
  ((s.splitOn " ").filter (· ≠ "")).headD "" |>.splitOn "\t" |>.headD ""

/-- Parse a DEHACKED patch (`DEH_ParseContext`), keeping `[STRINGS]`
assignments to known mnemonics. -/
def parse (data : ByteArray) : Strings := Id.run do
  let mut r : Reader := { data }
  let (first, r1) := r.readLine false
  r := r1
  match first with
  | some l => if l != "Patch File for DeHackEd v2.3" && l != "Patch File for DeHackEd v3.0" then return {}
  | none => return {}
  let mut out : Strings := {}
  let mut sec : Option String := none
  let mut allowStrings := false
  let mut fuel := data.size + 1
  while fuel > 0 do
    fuel := fuel - 1
    let extended := sec == some "[strings]"
    let (line?, r2) := r.readLine extended
    r := r2
    let some line0 := line? | break
    let line := clean line0
    if line.startsWith "#" then
      if (line.splitOn "*allow-extended-strings*").length > 1 then allowStrings := true
      continue
    if line.isEmpty then
      sec := none
      continue
    match sec with
    | some "[strings]" =>
      match line.splitOn "=" with
      | name :: rest =>
        if rest.isEmpty then continue
        let key := clean name
        let value := clean ("=".intercalate rest)
        if known.contains key then out := { entries := out.entries.push (key, value) }
      | [] => pure ()
    | some _ => pure ()
    | none =>
      let tok := (firstToken line).toLower
      if tok == "[strings]" && !allowStrings then continue
      if sectionNames.contains tok then
        sec := some tok
        if tok == "text" then
          -- `Text <from> <to>`: the section reads that many raw characters.
          let nums := ((line.splitOn " ").filter (· ≠ "")).drop 1 |>.filterMap String.toNat?
          let n := nums.foldl (· + ·) 0
          for _ in [0:n] do
            r := r.getChar.2
  return out

/-- The IWAD's string replacements: Chocolate Doom loads `DEHACKED` only for
the Freedoom and FreeDM variants (a `FREEDOOM` lump). -/
def load (wad : WadDirectory) : Strings :=
  if (checkNumForName wad "FREEDOOM").isNone then {}
  else match checkNumForName wad "DEHACKED" with
    | some i => match lumpData wad i with
      | .ok d => parse d
      | .error _ => {}
    | none => {}

end Doom.Front.Deh
