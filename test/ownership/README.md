# Scalar ownership checks

Run from the repository root:

```
python3 test/ownership/check.py --kernel /path/to/arm/zImage
```

This builds the actual `src/pid1/src/ownership.rs` byte adapter and generated
`TV.Hardware.Ownership` code on the host and ARM32, compares a deterministic
transcript, and exercises the adapter from a worker thread. Neither executable
links or initializes the Lean runtime. The generated C uses only scalar
UInt32/UInt64 operations; discarded boxed compiler wrappers are not reachable.

The oracle preserves the original Rust rules with the deliberate ASCII-only
kernel-record restriction. It covers exact serial-token boundaries, unsigned
PID syntax and overflow, all seven comm names, first-State-line semantics,
T/t recognition, malformed/Unicode records and random input. Non-ASCII comm
records have a separate invalid result so they cannot hide a recognized owner.

Results and generated files stay under `assets/build/ownership-check/`.
No kernel files, device nodes, or hardware services are modified by this test.
