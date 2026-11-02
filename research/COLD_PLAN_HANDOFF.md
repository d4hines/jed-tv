# Lean cold-configuration slice

Worktree: `/private/tmp/dande-cold-plan`, based on `f698010`.
This branch is separate from the frozen release and has not run on hardware.

`TV.Hardware.ColdPlan` owns the exact 35-flag/8-parameter INI schema, duplicate
and unknown-initializer checks, Rust-compatible Unicode trimming, and the
single-byte `MI_TUNER_INIT=1` to `0` preparation. It retains the former error
kinds/messages and rejects already-prepared input. Its checked theorems state
schema size/uniqueness and that successful preparation passes full validation.

The hardware worker submits owned text to `src/pid1/src/cold_plan.rs`; only
PID1's existing Lean executor evaluates it. The bounded channel carries no
Lean objects. Initializer failure is cached; queue, timeout and policy failures
return to the existing terminal staging-failure path. Preparation, post-bind
validation and pre-module validation remain separate checks. Core's public
interface is unchanged; its existing native tick services the requests.

`ColdPlatform` still owns every raw filesystem/device effect. The root-PID1,
serial/kernel, RAM-root, read-only eMMC, partition/loop/mount identity, module
ownership, phase and no-retry checks remain in Rust. No new boot ordering is
introduced, and live-handoff refusal remains unchanged. The removed Rust
validator exists only as a host/ARM differential oracle under `test/cold-plan`.
The standalone probe drops `validate-cold-ini`; its device status commands stay.

Validation: `test/cold-plan/check.py` runs the actual adapter/owner guard against
the old implementation on host and ARM32 Zr. Both produce transcript
`4c48e0dab7dc33b2` across 224 cases including maximum-size inputs, exact errors,
worker dispatch and wrong-thread refusal. All 48 remaining MI-bootstrap tests
pass. Full native ARM32 PID1 integration build passed (`repl`, existing target
configuration); see `assets/build/cold-plan-pid1.log`. The built image is an
offline validation artifact, not a staged or hardware-qualified release.

Integration note: `tools/tvinit/build.rs` adds `Hardware/ColdPlan` to its module
and build lists. Reconcile that small list edit with any concurrent
`Hardware/TeeBoundary` addition; do not alter either module's policy.
