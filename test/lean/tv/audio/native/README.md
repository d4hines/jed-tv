# Cooperative production-adapter check

This executable includes the actual `src/pid1/src/audio_service.rs`. It uses
real nonblocking localhost TCP and compiled Lean. Only individual MI ioctl results and the
application log are replaced. The actual native resource holder is included. The client thread runs no Lean; the main thread
owns every Lean object and calls finite audio ticks.

It checks fragmented open, 44.1 kHz mono conversion, exact PCM bytes, written
and played counters, reset, flush, a 1 MiB PCM block, positive short writes,
backpressure, invalid commands and cleanup. The gain is 0.0025 in a private
temporary directory, so it does not change the host's gain file.

Host:

```sh
CARGO_TARGET_DIR="$PWD/assets/build/audio-service-native" cargo run --offline --release --manifest-path test/lean/tv/audio/native/Cargo.toml
```

ARM32 uses the repository's `tools/toolchain/zcc-armv7-unknown-linux-musleabihf`
and `tools/pid1/linker-armv7`, with `AR_armv7_unknown_linux_musleabihf` pointing
to LLVM ar. Set `ZR_RUNTIME_DIR` to the production Cargo output directory
containing `libzr_native.a` and `zr/gen/{include,rt}`. Build with `--release
--target armv7-unknown-linux-musleabihf`, put the executable at `/init` with
`tools/mkinitramfs.py`, and run the normal ARM virt kernel with `-cpu max`.
The executable brings up loopback when running as PID1. It links no upstream
Init or Lean runtime. Both native socket handling and the shared Lean object
ownership functions are the real production sources.

The host test can require permission to bind loopback in a sandbox. It never
contacts a TV or router. A successful ARM run prints PASS and exits PID1 with
zero; the kernel's subsequent `Attempted to kill init` message is expected.
This tests the native adapter with fake hardware, not the TV audio path.

The test also compares 1,835,008 sample conversions (every signed 16-bit value,
four boosts and seven calibrated gains) to Rust through the actual TCP/Lean
pipeline. Eight streaming converter cases compare all input values for both
sample rates, channel counts, and whole/fragmented chunks to the original
Rust converter. These validate Zr's scalar Float32 primitives and integer
resampler behavior, including clipping, halfway rounding and chunk state.

Production wiring outside this patch: compile/link TV.Audio.Protocol,
TV.Audio.Converter and TV.Audio.Cooperative; register `audio_service` in PID1;
call `audio_service::tick()` from the same `Children::tick` as Core/volume.
The old audio thread is removed. Device ownership and open/release ordering now run in `TV.Audio.Device`.
The native resource holder performs individual operations and reports failure
or uncertain rollback; Lean retains PCM on unresolved routing and prohibits
reopening after uncertain cleanup. The test injects Start and route-restore
failures through the real ioctl adapters and checks those outcomes.
