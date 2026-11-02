/-
The firmware's heap in ZrLow (AGENTS.md): the blocks of the C runtime
(zr_rt_mem_alloc, zr_rt_mem_free: the Lean heap's pages and big objects, its
tables). First fit over an address-ordered list of free blocks kept in the
free memory itself, as linked_list_allocator (libsupport_zynq's ram.rs)
did; it was C in src/c/zr_platform.c, and these are the same choices, block
for block (test/diff/start_diff.rs checks them against a model of first fit).

The callers give the size and alignment back with every free, so a block
carries no header. Sizes are rounded up to a list node's size (two words:
8 bytes on the board) and alignments to at least that, so every block and
every gap holds a list node. Whatever is left in front of an aligned block
stays free; nothing is ever lost but the rounding. Freeing merges with both
neighbours.

A node is `[next, size]`, two pointer-width words. The head of the list is
one word, `cell`, that the C file reserves (`zr_fw_heap_head`, zero until
`init`): the address of a pointer to a node is either the head or a node's
own `next` field, which is at offset 0, so "the link in front of node f" is
just `f`. No allocation, no boxing: only `USize`s and the ZrLean.Low.Mem
accesses (checked by `zr-cgen check` on the generated C).
-/
import ZrLean.Low.Mem

namespace ZrLean.Low.Heap
open ZrLean.Low

/-- The address of the list head: the C file's one variable. -/
@[never_extract, extern "zr_prim_heap_cell"]
opaque cell (z : USize) : USize

/-- A list node's size, which sizes and alignments are rounded up to. -/
@[never_extract, extern "zr_prim_heap_grain"]
opaque grain (z : USize) : USize

/-- The heap is `[lo, hi)`: one free block, if it holds a node. -/
def init (lo hi : USize) : BaseIO Unit := do
  let lo := roundUp lo (grain 0)
  let hi := hi &&& ~~~((grain 0) - 1)
  if hi > lo && hi - lo >= (grain 0) then
    stp lo 0
    stp (lo + (wordBytes 0)) (hi - lo)
    stp (cell 0) lo
  else
    stp (cell 0) 0

/-- Walk `link` to the last node below `lo` (or the head). -/
partial def findLink (link lo : USize) : BaseIO USize := do
  let n ← ldp link
  if n != 0 && n < lo then findLink n lo else pure link

/-- First fit from the link `link`: the block, or 0. -/
partial def search (link size align : USize) : BaseIO USize := do
  let f ← ldp link
  if f == 0 then return 0
  let fsize ← ldp (f + (wordBytes 0))
  let hi := f + fsize
  let p := roundUp f align
  if p < f || p > hi || size > hi - p then
    search f size align
  else
    let next ← ldp f
    let e := p + size
    let next ← if e < hi then do
        stp e next
        stp (e + (wordBytes 0)) (hi - e)
        pure e
      else pure next
    if p > f then
      stp (f + (wordBytes 0)) (p - f)
      stp f next
    else
      stp link next
    return p

/-- `size` bytes aligned to `align` (a power of two, raised to a node's
size), or 0: no fit, a bad alignment or a size that wraps. -/
def alloc (size align : USize) : BaseIO USize := do
  let align := if align < (grain 0) then (grain 0) else align
  if align &&& (align - 1) != 0 then return 0
  let size := if size == 0 then 1 else size
  if size > (0 - 1) - (grain 0) then return 0
  search (cell 0) (roundUp size (grain 0)) align

/-- Return the block `[ptr, ptr + size)` of `alloc size _`, merged with the
free blocks next to it. -/
def free (ptr size : USize) : BaseIO Unit := do
  if ptr == 0 then return
  let size := roundUp (if size == 0 then 1 else size) (grain 0)
  let link ← findLink (cell 0) ptr
  let nxt ← ldp link
  stp (ptr + (wordBytes 0)) size
  stp ptr nxt
  -- With the next block.
  if nxt != 0 && ptr + size == nxt then
    let nsize ← ldp (nxt + (wordBytes 0))
    let nn ← ldp nxt
    stp (ptr + (wordBytes 0)) (size + nsize)
    stp ptr nn
  stp link ptr
  -- With the previous one: `link` is its `next` field, unless ptr is first.
  if link != (cell 0) then
    let psize ← ldp (link + (wordBytes 0))
    if link + psize == ptr then
      let bsize ← ldp (ptr + (wordBytes 0))
      let bn ← ldp ptr
      stp (link + (wordBytes 0)) (psize + bsize)
      stp link bn

/-! The C symbols. -/

@[export zr_fw_heap_init]
def initFfi (lo hi : USize) : BaseIO UInt8 := do init lo hi; pure 0

@[export zr_fw_heap_alloc]
def allocFfi (size align : USize) : BaseIO USize := alloc size align

@[export zr_fw_heap_free]
def freeFfi (ptr size _align : USize) : BaseIO UInt8 := do free ptr size; pure 0

end ZrLean.Low.Heap
