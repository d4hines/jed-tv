# Lean Core integration

Native PID 1 now runs the actual proved `TV.Core` on the Lean-written Zr
runtime. Core, menus, Settings views and file codec, Wi-Fi parsing/configuration,
volume policy, cooperative PCM service/conversion/device lifecycle, pixel
composition and OSD transaction policy are Lean. Runtime source is grouped
under `src/pid1/`; shared Zr remains under `src/lean/zr/`.

Linux effects, process creation/confinement mechanisms, filesystem operations,
device handles/ioctls and some cold-start hardware logic still contain Rust/C.
This is not a claim that every PID-1 instruction originates in Lean. App network
requests are not yet mediated by a domain/service broker. The developer root
HTTP/PTY interfaces bypass Core policy and are outside its security claim.

## Independent Lean ports while PR14 is implemented

The local integration includes three additional ports, without changing Core's
policy interface, the stored settings format, or the audio/display drivers:

- `TV.Storage` owns settings file/directory/device acceptance, cache geometry,
  and the atomic commit state machine. Nine proofs cover acceptance, root-only
  mount decisions, operation ordering, and failure never becoming success.
- `TV.Hardware.ColdPlan` owns the 35-flag/eight-parameter initialization schema,
  validation and exact tuner-byte preparation. Five proofs cover schema counts,
  uniqueness and successful preparation implying validation. The hardware worker
  submits owned text to the existing Lean executor; no boxed Lean value crosses
  the worker channel.
- `TV.Hardware.TeeBoundary` owns device-number and readiness-record parsing.
  Six proofs cover acceptance bounds and fail-closed transitions. Its scalar
  functions need no Lean runtime and can execute on the hardware worker. The
  accepted kernel record grammar is ASCII; non-ASCII records now fail closed.

The storage adapter passed 2,031,790 metadata cases, all 32 commit-result patterns,
12 injected effect/cleanup failures, concurrent scalar callers and real filesystem
checks under ARM32 QEMU. The cold-configuration adapter matched 224 host/ARM cases,
including Unicode, 64 KiB input, exact errors and worker ownership checks. The TEE
adapter passed its host/ARM32 corpus and a separate 41,120-case host comparison.
These tests exercise the actual production adapters, not only the specifications.

The combined native ELF SHA256
`1de2ea7d30bf27b48761955cc1eab26191403fa0d8491f8ffea488d4f33a4d43`
builds and passes the QEMU shell, app lifecycle, Home, volume, reload and reboot
checks. This image has not run on the physical TV. Evidence is under
`assets/research/lean-pid1-ports/20261101/` and
`assets/build/pid1-repl/qemu/1793576136778607000/`.

The clean-build failure was a race between TVInit and RPG's Cargo scripts over
shared Lake outputs. Both now hold the same build-only file lock while invoking
Lake. The isolated clean Nix build and eight-process contention check pass.
The combined local `nix build .#pid1 --no-link` also passes with all three ports.
Raw Linux operations, parts of startup/storage identity parsing, and the media
backend implementations still remain native; these ports do not claim an
all-Lean binary or implement PR14's new confinement policy.

## Verification

The integrated native ELF SHA256
`a2168dcd2b37990544d387b86d8d0841abb16c5e9b3f3d81dfce2d5846e89ea0`
passed Core/Zr, native root shell/lifecycle/volume/reload, all nine Settings UI
cases and all fourteen hostile-display cases in ARM32 QEMU. The hostile app
cannot access the private menu/raw framebuffer; Home selects the private menu
while the app is still corrupting its own frames, followed by TERM/KILL/reap.
Evidence index: `assets/build/shared-runtime-qualification.json`.

A stale surrogate kernel lacked file locking and an obsolete Slides binary
lacked shared-frame support. Tests now reject incompatible kernel media. The
qualified kernel is `demo/kernel/zImage.xz`, decompressed SHA256
`996d7306ab2e13e61c37636903378c3ce37caaa9e7605e640693015f1995784d`.

On the actual G06/V109, build `657c2a43...` remained alive past ninety seconds;
the owner confirmed the new Lean display and one-second audio jingle. The
temporary PR17 trace build `25a3b131...` also reached display/audio readiness
and the owner confirmed classic Doom picture and sound for over two minutes.
Those builds predate the shared runtime additions for the offline RPG; do not
substitute them for a physical qualification of the newer ELF above.

## Retained helpers and apps

The accepted steady-state service exceptions are stock `tee-supplicant`,
`wpa_supplicant`, and BusyBox `udhcpc`. With Wi-Fi active, this is PID 1, an app,
and three helpers. Settings also invokes transient CLI/shell/network commands.
G06 network binaries, credentials, control/reply sockets, PID files and hooks
are under root:root0700 `/run/tv-network`; the TEE devices/storage have their
separate root-only boundary. See `src/pid1/scripts/network/README.md` for scope,
including the separate G13 path and privilege assumptions.

Classic and Lean Doom/YouTube remain available together. PR16 is merged with a
follow-up that rejects closed TLS reuse and HTTP Connection:close tokens.
Infinite RPG keeps its original Lean engine, story and artwork, now in-process
and offline. App ID 5 has no networking permission. PizzINT has no active source,
menu or build entry. Development probes/candidates live under `research/`.

## Hardware observations

Remote Power shut down the native session, the owner confirmed picture and
backlight off, and no native/ADB return occurred for 120 seconds. The TV's physical
Power button then started Android. A remote-only wake attempt did not start it;
that remains a limitation to investigate. This is a one-shot RAM-boot workflow;
the next ordinary startup intentionally selects Android.
Evidence: `assets/research/power-button/20261101T225308Z/`.

PR17's physical smoke/cold-TEE/WPA/DHCP results are posted at
https://github.com/d4hines/dande-tv/pull/17#issuecomment-6565243318.
Raw logs remain private under `assets/research/confinement-trace/20261101T224930Z/`.
The temporary startup tracing hook is isolated and must not be merged/pushed.

The City Flight GPU path was physically visible. Renderer logs were mostly
22.4–23.1 fps and a captured app HUD reported 19.6 fps, with the simulation still
in slow motion. Results/fixes are in issue15. GPU work is separate from the
current Lean service qualification.
