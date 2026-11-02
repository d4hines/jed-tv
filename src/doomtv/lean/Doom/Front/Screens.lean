import Doom.Front.Automap
import Doom.Front.Deh
import Doom.Front.Draw
import Doom.Front.Finale
import Doom.Front.Intermission
import Doom.Game
import Doom.Playsim.GameState
import Doom.Render.Frame

/-!
# Doom.Front.Screens

The front-end screens for a game loop: the automap, and the drawers of the
intermission and finale. The game flow itself (`G_Ticker`: level exits,
`WI_Ticker`, `F_Ticker`, the cast, music) is the playsim's (`Doom.Game`);
this module only draws its state and handles the keys the front end owns.

- `respond`: the front-end part of `G_Responder` after `M_Responder`:
  `AM_Responder` in a level, `F_CastResponder` during the Doom II cast.
- `tic`: `AM_Ticker` (and closing the map when the level ends).
- `render`: `D_Display` for the level (or automap), intermission and finale.
-/

namespace Doom.Front.Screens

open Doom.Playsim.GameState
open Doom.Render.Types

structure State where
  deh : Deh.Strings := {}
  font : Draw.Font := {}
  /-- `gamemode == commercial`. -/
  commercial : Bool := false
  /-- `gameversion >= exe_ultimate`. -/
  retail : Bool := false
  automap : Automap.State := {}
  deriving Inhabited

/-- Kept for the game loop's interface; the playsim now emits sounds and
music itself, so the screens produce none. -/
inductive Event
  | sound (sfx : Nat)
  | music (name : String) (looping : Bool)
  | worldDone
  deriving BEq, Repr, Inhabited

/-- Load the IWAD's strings and font (`HU_Init`, `LoadIwadDeh`). -/
def State.init (wad : Doom.Wad.WadDirectory) (commercial retail : Bool) : State :=
  { deh := Deh.load wad, font := Draw.loadFont wad, commercial, retail }

/-- `G_InitNew`: a new game closes the map. -/
def State.newGame (st : State) : State :=
  { st with automap := { st.automap with active := false } }

/-- The front-end responders for one key (`down` = press). Returns whether
the key was eaten; automap messages go to `plr->message`. -/
def State.respond (st : State) (gs : GameState) (key : Nat) (down : Bool) :
    State × GameState × Array Event × Bool :=
  if gs.gamestate == gamestateLevel then
    let r := Automap.respond st.automap gs key down st.deh.str
    let st := { st with automap := r.state }
    match r.message, gs.players[gs.consoleplayer]? with
    | some msg, some pl =>
      (st, { gs with players := gs.players.set! gs.consoleplayer { pl with message := some msg } }, #[], r.eaten)
    | _, _ => (st, gs, #[], r.eaten)
  else if gs.gamestate == gamestateFinale && down && gs.finale.finalestage == Doom.Playsim.Finale.F_STAGE_CAST then
    (st, Doom.Game.castKeyPress gs, #[], true)
  else (st, gs, #[], false)

/-- After `G_Ticker`: `AM_Ticker`, and the map closes when the level ends
(`G_DoCompleted`, `F_StartFinale`). -/
def State.tic (st : State) (gs : GameState) : State × GameState × Array Event :=
  if gs.gamestate == gamestateLevel then ({ st with automap := Automap.tick st.automap gs }, gs, #[])
  else if st.automap.active then ({ st with automap := Automap.stop st.automap }, gs, #[])
  else (st, gs, #[])

/-- The finale drawer's view of the playsim's `f_finale.c` state: text, flat
and cast names resolved for this episode/map, stage and counters copied. -/
def finaleView (st : State) (gs : GameState) : Finale.State :=
  let (base, _) := Finale.start st.deh st.commercial st.retail gs.gameepisode gs.gamemap
  let f := gs.finale
  { base with
    stage := if f.finalestage == Doom.Playsim.Finale.F_STAGE_CAST then .cast
      else if f.finalestage == Doom.Playsim.Finale.F_STAGE_ARTSCREEN then .artscreen else .text
    finalecount := f.finalecount, castnum := f.castnum.toNatClampNeg, caststate := f.caststate
    casttics := f.casttics, castdeath := f.castdeath, castframes := f.castframes
    castonmelee := f.castonmelee, castattacking := f.castattacking }

/-- `D_Display` for a game in progress (the menu goes on top afterwards). -/
def State.render (st : State) (data : Doom.Render.Data.RenderData) (wad : Doom.Wad.WadDirectory)
    (gs : GameState) (c : Doom.Render.Frame.Cache) (screenblocks : Nat) :
    Except String (ByteArray × Doom.Render.Frame.Cache) :=
  if gs.gamestate == gamestateLevel && st.automap.active then
    Doom.Render.Frame.renderAutomap data wad gs c st.automap
      (st.deh.levelTitle st.commercial gs.gameepisode gs.gamemap)
  else if gs.gamestate == gamestateIntermission then
    Doom.Render.Frame.renderIntermission data wad c (Intermission.view gs {} st.commercial st.retail)
  else if gs.gamestate == gamestateFinale then
    Doom.Render.Frame.renderFinale data wad st.font c (finaleView st gs)
  else Doom.Render.Frame.render data wad gs c screenblocks

end Doom.Front.Screens
