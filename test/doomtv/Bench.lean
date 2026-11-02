import Doom.Game
import DoomTest.Harness.Real
import Doom.Render.Frame
/-!
# bench

Host/target timing: replay a shareware demo through the playsim and render a
full frame (status bar, 3D view, border, HUD) after every tic, the way the
game loop will. Prints per-tic averages for playsim and rendering.
-/

open Doom.Harness.Real
open Doom.Playsim.GameState
open Doom.Render.Constants
open Doom.Render.Data
open Doom.Render.Draw
open Doom.Render.Main
open Doom.Render.Palette
open Doom.Render.Types
open Doom.Render.View
open Doom.Render.Gfx.Flat
open Doom.Wad

def main (args : List String) : IO UInt32 := do
  let iwad := args.headD "../../../assets/third_party/doom_lean/src/fixtures/wads/doom1.wad"
  let tics := (args.drop 1).head?.bind String.toNat? |>.getD 700
  -- `compare`: also render each frame with the trace harness's from-scratch
  -- D_Display and report frames that differ from the live cached path. The
  -- live path reuses the previous frame like vanilla's screen buffer, so
  -- pixels the renderer leaves unpainted (gaps/HOM) legitimately differ.
  let compare := args.contains "compare"
  let blocks := (args.findSome? fun a => if a.startsWith "blocks=" then (a.drop 7).toString.toNat? else none).getD 9
  let mut mismatches := 0
  let mut harnessFuzz := 0
  let wad ← loadFile iwad
  let t0 ← IO.monoNanosNow
  let .ok data := initData wad | IO.eprintln "initData failed"; return 1
  let t1 ← IO.monoNanosNow
  IO.println s!"initData: {(t1 - t0) / 1000000} ms"
  let some idx := checkNumForName wad "DEMO1" | return 1
  let .ok demo := lumpData wad idx | return 1
  -- Demo playback through the game API: the first tic loads the level.
  let .ok gs0 := Doom.Game.playDemo wad demo | IO.eprintln "playDemo failed"; return 1
  let mut gs := gs0
  let mut simNs : Nat := 0
  let mut renderNs : Nat := 0
  let mut cache : Doom.Render.Frame.Cache := {}
  let mut sum : UInt64 := 0
  for g in [0:tics] do
    let a ← IO.monoNanosNow
    gs := Doom.Game.ticker gs #[]
    if let some e := gs.error then IO.eprintln s!"tic {g}: {e}"; return 1
    let b ← IO.monoNanosNow
    let c := cache
    cache := {}
    match Doom.Render.Frame.render data wad gs c blocks with
    | .error e => IO.eprintln s!"render {g}: {e}"; return 1
    | .ok (pal, c) =>
      sum := sum + (c.fb.pixels[(g * 7919) % 64000]!).toUInt64
      if compare then
        match Doom.Render.Render.renderLevelFrameWithFuzz wad gs harnessFuzz with
        | .error e => IO.eprintln s!"harness render {g}: {e}"; return 1
        | .ok (fb, pal2, fp) =>
          harnessFuzz := fp
          if fb.pixels != c.fb.pixels || pal != pal2 then
            mismatches := mismatches + 1
            if mismatches ≤ 3 then
              let diff := (List.range 64000).filter (fun i => fb.pixels[i]! != c.fb.pixels[i]!)
              let ys := diff.map (· / 320)
              let xs := diff.map (· % 320)
              IO.eprintln s!"frame {g}: {diff.length} pixels differ, x {xs.min?}..{xs.max?}, y {ys.min?}..{ys.max?}, palette equal {pal == pal2}"
      cache := c
    let c ← IO.monoNanosNow
    simNs := simNs + (b - a)
    renderNs := renderNs + (c - b)
  IO.println s!"{tics} tics: playsim {simNs / tics / 1000} us/tic, render {renderNs / tics / 1000} us/frame (check {sum})"
  if compare then
    IO.println s!"compare: {tics - mismatches}/{tics} live frames identical to the harness"
  return 0
