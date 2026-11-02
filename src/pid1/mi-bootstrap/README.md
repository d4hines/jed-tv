# V109 MI bootstrap module

This crate provides direct Rust ioctl adapters for the original G06. The
library is intended to be owned by the existing PID 1. Its CLI is strictly
read-only and cannot authorize initialization.

It implements exact V109 display/audio INIT requests, controller handoff and
muted amplifier preparation. Its staging helper validates read-only stock
mounts, creates a RAM-only INI copy and loads kernel modules. It never writes
partitions, invents SYS initialization parameters, or publishes readiness bits.
The CLI remains read-only. The cold initialization path has not been run on
the TV; warm display/audio tests belong to the separate media backends.

The kernel must have already initialized SYS and FLASH. On V109 this is done
by module loading with the stock configuration: `MI_DEV_SYS_Init` calls
`MI_FLASH_Init`, resolves `MODEL_ID` and onebin paths, then calls `MI_SYS_Init`.
Do not duplicate that with the 96-byte public SDK structure: the userspace
SYS ioctl wire structure is 392 bytes and carries additional pointers.
See `research/MIDAEMON_REPLACEMENT.md` for evidence and configuration paths.

## Build and inspect without hardware

From the repository root:

```sh
CARGO_TARGET_DIR=assets/research/midaemon-replacement/build/cargo RUSTFLAGS='-C linker=rust-lld -C target-feature=+crt-static' cargo build --offline --release --target armv7-unknown-linux-musleabihf --manifest-path src/pid1/mi-bootstrap/Cargo.toml
```

The output is
`assets/research/midaemon-replacement/build/cargo/armv7-unknown-linux-musleabihf/release/mi-bootstrap`.
No dependency downloads are required.

Offline tests cover missing prerequisites, stopping after an INIT error,
preserving false metadata after successful INIT, rejecting false audio metadata
without cold provenance, rejecting a boot already marked SYS_USER ready, and
rejecting cold ownership claims from an ordinary process, amplifier sequencing,
failure handling, GPIO level encoding, and the bounded IIC read wire layout.
On this Mac the system SDK requires the system linker rather than the older
linker from PATH:

```sh
CARGO_TARGET_DIR=assets/research/midaemon-replacement/build/cargo RUSTFLAGS='-C linker=/usr/bin/clang -C link-arg=-fuse-ld=/Library/Developer/CommandLineTools/usr/bin/ld' cargo test --offline --manifest-path src/pid1/mi-bootstrap/Cargo.toml
```

## Read-only probe

No arguments or `--help` opens no devices. On the TV, `mi-bootstrap status`
checks the original serial and reads statuses for MIKO, SYS kernel, FLASH,
SYS user, DISP, DISPOUT, OSD, AUDIO, AOUT and PCM.

There is no CLI INIT mode. Old `init --protected-native-boot` invocations are
rejected before opening any device.

`mi-bootstrap amp-status` looks up existing IIC port9 and GPIO pad30 handles.
`mi-bootstrap amp-read-status` additionally reads pad30's input level/inversion
and one byte from AD82111 register0x02. It makes no reset, INIT, speed change,
register-data write or retry. The register address is selected as the address
phase of the IIC read. The output includes both the raw MI level and decoded
physical level; MI level0 means physical high when inversion is disabled.
Stock mute writes are 0x1e and unmute writes are 0x10, but other returned values
must be reported without interpreting undocumented bits. These getters are
implemented from exact V109 code; the new register/level read still needs its
first hardware check. See `research/AMPLIFIER_REPLACEMENT.md`.

## Exclusive cold-boot provenance

The coordinator found selectors7/18/19 false while DISPOUT/AOUT/PCM were
already in use. A false status bit is therefore **not** authorization to
initialize. A warm PCM INIT even increments an internal reference count before
returning "already initialized" (vendor2); it is not a no-op.

The only mutation path requires `ColdBootProvenance`. Its unsafe constructor
is reserved for the audited, freshly booted PID-1 startup phase, before any
userspace MI initializer or client runs. This historical condition cannot be
reconstructed by looking for open file descriptors, names or status bits.
PID-1 ownership, UID0 and a one-time ownership claim are runtime-checked; the
caller must uphold the constructor's documented cold-start contract.
Do not call it from a live REPL or after restarting a service.

`permit_initialization` consumes that provenance and requires every visible
`mmcblk0*` device to already be read-only. It rejects SYS_USER ready1, as does
the sequence runner. The device permits at most one INIT attempt per module,
including failed attempts, to avoid incrementing vendor reference counts on
retry. It never changes block protection or publishes status bits itself.

For initial review, the candidate display sequence is DISP -> DISPOUT -> OSD,
and the candidate audio sequence is AUDIO -> AOUT -> PCM. These are test candidates,
not proven cold-boot sufficiency. The hardware coordinator must choose and
run a trial; offline workers should not invoke these commands against a TV.

Before the first INIT, selectors 40 (MIKO) and 0 (SYS kernel) must report
ready. The verified staged configuration must enable kernel FLASH initialization.
Selector3 is informational: SYS initialization overwrites its earlier ready bit
(see research/FLASH_STATUS_RESET.md). A requested module already reporting ready is skipped.
Otherwise its exact INIT is issued, return 0 or 2 is recorded, and its status
is read again. Any syscall error or other vendor result stops the sequence.
The library never rewrites metadata to make the readback look successful.
The false bit only decides whether to issue a call **after** exclusive cold
ownership has been independently established.

## PID-1 integration

Add a path dependency on this crate, retain `Device` in PID 1, and invoke it
only from the existing serialized hardware owner:

```rust,ignore
let mut mi = mi_bootstrap::Device::open()?;
// SAFETY: only in the audited PID-1 cold-start branch, before any userspace
// MI service or client has run. Never use this in the live REPL.
let cold = unsafe { mi_bootstrap::ColdBootProvenance::claim_exclusive_boot_phase()? };
mi.permit_initialization(cold)?;
let observations = mi_bootstrap::initialize(
    &mut mi,
    &[
        mi_bootstrap::Module::Display,
        mi_bootstrap::Module::DisplayOutput,
        mi_bootstrap::Module::Osd,
    ],
)?;
```

Inspect every observation and independently verify the physical output.
Some status bits are published by the vendor daemon, rather than the INIT
handler, so an INIT returning success can legitimately leave that metadata
bit false. Global readiness, panel power, scanout ownership, boot-logo removal,
PCM routing and amplifier configuration remain separate steps. Keep the
current app-facing capability interface unchanged while replacing these
backends.

## Cold amplifier handles

`amp_handles::Session` can initialize and open GPIO30 and IIC9 through the
existing cold-boot permission, without a middleware process. It verifies every
returned handle and rechecks the permission on every ioctl. It performs no
GPIO output, inversion or amplifier register writes. This remains offline
validated; it is not invoked by the current runtime. See
`research/AMPLIFIER_COLD_HANDLES.md` for exact ABI evidence.

## Owned startup stages

`owned::DisplayHardware::initialize(device, cold)` performs the exact cold
DISP/DISPOUT/OSD sequence and opens the controller before handing off the boot
logo. `initialize_audio(plan)` borrows that owner, initializes AUDIO/AOUT/PCM,
creates verified GPIO/IIC handles and programs the extracted amplifier plan,
ending muted. Retain both owners in PID1 for the entire boot. Audio failure
keeps the display owner available and blocks a second audio attempt.

This composition makes no shared-library calls and launches no vendor process.
Kernel modules, configuration mounts, the TEE RPC provider and authenticated
board-plan staging remain caller responsibilities. The coordinator builds for
ARM and its component contract tests pass; a cold hardware run is still pending.
