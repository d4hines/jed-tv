import Doom.Front.Deh
import Doom.Front.Draw
import Doom.Front.Strings
import Doom.Playsim.Info
import Doom.Render.Data
import Doom.Render.Types
import Doom.Wad

/-!
# Doom.Front.Finale

The end-of-episode and Doom II finales (`f_finale.c`) as a pure state
machine: `start` (`F_StartFinale`), `tick` (`F_Ticker`), `respond`
(`F_Responder`) and `draw` (`F_Drawer`). The text crawl over the episode's
flat, the art screens (E1 `CREDIT`/`HELP2`, E2 `VICTORY2`, E3 bunny scroll,
E4 `ENDPIC`) and the Doom II cast call.

Sounds and music come back as `Event`s, like the menu's effects; the caller
also draws the `M_Random` pitch value for each sound if it keeps that
generator in step. `F_BunnyScroll` starts its pistol sound from the drawer;
here the same stage test runs at the end of the tic instead, so `draw` stays
pure (the sound lands on the tic whose frame first shows the new stage).
Text and flat names honour the IWAD's DEHACKED strings (Freedoom).
-/

namespace Doom.Front.Finale

open Doom.Front.Draw
open Doom.Render.Types
open Doom.Wad

def TEXTSPEED : Int32 := 3
def TEXTWAIT : Int32 := 250

/-- `finalestage_t`. -/
inductive Stage | text | artscreen | cast
  deriving BEq, Repr, Inhabited

/-- What the finale asks of the game loop. -/
inductive Event
  /-- `S_StartSound(NULL, sfx)`. -/
  | sound (sfx : Nat)
  /-- `S_ChangeMusic` / `S_StartMusic` by lump suffix (`"victor"` for `D_VICTOR`). -/
  | music (name : String) (looping : Bool)
  /-- `gameaction = ga_worlddone` (Doom II: go on to the next map). -/
  | worldDone
  deriving BEq, Repr, Inhabited

/-- `f_finale.c` statics. -/
structure State where
  stage : Stage := .text
  /-- `finalecount` (unsigned in C; never near the wrap). -/
  finalecount : Int32 := 0
  commercial : Bool := false
  /-- `gameversion >= exe_ultimate`. -/
  retail : Bool := false
  gameepisode : Int32 := 1
  gamemap : Int32 := 1
  /-- `finaletext` / `finaleflat` after DEHACKED substitution. -/
  text : String := ""
  flat : String := ""
  castnum : Nat := 0
  caststate : UInt32 := 0
  casttics : Int32 := 0
  castdeath : Bool := false
  castframes : Int32 := 0
  castonmelee : Int32 := 0
  castattacking : Bool := false
  /-- `F_BunnyScroll`'s static `laststage`. -/
  laststage : Int32 := 0
  /-- Cast names after DEHACKED substitution (`castorder[].name`). -/
  castNames : Array String := #[]
  deriving Inhabited

/-- `textscreens[]` for `doom` and `doom2`: (episode or 0, map, text index, BEX flat key, flat). -/
def textscreens : Array (Bool × Int32 × Int32 × Nat × String × String) := #[
  (false, 1, 8, 0, "BGFLATE1", "FLOOR4_8"),
  (false, 2, 8, 1, "BGFLATE2", "SFLR6_1"),
  (false, 3, 8, 2, "BGFLATE3", "MFLR8_4"),
  (false, 4, 8, 3, "BGFLATE4", "MFLR8_3"),
  (true, 1, 6, 4, "BGFLAT06", "SLIME16"),
  (true, 1, 11, 5, "BGFLAT11", "RROCK14"),
  (true, 1, 20, 6, "BGFLAT20", "RROCK07"),
  (true, 1, 30, 7, "BGFLAT30", "RROCK17"),
  (true, 1, 15, 8, "BGFLAT15", "RROCK13"),
  (true, 1, 31, 9, "BGFLAT31", "RROCK19")]

/-- `castorder[].type` (`mobjtype_t`). -/
def castTypes : Array Nat := #[1, 2, 10, 11, 12, 18, 14, 17, 15, 20, 22, 5, 8, 3, 19, 21, 0]

/-! `statenum_t` values used by `F_CastTicker`. -/
def S_NULL : UInt32 := 0
def S_PLAY_ATK1 : UInt32 := 154
def S_POSS_ATK2 : UInt32 := 185
def S_SPOS_ATK2 : UInt32 := 218
def S_VILE_ATK2 : UInt32 := 256
def S_SKEL_FIST2 : UInt32 := 336
def S_SKEL_FIST4 : UInt32 := 338
def S_SKEL_MISS2 : UInt32 := 340
def S_FATT_ATK2 : UInt32 := 377
def S_FATT_ATK5 : UInt32 := 380
def S_FATT_ATK8 : UInt32 := 383
def S_CPOS_ATK2 : UInt32 := 417
def S_CPOS_ATK3 : UInt32 := 418
def S_CPOS_ATK4 : UInt32 := 419
def S_TROO_ATK3 : UInt32 := 454
def S_SARG_ATK2 : UInt32 := 486
def S_HEAD_ATK2 : UInt32 := 505
def S_BOSS_ATK2 : UInt32 := 538
def S_BOS2_ATK2 : UInt32 := 567
def S_SKULL_ATK2 : UInt32 := 590
def S_SPID_ATK2 : UInt32 := 616
def S_SPID_ATK3 : UInt32 := 617
def S_BSPI_ATK2 : UInt32 := 648
def S_CYBER_ATK2 : UInt32 := 685
def S_CYBER_ATK4 : UInt32 := 687
def S_CYBER_ATK6 : UInt32 := 689
def S_PAIN_ATK3 : UInt32 := 710

/-! `sfxenum_t` values. -/
def sfx_pistol : Nat := 1
def sfx_shotgn : Nat := 2
def sfx_dshtgn : Nat := 4
def sfx_plasma : Nat := 8
def sfx_rlaunc : Nat := 14
def sfx_firsht : Nat := 16
def sfx_sklatk : Nat := 51
def sfx_sgtatk : Nat := 52
def sfx_skepch : Nat := 53
def sfx_vilatk : Nat := 54
def sfx_claw : Nat := 55
def sfx_skeswg : Nat := 56
def sfx_skeatk : Nat := 107

deriving instance Inhabited for Doom.Playsim.Info.MobjInfo

private def stateAt (s : UInt32) : Doom.Playsim.Info.State :=
  (Doom.Playsim.Info.states[s.toNat]?).getD
    { sprite := 0, frame := 0, tics := -1, action := 0, nextstate := 0, misc1 := 0, misc2 := 0 }

private def info (castnum : Nat) : Doom.Playsim.Info.MobjInfo :=
  Doom.Playsim.Info.mobjinfo[(castTypes[castnum]?).getD 0]!

/-- `F_StartFinale` for the current episode and map. -/
def start (deh : Doom.Front.Deh.Strings) (commercial retail : Bool) (gameepisode gamemap : Int32) :
    State × Array Event := Id.run do
  let mut text := ""
  let mut flat := ""
  for (doom2, ep, lvl, ti, key, fl) in textscreens do
    if doom2 == commercial && (commercial || gameepisode == ep) && gamemap == lvl then
      let (tk, td) := Doom.Front.Strings.finaleTexts[ti]!
      text := deh.get tk td
      flat := deh.get key fl
  let castNames := Doom.Front.Strings.castNames.map fun (k, d) => deh.get k d
  let st : State := { commercial, retail, gameepisode, gamemap, text, flat, castNames }
  return (st, #[.music (if commercial then "read_m" else "victor") true])

/-- `F_StartCast`. -/
def startCast (st : State) : State × Array Event :=
  let s := (info 0).seestate
  ({ st with castnum := 0, caststate := s, casttics := (stateAt s).tics, castdeath := false,
             stage := .cast, castframes := 0, castonmelee := 0, castattacking := false },
   #[.music "evil" true])

/-- The `F_CastTicker` sound for entering state `st`, or 0. -/
def castSound (st : UInt32) : Nat :=
  if st == S_PLAY_ATK1 then sfx_dshtgn
  else if st == S_POSS_ATK2 then sfx_pistol
  else if st == S_SPOS_ATK2 then sfx_shotgn
  else if st == S_VILE_ATK2 then sfx_vilatk
  else if st == S_SKEL_FIST2 then sfx_skeswg
  else if st == S_SKEL_FIST4 then sfx_skepch
  else if st == S_SKEL_MISS2 then sfx_skeatk
  else if st == S_FATT_ATK8 || st == S_FATT_ATK5 || st == S_FATT_ATK2 then sfx_firsht
  else if st == S_CPOS_ATK2 || st == S_CPOS_ATK3 || st == S_CPOS_ATK4 then sfx_shotgn
  else if st == S_TROO_ATK3 then sfx_claw
  else if st == S_SARG_ATK2 then sfx_sgtatk
  else if st == S_BOSS_ATK2 || st == S_BOS2_ATK2 || st == S_HEAD_ATK2 then sfx_firsht
  else if st == S_SKULL_ATK2 then sfx_sklatk
  else if st == S_SPID_ATK2 || st == S_SPID_ATK3 then sfx_shotgn
  else if st == S_BSPI_ATK2 then sfx_plasma
  else if st == S_CYBER_ATK2 || st == S_CYBER_ATK4 || st == S_CYBER_ATK6 then sfx_rlaunc
  else if st == S_PAIN_ATK3 then sfx_sklatk
  else 0

/-- `F_CastTicker`. -/
def castTicker (st0 : State) : State × Array Event := Id.run do
  let mut st := { st0 with casttics := st0.casttics - 1 }
  if st.casttics > 0 then return (st, #[])
  let mut ev : Array Event := #[]
  let mut stopattack := false
  let cs := stateAt st.caststate
  if cs.tics == -1 || cs.nextstate == S_NULL then
    -- From the death state to the next monster.
    let n := if st.castnum + 1 >= castTypes.size then 0 else st.castnum + 1
    let inf := info n
    if inf.seesound != 0 then ev := ev.push (.sound inf.seesound.toNatClampNeg)
    st := { st with castnum := n, castdeath := false, caststate := inf.seestate, castframes := 0 }
  else if st.caststate == S_PLAY_ATK1 then
    stopattack := true  -- "Oh, gross hack!"
  else
    let s := cs.nextstate
    st := { st with caststate := s, castframes := st.castframes + 1 }
    let sfx := castSound s
    if sfx != 0 then ev := ev.push (.sound sfx)
  let inf := info st.castnum
  if !stopattack && st.castframes == 12 then
    -- Into an attack frame, alternating melee and missile.
    let s := if st.castonmelee != 0 then inf.meleestate else inf.missilestate
    let melee := st.castonmelee ^^^ 1
    let s := if s == S_NULL then (if melee != 0 then inf.meleestate else inf.missilestate) else s
    st := { st with castattacking := true, caststate := s, castonmelee := melee }
  if stopattack || (st.castattacking && (st.castframes == 24 || st.caststate == inf.seestate)) then
    st := { st with castattacking := false, castframes := 0, caststate := inf.seestate }
  let t := (stateAt st.caststate).tics
  return ({ st with casttics := if t == -1 then 15 else t }, ev)

/-- `F_Ticker`. `anyButtons`: some player's ticcmd has buttons pressed. -/
def tick (st0 : State) (anyButtons : Bool) : State × Array Event := Id.run do
  let mut st := st0
  let mut ev : Array Event := #[]
  if st.commercial && st.finalecount > 50 && anyButtons then
    if st.gamemap == 30 then
      let (s, e) := startCast st
      st := s; ev := ev ++ e
    else ev := ev.push .worldDone
  st := { st with finalecount := st.finalecount + 1 }
  if st.stage == .cast then
    let (s, e) := castTicker st
    return (s, ev ++ e)
  if st.commercial then return (st, ev)
  if st.stage == .text &&
      st.finalecount > Int32.ofNat st.text.utf8ByteSize * TEXTSPEED + TEXTWAIT then
    st := { st with finalecount := 0, stage := .artscreen }
    if st.gameepisode == 3 then ev := ev.push (.music "bunny" false)
  -- `F_BunnyScroll` stage sounds (see the module note).
  if st.stage == .artscreen && st.gameepisode == 3 && st.finalecount >= 1130 then
    if st.finalecount < 1180 then st := { st with laststage := 0 }
    else
      let stage := min 6 ((st.finalecount - 1180) / 5)
      if stage > st.laststage then
        ev := ev.push (.sound sfx_pistol)
        st := { st with laststage := stage }
  return (st, ev)

/-- `F_Responder` for a key press: the cast call's "kill this monster". -/
def respond (st : State) : State × Array Event × Bool :=
  if st.stage != .cast then (st, #[], false)
  else if st.castdeath then (st, #[], true)
  else
    let inf := info st.castnum
    let s := inf.deathstate
    ({ st with castdeath := true, caststate := s, casttics := (stateAt s).tics, castframes := 0,
               castattacking := false },
     if inf.deathsound != 0 then #[.sound inf.deathsound.toNatClampNeg] else #[], true)

/-- `F_TextWrite`: the flat and the text typed out so far. -/
def textWrite (wad : WadDirectory) (font : Font) (fb : Framebuffer) (st : State) :
    Except String Framebuffer := do
  let mut fb := fillFlat wad fb st.flat
  let mut cx : Int32 := 10
  let mut cy : Int32 := 10
  let count0 := (st.finalecount - 10) / TEXTSPEED
  let mut count := if count0 < 0 then 0 else count0.toNatClampNeg
  for c in st.text.toUTF8 do
    if count == 0 then break
    count := count - 1
    if c == 0 then break
    if c == 10 then
      cx := 10
      cy := cy + 11
      continue
    match font.glyph? c with
    | none => cx := cx + 4
    | some g =>
      if cx + g.width > 320 then break
      fb ← patch wad fb g cx cy
      cx := cx + g.width
  return fb

/-- `F_CastPrint`: centred at y = 180. -/
def castPrint (wad : WadDirectory) (font : Font) (fb : Framebuffer) (text : String) :
    Except String Framebuffer := do
  let width := text.toUTF8.foldl (fun w c => w + ((font.glyph? c).map (·.width)).getD 4) (0 : Int32)
  let mut cx := 160 - width / 2
  let mut fb := fb
  for c in text.toUTF8 do
    match font.glyph? c with
    | none => cx := cx + 4
    | some g =>
      fb ← patch wad fb g cx 180
      cx := cx + g.width
  return fb

/-- `F_CastDrawer`. -/
def castDrawer (wad : WadDirectory) (data : Doom.Render.Data.RenderData) (font : Font)
    (fb : Framebuffer) (st : State) : Except String Framebuffer := do
  let mut fb ← named wad fb "BOSSBACK" 0 0
  fb ← castPrint wad font fb (st.castNames[st.castnum]?.getD "")
  let cs := stateAt st.caststate
  let some sprdef := data.sprites[cs.sprite.toNat]? | return fb
  let some fr := sprdef.spriteframes[(cs.frame &&& 0x7fff).toNat]? | return fb
  let lump : Int32 := fr.lump[0]?.getD (-1)
  if lump < 0 then return fb
  let p := patchInfo wad (data.firstSprite + lump.toNatClampNeg)
  if fr.flip[0]?.getD 0 != 0 then return patchFlipped wad fb p 160 170
  patch wad fb p 160 170

/-- `F_BunnyScroll`: the E3 two-panel scroll and "THE END". -/
def bunnyScroll (wad : WadDirectory) (fb : Framebuffer) (st : State) : Except String Framebuffer := do
  let some p1 := patchNamed? wad "PFUB2" | return fb
  let some p2 := patchNamed? wad "PFUB1" | return fb
  let scrolled0 := 320 - (st.finalecount - 230) / 2
  let scrolled := if scrolled0 > 320 then 320 else if scrolled0 < 0 then 0 else scrolled0
  let mut fb := fb
  for x in [0:320] do
    let xs := Int32.ofNat x + scrolled
    fb := if xs < 320 then patchColumn wad fb p1 (Int32.ofNat x) xs.toNatClampNeg
      else patchColumn wad fb p2 (Int32.ofNat x) (xs - 320).toNatClampNeg
  if st.finalecount < 1130 then return fb
  let ex : Int32 := (320 - 13 * 8) / 2
  let ey : Int32 := (200 - 8 * 8) / 2
  if st.finalecount < 1180 then return (← named wad fb "END0" ex ey)
  let stage := min 6 ((st.finalecount - 1180) / 5)
  named wad fb s!"END{stage}" ex ey

/-- `F_ArtScreenDrawer`. -/
def artScreen (wad : WadDirectory) (fb : Framebuffer) (st : State) : Except String Framebuffer :=
  if st.gameepisode == 3 then bunnyScroll wad fb st
  else
    let name := if st.gameepisode == 1 then (if st.retail then "CREDIT" else "HELP2")
      else if st.gameepisode == 2 then "VICTORY2"
      else if st.gameepisode == 4 then "ENDPIC" else ""
    if name.isEmpty then pure fb else named wad fb name 0 0

/-- `F_Drawer` into `fb`. -/
def draw (wad : WadDirectory) (data : Doom.Render.Data.RenderData) (font : Font) (fb : Framebuffer)
    (st : State) : Except String Framebuffer :=
  match st.stage with
  | .cast => castDrawer wad data font fb st
  | .text => textWrite wad font fb st
  | .artscreen => artScreen wad fb st

end Doom.Front.Finale
