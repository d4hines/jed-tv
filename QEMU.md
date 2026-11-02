Current integration note: the default bundle retains Rust YouTube and Rust/C Doom.
The ARM32 Lean YouTube/runtime track is available separately as `nix build .#lean-youtube`.
Its sources are in `src/lean/{yt,zr}`, host tooling in `tools/lean`, and checks in `test/lean`.

# Minimal Youtube Hackathon

Current stock-kernel path: `python3 test/run.py faithful --stock-smoke --no-build`.
`faithful` boots the exact dumped G06 4.19.116+ kernel on our partial G06
QEMU machine. Four cores, basic syscalls and real tvinit startup are verified.
Build the emulator from the main checkout with
`python3 research/qemu-g06/build.py`. The remaining hardware gaps are tracked
in [issue #11](https://github.com/d4hines/dande-tv/issues/11).

`test/tv.sh` defaults to this stock-kernel path. Display, audio and packet I/O
are still unmodeled; use `test/tv.sh virt32` for the existing media UI tests.
The instructions and performance results below describe those explicitly
surrogate `virt32`/`fast` profiles, which retain Linux 6.18.57.

We're doing a hackathon on smart tv OS and applications.

## Goal

We want the smallest possible youtube application, so 
we can try to get it working in our smart tv in a highly 
secure, minimalist way.

We'll use Rust as our implementation language

## Prior art

Do your own research for prior art. However, for example there's
https://github.com/teamnewpipe/newpipe, and other open native youtube clients.

## Platform

Take a look in the repo for details about the TV.

We want to start with pure software simulation in QEMU.

Just try to use QEMU settings that are as close as we can
get to the target chipset.

## Stages

The goal is to squeeze more and more stuff out of the stack
in each stage. By the end, we want to know the exact API's
we need to implement to get it working, and hopefuly it's a small set.

Strategy: shrink from the top down and keep Linux until last. This track
starts at YouTube's servers and works down; a parallel track works up from
the TV's hardware and firmware. Both meet at the **platform API**: the short
list of services the player needs from below (network, display, audio,
input, clock, randomness). Open hardware questions are GitHub issues #1-#10.

| # | Where | Removed | Gate / output |
|---|---|---|---|
| 0 | Mac (residential IP) | nothing; measure | DONE 2026-10-31: two JSON POSTs + ranged GETs, no JS/token/login; fixtures + replay server. See `research/yt-stage0/PROTOCOL.md` |
| 1 | Mac, native Rust | Android, ART, Java | `ytplay <id> --res N` plays in sync from replayed traffic |
| 2 | QEMU + dumped stock Linux (core boot verified; devices pending) | all Android userspace | Kernel + initramfs holding only the static Rust binary as PID 1 |
| 3 | QEMU + trimmed Linux | unused kernel code | Minimal kernel config + seccomp allowlist = the API list |
| 4 | QEMU, bare metal | Linux | `no_std` Rust implements the platform API (virtio, smoltcp, rustls) |
| 5 | MT9221 | virtio devices | Real-chip drivers; RAM-only `fastboot boot` after reading RECOVERY_NOTES.txt |

Target (live survey by the bottom-up track, `research/g06-platform-capabilities.md`):
the connected TV is the original G06. 4x Cortex-A55 running a **32-bit ARM
kernel** (Linux 4.19, CONFIG_CPU_V7) with AArch32 NEON + AES/PMULL/SHA/CRC32;
1.43 GB RAM visible; panel **1280x720@60**; 100 Mbit Ethernet. Its kernel
exposes fbdev (`mstar_fbdev_mi`), ALSA and evdev (IR = event2), which are the
same interfaces QEMU's virtio devices give a Linux guest. Bare metal may
switch to AArch64 later (issue #1).

- Faithful profile (Linux stages): `qemu-system-aarch64 -M virt-11.1,highmem=off,gic-version=2
  -cpu cortex-a55,aarch64=off -smp 4 -m 1430M` (AArch32 v8 + crypto; TCG, not real time; gate on
  frame hashes and A/V drift)
- Fast dev profile (Mac only): `qemu-system-aarch64 -M virt-11.1 -accel hvf -cpu host`
  with an aarch64 build (same Linux platform code, near-native speed)
- Devices: virtio-mmio net, gpu (1280x720), sound, keyboard (remote
  stand-in), rng; PL011 console. No GPU acceleration.
- Platform API v0 (agreed with the bottom-up track): fbdev, ALSA PCM ioctls,
  evdev, TCP/UDP sockets, CLOCK_MONOTONIC, getrandom. TLS, DNS and video/audio
  decoding live in the app.

Media target (team decision): 720p max; H.264 + AAC, MP4 container only
(itags 136/135/134/133/160 + 140). AV1 in MP4 is a later option.

Never hit YouTube from QEMU loops or eduardo-vm (data-center IP); replay
the Stage 0 recordings instead.

Stages 1 and 2 are merged (decided 2026-10-31): the player runs only as
PID 1 in QEMU, with no throwaway Mac window or audio layer. The Mac keeps
headless `cargo test` for the pure logic.


### Running it (status 2026-10-31: platform self-test passes on both profiles)

- Kernels: `tools/kernel/build.sh arm64|arm OUT` on eduardo-vm (tinyconfig +
  `common.config` + `<arch>.config`, Linux 6.18 LTS, ~1 min). Copy to
  `assets/build/kernel/{arm64/Image,arm/zImage}`.
- Player: `app/` (Rust 1.92 pinned; static musl binaries via rust-lld).
- Replay server: `python3 test/replay.py --player ... --fixtures ...`
  (see `research/yt-stage0/PROTOCOL.md`); the guest reaches it at 10.0.2.2:8090.
- Boot: `python3 test/run.py fast|virt32 [--headless]`. Headless mode
  screenshots the guest, presses Enter in it, and records/analyses audio.

Self-test today: fbdev 1280x720 XRGB8888 double-buffered; ALSA S16_LE
48 kHz stereo (440 Hz tone verified in the recording); evdev keys; DHCP; the
two-step `/player` call + ranged GETs of the 720p H.264 and AAC MP4 headers.
Boot to PID 1 takes ~0.06 s (fast) / ~0.7 s (virt32).

Playback (status 2026-10-31 ~14:10 PDT):
- `python3 test/run.py fast|virt32 --headless --mode play|framemd5|bench|selftest`.
  `--server https://www.youtube.com` plays live; default is the replay server.
- Pipeline: two-step `/player` call -> sidx-indexed ranged GETs -> own MP4
  demuxer (crate `mp4`, cross-checked against ffprobe on every sample) ->
  FFmpeg 9.0.1 H.264-only decoder (static, zig cc, NEON) + symphonia AAC ->
  ALSA audio thread owns the clock; video frames shown against it, late ones
  dropped -> NEON-friendly YUV->RGB, multi-threaded, letterboxed into fb0.
- Bit-exact: all 4431 frames of Coffee Run 720p match `ffmpeg -f framemd5`
  on both arm64 and armv7 (`test/check_framemd5.py`).
- Live YouTube from the QEMU guest over rustls (ring, webpki roots, DNS from
  the kernel's DHCP via /proc/net/pnp): plays fine; googlevideo accepts
  rustls (the Stage 0 mitmproxy 403 was mitmproxy-specific).
- Fast profile: A/V offset mean ~+3 ms, max < 40 ms, 0 underruns; convert
  0.19 ms/frame (4 threads). Faithful profile: correct but drops frames,
  because virtio-gpu's fbdev emulation write-faults every page each frame
  (~80 ms/frame under TCG). The TV's fbdev is plain memory, so that's a QEMU
  artifact; real-chip speed is pending the TV benchmark
  (`research/platform-api.md` request B).
- Static binary: 4.3 MB arm64, 3.7 MB armv7 (incl. TLS, H.264, AAC, JSON).
- Doom: a parallel agent is porting the original GPL engine (doomgeneric +
  Freedoom) onto the same platform crate; brief: `research/doom-brief.md`.

The whole TV in one command (2026-10-31 ~14:35 PDT): `test/tv.sh` boots
tvinit's home screen (rows of 16:9 tiles: YouTube videos with real
thumbnails, Doom, About, Power off; clock in local time) with /bin/ytplay,
/bin/doomtv, Freedoom and tile art (`tools/make_tiles.py`, raw RGB so the TV
needs no image decoder). `test/tv.sh` opens a window; `test/tv.sh fast
--headless --at 3:shot:home ...` scripts it; `test/tv.sh virt32` for 32-bit.

Launcher and API census (2026-10-31 ~14:20 PDT):
- `src/pid1/tvinit` is PID 1: menu on fb0 (8x8 font), remote keys via
  evdev, each app (`/bin/ytplay`, `/bin/doomtv`) runs as its own child process
  and the menu returns when it exits (Back/Esc stops ytplay). Run:
  `python3 test/run.py fast --headless --bin tvinit --add-file <ytplay>:/bin/ytplay --at 3:key:ret ...`
- Syscall census (`ytplay.trace=1`, built-in ptrace tracer): 10 s of
  playback uses 30 syscalls and 11 ioctls (6 ALSA, 4 fbdev, 1 evdev) on both
  arm64 and armv7. Full tables: `research/syscall-census-20261031.md`.
- Enforced: ytplay sandboxes itself with a seccomp-bpf allowlist of that API
  (46 syscalls arm64 / 52 ARM, 11 ioctls), on by default. All modes, including
  live YouTube, pass; a stray getuid() is trapped. Kernels now set
  CONFIG_SECCOMP_FILTER (as the TV's does).

Platform API findings so far:
- virtio-mmio must be modern (`-global virtio-mmio.force-legacy=false`).
- virtio-gpu accepts FBIO_WAITFORVSYNC but doesn't wait; pace with a timer.
- 32-bit musl needs the legacy 32-bit time syscalls (`nanosleep`): the
  virt32 kernel sets CONFIG_COMPAT_32BIT_TIME, as the TV's 4.19 always has.

## Infra

You can use this PC. you can also sue eduardo-vm, it's much bigger, see ssh config. It's in Brazil though, so there may
be latency.


## Firmware-informed G06 constraints (2026-10-31)

`test/tv.sh` now defaults to the 32-bit `virt32` profile. Use `test/tv.sh
fast` for HVF development. Faithful runs use the observed 192.168.77.2/24
TV address and 192.168.77.1 host, and start with the observed stale date
2025-01-17. For tvinit, kernel IP autoconfiguration is disabled (`ip=off`):
our PID 1 configures the interface itself, as required by stock
CONFIG_IP_PNP=n. Direct ytplay runs still use kernel static configuration;
they do not establish userspace network bring-up. Offline replay remains
the default. Real TLS needs a separate trusted-time solution.

Each run records `fidelity.json`, including kernel/app hashes, the firmware
contract in `test/g06-firmware.json`, and explicit gaps. The older term
"virt32" means constrained AArch32 userspace testing, not a MediaTek
board emulator. Existing double-buffered virtio-gpu and virtio-snd remain
surrogates. Framebuffer page count and bare speaker routing on G06 are
unmeasured. The newer QEMU kernel does not prove 4.19 compatibility.

Offline disassembly of exact V109's mstar_fbdev_mi.ko shows that fb open
checks MI daemon readiness and returns EINVAL when it is false. Probe
reads sys.ini/OsdDefine.ini; the current m1b/m1c images omit these files.
The audio HAL's normal PCM functions call MI_PCM_Open/Write and connect
MI_AOUT. Loading kernel modules alone does not reproduce stock startup.
Do not add a pretend ready flag to QEMU and count that as vendor bring-up.
Stock glibc midaemon/libmi/libapiVDEC are useful reverse-engineering leads,
but reusing them adds proprietary userspace and a large dependency closure.

The handoff's fastboot boot plan is also superseded by the device-track
findings: no temporary boot handler was found in the inspected G06 table,
and real kexec handoffs failed. No QEMU result authorizes a hardware boot
attempt. See main worktree research/g06-ram-boot-status.md and
research/firmware-implications.txt for evidence and remaining hardware gates.
