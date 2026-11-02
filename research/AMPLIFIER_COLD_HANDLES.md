# V109 amplifier GPIO/IIC cold preparation

Offline evidence only. No TV calls made by this agent. Main's direct PCM/audio path is already physically proven; this work addresses the remaining stock middleware cold-bootstrap dependency.

Integrate only `src/mi-bootstrap/src/amp_handles.rs` and add `pub mod amp_handles;` to the existing crate root. The rest of the agent's local `src/mi-bootstrap` tree is a copied test fixture, not an integration payload.

Use in the exclusive cold-start phase, after SYS/FLASH and board configuration are ready, before stock MI clients and before publishing SYS_USER readiness:

```rust
let mut session = amp_handles::Session::new(amp_handles::DeviceIo::new(&mut device));
let handles = session.prepare()?;
// Retain session through this phase; never reconstruct it to retry a failure.
// Drop its borrow when finished, then existing MiBus can adopt the handles.
```

`Device` must already hold `ColdBootProvenance` through `permit_initialization`. A missing handle alone never authorizes initialization. The adapter checks that provenance plus SYS_USER=0 and prerequisites selectors40/0/3. Generic session is single-attempt even after errors. A failure may leave GPIO/IIC initialized or one handle allocated; no rollback or blind retry is attempted. Logical handles are deliberately left open for the owning PID1's later amplifier path. File descriptor release does not release these logical handles in the examined driver.

## Exact wire

All use the existing 16-byte outer MI envelope: u16 payload size, six zero bytes, u64 payload pointer. Payloads are little endian.

| Operation | Request | Payload |
|---|---|---|
| GPIO Init | 0xc0012900 | 1 zero byte |
| GPIO Open | 0xc0082902 | u32 result handle@0, u16 pad30@4, zero u16@6 |
| GPIO GetHandle | 0xc0082904 | same 8 bytes |
| IIC Init | 0xc0011d00 | 1 zero byte |
| IIC Open | 0xc0081d02 | u32 result handle@0, u32 port9@4 |
| IIC GetHandle | 0xc0081d04 | same 8 bytes |

Both getters are lookup-only, unlike the earlier PCM GetHandle. Exact kernel branches return4 when not initialized,9 when no active handle exists,0 with handle when found. Algorithm: lookup; on4, Init and accept only0/2, then lookup again; on9, Open accepting only0, then lookup and match handle. Any other vendor status or host error stops. Existing handles skip Init and Open. GPIO handle must be0x4b00001e. IIC has prefix0x55000000 and slot0..15; port9 does not imply slot1. Existing kernel configured slot assignment is preserved.

GPIO Open allocates slot/handle-manager metadata without driving the pin. IIC Open allocates metadata without initializing a hardware bus. Init is broader: GPIO calls mdrv_gpio_init and configures GPIO infrastructure; IIC loads board bus/device configuration. Therefore these are cold-only operations and must not be put in a warm diagnostic CLI.

Board configuration `[BOARD_I2C_HWBUS]` selects port9 for bus1, speed enum2. The existing INI/kernel initialization supplies pinmux and speed; do not copy an assumed clock rate or insert a SetSpeed call. The stock `mif_iic_init` calls Init, looks up configured ports and opens misses; no SetSpeed call occurs there. Stock broad GPIO setup also applies board inversion values, but the amplifier's own configuration explicitly uses GPIO30/inversion0. This helper does not duplicate global GPIO setup; existing Rust MiBus sets inversion0 immediately before controlled amplifier reset.

## Evidence

Generated exact disassembly under `assets/research/audio-agent-direct/`:

- `gpio-init-wrapper.txt`: libmi GPIO Init0x727b4; Open in `gpio-open-wrapper.txt`0x72dcc.
- `iic-init-wrapper.txt`: libmi IIC Init0x74b98; Open in `iic-open-wrapper.txt`0x74e90.
- `gpio-gethandle-kernel.txt`: mik MI_GPIO_GetHandle0x19424c; function ends before following SetInvert (do not attribute that write to getter).
- `gpio-open-kernel.txt`: MI_GPIO_Open0x193d18; `gpio-init-kernel.txt`: MI_GPIO_Init0x193248.
- `iic-gethandle-kernel.txt`: MI_IIC_GetHandle0x198b04; `iic-open-kernel.txt`: Open0x198720; `iic-init-kernel.txt`: Init0x198338.
- `mif-gpio-init-full.txt`: libfpp stock initialization0x223d80, Init0x223e76, lookup/Open0x22488e/0x22489e, SetInvert0x2248ae.
- `mif-iic-init.txt` and `mif-iic-init-tail.txt`: libfpp0x229124, Init0x229216, lookup/Open0x229aea/0x229ba8.
- Input libraries: `assets/research/firmware-implications/tvservice/glibc/libmi.so`, `assets/research/firmware-implications/lib/modules/mik.ko`, `assets/research/audio-native/libfpp.so`.
- Board input: `assets/research/firmware-implications/tvconfig/config/board.ini`.

Host verification: 22 crate tests pass including6 new tests (exact requests/payloads, provenance failure before IO, existing handles, cold status0/2, no reinit for missing slot, failure/readback/no-retry paths). Command used on this Mac:

```
env SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX26.5.sdk cargo +1.92.0 test --lib --manifest-path src/mi-bootstrap/Cargo.toml --config 'target.aarch64-apple-darwin.linker="/usr/bin/cc"'
```

Hardware validation remains pending: freshly booted owning PID1 with stock middleware omitted must verify handles, muted amplifier programming/readback and ultimately audible playback. This evidence does not prove cold GPIO/IIC initialization on the panel.
