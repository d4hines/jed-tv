## Integrated PID1 verification

The canonical REPL image now owns direct display and audio inside PID1. A
live G06 test on 2026-11-01 verified the root shell and launcher, then Doom
and a 120-frame local video run (zero drops, zero audio underruns). Playback
was muted; this is software/device-handle evidence, not a new physical screen
or sound confirmation. PID1 held /dev/fb1, DMA-BUF, /dev/mik/pcm and aout.
SurfaceFlinger, composer, audioserver, audio HAL and midaemon were stopped.
Startup still uses those vendor programs to initialize the hardware first.
Evidence: assets/build/pid1-repl/integrated-live/.

An initial cold-shell attempt found loopback down; PID1 now configures lo
before starting local services. QEMU checks the loopback address and flags.
The corrected image was reloaded in RAM and its audio listener became ready.

## Confirmed direct playback (2026-11-01)

The owner confirmed moving Doom with working controls and audible gunshots,
then moving local test video with audio. The video run submitted 360 frames
with zero dropped frames and zero audio underruns. This was a warm Android
handoff to standalone direct Rust drivers, not a cold PID1 initialization test.
Evidence: assets/build/direct-doom-confirmed-state.txt and
assets/build/direct-video-test.txt.

Immediate framebuffer mode 1 is required for motion; queued mode stalled on
this firmware. The driver now sets immediate mode when allocating scanout.
Keep its allocation alive across app changes because vendor destroy is broken.

## Live failure and recovery (2026-11-01)

The first direct trial allocated two720p pages, exported/mapped a full64-bit
IOVA, and returned success from presentation. The owner saw only black.
Kernel logs show WindowDestroy is an unsupported stub and layer destruction
failed; fbdev free nevertheless released backing memory, leaving an orphan
window/layer. The earlier statement that cleanup was proven is withdrawn.
Current Drop refuses to free backing memory unless queries prove no live
window remains. Keep the allocation owned/persistent during future trials.

Showing layer0 and restarting the presenter did not restore the image. A
restart of HWC/SF then failed to create/adopt the preexisting layer and used
handle0. Root requested a normal reboot to installed Android to reset this
RAM-only driver state. No firmware writes or global MI reset were used.
Evidence: `assets/build/backend-live-direct-display.txt`,
`backend-live-window-inspect.txt`, `backend-display-restart-logcat.txt`.
The owner confirmed normal Android after reboot; the later persistent trial also restored Android without reboot.

# Native display backend work

The current implementation is the static Rust `g06-display` library, using
MI OSD/framebuffer and MMA directly. PID1 owns its hardware resources;
`direct_stream.rs` is temporary standalone test scaffolding for the same code.
The app-facing TVF2 frame protocol and PID1 volume overlay are unchanged.
Moving Doom and video are physically confirmed with immediate flip mode1.

The former Bionic/libgui SurfaceFlinger presenter and its build wrapper have
been removed now that direct moving video and Doom are physically confirmed.
Its original probe evidence remains under assets/build/display-agent/,
including live-hidden-probe-v3.txt and live-visible-start.txt. Exact firmware
ABI research remains under assets/research/rust-display/ and
assets/research/rust-display-agent/ as historical evidence, not an active
runtime dependency or fallback.

The shared `frames`, `pixels`, `overlay`, and `service` modules remain part of
the current path. Read-only MI inspectors and the backlight diagnostic also
remain; removing a superseded presenter does not remove recovery evidence.

## Historical direct MI preparation (before physical qualification)

The read-only `mi-inspect` probe ran through the root coordinator. Evidence is
`assets/build/backend-direct-inspect.txt`. It never opens either framebuffer.
The live system reported MI SYS/DISP/OSD readiness, only layer0 (handle0x20000)
active, and fb1 `MStar FB0`, pitch5120, bpp32, virtual geometry1280x0 with no
fbdev descriptor owners. This is consistent with an unallocated optional MI
framebuffer alongside the Android compositor plane. It is not evidence of a
usable mapping or successful scanout.

Controller0x36004354 is valid: the exact kernel's MI_DISP_GetController at
0xfb2a4 constructs that literal. The probe's all-bits mute reason was rejected
with vendor status7; it must not be read as a muted display.

Driver review found that fb1 open/release are not read-only: first open calls
OSD setup, and last release at0xe58 invokes the cleanup at0x396c, which destroys
its saved layer/window/surface state. This is why the inventory reads sysfs
and MI getters rather than opening fbdev. Firmware config assigns FBDEV0 to
layer1, which was absent in the live inventory. No global OSD reset is needed
for the planned independent-layer trial.

`src/display/direct_mi.rs` is the candidate embeddable Rust hardware module.
It uses std plus raw kernel ioctls, with no libgui, SurfaceFlinger, vendor shared
libraries, or allocator-service calls. The separate `direct_trial.rs` executable
is bounded test scaffolding, not a permanent privileged service. Build using
`tools/display/build-direct.sh`; output `assets/build/display-agent/direct-display-trial`.
No changes to the existing native display binary were made in this phase.

The explicit `--show-color-bars-5s` mode checks the G06 serial, fb1 identity,
and that layer1 is absent; opens fb1; allocates two720p pages; creates the OSD
layer; checks the resulting geometry and bitfields; reads the64-bit IOVA;
exports it to a DMA-BUF through MMA; selects uncached mapping; writes bars;
pans and shows the layer; waits five seconds; hides/releases it. It intentionally
does not initialize/deinitialize SYS/DISP/OSD or manipulate layer0.

The mapping follows the earlier authored PID1 path:

- fbdev0x80104672 returns IOVA low/high words and allocation length.
- MMA0xc0284d12 returns a global allocation name and its full64-bit base.
- MMA0xc0284d0a imports that name into a DMA-BUF fd.
- MMA0x40284d06 selects uncached mapping for that fd.
- mmap uses offset0 on the exported fd, never truncated IOVA as an fbdev offset.

The root coordinator should pause the native display reader, SurfaceFlinger,
and composer during the first experiment, retain their PIDs, run the bounded
trial, then resume the exact processes unconditionally. Keep midaemon and audio
alive. Separate layer creation improves reversibility, but shared vendor kernel
state means this remains a hardware experiment. A successful ioctl sequence
cannot establish the bars appeared. The owner must confirm the physical panel.
If the old plane covers the new plane, first inventory layer ordering; do not
respond by resetting global OSD state.

After visible bars work, connect `Scanout::draw_rgba` to the existing TVF2
consumer and volume composition. Preserve byte-range lease ownership; the
hardware module must receive only a completed composed frame. Its bitfield
packing is required because the API's RGBA word layout need not equal MI's
ARGB8888 memory layout. Then test steady-state service independence using
coordinator-controlled pauses, and separately test cold bootstrap with the
midaemon replacement module. Warm success alone does not prove cold startup.

Cleanup audit: fd close alone does not release the IOMMU allocation. The
candidate now uses explicit fbdev0x4697 cleanup after hiding, unmapping and
closing its exported DMA-BUF; the ioctl destroys its windows/surfaces and calls
MI_OS_FreeIommuMemory. Then fd close destroys its saved layer. Allocation cleanup
also runs on ordinary Rust error returns after allocation. INT/TERM requests
an orderly exit. SIGKILL/abort still bypasses userspace cleanup; do not repeatedly
retry after an abnormal exit without checking allocation state.

Current direct-trial SHA256:
`66f117b255598f92b9c9362d5da97779938b909c8671fba45b0a7acecfc96e35`.
Older trial hashes were superseded before hardware execution. The parent said
the user is asleep and disabled backlight; all visible testing is deferred.

The follow-up `direct_stream.rs` scaffolding uses the identical TVF2 reader and
now-shared `pixels.rs` / `overlay.rs` logic. Build using
`tools/display/build-direct-stream.sh`. It requires explicit
`--run FRAMEFILE SECONDS` (1..120), preserving header/page record leases and
volume-overlay expiration while routing completed RGBA frames to Scanout.
SHA256: `f753fc1c9c052b459b6dc25ff0db6bdf8f5e04fe25e1c11eb7d51b623c0ba55d`.
Do not use it before basic bars and cleanup work on hardware.

The direct module also compiles as an ARMv7-musl rlib for eventual PID1
integration. mmap's offset ABI is selected explicitly: Android ARM32 off_t is
32-bit; musl ARM32 off_t is64-bit. Using the same raw signature for both is
incorrect despite passing offset0. Software exact-copy and aspect-fit tests
pass after the shared-composition extraction. The frozen live native binary
has not changed: SHA256 `0c45cc7b5ee207765f44bdefc774d35f4b7a95006afb2a852a2adb752f688c28`.

## Static direct-driver builds (coordinator integration)

Direct driver probes now use `tools/toolchain/build-static-rust.sh`: pinned
Rust 1.92, ARMv7 musl, Cortex-A55, with an ELF check rejecting PT_INTERP and
DT_NEEDED. These binaries do not require Bionic or an Android dynamic loader.
The superseded Bionic presenters are no longer part of the direct display runtime.

A fully static audio read-only probe ran successfully on the G06 without
LD_LIBRARY_PATH. Evidence: `assets/build/backend-audio-static-inspect.txt`;
binary SHA256 `8e83fe6045a9b5aaaa657be82daa492088c88a7eeb453562e8af6917773732aa`.
It completed all queries; backlight actual_brightness remained 0, bl_power 4,
and app-volume 0. This proves executable/readonly ioctl compatibility, not
direct playback or scanout. Direct write/display tests remain pending.

Post-integration audit: 5 direct allocation/layout/cleanup tests and 3 pure
composition tests pass. The mapper checks the complete DMA-BUF size and rejects
overlapping channels, invalid descriptor output and IOVA overflow. Current
static build outputs supersede the earlier Android-linked hashes above.

The direct adapter is also a dependency-free `g06-display` library in the main
Cargo workspace, so the eventual PID1 owner can link it directly without
launching a separate helper. Host library tests and ARMv7-musl release builds
pass. The standalone executables remain bounded hardware-test scaffolding.

Static shared-frame readback now works on the physical G06 without Android
libraries. `tools/display/build-frame-probe.sh` builds a read-only TVF2 probe;
the live run read 1280x720, sequence 8299, all 921600 pixels, in 27580 us including
checksum computation. This is not a presentation latency measurement. Evidence:
`assets/build/backend-static-frame-probe.txt`. The backlight remained off and
software gain zero. No framebuffer/OSD devices are opened by this probe.

The contemporaneous read-only process/library inventory is
`assets/build/runtime-process-inventory.txt`: 18 userspace processes including
the temporary inspection shell. Remaining vendor/system processes include
midaemon, SurfaceFlinger, audioserver, graphics allocator/composer, audio HAL,
binder managers, logd, tee-supplicant, mediametrics and suspend service.

## Persistent direct scanout physically verified

After a normal reboot, the persistent static Rust renderer ran as root under
Android. Android's SurfaceFlinger/composer were paused for the test; layer0
was hidden, and the renderer created and retained layer1 plus two720p buffers.
The owner confirmed colored bars. SIGUSR1 hid the retained layer; layer0 was
restored, original services resumed, and the owner confirmed Android was back.
Evidence: `assets/build/backend-stock-persistent-display-v3.txt`. The renderer
remains alive at the PID recorded in `/data/local/tmp/direct-display.pid`.
The latest boot ID is `89a3856d-3d7a-4717-be32-7697de496198`.
This proves warm direct scanout and reversible switching with persistent
resources, not cold initialization or destruction. Android names MI nodes
`/dev/mik!osd`, so the direct driver and control helper now accept both forms.

The `g06-display` library now exports the shared-frame reader, software
composition and volume overlay alongside `Scanout`, so a PID1 owner can link
the complete display path without a separate renderer process. Nine host
tests and the ARMv7-musl library build pass. This packaging change does not
itself establish cold-boot display initialization.

## Shared platform consolidation

PID1 now depends on src/platform, sharing the app framebuffer implementation.
Exported DMA-BUF mapping/lifetime and pre-mmap geometry checks were retained
from the old backend without removing TVF2 publication leases. Reproduce the
three focused host tests with `python3 tools/pid1/check-framebuffer.py`.
The default static ARM PID1 build passes. Agent validation additionally
covered MI+REPL builds and QEMU Hello/HTTP/two replacement handoffs/reboot;
evidence: `assets/build/display-agent/platform-consolidation-validation.json`.

The Nix app vendor hash was refreshed after the g06 driver crates entered
Cargo.lock. The resulting app-protocol QEMU run passed menu, video/audio,
nested YouTube Back, Home, Slides and Doom. Evidence is under
`assets/build/integration-qemu-1793523670887130000`. That run's Nix source
preceded the platform consolidation; it validates the dependency-pin repair
and app protocol baseline, not a cold direct-driver boot.

## Moving video and Doom physically confirmed

The owner confirmed animated Doom and moving video through the direct Rust
renderer. Immediate framebuffer flip mode1 fixed the previous queued-flip
animation failure and is now the implementation default. See
research/DISPLAY_PAN_AUDIT.md for the recovered mode ioctl and the distinction
between accepted queued frames and visible scanout. The SurfaceFlinger
presenter was removed after this confirmation; no runtime selection flag was
added. Cold hardware initialization remains a separate qualification gate.
