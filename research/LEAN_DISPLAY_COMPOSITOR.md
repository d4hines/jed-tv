# Lean display composition

`src/pid1/lean/TV/Display.lean` now owns the pixel composition formerly implemented
in `src/pid1/display/pixels.rs`: dimension/length validation, letterboxing, nearest
neighbor scaling, background clearing and the volume overlay. The Rust module
passes its live slices and turns the scalar status into an I/O result.

Raw memory access remains three explicit `BaseIO` effects. Their implementations
are generated from the typed C ABI description in
`src/lean/zr/ZrAbi/DisplayMemory.lean`: one word load, one word store, and a
nonoverlapping copy. The caller supplies actual lengths, aligned live slices,
and a destination disjoint from both read-only inputs. Invalid dimensions or
overlay lengths are rejected before any write. Padding outside visible width
is retained.

Lean compilation erases the effect's world token and emits an unboxed scalar
ABI. The build rejects a changed entry signature. The final test binaries have
no Lean allocator/runtime, module initializer or boxed wrapper symbols. This
path can therefore execute on the existing Rust display worker without using
the Core/audio executor or sharing Zr runtime state. The generated C is an
intermediate build artifact under `assets/build/lake`, not authored TV code.

The host and ARM32 QEMU oracle passed 2,084 valid cases, including 9,451,660
main-thread pixels, odd dimensions, padded rows, zero-size scaled images,
native 1280x720 images, scaling and overlays at their visibility boundaries.
It also checks malformed inputs leave the destination unchanged, guard words,
and four simultaneous callers with separate buffers. The old Rust algorithm
exists only as the test oracle. The existing 23 display unit tests passed,
including frame validation and switching the source back to Home.

Reproduce with `python3 test/display-compositor/check.py`; use `--kernel PATH`
for a surrogate kernel stored elsewhere. Evidence is in the ignored
`assets/build/display-compositor/report.json` and `qemu.log`.

This is an offline implementation and oracle result. It does not qualify
physical scanout timing. The display service's source lock, frame parser,
kernel/MI transport and hardware-resource management are unchanged Rust.

Standalone display programs link the same archive using
`tools/display/lean-library.py`. The direct stream probe uses its musl target;
the G13 host display builder uses its Android target. Both executables linked
successfully offline after the change. The Android build used extracted G06
Bionic libraries to check ABI/linkage; this is not G13 hardware qualification.

`TV/Display/Scanout.lean` also controls the OSD transaction sequence: Begin,
Flip, unconditional Apply after a successful Begin, first-frame Show, back-page
selection, Hide and the three alpha controls. The native side only executes
requested ioctls, preserves their errors, and supplies exclusive live handles.
The Lean state is a scalar two-bit word; no shared runtime is entered. Kernel
buffers, creation/destruction, model identity checks and the source-ownership
lock remain native.

The display suite now checks all 64 combinations of initial state and
Begin/Flip/Apply/Show failure, eight Hide cases, eight alpha failure cases,
and invalid states that must perform no device access. The same native
transport adapter is used with recording fake devices on host and ARM32 QEMU.
