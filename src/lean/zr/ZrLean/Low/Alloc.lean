/-
The Lean runtime's object allocator in ZrLow, part 3 of 3 (AGENTS.md,
"The allocator"): the fast paths (pop a free list, bump through the newest
page, push on free), the heaps of their own for the tests, and the C
symbols. The arithmetic and the layout are in `ZrLean.Low.AllocCore`, the
calls into the firmware heap in `ZrLean.Low.AllocCold`.
-/
import ZrLean.Low.AllocCold

namespace ZrLean.Low.Alloc
open ZrLean.Low

/-- A small block of `r` bytes (rounded, at most 512). -/
@[inline] def allocSmall (h r : USize) : BaseIO USize := do
  countAlloc h r
  let c := clsAt h r
  let p ← ldp c
  if p != 0 then
    stp c (← ldp p)
    pure p
  else
    let b ← ldp (c + wordBytes 0)
    let e ← ldp (c + 2 * wordBytes 0)
    if bumpOk b e r then
      stp (c + wordBytes 0) (b + r)
      pure b
    else refill h r

/-- A block of `size` bytes (rounded up to 8), 8-aligned, that `heapFree`
releases without being told the size. -/
@[inline] def alloc (h size : USize) : BaseIO USize := do
  let r := round size
  if r == 0 then oom 0
  else if isSmall r then allocSmall h r
  else allocLarge h r

/-- A block whose size the caller gives back (`dealloc`): arrays, strings,
closures. -/
@[inline] def allocObject (h size : USize) : BaseIO USize := do
  let r := round size
  if r == 0 then oom 0
  else if isSmall r then allocSmall h r
  else allocBig h r

/-- Free a live block whose rounded size is `r`. -/
@[inline] def freeKnown (h p r : USize) : BaseIO Unit := do
  stp (liveAt h) ((← ldp (liveAt h)) - r)
  if isSmall r then
    let c := clsAt h r
    stp p (← ldp c)
    stp c p
  else freeLarge h p r

/-- The rounded size of a live block of `alloc`. -/
@[inline] def sizeOf (p : USize) : BaseIO USize := do
  pure (← ld32 (pageOf p)).toUSize

/-- Free a live block of `alloc`. -/
@[inline] def free (h p : USize) : BaseIO Unit := do
  freeKnown h p (← sizeOf p)

/-- Free a block of `allocObject h size`. -/
@[inline] def dealloc (h p size : USize) : BaseIO Unit := do
  let r := round size
  if r == 0 then
    let _ ← oom 0
    pure ()
  else if isSmall r then freeKnown h p r
  else
    stp (liveAt h) ((← ldp (liveAt h)) - r)
    deallocBig p r

/-- Statistics of `h` (0: the runtime's): 0 live bytes, 1 peak live bytes, 2
blocks handed out so far, 3 pages, 4 live large blocks. -/
def stat (h : USize) (k : UInt8) : BaseIO USize := do
  let h := if h == 0 then theHeap 0 else h
  if k == 0 then ldp (liveAt h)
  else if k == 1 then ldp (peakAt h)
  else if k == 2 then ldp (allocsAt h)
  else if k == 3 then ldp (numPagesAt h)
  else if k == 4 then ldp (liveLargeAt h)
  else pure 0

/-! ### Heaps of their own, for the tests -/

partial def zero (a e : USize) : BaseIO Unit := do
  if a < e then
    stp a 0
    zero (a + wordBytes 0) e

/-- A new, empty heap. -/
def heapNew (_z : USize) : BaseIO USize := do
  let h ← memAlloc (heapBytes 0) (wordBytes 0)
  if h == 0 then oom 0
  else
    zero h (h + heapBytes 0)
    pure h

partial def freePages (p : USize) : BaseIO Unit := do
  if p != 0 then
    let next ← ldp (p + wordBytes 0)
    memFree p 4096 4096
    freePages next

/-- Release a heap's pages and the heap. -/
def heapDelete (h : USize) : BaseIO Unit := do
  freePages (← ldp (pagesAt h))
  memFree h (heapBytes 0) (wordBytes 0)

/-! ### The C symbols -/

/-- `lean_alloc_small`'s block (the C forwarder casts). -/
@[export zr_low_alloc_small]
def allocSmallFfi (sz : USize) : BaseIO USize := alloc (theHeap 0) sz

/-- `lean_alloc_object`'s block. -/
@[export zr_low_alloc_object]
def allocObjectFfi (sz : USize) : BaseIO USize := allocObject (theHeap 0) sz

/-- Free a block of `lean_alloc_small` (the size is in its page). -/
@[export zr_low_free_small]
def freeSmallFfi (p : USize) : BaseIO UInt8 := do free (theHeap 0) p; pure 0

/-- Free a block of `lean_alloc_object` of `sz` bytes. -/
@[export zr_low_dealloc_sized]
def deallocFfi (p sz : USize) : BaseIO UInt8 := do dealloc (theHeap 0) p sz; pure 0

/-- The heartbeat's counters (`ZrLean.Fw.heapStat`): live bytes, peak, blocks. -/
@[export zr_rt_heap_stat]
def heapStatFfi (k : UInt8) : BaseIO USize :=
  if k < 3 then stat 0 k else pure 0

/-! The test API: heaps of their own, and the arithmetic. -/

@[export zr_low_heap_stat]
def statFfi (h : USize) (k : UInt8) : BaseIO USize := stat h k
@[export zr_low_heap_new]
def heapNewFfi (z : USize) : BaseIO USize := heapNew z
@[export zr_low_heap_delete]
def heapDeleteFfi (h : USize) : BaseIO UInt8 := do heapDelete h; pure 0
@[export zr_low_heap_alloc]
def heapAllocFfi (h size : USize) : BaseIO USize := alloc h size
@[export zr_low_heap_free]
def heapFreeFfi (h p : USize) : BaseIO UInt8 := do free h p; pure 0
@[export zr_low_heap_alloc_object]
def heapAllocObjectFfi (h size : USize) : BaseIO USize := allocObject h size
@[export zr_low_heap_dealloc]
def heapDeallocFfi (h p size : USize) : BaseIO UInt8 := do dealloc h p size; pure 0
@[export zr_low_heap_size_of]
def sizeOfFfi (p : USize) : BaseIO USize := sizeOf p
@[export zr_low_heap_bytes]
def heapBytesFfi (z : USize) : BaseIO USize := pure (heapBytes z)
@[export zr_low_heap_round]
def roundFfi (size : USize) : BaseIO USize := pure (round size)
@[export zr_low_heap_is_small]
def isSmallFfi (r : USize) : BaseIO UInt8 := pure (if isSmall r then 1 else 0)
@[export zr_low_heap_class_of]
def classOfFfi (r : USize) : BaseIO USize := pure (classOf r)
@[export zr_low_heap_page_data]
def pageDataFfi (hdr : USize) : BaseIO USize := pure (pageData hdr)
@[export zr_low_heap_objects]
def objectsFfi (pd size : USize) : BaseIO USize := pure (objects pd size)
@[export zr_low_heap_carve_end]
def carveEndFfi (pd size : USize) : BaseIO USize := pure (carveEnd pd size)
@[export zr_low_heap_bump_ok]
def bumpOkFfi (b e size : USize) : BaseIO UInt8 := pure (if bumpOk b e size then 1 else 0)
@[export zr_low_heap_large_block_ok]
def largeBlockOkFfi (pd size : USize) : BaseIO UInt8 := pure (if largeBlockOk pd size then 1 else 0)

end ZrLean.Low.Alloc
