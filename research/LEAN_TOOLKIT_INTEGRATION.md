# Lean toolkit integration

The `d4hines/src-tv-only` work is merged alongside the tested Lean PID1
controller. The default launcher still packages Rust YouTube and Rust/C Doom.
`nix build .#lean-youtube` builds the separate ARM32 Lean YouTube executable.

The Zr runtime, syscall effects and assembly DSL live in `src/lean/zr`; the
Lean YouTube implementation lives in `src/lean/yt`. Host generators and
build commands live in `tools/lean`; conformance and application checks
live in `test/lean`. Doom and Slides host checks have likewise moved to
`test/doomtv` and `test/slides`, with path dependencies on runtime sources.

The Zr runtime currently targets ARM32 and is single-threaded. It is not a
drop-in replacement for PID1's existing multithreaded Lean runtime. The
controller remains Lean with Rust/C hardware effects; this merge does not
claim all userspace is Lean.

Validation:

- Default-app and standalone Lean YouTube Nix builds pass as separate outputs.
- Full QEMU tour passes: menu, video/audio, nested YouTube Back, Home,
  Slides, Doom, RPG and City Flight. Evidence is under
  `assets/build/integration-qemu-1793567490868005000/`.
- Lean runtime ARM32 conformance matches the upstream expected and panic
  transcripts; the app-model transcript is identical on host and ARM32.
- All 71 Lean YouTube generated C files link against Zr; the standalone app
  is a static ARM32 ELF (3,238,316 bytes in the local cross-build).
- Relocated policy, monadic lifecycle, ABI, Slides and Lean Doom build checks pass.

No TV state or firmware was changed for this integration. QEMU is the
regression environment; it does not establish physical display/audio or
Lean YouTube playback performance on the TV.
