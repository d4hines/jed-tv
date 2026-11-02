# PID 1 in Lean

This Lake package contains the executable policy, its proofs, and the supervisor's
Lean services. Generated C, objects, and binaries go to
`assets/build/lake/tv/`, outside `src/`.

- `TV/Core.lean` controls app launch, Home, PIN checks, enabled apps, and child
  lifecycle. `CoreProofs.lean` states its proved invariants.
- `MenuView.lean` and `SettingsView.lean` produce the UI and navigation state.
  The stored PIN remains private to Core and its storage adapter.
- `Pid1.lean` and `Volume/Native.lean` run the supervisor and finite volume steps
  on one Lean executor.
- `Audio/` contains the PCM protocol, sample conversion, queue policy, and
  cooperative service loop.
- `Apps.lean` and `Sandbox.lean` define app identities, limits, and seccomp BPF.
- `native/` contains the Linux launch mechanism and effect declarations.

The native interpreter is in `../tvinit/`; device operations and development
plumbing also contain Rust. ARM32 production builds link the Lean-written
shared runtime at `src/lean/zr/`. Host checks use the installed Lean runtime.
This is an ongoing port, not a claim that every userspace component is Lean.

From the repository root:

```sh
lake -d src/pid1/lean build
python3 test/lean/tv/check.py
python3 test/lean/tv/core-check.py
python3 test/lean/core-zr/check.py
nix build .#pid1
```

Core owns the display-source gate and revokes an app's presentation before
waiting for it to stop. Root-only native devices and the private menu buffer
are separate from app-writable frame buffers. Linux observations, signal
handling, filesystem operations, hardware transports, and debug endpoints
remain native effects. Development root access bypasses Core policy; broad
app networking still awaits a domain/service broker. See issue 12 for the
architecture and the limits of the current proofs.
