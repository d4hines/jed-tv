# Brief: port Doom to the TV platform (for a parallel agent/session)

Another agent (agent1) is working on the YouTube player in the same working
tree at the same time, so stay inside the files you own (listed below).

## Context (read these first)
- Repo: /Users/d4hines/repos/dande-tv/main (branch d4hines/qemu). Read `QEMU.md`
  (plan, profiles, "Running it") and `research/platform-api.md` (the platform API).
- The TV is a TCL with 4x Cortex-A55; its kernel is 32-bit ARM. We run our app
  as PID 1 on a minimal Linux kernel in QEMU, talking only to plain Linux
  interfaces: fbdev (/dev/fb0, 1280x720 XRGB8888, double-buffered by panning),
  ALSA PCM, evdev keys, sockets.
- The platform layer is the Rust crate `app/crates/platform` (modules `fb`,
  `alsa`, `input`, `sys`). Use it. See how `app/crates/ytplay` uses it, and how
  `app/decoder/build.sh` cross-compiles C (FFmpeg) with `zig cc` into static
  libs that `app/crates/ytplay/build.rs` links. Reuse that pattern.
- Installed: Rust 1.92 pinned in `app/rust-toolchain.toml` (targets
  aarch64-unknown-linux-musl, armv7-unknown-linux-musleabihf, linked by rust-lld,
  see `app/.cargo/config.toml`), zig 0.17, QEMU 11.1.3, nix. Kernels:
  `assets/build/kernel/arm64/Image`, `assets/build/kernel/arm/zImage`. Build
  outputs go under `assets/` (git-ignored).
- `qemu/run.py` boots a binary as PID 1 (`fast` = aarch64 + HVF, `faithful` =
  32-bit ARM, software emulated). `--headless` uses QMP for screenshots
  (`screendump`) and key presses (`send-key`); `--add-file HOST:GUEST` puts extra
  files (e.g. a WAD) in the initramfs; `--extra` appends kernel command line words.

## Goal
Doom running as PID 1 in our QEMU setup, on our platform crate, as a separate
binary crate `app/crates/doomtv`. Later it merges with the YouTube player
behind a launcher, so the remote can switch between YouTube and Doom. Design
for that: keep the main loop in Rust, and make sure quitting Doom never exits
PID 1. Doom's I_Quit/I_Error call exit(), and init exiting panics the kernel.

## Approach
- Port the ORIGINAL open-source Doom engine: id Software's GPL release, via
  doomgeneric (https://github.com/ozkl/doomgeneric), which needs only a few
  platform hooks: DG_Init, DG_DrawFrame, DG_SleepMs, DG_GetTicksMs, DG_GetKey,
  DG_SetWindowTitle. Do not write a new Doom from scratch or translate the
  engine into Rust. Rust does the platform glue: implement the DG_* hooks as
  `#[no_mangle] extern "C"` functions over the platform crate, plus the main loop.
- Fetch doomgeneric with git into its own new, empty directory under
  `assets/third_party/doomgeneric`. Pin and record the commit. Compile only the
  engine files; exclude the SDL/X11/Windows backends. Don't run scripts that
  ship with downloaded sources; build with our own script.
- Game data: Freedoom phase 1 (`freedoom1.wad`, BSD-licensed) from the official
  freedoom GitHub releases. Store it under `assets/third_party/freedoom/` and
  verify the published checksum. No commercial id WADs.
- Cross-compile the engine with `zig cc` for both musl targets into a static lib
  (`app/doom/build.sh` -> `assets/build/doom/<target>/libdoomgeneric.a`),
  linked by `app/crates/doomtv/build.rs`.
- Display: scale DG_ScreenBuffer (XRGB, DOOMGENERIC_RESX x RESY, default
  640x400) into 1280x720 at 4:3 (e.g. 960x720 centered). Use the fb's RGB
  offsets and double buffering. virtio-gpu's vsync wait doesn't really wait;
  pace with a timer.
- Input: map evdev keycodes to Doom keys (doomkeys.h): arrows, Enter, Escape,
  Ctrl (fire), Space (use), Shift (run), Y/N, digits. On the TV the IR remote's
  keycodes are still unknown.
- Sound: stretch goal, via `platform::alsa`.

## Files you own
`app/crates/doomtv/**`, `app/doom/**`, `assets/third_party/**`,
`assets/build/doom/**`. Additive changes only to `qemu/run.py` (e.g. a `--bin`
option) and `app/crates/platform`; list them in your report. Do not modify
`app/crates/ytplay`, `app/crates/mp4`, `app/decoder`, `research/yt-stage0`,
`QEMU.md` or `research/platform-api.md`.

## Hard constraints
- Do NOT touch the real TV (no adb/fastboot, nothing over 192.168.77.x).
- No git commit/push. Don't contact YouTube. No system-wide installs; prefer
  `nix shell nixpkgs#...` or `uvx` if a tool is missing.

## Gates (evidence under assets/build/qemu-runs/)
1. Fast profile: Doom boots as PID 1; title screen or attract demo in a headless screenshot.
2. Input: after QMP key presses (menu, new game), a screenshot shows gameplay.
3. Faithful 32-bit profile runs; report the frame/tic rate (Doom logic is 35 tics/s).
4. Quitting from the menu doesn't panic the kernel.
5. Binary sizes (both targets) and exact build/run commands.

Report: what works (screenshot paths, checked with your own eyes), commands,
doomgeneric commit, Freedoom version/checksum, shared-file changes, open problems.
