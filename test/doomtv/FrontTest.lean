import Doom.Front.Automap
import Doom.Front.Deh
import Doom.Front.Finale
import Doom.Front.Intermission
import Doom.Front.Screens
import Doom.Game
import Doom.Wad
import Doom.Render.Frame
import Doom.Render.Gfx.Flat
import Doom.Render.Gfx.Patch
/-!
# front-test

Host checks for the front-end screens (`Doom.Front.*`): renders the
intermission, finale and automap for several states with the Freedoom IWADs
(and the shareware IWAD from the upstream fixtures when present, for its
animated intermission maps), writes PPM and 2x PNG images, and checks them:
non-blank frames, expected patches at the positions `wi_stuff.c` /
`f_finale.c` compute (pixel-exact against an independent patch decoder),
digits for known statistics, plus state-machine checks (cast call, text
crawl timing, automap zoom bounds, follow, marks, cheat).

Usage: `front-test [OUT_DIR]` from `src/doomtv/lean`; the default output is
`../../../assets/build/lean-doom-front`.
-/

open Doom.Front
open Doom.Playsim.GameState
open Doom.Playsim.Player
open Doom.Render.Frame
open Doom.Render.Types
open Doom.Wad

/-! ## Images -/

private def crcTable : Array UInt32 := Id.run do
  let mut t := Array.emptyWithCapacity 256
  for n in [0:256] do
    let mut c := n.toUInt32
    for _ in [0:8] do
      c := if c &&& 1 != 0 then (0xedb88320 : UInt32) ^^^ (c >>> 1) else c >>> 1
    t := t.push c
  return t

private def crc32 (b : ByteArray) : UInt32 := Id.run do
  let mut c : UInt32 := 0xffffffff
  for x in b do
    c := crcTable[((c ^^^ x.toUInt32) &&& 0xff).toNat]! ^^^ (c >>> 8)
  return c ^^^ 0xffffffff

private def be32 (v : UInt32) : ByteArray :=
  ⟨#[(v >>> 24).toUInt8, (v >>> 16).toUInt8, (v >>> 8).toUInt8, v.toUInt8]⟩

private def chunk (ty : String) (data : ByteArray) : ByteArray :=
  let body := ty.toUTF8 ++ data
  be32 data.size.toUInt32 ++ body ++ be32 (crc32 body)

/-- zlib stream of stored (uncompressed) deflate blocks. -/
private def zlibStored (raw : ByteArray) : ByteArray := Id.run do
  let mut out : ByteArray := ⟨#[0x78, 0x01]⟩
  let mut i := 0
  while i < raw.size || raw.size == 0 do
    let n := min 65535 (raw.size - i)
    let final : UInt8 := if i + n >= raw.size then 1 else 0
    out := out.push final
    out := out.push n.toUInt8 |>.push (n >>> 8).toUInt8
    out := out.push (~~~n.toUInt16).toUInt8 |>.push ((~~~n.toUInt16) >>> 8).toUInt8
    out := out ++ raw.extract i (i + n)
    i := i + n
    if raw.size == 0 then break
  let mut a : UInt32 := 1
  let mut b : UInt32 := 0
  for x in raw do
    a := (a + x.toUInt32) % 65521
    b := (b + a) % 65521
  return out ++ be32 ((b <<< 16) ||| a)

/-- RGB PNG of the frame, scaled by `k`. -/
def pngOf (fb : Framebuffer) (pal : ByteArray) (k : Nat := 2) : ByteArray := Id.run do
  let w := 320 * k
  let h := 200 * k
  let mut raw := ByteArray.emptyWithCapacity (h * (w * 3 + 1))
  for y in [0:h] do
    raw := raw.push 0
    for x in [0:w] do
      let c := (fb.pixels[(y / k) * 320 + x / k]?.getD 0).toNat
      raw := raw.push (pal[c * 3]?.getD 0) |>.push (pal[c * 3 + 1]?.getD 0) |>.push (pal[c * 3 + 2]?.getD 0)
  let ihdr := be32 w.toUInt32 ++ be32 h.toUInt32 ++ ⟨#[8, 2, 0, 0, 0]⟩
  return ⟨#[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]⟩ ++ chunk "IHDR" ihdr ++
    chunk "IDAT" (zlibStored raw) ++ chunk "IEND" ByteArray.empty

/-! ## Check bookkeeping -/

structure Ctx where
  out : System.FilePath
  failures : IO.Ref (Array String)
  passes : IO.Ref Nat
  images : IO.Ref (Array String)

def check (c : Ctx) (name : String) (ok : Bool) (detail : String := "") : IO Unit := do
  if ok then c.passes.modify (· + 1)
  else
    c.failures.modify (·.push s!"{name} {detail}")
    IO.eprintln s!"front-test FAIL: {name} {detail}"

def save (c : Ctx) (name : String) (fb : Framebuffer) (pal : ByteArray) : IO Unit := do
  Doom.Render.Dump.writePpm (c.out / s!"{name}.ppm") fb pal
  IO.FS.writeBinFile (c.out / s!"{name}.png") (pngOf fb pal)
  c.images.modify (·.push s!"{name}.png")

def orFail {α} (c : Ctx) (what : String) (r : Except String α) : IO (Option α) := do
  match r with
  | .ok v => pure (some v)
  | .error e => check c what false e; pure none

/-- Distinct colours in the frame. -/
def colours (fb : Framebuffer) : Nat := Id.run do
  let mut seen := Array.replicate 256 false
  for b in fb.pixels do seen := seen.set! b.toNat true
  return seen.foldl (fun n b => if b then n + 1 else n) 0

def countColour (fb : Framebuffer) (c : UInt8) (y0 y1 : Nat := 0) (y1' : Nat := 200) : Nat := Id.run do
  let _ := y1
  let mut n := 0
  for y in [y0:y1'] do
    for x in [0:320] do
      if fb.pixels[y * 320 + x]! == c then n := n + 1
  return n

/-- Every opaque pixel of patch `name` drawn at (x, y) (offsets applied)
is in `fb`, decoded independently of the drawing code. -/
def patchAt (wad : WadDirectory) (fb : Framebuffer) (name : String) (x y : Int32) : Bool := Id.run do
  let some idx := checkNumForName wad name | return false
  let .ok (hdr, data) := Doom.Render.Gfx.Patch.loadPatch wad idx | return false
  let x0 := x - hdr.leftOffset.toInt16.toInt32
  let y0 := y - hdr.topOffset.toInt16.toInt32
  let mut nOpaque := 0
  for col in [0:hdr.width.toNat] do
    let .ok off := Doom.Render.Gfx.Patch.columnOffset data hdr col | return false
    let .ok pixels := Doom.Render.Gfx.Patch.decodeColumnPixels data off hdr.height.toNat | return false
    for (row, pix) in pixels do
      let sx := x0 + Int32.ofNat col
      let sy := y0 + Int32.ofNat row
      if sx >= 0 && sx < 320 && sy >= 0 && sy < 200 then
        nOpaque := nOpaque + 1
        if fb.pixels[sy.toNatClampNeg * 320 + sx.toNatClampNeg]! != pix then return false
  return nOpaque > 0

/-- Fraction (in percent) of patch `name`'s on-screen opaque pixels found at
(x, y): for patches that later drawing may partly cover. -/
def patchPct (wad : WadDirectory) (fb : Framebuffer) (name : String) (x y : Int32) : Nat := Id.run do
  let some idx := checkNumForName wad name | return 0
  let .ok (hdr, data) := Doom.Render.Gfx.Patch.loadPatch wad idx | return 0
  let x0 := x - hdr.leftOffset.toInt16.toInt32
  let y0 := y - hdr.topOffset.toInt16.toInt32
  let mut total := 0
  let mut hit := 0
  for col in [0:hdr.width.toNat] do
    let .ok off := Doom.Render.Gfx.Patch.columnOffset data hdr col | return 0
    let .ok pixels := Doom.Render.Gfx.Patch.decodeColumnPixels data off hdr.height.toNat | return 0
    for (row, pix) in pixels do
      let sx := x0 + Int32.ofNat col
      let sy := y0 + Int32.ofNat row
      if sx >= 0 && sx < 320 && sy >= 0 && sy < 200 then
        total := total + 1
        if fb.pixels[sy.toNatClampNeg * 320 + sx.toNatClampNeg]! == pix then hit := hit + 1
  return if total == 0 then 0 else hit * 100 / total

def patchW (wad : WadDirectory) (name : String) : Int32 :=
  ((checkNumForName wad name).map (fun i => (Draw.patchInfo wad i).width)).getD 0

def patchH (wad : WadDirectory) (name : String) : Int32 :=
  ((checkNumForName wad name).map (fun i => (Draw.patchInfo wad i).height)).getD 0

/-! ## Game states -/

def mapLabel (commercial : Bool) (episode map : Nat) : String :=
  if commercial then (if map < 10 then s!"MAP0{map}" else s!"MAP{map}") else s!"E{episode}M{map}"

def startLevel (wad : WadDirectory) (_commercial : Bool) (episode map : Nat) : Except String GameState :=
  Doom.Game.newGame wad { skill := 2, episode := Int32.ofNat episode, map := Int32.ofNat map }

/-- One `G_Ticker` with player 0 pressing `buttons`. -/
def tic (gs : GameState) (buttons : UInt32) : Except String GameState :=
  let gs := Doom.Game.ticker gs #[{ TicCmd.zero with buttons }, TicCmd.zero, TicCmd.zero, TicCmd.zero]
  match gs.error with
  | some e => .error e
  | none => .ok gs

def withCmd (gs : GameState) (buttons : UInt32) : GameState :=
  match gs.players[0]? with
  | some p => { gs with players := gs.players.set! 0 { p with cmd := { p.cmd with buttons } } }
  | none => gs

/-! ## Intermission -/

def wiChecks (c : Ctx) (wad : WadDirectory) (data : Doom.Render.Data.RenderData) (tag : String)
    (commercial retail : Bool) : IO Unit := do
  let pal := (Doom.Render.Palette.playpalSlice data.playpal 0).toOption.getD ByteArray.empty
  let num0W := patchW wad "WINUM0"
  let num0H := patchH wad "WINUM0"
  let lh := (3 * num0H) / 2
  let colonW := patchW wad "WICOLON"
  -- Single-player stats: 50% kills, 75% items, 100% secrets, 1:35 vs par 0:30.
  let epsds : List Nat := if commercial then [0] else if retail then [0, 1, 2, 3] else [0, 1, 2]
  for e in epsds do
    let v : Intermission.View := {
      stage := .statCount, commercial, retail, epsd := e, last := 0, next := 1,
      cntKills := 50, cntItems := 75, cntSecret := 100, cntTime := 95, cntPar := 30 }
    let some fb ← orFail c s!"{tag} WI stats e{e}" (Intermission.draw wad Framebuffer.initBlack v) | continue
    save c s!"{tag}-wi-stats-e{e + 1}" fb pal
    let bg := Intermission.background v
    check c s!"{tag} WI stats e{e} background {bg}" (bg == (if commercial || (retail && e == 3) then "INTERPIC" else s!"WIMAP{e}"))
    check c s!"{tag} WI stats e{e} colourful" (colours fb > 16)
    let lname := if commercial then "CWILV00" else s!"WILV{e}0"
    check c s!"{tag} WI stats e{e} level name {lname}"
      (patchAt wad fb lname ((320 - patchW wad lname) / 2) 2)
    check c s!"{tag} WI stats e{e} finished" (patchAt wad fb "WIF" ((320 - patchW wad "WIF") / 2) (2 + (5 * patchH wad lname) / 4))
    check c s!"{tag} WI stats e{e} kills title" (patchAt wad fb "WIOSTK" 50 50)
    check c s!"{tag} WI stats e{e} 50% kills" (patchAt wad fb "WIPCNT" 270 50 &&
      patchAt wad fb "WINUM0" (270 - num0W) 50 && patchAt wad fb "WINUM5" (270 - 2 * num0W) 50)
    check c s!"{tag} WI stats e{e} 75% items" (patchAt wad fb "WINUM5" (270 - num0W) (50 + lh) &&
      patchAt wad fb "WINUM7" (270 - 2 * num0W) (50 + lh))
    check c s!"{tag} WI stats e{e} 100% secret" (patchAt wad fb "WISCRT2" 50 (50 + 2 * lh) &&
      patchAt wad fb "WINUM1" (270 - 3 * num0W) (50 + 2 * lh))
    -- 01:35 right-aligned at x = 144.
    let x1 := 144 - 2 * num0W - colonW
    check c s!"{tag} WI stats e{e} time 01:35" (patchAt wad fb "WITIME" 16 168 &&
      patchAt wad fb "WINUM5" (144 - num0W) 168 && patchAt wad fb "WINUM3" (144 - 2 * num0W) 168 &&
      patchAt wad fb "WICOLON" x1 168 && patchAt wad fb "WINUM1" (x1 - num0W) 168 &&
      patchAt wad fb "WINUM0" (x1 - 2 * num0W) 168)
    let parShown := patchAt wad fb "WIPAR" 176 168
    check c s!"{tag} WI stats e{e} par shown iff episode < 4" (parShown == (e < 3))
    if e < 3 then
      check c s!"{tag} WI stats e{e} par 00:30" (patchAt wad fb "WINUM0" (304 - num0W) 168 &&
        patchAt wad fb "WINUM3" (304 - 2 * num0W) 168)
  -- Counting has not reached secrets yet: -1 hides the percentage.
  let v : Intermission.View := {
    commercial, retail, cntKills := 12, cntItems := -1, cntSecret := -1, cntTime := -1, cntPar := -1 }
  if let some fb ← orFail c s!"{tag} WI partial" (Intermission.draw wad Framebuffer.initBlack v) then
    save c s!"{tag}-wi-counting" fb pal
    check c s!"{tag} WI partial: 12% kills, no items yet" (patchAt wad fb "WINUM2" (270 - num0W) 50 &&
      !patchAt wad fb "WIPCNT" 270 (50 + lh))
  -- Over 61:59 the time reads "sucks".
  let v : Intermission.View := {
    commercial, retail, cntKills := 0, cntItems := 0, cntSecret := 0, cntTime := 61 * 60, cntPar := 30 }
  if let some fb ← orFail c s!"{tag} WI sucks" (Intermission.draw wad Framebuffer.initBlack v) then
    save c s!"{tag}-wi-sucks" fb pal
    check c s!"{tag} WI sucks" (patchAt wad fb "WISUCKS" (144 - patchW wad "WISUCKS") 168)
  -- Entering the next level, with splats and the pointer on Doom 1 maps.
  for (e, last, next, ptr, secret) in (if commercial then [(0, 4, 5, true, false)]
      else [(0, 2, 3, true, false), (0, 8, 3, false, true), (1, 4, 5, true, false), (2, 0, 1, true, false)]) do
    let v : Intermission.View := {
      stage := .showNextLoc, commercial, retail, epsd := e, last, next, pointerOn := ptr, didsecret := secret }
    let some fb ← orFail c s!"{tag} WI next e{e}" (Intermission.draw wad Framebuffer.initBlack v) | continue
    save c s!"{tag}-wi-entering-e{e + 1}-m{next + 1}{if secret then "-secret" else ""}{if ptr then "" else "-blink"}" fb pal
    let lname := if commercial then s!"CWILV0{next}" else s!"WILV{e}{next}"
    check c s!"{tag} WI entering e{e}" (patchAt wad fb "WIENTER" ((320 - patchW wad "WIENTER") / 2) 2 &&
      patchAt wad fb lname ((320 - patchW wad lname) / 2) (2 + (5 * patchH wad "WIENTER") / 4))
    if !commercial && patchW wad "WISPLAT" > 1 then
      let (sx, sy) := Intermission.lnodes[e]![0]!
      check c s!"{tag} WI splat on first level e{e}" (patchPct wad fb "WISPLAT" sx sy >= 20)
      let (nx, ny) := Intermission.lnodes[e]![next]!
      check c s!"{tag} WI pointer e{e} {ptr}" (ptr == (patchAt wad fb "WIURH0" nx ny || patchAt wad fb "WIURH1" nx ny))
      if secret then
        let (qx, qy) := Intermission.lnodes[e]![8]!
        check c s!"{tag} WI secret level splat" (patchPct wad fb "WISPLAT" qx qy >= 20)
  -- Cooperative and deathmatch tallies.
  let net : Intermission.NetView := {
    playeringame := #[true, true, true, false], me := 1, dofrags := true,
    cntKills := #[40, 60, 0, 0], cntItems := #[10, 20, 30, 0], cntSecret := #[0, 100, 50, 0],
    cntFrags := #[3, -2, 0, 0],
    dmFrags := #[#[0, 5, 2, 0], #[1, 0, 7, 0], #[3, 4, 0, 0], #[0, 0, 0, 0]], dmTotals := #[7, 8, 7, 0] }
  let v : Intermission.View := { commercial, retail, netgame := true, net }
  if let some fb ← orFail c s!"{tag} WI coop" (Intermission.draw wad Framebuffer.initBlack v) then
    save c s!"{tag}-wi-coop" fb pal
    let statsx := 32 + patchW wad "STFST01" / 2
    let y := 50 + patchH wad "WIOSTK"
    check c s!"{tag} WI coop titles and players" (patchAt wad fb "WIOSTK" (statsx + 64 - patchW wad "WIOSTK") 50 &&
      patchAt wad fb "WIFRGS" (statsx + 256 - patchW wad "WIFRGS") 50 &&
      patchAt wad fb "STPB0" (statsx - patchW wad "STPB0") y &&
      patchAt wad fb "STFST01" (statsx - patchW wad "STPB1") (y + 33))
  let v : Intermission.View := { commercial, retail, netgame := true, deathmatch := true, net }
  if let some fb ← orFail c s!"{tag} WI deathmatch" (Intermission.draw wad Framebuffer.initBlack v) then
    save c s!"{tag}-wi-deathmatch" fb pal
    check c s!"{tag} WI deathmatch matrix" (patchAt wad fb "WIKILRS" 10 100 && patchAt wad fb "WIVCTMS" 5 50 &&
      patchAt wad fb "STFDEAD0" (82 + 40 - patchW wad "STPB1" / 2) 35 &&
      patchAt wad fb "WINUM5" (122 - num0W + num0W) 78)

/-- The playsim's ticker driving the drawer: complete E1M1, count, skip, enter E1M2. -/
def wiFlow (c : Ctx) (wad : WadDirectory) (data : Doom.Render.Data.RenderData) (tag : String)
    (commercial retail : Bool) : IO Unit := do
  let pal := (Doom.Render.Palette.playpalSlice data.playpal 0).toOption.getD ByteArray.empty
  let some gs ← orFail c s!"{tag} WI flow level" (startLevel wad commercial 1 1) | return
  let gs := match gs.players[0]? with
    | some p =>
      let p := { p with killcount := gs.totalkills / 2, itemcount := gs.totalitems, secretcount := 0 }
      { gs with players := gs.players.set! 0 p }
    | none => gs
  let gs := { gs with leveltime := 35 * 75, gameaction := ga_completed }
  let some gs0 ← orFail c s!"{tag} WI flow complete" (tic gs 0) | return
  check c s!"{tag} WI flow in intermission" (gs0.gamestate == gamestateIntermission)
  let mut gs := gs0
  let mut cache : Cache := {}
  let mut shots : Array Nat := #[]
  let mut sawTime := false
  for t in [1:400] do
    -- Press use at tic 200 (skip counting), again at 260 (leave the stats).
    let b : UInt32 := if t == 200 || t == 260 then BT_USE else 0
    let some g ← orFail c s!"{tag} WI flow tic {t}" (tic gs b) | return
    gs := g
    if gs.gamestate != gamestateIntermission then
      check c s!"{tag} WI flow leaves the intermission after the skip" (t > 260 && t < 260 + 4 * 35 + 12) s!"tic {t}"
      break
    if t ∈ [1, 40, 120, 199, 201, 262, 300] then
      let v := Intermission.view gs {} commercial retail
      match renderIntermission data wad cache v with
      | .ok (pal', cache') =>
        cache := cache'
        save c s!"{tag}-wi-flow-t{t}" cache.fb pal'
        shots := shots.push t
        if t == 201 && v.cntTime >= 0 && v.cntPar >= 0 then sawTime := true
      | .error e => check c s!"{tag} WI flow render t{t}" false e
  let _ := pal
  check c s!"{tag} WI flow frames" (shots.size >= 6) s!"{shots}"
  check c s!"{tag} WI flow: use skips to the full tally" sawTime

/-- Tracker for episodes the playsim does not animate. -/
def trackerChecks (c : Ctx) : IO Unit := do
  -- Episode 3 background: `WI_initAnimatedBack` draws one M_Random per anim.
  let (anims, draws) := Intermission.initAnims 2 0 { prndindex := 0, rndindex := 0 }
  check c "tracker E3 init draws 6 M_Random" (draws == 6)
  -- rndtable[1..6] = 8, 109, 220, 222, 241, 149; periods 11 x5, 8.
  let expect : List Int32 := [1 + 8 % 11, 1 + 109 % 11, 1 + 220 % 11, 1 + 222 % 11, 1 + 241 % 11, 1 + 149 % 8]
  check c "tracker E3 nexttic from M_Random" ((anims.toList.map (·.nexttic)) == expect) s!"{anims.toList.map (·.nexttic)}"
  -- Episode 2 level animations: during StatCount only the anims for `next`
  -- (except 7) start; after ShowNextLoc the big one (7) runs to its last frame.
  let (a2, d2) := Intermission.initAnims 1 0 { prndindex := 0, rndindex := 0 }
  check c "tracker E2 init no draws" (d2 == 0)
  let mut a := a2
  for b in [1:60] do a := Intermission.updateAnims 1 a (Int32.ofNat b) true 7
  check c "tracker E2 statcount: anim 6 (data1 7) shown, 7 not" (a[6]!.ctr == 0 && a[7]!.ctr == -1 && a[0]!.ctr == -1)
  let mut b2 := (Intermission.initAnims 1 100 { prndindex := 0, rndindex := 0 }).1
  for b in [101:200] do b2 := Intermission.updateAnims 1 b2 (Int32.ofNat b) false 8
  check c "tracker E2 shownextloc: anim 7 reaches frame 2 and holds" (b2[7]!.ctr == 2 && b2[8]!.ctr == 0)

/-! ## Finale -/

def finaleChecks (c : Ctx) (wad : WadDirectory) (data : Doom.Render.Data.RenderData) (tag : String)
    (commercial retail : Bool) (deh : Deh.Strings) : IO Unit := do
  let pal := (Doom.Render.Palette.playpalSlice data.playpal 0).toOption.getD ByteArray.empty
  let font := Draw.loadFont wad
  let render (st : Finale.State) : Except String Framebuffer := Finale.draw wad data font Framebuffer.initBlack st
  if !commercial then
    let episodes := if retail then [1, 2, 3, 4]
      else if (checkNumForName wad "E2M1").isSome then [1, 2, 3] else [1]
    for e in episodes do
      let (st0, ev) := Finale.start deh false retail (Int32.ofNat e) 8
      check c s!"{tag} finale E{e} music victor" (ev == #[.music "victor" true])
      check c s!"{tag} finale E{e} has text and flat" (!st0.text.isEmpty && (checkNumForName wad st0.flat).isSome) st0.flat
      -- Text crawl: one character per 3 tics after 10.
      let mut st := st0
      for _ in [0:130] do st := (Finale.tick st false).1
      check c s!"{tag} finale E{e} count after 130 tics" (st.finalecount == 130 && st.stage == .text)
      if let some fb ← orFail c s!"{tag} finale E{e} text" (render st) then
        save c s!"{tag}-finale-e{e}-text-130" fb pal
        let first := st.text.toUTF8[0]!
        let g := (font.glyph? first).get!
        check c s!"{tag} finale E{e} first glyph at (10,10)" (patchAt wad fb (Doom.Render.Hud.stcfnName (if first >= 97 then first.toNat - 32 else first.toNat)) 10 10)
        check c s!"{tag} finale E{e} flat behind text" (fb.pixels[199 * 320 + 319]! != 0 || colours fb > 4)
        let _ := g
      let len := Int32.ofNat st0.text.utf8ByteSize
      let full := len * 3 + 250
      for _ in [130:full.toNatClampNeg] do st := (Finale.tick st false).1
      check c s!"{tag} finale E{e} still text at count {full}" (st.stage == .text && st.finalecount == full)
      if let some fb ← orFail c s!"{tag} finale E{e} full text" (render st) then
        save c s!"{tag}-finale-e{e}-text-full" fb pal
      let (st', ev') := Finale.tick st false
      check c s!"{tag} finale E{e} art screen after strlen*3+250" (st'.stage == .artscreen && st'.finalecount == 0)
      check c s!"{tag} finale E{e} bunny music" ((e == 3) == ev'.contains (.music "bunny" false))
      st := st'
      if e == 3 then
        let mut pistols := 0
        for t in [1:1300] do
          let (s2, ev2) := Finale.tick st false
          st := s2
          pistols := pistols + (ev2.filter (· == .sound Finale.sfx_pistol)).size
          if t ∈ [1, 400, 600, 1150, 1190, 1250] then
            if let some fb ← orFail c s!"{tag} bunny {t}" (render st) then
              save c s!"{tag}-finale-e3-bunny-{t}" fb pal
              if t == 1 then
                check c s!"{tag} bunny starts on PFUB1" (patchAt wad fb "PFUB1" 0 0)
              if t == 1250 then
                check c s!"{tag} bunny ends on PFUB2 with END6" (patchPct wad fb "PFUB2" 0 0 > 90 &&
                  patchAt wad fb "END6" 108 68)
              if t == 1150 then
                check c s!"{tag} bunny END0" (patchAt wad fb "END0" 108 68)
        check c s!"{tag} bunny six pistol shots" (pistols == 6) s!"{pistols}"
      else
        if let some fb ← orFail c s!"{tag} art E{e}" (render st) then
          save c s!"{tag}-finale-e{e}-art" fb pal
          let lump := if e == 1 then (if retail then "CREDIT" else "HELP2") else if e == 2 then "VICTORY2" else "ENDPIC"
          check c s!"{tag} finale E{e} art {lump}" (patchAt wad fb lump 0 0)
  else
    -- Doom II: MAP06 text, skip with a button after 50 tics.
    let (st0, ev) := Finale.start deh true false 1 6
    check c s!"{tag} finale MAP06 read_m music" (ev == #[.music "read_m" true])
    check c s!"{tag} finale MAP06 flat" ((checkNumForName wad st0.flat).isSome) st0.flat
    let mut st := st0
    for _ in [0:51] do st := (Finale.tick st true).1
    let (_, ev51) := Finale.tick st true
    check c s!"{tag} finale MAP06 button after 50 tics -> worldDone" (ev51.contains .worldDone)
    let (_, ev50) := Finale.tick st0 true
    check c s!"{tag} finale MAP06 no skip before 50 tics" (!ev50.contains .worldDone)
    for _ in [0:300] do st := (Finale.tick st false).1
    if let some fb ← orFail c s!"{tag} MAP06 text" (render st) then
      save c s!"{tag}-finale-map06-text" fb pal
    -- MAP30: button -> cast call.
    let (st30, _) := Finale.start deh true false 1 30
    let mut sc := st30
    for _ in [0:51] do sc := (Finale.tick sc false).1
    let (stc, evc) := Finale.tick sc true
    check c s!"{tag} MAP30 button starts the cast" (stc.stage == .cast && evc.contains (.music "evil" true))
    sc := stc
    let mut seen : Array Nat := #[sc.castnum]
    let mut attacks := 0
    let mut sounds := 0
    for t in [1:2400] do
      -- Kill the current monster now and then (F_CastResponder).
      if t % 300 == 0 then
        let (s2, ev2, eaten) := Finale.respond sc
        check c s!"{tag} cast responder eats keys" eaten
        sc := s2
        sounds := sounds + ev2.size
      let (s2, ev2) := Finale.tick sc false
      sc := s2
      sounds := sounds + ev2.size
      if sc.castattacking then attacks := attacks + 1
      if seen.back? != some sc.castnum then seen := seen.push sc.castnum
      if t ∈ [5, 100, 302, 900, 1500] then
        if let some fb ← orFail c s!"{tag} cast {t}" (render sc) then
          save c s!"{tag}-finale-cast-{t}-{sc.castnum}" fb pal
          check c s!"{tag} cast frame {t} colourful" (colours fb > 16)
          -- The name is centred at y=180 over BOSSBACK.
          let name := sc.castNames[sc.castnum]!
          let w := name.toUTF8.foldl (fun w ch => w + ((font.glyph? ch).map (·.width)).getD 4) (0 : Int32)
          let first := name.toUTF8[0]!
          let up := if first >= 97 && first <= 122 then first.toNat - 32 else first.toNat
          check c s!"{tag} cast name '{name}' centred" (patchAt wad fb (Doom.Render.Hud.stcfnName up) (160 - w / 2) 180)
    check c s!"{tag} cast advances through monsters" (seen.size >= 4) s!"{seen}"
    check c s!"{tag} cast attacks" (attacks > 0)
    check c s!"{tag} cast sounds" (sounds > 3) s!"{sounds}"
    let (sd, evd, _) := Finale.respond { sc with castdeath := false }
    check c s!"{tag} cast death state" (sd.castdeath && sd.castframes == 0 && !sd.castattacking && evd.size ≤ 1)

/-! ## Automap -/

def amChecks (c : Ctx) (wad : WadDirectory) (data : Doom.Render.Data.RenderData) (tag : String)
    (commercial : Bool) (deh : Deh.Strings) : IO Unit := do
  let some gs ← orFail c s!"{tag} AM level" (startLevel wad commercial 1 1) | return
  let title := deh.levelTitle commercial 1 1
  -- One 3D frame marks the lines in view.
  let some (_, cache) ← orFail c s!"{tag} AM level frame" (renderLevel data wad gs {} 9) | return
  let seen := cache.mapped.foldl (fun n b => if b != 0 then n + 1 else n) 0
  check c s!"{tag} AM view marks lines" (seen > 5 && cache.mapped.size == gs.level.lines.size) s!"{seen}/{cache.mapped.size}"
  let r := Automap.respond {} gs Automap.key_map_toggle true deh.str
  check c s!"{tag} AM toggle opens" (r.eaten && r.state.active)
  let mut am := r.state
  check c s!"{tag} AM initial scale within bounds" (am.scale_mtof >= am.min_scale_mtof && am.scale_mtof <= am.max_scale_mtof)
  am := Automap.tick am gs
  let shot (name : String) (am : Automap.State) (gs : GameState) (cache : Cache) : IO (Option Framebuffer) := do
    match renderAutomap data wad gs cache am title with
    | .ok (pal, c') => save c s!"{tag}-am-{name}" c'.fb pal; pure (some c'.fb)
    | .error e => check c s!"{tag} AM {name}" false e; pure none
  if let some fb ← shot "seen" am gs cache then
    check c s!"{tag} AM crosshair" (fb.pixels[(320 * 169) / 2]! == Automap.XHAIRCOLORS)
    check c s!"{tag} AM player arrow at the centre (follow)"
      (fb.pixels[84 * 320 + 160 - 1]! == Automap.WHITE || fb.pixels[84 * 320 + 160 + 1]! == Automap.WHITE ||
       fb.pixels[85 * 320 + 160]! == Automap.WHITE || fb.pixels[83 * 320 + 160]! == Automap.WHITE)
    check c s!"{tag} AM seen walls drawn" (countColour fb Automap.WALLCOLORS 0 0 168 + countColour fb Automap.FDWALLCOLORS 0 0 168 > 20)
    check c s!"{tag} AM status bar below, as in the level frame"
      (fb.pixels.extract (168 * 320) (200 * 320) == cache.fb.pixels.extract (168 * 320) (200 * 320))
    let fh := patchH wad "STCFN033"
    let first := title.toUTF8[0]!
    check c s!"{tag} AM level title '{title}'" (patchAt wad fb (Doom.Render.Hud.stcfnName first.toNat) 0 (167 - fh))
  -- iddt: every line; twice: things too.
  let mut am2 := am
  for k in "iddt".toList do am2 := (Automap.respond am2 gs k.toNat true deh.str).state
  check c s!"{tag} AM iddt once" (am2.cheating == 1)
  if let some fb ← shot "iddt" am2 gs cache then
    check c s!"{tag} AM iddt shows more walls" (countColour fb Automap.TSWALLCOLORS 0 0 168 > 0)
  for k in "iddt".toList do am2 := (Automap.respond am2 gs k.toNat true deh.str).state
  check c s!"{tag} AM iddt twice" (am2.cheating == 2)
  if let some fb ← shot "iddt2-things" am2 gs cache then
    check c s!"{tag} AM things drawn" (countColour fb Automap.THINGCOLORS 0 0 168 > 10)
  -- Grid and computer map.
  let r := Automap.respond am gs Automap.key_map_grid true deh.str
  check c s!"{tag} AM grid message" (r.message == some (deh.str "AMSTR_GRIDON")) s!"{r.message}"
  if let some fb ← shot "grid" r.state gs cache then
    check c s!"{tag} AM grid lines" (countColour fb Automap.GRIDCOLORS 0 0 168 > 100)
  let gsMap := match gs.players[0]? with
    | some p => { gs with players := gs.players.set! 0 { p with powers := p.powers.set! pw_allmap 1 } }
    | none => gs
  if let some fb ← shot "computer-map" am gsMap cache then
    check c s!"{tag} AM computer map greys" (countColour fb (Automap.GRAYS + 3) 0 0 168 > 50)
  -- Zoom out to the whole map and in to the limit.
  let mut z := (Automap.respond am gs Automap.key_map_zoomout true).state
  for _ in [0:400] do z := Automap.tick z gs
  check c s!"{tag} AM zoom out stops at min scale" (z.scale_mtof == z.min_scale_mtof)
  z := (Automap.respond z gs Automap.key_map_zoomout false).state
  check c s!"{tag} AM zoom key up stops" (z.mtof_zoommul == 65536 && z.ftom_zoommul == 65536)
  let _ ← shot "zoomed-out" z gs cache
  z := (Automap.respond z gs Automap.key_map_zoomin true).state
  for _ in [0:400] do z := Automap.tick z gs
  check c s!"{tag} AM zoom in stops at max scale" (z.scale_mtof == z.max_scale_mtof)
  if let some fb ← shot "zoomed-in" z gs cache then
    -- The arrow spans the window: R = 8*PLAYERRADIUS/7 at 168 px per 32 units.
    let white := countColour fb Automap.WHITE 0 0 168
    check c s!"{tag} AM zoomed-in arrow" (white > 150) s!"{white}"

  -- Maximum-zoom toggle and back.
  let big := (Automap.respond am gs Automap.key_map_maxzoom true).state
  check c s!"{tag} AM max zoom toggle" (big.bigstate && big.scale_mtof == big.min_scale_mtof)
  let back := (Automap.respond big gs Automap.key_map_maxzoom true).state
  check c s!"{tag} AM max zoom restore" (!back.bigstate && back.m_w == am.m_w && back.scale_mtof == am.scale_mtof)
  -- Follow off: arrows pan (and are eaten); clamped to the map.
  let r := Automap.respond am gs Automap.key_map_follow true deh.str
  check c s!"{tag} AM follow off message" (r.message == some (deh.str "AMSTR_FOLLOWOFF"))
  let r2 := Automap.respond am gs Automap.key_map_east true
  check c s!"{tag} AM arrows go to the game while following" (!r2.eaten)
  let mut p := (Automap.respond r.state gs Automap.key_map_east true).state
  check c s!"{tag} AM pan right speed FTOM(4)" (p.m_paninc_x == p.ftom 4)
  for _ in [0:2000] do p := Automap.tick p gs
  check c s!"{tag} AM pan clamped at max_x" (p.m_x + p.m_w / 2 == p.max_x)
  let _ ← shot "panned" p gs cache
  -- Marks.
  let r := Automap.respond am gs Automap.key_map_mark true deh.str
  let markMsg := s!"{deh.str "AMSTR_MARKEDSPOT"} 0"
  let markMsg := if markMsg.length > 19 then String.ofList (markMsg.toList.take 19) else markMsg
  check c s!"{tag} AM mark message (20-byte buffer)" (r.message == some markMsg) s!"{r.message}"
  if let some fb ← shot "mark" r.state gs cache then
    let mp := r.state.markpoints[0]!
    check c s!"{tag} AM mark digit drawn" (patchAt wad fb "AMMNUM0" (r.state.cxmtof mp.x) (r.state.cymtof mp.y))
  let r := Automap.respond r.state gs Automap.key_map_clearmark true deh.str
  check c s!"{tag} AM marks cleared" (r.state.markpoints.all (·.x == -1))
  -- Toggle closes.
  let r := Automap.respond am gs Automap.key_map_toggle true
  check c s!"{tag} AM toggle closes" (r.eaten && !r.state.active)
  -- Drawer cost (host): the iddt map of the first level.
  let t0 ← IO.monoNanosNow
  let mut fb := Framebuffer.initBlack
  for _ in [0:200] do
    match Automap.draw am2 gs cache.mapped wad fb with
    | .ok f => fb := f
    | .error _ => pure ()
  let t1 ← IO.monoNanosNow
  IO.println s!"front-test: {tag} AM_Drawer (iddt, {gs.level.lines.size} lines): {(t1 - t0) / 200 / 1000} us/frame on host"


/-! ## Game flow: the playsim's `G_Ticker` with the front end's screens -/

/-- `G_Ticker` then the front end for one tic with the given buttons. -/
def flowTic (st : Screens.State) (gs : GameState) (buttons : UInt32) :
    Except String (Screens.State × GameState) := do
  let gs ← tic gs buttons
  let (st, gs, _) := st.tic gs
  pure (st, gs)

/-- Music started during a tic, from the playsim's sound events. -/
def musicOf (gs : GameState) : Array Nat :=
  gs.soundEvents.filterMap fun | .music m _ => some m | _ => none

/-- Play out an intermission (pressing use to skip) until it ends. -/
def throughIntermission (st0 : Screens.State) (gs0 : GameState) :
    Except String (Screens.State × GameState × Array Nat × Nat) := do
  let mut st := st0
  let mut gs := gs0
  let mut music : Array Nat := #[]
  for t in [0:2000] do
    let b : UInt32 := if t % 40 == 20 then BT_USE else 0
    let (st', gs') ← flowTic st gs b
    st := st'; gs := gs'; music := music ++ musicOf gs
    if gs.gamestate != gamestateIntermission then return (st, gs, music, t)
  return (st, gs, music, 2000)

def flowChecks (c : Ctx) (wad : WadDirectory) (data : Doom.Render.Data.RenderData) (tag : String)
    (commercial retail : Bool) : IO Unit := do
  let st0 := Screens.State.init wad commercial retail
  -- Level with the automap open, then the exit: the map closes, intermission music starts.
  let some gs ← orFail c s!"{tag} flow level" (startLevel wad commercial 1 1) | return
  let (st, gs, _, _) := st0.respond gs Automap.key_map_toggle true
  check c s!"{tag} flow automap open" st.automap.active
  let some (st, gs) ← orFail c s!"{tag} flow exit" (flowTic st { gs with gameaction := ga_completed } 0) | return
  let interMus := if commercial then Doom.Playsim.Defs.mus_dm2int else Doom.Playsim.Defs.mus_inter
  let musicStart := musicOf gs
  let some (st, gs) ← orFail c s!"{tag} flow intermission start" (flowTic st gs 0) | return
  let musicStart := musicStart ++ musicOf gs
  check c s!"{tag} flow exit -> intermission, map closed"
    (gs.gamestate == gamestateIntermission && !st.automap.active)
  let some (st, gs, music, t) ← orFail c s!"{tag} flow intermission" (throughIntermission st gs) | return
  check c s!"{tag} flow intermission music" (musicStart.contains interMus) s!"{musicStart}"
  check c s!"{tag} flow intermission ends in the next map" (gs.gamestate == gamestateLevel && gs.gamemap == 2) s!"tic {t} map {gs.gamemap}"
  if let .ok (pal, cache) := st.render data wad gs {} 9 then save c s!"{tag}-flow-next-map" cache.fb pal
  -- Episode / chapter ends.
  let finaleMap := if commercial then 6 else 8
  let some gs ← orFail c s!"{tag} flow finale level" (startLevel wad commercial 1 finaleMap) | return
  let some (st, gs) ← orFail c s!"{tag} flow finale exit" (flowTic st0 { gs with gameaction := ga_completed } 0) | return
  let (st, gs) ← if commercial then do
      -- MAP06: intermission first, then `G_WorldDone` starts the text screen instead of the next map.
      let some (st, gs) ← orFail c s!"{tag} flow MAP06 intermission start" (flowTic st gs 0) | return
      check c s!"{tag} flow MAP06 -> intermission" (gs.gamestate == gamestateIntermission)
      let some (st, gs, _, _) ← orFail c s!"{tag} flow MAP06 intermission" (throughIntermission st gs) | return
      check c s!"{tag} flow MAP06 -> finale text" (gs.gamestate == gamestateFinale)
      pure (st, gs)
    else do
      -- E1M8: `ga_victory` -> `F_StartFinale`.
      let some (st, gs) ← orFail c s!"{tag} flow E1M8 victory" (flowTic st gs 0) | return
      check c s!"{tag} flow E1M8 -> finale" (gs.gamestate == gamestateFinale)
      pure (st, gs)
  let mut st := st
  let mut gs := gs
  let mut continued := false
  for t in [0:400] do
    let b : UInt32 := if t == 300 then BT_USE else 0
    let some (st', gs') ← orFail c s!"{tag} flow finale tic" (flowTic st gs b) | return
    st := st'; gs := gs'
    if t == 100 then
      if let .ok (pal, cache) := st.render data wad gs {} 9 then save c s!"{tag}-flow-finale" cache.fb pal
    if gs.gamestate == gamestateLevel then continued := true; break
  check c s!"{tag} flow finale: Doom II continues on a button, Doom 1 does not" (continued == commercial)

/-! ## Strings -/

def dehChecks (c : Ctx) (d1 d2 : Deh.Strings) (dsw : Option Deh.Strings) : IO Unit := do
  check c "deh freedoom1 E1M1 title" (d1.levelTitle false 1 1 == "E1M1: Outer Prison") (d1.levelTitle false 1 1)
  check c "deh freedoom1 E4M9 title" (d1.levelTitle false 4 9 == "E4M9: Operations")
  check c "deh freedoom1 cast name" (d1.str "CC_ZOMBIE" == "zombie")
  check c "deh freedoom1 E1TEXT" ((d1.str "E1TEXT").startsWith "Free at last from that dreadful outpost!\nBut you're")
  check c "deh freedoom1 E1 flat" (d1.get "BGFLATE1" "FLOOR4_8" == "AQF051")
  check c "deh freedoom2 MAP01 title" (d2.levelTitle true 1 1 == "MAP01: Hydroelectric Plant")
  check c "deh freedoom2 automap message" (d2.str "AMSTR_FOLLOWOFF" == "Map no longer following player.")
  if let some d := dsw then
    check c "deh shareware has none" (d.entries.isEmpty && d.levelTitle false 1 1 == "E1M1: Hangar")

def main (args : List String) : IO UInt32 := do
  let out : System.FilePath := (args.headD "../../../assets/build/lean-doom-front")
  IO.FS.createDirAll out
  let c : Ctx := { out, failures := ← IO.mkRef #[], passes := ← IO.mkRef 0, images := ← IO.mkRef #[] }
  let root : System.FilePath := "../../../assets/third_party"
  let f1 ← loadFile (root / "freedoom" / "freedoom1.wad")
  let f2 ← loadFile (root / "freedoom" / "extract" / "freedoom-0.13.0" / "freedoom2.wad")
  let swPath := root / "doom_lean" / "src" / "fixtures" / "wads" / "doom1.wad"
  let sw ← if ← swPath.pathExists then some <$> loadFile swPath else pure none
  let d1 := Deh.load f1
  let d2 := Deh.load f2
  dehChecks c d1 d2 (sw.map Deh.load)
  trackerChecks c
  let iwads : List (String × WadDirectory × Bool × Bool) :=
    [("fd1", f1, false, true), ("fd2", f2, true, false)] ++
    (match sw with | some w => [("sw", w, false, false)] | none => [])
  for (tag, wad, commercial, retail) in iwads do
    let some data ← orFail c s!"{tag} initData" (Doom.Render.Data.initData wad) | continue
    let deh := Deh.load wad
    wiChecks c wad data tag commercial retail
    wiFlow c wad data tag commercial retail
    finaleChecks c wad data tag commercial retail deh
    amChecks c wad data tag commercial deh
    flowChecks c wad data tag commercial retail
  let fails ← c.failures.get
  let passes ← c.passes.get
  let imgs ← c.images.get
  IO.println s!"front-test: {passes} checks passed, {fails.size} failed; {imgs.size} images in {out}"
  return if fails.isEmpty then 0 else 1
