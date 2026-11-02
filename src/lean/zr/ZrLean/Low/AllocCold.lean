/-
The allocator's slow paths, part 2 of 3 (AGENTS.md, "The allocator"): the
calls into the firmware heap (a new page, a block of more than 512 bytes and
their release). They are a module of their own so that the C compiler, which
inlines within a file and not across files, keeps them out of line: the fast
paths of `ZrLean.Low.Alloc` then reach them by a tail call and need no stack
frame (the C allocator had these `noinline, cold`).
-/
import ZrLean.Low.AllocCore

namespace ZrLean.Low.Alloc
open ZrLean.Low

/-- A new page for class `r`: its first object. -/
def refill (h r : USize) : BaseIO USize := do
  let page ← memAlloc 4096 4096
  if page == 0 then oom 0
  else
    st32 page r.toUInt32
    stp (page + wordBytes 0) (← ldp (pagesAt h))
    stp (pagesAt h) page
    stp (numPagesAt h) ((← ldp (numPagesAt h)) + 1)
    let c := clsAt h r
    -- The first object at the page data; at least one fits and the carved
    -- end is at most the page.
    stp (c + wordBytes 0) (page + dataOff 0 + r)
    stp (c + 2 * wordBytes 0) (page + carveEnd (dataOff 0) r)
    pure (page + dataOff 0)

/-- A block of more than 512 bytes (rounded) that `heapFree` can release
without being told the size: page-aligned, the header in front. -/
def allocLarge (h r : USize) : BaseIO USize := do
  if !largeBlockOk (dataOff 0) r then oom 0
  else
    let blk ← memAlloc (dataOff 0 + r) 4096
    if blk == 0 then oom 0
    else
      st32 blk r.toUInt32
      stp (blk + wordBytes 0) 0
      countAlloc h r
      stp (liveLargeAt h) ((← ldp (liveLargeAt h)) + 1)
      pure (blk + dataOff 0)

/-- A block of more than 512 bytes (rounded) of `allocObject`: a block of
its own, outside the pages, 16-aligned, so a large byte array's data (after
its 16-byte header on 32-bit) is 16-aligned as SIMD code expects (FFmpeg's
NEON kernels load picture rows with 128-bit alignment hints). -/
def allocBig (h r : USize) : BaseIO USize := do
  countAlloc h r
  let p ← memAlloc r 16
  if p == 0 then oom 0 else pure p

/-- Release a block of more than 512 bytes of `alloc` (the page header is in
front of it). -/
def freeLarge (h p r : USize) : BaseIO Unit := do
  stp (liveLargeAt h) ((← ldp (liveLargeAt h)) - 1)
  memFree (pageOf p) (dataOff 0 + r) 4096

/-- Release a block of more than 512 bytes of `allocObject`. -/
def deallocBig (p r : USize) : BaseIO Unit := memFree p r 16

end ZrLean.Low.Alloc
