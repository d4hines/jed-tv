/-
Size-class allocator arithmetic, layout and terminal allocation boundaries,
shared by `AllocCold` and `Alloc`. Small objects (up to 512 bytes) use
8-byte classes carved from 4 KiB pages; larger blocks use separate
allocations. Page headers record object size and free-list linkage.

The heap state stores 64 class records, page links and allocation/live/peak
counters. `ZrAbi`, generated terminal providers and `ZrLean.Low.Heap` supply
the storage and platform boundary. See `AGENTS.md` and allocator tests
under `test/leanrt`. `zr-cgen check` checks generated scalar code.
-/
import ZrLean.Low.Mem

namespace ZrLean.Low.Alloc
open ZrLean.Low

/-- The runtime's heap: the C file's one array. -/
@[never_extract, extern "zr_prim_alloc_heap"]
opaque theHeap (z : USize) : USize

/-- The firmware heap (`zr_rt_mem_alloc`: the C forwarder to
`ZrLean.Low.Heap` on the board, the test platform's allocator on the host):
`size` bytes aligned to `align`, or 0. -/
@[extern "zr_prim_alloc_mem_alloc"]
opaque memAlloc (size align : USize) : BaseIO USize

/-- Return a block of `memAlloc size align`. -/
@[extern "zr_prim_alloc_mem_free"]
opaque memFree (p size align : USize) : BaseIO Unit

/-- Out of memory: the runtime's fatal error, which does not return. -/
@[extern "zr_prim_alloc_oom"]
opaque oom (z : USize) : BaseIO USize

/-! ### Allocation arithmetic -/

/-- A request rounded up to a multiple of 8 (0 counts as 1); 0 if that is not
representable. -/
@[inline] def round (size : USize) : USize :=
  let s := if size == 0 then 1 else size
  -- `s + 7` wraps exactly when `s > SIZE_MAX - 7` (written so: a constant
  -- `0 - 1` would be a closed term, AGENTS.md).
  let t := s + 7
  if t < s then 0 else t / 8 * 8

/-- Served from a class free list. -/
@[inline] def isSmall (r : USize) : Bool := r ≤ 512

/-- Class index of a rounded small size; always below 64. -/
@[inline] def classOf (r : USize) : USize := ((r - 8) / 8) % 64

/-- The page header (two words) rounded up to 8: where a page's objects start. -/
@[inline] def pageData (hdr : USize) : USize := (hdr + 7) / 8 * 8

/-- Objects of `size` bytes that fit in a page after `pd` bytes. -/
@[inline] def objects (pd size : USize) : USize :=
  if size == 0 || pd ≥ 4096 then 0 else (4096 - pd) / size

/-- End offset of the carved part of a page. -/
@[inline] def carveEnd (pd size : USize) : USize := pd + objects pd size * size

/-- Is there room for `size` in the free tail `[b, e)`? -/
@[inline] def bumpOk (b e size : USize) : Bool := e ≥ b && e - b ≥ size

/-- Does a large unsized block fit: the header and then the object (`pd +
size` does not wrap) with the object size in the header's u32? -/
@[inline] def largeBlockOk (pd size : USize) : Bool :=
  !(size > 0xFFFFFFFF || pd + size < size)

/-- The page header's size: `{u32; pointer}`. -/
@[inline] def hdrSize (z : USize) : USize := 2 * wordBytes z

/-- Where a page's objects start. -/
@[inline] def dataOff (z : USize) : USize := pageData (hdrSize z)

/-- The page (header) of a block. -/
@[inline] def pageOf (p : USize) : USize := p / 4096 * 4096

/-! ### The state -/

/-- Class `r`'s (free, bump, end) triple. -/
@[inline] def clsAt (h r : USize) : USize := h + classOf r * (3 * wordBytes 0)
@[inline] def pagesAt (h : USize) : USize := h + 192 * wordBytes 0
@[inline] def numPagesAt (h : USize) : USize := h + 193 * wordBytes 0
@[inline] def liveAt (h : USize) : USize := h + 194 * wordBytes 0
@[inline] def liveLargeAt (h : USize) : USize := h + 195 * wordBytes 0
@[inline] def peakAt (h : USize) : USize := h + 196 * wordBytes 0
@[inline] def allocsAt (h : USize) : USize := h + 197 * wordBytes 0

/-- The state's size in bytes. -/
@[inline] def heapBytes (z : USize) : USize := 198 * wordBytes z

/-- A block of `r` bytes (rounded) is handed out. -/
@[inline] def countAlloc (h r : USize) : BaseIO Unit := do
  let live := (← ldp (liveAt h)) + r
  stp (liveAt h) live
  stp (allocsAt h) ((← ldp (allocsAt h)) + 1)
  if live > (← ldp (peakAt h)) then stp (peakAt h) live

end ZrLean.Low.Alloc
