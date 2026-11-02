# Monadic foreground lifecycle

This historical design has been replaced by `src/lean/TV/Core.lean` and the
thirteen-effect interface in `src/lean/native/core_effects.h`. The old runtime
and its tests were removed after the Core passed ARM32 and physical-TV tests.
The remainder of this note describes the earlier implementation.

The former `src/lean/TV/Runtime.lean` owned foreground lifecycle ordering. It was parameterized
by a typed `Effects m` record: read policy, spawn, nonblocking reap, poll event,
signal, input forwarding, power and home restoration. `Runtime.Native` supplies
the scalar IO adapter. The module has no direct Linux calls.

The controller uses the existing executable policies in `TV.Apps`,
`TV.Settings` and `TV.Supervisor`. It checks app identity and current policy
before spawning. Reaping takes priority over input. Home and epoch cancellation
stop key forwarding; the first non-repeated Home sends TERM, and the existing
strict monotonic deadline sends KILL once. Natural exit, spawn/policy errors and
wait errors produce typed outcomes, then one home-restoration effect.

`src/tvinit/src/runtime_effects.rs` implements individual Linux operations.
A scoped thread-local context owns borrowed launch arguments, the process
registry, input devices and launch socket. Signals and forwarded input require
its current child identity. The context pointer is cleared on all Rust returns;
foreground file descriptors are released before the next home frame. The Rust
adapter formats completion status and the existing home renderer remains Rust.
The former Rust wait/input/escalation loop has been deleted.

The IO ABI is documented in `src/lean/native/runtime_effects.h`. The pinned Lean
4.35 compiler erases IO world tokens, so the callbacks and exported entry return
owned IO result objects without an extra world argument. Effect handlers use
Lean-exported result constructors; shared `lean_runtime.rs` owns result decoding,
reference release, module initialization and per-thread runtime attachment.
Both standalone QEMU and native PID1 must link the full Lean runtime.

Startup-sensitive modules explicitly use `prelude` and narrow imports, avoiding
`import Init` and its eager random-generator initialization. The pure policy
proofs remain separate from hardware guarantees. In particular,
`valid_app_identity_is_nonroot` proves UID/GID policy, not Linux file access or
TEE descriptor isolation; the latter still requires the independent runtime
boundary probes.

Offline validation:

- `python3 test/lean/tv/check-runtime.py`: 17 lifecycle traces and 13 scalar ABI
  decoding cases; also rejects direct full-Init/Random initializers.
- `lake build TV.Proofs`: the nonroot identity theorem compiles with the default
  UID base and a separately built copy with the G13 base of 19000.
- ARM32 Cargo check of the wired `tvinit` passed in the isolated
  `assets/build/monadic-runtime-check` output directory.

The combined native PID1 passed entropy-disabled QEMU startup, persistent shell,
app/Home, live reload, and reboot checks. The standalone controller also passed
ignored-SIGTERM, duplicate-Home, TERM-to-KILL, and subsequent-app checks. All nine
Settings UI cases passed, including disabled-launch refusal and PIN persistence.

On the G06, build `90c15507804e8bd42267a3146eca324f07bb3e2fccef115ba2fdf903a04787dc`
booted with its recovery shell, stock TEE helper, display, and audio ready. Lean
reaped the real Slides child after Home; Doom submitted frames. Live evidence is
under `assets/research/lean-pid1-live/`. The TEE helper runs directly, without a
tracer process; its readiness check accepts the vendor kernel's observed zero
syscall-number field only with the expected receive request and private device.

This is a Lean control-flow port, not a claim that all PID1 code is now Lean.
Rendering, device operations, network/console plumbing, and narrow native effect
handlers remain Rust/C. The monadic controller and policy stay in Lean.

The global volume service also runs its loop in Lean (`TV.Volume` and
`TV.Volume.Native`). Lean owns calibrated step rounding, clamping, mute memory,
external-update handling, and commit-before-publish ordering. A failed atomic
write retains the prior logical state and does not publish a new overlay.
Values cross the scalar boundary as millionths of full-scale amplitude; the
existing 0.30 ceiling, twenty steps, and 0.075 default are preserved.

`src/pid1/src/volume_control.rs` supplies input polling, atomic file writes and
OSD publication. Its thread attaches to the Lean runtime and invokes the Lean
loop; it contains no second volume state machine. `test/lean/tv/check-volume.py`
checks the model and effect traces, and `tools/pid1/check-repl-qemu.py --apps …
--media …` exercises ten volume presses in each direction plus mute/unmute
through the actual ARM32 service and virtual input device.

Validation of this volume port: native ARM32 build and the clean Nix Lean checks
passed. `assets/build/pid1-repl/qemu/1793567847253227000/` records the complete
native-supervisor test with boot entropy disabled, real virtual-key delivery,
all ten up/down transitions, mute/unmute, live reload and root-shell checks.
The new volume service has not yet been run on the physical TV. The standalone
launcher fallback used without native PID1 still has its older volume handler;
the native PID1 path now uses the Lean service exclusively.
