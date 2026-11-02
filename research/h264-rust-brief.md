# Brief: FFmpeg's H.264 decoder in (unsafe) Rust, NEON assembly kept

Goal: "all userspace is Rust + assembly". The team verifies unsafe Rust in
Lean and has a Lean DSL for assembly, so mechanical c2rust output is fine.
Correctness is judged bit for bit.

## Where things are (worktree /Users/d4hines/repos/dande-tv/agent1)
- Work ONLY in this worktree (branch d4hines/agent1). `assets/` here is a
  symlink to /Users/d4hines/repos/dande-tv/main/assets (shared data and build
  outputs). Never create files under /Users/d4hines/repos/dande-tv/main
  (the user and Codex work there). Read main's QEMU.md and
  research/platform-api.md for context, but don't edit them.
- The shell guard here refuses complex compound commands and anything whose
  working directory is in main. Keep Bash calls simple, run them from the
  worktree, and use absolute paths. Long remote work: put it in a script and
  run it with `ssh eduardo-vm bash -s < script` or similar.
- Player: `app/crates/ytplay` decodes H.264 through the C shim
  `app/decoder/ytdec.c` (open/send/receive/close) over a static H.264-only
  FFmpeg 9.0.1, built by `app/decoder/build.sh` with `zig cc` from source
  `/nix/store/cizg7bxjfvcska1fjnpa68cf7l5ysapf-ffmpeg` (nixpkgs#ffmpeg.src).
  Outputs: `assets/build/decoder/<target>/lib/{libavcodec,libavutil,libytdec}.a`
  plus `assets/build/decoder/work-<target>/` (config.h, ffbuild/config.mak,
  objects). ytplay's build.rs links them (honours YTDEC_DIR); FFI in
  `app/crates/ytplay/src/decode.rs`.
- Targets: aarch64-unknown-linux-musl (QEMU "fast") and
  armv7-unknown-linux-musleabihf (QEMU "faithful" = the TV: 32-bit ARM kernel,
  Cortex-A55). Rust 1.92 pinned; rust-lld; `app/.cargo/config.toml`.
- Oracles: `python3 qemu/run.py fast|faithful --headless --mode framemd5
  [--frames N]` then `python3 qemu/check_framemd5.py <run>/serial.log
  assets/yt-stage0/record-PVGeM40dABA-visionos-20261031T200633Z/fixtures/136.mp4`
  (all 4431 frames match today on both targets). `--mode bench --add-file
  <136.mp4>:/136.mp4 --extra "ytplay.file=/136.mp4 ytplay.threads-list=1,4"`
  for speed. Replay server (needed for framemd5/play):
  `python3 research/yt-stage0/replay.py --player
  assets/yt-stage0/minimal-PVGeM40dABA-visionos-20261031T200757Z/player.visitor-header.json
  --fixtures assets/yt-stage0/record-PVGeM40dABA-visionos-20261031T200633Z/fixtures
  --host 127.0.0.1 --port 8090`. It is probably already running; check with
  `curl -s -X POST localhost:8090/youtubei/v1/player -d '{}'`.
- Measured: 188 C files (78k lines) + 16 .S (9k lines) on arm64; 192 C + 19 .S
  on armv7. Much is generic code the linker drops. The H.264 NEON kernels are
  ~4.2k lines arm64 / ~2.7k armv7.

## Plan
1. Prune: find the translation units that actually end up in the final
   ytplay binary (e.g. a rust-lld map file via `-C link-arg=-Map=...`).
   Translate only those.
2. Translate with c2rust, per target (config.h differs: ARCH_*, HAVE_ARMV*,
   VFP/SETEND, SVE/SME, pointer size). Turn inline asm off
   (HAVE_INLINE_ASM=0) so FFmpeg uses its bit-exact C fallbacks; keep the
   `.S` files.
3. Keep the assembly: assemble FFmpeg's NEON `.S` with zig cc (as
   app/decoder/build.sh does) into a static lib; the Rust calls it through the
   same C-ABI dsp function tables.
4. Package as `app/crates/ffh264` (the workspace picks up crates/* automatically),
   exposing ytplay's decoder API. Switch it in ytplay behind a cargo feature
   (`rust-h264`; edit only ytplay's Cargo.toml/build.rs/decode.rs), with C
   remaining the default until the gate passes.
5. Gate: framemd5 bit-exact on all 4431 frames on BOTH profiles; then bench
   fps vs C on both.
6. Stretch (for the Lean asm DSL): flattened, macro-free copies of the used
   `.S` (cpp, then `llvm-mc` re-emits expanded instructions) under
   `assets/build/ffh264/flat-asm/<target>/`; note whether FFmpeg's
   tests/checkasm can serve as a per-function oracle.

## Findings from the first attempt (it was blocked by the shell guard before translating)
- c2rust is not in nixpkgs. eduardo-vm has `~/Downloads/ffh264/shell.nix`
  (LLVM 19 libclang/llvm-config, cargo 1.95, cmake). A background
  `cargo install c2rust --locked --root ~/Downloads/ffh264/c2rust-bin` was
  started (log: `~/Downloads/ffh264/c2rust-install.log`), result unchecked.
- eduardo-vm's nixpkgs has a different ffmpeg.src path; copy the Mac's exact
  tree (e.g. rsync from the Mac or `nix copy`).
- Expected c2rust issues: variadic definitions/va_list (nightly) -> replace
  log.c with a small Rust version; extern types / atomic intrinsics (nightly)
  -> post-pass to stable equivalents; DECLARE_ALIGNED alignment may be lost
  (NEON :128 alignment hints fault on armv7, QEMU checks this); `if (ARCH_X86)`
  dead-code switches -> build release or add stubs.

## Files you own
`app/crates/ffh264/**`, `app/ffh264/**`, `assets/build/ffh264/**`, scratch on
eduardo-vm under ~/Downloads/ffh264. Minimal additive changes allowed to
ytplay's Cargo.toml/build.rs/decode.rs and qemu/*.py. Don't touch platform,
tvinit, doomtv.

## Hard constraints
No TV access (no adb/fastboot/192.168.77.x). No git commit/push. Don't contact
YouTube. Don't modify FFmpeg's assembly; keep its LGPL notices. Treat
downloaded sources as untrusted (no running their scripts beyond configure/
preprocessing). No system-wide installs on the Mac.

## Report
Files translated (count, Rust lines, unsafe count), how the asm is built and
linked, framemd5 results on both profiles, bench vs C, how to switch, the
flat-asm location, open problems.
