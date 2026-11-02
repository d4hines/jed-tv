# agent2 (bottom-up TV track) handoff, 2026-10-31 ~20:25Z

Role: explore the TV upward toward the platform API, while agent1 (the
top-down/QEMU track, also in tmux) shrinks the YouTube client downward.
Main write-up: `research/g06-platform-capabilities.md`. Raw evidence:
`assets/research/g06-capabilities-20261031/` and
`assets/research/g06-ethernet-reconnect-20261031T2012Z/`.

## Connection (working as of 20:25Z)

- The TV on the cable is the **ORIGINAL G06** (V8-T221T04-LF1V109, Android
  11, serial 4C0490290FA05A0C0, unlocked/orange), not the G13. Both TVs use
  192.168.77.2, so check `getprop ro.product.device` first.
- Mac: USB LAN dongle `en8` (RTL8153), service "USB 10/100/1000 LAN", manual
  **192.168.77.1/24, no router**. The TV's static config uses .1 as its
  gateway, and the link flapped while the Mac used another address.
- `adb` = android-tools 37.0.0 in the owner's nix profile. The laptop key
  (MD5 82:2A:3D:…:CC:42) is authorized on the TV. Target: `192.168.77.2:5555`.
  The adb server is shared with the owner, so don't run kill-server or
  disconnect.
- Before connecting, check `route -n get 192.168.77.2` shows `en8`. When
  the link is down, traffic falls back to Wi-Fi.
- The TV sleeps when idle (MStar STR), which drops the Ethernet link. The
  link dropped 20:18:50Z and came back 20:19:43Z. Ask the owner to wake it.
- The TV has no internet. Its clock reads 2025-01-17 and DNS fails.
  Laptop NAT/NTP needs the owner's sudo and has not been done.

## Done

- Read-only surveys: kernel config (`/proc/config.gz`, 32-bit ARM kernel),
  devices, modules, display (1280x720@60), codecs (HW AVC/HEVC/VP9/AV1 ≤1080p,
  tunneled), audio (AudioFlinger 48k s16 stereo), input (IR = event2),
  network (MStar EMAC, 100 Mbit).
- `research/tv-probe/`: a std-only static Rust probe
  (`armv7-unknown-linux-musleabihf`, linked with rust-lld; no `libc` crate
  because host rustc 1.80 can't link build scripts against the macOS 27 SDK).
  Ran it from `/data/local/tmp/tv-probe`. Output:
  `assets/research/g06-capabilities-20261031/tv-probe-run1.txt`. Seccomp 0,
  network ok (43.6 Mbit/s in), getrandom ok, clocks ok (no vDSO, ~620 ns per
  read), evdev readable; fb0/fb1 and ALSA are DAC-denied to shell.
- Extra facts (also in the issue comments): `/dev/rtc0` is world-rw by DAC
  but SELinux denies shell; kernel has `CONFIG_MP_Android_DUMMY_MSTAR_RTC=y`.
  Wi-Fi is a USB dongle (MT7663/76x3/76x8/79x1 or RTL8192EU/8733BU/88x2CU
  drivers shipped), and no Wi-Fi driver is loaded. Shell can't run `dmesg`
  (klogctl denied) or read `/proc/cmdline`, `/proc/iomem`, `/proc/asound/*`,
  or fb/sound/rtc sysfs attributes.
- Posted G06-labelled partial answers on GitHub issues #3, #4, #5, #6, #9.
  Left #1, #2, #7, #8 (insufficient data) and #10 (Stage 0 already answered it).
- Messaged agent1 with the findings. Its reply set the target platform API v0:
  **fbdev + ALSA PCM (raw ioctls) + evdev EV_KEY + sockets**, software
  H.264 High 720p≤30 + AAC-LC decode in-app, CLOCK_MONOTONIC, getrandom;
  wall time is supplied externally. agent1 is moving Linux-stage QEMU to
  32-bit `qemu-system-arm -cpu max -m 1430M`, with fb 1280x720.

## Open: agent1's requested probes (in priority order)

1. **Software decode benchmark.** agent1 will supply a static armv7 ffmpeg
   H.264-only decode-bench plus a 720p fixture (built on eduardo-vm). Run it
   on the TV at 1/2/4 threads. Not received yet.
2. **fb0 details:** FBIOGET_VSCREENINFO/FSCREENINFO (xres/yres, virtual size
   for double-buffering, bpp, RGBA offsets, line_length, smem_len),
   whether FBIOPAN_DISPLAY and FBIO_WAITFORVSYNC work, and whether a pattern
   written to fb0 appears on the panel above the video plane (needs a photo).
   Blocked for the shell user (DAC + SELinux). Not resolved.
3. **ALSA:** which card/device drives the speaker, and accepted
   formats/rates (44100?), channels and period sizes; would it make sound
   without the audio HAL/utopia (amp/mute GPIO, DSP routing)? Blocked for the
   shell user. Non-root leads: `/vendor/etc/audio_policy_configuration.xml`,
   `audio_policy.conf`, `/vendor/lib/hw/audio.primary.mt5867.so`, and the
   V109 dump in `assets/backups/v109-stock-20261031/`.
4. **IR keycodes** on event2 for up/down/left/right/OK/back/play-pause. Needs
   someone pressing remote buttons while `adb shell getevent -l
   /dev/input/event2` runs.

The replay server agent1 plans for on-TV tests: 192.168.77.1:8090 (plain
HTTP); see `research/yt-stage0/PROTOCOL.md`.

## Other state left behind

- Uncommitted: `AGENTS.md` (new "CURRENTLY CONNECTED" entry for the G06,
  plus pointers), `research/g06-platform-capabilities.md`,
  `research/tv-probe/`, this file. Nothing committed or pushed.
- On the TV: `/data/local/tmp/tv-probe` (sha256 a12c5da6…). Nothing else
  changed.
- Laptop: Rust target `armv7-unknown-linux-musleabihf` added via rustup;
  `assets/research/tools/r8-8.3.37.jar` downloaded from Google Maven
  (SHA-1 matched the published .sha1) but not used.
- `research/ROOT_READBACK_NOTES.txt` references tooling in
  `/home/eduardo/tv_hack` on eduardo-vm. That directory is not readable by
  our account there, and that tooling is not on this laptop.
