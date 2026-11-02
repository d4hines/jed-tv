# Rust staging for the owned cold bootstrap

`src/mi-bootstrap/src/staging.rs` prepares the fixed G06 filesystem layout,
stages the explicit-init configuration in RAM, and loads kernel modules.
It does not initialize MI subsystems, publish status, open fbdev, start vendor
processes, or integrate itself into the current PID-1 entry point.

## API and lifecycle

The short composition is:

```rust,ignore
// SAFETY: fresh native PID1; no userspace MI client or initializer has run;
// this caller serializes all mount/module work until preparation completes.
let staged = unsafe {
    mi_bootstrap::staging::prepare_cold_platform(|event| {
        state.event(format!("{}: {}", event.action, event.detail));
    })?
};
// The same owner still controls the entire fresh-boot history.
let cold = unsafe { mi_bootstrap::ColdBootProvenance::claim_exclusive_boot_phase()? };
// Start the already-reviewed TEE provider here before dependent MI init, if needed.
let hardware = mi_bootstrap::owned::DisplayHardware::initialize(
    mi_bootstrap::Device::open()?, cold)?;
// Retain hardware and all subsequent scanout resources until reboot.
```

For checkpoints use `unsafe ColdPlatform::begin()`, then `mount_stock`,
`stage_configuration`, `load_modules`, and `finish`. Each stage accepts an
observer; `events()` retains completed observations even after failure.
The phase machine refuses out-of-order calls and retries. There is also one
process-wide staging attempt. Failure leaves already-created RAM mounts and
loaded modules in place for inspection; nothing is torn down automatically.

The unsafe contract is historical: no user MI initializer/client since boot,
and no concurrent mount/module work. PID, serial, kernel version, module list
and read-only status checks constrain it but cannot prove that history.
No CLI exposes staging or module loading.

## Exact stages

Before work, require root PID1, original G06 serial, kernel4.19.116+, a native
RAM root, and every visible `mmcblk0*` sysfs read-only flag already1. The helper
does not change block-device protection. The only accepted existing preload modules are `iniparser`, `kdrv_xc`, and
`utpa2k`, already used by the current REPL watchdog. The caller attests that
these came from that stock startup and keeps the watchdog running. Every
other existing MI preload member, including the framebuffer, is rejected.

The fixed partition checks come from the original backup manifest:

| Node | Name | Start sector | Sectors |
| --- | --- | ---: | ---: |
| mmcblk0p7 | tvconfig | 51200 | 65536 |
| mmcblk0p27 | tclconfig | 1708032 | 245760 |
| mmcblk0p28 | super | 1953792 | 7065600 |

The helper opens p28 read-only and attaches its vendor slice at byte734003200
to `/dev/loop1`. An existing loop1 is adopted only if device/inode/rdev,
offset, size limit, no encryption, and read-only flag all match. A different
mapping is left untouched. Every new loop mapping is read back before use.

It mounts loop1 at `/stock-vendor`, read-only with `noload`, then binds
`/stock-vendor/lib` at `/vendor/lib` and remounts the bind read-only. It mounts
p7 at `/vendor/tvconfig` and p27 at `/tclconfig`, likewise `ro,noload`.
Existing mount records must match the source device number, filesystem root,
ext4 type and effective read-only option; unexpected nested/stacked mounts
are rejected. Mount targets cannot be symlinks. It preserves or creates the
expected `/config`, `/vendor/tclconfig` and named eMMC aliases without replacing
an alias to another object.

Configuration staging reads the stock INI, changes exactly the tuner flag
from1 to0, and checks all35 INIT flags plus DISP/OSD arguments. It refuses an
already-edited source or duplicate/unknown enabled initializer. It writes
`/tmp/mi-bootstrap-stage/mik_init.ini` under a new RAM directory, verifies its bytes,
binds it over the stock INI, remounts that bind read-only and verifies the
visible bytes again. The stock partition file remains untouched.

Module loading uses `PRE_DISPLAY_MODULES` in its exact order. Each file must
be bounded, regular, ELF32 little-endian ARM relocatable, with the stock kernel
vermagic. This is an architecture/version gate, not a cryptographic firmware
authenticator. The source filesystem is the validated original vendor slice;
prior backup/module hash evidence remains the identity reference. The kernel
still performs its own module/modversion validation.

The three inherited watchdog modules get an adoption event and are not
reinserted or unloaded; every new module gets a before/after event. Any new insertion error, including EEXIST,
stops the stage; successful insertion must be reflected in `/proc/modules`.
Parameters are the empty string, preserving stock defaults including HDCP.
There is no framebuffer module in this list.

## Linux ABI checks

The loop structure follows Linux4.19 `loop_info64`: five initial u64 fields,
four u32 fields, byte arrays64/64/32, and two final u64 fields. Compile-time
assertions cover size232 and offset/flags/name offsets24/52/56 on ARM as well
as the host. Requests are LOOP_SET_FD0x4c00, SET_STATUS64 0x4c04 and
GET_STATUS64 0x4c05. Read-only is flag1. The kernel returns device/inode/rdev
from the backing file, which the implementation compares to its open file.
See the [v4.19 loop ABI](https://raw.githubusercontent.com/torvalds/linux/v4.19/include/uapi/linux/loop.h)
and [loop status implementation](https://raw.githubusercontent.com/torvalds/linux/v4.19/drivers/block/loop.c).

Module loading is the ARM EABI `init_module` syscall128, with the bounded
image pointer/length and empty NUL-terminated parameter string. This is
compiled only for ARM Linux/Android. See the
[v4.19 ARM syscall table](https://github.com/torvalds/linux/blob/v4.19/arch/arm/tools/syscall.tbl).
Other platforms return Unsupported; host tests never execute target mount
or module-loading paths.

## Caller responsibilities and validation limits

The stage intentionally does not mount system/product/APEX, prepare Android
properties or Binder, or start TEE. If the current TEE provider needs those
runtime files, retain their existing mounting/launching setup separately.
This task preserves stock HDCP parameters; it does not remove the known TEE
dependency. The complete MI/config/kernel initialization remains a hardware
validation gate.

The caller must keep diagnostics alive, preserve the exact INI/config bytes
through module initialization, claim cold provenance, and invoke the existing
owned MI initialization afterward. No subsequent user MI process may race that
transition. Keep returned owners and persistent scanout memory until reboot.

Validation so far:36 unit tests pass, including loop ABI/identity, mount source
and read-only checks, mount path boundaries, phase refusal, non-PID1 refusal,
and exact single-byte INI edit. All12 extracted module files pass the same
ELF/vermagic gate. The static ARM build succeeds. No target mount, loop setup,
module insertion, vendor process or TV operation was run during this task.

Integration patch: `assets/research/midaemon-replacement/cold-staging.patch`.
It adds the new module plus the `pub mod staging;` export only; `owned.rs`
is deliberately preserved.

The integrated stage also validates p4 (tvcertificate, start22528,16384sectors)
and mounts it at /vendor/tvcertificate with ro,noload. The original ext4 image
has the journal-recovery flag set; it must not be replayed or substituted with
an empty RAM directory. Non-secret verification record:
assets/build/display-agent/tvcertificate-filesystem-check.json.
