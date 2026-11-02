/-
Reference counting's cold paths, part 2 of 4 (AGENTS.md, "The runtime's
objects"): `lean_inc_ref_huge_n` (a count raised by more than `LEAN_RC_INC_MAX`
at once: only `Array.replicate` does) and `lean_mark_persistent` (closed
terms, at startup). A module of their own so the C compiler keeps them out of
the hot path's code, as the allocator's slow paths are.
-/
import ZrLean.Low.Obj

namespace ZrLean.Low.RcCold
open ZrLean.Low ZrLean.Low.Alloc ZrLean.Low.Obj


/-- Raise the count of `o` by `n` thread-shared steps (counts above
`LEAN_RC_STICKY`, 0x90000000 as unsigned, are the ones increments still adjust;
`LEAN_RC_INC_MAX` is 0x10000). -/
partial def incMt (o n : USize) : BaseIO Unit := do
  let rc ← ld32 o
  if n != 0 && rc > 0x90000000 then
    let chunk := if n < 0x10000 then n else 0x10000
    st32 o (rc - chunk.toUInt32)
    incMt o (n - chunk)

/-- `lean_inc_ref_huge_n`: apply all of `n` to the count of `o`, or freeze
it (`LEAN_RC_STUCK_ST`, `INT_MIN + LEAN_RC_INC_MAX`). -/
def incRefHugeN (o n : USize) : BaseIO Unit := do
  let rc ← ld32 o
  if sgt32 rc 0 then
    -- n > INT_MAX - rc (rc positive)
    if n > (0x7fffffff - rc).toUSize then st32 o 0x80010000
    else st32 o (rc + n.toUInt32)
  else incMt o n

/-! ### `lean_mark_persistent` -/

/-- The capacity a work list needs to have room for `k` more words after
`n`: `cap` doubled until it does (the out-of-memory stop when that would wrap). -/
partial def grow (cap n k : USize) : BaseIO USize := do
  if k > cap - n then
    if cap > sizeMax 0 / 2 / wordBytes 0 then fatal 0 else grow (cap * 2) n k
  else pure cap

/-- The C file's first work list (`zr_mark_stack`, 64 words), as the C kept it
on the stack: the common case needs no allocation. -/
@[never_extract, extern "zr_prim_rccold_mark_stack"]
opaque markStack (z : USize) : USize

/-- Give back a work list that is not the first one. -/
@[inline] def releaseList (stack cap : USize) : BaseIO Unit := do
  if stack != markStack 0 then memFree stack (cap * wordBytes 0) (wordBytes 0)

mutual
/-- Pop and mark until the list is empty, then release it. -/
partial def markLoop (stack cap n : USize) : BaseIO Unit := do
  if n == 0 then
    releaseList stack cap
  else
    let n := n - 1
    let o ← ldp (stack + n * wordBytes 0)
    if o &&& 1 != 0 then markLoop stack cap n
    else if (← ld32 o) == 0 then markLoop stack cap n
    else
      st32 o 0
      let tag ← tagOf o
      if tag ≤ 243 then
        pushKids stack cap n (o + ctorObjs 0) (← otherOf o).toUSize
      else if tag == 245 then
        pushKids stack cap n (o + cloObjs 0) (← ld16 (o + cloFixedOff 0)).toUSize
      else if tag == 246 then
        pushKids stack cap n (o + arrData 0) (← ldp (o + arrSizeOff 0))
      -- A persistent reference remains mutable. Its current value retains
      -- ordinary ownership and must not itself become immortal.
      else if tag == 248 || tag == 249 || tag == 250 || tag == 253 then
        markLoop stack cap n
      else
        let _ ← fatal 2
        pure ()

/-- Make room for the `k` children at `base` and push them. -/
partial def pushKids (stack cap n base k : USize) : BaseIO Unit := do
  let ncap ← grow cap n k
  if ncap != cap then
    let bigger ← memAlloc (ncap * wordBytes 0) (wordBytes 0)
    if bigger == 0 then
      let _ ← fatal 0
      pure ()
    else
      copy bigger stack (n * wordBytes 0)
      releaseList stack cap
      copyKids bigger ncap n base 0 k
  else copyKids stack cap n base 0 k

/-- Push `base[i..k]` and carry on with the list. -/
partial def copyKids (stack cap n base i k : USize) : BaseIO Unit := do
  if i < k then
    stp (stack + n * wordBytes 0) (← ldp (base + i * wordBytes 0))
    copyKids stack cap (n + 1) base (i + 1) k
  else markLoop stack cap n
end

/-- `lean_mark_persistent`: make `o` and its immutable descendants persistent
(count 0). A mutable reference retains ordinary ownership of its contents.
The work list starts in the C file's 64 words and moves
to blocks of the firmware heap when it outgrows them. -/
def markPersistent (o : USize) : BaseIO Unit := do
  stp (markStack 0) o
  markLoop (markStack 0) 64 1

@[export zr_low_inc_ref_huge_n]
def incRefHugeNFfi (o n : USize) : BaseIO UInt8 := do incRefHugeN o n; pure 0

@[export zr_low_mark_persistent]
def markPersistentFfi (o : USize) : BaseIO UInt8 := do markPersistent o; pure 0

end ZrLean.Low.RcCold
