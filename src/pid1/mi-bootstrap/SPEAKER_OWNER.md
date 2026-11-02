# Ordered native output ownership

The current cold owner creates ALL (path0), then MAIN_SPEAKER (path4), matching
stock HAL initialization. Path0 runs global DSP speaker configuration that a
path4-only owner misses; exact firmware evidence is in
`research/AUDIO_ALL_OUTPUT_COLD_CANDIDATE.md`. The parent coordinator's live
late-Open trial succeeded, but physical audibility and the new cold ordering
still require separate hardware confirmation.

`Output` is the single-path resource. `Outputs` owns the pair: close speaker
first, then ALL; if a route remains or Close is ambiguous, retain both owners.
A failed speaker Open also retains global setup. Neither object initializes
modules, opens PCM, attaches routes or chooses a playback gain.

The library's 53 host checks pass, including creation/close order, live-route
retention, ambiguous Close and failed second Open. The implementation replaces
the earlier sole-speaker cold owner; the remaining text records its original
wire and hardware evidence.

# Persistent native speaker output

The live cold boot initialized AUDIO/AOUT/PCM and the AD82111, but the app
backend's GetHandle(path4) returned7: there was no speaker output object.
This module supplies the missing lifetime. No AOUT Start/Stop operation exists
for ordinary output; Open creates the output and its pipeline object.

## Files and integration

New source: `src/speaker.rs` and `src/bin/speaker-owner-probe.rs`.
Add `pub mod speaker;` to the crate root. `src/owned.rs` now creates and retains
`Speaker<NativeIo>` inside AudioHardware after muted amplifier initialization.
The field is declared first, so its cleanup happens before the amplifier and
Device references disappear. `speaker_handle()` exposes its identity for logs.
The existing PID1 hardware loop already retains AudioHardware for the boot;
no PID1 edit is needed to extend that lifetime.

NativeIo construction requires the existing Device cold permission, successful
native AUDIO/AOUT/PCM initialization recorded by Device, and SYS_USER still
unpublished. It does not infer authority from missing handles or call any INIT.
The diagnostic's root/serial checks are narrower safeguards: its quiescent
current-boot authority comes from the coordinator, not a manufactured cold token.

## Exact operation

GetHandle uses0xc0081116,8bytes `{path4:u32,handle:u32}`. Only status7 permits
Open. Existing success is refused, not adopted. All other statuses stop before
Open; in particular no reinitialization is attempted.

Open uses0xc0581103 and88bytes:

- result handle at0;
- empty64-byte name at4 (stock HAL passes a null/empty name);
- path4 at68;
- custom-volume-table flag0 at72;
- zero padding and null u64 table pointer at80.

Successful Open must return a0x17-prefixed handle, then another GetHandle must
return exactly the same value. The driver copies its default volume table when
the custom flag is false. This code does not set gain, connect PCM, unmute the
external amplifier or emit a signal. The cold AudioHardware retains its existing
muted-amplifier sequencing, with its later explicit unmute unchanged.

Close requires both connection directions to have zero edges, using
0xc010111e,16bytes `{handle, input_direction, module0, count}`. Then it sets
USER mute via0xc00c110d with12bytes `{handle,type0,mute1,padding}` and closes via
**0x40041104**,4-byte handle. The Close direction differs from other AOUT ioctls
and was verified directly in libmi's wrapper. Only our successful Open can reach
this cleanup. A final Close with an uncertain/error result is never retried.

If routes remain or a query/mute/close fails during Drop, the module retains
(leaks) the owned descriptor/output rather than risk closing live references.
Normal lifetime is one entire boot. Explicit `close()` can report failure while
the owner stays alive; a successful close is idempotent.

## Current-boot diagnostic, coordinator only

Built V2 static ARM binary:

```
assets/build/audio-agent/speaker-owner-v2/armv7-unknown-linux-musleabihf/release/speaker-owner-probe
```

After the coordinator checks original G06 identity, initialized native modules,
muted amplifier and no active PCM clients, upload and run as root:

```
setsid /tmp/speaker-owner-probe </dev/null > /tmp/speaker-owner-probe.log 2>&1 &
```

It prints READY with the new handle and remains alive. Keep it alive while
DirectBackend uses that output. SIGTERM/SIGINT asks it to close; when routes or
errors prevent cleanup, it logs the reason and stays alive. Stop clients and
send SIGTERM again after detachment. Do not force-kill it while audio routes
reference the output. It does not flash, reboot, touch display, run a tone or
change volume. No hardware command was executed by this agent.

Build command used:

```
CARGO_TARGET_DIR="$PWD/assets/build/audio-agent/speaker-owner-v2" RUSTFLAGS='-C linker=rust-lld -C target-cpu=cortex-a55' cargo +1.92.0 build --offline --release --manifest-path research/probes/mi-bootstrap/Cargo.toml --target armv7-unknown-linux-musleabihf --bin speaker-owner-probe
```

48 host crate tests pass, including10 new ownership/wire tests covering existing
output refusal, exact Open bytes, permission/status failures, owned readback
failure cleanup, live-edge retention, query errors, Close direction and no retry
of ambiguous Close. The helper's actual hardware behavior remains for the
coordinator to qualify.

## Evidence

Exact V109 library/kernel evidence under
`assets/research/audio-agent-direct/`:

- `aout-open-wrapper.txt`: libmi MI_AOUT_Open at0x4938c, request at0x49458.
- `aout-open-kernel.txt`: mik MI_AOUT_Open at0x798e4.
- `aout-gethandle-effects.txt`: lookup without allocation.
- Stock HAL `audio.primary.mt5867.so.thumb.txt`: mi_device_open at0x36794,
  accepts lookup7 at0x3680c, prepares empty/default Open and calls it at0x36886.
- libmi MI_AOUT_Close at0x49768 constructs0x40041104 at0x4979c..0x497a0.
- DWARF `MI_DEV_AOUT_OpenParams_s` gives offsets0/4/68/72/80 and size88;
  `MI_AOUT_MuteParams_s` gives mute type0 and bool offset4 within its8bytes.

## First live result

On the original G06 cold PID1 session, V2 created speaker0x17000000 and held
its process alive. The existing PID1 PCM backend then successfully opened
and drained48000 frames for the one-second jingle, where its previous
GetHandle had failed with status7. Physical audibility confirmation remains
pending. Evidence: assets/research/tee-live/20261101T185641Z/.
The integrated next-boot owner has48 passing host tests and passes the native
PID1 QEMU shell/reload check; cold hardware integration is still untested.
