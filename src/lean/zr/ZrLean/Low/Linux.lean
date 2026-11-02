/-
The runtime's platform on Linux userspace (32-bit ARM), in ZrLow style: the
heap's memory (the first-fit heap of ZrLean.Low.Heap over mmap regions,
grown on demand), the fatal stop and the panic log. The only way to the
kernel is `zr_sys6` (ZrAsm.Linux, `svc #0`). runtime.S binds the names
lean.h calls (`zr_rt_mem_alloc`, ...) to these exports. Single-threaded.
-/
import ZrLean.Low.Heap
import ZrLean.Low.Obj

set_option compiler.extract_closed false

namespace ZrLean.Low.Linux
open ZrLean.Low

@[extern "zr_sys6"]
opaque sys6 (nr a0 a1 a2 a3 a4 a5 : USize) : BaseIO USize

/-- Four words of storage (ZrAsm.Linux): word 0 is 1 once the heap has
memory; word 1 holds a newline byte for the logs. -/
@[never_extract, extern "zr_linux_state_addr"]
opaque state (z : USize) : USize

@[inline] def isErr (r : USize) : Bool := r > Obj.sizeMax 0 - 4096

/-- Address space taken from the kernel at a time (committed on first touch). -/
@[inline] def chunkBytes (z : USize) : USize := (z ||| 64) <<< 20

/-- A private anonymous mapping of `bytes` (mmap2, MAP_NORESERVE), or 0. -/
def mapRegion (bytes : USize) : BaseIO USize := do
  let r ← sys6 192 0 bytes 3 0x4022 (Obj.sizeMax 0) 0
  pure (if isErr r then 0 else r)

/-- `zr_rt_mem_alloc`: `size` bytes aligned to `align`, or 0. -/
@[export zr_low_linux_mem_alloc]
def memAlloc (size align : USize) : BaseIO USize := do
  let st := state 0
  if (← ldp st) == 0 then
    let base ← mapRegion (chunkBytes 0)
    if base == 0 then return 0
    Heap.init base (base + chunkBytes 0)
    stp st 1
  let p ← Heap.alloc size align
  if p != 0 then return p
  -- grow: a new region that fits this block, handed to the free list
  let want := roundUp (size + align + 4096) 4096
  let bytes := if want > chunkBytes 0 then want else chunkBytes 0
  let base ← mapRegion bytes
  if base == 0 then return 0
  Heap.free base bytes
  Heap.alloc size align

/-- `zr_rt_mem_free`. -/
@[export zr_low_linux_mem_free]
def memFree (p size _align : USize) : BaseIO UInt8 := do
  Heap.free p size
  pure 0

def write2 (p n : USize) : BaseIO Unit := do
  let _ ← sys6 4 2 p n 0 0 0
  pure ()

def newline (_z : USize) : BaseIO Unit := do
  let cell := state 0 + wordBytes 0
  st8 cell 10
  write2 cell 1

/-- `zr_rt_report_panic`: the message on stderr, newline-terminated. -/
@[export zr_low_linux_report_panic]
def reportPanic (msg len : USize) : BaseIO UInt8 := do
  write2 msg len
  if len == 0 || (← ld8 (msg + len - 1)) != 10 then newline 0
  pure 0

/-- `zr_rt_fatal`: the message on stderr, then exit_group(134). -/
@[export zr_low_linux_fatal]
def fatal (msg len : USize) : BaseIO USize := do
  write2 msg len
  newline 0
  sys6 248 134 0 0 0 0 0

end ZrLean.Low.Linux
