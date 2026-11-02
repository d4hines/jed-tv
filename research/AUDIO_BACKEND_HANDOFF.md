Current status (2026-11-01)
The app path now uses direct Rust MI PCM/AOUT. The old AAudio bridge and
Rust/Bionic build wrappers mentioned below are retired. Physical Doom effects
and local test video audio passed, with -27 dB speaker attenuation, app gain
0.03 and Doom input boost 4. Stock hardware bootstrap is still required.
The remainder preserves the investigation history.

## Correction from live testing (2026-11-01)

The earlier claim that `MI_PCM_GetHandle` was read-only was wrong. On this
firmware it allocates a named metadata slot with PID0 and a handle containing
bit0x100. Three such slots caused the first direct Open to try closing an
unopened lower device and fail. The current probe source no longer calls
PCM GetHandle. Root released exactly 0x81000100/101/102 through the verified
metadata-only MI_PCM_Close branch; all three succeeded. See
`assets/build/backend-live-silent-pcm.txt`, `backend-live-window-inspect.txt`,
and `backend-pcm-probe-cleanup.txt`. Do not run older probe binaries.

The first direct PCM write test did not reach Start or Write. No direct
audio output has been established. AOUT GetHandle was separately audited
and does not allocate, and the new routing adapter has 15 host tests.

# Native Rust audio backend

Worktree: `codex/audio-backend`. This is an intermediate replacement for the
Java audio bridge, retaining Android AudioFlinger and the stock audio HAL.
The final target is a Rust direct MI PCM/AOUT backend inside PID1; no direct
hardware path is claimed yet. The standalone service is temporary test and
migration scaffolding, not a permanent additional privileged process. The root coordinator owns all TV access.

## Files and current artifacts

- `src/audio/bridge.rs`: native Rust/Bionic TCP server, unchanged O/W/D/R/F wire
  protocol, default `127.0.0.1:8878`; Java remains on 8877 for A/B testing.
- `src/audio/protocol-probe.rs`: static ARM32 client; default is 14 silent tests.
- `tools/audio/rust-build.sh`: now accepts `TV_AUDIO_BUILD_DIR` so isolated
  builds do not overwrite another agent's startup object.
- `tools/audio/android-linker.sh`: consumes that startup-object environment.

Build from the worktree:

```sh
TV_AUDIO_BUILD_DIR="$PWD/assets/build/audio-agent" \
  ./tools/audio/rust-build.sh src/audio/bridge.rs \
  "$PWD/assets/build/audio-agent/rust-audio-bridge-v3"
rustc +1.92.0 --edition=2021 --target armv7-unknown-linux-musleabihf \
  -C linker=rust-lld -C target-cpu=cortex-a55 -C panic=abort -C opt-level=2 \
  -C strip=symbols src/audio/protocol-probe.rs \
  -o assets/build/audio-agent/audio-protocol-probe
```

AAudio configuration is MMAP NEVER, shared, performance NONE, PCM I16,
requested rate/channels and media usage. Opening rejects changed PCM format.
No ART, app_process, Java or private C++ object layout is used by this server.
The shared libraries beneath AAudio still remain.

## Protocol and lifetime

The client owns 250 ms startup priming, just as with Java. Server counters
start at zero and reset recreates the AAudio stream. `getFramesRead` matches
the former AudioTrack playback-head meaning; it is not an acoustic timestamp.
Writes use AAudio timeout zero and bounded retries, handle partial writes and
reject impossible over-writes. Maximum payload is 1 MiB. At most four clients
are live; valid idle streams may remain open while paused. Partial command
payloads time out after three seconds. Drains stop waiting after five seconds.

The current software gain is read from `/tmp/app-volume`; no file is created
or modified. Missing/invalid input means muted; valid input clamps to 0..0.03.
Boost retains the existing 1..4 semantics. No test raises volume.

## Test requests for the coordinator

Run the service alongside Java with the existing Android environment and
library paths, then run the static client locally on the TV:

```sh
/tmp/audio-protocol-probe 127.0.0.1:8878
```

The default test is silent. It covers 44.1/48 kHz mono/stereo, fragmented
transport, exact written counts, monotonic read counts, reset, drain, invalid
format/command/block sizes, truncated payloads and a valid client afterward.

Only after owner confirmation, an optional separate invocation emits exactly
one second of a four-note jingle at the existing server gain:

```sh
/tmp/audio-protocol-probe 127.0.0.1:8878 --jingle
```

Switch applications by changing only launcher `TV_AUDIO_ADDR` to localhost
8878; retain all existing app binaries and the Java service until parity is
established. No app API changes are needed. Rollback is the original 8877.

## Cold initialization is not yet proven

The first successful coordinator test used already-running AudioFlinger,
HAL, mixer policy and amplifier state. It proves a native client route, not
cold setup without Java. `--init-policy` is an explicit, default-off operation
which reproduces Java's AudioSystem media range 0..100 and speaker index 30.
It does not alter the separate software gain file. Root should test that flag
only as a deliberate policy-initialization experiment. Amplifier setup remains
separate vendor code and must not be guessed from these policy values.

## Next removal stage

The stock audio HAL's actual playback route is MI_PCM_Open/Write and
MI_AOUT_ConnectInput. Mere presence of ALSA nodes does not establish a working
speaker route. Coordinate exact wire structures and initialization dependencies
with the midaemon investigation before replacing those libraries. Preserve the
same server protocol so applications and their Lean replacements remain stable.

### Recovered direct PCM ABI (offline)

The midaemon agent recovered exact V109 envelopes and structures in
`assets/research/midaemon-replacement/pcm-wire-types.txt`:

- Every ioctl takes a 16-byte outer envelope: u16 argument size, six pad bytes,
  then u64 user argument pointer.
- PCM Init `0xc0013400`: one zero byte.
- PCM Open `0xc0803402`: 128-byte structure with handle at 0, name[64] at 4,
  blocking/mixing at 68/69, 8-byte format at 72, channel map[10] at 80,
  duration flag at 120 and duration at 124.
- PCM Write `0xc0383408`: 56-byte structure with handle at 0, pointer at 8,
  byte count at 16, PTS at 24, flags at 32, metadata index at 40 and returned
  byte count at 48.

These are recovered layouts, not a validated playback recipe. Open format
values, routing, timestamp flags and delay accounting still need evidence.
Do not call init/open blindly while the production HAL owns the same path.

## Coordinator hardware results received

Root reported these live results; this agent did not connect to the TV:

- Toolchain probe passed std thread/file/Unix socket and AAudio symbols:
  `assets/build/audio-agent/live-toolchain-probe.txt`.
- Initial native service passed the existing silent app API probe, 48 kHz
  stereo; AAudio reported capacity 6924 and burst 3462 frames:
  `assets/build/audio-agent/live-silent-protocol.txt`.
- v3 (without `--init-policy`) passed all fourteen static protocol cases:
  `assets/build/audio-agent/live-protocol-suite.txt`.
- Owner's software gain remained zero during silent tests. Root subsequently
  ran the separately approved one-second jingle at software gain 0.0025; the
  owner confirmed hearing it. Gain was restored to zero. Evidence:
  `assets/build/audio-agent/live-jingle.txt`. This establishes warm audible
  parity; explicit policy initialization is still a separate test.

v3 SHA256: `78001a6f7d1d66bc1742c1b3957b2f6d5659e049c628e7f94381acd150b2d733`.
Protocol probe SHA256: `cd17589b48ea3af51348a1f662d481d9029ce738efa4f2708b01c505b643eaf6`.
A proposed (not applied) integration patch is at
`assets/build/audio-agent/integration.patch`: opt-in backend selector,
unchanged production port 8877, existing Java backend retained as default.

## Direct MI progress (2026-11-01)

Root ran `mi-readonly-probe` successfully in the native PID1 environment.
Evidence: `assets/build/backend-direct-inspect.txt`. Main speaker path 4 has
handle 0x17000001. HW DMA Reader1/Reader2 and SW DMA Reader2 resolve to handles
0x81000100/0x81000101/0x81000102. Buffer and delay getters succeeded and read
zero while the menu was silent. GetHandle existence does not imply that a
stream is currently open or owned by us.

`src/audio/mi-pcm.rs` is a small Rust adapter intended for PID1 embedding.
`src/audio/mi-silent-probe.rs` is temporary test scaffolding. Static ARM binary:
`assets/build/audio-agent/mi-silent-probe`, SHA256
`203365d76d0dbc6b2d1ffbd47b021656232bd475cbd36aace61f1aaa847e7f4a`.
The coordinator may run `/tmp/mi-silent-probe --exclusive-silent-trial` only
after closing all app audio streams. It uses the existing initialized kernel
state; it does not call init, route, mute, volume, or change any service.
It opens HW DMA Reader1 with 48k stereo S16LE, zero channel-map (the stock
stereo mapping), nonblocking, and 100ms buffer duration. It starts the stream,
writes exactly 100ms of zero PCM in ten 10ms blocks, queries buffer fill,
then stops/closes only a handle obtained by a successful Open. Failure after
Open also triggers cleanup. Open failure never closes a preexisting handle.
There is a two-second userspace retry/drain deadline; individual kernel ioctls
are not yet proven bounded, so an external timeout remains prudent.

Exact additional lifecycle ABI from libmi disassembly:

- Start `0xc0083404`: handle u32, reserved byte and three zero pad bytes.
- Stop `0xc0043405`, Close `0xc0043403`: handle u32.
- Pause `0xc0043406`, Resume `0xc0043407`: handle u32 (not used yet).
- BUFFER_INFO output is three u32 words: capacity bytes, queued bytes, free
  bytes. Stock HAL divides queued bytes by channels*(bits/8) for frames.
- OUTPUT_DELAY is configured delay in milliseconds on HW DMA Reader1;
  it is not the instantaneous queue latency.

AOUT routing, independently recovered from the exact HAL and libmi:

- Android speaker device 2 maps to MI MAIN_SPEAKER path 4.
- GetHandle `0xc0081116`: `{path:u32, handle:u32}`.
- Open `0xc0581103`: 88-byte wire: handle at 0, name[64] at 4, path at 68,
  custom-volume-table flag byte at 72, u64 table pointer at 80.
- ConnectInput `0xc00c111c`: 12 bytes, output handle, input handle, reserved
  zero byte and padding. DisconnectInput `0xc008111d`: both handles.
- GetConnectedNum `0xc010111e`: handle, input-direction byte and padding,
  module selector, returned count (16 bytes). GetConnected `0xc018111f` adds
  the u64 handle-array pointer at offset16. Kernel helper 0x1e89a8 maps module
  0 to all modules and 0x81 to PCM. GetConnectedNum at 0x1eae6c confirms the
  zero wildcard. Direction 1 selects inputs. The read-only v2 probe now reads
  count then a bounded (maximum32) handle array per existing output.
  Evidence: `assets/research/audio-agent-direct/aout-module-filter.txt` and
  `aout-connect-get-wire.txt`. Binary `mi-readonly-probe-v2` SHA256:
  `46272a8f84d57584eea25c171711f3959b5461ccabf535d99c7ab62504a95cd4`.

Direct PCM hardware write/drain and routing remain unproven until the root
coordinator reports the bounded trial. Even a successful warm direct write
will not prove cold hardware initialization without AudioFlinger/vendor HAL.
The software `/tmp/app-volume` control does not independently mute a raw MI
stream: the direct probe emits only zero PCM, but hardware gain/previous route
state must be considered separately by the coordinator before Start.

## Static direct-driver builds (coordinator integration)

Direct driver probes now use `tools/toolchain/build-static-rust.sh`: pinned
Rust 1.92, ARMv7 musl, Cortex-A55, with an ELF check rejecting PT_INTERP and
DT_NEEDED. These binaries do not require Bionic or an Android dynamic loader.
The native AAudio/libgui fallback helpers still use the Android build tool.

A fully static audio read-only probe ran successfully on the G06 without
LD_LIBRARY_PATH. Evidence: `assets/build/backend-audio-static-inspect.txt`;
binary SHA256 `8e83fe6045a9b5aaaa657be82daa492088c88a7eeb453562e8af6917773732aa`.
It completed all queries; backlight actual_brightness remained 0, bl_power 4,
and app-volume 0. This proves executable/readonly ioctl compatibility, not
direct playback or scanout. Direct write/display tests remain pending.

The integrated PCM transport now has 17 host mock tests (tools/audio/check-mi-pcm.sh).
They exercise ownership, short writes, deadlines, vendor errors, mutated ioctl
arguments and cleanup failures. The stored handle is never reused as mutable
ioctl wire storage. No direct PCM Start/Write has been run on hardware yet.

The direct adapter is also a dependency-free `g06-audio` library in the main
Cargo workspace, so the eventual PID1 owner can link it directly without
launching a separate helper. Host library tests and ARMv7-musl release builds
pass. The standalone executables remain bounded hardware-test scaffolding.

## Shared protocol and direct backend integration

The following files are now integrated in main.
The `g06-audio` library exposes these modules for direct PID1 linkage.

- `src/audio/protocol.rs`: shared O/W/D/R/F parser, validation, gain application,
  counter/reset/drain replies and listener scaffolding. PID1 can directly own
  `client()`; it need not start a permanent extra privileged process.
- `src/audio/bridge.rs`: existing AAudio adapter now uses that shared engine.
  Its native dependency stack and defaults remain the same. Refactored binary
  has not been tested live; retain the currently proven binary for rollback.
- `src/audio/direct-backend.rs`: one-owner MI adapter using the tested PCM
  module. A second owner fails before opening hardware. Ownership is released
  after PCM cleanup. No AOUT routing, hardware initialization or gain change.
- `src/audio/pcm-converter.rs`: causal linear 44.1-to-48k conversion and mono
  duplication. Integer phase is retained across arbitrary input blocks.
  48k stereo is bit-exact. Total output is floor(N*48000/input_rate).
  Playback counters use the inverse count map and remain in app input frames.
  Conversion quality is intentionally basic; there is no new codec dependency.
- `src/audio/direct-bridge.rs`: explicitly opted-in temporary static test
  listener, default127.0.0.1:8879. Requires `--exclusive-warm-pcm` and original
  G06 serial. Starting the listener alone does not open PCM; a valid app O does.
- `test/unit/audio/protocol-tests.rs`, `direct-backend-tests.rs`: memory-only tests.
- `tools/audio/check-direct.sh`, `build-direct.sh`: host tests and ARM build.

`tools/audio/check-direct.sh` passes 35 host tests: 17 PCM lifecycle/ABI, eight
shared protocol, six conversion, four direct lifecycle tests. These cover
one-byte fragmented protocol transport, gain/mute, reset/drop, typed errors,
short hardware writes without lost samples, exact long-run conversion counts,
byte-identical conversion across chunk boundaries, and owner exclusion.
An F counter error now returns -1 before closing, rather than silently dropping
the connection. Unsupported formats return -22 before any hardware open.
Direct supports precisely 44.1k/48k, mono/stereo; rates outside those are rejected.

Built ARM artifacts:

- `assets/build/audio-agent/rust-audio-shared-engine`
  SHA256 `aaecebc4631857a5f95cd587bb9dfbbd36e4d0a5e70d95a59cdfc232b7aebc5c`.
- `assets/build/audio-agent/direct-audio-bridge-v1`
  SHA256 `01bfc9f78e86e35d0bad94827c7163c331544e1af27fcd603aa47b265b031d35`.
  Static ARM ELF, no interpreter or dynamic-library dependencies.

Neither refactored binary has been run on the TV. Direct PCM enqueue/drain,
correct physical audio routing, and cold initialization remain hardware gates.
The direct backend intentionally relies on already initialized kernel hardware
and an independently established AOUT route; it does not yet replace the full
boot-time audio initialization stack. Do not deploy/start while the user sleeps.

## Direct PCM enqueue/drain verified after recovery reboot

On restored G06 Android, the static Rust PCM trial opened handle0x81000000,
reported a19200-byte buffer, wrote ten1920-byte zero blocks and observed the
queue return to zero. Stop/Close completed. Evidence:
`assets/build/backend-stock-silent-pcm.txt`. This confirms direct kernel PCM
operation without AudioFlinger for the probe's stream.

The direct speaker-route jingle also completed48000 frames and restored the
previously empty route, at amplitudes64 and512. The owner heard neither.
Evidence: `backend-stock-direct-jingle.txt` and
`backend-stock-direct-jingle-512.txt` under `assets/build`. Physical direct
audio output remains unproven; do not increase gain blindly.

## Sleeping-session diagnosis

Read-only status after Ethernet returned found the existing speaker unmuted,
output enabled and amplifier unmuted, but at volume index5 / -47dB attenuation.
The amplifier control register was0x10 (unmuted), and pad30 was physically high.
Evidence: `assets/build/backend-sleep-audio-state.txt`. A512/32768-scale jingle
combined with that attenuation is approximately -83dBFS before analog gain;
this plausibly explains inaudibility, but does not establish audible routing.
No gain, amplifier register or power state was changed by these probes.

Future audible comparison must capture hardware attenuation during the
previously comfortable AAudio test and use a bounded save/restore operation;
do not blindly raise volume based on software queue success.

## Routed backend integration

The direct backend now supports explicit `--route-speaker` through an owned
route guard. It restores AOUT before stopping/closing PCM, including reset and
error paths. If restoration fails, PCM remains allocated and the backend is
poisoned rather than freeing a possibly referenced object or opening another
stream. The error includes the retained handle and original route snapshot.
No routing, hardware gain, or initialization change occurs by default.

64 host tests cover shared protocol, rate conversion, driver I/O, routing and
cleanup, including route-restoration failure with zero PCM Stop/Close calls.
The static ARM build passes. This routed server has not yet been deployed;
the earlier one-second hardware test used the separate bounded trial.
