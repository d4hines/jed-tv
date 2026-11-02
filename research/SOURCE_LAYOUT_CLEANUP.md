# TV source layout cleanup — 2026-11-01

The current layout keeps TV runtime source under `src/`, host build/codegen
programs under `tools/`, Nix integration under `nix/`, and standalone test
fixtures/harnesses under `test/` or the matching tool directory. No device
access, deployment, commit or push was performed during this cleanup.

## Moves

94 source/tool/fixture files moved; package metadata and runtime files remain
in their package directories. The first 86 moves have original SHA-256
manifests under `assets/layout-migration/{build-tools-20261101T200012Z,
test-tools-20261101T200104Z}/moves.json`. Eight additional moves are the four
Doom Harness modules, platform pixel tests, no-libc selftest source, and the
two generated FFmpeg structure-layout assertion files.

- App build scripts, check scripts, slide generation and fixtures:
  `tools/{doomtv,slides}/`.
- Doom oracle generators, patches, cases, host verification/benchmark programs,
  audio checks and network simulation tests: `tools/doomtv/`.
- Lean C-ABI drivers/checks: `tools/lean/`; their Nix check: `nix/lean-check.nix`.
- Cargo build scripts: `tools/{tvinit,ffh264,legacy-doom,experiments/baremetal}/`;
  Cargo manifests name those paths explicitly. Their manifest-relative runtime
  source resolution is unchanged.
- QEMU guest fixtures and no-libc selftest: `test/fixtures/`.
- Separate audio, TEE, MP4 and pixel tests, and generated FFmpeg compile-time
  layout assertions: `test/unit/`.

Updated callers include Nix packaging, local/native builds, Lake source
roots, Cargo test targets, fixture builders, generated-code publication,
source hashing and documentation. New paths are marked intent-to-add so Nix
can include them before the eventual commit.

## Retained source

All remaining shell scripts under `src/` are TV-executed runtime or diagnostic
scripts under `src/runtime-scripts/`. There are no remaining Python scripts,
Nix expressions, Cargo build.rs files or standalone test directories there.
Package manifests/toolchain metadata, licenses and documentation remain with
the runtime packages. Inline Rust unit tests and Lean proof modules remain
adjacent to the runtime policy they verify. Doom's `Harness/Fnv.lean` and
`Harness/TraceFormat.lean` are imported by runtime engine modules and remain
under `src/`; the four host-only Harness modules were moved. Historical target
probes remain under `src/probes/`, since they execute on the TV.

## Validation

Passed after relocation:

- `python3 tools/lean/check.py`: 193,536 menu cases, 80,387 supervisor cases,
  655,956 sandbox cases; Lean policy proofs compile.
- `nix build .#checks.aarch64-darwin.lean --no-link`: passed.
- `python3 tools/slides/check.py`: parser/navigation and all 12 deck slides,
  plus six image-feature fixture slides. Main generated deck restored.
- `lake build net-test audio-check verify front-test map-sweep` in
  `src/doomtv/lean`: all 228 jobs passed.
- ARM32 Cargo check for `tvinit`, `ffh264`, `platform`, using separate
  `assets/build/layout-validation/cargo` output: passed.
- `bash tools/audio/check-mi-pcm.sh`: 17 tests.
- `bash tools/audio/check-direct.sh`: 72 tests.
- `bash tools/tee-rpc/check.sh`: 33 tests and ARM32 library compilation.
- `python3 tools/pid1/check-framebuffer.py`: three tests.
- Python syntax, shell syntax, executable-root selection and `git diff --check`.

The separately requested cache-root regression also passed in ARM32 QEMU:
all six `settings_store` tests, including mounted expected-device UID 1000
acceptance and unmounted nonroot refusal. Evidence:
`assets/build/layout-validation/settings-arm32.log`. Its normal test-runner
exit causes a QEMU PID-1 exit panic only after the successful result.

The initial unrestricted-host Cargo test attempt was inappropriate for these
Linux-specific modules and did not pass on macOS; validation used the ARM32
Linux target instead. The initial debug test link also enabled compiler UBSan
helpers unavailable to rust-lld; the project's normal optimized release test
build passed. These were validation configuration issues, not suppressed tests.

A complete rebuilt native five-app bundle and hardware playtest remain the
coordinator's responsibility; this offline pass did not overwrite deployed
artifacts or exercise the TV. No runtime policy changed in the layout work.
The only runtime-file addition beyond import paths was the coordinator-requested
pure cache mount decision helper and its regression test, preserving the
coordinator's existing device/ownership checks.
