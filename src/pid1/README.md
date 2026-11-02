# PID 1

The TV's supervisor and its privileged services live here, in every authored
runtime language. Build tools remain under `tools/`; generated Lean C, objects,
and binaries remain under `assets/build/`.

| Path | Responsibility |
|---|---|
| `lean/TV/` | Core policy and proofs, app lifecycle, menus, Settings, volume, and audio protocol/mixing |
| `lean/native/` | Linux ABI declarations and native mechanisms used by Lean |
| `src/` | Native entry, hardware startup, cooperative service effects, and development console |
| `tvinit/` | Core interpreter, app confinement, menu effects, and settings storage |
| `audio/`, `display/` | Privileged media device transports and presentation |
| `mi-bootstrap/` | Firmware-specific cold initialization and speaker routing |
| `scripts/` | Runtime setup, app staging, networking, and native/Android hosting scripts |

Shared app interfaces remain in `src/platform/` and `src/shared/`. The
Lean-written runtime in `src/lean/zr/` is shared with apps. App implementations
and the bootloader have their own directories outside PID 1.
Development probes, the experimental TEE replacement, and the RAM loader live
under `research/`.

`Cargo.toml` builds the native G06 entry. Its `tvinit`, audio, and display
dependencies also belong to the app workspace in `src/Cargo.toml`, which
builds the QEMU and Android-hosted supervisor. Both use the same source.
