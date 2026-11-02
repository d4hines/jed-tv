# zr: the Lean runtime and 32-bit ARM assembly DSL

Copied from dande-router (`~/repos/dande-router/main`) at commit
`d5fb24e5c7e49236d4678cd766f1bb6e5123d754` ("Describe the target data-plane
architecture"), from its `src/pid1/lean`, `tools/zrlow-check.py` (now `zr-cgen check`), `test/asm`,
`test/leanrt/csrc` and the runtime conformance suite in `test/leanrt/lean`.
This is our own copy: it is not pinned to or synced with the router.
The router's `test/asm` (the differential test of `ZrAsm.Sem` against QEMU
user mode, plus its firmware routines) was removed here. It needs that
repo's firmware modules, `qemu-arm` and systemd. `ZrAsm.Isa`/`Sem` are as
validated there; the NEON additions are not modelled.
Module names (`ZrLean.Low`, `ZrAsm`, `ZrAbi`) are kept so that diffs against
the router stay readable.

- `ZrLean/Low`: "ZrLow", a Lean runtime written in Lean: allocator, reference
  counting, arrays, byte arrays, strings, closures, big naturals, integers,
  once-cells and single-executor mutable references.
  Its compiled C replaces upstream's `libleanrt` (C++, libuv, GMP, mimalloc).
  Single-threaded; links no Lean `Init` code (`zr-build link-check` fails on any
  `l_*` symbol the runtime does not define).
- Native ARM32 PID 1 now links this runtime by default via `tools/tvinit/build.rs`.
  Core, menus, finite volume and cooperative audio share one Lean executor;
  native display/console workers do not enter it. Host-side tests may still
  link the installed Lean runtime. No upstream runtime is needed to build PID 1.
  The finite initializer alias list in `ZrAsm/Runtime.lean` covers proof/type
  imports only; unsupported executable imports must fail at link time.
- `ZrAbi`: generates the freestanding headers, including `lean.h` itself.
- `ZrCGen` (`zr-cgen`): the passes `gen.sh` runs over the compiler's C
  (formerly the router's Python): drop the redundant `zr_prim_apply_stack`
  declaration, inline the scalar header modules, extract the array-push fast
  paths, and the ZrLow check, which first plants each kind of violation in the
  real C to show that it catches them.
- `ZrAsm`: ARMv7-A ARM-mode assembly DSL: AST (`Isa`), executable semantics
  (`Sem`), printer (`Print`) and the `Asm` monad (`Dsl`, and `Neon` for
  Advanced SIMD). `Sem` interprets the scalar core subset only: NEON, the
  ARMv6 media instructions and FFmpeg's extra load/store forms are printed and
  assembled but a step on them is stuck.
- `ZrAsm.Ffmpeg`: FFmpeg's 32-bit ARM H.264 assembly, one module per file,
  written in the DSL. `zr-asm-gen ffmpeg-arm OUT` writes it; `zr-build rt`
  assembles the H.264 kernels from it into OUT/obj/h264 for the Lean
  decoder's `ZrTv.H264Kernels`. Each file assembles to the same code, data,
  relocations and symbols as FFmpeg's original: `lake env lean --run
  test/ffmpeg/AsmCheck.lean` (the originals come from the repository's
  history, where they were vendored for FFmpeg's C decoder).
- `ZrAsm.Libc`: what apps need from a C library, linked with each app's
  `main` (`zr-asm-gen app`; apps and the conformance binary link
  `-nostdlib`): `_start`, NEON `memcpy`/`memmove`, `__aeabi_uldivmod`
  (`lake env lean --run ../../../test/lean/zr/libc/Uldivmod.lean` runs it in `ZrAsm.Sem`
  against `Nat` division).
- `ZrAsm.Yuv`: the YUV 4:2:0 -> RGB row kernel in NEON, with the row scaler
  `zr_u32_scale_row`, in the runtime build's `tv.S` (`zr-asm-gen OUT`), for
  the Lean video blit `ZrTv.Video.blitYuv`;
  `lake env lean --run ../../../test/lean/zr/yuv/ScaleRow.lean` runs the scaler in
  `ZrAsm.Sem` against `x * sw / w`.
- `test/conformance`: the runtime conformance transcript recorded on
  upstream Lean's runtime; our runtime must reproduce it byte for byte.

Removed in the copy: the router's application modules, its FSBL, and the
bare-metal board pieces (Zynq startup, CP15/MMU, console, fault reports,
telemetry, CCMP/CRC assembly, EABI division stubs, memcpy bursts).

## Linux (32-bit ARM)

`ZrLean.Low.Linux` gives the runtime its heap (64 MiB mmap2 chunks, grown on
demand), the fatal stop and the panic log, over the DSL system call.
`zr-build conformance` compiles the runtime, the conformance suite and its
Lean entry point (`test/conformance/ConformanceLinux.lean`, with a DSL
`main`) for 32-bit ARM (zig cc, no C library), runs it as PID 1 in QEMU
(`zr-qemu32`, ZrQemu32.lean: AArch32 Cortex-A55, the repo's surrogate kernel) and
requires byte-identical output to upstream Lean (recorded by `zr-build
conformance --record`). It passes, and is a test/check.py gate.

`ZrLean/Low/Str.lean` adds what apps need beyond the router's firmware:
strings (push, append, compare, UTF-8 validation and decoding), List/Array
conversion, `panic!`, `Nat.log2` and byte-array equality. The router only
linked C versions of these into its tests; here they are Lean, and
`zr-cgen check` (run by `gen.sh`) confirms every runtime function
compiles to scalar C with no allocation, boxing or closed terms.

No C: the Linux heap, fatal stop and panic output are Lean
(`ZrLean.Low.Linux`); the system call and each app's `main` are DSL
(`ZrAsm.Linux`). Big naturals are limited to 2^128 by design, so
cryptographic bignums must be written separately. `Int` (`BigInt`,
`HeaderInt`) uses upstream's representation for small values (scalars in
[-2^30, 2^30) on 32-bit) and the big-Nat object, in 128-bit two's
complement, beyond: Ints are limited to [-2^127, 2^127), and a result
outside stops the program with `lean: Int outside [-2^127, 2^127)`.

## Tools for Lean that runs on this runtime

- `zr-build` (`ZrBuild.lean`; `lake -d tools/lean/zr exe zr-build ...`):
  - `rt [OUT]`: the runtime for 32-bit ARM (default `tools/lean/zr/.lake/rt`):
    generated C/headers/runtime.S and objects; `gen OUT` only generates;
  - `app PKG MODULE ENTRY OUT`: a Lean app, linked without a C library;
  - `link-check PKG`: compiles a Lake package's C for 32-bit ARM and links
    it with the runtime; lists missing `l_*` (Init) and `lean_*` symbols.
  - `transcript PKG [TIMEOUT]`: runs a package's transcript on the host
    (upstream Lean) and on this runtime in 32-bit QEMU; must be identical.
  - `timed PKG FN STAGES [--keep DIR] [-- QEMU_ARGS]`: a package's FN on
    each stage, timed in the guest (`-icount shift=0`: instruction counts);
  - `conformance [--record]`: the runtime conformance suite on 32-bit ARM
    against upstream Lean's recorded output (or record that output);
  - `profile PKG FN STAGE [TOP] [--callers F,G] [--focus F] [--keep DIR]`:
    where `FN STAGE`'s instructions go (a QEMU plugin, test/profile/plugin.c).
  Their binaries' `main`s are DSL (`ZrAsm.Linux`): no C, no C library.
- `ZrLean/Low/InitShim.lean`: the standard-library shim, the few `Init`
  functions ordinary code reaches without inlining (`xs[i]!`, `toString` on
  `Nat`, `Char.ofNat`, `ByteArray.extract`, `Array.reverse`, `Int.toNat`,
  `Int.pow`, `>>>`/`<<<` on `Int`, `default` of `Int` and `Array`), bound to
  upstream's names by runtime.S. `lake env lean --run ZrInitCheck.lean`
  confirms upstream's signatures (borrowed/owned) are the ones it assumes;
  `test/shim` (and, for `Int`, the conformance suite) compares its
  behaviour with upstream on 32-bit ARM.

The rules for app code are in `../yt/RULES.md`.

## Integration boundary

The Zr runtime and Lean YouTube are separate build targets. They do not replace
the current multi-threaded PID1 runtime or the default applications. Zr is
single-threaded: port the callers or provide equivalent synchronization before
using it in a process that runs Lean on multiple OS threads.

`zr-build app` reuses an existing runtime build. After changing runtime source,
run `lake -d tools/lean/zr exe zr-build rt` explicitly before rebuilding an app.
