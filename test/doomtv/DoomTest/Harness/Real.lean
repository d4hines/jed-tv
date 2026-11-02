import Doom.Game
import Doom.Harness.Fnv
import DoomTest.Harness.TraceFormat
import DoomTest.Render
import DoomTest.Playsim.TraceEmit
import Doom.Playsim.SpecUtil
import Doom.Wad
/-!
# Doom.Harness.Real

Play a demo through `Doom.Game` exactly as the oracle's `-tracedemo` does
(`D_DoomLoop`: tic 0 without display, then tic + display per frame), and
write the per-tic digest stream (`.dig`) and optionally the full trace
(`.trc`). Stops when the demo marker is read, like the oracle (which quits
inside that `G_Ticker`, before dumping).
-/

namespace Doom.Harness.Real

open Doom.Harness.Fnv
open Doom.Harness.TraceFormat
open Doom.Playsim.GameState
open Doom.Playsim.TraceEmit
open Doom.Render
open Doom.Wad

/-- Demo bytes: an existing file path, else a lump name in the WAD. -/
def loadDemo (wad : WadDirectory) (demoName : String) : IO (Except String ByteArray) := do
  let p : System.FilePath := demoName
  if ← p.pathExists then return .ok (← IO.FS.readBinFile p)
  match checkNumForName wad demoName with
  | none => return .error s!"demo lump {demoName} not found"
  | some idx => return lumpData wad idx

structure RunResult where
  tics : Nat
  error : Option String

/--
Run up to `maxTics` tics of `demoName`; writes `pathBase.dig` (and
`pathBase.trc` when `writeTrace`). Renderer frame dumps as before when
`fbDir` is given.
-/
def runDemo (iwadPath : System.FilePath) (demoName : String) (maxTics : Nat)
    (pathBase : System.FilePath) (writeTrace : Bool := false)
    (fbDir : Option System.FilePath := none) (fbTics : Array Nat := #[])
    (timelimit : Int32 := 0) (digest : Bool := true)
    (exits : Array (Nat × Bool) := #[]) (logFlow : Bool := false)
    (logSounds : Bool := false) :
    IO (Except String RunResult) := do
  try
    let wad ← loadFile iwadPath
    let demo ← match ← loadDemo wad demoName with
      | .ok d => pure d
      | .error e => return .error e
    let mut gs ← match Doom.Game.playDemo wad demo with
      | .ok g => pure { g with timelimit }
      | .error e => return .error e
    let mut dig := encodeDigestHeader
    let mut trc := if writeTrace then encodeTraceHeader else ByteArray.empty
    let mut g := 0
    let mut err : Option String := none
    let mut fuzzpos := 0
    let mut flow := ""
    let mut nStart := 0
    let mut nAudible := 0
    let mut nStop := 0
    let mut nMusic := 0
    while g < maxTics do
      gs := { gs with gametic := g.toUInt32 }
      gs := Doom.Game.ticker gs #[]
      if gs.demoEnded then break
      -- `-traceexits` harness hook: forced exits at the end of G_Ticker
      for (et, secret) in exits do
        if et == g then
          gs := ((if secret then Doom.Playsim.SpecUtil.gSecretExitLevel
                  else Doom.Playsim.SpecUtil.gExitLevel).run gs).2
      if let some e := gs.error then
        err := some s!"tic {g}: {e}"
        break
      if logSounds then
        for ev in gs.soundEvents do
          match ev with
          | .start _ _ aud => nStart := nStart + 1; if aud then nAudible := nAudible + 1
          | .stop _ => nStop := nStop + 1
          | .music m l =>
            nMusic := nMusic + 1
            IO.println s!"  tic {g}: music {m} looping={l}"
      if logFlow then
        let st := match gs.gamestate.toNatClampNeg with
          | 0 => if gs.res.gamemode == gm_commercial then s!"MAP{gs.gamemap}"
                 else s!"E{gs.gameepisode}M{gs.gamemap}"
          | 1 => "WI" | 2 => s!"F{gs.finale.finalestage}" | _ => "?"
        if st != flow then
          IO.println s!"  tic {g}: {st}"
          flow := st
      if digest then
        match emitTicRecord gs with
        | .error e => err := some e; break
        | .ok rec =>
          let bytes := encodeTicRecord rec
          dig := pushU64LE dig (fnv1a64 bytes)
          if writeTrace then trc := appendBytes trc bytes
      if g > 0 then
        gs := Doom.Game.displayFrame gs
        if fbDir.isSome && gs.gamestate == gamestateLevel then
          match ← dumpIfRequested wad gs fbDir fbTics fuzzpos with
          | .error e => err := some e; break
          | .ok fp => fuzzpos := fp
      g := g + 1
    if logSounds then
      IO.println s!"  sound events: {nStart} starts ({nAudible} audible to the console player), {nStop} stops, {nMusic} music changes"
    IO.FS.writeBinFile (pathBase.toString ++ ".dig") dig
    if writeTrace then IO.FS.writeBinFile (pathBase.toString ++ ".trc") trc
    return .ok { tics := g, error := err }
  catch e =>
    return .error (toString e)

/--
Play a demo through the public front-end API instead of demo playback:
`Game.newGame` with the options from the demo header, then `Game.ticker`
with the demo's commands passed as per-player commands. Exercises exactly
the path a front end or netgame uses.
-/
def runApi (iwadPath : System.FilePath) (demoName : String) (maxTics : Nat)
    (pathBase : System.FilePath) : IO (Except String RunResult) := do
  try
    let wad ← loadFile iwadPath
    let d ← match ← loadDemo wad demoName with
      | .ok d => pure d
      | .error e => return .error e
    if d.size < 13 || d[0]! != 109 then return .error "not a v1.9 demo"
    let pig := Array.ofFn (n := 4) fun i => d[9 + i.val]! != 0
    let opts : Doom.Game.NewGameOptions := {
      skill := d[1]!.toUInt32.toInt32, episode := d[2]!.toUInt32.toInt32
      map := d[3]!.toUInt32.toInt32, deathmatch := d[4]!.toUInt32.toInt32
      respawn := d[5]! != 0, fast := d[6]! != 0, nomonsters := d[7]! != 0
      consoleplayer := d[8]!.toNat, playeringame := pig, netgame := some pig[1]! }
    let mut gs ← match Doom.Game.newGame wad opts with
      | .ok g => pure g
      | .error e => return .error e
    let mut dig := encodeDigestHeader
    let mut cur := 13
    let mut g := 0
    let mut err : Option String := none
    while g < maxTics do
      let mut cmds : Array Doom.Playsim.Player.TicCmd := Array.replicate 4 Doom.Playsim.Player.TicCmd.zero
      let mut ended := false
      for i in [0:4] do
        if pig[i]! then
          if cur >= d.size || d[cur]! == 0x80 then ended := true
          else
            let sx (b : UInt8) : Int32 := b.toInt8.toInt32
            cmds := cmds.set! i { forwardmove := sx d[cur]!, sidemove := sx d[cur+1]!
                                  angleturn := sx d[cur+2]! <<< 8, buttons := d[cur+3]!.toUInt32 }
            cur := cur + 4
      if ended then break
      gs := { gs with gametic := g.toUInt32 }
      gs := Doom.Game.ticker gs cmds
      if let some e := gs.error then
        err := some s!"tic {g}: {e}"
        break
      match emitTicRecord gs with
      | .error e => err := some e; break
      | .ok rec => dig := pushU64LE dig (fnv1a64 (encodeTicRecord rec))
      if g > 0 then gs := Doom.Game.displayFrame gs
      g := g + 1
    IO.FS.writeBinFile (pathBase.toString ++ ".dig") dig
    return .ok { tics := g, error := err }
  catch e =>
    return .error (toString e)

/-- Back-compatible entry point (used by `fb-test`). -/
def runReal (iwadPath : System.FilePath) (demoName : String) (tics : Nat)
    (pathBase : System.FilePath) (fbDir : Option System.FilePath := none)
    (fbTics : Array Nat := #[]) : IO (Except String Unit) := do
  match ← runDemo iwadPath demoName tics pathBase true fbDir fbTics with
  | .error e => return .error e
  | .ok r => match r.error with
    | some e => return .error e
    | none => return .ok ()

end Doom.Harness.Real
