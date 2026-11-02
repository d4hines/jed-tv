Current integration note: the default bundle retains Rust YouTube and Rust/C Doom.
The ARM32 Lean YouTube/runtime track is available separately as `nix build .#lean-youtube`.
Its sources are in `src/lean/{yt,zr}`, host tooling in `tools/lean`, and checks in `test/lean`.

# Next steps: handoff from agent1 (QEMU track) to Codex

Written 2026-10-31 by agent1 (Claude). Branch `d4hines/agent1`, worktree
`/Users/d4hines/repos/dande-tv/agent1`, head `0b1c5f5` plus this file.
**Not merged into `main`**; `main` still has the old `app/` layout.
`assets` here is a symlink to `../main/assets` (ignored by Git).

## Goal and rules

- Goal: the smallest, most secure YouTube app for the TCL TV, written in Rust
  (Lean proofs come later). Unsafe Rust is fine, and FFmpeg's NEON assembly
  stays assembly.
- First milestone: Linux with **our app as PID 1**, so every userspace
  process is ours. Done in QEMU, not yet on the TV.
- Standing rule from the owner: no flags, as few options as possible, no
  backwards compatibility. It's a hackathon; keep moving forward.
- Device safety: read `AGENTS.md` and `RECOVERY_NOTES.txt` first. Only
  RAM-boot (`fastboot boot`), never flash, never repeat unlock. Ask the owner
  before any boot-state change.
- Don't contact YouTube from eduardo-vm. Live YouTube tests run only from the
  owner's Mac (residential IP).
- Use nix shells; no system-wide installs.

## Layout

```
src/          everything flashed: tvinit, ytplay, doomtv, slides + platform, mp4,
              ffh264 (generated), and the Lean Doom engine. See src/README.md.
test/         QEMU harness and gates: run.py, tv.sh, check.py, check_framemd5.py,
              replay.py (offline YouTube replay server used by the gates)
tools/        toolchain/ (zig cc wrappers), tools/legacy-doom/build.sh, ffh264/ (c2rust
              regeneration), kernel/ (QEMU kernel, built on eduardo-vm),
              make_tv_boot.py, mkinitramfs.py, make_tiles.py, udplog.py, loc.sh
experiment/   baremetal/ (no-Linux spike), nolibc/ (our own libc in Rust)
research/     notes; yt-stage0/PROTOCOL.md is the YouTube protocol we use
QEMU.md       design doc and history of the QEMU track
```

## Current state (all verified in QEMU, none on the TV)

- **Home screen (`tvinit`, PID 1):** rows of tiles for YouTube Search, three
  videos, Slides, Doom, About and Power off. Each app runs as a child process,
  and the Home key always returns to the home screen. Network bring-up comes
  from the kernel command line or `/etc/tvinit.conf`.
- **YouTube (`ytplay`):** search with an on-screen keyboard and thumbnails;
  playback up to 720p.
  - Protocol: the VISIONOS player API plus ranged GETs. No JavaScript, no PO
    token. The search uses the WEB client.
  - Player: pause, seek ±10 s, digit keys jump to N×10%, progress overlay.
  - TLS: rustls with the pure-Rust RustCrypto provider.
  - Sandbox: seccomp allowlist of 33 syscalls and 12 ioctls.
- **Decoder (`ffh264`):** FFmpeg 9.0.1 H.264 translated to Rust by c2rust,
  bit-exact against FFmpeg (framemd5 over all 4431 frames), with the NEON
  assembly kept.
- **Doom (`doomtv`) and Slides (`slides`):** both work. The deck is
  `src/slides/hackathon.md`, compiled into the binary.
- **Gates:** `python3 test/check.py --quick` passes 12/12, and the full run
  adds 5/5 faithful (32-bit) checks. `--live` also plays from real YouTube
  (Mac only).
- **Size:** we wrote about 6.7k lines; `tools/loc.sh` prints the split.
  `cloc src` reports about 772k. That total includes the generated decoder,
  translated once per architecture: 354k lines for 32-bit ARM and 356k for
  64-bit, with only one copy in any binary. It also includes the Doom engine
  (42k lines of C). There are 95 crates.io dependencies, which are not in
  `src/`.

### How to run

```
cd src && cargo build --release --target armv7-unknown-linux-musleabihf   # the TV's ABI
python3 test/check.py --quick        # all gates, headless
test/tv.sh                           # the whole TV in a QEMU window (fast profile)
test/tv.sh faithful                  # same on 32-bit ARM, like the TV kernel
```

- Run Cargo from `src/`, so the pinned Rust 1.92 and `.cargo/config.toml`
  apply.
- `tools/doomtv/build.sh` builds the Lean engine against the shared platform library.
- Kernels for QEMU are already built under `assets/build/kernel/`. Rebuild
  them with `tools/kernel/build.sh` on eduardo-vm.
- The gates start `test/replay.py` on port 8090 if nothing is listening there.

## Next steps, in priority order

### 1. Measure software decode on the real TV (decides 720p vs 480p)

The biggest open risk. All playback numbers so far come from the Mac.
- It runs as the shell user from `/data/local/tmp` and needs no root,
  framebuffer or network.
- Run both frozen binaries. Each prints decode-only fps at 1, 2 and 4 threads,
  then end-to-end decode plus YUV→RGB, then the conversion on its own.

```
adb push assets/build/tv-bench/ytplay-armv7-rust-20579228 /data/local/tmp/ytplay   # current Rust decoder
adb push assets/yt-stage0/record-PVGeM40dABA-visionos-20261031T200633Z/fixtures/136.mp4 /data/local/tmp/136.mp4
adb shell /data/local/tmp/ytplay ytplay.mode=bench ytplay.file=/data/local/tmp/136.mp4 ytplay.frames=1200
adb shell cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq
# reference: the C decoder (older binary still takes ytplay.threads-list=1,2,4)
adb push assets/build/tv-bench/ytplay-armv7-c83a4327 /data/local/tmp/ytplay-c
adb shell /data/local/tmp/ytplay-c ytplay.mode=bench ytplay.file=/data/local/tmp/136.mp4 ytplay.frames=1200 ytplay.threads-list=1,2,4
adb shell rm /data/local/tmp/ytplay /data/local/tmp/ytplay-c /data/local/tmp/136.mp4
```

- The Rust binary's sha256 is
  `20579228c428732307b75022445d1b7b0684443ad8f8a41ea4d54b67ddae3783`. The
  input clip is 1280x536, 24 fps, High@3.1.
- Target: at least 24 fps end to end, with margin.
- If it falls short, lower the cap in `src/ytplay/src/main.rs`
  (`yt::pick_video(&v.formats, 720)`) to 480. Hardware decode would be a much
  larger project; that's GitHub issue #4.
- Record the results in `research/platform-api.md`.

### 2. Our app as PID 1 on the real TV

- `tools/make_tv_boot.py m1a|m1b|m1c` builds RAM-boot images: the stock
  kernel and DTB, byte-identical, plus our ramdisk. Details and hashes are in
  `research/tv-pid1-handoff.md`.
  - m1a: `tvinit` plus a static IP, with the log sent over UDP to
    `tools/udplog.py`.
  - m1b: m1a plus the MStar display modules.
  - m1c: m1b plus the home screen.
- **These images target the ORIGINAL G06/V109 TV.** The TV now connected is
  the G13 / T221T10 V013, Android 14, an A/B layout with `init_boot` and
  `vendor_boot`. Its kernel, modules and ramdisk location differ. The work:
  1. Get `uname -a`, `/proc/config.gz` if present, and the list of loaded
     modules from the G13 over ADB.
  2. Teach `make_tv_boot.py` the G13 layout from
     `assets/backups/g13-v013-stock-20261031`.
  3. RAM-boot m1a first, but only with the owner's approval and a known way
     back.
- Then fill in the platform API on the real hardware (`src/platform`):
  - Display: fbdev, or DRM/KMS, which would need dumb buffers in
    `platform::fb`?
  - The ALSA device path.
  - The IR remote's evdev node and keycodes. The G06 values are in
    `src/platform/src/remote.rs`; check the G13's keylayout `.kl` files.
  - The Ethernet interface name.
  - `research/platform-api.md` lists the exact probes.
- Answer and close GitHub issues #1–#10 as facts come in. All are still open.

### 3. Shrink the decoder (mechanical, safe)

1. Keep only 8-bit 4:2:0. FFmpeg instantiates its H.264 templates for 8, 9,
   10, 12 and 14 bits; YouTube only sends 8-bit.
2. Delete functions the linker never keeps (build with a map file and
   `--gc-sections`, then prune them from the source).
3. Keep both architectures bit-exact with the framemd5 gate
   (`test/check.py -k framemd5`; the full run includes faithful).

- Regenerate from `tools/ffh264/` rather than hand-editing, or prune the
  generated output with a script. Re-run `tools/loc.sh` and report the size.
- Longer term: a hand-written decoder for the main and high profiles, which
  would be much smaller and easier to prove things about.

### 4. Cheap security hardening in `tvinit`

- Start each app as a non-root user with `no_new_privs`.
- Give it an empty or read-only filesystem view: a private mount namespace,
  or chroot into `/share`.
- Close inherited file descriptors.
- Give Doom and Slides their own seccomp allowlists, as ytplay has
  (`src/platform/src/seccomp.rs`).
- Everything here works on a 4.19 kernel; don't rely on Landlock.

### 5. Remove remaining options (standing rule)

- `doom.args`, `doom.iwad`, `doom.onquit`, `doom.sound`, `doom.stats` in
  `src/doomtv`.
- `slides.start`, `slides.seccomp` in `src/slides`.
- Hard-code the defaults, then update `test/check.py` and `test/tv.sh` if
  they pass any of these.

### 6. Later

- **Drop musl entirely.** `experiment/nolibc` already runs tvinit and slides
  on our own Rust libc (stable Rust, both architectures).
  - ytplay still needs 65 libc symbols, 61 of them from c2rust's ffh264:
    pthread mutex/condvar, libm, printf-family functions, FILE. doomtv needs
    41.
  - Estimate: about 3 days for ytplay plus 1 for doomtv.
  - Inventory: `assets/build/nolibc/inventory/SUMMARY.md`.
  - Also add DNS in Rust (the replacement libc only resolves numeric
    addresses).
- **Doom multiplayer** (owner asked). doomgeneric dropped Chocolate Doom's
  networking. Restore it with a UDP transport on the platform layer, and use
  the FreeDM WAD for deathmatch.
- **Lean:** prove properties of `platform` and the demuxer first; c2rust
  output last.
- **Bare metal:** the spike works in QEMU (`experiment/baremetal`). The
  recommendation is to keep the vendor Linux kernel on the TV.

## Gotchas learned the hard way

- QEMU's virtio-snd stalls after DROP plus PREPARE on a running stream, so
  `Pcm::reset()` reopens the device on seek.
- QEMU CoreAudio drains 10 ms periods too slowly; we use 40–100 ms periods.
- Linux virtio-gpu and virtio-snd need modern virtio-mmio:
  `-global virtio-mmio.force-legacy=false`.
- `FBIO_WAITFORVSYNC` doesn't block on virtio-gpu, so frame pacing uses the
  clock.
- A bare player POST gets LOGIN_REQUIRED. Send one request first to get
  `visitorData`, then repeat it (see `research/yt-stage0/PROTOCOL.md`).
- On 32-bit kernels, musl needs `COMPAT_32BIT_TIME` (QEMU kernel config);
  the stock 4.19 has it.
- Build scripts must read `CARGO_MANIFEST_DIR` at runtime, not with `env!`,
  because the target directory is shared between worktrees.
