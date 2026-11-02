import Doom.Front.Automap
import Doom.Front.Deh
import Doom.Front.Finale
import Doom.Front.Intermission
import Doom.Render.Render

/-!
# Doom.Render.Frame

`D_Display` for the live game with renderer tables built once per WAD
(`R_InitData`), rather than per frame as the trace harness does.
-/

namespace Doom.Render.Frame

open Doom.Playsim.GameState
open Doom.Render.Constants
open Doom.Render.Data
open Doom.Render.Draw
open Doom.Render.Main
open Doom.Render.Palette
open Doom.Render.Render
open Doom.Render.Types
open Doom.Render.View
open Doom.Wad

/-- State the live display keeps between frames. Like vanilla's screen
buffer, the framebuffer is reused: every level frame repaints the status bar,
border and view, so nothing stale survives except where vanilla also shows
the previous frame (HOM). -/
structure Cache where
  fb : Framebuffer := Framebuffer.initBlack
  /-- `R_FillBackScreen` output for `backKey` (view width, height, commercial). -/
  back : Option Framebuffer := none
  backKey : Int32 × Int32 × Bool := (0, 0, false)
  fuzzpos : Nat := 0
  /-- `R_ExecuteSetViewSize` result for `viewBlocks` (its tables are costly). -/
  view : Option ViewSize := none
  viewBlocks : Nat := 0
  /-- `ML_MAPPED` per line of the current level: set when the view draws a
  wall of that line (`R_StoreWallRange`), read by the automap. -/
  mapped : ByteArray := ByteArray.empty
  /-- The level `mapped` belongs to: episode, map, and the `leveltime` last
  seen (it only grows within one level). -/
  mappedEpisode : Int32 := -1
  mappedMap : Int32 := -1
  mappedTime : UInt32 := 0

/-- The framebuffer taken out of the cache, so it stays uniquely referenced
while it is drawn into. -/
@[inline] def Cache.takeFb (c : Cache) : Framebuffer × Cache :=
  (c.fb, { c with fb := { pixels := ByteArray.empty } })

/-- The mapped-lines bytes for `gs`'s level, cleared when the level changes. -/
def levelMapped (c : Cache) (gs : GameState) : Cache :=
  if c.mapped.size == gs.level.lines.size && c.mappedEpisode == gs.gameepisode &&
      c.mappedMap == gs.gamemap && gs.leveltime >= c.mappedTime then
    { c with mappedTime := gs.leveltime }
  else
    { c with mapped := ByteArray.mk (Array.replicate gs.level.lines.size 0),
             mappedEpisode := gs.gameepisode, mappedMap := gs.gamemap, mappedTime := gs.leveltime }

/-- Mark the lines of this frame's drawsegs (`linedef->flags |= ML_MAPPED`). -/
def markMapped (c0 : Cache) (gs : GameState) (segs : Array Doom.Render.Bsp.DrawSegRecord) : Cache :=
  let c := levelMapped c0 gs
  let m0 := c.mapped
  let c := { c with mapped := ByteArray.empty }
  let m := segs.foldl (fun (m : ByteArray) d =>
    let i := d.replayMeta.linedef.toNat
    if i < m.size then m.set! i 1 else m) m0
  { c with mapped := m }

/-- One level frame: status bar, 3D view, view border, HUD messages. -/
def renderLevel (data : RenderData) (wad : WadDirectory) (gs : GameState) (c0 : Cache)
    (screenblocks : Nat := screenBlocksDefault) : Except String (ByteArray × Cache) := do
  let pal ← playpalSlice data.playpal (Doom.Render.StatusBar.doPaletteStuff gs)
  let vs := match c0.view with
    | some v => if c0.viewBlocks == screenblocks then v else (executeSetViewSize (setViewSize (Int32.ofNat screenblocks) 0)).1
    | none => (executeSetViewSize (setViewSize (Int32.ofNat screenblocks) 0)).1
  let key := (vs.scaledviewwidth, vs.viewheight, gs.commercial)
  let (fb0, c) := c0.takeFb
  let back ← if c.back.isSome && c.backKey == key then pure c.back
    else fillBackScreen wad vs.scaledviewwidth vs.viewheight vs.viewwindowx vs.viewwindowy gs.commercial
  let fb1 ← Doom.Render.StatusBar.drawer wad fb0 gs (vs.viewheight == 200) true
  let (fb2, fp, segs) ← renderPlayerViewSegs data wad gs fb1 vs c.fuzzpos
  let fb3 := drawViewBorder fb2 back vs.scaledviewwidth vs.viewheight
  let fb4 ← Doom.Render.Hud.drawer wad gs fb3
  let c := markMapped c gs segs
  pure (pal, { c with fb := fb4, back, backKey := key, fuzzpos := fp, view := some vs, viewBlocks := screenblocks })

/-- The automap frame (`D_Display` with `automapactive`): `AM_Drawer`, the
status bar (always on with the map), `HU_Drawer` with the level title. -/
def renderAutomap (data : RenderData) (wad : WadDirectory) (gs : GameState) (c0 : Cache)
    (am : Doom.Front.Automap.State) (title : String) : Except String (ByteArray × Cache) := do
  let pal ← playpalSlice data.playpal (Doom.Render.StatusBar.doPaletteStuff gs)
  let (fb0, c) := (levelMapped c0 gs).takeFb
  let fb1 ← Doom.Front.Automap.draw am gs c.mapped wad fb0
  let fb2 ← Doom.Render.StatusBar.drawer wad fb1 gs false true
  let fb3 ← Doom.Render.Hud.drawer wad gs fb2
  -- `w_title` at (HU_TITLEX, HU_TITLEY = 167 - hu_font[0] height).
  let fontH := match checkNumForName wad (Doom.Render.Hud.stcfnName 33) with
    | some i => (Doom.Front.Draw.patchInfo wad i).height
    | none => 0
  let fb4 ← Doom.Render.Hud.drawTextLine wad fb3 0 (167 - fontH) title
  pure (pal, { c with fb := fb4 })

/-- `WI_Drawer` frame. -/
def renderIntermission (data : RenderData) (wad : WadDirectory) (c : Cache)
    (v : Doom.Front.Intermission.View) : Except String (ByteArray × Cache) := do
  let pal ← playpalSlice data.playpal 0
  let (fb0, c) := c.takeFb
  let fb ← Doom.Front.Intermission.draw wad fb0 v
  pure (pal, { c with fb })

/-- `F_Drawer` frame. -/
def renderFinale (data : RenderData) (wad : WadDirectory) (font : Doom.Front.Draw.Font) (c : Cache)
    (st : Doom.Front.Finale.State) : Except String (ByteArray × Cache) := do
  let pal ← playpalSlice data.playpal 0
  let (fb0, c) := c.takeFb
  let fb ← Doom.Front.Finale.draw wad data font fb0 st
  pure (pal, { c with fb })

/-- Live `D_Display`: level, intermission, finale, or title. The frame is
`cache.fb` of the result. -/
def render (data : RenderData) (wad : WadDirectory) (gs : GameState) (c : Cache)
    (screenblocks : Nat := screenBlocksDefault) : Except String (ByteArray × Cache) :=
  if gs.gamestate == gamestateLevel then renderLevel data wad gs c screenblocks
  else do
    let (fb, pal, fp) ← renderDisplay wad gs c.fuzzpos
    pure (pal, { c with fb, fuzzpos := fp })

/-- `D_PageDrawer`: a full-screen page (TITLEPIC, CREDIT, HELP*). -/
def renderPage (data : RenderData) (wad : WadDirectory) (name : String) (c : Cache) :
    Except String (ByteArray × Cache) := do
  let pal ← playpalSlice data.playpal 0
  let (fb0, c) := c.takeFb
  let fb ← match checkNumForName wad name with
    | some i => Doom.Render.Video.drawPatch wad fb0 i 0 0
    | none => pure fb0
  pure (pal, { c with fb })

end Doom.Render.Frame
