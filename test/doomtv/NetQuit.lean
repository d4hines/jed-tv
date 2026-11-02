import Doom.Game
/-!
# net-quit

A LAN player leaving, on the target runtime: a two-player netgame runs a tic
with both players, then one without player 2 in the tic's in-game mask
(`Doom.Game.tickerNet`, as the lockstep session delivers it). Player 1 must
get vanilla's "Player 2 left the game". The message is built with
`String.ofList` of a `String.toList`, which crashed on 32-bit ARM until the
cross-built runtime's list terminator was fixed (tools/lean-arm/build.sh);
`check-net-quit-arm32.sh` runs this as PID 1 in the ARM32 QEMU surrogate.

    net-quit IWAD
-/

open Doom.Playsim.Player (TicCmd)

def main (args : List String) : IO UInt32 := do
  let wad ← Doom.Wad.loadFile (args.headD "/f/iwad.wad")
  let o : Doom.Game.NewGameOptions :=
    { episode := 1, map := 1, playeringame := #[true, true, false, false], netgame := some true }
  let gs ← match Doom.Game.newGame wad o with
    | .ok gs => pure gs
    | .error e => IO.println s!"net-quit: FAIL: cannot start: {e}"; return 1
  let cmds := Array.replicate 4 TicCmd.zero
  let gs := Doom.Game.tickerNet gs cmds #[true, true, false, false]
  let gs := Doom.Game.tickerNet gs cmds #[true, false, false, false]
  let msg := (gs.players[0]!.message).getD gs.hu.messageText
  if gs.error.isNone && !gs.playeringame[1]! && msg == "Player 2 left the game" then
    IO.println "net-quit: PASS"
    return 0
  IO.println s!"net-quit: FAIL: in game {gs.playeringame}, message {repr msg}, error {gs.error}"
  return 1
