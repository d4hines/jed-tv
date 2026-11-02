# Development probes

These diagnostic programs and one-off experiments are kept outside the
auditable production sources in `src/`. Audio, display, and MI probes reference
the real runtime modules by path instead of copying their implementations.
Build outputs and captured firmware/evidence belong under `assets/`.

`tools/native-app/run-g13.py` still builds `android-client.rs`, `adb-relay.rs`,
and `audio/protocol-probe.rs` as development helpers. Their paths were updated;
moving them here does not remove that diagnostic workflow.

Related experiments live in `research/tee-provider`, `research/tee-rpc`,
`research/tee-trace`, `research/hello`, `research/gpu`, and `research/launcher`.
The active City Flight GPU renderer remains in `src/flightgpu`.
