# Lean app QEMU diagnosis (2026-11-01)

Worktree base: f56ab19. All runs below are offline ARM32 QEMU; no TV connection,
boot, flash, or physical audio/display test was performed. Defaults remain Rust
YouTube and Rust/C Doom. The changes concern the retained Lean Doom app and test
reliability, plus the common launcher's readiness marker.

## Findings

1. **The first input could arrive before the launcher opened its input devices.**
   `home screen: ...` was logged by `session` before `home` opened input and drew
   both buffers. The Lean YouTube harness immediately sent Enter. The first
   reproduction saved a launcher screenshot and zero PCM despite returning
   success: Lean YouTube never ran. Moving the readiness log after input-open and
   both draws makes one Enter reliable. The harness now requires the app-1 launch
   and a Lean app entry log before claiming a screenshot run succeeded.

2. **The old Doom QEMU harness lacked the existing QEMU settings identity.**
   It was missing both `hello.qemu=1` and `tvinit.iface=...`. Current root-owned
   settings correctly rejected the launch: `No qualified settings storage on
   this system`. The harness now supplies `hello.qemu=1`; it does not weaken the
   settings guard.

3. **Lean Doom's PCM feeding was coupled to video frames.**
   The old code mixed a buffer in a background task, but only submitted it and
   scheduled the next task from `pumpAudio`, once per main loop. Its target was
   4096 frames (85 ms at 48 kHz). QEMU render/present cost was commonly 140–210 ms
   per frame, before other work, so the PCM queue drained even if the mixing task
   itself was fast. A 32-second attract-mode run produced 5.642 seconds of PCM,
   only 17.8% of the logged active duration.

   The retained Lean app now has one dedicated Lean IO producer. It owns the
   sound system and continuously fills the PCM queue independently of rendering.
   The game submits immutable sound commands through an atomic `IO.Ref` mailbox.
   The worker closes its mailbox on failure and joins on normal application exit.
   This adds no native audio implementation or alternate backend.

   The first matched run produced 27.544 seconds of PCM, 86.4% of logged active
   time, about 4.9 times the old output. It still rendered about 3–5 fps during the
   demo under TCG. This is a large starvation improvement, not proof of perfect
   audio or playable hardware performance. No volume constants were changed.
   The engine's current default music and SFX controls are both 8/15.

4. **Lean YouTube renders and produces PCM in the offline test.**
   With the readiness race removed, the 640x360 generated clip launched with one
   Enter and stopped cleanly after 25 video frames in roughly five seconds. A
   separate ten-second capture had continuous nonzero PCM (100 ms RMS windows
   around 2890 for the generated tone). Video decoding lagged the audio clock:
   a screenshot about halfway through the first test showed video PTS 0.792 s.
   This is consistent with the existing software-decoder performance limitation;
   it is not a demonstrated codec-correctness or gain failure.

   The shared-frame/TCP-PCM bridge test also passed: 21 video frames, one audio
   open, 192224 frames delivered, no PCM protocol errors. That bridge is a QEMU
   adapter test, not the G06 hardware driver.

## Regressions and evidence

- `test/doomtv/check-audio-coverage.py RUN_DIR` measures PCM duration versus the
  logged active interval. Its coarse 50% floor fails the old run and passes the
  worker run; it catches gross starvation, not every audible gap.
- QEMU tests request QMP quit on cleanup rather than unconditionally killing the
  emulator. Streaming WAV headers can still contain zero lengths; the coverage
  reader accepts that format while checking actual payload length.
- The final attract test also quits through Doom's own menu, exercising worker
  shutdown rather than only force-killing the VM.
- Evidence is in this worktree's ignored `assets/build/diagnostics/`:
  `yt-offline*` (missed first key), `yt-fixed*`, `yt-bridge*`, `doom/` (settings
  rejection), `doom-fixed/` (old PCM loop), `doom-worker/` (first worker run),
  `doom-final/`, and `doom-audio-{before,after}.json`.

Toolchain: Lean 4.35.0, Zig 0.17.0, QEMU 11.1.3; four AArch32 Cortex-A55 vCPUs,
1430 MiB RAM, virtio GPU/audio. Surrogate kernel SHA256:
`996d7306ab2e13e61c37636903378c3ce37caaa9e7605e640693015f1995784d`.
Lean YouTube SHA256:
`f888c9a2dba4d851b31bdded8115018149eff5d71c00b90a0f2136403406413f`.
Baseline Lean Doom SHA256:
`48f53dab124c6c8d15ba276400b448549cbe8fdd368d034ec2997f63fe1377db`.
Final worker Lean Doom SHA256:
`2e6a48c9efc282750251aada5aefcc9c999039a6291ea00884cbe2cc2948d3a4`.

TCG frame rates are not TV measurements. The remaining physical sound-level,
cold-start, and real-time decoder questions require a separately authorized
hardware test.

Final validation: `doom-final/qemu-20261101T132220` passed all five screenshots,
played DEMO1, and exited normally with status 0 through the Quit menu; the PCM
worker joined before the launcher observed exit. The final coverage result is
recorded in `assets/build/diagnostics/doom-audio-final.json`.
