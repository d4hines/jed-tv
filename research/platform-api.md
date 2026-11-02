# Platform API: the meeting point of the two tracks

The QEMU track (top-down) and the device track (bottom-up) meet here: the
short list of services the player needs from below. QEMU side status and
evidence: `QEMU.md`. Device side: `research/g06-platform-capabilities.md`.

## API v0 (what the player calls today, verified in QEMU on arm64 and armv7)

| Service | Linux interface | QEMU status | Needed from the TV |
|---|---|---|---|
| display | `/dev/fb0`: FBIOGET_V/FSCREENINFO, mmap, FBIOPAN_DISPLAY (double buffer), FBIO_WAITFORVSYNC if it really waits | works: 1280x720 XRGB8888, 2 pages | probe D below |
| audio | ALSA PCM by raw ioctl on `/dev/snd/pcmC0D0p`: HW_PARAMS, PREPARE, WRITEI_FRAMES, DELAY, DRAIN. S16_LE stereo; source is AAC 44.1 kHz, resampled to 48 kHz if 44.1 is refused | works at 44100 natively | probe A below |
| input | evdev EV_KEY from `/dev/input/event*` | works (virtio keyboard) | probe K below |
| network | TCP/UDP sockets; DHCP by the kernel (`ip=dhcp`) in QEMU | works | known: works from adb shell |
| clock | CLOCK_MONOTONIC; wall time comes from outside | works | known: no vDSO, 620 ns/call, fine |
| randomness | getrandom | works | known: works |

The player does TLS, DNS, MP4 demuxing, H.264 decoding (FFmpeg's decoder only,
NEON) and AAC decoding itself. No GPU, no MediaCodec, no Android services.
For 32-bit kernels: musl needs the legacy 32-bit time syscalls (`nanosleep`);
4.19 always has them.

## Requests for the device track, in priority order

### B. Software decode benchmark (most important)

Answers: can 4x A55 in AArch32 decode 720p H.264 and convert it to RGB in
real time (>= 24 fps end to end, with margin)? Runs as the shell user from
`/data/local/tmp`, needs no network, framebuffer or audio, changes nothing.

Files (repo-relative):
- binary: `assets/build/tv-bench/ytplay-armv7-c83a4327` (frozen copy, static,
  3,653,480 bytes, built 2026-10-31 14:07 PDT, sha256
  c83a4327d61c350692b6b049af8d0071ab335513e8f92689713091e061915857; NEON
  YUV->RGB conversion, multi-threaded)
- input: `assets/yt-stage0/record-PVGeM40dABA-visionos-20261031T200633Z/fixtures/136.mp4`
  (Coffee Run, YouTube itag 136: H.264 High@3.1, 1280x536, 24 fps, byte-exact
  from YouTube; sha256 1e46c7c5238c2750e9beb516320b45eeea27890d66eba0f8730339bf223a7c9e)

```
adb push assets/build/tv-bench/ytplay-armv7-c83a4327 /data/local/tmp/ytplay
adb push assets/yt-stage0/record-PVGeM40dABA-visionos-20261031T200633Z/fixtures/136.mp4 /data/local/tmp/136.mp4
adb shell /data/local/tmp/ytplay ytplay.mode=bench ytplay.file=/data/local/tmp/136.mp4 ytplay.frames=1200 ytplay.threads-list=1,2,4
adb shell cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq   # during/after the run
adb shell rm /data/local/tmp/ytplay /data/local/tmp/136.mp4
```

Output: decode-only fps at each decoder thread count, decode+convert fps end
to end, then YUV->RGB conversion alone (1280x536 -> 1280x720) on 1, 2 and 4
threads. Reference (same decode/convert code, aarch64): M2 under HVF 1269 / 3305 fps
decode-only at 1/4 threads, convert 0.39 / 0.19 ms/frame at 1/4 threads.
QEMU faithful (emulated 32-bit, not predictive): 60 / 185 fps, 19 / 13 ms.

### D. Framebuffer (as root, if SELinux allows)

- FBIOGET_VSCREENINFO / FSCREENINFO for fb0 and fb1: xres/yres,
  xres_virtual/yres_virtual (room for double buffering?), bits_per_pixel,
  RGBA offsets/lengths, line_length, smem_len.
- Do FBIOPAN_DISPLAY and FBIO_WAITFORVSYNC work, and does the latter wait?
- Does a test pattern written to fb0 reach the panel (photo, not just
  screencap), and does it sit above the video plane?

### A. ALSA

- Which card/device drives the speaker (card0 has D0-D3, D7)?
- Accepted formats, rates (is 44100 accepted?), channels, period sizes
  (`/proc/asound/card0/pcm*p/info`, `hw_params` while playing, or a HW_PARAMS
  refine as root).
- Hints on whether it makes sound without the Android audio HAL / utopia
  (amp enable, mute GPIO, DSP routing): init scripts, HAL config.

### K. IR keycodes

Linux keycodes on event2 for: up, down, left, right, OK, back, play/pause.

## Answers so far

(Device track: append results here with date and evidence path.)
