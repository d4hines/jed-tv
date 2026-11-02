# Gpu: machine-checked facts about GPU pipeline operations

A small Lean 4 library (core Lean only, no Mathlib, no Batteries) with
proofs about four operations every GPU pipeline relies on:

| Topic | Module | What is proved |
|---|---|---|
| A. Depth test | `Gpu.Depth` | Drawing order does not matter: each pixel ends up with a nearest fragment. |
| B. Fill rule | `Gpu.Raster` | With the top-left rule, two triangles that share an edge draw a pixel centre on that edge exactly once. |
| C. Pixel formats | `Gpu.Pixel` | Packing `0xAARRGGBB` and unpacking are inverses; panel layouts round-trip; 8-bit blending stays in range. |
| D. Frame handoff | `Gpu.Handoff` | A sequence-lock reader that passes its counter check never sees a torn frame, and sees the newest complete one. |

Nothing here is specific to this TV.  The models are small executable
Lean definitions; the theorems are about those definitions.

## Build

```sh
cd proofs/gpu
lake build          # Lean 4.35.0, see lean-toolchain
```

Each topic has a `Spec.lean` (definitions, `#guard` unit tests) and a
`Proofs.lean` (theorems, concrete instances, negative examples).
`Gpu/Audit.lean` runs `#print axioms` and `#spec_surface` on every main
theorem; the build log shows the result.  A clean build takes a few
seconds.  There are no `sorry`, `axiom`, `native_decide`, `bv_decide`,
`implemented_by` or `unsafe` anywhere; the only axioms that appear are
`propext`, `Classical.choice` and `Quot.sound`.

## A. Depth test (`Gpu/Depth/`)

A fragment is (pixel index, depth, colour).  A framebuffer maps each
pixel to `none` or to a stored (depth, colour).  `writeFragment` stores
a fragment only if its pixel is empty or the fragment is strictly nearer
(smaller depth).  `draw` applies `writeFragment` to a list of fragments
in order (a left fold).

* **`draw_nearestWins` (A1).** After drawing any list onto an empty
  framebuffer, every pixel is empty exactly when no fragment targets it,
  and otherwise holds the depth and colour of some fragment that targets
  it, with no fragment targeting that pixel being nearer.
* **`draw_depth_perm` (A2, depth).** Drawing the same fragments in any
  other order (`List.Perm`) gives the same depth at every pixel, from
  any starting framebuffer.
* **`draw_perm_of_noDepthTies` (A2, colour).** If no two fragments at
  the same pixel share a depth (`NoDepthTies`), any order gives exactly
  the same framebuffer, colours included.
* **Negative examples.** `tie_colour_depends_on_order`: with two
  fragments at pixel 0, both at depth 5, swapping them changes the
  colour (so the tie rule in A2 is needed).  `farWon_not_nearestWins`:
  a framebuffer where the farther of two fragments won fails
  `NearestWins`.

What it does *not* say: nothing about depth precision, depth ranges,
`<=` vs `<` conventions other than the one defined, or what a real GPU
does with ties (the model uses "first drawn wins ties").

## B. Watertight rasterization, the top-left rule (`Gpu/Raster/`)

Coordinates are integers in *doubled* screen units: pixel `(i, j)` has
centre `(2i+1, 2j+1)`; vertices can be on any half-pixel position.
The screen is y-down.  The edge function of edge `a → b` at `p` is the
cross product `(b - a) × (p - a)`.  Triangles are listed so that all
three edge functions are positive at the opposite vertex
(`IsPositiveWinding`; counter-clockwise in a y-up maths frame).  A point
passes an edge test if its edge function is positive, or is zero and
the edge is a *top* edge (horizontal, pointing right) or a *left* edge
(pointing up the screen).  `covers t p` is the conjunction of the three
edge tests.

* **`edgePasses_shared_exactly_once` (B1, one edge).** If `p` is on the
  line of edge `a → b` and `a ≠ b`, then `p` passes the test for `b → a`
  exactly when it fails the test for `a → b`.
* **`covers_shared_exactly_once` (B1).** Triangles `⟨a, b, c⟩` and
  `⟨b, a, d⟩` share the edge between `a` and `b`.  If `p` is strictly
  inside that edge (`OnOpenSegment a b p`) and passes the other two edge
  tests of both triangles (an explicit hypothesis), then exactly one of
  the two triangles covers `p`.
* **`covers_shared_no_gap` (B2).** Under the same hypotheses, it is not
  the case that neither triangle covers `p`.
* **Negative example.** `naive_covers_twice`: with the naive rule
  (`≥ 0` everywhere, no tie-break), the concrete pixel centre `(5, 1)`
  on the edge shared by `tri1` and `tri2` is covered by both.
  `degenerate_edge_gap`: with `a = b` neither direction passes, which is
  why B1 needs `a ≠ b`.

What it does *not* say: that a point strictly inside a shared edge
passes the other four edge tests (that is assumed, and shown on a
concrete example); anything about vertices, points outside the shared
edge, sub-pixel precision beyond halves, or fill convention for
degenerate triangles.

## C. Pixel formats and blending (`Gpu/Pixel/`)

`pack` puts 8-bit r, g, b, a into a `UInt32` as `0xAARRGGBB`;
`unpack` reads them back.  A `PanelLayout` gives the bit offset of each
colour channel in a panel word; `rgbLayout = ⟨16, 8, 0⟩` and
`bgrLayout = ⟨0, 8, 16⟩`.  `blendNat src dst a =
(src * a + dst * (255 - a) + 127) / 255` is one channel of source-over
blending; `blendChannel` is the same on `UInt8`.

* **`unpack_pack`, `pack_unpack` (C1).** `unpack (pack c) = c` for every
  colour and `pack (unpack v) = v` for every 32-bit word.
* **`fromPanel_toPanel_rgb`, `fromPanel_toPanel_bgr` (C1).** Writing a
  colour into a panel word and reading it back gives the colour, for
  both layouts.  `fromPanel_packedToPanel` says the same starting from a
  packed `0xAARRGGBB` word.
* **`blendNat_le_255` (C2).** If `src`, `dst`, `a ≤ 255` then the blend
  is `≤ 255`; `blendChannel_toNat` then says the 8-bit version never
  wraps.
* **`blendNat_alpha_full`, `blendNat_alpha_zero` (C2).** Alpha 255 gives
  the source, alpha 0 gives the destination (also on `UInt8`:
  `blendChannel_alpha_full`, `blendChannel_alpha_zero`, and for whole
  colours: `blendOver_opaque`, `blendOver_transparent`).
* **Negative example.** `bgr_read_as_rgb_swaps`: a colour written for a
  BGR panel but read with the RGB layout comes back with red and blue
  swapped.

The bit-level proofs do not use `bv_decide` (which would add a
native-computation axiom).  Instead, shifts and ORs of non-overlapping
bytes are rewritten to multiplication and addition, and `omega` does
the arithmetic.

What it does *not* say: anything about premultiplied alpha, gamma,
rounding modes other than `+127`, or 10-bit and 16-bit formats.

## D. Renderer-to-presenter frame handoff, a sequence lock (`Gpu/Handoff/`)

**The model.**  A shared record has a counter `seq` and fields
(a function from field index to value; only indices below the field
count `n` matter).  The writer publishes each frame of its script by:
`seq + 1` (odd: write in progress), write fields `0 .. n-1` one at a
time, `seq + 1` (even: frame complete).  The reader does: read `seq`
as `c1`, read fields `0 .. n-1` one at a time, read `seq` as `c2`;
`accepts` iff `c1` is even, `c1 ≠ 0`, and `c1 = c2`.  A schedule is a
list of `Actor`s; `run` executes one step of the named side per entry,
in any interleaving.  `frameIndex c = c / 2 - 1` is the script entry an
even counter value names.

**The one trusted assumption.**  Each step reads or writes one counter
or field value atomically, and all steps happen in a single global
order (sequential consistency).  This stands in for the acquire/release
atomics of real code.  Weak memory orderings are not modelled.

* **`no_torn_frames` (D1).** For every field count, script and
  schedule: if the reader finishes with `(c1, got, c2)` and `accepts`,
  then `frameIndex c1` is below the number of frames the writer has
  completed, and `got` is exactly the `n` fields of script entry
  `frameIndex c1`.  No mixing of fields from different frames.
* **`freshness` (D2).** Split the schedule as `before ++ [reader] ++
  after` where the reader has not started after `before` (so the next
  step is its first counter read).  If it later accepts, `c1` is twice
  the number of frames completed at that moment, and the frame it read
  is the newest completed one at that moment.
* **`counter_identifies_frame` (D2).** Two accepted counter values that
  name the same frame are equal.
* **Negative examples** on `tornSchedule` (writer publishes frame 1;
  reader reads `c1 = 2` and field 0; writer starts frame 2 and writes
  both fields; reader reads field 1 and `c2 = 3`): `torn_is_mixed` (the
  fields read are `[10, 21]`, not any frame), `torn_rejected`
  (`accepts` is false), `torn_accepted_by_naive` (a check without the
  second counter read, `acceptsNaive`, would accept it).
  `mid_write_rejected`: a read that starts mid-write sees an odd `c1`
  and is rejected.

What it does *not* say: anything about liveness (a reader might retry
forever if the writer is faster), about more than one reader or writer,
about the real Rust code, or about memory orderings weaker than
sequential consistency.

## What is trusted

A theorem means what its statement says, and the statement means what
its definitions mean.  `#spec_surface` in `Gpu/Audit.lean` prints the
list below from the Lean environment (generated names such as
constructors and projections are left out).

* **`draw_nearestWins`, `draw_depth_perm`, `draw_perm_of_noDepthTies`**
  depend on `Fragment`, `Sample`, `Framebuffer`, `Framebuffer.empty`,
  `passesDepthTest`, `stepPixel`, `writeFragment`, `draw`, and
  respectively `NearestWins`, `depthAt`, `NoDepthTies` (plus core
  `List.Perm`, `List.foldl`, `List.Pairwise`).  Read `passesDepthTest`
  and `NearestWins` most carefully: they say what "nearest" means.
* **`edgePasses_shared_exactly_once`** depends on `Point`, `edgeFn`,
  `isTopEdge`, `isLeftEdge`, `isTopLeft`, `edgePasses`.
  **`covers_shared_exactly_once`, `covers_shared_no_gap`** add
  `Triangle`, `covers`, `OnOpenSegment`.  The convention (which edges
  are "top" and "left", and the winding) is fixed by `isTopEdge`,
  `isLeftEdge` and `IsPositiveWinding`; a different convention is a
  different theorem.
* **`unpack_pack`, `pack_unpack`** depend on `Rgba`, `pack`, `unpack`.
  **`fromPanel_toPanel_rgb/bgr`** depend on `Rgb`, `PanelLayout`,
  `rgbLayout`, `bgrLayout`, `toPanel`, `fromPanel`.
  **`blendNat_*`** depend on `blendNat` only; **`blendChannel_*`** add
  `blendChannel`.  All use core `UInt8`/`UInt32` operations.
* **`no_torn_frames`, `freshness`** depend on `Frame`, `zeroFrame`,
  `fieldsOf`, `Shared`, `WriterPhase`, `Writer`, `ReaderPhase`,
  `System`, `init`, `writerStep`, `readerStep`, `Actor`, `step`, `run`,
  `accepts`, `frameIndex`.  `writerStep` and `readerStep` *are* the
  protocol; `run` is the interleaving model and carries the
  sequential-consistency assumption.  `counter_identifies_frame`
  depends on `frameIndex` only.
* **Accepted axioms:** `propext`, `Classical.choice`, `Quot.sound`
  (standard Lean).  No others.
* **Tooling:** Lean 4.35.0 core (kernel, `omega`, `decide`, `simp`).

## Limits

* Integer-only.  No real numbers, no perspective division, no
  sub-pixel precision finer than half a pixel in B.
* These are models written in Lean, not the Rust or shader code that
  runs on the TV.  Nothing links them to that code.  They are facts
  about the operations, usable as a reference for what the code should
  do.
* A and B say nothing about performance or about how fragments are
  generated; B assumes the point passes the non-shared edge tests.
* C is about one packing and two panel layouts; other layouts need
  their own (short) proofs.  `fromPanel_packedToPanel` is stated for
  exactly those two layouts.
* D models one writer, one reader, one read attempt, sequential
  consistency, and a writer that only moves forward through a fixed
  script.  Retries, several readers, and weak memory are out of scope.
