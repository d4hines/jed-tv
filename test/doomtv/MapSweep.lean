import Doom.Game
import Doom.Wad
import Doom.Playsim.Input
import Doom.Render.Frame
/-!
# map-sweep

Robustness sweep: load every map of an IWAD (ExMy or MAPxx), play it for N
tics with scripted input (walk, turn, strafe, fire, use) and render every 4th
tic. Reports per map whether it loaded, how far it ran, and the first engine
error. Not a correctness oracle (that is check-demos.sh); it finds maps or
specials the port cannot handle yet.

  map-sweep IWAD [TICS] [SKILL]
-/

open Doom.Playsim.GameState
open Doom.Playsim.Input
open Doom.Playsim.Player
open Doom.Render.Data
open Doom.Wad

def mapNames (wad : WadDirectory) : Array String := Id.run do
  let mut out := #[]
  for e in [1:5] do
    for m in [1:10] do
      let n := s!"E{e}M{m}"
      if (checkNumForName wad n).isSome then out := out.push n
  for m in [1:33] do
    let n := if m < 10 then s!"MAP0{m}" else s!"MAP{m}"
    if (checkNumForName wad n).isSome then out := out.push n
  return out

/-- Scripted keys for tic `t`: mostly forward with periodic turns, strafes,
fire and use, so doors, lifts and pickups get exercised. -/
def keysAt (t : Nat) : List Nat :=
  let phase := t % 140
  let base := [key_up]
  let turn := if phase < 20 then [key_left] else if phase ≥ 70 && phase < 85 then [key_right] else []
  let fire := if t % 9 < 3 then [key_fire] else []
  let use := if t % 23 == 0 then [key_use] else []
  let strafe := if phase ≥ 100 && phase < 110 then [key_strafe, key_left] else []
  base ++ turn ++ fire ++ use ++ strafe

def runMap (wad : WadDirectory) (data : RenderData) (name : String) (tics skill : Nat) :
    String := Id.run do
  let doom2 := name.startsWith "MAP"
  let ep := if doom2 then 1 else (name.drop 1).take 1 |>.toString.toNat?.getD 1
  let mp := (name.drop 3).toString.toNat?.getD 1
  let o : Doom.Game.NewGameOptions := { skill := skill.toUInt32.toInt32, episode := Int32.ofNat ep, map := Int32.ofNat mp }
  let mut gs ← match Doom.Game.newGame wad o with
    | .ok g => pure g
    | .error e => return s!"{name}: LOAD ERROR {e}"
  let mut inp := InputState.init
  let mut cache : Doom.Render.Frame.Cache := {}
  for t in [0:tics] do
    let mut i := InputState.init
    i := { i with turnheld := inp.turnheld }
    for k in keysAt t do i := gResponder i true k.toUInt32
    let (i', cmd) := buildTiccmd i
    inp := i'
    gs := Doom.Game.ticker gs #[cmd, TicCmd.zero, TicCmd.zero, TicCmd.zero]
    if let some e := gs.error then return s!"{name}: ENGINE ERROR at tic {t}: {e}"
    if gs.gamestate != gamestateLevel then
      return s!"{name}: ok, left the level at tic {t} (gamestate {gs.gamestate})"
    if t % 4 == 0 then
      let c := cache
      cache := {}
      match Doom.Render.Frame.render data wad gs c with
      | .error e => return s!"{name}: RENDER ERROR at tic {t}: {e}"
      | .ok (_, c') => cache := c'
  let health := (gs.players[gs.consoleplayer]?.map (·.health)).getD 0
  return s!"{name}: ok, {tics} tics (health {health})"

def main (args : List String) : IO UInt32 := do
  let some iwad := args.head? | IO.eprintln "usage: map-sweep IWAD [TICS] [SKILL]"; return 2
  let tics := (args.drop 1).head?.bind String.toNat? |>.getD 700
  let skill := (args.drop 2).head?.bind String.toNat? |>.getD 2
  let wad ← loadFile iwad
  let .ok data := initData wad | IO.eprintln "renderer init failed"; return 1
  let names := mapNames wad
  let mut bad := 0
  for n in names do
    let r := runMap wad data n tics skill
    IO.println r
    if !(r.splitOn ": ok").length > 1 then bad := bad + 1
  IO.println s!"map-sweep: {names.size - bad}/{names.size} maps ran without an engine error"
  return if bad == 0 then 0 else 1
