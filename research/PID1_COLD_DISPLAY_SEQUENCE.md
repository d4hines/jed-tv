# Cold display sequence for the current PID-1 backend

The working backend now lives in `src/display/{service,osd_scanout,mma,osd_abi}.rs`
and is owned by `src/pid1/src/direct_services.rs`. It bypasses fbdev, so removing
midaemon does not require publishing SYS_USER selector1. What it still inherits
from vendor startup is initialized SYS/display/output/OSD state, an opened
display controller, boot-logo handoff, and panel power. This note turns those
into an explicit candidate sequence. It has not been run on a cold TV.

## Existing mount/layout integration

For the first controlled trial, reuse the current read-only mount setup rather
than also changing how partitions are discovered:

1. Existing PID-1 diagnostics/network/root console first; claim and retain
   `ColdBootProvenance` before any user MI operation.
2. Retain the current eMMC read-only protection and named partition aliases.
3. `mount-stock-runtime.sh` mounts stock system/vendor from p28 using read-only
   loop devices. The relevant path for this work is `/vendor/lib/modules`.
   The first trial may retain the other mounts; no Android process must be
   launched merely because its files are mounted.
4. Mount p7 at `/vendor/tvconfig` and p27 at `/tclconfig`, both `ro,noload`.
   Keep `/config → /vendor/tvconfig/config` and
   `/vendor/tclconfig → /tclconfig`. Named `/dev/block` aliases remain useful
   to the stock flash/config code. p19 `/mnt/vendor/tvservice`, `/lib` glibc,
   Binder/property snapshots and Android service managers are not needed by
   the new static Rust calls themselves.
5. Copy the stock `mik_init.ini` into RAM, make only the reviewed explicit-init
   change `MI_TUNER_INIT=0`, verify it with `validate_explicit_ini`, and bind
   that RAM file over `/vendor/tvconfig/config/mik_init.ini` before loading
   mik. Never edit p7. All other INITs are already0 except SYS/FLASH1.
   Alternatively stage the prepared complete RAM pack; do not mix profiles.

Keep all existing panel/PQ/OSD/onebin dependencies available. The config loader
uses MODEL_ID, not userspace MemTotal, to select the memory map. The existing
331-file candidate preserves those dependencies but is not a minimality proof.

## Candidate operation order

Load the twelve pre-display modules from `cold_start::PRE_DISPLAY_MODULES`,
with per-module failure logging. There is no `mstar_fbdev_mi` in this direct
OSD path. Initially retain the three stock runtime-registration extras. Module
insertion parameters need an explicit slot: the display agent found that
hdcp2x module defaults can override copied INI key-loading flags. Its proposed
`DISABLE_LOAD_RX_KEY=1 DISABLE_LOAD_TX_KEY=1` parameters are under separate
exact-path review and are not yet built into this loader.

After `mik` succeeds, keep its module/config logs and require SYS/MIKO
selectors0/40 ready with verified kernel FLASH-init configuration. Then use the owned adapter:

```rust,ignore
let mut mi = mi_bootstrap::Device::open()?;
mi.permit_initialization(cold_provenance)?;
let results = mi_bootstrap::initialize(
    &mut mi, mi_bootstrap::cold_start::DISPLAY_SEQUENCE)?;
record_exact_results(&results);
let handoff = mi.finish_display_bootstrap()?;
// Retain mi, handoff and all later scanout resources in PID 1.
```

`DISPLAY_SEQUENCE` is DISP → DISPOUT → OSD. The new handoff method requires
all three to have completed on the same Device under cold ownership; a ready
bit that caused INIT to be skipped does not create that completion evidence.
This intentionally targets the explicit-init profile. It also requires
selector1 to remainfalse. It does not write any status bit.

The additional handoff operations are:

| Operation | Exact wire / behavior |
| --- | --- |
| Query absent controller | DISP0xc0081206, inner8 zero bytes; expect vendor3 and handle0 |
| Open controller | DISP0xc0081204, inner8 `{handle_out:u32=0,reserved:u8=0,pad3}`; expect vendor0 and handle0x36004354 |
| Read back controller | DISP0xc0081206 again; expect the exact opened handle |
| Disable OSD boot logo | Raw OSD ioctl0x144e, null argument; no MI envelope |
| Disable display boot logo | DISP0xc0041203, inner4 containing the returned controller |

The handoff is one attempt per Device, including failures. It never closes the
controller, resets OSD, publishes SYS_USER, or retries after partial mutation.
No CLI exposes this operation. PID 1 must keep its Device and controller
ownership until reboot.

### Why opening the controller is a real missing step

The exact `MI_DISP_GetController` checks `_bDispInited` at0xfb1ac, then checks
`_hDispController` at0xfb290. With no controller it returns vendor3 at0xfb55c.
It does not allocate one. `MI_DISP_OpenController` sets that flag at0xf9f30–34
and returns0x36004354 at0xf9f70–78. The only other writes found are its close
path. DISP initialization by itself does not provide the open controller
that the warm `tv-bootlogo` helper inherited from midaemon.

The 8-byte open wire is confirmed both by DWARF
`MI_DEV_DISP_OpenControllerParams_s` and the exact libmi wrapper at0x5b1f4
(request construction0x5b23c–54). The caller supplies a reserved zero byte;
there is no guessed timing, controller index or embedded pointer.

## Timing, logo and panel conditions

Keep the exact stock INIT parameters for the first trial: DISP `{6,6,0}` and
OSD twelve zero bytes. Do not substitute width/height for those enum fields.
The saved working native output readback is timing12 (1920x1080 60p), while
the INI input timing6 means1920x1080 50i. That difference must be observed
after cold initialization; the snapshot does not prove which stock/MBOOT
step changed it. Do not blindly add a timing setter to match the snapshot.

After initialization and controller open, the existing read-only timing query
is DISP0xc008122d with `{controller,timing_out}`. Preserve its result in the
test log. The working scanout uses1280x720 RGBA input, pitch5120, and a1920x1080
display rectangle. Its external surfaces are ARGB8888 enum8 in PHY_OS mode2,
with two pages in one7,372,800-byte MMA allocation and full64-bit IOVA. Leave
that already tested geometry/format path unchanged for this comparison.

Boot-logo handoff must precede creating the new layer. `_MI_OSD_SetGopMux`
skips programming the normal layer mux while the boot logo is active. The
existing proven helper order is OSD DisableBootLogo then DISP DisableBootLogo.
Both calls are real hardware/state operations, unlike setting SYS_USER.
There is no need to hide a SurfaceFlinger layer0 in a genuinely cold path
where SurfaceFlinger never ran. Query existing layer ownership first; do not
destroy any unexpected layer to force allocation to succeed.

For the first trial, preserve the working MBOOT panel/backlight state. The
saved `bl_power=4` experiment cuts panel VCC and did not recover simply by
restoring brightness/bl_power; it needed Android reboot. It is not a suitable
blanking primitive for a fast graphics loop. Full panel power-up after rail
loss is a separate unresolved sequence, not implemented by this adapter.

## Start the current backend, then add audio

Only after successful handoff start the current `g06_display::Service`; its
`Scanout::create()` requires layer1 absent, allocates once, creates its own
layer/window/surfaces, and brackets flips with BeginConfig/ApplyConfig. Keep
all objects and backing descriptors for the entire boot. A first setup error
must leave the console reachable and stop that attempt; never reset/retry
OSD or release published MMA memory into stale windows.

Do not use `direct_services::start` unchanged for the earliest display-only
cold test: it also starts a direct-audio listener/backend that currently
expects audio setup. First test just the display service, then initialize
the owned AUDIO/AOUT/PCM path and GPIO/IIC/amplifier when that path is ready.
The adapter does not forbid later distinct subsystem INITs while cold
ownership remains valid, but never repeat already attempted modules.

## Remaining explicit blockers

- Cold execution itself is untested, including effects of removing midaemon's
  user SYS helper initialization and callbacks.
- HDCP/TEE avoidance needs the display agent's reviewed module parameters;
  copying an INI alone is insufficient if module-init overwrites it.
- Cold output timing versus the known working timing12 needs readback.
- Panel re-power after VCC loss is not implemented. Preserve MBOOT state.
- Direct scanout and controller ownership must remain singular and persistent.

Evidence: `assets/research/midaemon-replacement/cold-display-handoff.txt`,
exact libmi DWARF, `assets/build/backend-live-output-inspect.txt`, and the
existing display/backend sources cited above. The new adapter has21 total
crate tests passing, including controller absence, exact readback, stop-on-error,
logo order and one-attempt authorization tests. Static ARM build passes.
No hardware, boot, or main-worktree changes were performed for this work.

FLASH selector3 is not a readiness gate: kernel SYS initialization erases it.
See research/FLASH_STATUS_RESET.md. The reusable Rust composition now lives in
src/mi-bootstrap/src/owned.rs; it is not yet invoked by the running PID1.

## Current integration

The canonical REPL build now wires these stages through
`src/pid1/src/hardware.rs`: Rust configuration/module staging, stock TEE
provider under the owned tracer, direct display init/controller handoff,
direct audio init and muted amplifier programming. No Binder manager,
midaemon, SurfaceFlinger, AudioFlinger or Java process is launched.
The TEE provider is monitored, and failed audio startup retains the display
owner. In-place PID1 replacement is refused after hardware ownership begins.

This is an offline-built candidate, not a cold-TV success claim. The next
required test is a clean one-shot boot on the original G06, inspect /status
and /logs, then capture /tmp/tee-rpc.jsonl and confirm the physical display.
Do not run it during the user's no-light/no-sound period.

A separate Rust replacement for the remaining stock TEE provider is built by
`tools/tee-provider/build.sh`. It has not run on the TV. Its documented missing
RPC classes and external-memory behavior must be resolved from the captured
cold trace before replacing the stock provider in this boot path.
