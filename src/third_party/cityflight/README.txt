City flight demo: simulation, safety filter and aircraft dynamics
================================================================

Vendored from https://github.com/uyhcire/vibecheck-tsa, branch
share/smart-tv-starter, commit c8f5e9873b1bd487b6af6c16b77875b196f42a91
(2026-11-01), shared by its author for a smart-TV port. The repository
carries no license file; these files are used here for the hackathon port
only. Nothing in this directory is edited except where noted below and one
change in src/envfilter.rs: `thread_cpu` calls musl's `__clock_gettime64` on
32-bit Linux (the plain symbol there is the old 32-bit-time entry point) and
reads a nanoseconds field of 32 bits plus padding (musl time64 layout). It
only affects the reported CPU time per tick.

lean/   JsbExact (a Lean 4 port of JSBSim 1.3.3's c172p model, bit-exact with
        JSBSim on macOS) and CityJsb (the flight-sim plant over it). Built for
        the TV by demo/flight.nix: Lean emits C, zig compiles it for 32-bit ARM
        with the Lean runtime from tools/lean-arm/build.sh. lakefile.lean here
        is ours. Ours too are JsbExact/F/ and CityJsb/F/: a copy of the model
        with every structure field typed `Float` instead of a generic scalar,
        so Lean stores the numbers unboxed (the generic version allocates one
        heap object per number, which made a plant step about 2.3x slower).
        The operations are the same in the same order; the exported plant
        entry points in CityJsb/Plant.lean call the F version. Tools/JsbDiff.lean
        (`lake exe jsbdiff`) steps both versions 80,000 frames and checks every
        state and observation field bit for bit, and times them.
c/      The project's C API over CityJsb (cityjsb.h, cityjsb_abi.c; the
        latter marks the plant state multi-threaded when it is cloned, for
        the filter's rollout workers). The sin/cos shims (their jxmath.c) are
        Darwin-specific; the Linux version is written by demo/flight.nix and
        jxmath_host.c is the same for the host-side differential test.
src/    The GUI-free part of their Rust crate `cityflight`: sim.rs (the
        simulation loop), envfilter.rs (the safety filter), escape.rs,
        filter.rs, model.rs, pilot.rs, world.rs (the procedural city), jsb.rs
        (FFI to the Lean plant) and harness.rs. Their renderer, window and
        input code (macroquad, Metal) are not included; src/flighttv draws
        the same world on the TV's framebuffer.

Cargo.toml, build.rs and src/lib.rs are ours: the crate always has the `jsb`
feature, and build.rs links the static libraries from demo/flight.nix
(CITYJSB_DIR) instead of their macOS dynamic library.
