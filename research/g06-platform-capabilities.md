# G06 platform capabilities, bottom-up (2026-10-31)

Live, read-only survey of the ORIGINAL TV (G06, V8-T221T04-LF1V109, Android 11,
kernel 4.19.116+, unlocked/orange) over Ethernet ADB from the Mac. Evidence:
`assets/research/g06-capabilities-20261031/` (kernel config, surveys, codec
XMLs, probe output, screenshot). Probe source: `research/tv-probe/`.

Nothing on the TV was modified except `/data/local/tmp/tv-probe` (the probe
binary, sha256 a12c5da6…). No boot, partition, setting or input changes.

## Headline facts for the top-down track

| Item | Finding | Consequence |
|---|---|---|
| Panel | **1280x720**, 60 Hz (50 Hz mode too); HDR10/HLG flags | Never fetch above 720p. 1080p streams are wasted bytes and decode work. |
| CPU | 4x Cortex-A55 (0xd05), 750–1500 MHz, ondemand | — |
| Kernel ISA | **Linux/arm 32-bit** (CONFIG_CPU_V7, VMSPLIT_3G, HIGHMEM), not just 32-bit userspace | Stage 5 AArch64 means replacing the kernel too; inside today's kernel, only armv7 binaries run. |
| CPU features (AT_HWCAP/2) | neon vfpv4 idiva lpae + **aes pmull sha1 sha2 crc32** | rustls/ring AES-GCM and SHA-2 get hardware acceleration in AArch32. |
| RAM | MemTotal 1.43 GB, ~740 MB available under Android | Comfortable for a minimal player. |
| Memory bandwidth | memcpy ~2.1 GB/s | — |
| CPU throughput | FNV hash 294 MB/s single core, 1010 MB/s on 4 threads (3.4x) | Threads scale; software decode is plausible to benchmark. |
| HW video decode | OMX.MS **AVC, HEVC, VP9, AV1** up to 1920x1088, plus `.secure` variants; all **tunneled-playback** | Every YouTube codec has a hardware decoder. Tunneled mode puts video on MStar's video plane (XC/MVOP) beneath the UI (GOP OSD). |
| DRM | Widevine (`libwvdrmengine.so`) and OP-TEE (`/dev/tee0`) present | Not needed for ordinary YouTube streams. |
| Audio | AudioFlinger mixer at 48 kHz, 16-bit stereo to the speaker; ALSA card0/card2 (MStar `platform/sound`) | — |
| Input | evdev: `MStar Smart TV IR Receiver` (event2), `MTK TV KEYPAD` (event0), plus virtual devices | The remote is plain evdev. |
| Network | Built-in MStar EMAC (`eth0`, 100 Mbit). **43.6 Mbit/s** measured laptop→TV TCP into a static Rust binary | Far more than 720p needs (~2–5 Mbit/s). |
| Clock | MStar MMIO clocksource; **no vDSO** (`CONFIG_VDSO` unset, no arch timer) → `clock_gettime` ≈ **620 ns** syscall; `sleep(1ms)` oversleeps by ~68 µs on average, 515 µs worst | Fine for A/V sync, but avoid clock reads in hot loops. |
| Wall clock | **Wrong: reads 2025-01-17** (no NTP; the TV has no internet) | TLS certificate validity checks will fail unless time comes from elsewhere (NTP via the laptop, or replay). |
| Randomness | `getrandom()` works; `/dev/urandom` ok; `/dev/hw_random` exists (root:system, denied to shell) | Covered. |
| GPU | Mali (`mali_kbase`), GLES 3.2, Vulkan HAL present; `/dev/mali0` and `/dev/ion` are world-rw | — |

## What a static Rust binary gets from `adb shell` today

`armv7-unknown-linux-musleabihf`, fully static, linked with rust-lld (no C
toolchain). It runs from `/data/local/tmp` as uid 2000 in `u:r:shell:s0`, with
**Seccomp: 0** (no syscall filter), CapEff 0, NoNewPrivs 0.

| Platform API | Status for the shell-launched binary |
|---|---|
| network (TCP/UDP) | **Works** (inet group). Can listen and connect on the 192.168.77.0/24 link. |
| DNS | musl resolver fails ("Try again"); the TV's configured DNS is 8.8.8.8 via gateway .1 (the laptop), which doesn't forward. Supply our own DNS or fixed IPs. |
| clock | **Works**, but the wall clock is wrong (see above). |
| randomness | **Works** (getrandom). |
| input | **Works**: can open `/dev/input/event*` read-only (input group); EVIOCGNAME ok. Reading key events should work without a grab (not yet tested with key presses). |
| display | **Denied**: `/dev/graphics/fb0`/`fb1` are root:graphics 0660. Options: go through SurfaceFlinger (libgui/NDK, bionic-linked), or own the framebuffer once Android userspace is gone (stage 2+, as root). `screencap` from shell works, for verification. |
| audio | **Denied**: `/dev/snd/*` are system:audio 0660. Options: AudioFlinger via AAudio/OpenSL (bionic-linked), or ALSA directly once Android userspace is gone. |
| HW decode | Not reachable from a static musl binary. Options: NDK `AMediaCodec` (bionic-linked), or drive MStar's proprietary utopia/VDEC ourselves (very hard). |

## Kernel-side map (useful for stage 2+ on the real chip)

- Proprietary MStar/MediaTek modules: `utpa2k` (Utopia2K HAL, 12 MB), `mik`,
  `misck`, `kdrv_xc` and `xcker` (scaler/XC), `kdrv_alg`, `mstar_fbdev_mi`
  (fbdev on the GOP OSD), `mali_kbase`, `dtv_driver`, `hdcp1x/2x`, `mwgifker`.
  The kernel can load modules (CONFIG_MODULES=y), and KEXEC=y.
- Built-in kernel config includes DRM core (no SoC DRM driver is bound; video
  is not on DRM), FB, ALSA/SoC, evdev, uinput, HID, USB HID, USB audio,
  usbnet (RTL8152/AX88179/CDC), TUN, ION, OPTEE/TEE, SECCOMP_FILTER, BPF.
  `/proc/config.gz` is readable by shell.
- DT top-level nodes include `xc`, `scaler`, `gflip`, `gop_int`, `hvd_int`,
  `evd_int`, `mvd_int`, `emac`, `ir`, `keypad`, `alsa`, `mstar-rtc@0`.
  The DT node names are listable by shell, but property contents are denied;
  they can be read from the dump (`assets/backups/v109-stock-20261031`).

## Setup notes

- Host: Mac dongle `en8` = 192.168.77.1/24, no router. TV: 192.168.77.2/24,
  gateway .1, DNS 8.8.8.8/8.8.4.4 (set statically in TV settings).
- The TV goes into suspend-to-RAM (MStar STR) when idle: BOOTTIME −
  MONOTONIC was about 41 minutes at 20:15Z. The Ethernet link drops while it
  sleeps.
- `screencap -p` stdout is prefixed with vendor debug text ("Init wrapper sys
  mutex…", "open mma dev failed"). Strip everything before the PNG magic.
- Build: `cd research/tv-probe && cargo build --release --target
  armv7-unknown-linux-musleabihf`. The `libc` crate is deliberately avoided
  because the host rustc 1.80 cannot link build scripts against the macOS 27
  SDK. On 32-bit musl ≥1.2, link `__clock_gettime64`, not `clock_gettime`.
  Output: `assets/build/tv-probe/`.

## Open questions / next probes

1. Read remote key events from `event2` (needs someone pressing keys).
2. Software decode benchmark (dav1d/AV1, VP9, H.264 at 720p30) as armv7 on the TV.
3. Can a shell-launched native binary put pixels and sound on the TV through
   SurfaceFlinger/AudioFlinger (bionic NDK build)? That is the cheapest path
   to an on-TV demo before stage 5.
4. Laptop NAT/NTP so the TV can reach YouTube and get a correct clock (needs
   the owner's sudo; deliberately not done).
