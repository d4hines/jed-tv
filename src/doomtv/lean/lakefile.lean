import Lake
open Lake DSL

package doom where
  buildDir := "../../../assets/build/lake/doom"
/-- The engine: every module under Doom/ here runs on the TV (DoomTV's
imports). Its host checks (Chocolate Doom comparisons, audio, front end,
network, benchmarks) are the package test/doomtv. -/
@[default_target] lean_lib Doom where
  globs := #[.submodules `Doom]

/-- The TV app; cross-linked with native/tv_ffi.c and the platform library by
tools/doomtv/build-lean.sh. -/
lean_exe doomtv where root := `DoomTV

/-- Pure lockstep UDP multiplayer session (Doom/Net). -/
@[default_target] lean_lib DoomNet where
  roots := #[`Doom.Net.Session]

/-- Audio engine (pure Lean): sfx, mixer, MUS/MIDI, GENMIDI, OPL3, music, sound. -/
@[default_target] lean_lib DoomAudio where
  globs := #[.submodules `Doom.Audio]
