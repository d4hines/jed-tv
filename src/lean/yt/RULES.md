# Writing Lean for the TV

All Lean in `src/lean/yt` runs on **our** runtime (`src/lean/zr`, "ZrLow"), not
upstream's. Same compiler, same object layout, but:

1. **No standard-library code is linked.** Calls to `Init` functions that the
   compiler does not inline become `l_*` symbols that do not exist on the TV.
   Common offenders: `toString`/`s!"{n}"` on numbers (`Nat.repr`),
   `ByteArray.extract`, `Array.reverse`, `String.splitOn`, `List.toString`,
   `HashMap`. Loops (`for`, `while`), `Array.map/foldl/push`, `ByteArray` get/
   set/push/append, `String.push/append`, `String.fromUTF8?`, `qsort`... are
   fine. Don't guess: run the checker.
2. **Check every change:** `lake -d tools/lean/zr exe zr-build link-check <your lake package dir>`
   compiles your generated C for 32-bit ARM and links it with the runtime. It
   must print PASS. It lists missing `l_*` (write a helper, e.g. in your own
   area or `Yt/Bytes.lean`) and missing `lean_*` (runtime functions not
   written yet: report them to the coordinator rather than working around
   them in C).
3. **Run your tests on both runtimes:** give your test package a transcript
   (`@[export zr_test_transcript] def transcript (k : Nat) : ByteArray`, `k`
   always 0, plus `lean_exe transcript` printing it) and run
   `lake -d tools/lean/zr exe zr-build transcript <test package dir>`. It compares the
   host (upstream Lean) output with our runtime on 32-bit ARM in QEMU byte for
   byte. `USize` is 32 bits there.
4. **Big numbers:** `Nat` beyond 2^128 is a fatal error on our runtime, and
   values from 2^31 allocate. `Int` outside [-2^127, 2^127) is a fatal error,
   and values outside [-2^30, 2^30) allocate. Use `UInt32/UInt64/USize` in
   hot loops; write your own limb arithmetic for cryptography.
5. **Single-threaded.** No `Task`, `IO.asTask`, threads, `IO.Ref` across
   threads. No `IO` filesystem/process/network APIs either: the app gets the
   platform through its own `@[extern]` functions. Libraries should be pure
   functions over `ByteArray`/`String`/scalars, or state machines driven by
   the caller.
6. **No extra imports of `Init.*`, `Std.*` or `Lean.*` modules.** The prelude
   (`Init`) is available implicitly; importing its submodules or `Std`/`Lean`
   pulls in initializers that don't exist on the TV.
7. **No floating point.** The runtime has no `Float` arithmetic,
   conversions or literals (`lean_float_*`, `Float.ofScientific`). Use
   fixed-point integers (the AAC decoder does).

Layout: library code in `Yt/<Area>/*.lean` (all modules under `Yt/` are
built, no root file to edit). Tests in their own Lake package,
`test/<area>/` (with `[[require]] name = "Yt" path = "../.."`), so areas never
edit shared files.

## Building and running an app

- `src/lean/yt/Yt/Platform.lean`: typed system calls and the app contract
  (display lease with bounded drawing primitives, PCM audio, remote input).
  The pixel pointer stays inside the runtime's ZrTv module.
- `lake -d tools/lean/zr exe zr-build app src/lean/yt MODULE ENTRY OUT`: links a module's exported entry
  `(argc : UInt32) : BaseIO UInt32` (each app its own name, e.g.
  `yt_app_main`) and the Yt modules it imports with the runtime, ZrTv and
  the native library into a static 32-bit ARM binary.
- `test/lean/yt/demo-app.py BINARY --apps ... --media ...`: runs it as
  `/bin/ytplay` under the real launcher and confinement in the graphical QEMU
  demo, screenshots it and prints the serial log. In that surrogate, fbdev
  dirty tracking costs about 150 us per 4 KiB page written per frame (a
  full-screen redraw is about 130 ms); this is QEMU's fbdev, not the TV's
  shared-buffer path.

## Performance pitfalls (found by the library work)

- `Int32`/`Int64` literals become lazily initialized constants checked on
  every use; prefer unsigned types with explicit two's-complement handling.
- A `for`/`while` loop with two or more mutable variables allocates a tuple
  per iteration; hot loops should be tail-recursive functions over scalars.
- `a.size` read in the same call's arguments after `a.push` forces the push
  to copy the array (it is still shared there). This made a parser quadratic.
- An `@[inline]` function containing a loop also gets a standalone copy using
  generic loop helpers that do not link; keep loops in non-inline functions.
- `for x in [a:b]` allocates a range object each time the loop starts (nested
  loops: every row); write hot loops as structural recursion.
- A structure update (`{ s with .. }`) is done in place only on one path:
  updates in several branches (or one feeding an inlined function that
  updates again) allocate a fresh object on the others. Compute the new
  field values first and update once.
- A structure update inlined into a function that also passes the structure
  to (borrowed) helpers gets its field projections hoisted above those calls,
  so the structure is still live at the update and is copied whole. Read
  through helpers first and update through small `@[noinline]` setters
  (`Dec.withY`, `Dec.setMvC` in Yt/H264/State.lean).
- Such a hoisted update is also expensive when it does stay in place: with a
  branch or a borrowed call between the projections and the update, every
  field is retained and released (36 for the H.264 `Dec`) instead of one
  field stored. Keep setters free of branches (a kernel that checks its own
  bounds, not an inlined check in the setter); `lean_dec(v_unused` in the
  generated C marks the slow form.
- `Nat` arithmetic on positions costs an overflow check, boxing and a
  reference-count test per operation (about ten instructions). Hot index
  arithmetic should be `USize` (one instruction), reading and writing bytes
  through `Yt.Bytes.u8`/`u16`/`u32`/`u8set` (one size check, no `Nat`):
  converting the H.264 deblocking driver alone saved 9% of the decoder.
- `UInt32` values stored in an `Array`, a tuple, an `Option` or a closure are
  boxed, which allocates on 32-bit ARM. Keep them in structure fields or
  `ByteArray`s; return extra results in fields rather than tuples.
- Don't build a `ByteArray` as `⟨Array.replicate n v⟩` (an array of `n` boxed
  bytes, built and freed): use `Yt.Bytes.filled`. Block moves:
  `ByteArray.copySlice` (in place when the destination is unshared).
- `Int` `%`/`/` take a 64-bit division path; wrap with `Int.toInt16` etc.
- Where the instructions go: `lake -d tools/lean/zr exe zr-build profile PKG FN
  STAGE` runs a package's exported `FN STAGE` as PID 1 in QEMU with an
  instruction-counting plugin and prints a per-function profile (`--callers`
  counts calls by caller, `--focus` lists a function's hot blocks, `--keep`
  keeps the binary for disassembly); `zr-build timed` gives the counts alone.
- Not linkable yet (avoid or ask for a shim): `deriving Repr`,
  `String.fromUTF8!`, `String.drop`, `Int64` arithmetic, `UInt64.toFloat`,
  `Array.qsort`/`swap` (`lean_array_fswap`).
