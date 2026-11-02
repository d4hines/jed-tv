import Doom.Playsim.Fixed
import Doom.Playsim.Angle
import Doom.Playsim.Tables
import Doom.Playsim.Random
import Doom.Playsim.Level
import Doom.Playsim.Info
import Doom.Playsim.Defs
import Doom.Playsim.Flags
import Doom.Playsim.Thinker
import Doom.Playsim.Mobj
import Doom.Playsim.Player
import Doom.Playsim.Input
import Doom.Playsim.GameState
import Doom.Playsim.Bsp
import DoomTest.Playsim.Demo
import Doom.Playsim.Sound
import Doom.Playsim.MapUtil
import Doom.Playsim.Sight
import Doom.Playsim.MobjBase
import Doom.Playsim.Inter
import Doom.Playsim.SpecDefs
import Doom.Playsim.SpecUtil
import Doom.Playsim.Doors
import Doom.Playsim.Floors
import Doom.Playsim.Switch
import Doom.Playsim.CoreBase
import Doom.Playsim.Core
import Doom.Playsim.Movers
import Doom.Playsim.MobjThink
import Doom.Playsim.User
import Doom.Playsim.Tick
import Doom.Playsim.Resources
import Doom.Playsim.Setup
import Doom.Playsim.Intermission
import Doom.Playsim.FinaleText
import Doom.Playsim.Finale
import DoomTest.Playsim.TraceEmit
/-!
# Doom.Playsim

The Doom playsim, ported function by function from Chocolate Doom
353cf5001dfd5777c13327010fa58acb57b913b2 and verified tic-identical against
it on demos (see `check-demos.sh`).

## Conventions

* Wrapping `UInt32`/`Int32` arithmetic only; C pointers are array indices
  (`-1` = NULL).
* One `GameState` holds everything, including the C file-scope globals, and
  is threaded through the state monad `GameState.M` so it stays unshared and
  array updates are in place.
* `Core` is one mutually recursive block, mirroring the C call graph
  (state actions ↔ damage ↔ movement ↔ specials).
-/
