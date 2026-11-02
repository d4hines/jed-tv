import Doom.Front.Deh
import Doom.Front.Finale
import Doom.Front.Intermission
import Doom.Playsim.GameState
import Doom.Render.Hud
import Doom.Render.Palette
import Doom.Render.Types
import Doom.Render.Video
import Doom.Wad

/-!
# Doom.Render.Screens

Title, intermission and finale frames from the game state alone (the
stateless `D_Display` path of `Doom.Render.Render.renderDisplay`). The live
app draws through `Doom.Render.Frame` with the front-end state of
`Doom.Front.Intermission` / `Doom.Front.Finale`; here the intermission has
only the playsim's background animation and the finale shows its full text.
Not part of the playsim identity definition.
-/

namespace Doom.Render.Screens

open Doom.Playsim.GameState
open Doom.Render.Hud
open Doom.Render.Palette
open Doom.Render.Types
open Doom.Render.Video
open Doom.Wad

private def drawNamed (wad : WadDirectory) (fb : Framebuffer) (name : String)
    (x y : Int32) : Except String Framebuffer := do
  match checkNumForName wad name with
  | none => pure fb
  | some idx => drawPatch wad fb idx x y

def drawTitle (wad : WadDirectory) : Except String (Framebuffer × ByteArray) := do
  let palLump ← loadPlaypal wad
  let pal ← playpalSlice palLump 0
  let fb0 := Framebuffer.initBlack
  let fb ← drawNamed wad fb0 "TITLEPIC" 0 0
  let fb ← drawTextLine wad fb 8 180 "PRESS ENTER"
  pure (fb, pal)

def drawIntermission (wad : WadDirectory) (gs : GameState) :
    Except String (Framebuffer × ByteArray) := do
  let palLump ← loadPlaypal wad
  let pal ← playpalSlice palLump 0
  let retail := (checkNumForName wad "E4M1").isSome
  let v := Doom.Front.Intermission.view gs {} gs.commercial retail
  let fb ← Doom.Front.Intermission.draw wad Framebuffer.initBlack v
  pure (fb, pal)

def drawFinale (wad : WadDirectory) (gs : GameState) : Except String (Framebuffer × ByteArray) := do
  let palLump ← loadPlaypal wad
  let pal ← playpalSlice palLump 0
  let retail := (checkNumForName wad "E4M1").isSome
  let (st, _) := Doom.Front.Finale.start (Doom.Front.Deh.load wad) gs.commercial retail
    gs.gameepisode gs.gamemap
  let st := { st with finalecount := 10 + 3 * Int32.ofNat st.text.utf8ByteSize }
  let fb ← Doom.Front.Finale.textWrite wad (Doom.Front.Draw.loadFont wad) Framebuffer.initBlack st
  pure (fb, pal)

end Doom.Render.Screens
