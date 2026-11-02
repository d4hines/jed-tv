import Doom.Front.Draw
import Doom.Playsim.Fixed
import Doom.Playsim.GameState
import Doom.Playsim.Tables
import Doom.Render.Types
import Doom.Wad

/-!
# Doom.Front.Automap

The automap (`am_map.c`): `respond` (`AM_Responder`), `tick` (`AM_Ticker`)
and `draw` (`AM_Drawer`), with `AM_Start` / `AM_Stop` behind the toggle key.
Follow mode, zoom and pan, maximum zoom toggle, grid, marks, the `iddt`
cheat levels, walls coloured by kind (one-sided, floor and ceiling changes,
teleporters, secret doors, and grey for unseen lines with the computer map),
player arrows (coloured in net games) and the crosshair.

`am_map.c`'s statics live in `State`. The seen-lines flag `ML_MAPPED` is set
by the renderer in vanilla (`R_StoreWallRange`); here the caller passes it as
a byte per line (see `Doom.Render.Frame`), combined with lines the map
itself marks as mapped. Messages for `plr->message` come back as strings.
Everything is `Int32` arithmetic, wrapping like the C `int`s.
-/

namespace Doom.Front.Automap

open Doom.Front.Draw
open Doom.Playsim.Fixed
open Doom.Playsim.GameState
open Doom.Render.Types
open Doom.Wad

/-! ## Colours and constants -/

def REDS : UInt8 := 256 - 5 * 16
def REDRANGE : UInt8 := 16
def GREENS : UInt8 := 7 * 16
def GRAYS : UInt8 := 6 * 16
def GRAYSRANGE : UInt8 := 16
def BROWNS : UInt8 := 4 * 16
def YELLOWS : UInt8 := 256 - 32 + 7
def WHITE : UInt8 := 256 - 47
def BACKGROUND : UInt8 := 0
def WALLCOLORS : UInt8 := REDS
def WALLRANGE : UInt8 := REDRANGE
def TSWALLCOLORS : UInt8 := GRAYS
def FDWALLCOLORS : UInt8 := BROWNS
def CDWALLCOLORS : UInt8 := YELLOWS
def THINGCOLORS : UInt8 := GREENS
def SECRETWALLCOLORS : UInt8 := WALLCOLORS
def GRIDCOLORS : UInt8 := GRAYS + GRAYSRANGE / 2
def XHAIRCOLORS : UInt8 := GRAYS

def AM_NUMMARKPOINTS : Nat := 10
/-- `INITSCALEMTOF` = `.2*FRACUNIT`. -/
def INITSCALEMTOF : Int32 := 13107
def F_PANINC : Int32 := 4
/-- `M_ZOOMIN` = `(int)(1.02*FRACUNIT)`. -/
def M_ZOOMIN : Int32 := 66846
/-- `M_ZOOMOUT` = `(int)(FRACUNIT/1.02)`. -/
def M_ZOOMOUT : Int32 := 64250
def PLAYERRADIUS : Int32 := 16 * FRACUNIT
def MAPBLOCKUNITS : Int32 := 128
def INT_MAX : Int32 := 0x7fffffff

/-! `linedef` flags (`doomdata.h`). -/
def ML_SECRET : Int32 := 32
def ML_DONTDRAW : Int32 := 128
def ML_MAPPED : Int32 := 256

/-! Default key bindings (`m_controls.c`). -/
def KEY_TAB : Nat := 9
def KEY_RIGHTARROW : Nat := 0xae
def KEY_LEFTARROW : Nat := 0xac
def KEY_UPARROW : Nat := 0xad
def KEY_DOWNARROW : Nat := 0xaf
def key_map_north : Nat := KEY_UPARROW
def key_map_south : Nat := KEY_DOWNARROW
def key_map_east : Nat := KEY_RIGHTARROW
def key_map_west : Nat := KEY_LEFTARROW
def key_map_zoomin : Nat := '='.toNat
def key_map_zoomout : Nat := '-'.toNat
def key_map_toggle : Nat := KEY_TAB
def key_map_maxzoom : Nat := '0'.toNat
def key_map_follow : Nat := 'f'.toNat
def key_map_grid : Nat := 'g'.toNat
def key_map_mark : Nat := 'm'.toNat
def key_map_clearmark : Nat := 'c'.toNat

/-! ## Vector characters -/

/-- A map-space line (`mline_t`). -/
structure MLine where
  ax : Int32
  ay : Int32
  bx : Int32
  by_ : Int32
  deriving Inhabited

private def ml (ax ay bx by_ : Int32) : MLine := { ax, ay, bx, by_ }

/-- `player_arrow`. -/
def playerArrow : Array MLine :=
  let r : Int32 := (8 * PLAYERRADIUS) / 7
  #[ml (-r + r / 8) 0 r 0,
    ml r 0 (r - r / 2) (r / 4),
    ml r 0 (r - r / 2) (-r / 4),
    ml (-r + r / 8) 0 (-r - r / 8) (r / 4),
    ml (-r + r / 8) 0 (-r - r / 8) (-r / 4),
    ml (-r + 3 * r / 8) 0 (-r + r / 8) (r / 4),
    ml (-r + 3 * r / 8) 0 (-r + r / 8) (-r / 4)]

/-- `cheat_player_arrow` ("ddt" inside the arrow). -/
def cheatPlayerArrow : Array MLine :=
  let r : Int32 := (8 * PLAYERRADIUS) / 7
  #[ml (-r + r / 8) 0 r 0,
    ml r 0 (r - r / 2) (r / 6),
    ml r 0 (r - r / 2) (-r / 6),
    ml (-r + r / 8) 0 (-r - r / 8) (r / 6),
    ml (-r + r / 8) 0 (-r - r / 8) (-r / 6),
    ml (-r + 3 * r / 8) 0 (-r + r / 8) (r / 6),
    ml (-r + 3 * r / 8) 0 (-r + r / 8) (-r / 6),
    ml (-r / 2) 0 (-r / 2) (-r / 6),
    ml (-r / 2) (-r / 6) (-r / 2 + r / 6) (-r / 6),
    ml (-r / 2 + r / 6) (-r / 6) (-r / 2 + r / 6) (r / 4),
    ml (-r / 6) 0 (-r / 6) (-r / 6),
    ml (-r / 6) (-r / 6) 0 (-r / 6),
    ml 0 (-r / 6) 0 (r / 4),
    ml (r / 6) (r / 4) (r / 6) (-r / 7),
    ml (r / 6) (-r / 7) (r / 6 + r / 32) (-r / 7 - r / 32),
    ml (r / 6 + r / 32) (-r / 7 - r / 32) (r / 6 + r / 10) (-r / 7)]

/-- `thintriangle_guy` (things, with `iddt` twice). -/
def thinTriangleGuy : Array MLine :=
  #[ml (-32768) (-45875) 65536 0,
    ml 65536 0 (-32768) 45875,
    ml (-32768) 45875 (-32768) (-45875)]

/-! ## State -/

/-- A mark (`mpoint_t`); `x = -1` is empty. -/
structure MPoint where
  x : Int32 := -1
  y : Int32 := 0
  deriving Inhabited

/-- `am_map.c` statics. -/
structure State where
  active : Bool := false
  stopped : Bool := true
  cheating : Nat := 0
  grid : Bool := false
  followplayer : Bool := true
  bigstate : Bool := false
  /-- `AM_Start`'s `lastlevel` / `lastepisode`. -/
  lastlevel : Int32 := -1
  lastepisode : Int32 := -1
  f_w : Int32 := 320
  f_h : Int32 := 200 - 32
  amclock : Int32 := 0
  lightlev : Int32 := 0
  m_paninc_x : Int32 := 0
  m_paninc_y : Int32 := 0
  mtof_zoommul : Int32 := FRACUNIT
  ftom_zoommul : Int32 := FRACUNIT
  m_x : Int32 := 0
  m_y : Int32 := 0
  m_x2 : Int32 := 0
  m_y2 : Int32 := 0
  m_w : Int32 := 0
  m_h : Int32 := 0
  min_x : Int32 := 0
  min_y : Int32 := 0
  max_x : Int32 := 0
  max_y : Int32 := 0
  max_w : Int32 := 0
  max_h : Int32 := 0
  min_w : Int32 := 0
  min_h : Int32 := 0
  min_scale_mtof : Int32 := 0
  max_scale_mtof : Int32 := 0
  old_m_w : Int32 := 0
  old_m_h : Int32 := 0
  old_m_x : Int32 := 0
  old_m_y : Int32 := 0
  f_oldloc_x : Int32 := 0
  f_oldloc_y : Int32 := 0
  scale_mtof : Int32 := INITSCALEMTOF
  scale_ftom : Int32 := 0
  /-- Index of the player shown (`plr`). -/
  plr : Nat := 0
  markpoints : Array MPoint := Array.replicate AM_NUMMARKPOINTS {}
  markpointnum : Nat := 0
  /-- `cheat_amap` ("iddt") characters matched so far. -/
  cheatRead : Nat := 0
  deriving Inhabited

@[inline] def shl16 (x : Int32) : Int32 := x <<< (16 : Int32)

/-- `FTOM`. -/
@[inline] def State.ftom (st : State) (x : Int32) : Int32 := fixedMul (shl16 x) st.scale_ftom
/-- `MTOF`. -/
@[inline] def State.mtof (st : State) (x : Int32) : Int32 := fixedMul x st.scale_mtof >>> (16 : Int32)
/-- `CXMTOF` (`f_x = 0`). -/
@[inline] def State.cxmtof (st : State) (x : Int32) : Int32 := st.mtof (x - st.m_x)
/-- `CYMTOF` (`f_y = 0`). -/
@[inline] def State.cymtof (st : State) (y : Int32) : Int32 := st.f_h - st.mtof (y - st.m_y)

/-- The player's map object position and angle. -/
private def plrMo (gs : GameState) (i : Nat) : Int32 × Int32 × UInt32 :=
  match gs.players[i]? with
  | some p => match gs.mobjs[p.mo.toNatClampNeg]? with
    | some m => if p.mo < 0 then (0, 0, 0) else (m.x, m.y, m.angle)
    | none => (0, 0, 0)
  | none => (0, 0, 0)

/-- `AM_activateNewScale`. -/
def activateNewScale (st : State) : State :=
  let mx := st.m_x + st.m_w / 2
  let my := st.m_y + st.m_h / 2
  let mw := st.ftom st.f_w
  let mh := st.ftom st.f_h
  let mx := mx - mw / 2
  let my := my - mh / 2
  { st with m_x := mx, m_y := my, m_w := mw, m_h := mh, m_x2 := mx + mw, m_y2 := my + mh }

/-- `AM_saveScaleAndLoc`. -/
def saveScaleAndLoc (st : State) : State :=
  { st with old_m_x := st.m_x, old_m_y := st.m_y, old_m_w := st.m_w, old_m_h := st.m_h }

/-- `AM_restoreScaleAndLoc`. -/
def restoreScaleAndLoc (st : State) (gs : GameState) : State :=
  let st := { st with m_w := st.old_m_w, m_h := st.old_m_h }
  let (px, py, _) := plrMo gs st.plr
  let st := if !st.followplayer then { st with m_x := st.old_m_x, m_y := st.old_m_y }
    else { st with m_x := px - st.m_w / 2, m_y := py - st.m_h / 2 }
  let st := { st with m_x2 := st.m_x + st.m_w, m_y2 := st.m_y + st.m_h }
  let s := fixedDiv (shl16 st.f_w) st.m_w
  { st with scale_mtof := s, scale_ftom := fixedDiv FRACUNIT s }

/-- `AM_addMark`. -/
def addMark (st : State) : State :=
  { st with markpoints := st.markpoints.set! st.markpointnum
                { x := st.m_x + st.m_w / 2, y := st.m_y + st.m_h / 2 }
            markpointnum := (st.markpointnum + 1) % AM_NUMMARKPOINTS }

/-- `AM_findMinMaxBoundaries` (including its `else if`: a vertex that lowers
the minimum is not checked against the maximum). -/
def findMinMaxBoundaries (st : State) (gs : GameState) : State := Id.run do
  let mut minx := INT_MAX
  let mut miny := INT_MAX
  let mut maxx := -INT_MAX
  let mut maxy := -INT_MAX
  for v in gs.level.vertexes do
    if v.x < minx then minx := v.x
    else if v.x > maxx then maxx := v.x
    if v.y < miny then miny := v.y
    else if v.y > maxy then maxy := v.y
  let maxw := maxx - minx
  let maxh := maxy - miny
  let a := fixedDiv (shl16 st.f_w) maxw
  let b := fixedDiv (shl16 st.f_h) maxh
  return { st with min_x := minx, min_y := miny, max_x := maxx, max_y := maxy,
                   max_w := maxw, max_h := maxh, min_w := 2 * PLAYERRADIUS, min_h := 2 * PLAYERRADIUS,
                   min_scale_mtof := if a < b then a else b,
                   max_scale_mtof := fixedDiv (shl16 st.f_h) (2 * PLAYERRADIUS) }

/-- `AM_changeWindowLoc`. -/
def changeWindowLoc (st0 : State) : State := Id.run do
  let mut st := st0
  if st.m_paninc_x != 0 || st.m_paninc_y != 0 then
    st := { st with followplayer := false, f_oldloc_x := INT_MAX }
  let mut mx := st.m_x + st.m_paninc_x
  let mut my := st.m_y + st.m_paninc_y
  if mx + st.m_w / 2 > st.max_x then mx := st.max_x - st.m_w / 2
  else if mx + st.m_w / 2 < st.min_x then mx := st.min_x - st.m_w / 2
  if my + st.m_h / 2 > st.max_y then my := st.max_y - st.m_h / 2
  else if my + st.m_h / 2 < st.min_y then my := st.min_y - st.m_h / 2
  return { st with m_x := mx, m_y := my, m_x2 := mx + st.m_w, m_y2 := my + st.m_h }

/-- `AM_initVariables`. -/
def initVariables (st0 : State) (gs : GameState) : State :=
  let st := { st0 with active := true, f_oldloc_x := INT_MAX, amclock := 0, lightlev := 0,
                       m_paninc_x := 0, m_paninc_y := 0,
                       ftom_zoommul := FRACUNIT, mtof_zoommul := FRACUNIT }
  let st := { st with m_w := st.ftom st.f_w, m_h := st.ftom st.f_h }
  -- The console player, or the first player in the game.
  let plr := if gs.playeringame.getD gs.consoleplayer false then gs.consoleplayer
    else ((List.range 4).find? (gs.playeringame.getD · false)).getD 0
  let (px, py, _) := plrMo gs plr
  let st := changeWindowLoc { st with plr, m_x := px - st.m_w / 2, m_y := py - st.m_h / 2 }
  { st with old_m_x := st.m_x, old_m_y := st.m_y, old_m_w := st.m_w, old_m_h := st.m_h }

/-- `AM_clearMarks`. -/
def clearMarks (st : State) : State :=
  { st with markpoints := Array.replicate AM_NUMMARKPOINTS {}, markpointnum := 0 }

/-- `AM_LevelInit`. -/
def levelInit (st0 : State) (gs : GameState) : State :=
  let st := findMinMaxBoundaries (clearMarks { st0 with f_w := 320, f_h := 200 - 32 }) gs
  let s := fixedDiv st.min_scale_mtof 45875  -- (int)(0.7*FRACUNIT)
  let s := if s > st.max_scale_mtof then st.min_scale_mtof else s
  { st with scale_mtof := s, scale_ftom := fixedDiv FRACUNIT s }

/-- `AM_Stop`. -/
def stop (st : State) : State := { st with active := false, stopped := true }

/-- `AM_Start`. -/
def start (st0 : State) (gs : GameState) : State :=
  let st := if !st0.stopped then stop st0 else st0
  let st := { st with stopped := false }
  let st := if st.lastlevel != gs.gamemap || st.lastepisode != gs.gameepisode then
      { levelInit st gs with lastlevel := gs.gamemap, lastepisode := gs.gameepisode }
    else st
  initVariables st gs

/-- `AM_minOutWindowScale`. -/
def minOutWindowScale (st : State) : State :=
  activateNewScale { st with scale_mtof := st.min_scale_mtof,
                             scale_ftom := fixedDiv FRACUNIT st.min_scale_mtof }

/-- `AM_maxOutWindowScale`. -/
def maxOutWindowScale (st : State) : State :=
  activateNewScale { st with scale_mtof := st.max_scale_mtof,
                             scale_ftom := fixedDiv FRACUNIT st.max_scale_mtof }

/-- `cht_CheckCheat(&cheat_amap, key)` for "iddt". -/
private def checkCheat (st : State) (key : Nat) : State × Bool :=
  let seq := "iddt".toList.map (·.toNat)
  let read := if seq[st.cheatRead]? == some key then st.cheatRead + 1 else 0
  if read >= seq.length then ({ st with cheatRead := 0 }, true) else ({ st with cheatRead := read }, false)

/-- Result of `AM_Responder`. -/
structure Response where
  state : State
  /-- The key was used by the automap. -/
  eaten : Bool
  /-- New `plr->message`. -/
  message : Option String := none

/-- `AM_Responder` for one key event (`down` = key press). `amstr` gives the
(possibly DEHACKED) automap message for a BEX mnemonic. -/
def respond (st : State) (gs : GameState) (key : Nat) (down : Bool)
    (amstr : String → String := id) : Response := Id.run do
  if !st.active then
    if down && key == key_map_toggle then return { state := start st gs, eaten := true }
    return { state := st, eaten := false }
  if !down then
    let mut st := st
    if key == key_map_east || key == key_map_west then
      if !st.followplayer then st := { st with m_paninc_x := 0 }
    else if key == key_map_north || key == key_map_south then
      if !st.followplayer then st := { st with m_paninc_y := 0 }
    else if key == key_map_zoomout || key == key_map_zoomin then
      st := { st with mtof_zoommul := FRACUNIT, ftom_zoommul := FRACUNIT }
    return { state := st, eaten := false }
  let mut st := st
  let mut rc := true
  let mut msg : Option String := none
  if key == key_map_east then
    if !st.followplayer then st := { st with m_paninc_x := st.ftom F_PANINC } else rc := false
  else if key == key_map_west then
    if !st.followplayer then st := { st with m_paninc_x := -st.ftom F_PANINC } else rc := false
  else if key == key_map_north then
    if !st.followplayer then st := { st with m_paninc_y := st.ftom F_PANINC } else rc := false
  else if key == key_map_south then
    if !st.followplayer then st := { st with m_paninc_y := -st.ftom F_PANINC } else rc := false
  else if key == key_map_zoomout then
    st := { st with mtof_zoommul := M_ZOOMOUT, ftom_zoommul := M_ZOOMIN }
  else if key == key_map_zoomin then
    st := { st with mtof_zoommul := M_ZOOMIN, ftom_zoommul := M_ZOOMOUT }
  else if key == key_map_toggle then
    st := stop { st with bigstate := false }
  else if key == key_map_maxzoom then
    st := { st with bigstate := !st.bigstate }
    st := if st.bigstate then minOutWindowScale (saveScaleAndLoc st) else restoreScaleAndLoc st gs
  else if key == key_map_follow then
    st := { st with followplayer := !st.followplayer, f_oldloc_x := INT_MAX }
    msg := some (amstr (if st.followplayer then "AMSTR_FOLLOWON" else "AMSTR_FOLLOWOFF"))
  else if key == key_map_grid then
    st := { st with grid := !st.grid }
    msg := some (amstr (if st.grid then "AMSTR_GRIDON" else "AMSTR_GRIDOFF"))
  else if key == key_map_mark then
    -- `M_snprintf` into a 20-byte buffer: at most 19 characters.
    let full := s!"{amstr "AMSTR_MARKEDSPOT"} {st.markpointnum}"
    msg := some (String.fromUTF8? (full.toUTF8.extract 0 19) |>.getD full)
    st := addMark st
  else if key == key_map_clearmark then
    st := clearMarks st
    msg := some (amstr "AMSTR_MARKSCLEARED")
  else rc := false
  if !gs.deathmatch then
    let (st', hit) := checkCheat st key
    st := st'
    if hit then
      rc := false
      st := { st with cheating := (st.cheating + 1) % 3 }
  return { state := st, eaten := rc, message := msg }

/-- `AM_changeWindowScale`. -/
def changeWindowScale (st : State) : State :=
  let s := fixedMul st.scale_mtof st.mtof_zoommul
  let st := { st with scale_mtof := s, scale_ftom := fixedDiv FRACUNIT s }
  if s < st.min_scale_mtof then minOutWindowScale st
  else if s > st.max_scale_mtof then maxOutWindowScale st
  else activateNewScale st

/-- `AM_doFollowPlayer`. -/
def doFollowPlayer (st : State) (gs : GameState) : State :=
  let (px, py, _) := plrMo gs st.plr
  if st.f_oldloc_x != px || st.f_oldloc_y != py then
    let mx := st.ftom (st.mtof px) - st.m_w / 2
    let my := st.ftom (st.mtof py) - st.m_h / 2
    { st with m_x := mx, m_y := my, m_x2 := mx + st.m_w, m_y2 := my + st.m_h,
              f_oldloc_x := px, f_oldloc_y := py }
  else st

/-- `AM_Ticker`. -/
def tick (st0 : State) (gs : GameState) : State := Id.run do
  if !st0.active then return st0
  let mut st := { st0 with amclock := st0.amclock + 1 }
  if st.followplayer then st := doFollowPlayer st gs
  if st.ftom_zoommul != FRACUNIT then st := changeWindowScale st
  if st.m_paninc_x != 0 || st.m_paninc_y != 0 then st := changeWindowLoc st
  return st

/-! ## Drawing -/

/-- `AM_clipMline` then `AM_drawFline` (Cohen-Sutherland, then Bresenham). -/
def drawMline (st : State) (px : ByteArray) (l : MLine) (color : UInt8) : ByteArray := Id.run do
  let LEFT : UInt32 := 1
  let RIGHT : UInt32 := 2
  let BOTTOM : UInt32 := 4
  let TOP : UInt32 := 8
  let fw := st.f_w
  let fh := st.f_h
  let mut oc1 : UInt32 := 0
  let mut oc2 : UInt32 := 0
  if l.ay > st.m_y2 then oc1 := TOP else if l.ay < st.m_y then oc1 := BOTTOM
  if l.by_ > st.m_y2 then oc2 := TOP else if l.by_ < st.m_y then oc2 := BOTTOM
  if oc1 &&& oc2 != 0 then return px
  if l.ax < st.m_x then oc1 := oc1 ||| LEFT else if l.ax > st.m_x2 then oc1 := oc1 ||| RIGHT
  if l.bx < st.m_x then oc2 := oc2 ||| LEFT else if l.bx > st.m_x2 then oc2 := oc2 ||| RIGHT
  if oc1 &&& oc2 != 0 then return px
  let outcode (x y : Int32) : UInt32 :=
    (if y < 0 then TOP else if y >= fh then BOTTOM else 0) |||
    (if x < 0 then LEFT else if x >= fw then RIGHT else 0)
  let mut ax := st.cxmtof l.ax
  let mut ay := st.cymtof l.ay
  let mut bx := st.cxmtof l.bx
  let mut by_ := st.cymtof l.by_
  oc1 := outcode ax ay
  oc2 := outcode bx by_
  if oc1 &&& oc2 != 0 then return px
  let mut guard := 8
  while (oc1 ||| oc2) != 0 && guard > 0 do
    guard := guard - 1
    let outside := if oc1 != 0 then oc1 else oc2
    let mut tx : Int32 := 0
    let mut ty : Int32 := 0
    if outside &&& TOP != 0 then
      let dy := ay - by_
      let dx := bx - ax
      tx := ax + (dx * ay) / dy
      ty := 0
    else if outside &&& BOTTOM != 0 then
      let dy := ay - by_
      let dx := bx - ax
      tx := ax + (dx * (ay - fh)) / dy
      ty := fh - 1
    else if outside &&& RIGHT != 0 then
      let dy := by_ - ay
      let dx := bx - ax
      ty := ay + (dy * (fw - 1 - ax)) / dx
      tx := fw - 1
    else if outside &&& LEFT != 0 then
      let dy := by_ - ay
      let dx := bx - ax
      ty := ay + (dy * (-ax)) / dx
      tx := 0
    if outside == oc1 then
      ax := tx; ay := ty
      oc1 := outcode ax ay
    else
      bx := tx; by_ := ty
      oc2 := outcode bx by_
    if oc1 &&& oc2 != 0 then return px
  -- `AM_drawFline`'s sanity check.
  if ax < 0 || ax >= fw || ay < 0 || ay >= fh || bx < 0 || bx >= fw || by_ < 0 || by_ >= fh then
    return px
  let w := fw.toNatClampNeg
  let dx := bx - ax
  let axx : Int32 := 2 * (if dx < 0 then -dx else dx)
  let sx : Int32 := if dx < 0 then -1 else 1
  let dy := by_ - ay
  let ayy : Int32 := 2 * (if dy < 0 then -dy else dy)
  let sy : Int32 := if dy < 0 then -1 else 1
  let mut x := ax
  let mut y := ay
  let mut px := px
  let mut fuel := 2 * (w + fh.toNatClampNeg) + 4
  if axx > ayy then
    let mut d := ayy - axx / 2
    while fuel > 0 do
      fuel := fuel - 1
      px := px.set! (y.toNatClampNeg * w + x.toNatClampNeg) color
      if x == bx then break
      if d >= 0 then
        y := y + sy
        d := d - axx
      x := x + sx
      d := d + ayy
  else
    let mut d := axx - ayy / 2
    while fuel > 0 do
      fuel := fuel - 1
      px := px.set! (y.toNatClampNeg * w + x.toNatClampNeg) color
      if y == by_ then break
      if d >= 0 then
        x := x + sx
        d := d - ayy
      y := y + sy
      d := d + axx
  return px

/-- `AM_drawGrid`: blockmap-aligned grid lines. -/
def drawGrid (st : State) (gs : GameState) (px : ByteArray) (color : UInt8) : ByteArray := Id.run do
  let unit : Int32 := shl16 MAPBLOCKUNITS
  let orgx := gs.level.blockmap.originX
  let orgy := gs.level.blockmap.originY
  let mut px := px
  let mut start := st.m_x
  if (start - orgx) % unit != 0 then start := start + unit - (start - orgx) % unit
  let mut x := start
  let mut fuel := 4096
  while x < st.m_x + st.m_w && fuel > 0 do
    fuel := fuel - 1
    px := drawMline st px { ax := x, ay := st.m_y, bx := x, by_ := st.m_y + st.m_h } color
    x := x + unit
  start := st.m_y
  if (start - orgy) % unit != 0 then start := start + unit - (start - orgy) % unit
  let mut y := start
  fuel := 4096
  while y < st.m_y + st.m_h && fuel > 0 do
    fuel := fuel - 1
    px := drawMline st px { ax := st.m_x, ay := y, bx := st.m_x + st.m_w, by_ := y } color
    y := y + unit
  return px

/-- `AM_drawWalls`. `mapped[i] ≠ 0` when the renderer has drawn line `i`. -/
def drawWalls (st : State) (gs : GameState) (mapped : ByteArray) (px : ByteArray) : ByteArray := Id.run do
  let lines := gs.level.lines
  let verts := gs.level.vertexes
  let sectors := gs.sectors
  let lightlev := st.lightlev.toUInt32.toUInt8
  let cheating := st.cheating != 0
  let allmap := match gs.players[st.plr]? with
    | some p => (p.powers[Doom.Playsim.Player.pw_allmap]?.getD 0) != 0
    | none => false
  let mut px := px
  for h : i in [0:lines.size] do
    let ld := lines[i]'h.upper
    let some v1 := verts[ld.v1.toNat]? | continue
    let some v2 := verts[ld.v2.toNat]? | continue
    let l : MLine := { ax := v1.x, ay := v1.y, bx := v2.x, by_ := v2.y }
    let isMapped := (ld.flags &&& ML_MAPPED) != 0 || (mapped[i]?.getD 0) != 0
    if cheating || isMapped then
      if (ld.flags &&& ML_DONTDRAW) != 0 && !cheating then continue
      if ld.backsector < 0 then
        px := drawMline st px l (WALLCOLORS + lightlev)
      else if ld.special == 39 then
        px := drawMline st px l (WALLCOLORS + WALLRANGE / 2)  -- teleporters
      else if (ld.flags &&& ML_SECRET) != 0 then
        px := drawMline st px l ((if cheating then SECRETWALLCOLORS else WALLCOLORS) + lightlev)
      else
        let some back := sectors[ld.backsector.toNatClampNeg]? | continue
        let some front := sectors[ld.frontsector.toNatClampNeg]? | continue
        if back.floorheight != front.floorheight then
          px := drawMline st px l (FDWALLCOLORS + lightlev)
        else if back.ceilingheight != front.ceilingheight then
          px := drawMline st px l (CDWALLCOLORS + lightlev)
        else if cheating then
          px := drawMline st px l (TSWALLCOLORS + lightlev)
    else if allmap then
      if (ld.flags &&& ML_DONTDRAW) == 0 then px := drawMline st px l (GRAYS + 3)
  return px

/-- `AM_rotate`. -/
@[inline] def rotate (x y : Int32) (a : UInt32) : Int32 × Int32 :=
  let i := (a >>> 19).toNat
  let c := Doom.Playsim.Tables.finecosine[i]!
  let s := Doom.Playsim.Tables.finesine[i]!
  (fixedMul x c - fixedMul y s, fixedMul x s + fixedMul y c)

/-- `AM_drawLineCharacter`. -/
def drawLineCharacter (st : State) (px : ByteArray) (guy : Array MLine) (scale : Int32) (angle : UInt32)
    (color : UInt8) (x y : Int32) : ByteArray := Id.run do
  let mut px := px
  for g in guy do
    let mut ax := g.ax
    let mut ay := g.ay
    let mut bx := g.bx
    let mut by_ := g.by_
    if scale != 0 then
      ax := fixedMul scale ax; ay := fixedMul scale ay
      bx := fixedMul scale bx; by_ := fixedMul scale by_
    if angle != 0 then
      let (rx, ry) := rotate ax ay angle
      ax := rx; ay := ry
      let (rx, ry) := rotate bx by_ angle
      bx := rx; by_ := ry
    px := drawMline st px { ax := ax + x, ay := ay + y, bx := bx + x, by_ := by_ + y } color
  return px

/-- `AM_drawPlayers`. -/
def drawPlayers (st : State) (gs : GameState) (px : ByteArray) : ByteArray := Id.run do
  if !gs.netgame then
    let (x, y, a) := plrMo gs st.plr
    return drawLineCharacter st px (if st.cheating != 0 then cheatPlayerArrow else playerArrow) 0 a WHITE x y
  let theirColors : Array UInt8 := #[GREENS, GRAYS, BROWNS, REDS]
  let mut px := px
  for i in [0:4] do
    -- `singledemo` is never set here.
    if gs.deathmatch && i != st.plr then continue
    if !gs.playeringame.getD i false then continue
    let some p := gs.players[i]? | continue
    let color : UInt8 := if (p.powers[Doom.Playsim.Player.pw_invisibility]?.getD 0) != 0 then 246
      else theirColors[i]!
    let (x, y, a) := plrMo gs i
    px := drawLineCharacter st px playerArrow 0 a color x y
  return px

/-- `AM_drawThings` (`iddt` twice): every thing as a small triangle. -/
def drawThings (st : State) (gs : GameState) (px : ByteArray) (color : UInt8) : ByteArray := Id.run do
  let mut px := px
  for sec in gs.sectors do
    let mut t := sec.thinglist
    let mut fuel := gs.mobjs.size + 1
    while t >= 0 && fuel > 0 do
      fuel := fuel - 1
      let some m := gs.mobjs[t.toNatClampNeg]? | break
      px := drawLineCharacter st px thinTriangleGuy (shl16 16) m.angle (color + st.lightlev.toUInt32.toUInt8) m.x m.y
      t := m.snext
  return px

/-- `AM_drawMarks`: the `AMMNUMn` digits at each mark. -/
def drawMarks (st : State) (wad : WadDirectory) (fb : Framebuffer) : Except String Framebuffer := do
  let mut fb := fb
  for i in [0:AM_NUMMARKPOINTS] do
    let mp := st.markpoints[i]!
    if mp.x != -1 then
      let w : Int32 := 5  -- "because something's wrong with the wad, i guess"
      let h : Int32 := 6
      let fx := st.cxmtof mp.x
      let fy := st.cymtof mp.y
      if fx >= 0 && fx <= st.f_w - w && fy >= 0 && fy <= st.f_h - h then
        fb ← named wad fb s!"AMMNUM{i}" fx fy
  return fb

/-- `AM_Drawer` into the top `f_h` rows of `fb`. -/
def draw (st : State) (gs : GameState) (mapped : ByteArray) (wad : WadDirectory) (fb : Framebuffer) :
    Except String Framebuffer := do
  if !st.active then return fb
  let fb := clearTop fb st.f_w.toNatClampNeg st.f_h.toNatClampNeg BACKGROUND
  let mut px := fb.pixels
  if st.grid then px := drawGrid st gs px GRIDCOLORS
  px := drawWalls st gs mapped px
  px := drawPlayers st gs px
  if st.cheating == 2 then px := drawThings st gs px THINGCOLORS
  -- `AM_drawCrosshair`: one dot.
  px := px.set! ((st.f_w * (st.f_h + 1)) / 2).toNatClampNeg XHAIRCOLORS
  drawMarks st wad { pixels := px }

end Doom.Front.Automap
