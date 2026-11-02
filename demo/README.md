# Graphical PID-1 demo

From this branch, run:

```sh
nix run .#pid1
```

Nix builds the Lean control logic into the ARM32 app from source, starts QEMU, and serves a generated
16-second video/audio sample locally. No TV connection, stock firmware,
private recordings, manually prepared assets, Rust installation or existing
replay server is required. First launch downloads build dependencies.

A window with the TV's TCL remote opens next to QEMU (`remote.py`): Power,
Mute, D-pad with OK, Back, Home, Vol/Ch rockers, NETFLIX, prime video and
YouTube, nothing else, because that is the handset. Each button sends the
keycode the TV's IR receiver emits to PID 1's virtual IR receiver
(`remote.udp=PORT`), so even buttons a keyboard cannot type (YouTube,
Netflix, prime video, Ch +/-) reach the apps. Holding a button repeats.
A keyboard works too: arrows, Enter, Escape, Page Up/Down for Ch +/-.
The menu includes the generated video, Slides, Doom, About and Power off.

Every app is playable with the eleven buttons PID 1 forwards (PID 1 keeps
Power, Home, Volume and Mute). Home always returns to the launcher.

| App | D-pad / OK / Back | Ch + / Ch - | NETFLIX | prime video | YouTube |
|---|---|---|---|---|---|
| Launcher | select / open | | | | open YouTube |
| Video, YouTube player | OK pause, Left/Right 10 s, Back stop | +/- 60 s | | | |
| YouTube search | move / type / back | | | | |
| Slides | Right, Down, OK next; Up, Left back; Back leave | next / back | | | |
| Doom | move, turn / fire, select, yes / menu | use, map zoom in / run, zoom out | previous weapon | next weapon | automap |
| Infinite RPG | choose / pick / leave | | | | |
| City flight | pitch, roll / filter (fly again after a crash) / exit | throttle | kamikaze autopilot | camera | next scenario |
| Axiom | move, turn / mine / pause | look up / down | jump (hold: rise in flight) | build | next block (hold: sink in flight) |
| Axiom, paused | OK resume, Back leave | | respawn | toggle flight | |
| Snake | Left/Right turn, OK again, Back leave | | | | |
| Settings | move / open / back | connect / delete (password) | | | |
The replay server has its own ephemeral loopback port and stops with QEMU.

This is the **graphical virtio surrogate**, with Linux 6.18.57, not the
stock G06 kernel. The separate `faithful` hardware-contract environment
remains serial-only. The window and console identify which environment runs.

The pinned generic Linux kernel and full configuration are under `kernel/`;
`manifest.json` records its hash, upstream source and rebuild command.
Rust compiler/standard-library downloads, Cargo dependencies, Freedoom and
Nixpkgs are pinned. Freedoom's license is retained in the media derivation.
Video and tile assets are generated from FFmpeg's test pattern and sine source.

For the headless end-to-end check:

```sh
nix run .#pid1 -- --check
```

This verifies active 1280×720 output, local video/audio playback, Home,
Slides, Doom, the other apps, and the remote's YouTube button arriving
through the virtual IR receiver; it saves `assets/build/pid1-demo-check.png`
in the working directory.
The launcher supports macOS (Cocoa) and Linux (GTK desktop); initial build
and runtime verification were on Apple Silicon macOS.

The menu and child-supervisor policy are Lean-generated code. See
`../lean/README.md` for scope and `nix run .#lean-check` for executable ABI
checks. `nix run .#pid1 -- --check-supervisor` exercises per-app UID, no-process/no-exec confinement, thread compatibility,
PID1-owned remote input and Home escalation against an app that ignores SIGTERM. Existing proofs are
optional; the normal build is focused on executable development.

The integrated hardware branch also tests the TV adapter protocols:

```sh
nix run .#pid1 -- --check-tv-bridge
```

This runs the same confined ARM32 apps through shared RGBA frames and a TCP
PCM test peer. A QEMU-only Rust init forwards frames to virtio for inspection.
It checks video/audio submission, nested YouTube Back, Slides and Doom, while
retaining the normal framebuffer/ALSA test above. PCM timing is synthetic;
this does not emulate Android, the panel, amplifier, or vendor kernel drivers.
Each headless run saves logs, binary/kernel hashes and adapter information in
`assets/build/integration-qemu-*`. Exact-kernel confinement remains a separate
check described in `research/ISSUE_12_VALIDATION.txt`.

The app interface is documented in `research/APP_API_V1.md`, with a C-compatible
header in `src/platform/include/tv_app_v1.h` for Rust/Lean clients. The app output
includes `libplatform.a`. Check buffer ownership and ABI behavior with:

```sh
nix run .#pid1 -- --check-contract
```

The shared-frame test now uses byte-range ownership locks. The generic kernel
therefore enables `CONFIG_FILE_LOCKING`, matching the stock TV kernel.
