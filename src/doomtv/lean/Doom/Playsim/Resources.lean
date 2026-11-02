import Doom.Playsim.NameMap
import Doom.Playsim.GameState
import Doom.Playsim.Info
import Doom.Wad

/-!
# Doom.Playsim.Resources

Per-game static tables from the IWAD: game mode and version
(`D_IdentifyVersion` / `InitGameVersion`), texture names and heights,
flat numbering, `P_InitSwitchList`, `P_InitPicAnims`, and the DEHACKED subset
Chocolate Doom applies to the Freedoom IWADs (frames and BEX strings).
-/

namespace Doom.Playsim.Resources

open Doom.Playsim.GameState
open Doom.Playsim.Info
open Doom.Wad

instance : Inhabited LumpEntry := ⟨{ filepos := 0, size := 0, name := ByteArray.empty }⟩

/-- Upper-cased 8-byte name as a `String` key (NUL-trimmed). -/
def nameKey (b : ByteArray) : String := Id.run do
  let mut s := ""
  for i in [0:min 8 b.size] do
    let c := b[i]!
    if c == 0 then break
    let c := if c >= 97 && c <= 122 then c - 32 else c
    s := s.push (Char.ofNat c.toNat)
  return s

def keyOf (s : String) : String := s.toUpper

private def rdI32 (d : ByteArray) (o : Nat) : Int32 :=
  (d[o]!.toUInt32 ||| (d[o+1]!.toUInt32 <<< 8) ||| (d[o+2]!.toUInt32 <<< 16) |||
    (d[o+3]!.toUInt32 <<< 24)).toInt32
private def rdI16 (d : ByteArray) (o : Nat) : Int32 :=
  (d[o]!.toUInt32 ||| (d[o+1]!.toUInt32 <<< 8)).toUInt16.toInt16.toInt32

/-- Texture names and heights from `TEXTURE1` (+ `TEXTURE2`). -/
def loadTextures (wad : WadDirectory) : Except String (Array ByteArray × Array Int32) := do
  let mut names : Array ByteArray := #[]
  let mut heights : Array Int32 := #[]
  for lname in ["TEXTURE1", "TEXTURE2"] do
    match checkNumForName wad lname with
    | none => if lname == "TEXTURE1" then throw "R_InitTextures: TEXTURE1 missing"
    | some idx =>
      let d ← lumpData wad idx
      let n := (rdI32 d 0).toNatClampNeg
      for i in [0:n] do
        let off := (rdI32 d (4 + 4 * i)).toNatClampNeg
        let nm := (d.extract off (off + 8)).toList.map fun c =>
          if c >= 97 && c <= 122 then c - 32 else c
        names := names.push (ByteArray.mk nm.toArray)
        heights := heights.push ((rdI16 d (off + 14)) <<< 16)
  return (names, heights)

/-- Last-occurrence lump index by name (`W_CheckNumForName` semantics). -/
def lumpIndex (wad : WadDirectory) : Doom.Playsim.NameMap := Id.run do
  let mut m : Doom.Playsim.NameMap := {}
  for i in [0:wad.entries.size] do
    m := m.insert (nameKey wad.entries[i]!.name) i
  return m

/-- `alphSwitchList` (name1, name2, episode). -/
def alphSwitchList : Array (String × String × Nat) := #[
  ("SW1BRCOM","SW2BRCOM",1), ("SW1BRN1","SW2BRN1",1), ("SW1BRN2","SW2BRN2",1),
  ("SW1BRNGN","SW2BRNGN",1), ("SW1BROWN","SW2BROWN",1), ("SW1COMM","SW2COMM",1),
  ("SW1COMP","SW2COMP",1), ("SW1DIRT","SW2DIRT",1), ("SW1EXIT","SW2EXIT",1),
  ("SW1GRAY","SW2GRAY",1), ("SW1GRAY1","SW2GRAY1",1), ("SW1METAL","SW2METAL",1),
  ("SW1PIPE","SW2PIPE",1), ("SW1SLAD","SW2SLAD",1), ("SW1STARG","SW2STARG",1),
  ("SW1STON1","SW2STON1",1), ("SW1STON2","SW2STON2",1), ("SW1STONE","SW2STONE",1),
  ("SW1STRTN","SW2STRTN",1),
  ("SW1BLUE","SW2BLUE",2), ("SW1CMT","SW2CMT",2), ("SW1GARG","SW2GARG",2),
  ("SW1GSTON","SW2GSTON",2), ("SW1HOT","SW2HOT",2), ("SW1LION","SW2LION",2),
  ("SW1SATYR","SW2SATYR",2), ("SW1SKIN","SW2SKIN",2), ("SW1VINE","SW2VINE",2),
  ("SW1WOOD","SW2WOOD",2),
  ("SW1PANEL","SW2PANEL",3), ("SW1ROCK","SW2ROCK",3), ("SW1MET2","SW2MET2",3),
  ("SW1WDMET","SW2WDMET",3), ("SW1BRIK","SW2BRIK",3), ("SW1MOD1","SW2MOD1",3),
  ("SW1ZIM","SW2ZIM",3), ("SW1STON6","SW2STON6",3), ("SW1TEK","SW2TEK",3),
  ("SW1MARB","SW2MARB",3), ("SW1SKULL","SW2SKULL",3)]

/-- `animdefs[]` (istexture, endname, startname, speed). -/
def animdefs : Array (Bool × String × String × Int32) := #[
  (false,"NUKAGE3","NUKAGE1",8), (false,"FWATER4","FWATER1",8), (false,"SWATER4","SWATER1",8),
  (false,"LAVA4","LAVA1",8), (false,"BLOOD3","BLOOD1",8), (false,"RROCK08","RROCK05",8),
  (false,"SLIME04","SLIME01",8), (false,"SLIME08","SLIME05",8), (false,"SLIME12","SLIME09",8),
  (true,"BLODGR4","BLODGR1",8), (true,"SLADRIP3","SLADRIP1",8), (true,"BLODRIP4","BLODRIP1",8),
  (true,"FIREWALL","FIREWALA",8), (true,"GSTFONT3","GSTFONT1",8), (true,"FIRELAVA","FIRELAV3",8),
  (true,"FIREMAG3","FIREMAG1",8), (true,"FIREBLU2","FIREBLU1",8), (true,"ROCKRED3","ROCKRED1",8),
  (true,"BFALL4","BFALL1",8), (true,"SFALL4","SFALL1",8), (true,"WFALL4","WFALL1",8),
  (true,"DBRAIN4","DBRAIN1",8)]

/-- BEX `[STRINGS]` mnemonics the playsim uses, with their original text. -/
def bexStringTable : Array (String × String) := #[
  ("GOTARMOR","Picked up the armor."), ("GOTMEGA","Picked up the MegaArmor!"),
  ("GOTHTHBONUS","Picked up a health bonus."), ("GOTARMBONUS","Picked up an armor bonus."),
  ("GOTSTIM","Picked up a stimpack."), ("GOTMEDINEED","Picked up a medikit that you REALLY need!"),
  ("GOTMEDIKIT","Picked up a medikit."), ("GOTSUPER","Supercharge!"),
  ("GOTBLUECARD","Picked up a blue keycard."), ("GOTYELWCARD","Picked up a yellow keycard."),
  ("GOTREDCARD","Picked up a red keycard."), ("GOTBLUESKUL","Picked up a blue skull key."),
  ("GOTYELWSKUL","Picked up a yellow skull key."), ("GOTREDSKULL","Picked up a red skull key."),
  ("GOTINVUL","Invulnerability!"), ("GOTBERSERK","Berserk!"), ("GOTINVIS","Partial Invisibility"),
  ("GOTSUIT","Radiation Shielding Suit"), ("GOTMAP","Computer Area Map"),
  ("GOTVISOR","Light Amplification Visor"), ("GOTMSPHERE","MegaSphere!"),
  ("GOTCLIP","Picked up a clip."), ("GOTCLIPBOX","Picked up a box of bullets."),
  ("GOTROCKET","Picked up a rocket."), ("GOTROCKBOX","Picked up a box of rockets."),
  ("GOTCELL","Picked up an energy cell."), ("GOTCELLBOX","Picked up an energy cell pack."),
  ("GOTSHELLS","Picked up 4 shotgun shells."), ("GOTSHELLBOX","Picked up a box of shotgun shells."),
  ("GOTBACKPACK","Picked up a backpack full of ammo!"), ("GOTBFG9000","You got the BFG9000!  Oh, yes."),
  ("GOTCHAINGUN","You got the chaingun!"), ("GOTCHAINSAW","A chainsaw!  Find some meat!"),
  ("GOTLAUNCHER","You got the rocket launcher!"), ("GOTPLASMA","You got the plasma gun!"),
  ("GOTSHOTGUN","You got the shotgun!"), ("GOTSHOTGUN2","You got the super shotgun!"),
  ("PD_BLUEO","You need a blue key to activate this object"),
  ("PD_REDO","You need a red key to activate this object"),
  ("PD_YELLOWO","You need a yellow key to activate this object"),
  ("PD_BLUEK","You need a blue key to open this door"),
  ("PD_REDK","You need a red key to open this door"),
  ("PD_YELLOWK","You need a yellow key to open this door")]

/-- Parsed DEHACKED subset. -/
structure Deh where
  /-- (frame, field, value). -/
  frames : Array (Nat × String × Int32) := #[]
  /-- BEX `[STRINGS]` (mnemonic, value). -/
  bex : Array (String × String) := #[]

/-- Character-list helpers (independent of the `String` API). -/
private def isWs (c : Char) : Bool := c == ' ' || c == '\t' || c == '\r'
private def trimL (l : List Char) : List Char := l.dropWhile isWs
private def trimR (l : List Char) : List Char := (l.reverse.dropWhile isWs).reverse
private def trimC (l : List Char) : List Char := trimR (trimL l)
private def splitLines (l : List Char) : List (List Char) :=
  let rec go (acc cur : List (List Char) × List Char) : List Char → List (List Char)
    | [] => (acc.1.reverse) ++ [acc.2.reverse]
    | c :: cs => if c == '\n' then go ((acc.2.reverse :: acc.1), []) cur cs
                 else go (acc.1, c :: acc.2) cur cs
  go ([], []) ([], []) l
private def str (l : List Char) : String := String.ofList l
private def splitWords (l : List Char) : List (List Char) :=
  let rec go (acc : List (List Char)) (cur : List Char) : List Char → List (List Char)
    | [] => (if cur.isEmpty then acc else cur.reverse :: acc).reverse
    | c :: cs => if isWs c then go (if cur.isEmpty then acc else cur.reverse :: acc) [] cs
                 else go acc (c :: cur) cs
  go [] [] l
private def parseIntC (l : List Char) : Option Int :=
  match l with
  | '-' :: rest => (parseIntC rest).map (- ·)
  | [] => none
  | ds => if ds.all Char.isDigit then
      some (ds.foldl (fun n c => n * 10 + (c.toNat - '0'.toNat)) 0) else none

/-- Unescape a BEX string value (`\n`, `\\`). -/
private def unescape (l : List Char) : String := Id.run do
  let mut out : List Char := []
  let mut rest := l
  while !rest.isEmpty do
    match rest with
    | '\\' :: n :: cs => out := (if n == 'n' then '\n' else n) :: out; rest := cs
    | c :: cs => out := c :: out; rest := cs
    | [] => rest := []
  return String.ofList out.reverse

/-- Parse the DEHACKED lump text (Frame sections and BEX `[STRINGS]`). -/
def parseDeh (text : List Char) : Deh := Id.run do
  let mut deh : Deh := {}
  let mut sect : List Char := []
  let mut frame : Nat := 0
  let mut pending : Option (String × List Char) := none
  for raw in splitLines text do
    if let some (k, v) := pending then
      let l := trimC raw
      if l.getLast? == some '\\' then
        pending := some (k, v ++ l.dropLast)
      else
        deh := { deh with bex := deh.bex.push (k, unescape (v ++ l)) }
        pending := none
      continue
    let line := trimC raw
    if line.isEmpty || line.head? == some '#' then continue
    if line.head? == some '[' then
      sect := (line.takeWhile (· != ']')) ++ [']']
      continue
    let hasEq := line.contains '='
    let words := splitWords line
    if !hasEq && words.length >= 2 && words.head? == some "Frame".toList then
      sect := "Frame".toList
      frame := (parseIntC (words[1]!)).getD 0 |>.toNat
      continue
    if !hasEq then
      sect := words.head?.getD []
      continue
    let key := trimC (line.takeWhile (· != '='))
    let value := trimC ((line.dropWhile (· != '=')).drop 1)
    if sect == "Frame".toList then
      if let some v := parseIntC value then
        deh := { deh with frames := deh.frames.push (frame, str key, Int32.ofInt v) }
    else if sect == "[STRINGS]".toList then
      if value.getLast? == some '\\' then pending := some (str key, value.dropLast)
      else deh := { deh with bex := deh.bex.push (str key, unescape value) }
  return deh

/-- Apply DEH frame edits to `states`. -/
def applyFrames (states : Array State) (deh : Deh) : Array State := Id.run do
  let mut st := states
  for (f, key, v) in deh.frames do
    if f < st.size then
      st := st.modify f fun s =>
        if key == "Sprite number" then { s with sprite := v.toUInt32 }
        else if key == "Sprite subnumber" then { s with frame := v.toUInt32 }
        else if key == "Duration" then { s with tics := v }
        else if key == "Next frame" then { s with nextstate := v.toUInt32 }
        else if key == "Unknown 1" then { s with misc1 := v }
        else if key == "Unknown 2" then { s with misc2 := v }
        else s
  return st

/-- Build the per-game resources and the (DEH-patched) `states` table. -/
def buildRes (wad : WadDirectory) : Except String (Res × Array State) := do
  let lumps := lumpIndex wad
  let has (n : String) := lumps.contains n
  -- mission: first of MAP01 / E1M1 in lump order
  let mut commercial := false
  for e in wad.entries do
    let k := nameKey e.name
    if k == "MAP01" then commercial := true; break
    if k == "E1M1" then break
  let gamemode :=
    if commercial then gm_commercial
    else if has "E4M1" then gm_retail
    else if has "E3M1" then gm_registered
    else gm_shareware
  -- InitGameVersion (demo-lump version detection covers 1.9 only)
  let gameversion :=
    if gamemode == gm_retail then exe_ultimate
    else exe_doom_1_9
  let (textureNames, textureHeights) ← loadTextures wad
  let mut texIdx : Doom.Playsim.NameMap := {}
  for i in [0:textureNames.size] do
    let k := nameKey textureNames[i]!
    if !texIdx.contains k then texIdx := texIdx.insert k i
  let fstart ← match lumps.get? "F_START" with
    | some i => pure i | none => throw "missing F_START"
  let fend ← match lumps.get? "F_END" with
    | some i => pure i | none => throw "missing F_END"
  let firstflat := fstart + 1
  let mut flatNames : Array ByteArray := #[]
  for i in [firstflat:fend] do
    flatNames := flatNames.push wad.entries[i]!.name
  let flatNum (n : String) : Option Nat := (lumps.get? (keyOf n)).map (· - firstflat)
  let texNum (n : String) : Option Nat :=
    if n.startsWith "-" then some 0 else texIdx.get? (keyOf n)
  let skyflatnum := ((flatNum "F_SKY1").getD 0).toInt32
  -- P_InitSwitchList
  let ep := if gamemode == gm_registered || gamemode == gm_retail then 2
            else if gamemode == gm_commercial then 3 else 1
  let mut switchlist : Array Int32 := #[]
  for (n1, n2, e) in alphSwitchList do
    if e <= ep then
      let a ← match texNum n1 with | some t => pure t | none => throw s!"R_TextureNumForName: {n1} not found"
      let b ← match texNum n2 with | some t => pure t | none => throw s!"R_TextureNumForName: {n2} not found"
      switchlist := switchlist.push a.toInt32 |>.push b.toInt32
  -- P_InitPicAnims
  let mut picAnims : Array PicAnim := #[]
  for (istex, endname, startname, speed) in animdefs do
    let (base?, pic?) :=
      if istex then (texNum startname, texNum endname)
      else if has (keyOf startname) then (flatNum startname, flatNum endname) else (none, none)
    match base?, pic? with
    | some b, some p =>
      let numpics := p.toInt32 - b.toInt32 + 1
      if numpics < 2 then throw s!"P_InitPicAnims: bad cycle from {startname} to {endname}"
      picAnims := picAnims.push { istexture := istex, picnum := p.toInt32, basepic := b.toInt32,
                                  numpics, speed }
    | some _, none => throw s!"P_InitPicAnims: {endname} not found"
    | _, _ => pure ()
  -- DEHACKED (Freedoom / FreeDM IWADs)
  let mut states := Info.states
  let mut dehStrings : Array (String × String) := #[]
  if has "FREEDOOM" then
    if let some di := lumps.get? "DEHACKED" then
      let bytes ← lumpData wad di
      let deh := parseDeh (bytes.toList.map fun b => Char.ofNat b.toNat)
      states := applyFrames states deh
      for (k, v) in deh.bex do
        if let some (_, orig) := bexStringTable.find? (·.1 == k) then
          dehStrings := dehStrings.push (orig, v)
  let mut dehBex : Array (String × String) := #[]
  if has "FREEDOOM" then
    if let some di := lumps.get? "DEHACKED" then
      let bytes ← lumpData wad di
      dehBex := (parseDeh (bytes.toList.map fun b => Char.ofNat b.toNat)).bex
  let res : Res := {
    wad, gamemode, gameversion, textureNames, textureHeights, flatNames, firstflat
    skyflatnum, switchlist, picAnims, hasMap31 := has "MAP31", dehStrings, dehBex
    texIdx, lumpIdx := lumps }
  return (res, states)

/-- Texture number for a side texture name (`R_TextureNumForName`). -/
def textureNumForName (res : Res) (name : ByteArray) : Except String Int32 :=
  if name.size > 0 && name[0]! == 45 then .ok 0  -- '-'
  else match res.texIdx.get? (nameKey name) with
    | some i => .ok i.toInt32
    | none => .error s!"R_TextureNumForName: {nameKey name} not found"

/-- `R_FlatNumForName`. -/
def flatNumForName (res : Res) (name : ByteArray) : Except String Int32 :=
  match res.lumpIdx.get? (nameKey name) with
  | some i => .ok (i.toInt32 - res.firstflat.toInt32)
  | none => .error s!"R_FlatNumForName: {nameKey name} not found"

end Doom.Playsim.Resources
