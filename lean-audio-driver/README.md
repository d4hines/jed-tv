# Lean audio driver (dande-tv), kept separate for now

**Status note.** This folder is a verbatim snapshot of the Lean audio driver
from dande-tv (branch `lean-audio-g13`, commit in `integration/source-commit.txt`,
built against dande-tv `main` at `a6a3c7d`). It is deliberately left in its own
folder and not wired into this repository's tree yet. **The next push should
integrate it**: move the files to their places in the tree (the paths below are
dande-tv's), apply `integration/dande-tv-integration.patch` to PID1's crate,
build script, `TV/Pid1.lean` and the Lean check gate, and delete
`src/lean/lakefile.toml` here (the modules belong to the existing `tv` Lake
package). dande-tv's `main` has restructured `src/pid1` since `a6a3c7d`, so the
patch is a guide to what the integration touches, not something to apply blind.

Hardware status: not yet tested on either TV. G06 trial:
[dande-tv#20](https://github.com/d4hines/dande-tv/issues/20); G13 trial (gated
by the wire-layout check):
[dande-tv#21](https://github.com/d4hines/dande-tv/issues/21).

## What it is

PID1's direct MI PCM audio driver with every decision in Lean and only
mechanisms in Rust. Design: `research/LEAN_AUDIO_DRIVER.txt`; plan and status:
`research/LEAN_AUDIO_PLAN.md`; G13: `research/LEAN_AUDIO_G13.txt`.

| Path | Role |
|---|---|
| `src/lean/TV/Audio/Fault.lean` | Why a mechanism failed; decoding of Rust's result words |
| `src/lean/TV/Audio/Gain.lean` | `scale`: boost, clamp, global gain in millionths, ceiling 0.3 (max output 9830) |
| `src/lean/TV/Audio/Resample.lean` | 44.1/48 kHz mono/stereo to 48 kHz stereo: scalar `step`, `feed` per frame, a fold |
| `src/lean/TV/Audio/Protocol.lean` | The O/W/D/R/F socket protocol's decisions |
| `src/lean/TV/Audio/Session.lean` | The driver over an `Effects m` record: open, write, played, flush, cleanup, connection loop |
| `src/lean/TV/Audio/Native.lean` | `@[extern]` bindings and `tv_audio_client` |
| `src/lean/TV/Audio/Proofs.lean` | The proofs (built by the check, not shipped) |
| `src/lean/native/audio.c` | ByteArray/String marshalling via `lean.h` |
| `src/audio/service.rs` | Rust mechanisms: sockets, clock, lease, raw ioctls, ownership check |
| `src/audio/{mi-pcm,aout-route}.rs` | Exact V109 MI wire structs, one function per ioctl |
| `src/audio/direct-main.rs` | The driver as its own static ARM32 process (G13 trial) |
| `tools/audio/build.rs` | Compiles the Lean C and `audio.c` into the crate's archive (feature `lean`) |
| `tools/audio/mi3-audio-requests.py` | Recovers MI ioctl request numbers from a `libmi3.so`; the G13 gate |
| `tools/audio/g13-direct.py` | G13 trial: build, stage, quiesce Android audio, run, restore |
| `test/lean/tv/audio/Check.lean` | 30 mock-trace cases ported from the old Rust unit tests |
| `test/lean/tv/audio/Axioms.lean` | Axiom audit of the main theorems |
| `test/lean/tv/check-audio.py` | The host check: cases, proofs, audit, initializer and C checks |
| `test/unit/audio/*.rs` | MI wire-layout tests for the Rust crate |
| `integration/` | What changed outside this folder in dande-tv, as a patch |

## Proved (about the shipping definitions)

- `scale_never_exceeds_ceiling`: every output sample lies in [-9830, 9830] for
  any boost, any gain file contents and any sample (`bv_decide`); the bound is
  reached, so no tighter bound is true.
- `output_frames_match_rate`, `feed_48k_is_identity`, `chunks_do_not_change_output`,
  `consumed_is_inverse`: the resampler is exact rational arithmetic and a fold.
- `cleanup_in_order`: gain restore, then route disconnect, then PCM stop/close,
  then the lease; an unresolved route keeps the PCM open and retained; every
  failure poisons; checked on all 256 combinations of what the open took and
  which mechanism fails (`native_decide`, since the kernel cannot unfold core's
  `Array` loops). `poisoned_lease_refuses_open` and friends by `rfl`.

Trusted: the `@[extern]` mechanisms in `service.rs` and `audio.c`, the MI wire
layouts, and the certificate axioms of `bv_decide`/`native_decide`.

## Checking this snapshot

Lean 4.35.0 on PATH (`lean-toolchain` pins it), a C compiler for the `-fsyntax-only`
check of `audio.c`:

```
python3 test/lean/tv/check-audio.py
```

Expected: nine `PASS:` lines (faults, gain, resampler, protocol, session
open/cleanup 20 cases, session protocol 10 cases, proofs with the allowed
axioms, no full-Init initializers, C compiles). The Rust crate needs dande-tv's
`tvinit` crate for its `lean` feature and its workspace for `cargo test`, so it
is not built here; the wire-layout tests are included for the integration.
