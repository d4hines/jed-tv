# Cold-plan adapter checks

From the repository root:

```
python3 test/cold-plan/check.py --kernel /path/to/arm/zImage
```

The runner compares the frozen pre-port Rust validator/preparer with the
compiled Lean policy, first on the host and then on the owned ARM32 Zr runtime
under QEMU. It uses the production `cold_plan.rs` channel/callback adapter and
`lean_runtime.rs` executor guard. Its isolated runtime initializer initializes
ColdPlan directly; it does not exercise the unrelated Core startup sequence.

Coverage includes all 35 flags and 8 fixed parameters, global duplicate keys,
unknown initializers, exact errors, Unicode whitespace, CRLF and NUL bytes,
64 KiB input, byte-preserving tuner substitution, worker/main requests,
wrong-thread rejection, and no simulated write after a policy failure. The
full MI-bootstrap test suite separately retains device/mount/ownership checks.

`--runtime` accepts an existing *matching-source* test runtime cache containing
`gen/` and `libzr_native.a`; without it the runner builds the runtime. All build
artifacts and logs go under `assets/build/cold-plan-check/`.

These tests do not mount a partition, load a kernel module, or access a TV.
