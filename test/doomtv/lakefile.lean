import Lake
open Lake DSL

/-- Host checks of the Doom engine (src/doomtv/lean): demos and frames
against Chocolate Doom, audio, front end, network, benchmarks. The modules
here (the harness, the network tests, the demo player and trace emitter)
are host-only; the engine they test runs on the TV. -/
package «doom-test»

require doom from "../../src/doomtv/lean"

/-- The host harness and test modules (`DoomTest.*`: the trace harness, the
demo player and trace emitter, the network tests). Their code keeps the
engine's `Doom.*` namespaces; only the module names differ. -/
@[default_target] lean_lib DoomTest where
  globs := #[.andSubmodules `DoomTest]

/-- Host checks against Chocolate Doom fixtures (see src/doomtv/lean/UPSTREAM.txt). -/
lean_exe verify where root := `Verify
lean_exe «fb-test» where root := `FbTest
lean_exe bench where root := `Bench
lean_exe «keys-test» where root := `KeysTest

/-- Doom.Net checks (DoomTest.Net): codec round trips, decoder fuzzing, network simulation. -/
lean_exe «net-test» where root := `DoomTest.Net.Codec
lean_exe «net-fuzz» where root := `DoomTest.Net.Fuzz
lean_exe «net-sim» where root := `DoomTest.Net.Sim
/-- A LAN player leaving, run on the target (check-net-quit-arm32.sh). -/
lean_exe «net-quit» where root := `NetQuit

/-- Host-only support modules of the `audio-check` executable. -/
lean_lib AudioCheckSupport where
  globs := #[.submodules `AudioCheck]

/-- Host audio checks (parsing, OPL bit-exactness, WAV renders, benchmarks). -/
lean_exe «audio-check» where
  root := `AudioCheck

/-- Front-end screens (intermission, finale, automap): renders and checks. -/
lean_exe «front-test» where root := `FrontTest
lean_exe «map-sweep» where root := `MapSweep
