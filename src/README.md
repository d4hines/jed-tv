# src/: everything that runs on the TV

This directory holds the auditable source inputs for TV userspace. Lean
compiler output lives under `assets/build/lake`, outside this tree, so
`cloc src` does not count generated Lean C. `tools/loc.sh` splits the count into code we wrote,
generated code and vendored code, and lists the crates.io dependencies.
The kernel is not here. On the TV it is the stock vendor kernel; in QEMU it is
built by `tools/kernel/build.sh`.

PID 1's Lean, Rust, native ABI, backends, and runtime setup are grouped under
[`pid1/`](pid1/README.md). Shared app interfaces and language runtimes stay
outside that directory.
Development probes and one-off candidates live under `research/`.

## Apps (each is a static binary)

| Binary | What it is |
|---|---|
| `tvinit` | PID 1: mounts, network bring-up, the home screen. Runs each app as a child process; the Home key always gets back to it. |
| `ytplay` | YouTube: on-screen-keyboard search, thumbnails, player (pause, seek, overlay). |
| `doomtv` | Doom (Freedoom WAD), original C engine with Rust platform glue; see `doomtv/README.md`. |
| `flighttv` | City flight demo. |
| `axiomtv` | Axiom: First Light, a voxel game written in Lean 4 (petergpt/lean-3d-game-axiom); `axiomtv/gfx` is its Rust platform glue. |
| `rpgtv` | Offline RPG, with the original Lean rules and story linked into its process. |
| `slides` | The hackathon deck. `slides/hackathon.md` is compiled into the binary at build time. |
| `snaketv` | Snake on a torus: the ESP32 prototype's RV32I game (`main.S`) rewritten in Rust, step for step, drawn on the framebuffer and steered with the remote (Left/Right turn, OK restarts). `cargo test -p snaketv` checks the game against the original's behaviour. |

The retained `ffh264` crate still contains generated Rust translated from
FFmpeg; `tools/loc.sh` reports it separately from authored code. Handwritten
and vendored C (including Doom) remain source inputs here.

## Libraries

| Crate | What it is |
|---|---|
| `platform` | The platform API, shared by all apps: fbdev, ALSA (raw ioctls), evdev, remote keymap, sockets/net bring-up, seccomp allowlist. |
| `mp4` | MP4/fMP4 demuxer (sidx, elst). |
| `ffh264` | **Generated.** FFmpeg 9.0.1's H.264 decoder translated to unsafe Rust by c2rust (`src/arm`, `src/aarch64`), plus FFmpeg's NEON assembly unmodified (`asm/`). Regenerate with `tools/ffh264/regenerate.sh`; do not edit by hand. |

The experimental `lean/yt` app and `lean/zr` runtime build separately with
`nix build .#lean-youtube`. They do not change the default YouTube/Doom apps.
Host generators live in `tools/lean`; their checks live in `test/lean`.

## Vendored

- `third_party/doomgeneric`: the active C Doom engine; source provenance and local
  multiplayer changes are recorded in `third_party/doomgeneric/UPSTREAM`.

- `doomtv/lean`: the Lean Doom engine, derived from a Lean port of Chocolate
  Doom (GPL-2.0-or-later); provenance in `doomtv/lean/UPSTREAM.txt`.
- `slides/fonts/dejavu-2.37`: fonts embedded in `slides`.

## Building

    bash tools/doom/build.sh aarch64-unknown-linux-musl armv7-unknown-linux-musleabihf
    cd src
    cargo build --release --target aarch64-unknown-linux-musl         # QEMU fast
    cargo build --release --target armv7-unknown-linux-musleabihf     # the TV, QEMU faithful

Doom is built by `../tools/doomtv/build.sh` (Rust/C). The paused Lean version can be built explicitly with `../tools/doomtv/build-lean.sh`.
Cross C/assembly uses the zig wrappers in `tools/toolchain`.
Build output goes to `assets/build/app-target` (not in Git). `test/check.py`
builds everything and runs the gates; `test/tv.sh` boots the whole TV image in QEMU.
