import ZrLean.Low.Arr

/-! Closure application below the runtime: all objects are addresses. The
bounded argument scratch is stack memory owned by this invocation, so callbacks
may recursively apply more closures without sharing any work buffers. -/
namespace ZrLean.Low.Apply
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc

@[never_extract, extern "zr_prim_apply_fun_off"]
opaque funOff (z : USize) : USize
@[never_extract, extern "zr_prim_apply_arity_off"]
opaque arityOff (z : USize) : USize
@[never_extract, extern "zr_prim_apply_closure_header"]
opaque closureHeader (z : USize) : UInt32

/-- One compiler stack allocation, inlined into the frame that uses it. -/
@[extern "zr_prim_apply_stack"]
opaque stack (bytes : USize) : BaseIO USize

@[extern "zr_prim_apply_invoke1"]
opaque invoke1 (fn a1 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke2"]
opaque invoke2 (fn a1 a2 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke3"]
opaque invoke3 (fn a1 a2 a3 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke4"]
opaque invoke4 (fn a1 a2 a3 a4 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke5"]
opaque invoke5 (fn a1 a2 a3 a4 a5 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke6"]
opaque invoke6 (fn a1 a2 a3 a4 a5 a6 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke7"]
opaque invoke7 (fn a1 a2 a3 a4 a5 a6 a7 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke8"]
opaque invoke8 (fn a1 a2 a3 a4 a5 a6 a7 a8 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke9"]
opaque invoke9 (fn a1 a2 a3 a4 a5 a6 a7 a8 a9 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke10"]
opaque invoke10 (fn a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke11"]
opaque invoke11 (fn a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke12"]
opaque invoke12 (fn a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke13"]
opaque invoke13 (fn a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke14"]
opaque invoke14 (fn a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke15"]
opaque invoke15 (fn a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 : USize) : BaseIO USize
@[extern "zr_prim_apply_invoke16"]
opaque invoke16 (fn a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 : USize) : BaseIO USize

@[extern "zr_prim_apply_invoke_array"]
opaque invokeArray (fn args : USize) : BaseIO USize

def call (fn n args : USize) : BaseIO USize := do
  if n == 0 then fatal 8
  else if n == 1 then invoke1 fn (← ldp (args + 0 * wordBytes 0))
  else if n == 2 then invoke2 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0))
  else if n == 3 then invoke3 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0))
  else if n == 4 then invoke4 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0))
  else if n == 5 then invoke5 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0))
  else if n == 6 then invoke6 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0))
  else if n == 7 then invoke7 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0))
  else if n == 8 then invoke8 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0)) (← ldp (args + 7 * wordBytes 0))
  else if n == 9 then invoke9 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0)) (← ldp (args + 7 * wordBytes 0)) (← ldp (args + 8 * wordBytes 0))
  else if n == 10 then invoke10 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0)) (← ldp (args + 7 * wordBytes 0)) (← ldp (args + 8 * wordBytes 0)) (← ldp (args + 9 * wordBytes 0))
  else if n == 11 then invoke11 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0)) (← ldp (args + 7 * wordBytes 0)) (← ldp (args + 8 * wordBytes 0)) (← ldp (args + 9 * wordBytes 0)) (← ldp (args + 10 * wordBytes 0))
  else if n == 12 then invoke12 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0)) (← ldp (args + 7 * wordBytes 0)) (← ldp (args + 8 * wordBytes 0)) (← ldp (args + 9 * wordBytes 0)) (← ldp (args + 10 * wordBytes 0)) (← ldp (args + 11 * wordBytes 0))
  else if n == 13 then invoke13 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0)) (← ldp (args + 7 * wordBytes 0)) (← ldp (args + 8 * wordBytes 0)) (← ldp (args + 9 * wordBytes 0)) (← ldp (args + 10 * wordBytes 0)) (← ldp (args + 11 * wordBytes 0)) (← ldp (args + 12 * wordBytes 0))
  else if n == 14 then invoke14 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0)) (← ldp (args + 7 * wordBytes 0)) (← ldp (args + 8 * wordBytes 0)) (← ldp (args + 9 * wordBytes 0)) (← ldp (args + 10 * wordBytes 0)) (← ldp (args + 11 * wordBytes 0)) (← ldp (args + 12 * wordBytes 0)) (← ldp (args + 13 * wordBytes 0))
  else if n == 15 then invoke15 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0)) (← ldp (args + 7 * wordBytes 0)) (← ldp (args + 8 * wordBytes 0)) (← ldp (args + 9 * wordBytes 0)) (← ldp (args + 10 * wordBytes 0)) (← ldp (args + 11 * wordBytes 0)) (← ldp (args + 12 * wordBytes 0)) (← ldp (args + 13 * wordBytes 0)) (← ldp (args + 14 * wordBytes 0))
  else if n == 16 then invoke16 fn (← ldp (args + 0 * wordBytes 0)) (← ldp (args + 1 * wordBytes 0)) (← ldp (args + 2 * wordBytes 0)) (← ldp (args + 3 * wordBytes 0)) (← ldp (args + 4 * wordBytes 0)) (← ldp (args + 5 * wordBytes 0)) (← ldp (args + 6 * wordBytes 0)) (← ldp (args + 7 * wordBytes 0)) (← ldp (args + 8 * wordBytes 0)) (← ldp (args + 9 * wordBytes 0)) (← ldp (args + 10 * wordBytes 0)) (← ldp (args + 11 * wordBytes 0)) (← ldp (args + 12 * wordBytes 0)) (← ldp (args + 13 * wordBytes 0)) (← ldp (args + 14 * wordBytes 0)) (← ldp (args + 15 * wordBytes 0))
  else invokeArray fn args

partial def incCopy (src dst i n : USize) : BaseIO Unit := do
  if i < n then
    let a ← ldp (src + i * wordBytes 0)
    inc a
    stp (dst + i * wordBytes 0) a
    incCopy src dst (i + 1) n

partial def rawCopy (src dst i n : USize) : BaseIO Unit := do
  if i < n then
    stp (dst + i * wordBytes 0) (← ldp (src + i * wordBytes 0))
    rawCopy src dst (i + 1) n

partial def incArgs (args i n : USize) : BaseIO Unit := do
  if i < n then
    inc (← ldp (args + i * wordBytes 0))
    incArgs args (i + 1) n

partial def decArgs (args i n : USize) : BaseIO Unit := do
  if i < n then
    dec (← ldp (args + i * wordBytes 0))
    decArgs args (i + 1) n

def fixArgs (f n args : USize) : BaseIO USize := do
  let fixed ← ld16 (f + cloFixedOff 0)
  let arity ← ld16 (f + arityOff 0)
  let count := fixed.toUSize + n
  let r ← allocObj (← checkedBytes (cloObjs 0) (wordBytes 0) count)
  setHeader r (closureHeader 0)
  stp (r + funOff 0) (← ldp (f + funOff 0))
  st16 (r + arityOff 0) arity
  st16 (r + cloFixedOff 0) count.toUInt16
  let src := f + cloObjs 0
  let dst := r + cloObjs 0
  if ← isExclusive f then
    copy dst src (fixed.toUSize * wordBytes 0)
    freeObject f
  else
    incCopy src dst 0 fixed.toUSize
    decRef f
  copy (dst + fixed.toUSize * wordBytes 0) args (n * wordBytes 0)
  pure r

partial def applyLoop (f n args buf : USize) : BaseIO USize := do
  if f &&& 1 != 0 then
    decArgs args 0 n
    pure f
  else
    let arity := (← ld16 (f + arityOff 0)).toUSize
    let fixed := (← ld16 (f + cloFixedOff 0)).toUSize
    let fn ← ldp (f + funOff 0)
    if arity > fixed + n then fixArgs f n args
    else if arity > 64 then fatal 9
    else
      -- Allocated once, only after the arity check. Each recursive callback
      -- owns a different frame. Over-application reuses this same buffer.
      let buf ← if buf == 0 then stack (64 * wordBytes 0) else pure buf
      if arity == fixed + n then
        rawCopy (f + cloObjs 0) buf 0 fixed
        copy (buf + fixed * wordBytes 0) args (n * wordBytes 0)
        if arity ≤ 16 && (← isExclusive f) then
          let r ← call fn arity buf
          freeObject f
          pure r
        else
          incArgs buf 0 fixed
          let r ← call fn arity buf
          decRef f
          pure r
      else
        incCopy (f + cloObjs 0) buf 0 fixed
        let used := arity - fixed
        copy (buf + fixed * wordBytes 0) args (used * wordBytes 0)
        let g ← call fn arity buf
        decRef f
        applyLoop g (n - used) (args + used * wordBytes 0) buf

@[export zr_low_apply]
def apply (f n args : USize) : BaseIO USize := applyLoop f n args 0

@[export zr_low_apply_1]
def apply1 (f a1 : USize) : BaseIO USize := do
  let args ← stack (1 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  applyLoop f 1 args 0

@[export zr_low_apply_2]
def apply2 (f a1 a2 : USize) : BaseIO USize := do
  let args ← stack (2 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  applyLoop f 2 args 0

@[export zr_low_apply_3]
def apply3 (f a1 a2 a3 : USize) : BaseIO USize := do
  let args ← stack (3 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  applyLoop f 3 args 0

@[export zr_low_apply_4]
def apply4 (f a1 a2 a3 a4 : USize) : BaseIO USize := do
  let args ← stack (4 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  applyLoop f 4 args 0

@[export zr_low_apply_5]
def apply5 (f a1 a2 a3 a4 a5 : USize) : BaseIO USize := do
  let args ← stack (5 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  applyLoop f 5 args 0

@[export zr_low_apply_6]
def apply6 (f a1 a2 a3 a4 a5 a6 : USize) : BaseIO USize := do
  let args ← stack (6 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  applyLoop f 6 args 0

@[export zr_low_apply_7]
def apply7 (f a1 a2 a3 a4 a5 a6 a7 : USize) : BaseIO USize := do
  let args ← stack (7 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  applyLoop f 7 args 0

@[export zr_low_apply_8]
def apply8 (f a1 a2 a3 a4 a5 a6 a7 a8 : USize) : BaseIO USize := do
  let args ← stack (8 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  stp (args + 7 * wordBytes 0) a8
  applyLoop f 8 args 0

@[export zr_low_apply_9]
def apply9 (f a1 a2 a3 a4 a5 a6 a7 a8 a9 : USize) : BaseIO USize := do
  let args ← stack (9 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  stp (args + 7 * wordBytes 0) a8
  stp (args + 8 * wordBytes 0) a9
  applyLoop f 9 args 0

@[export zr_low_apply_10]
def apply10 (f a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 : USize) : BaseIO USize := do
  let args ← stack (10 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  stp (args + 7 * wordBytes 0) a8
  stp (args + 8 * wordBytes 0) a9
  stp (args + 9 * wordBytes 0) a10
  applyLoop f 10 args 0

@[export zr_low_apply_11]
def apply11 (f a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 : USize) : BaseIO USize := do
  let args ← stack (11 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  stp (args + 7 * wordBytes 0) a8
  stp (args + 8 * wordBytes 0) a9
  stp (args + 9 * wordBytes 0) a10
  stp (args + 10 * wordBytes 0) a11
  applyLoop f 11 args 0

@[export zr_low_apply_12]
def apply12 (f a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 : USize) : BaseIO USize := do
  let args ← stack (12 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  stp (args + 7 * wordBytes 0) a8
  stp (args + 8 * wordBytes 0) a9
  stp (args + 9 * wordBytes 0) a10
  stp (args + 10 * wordBytes 0) a11
  stp (args + 11 * wordBytes 0) a12
  applyLoop f 12 args 0

@[export zr_low_apply_13]
def apply13 (f a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 : USize) : BaseIO USize := do
  let args ← stack (13 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  stp (args + 7 * wordBytes 0) a8
  stp (args + 8 * wordBytes 0) a9
  stp (args + 9 * wordBytes 0) a10
  stp (args + 10 * wordBytes 0) a11
  stp (args + 11 * wordBytes 0) a12
  stp (args + 12 * wordBytes 0) a13
  applyLoop f 13 args 0

@[export zr_low_apply_14]
def apply14 (f a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 : USize) : BaseIO USize := do
  let args ← stack (14 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  stp (args + 7 * wordBytes 0) a8
  stp (args + 8 * wordBytes 0) a9
  stp (args + 9 * wordBytes 0) a10
  stp (args + 10 * wordBytes 0) a11
  stp (args + 11 * wordBytes 0) a12
  stp (args + 12 * wordBytes 0) a13
  stp (args + 13 * wordBytes 0) a14
  applyLoop f 14 args 0

@[export zr_low_apply_15]
def apply15 (f a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 : USize) : BaseIO USize := do
  let args ← stack (15 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  stp (args + 7 * wordBytes 0) a8
  stp (args + 8 * wordBytes 0) a9
  stp (args + 9 * wordBytes 0) a10
  stp (args + 10 * wordBytes 0) a11
  stp (args + 11 * wordBytes 0) a12
  stp (args + 12 * wordBytes 0) a13
  stp (args + 13 * wordBytes 0) a14
  stp (args + 14 * wordBytes 0) a15
  applyLoop f 15 args 0

@[export zr_low_apply_16]
def apply16 (f a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 : USize) : BaseIO USize := do
  let args ← stack (16 * wordBytes 0)
  stp (args + 0 * wordBytes 0) a1
  stp (args + 1 * wordBytes 0) a2
  stp (args + 2 * wordBytes 0) a3
  stp (args + 3 * wordBytes 0) a4
  stp (args + 4 * wordBytes 0) a5
  stp (args + 5 * wordBytes 0) a6
  stp (args + 6 * wordBytes 0) a7
  stp (args + 7 * wordBytes 0) a8
  stp (args + 8 * wordBytes 0) a9
  stp (args + 9 * wordBytes 0) a10
  stp (args + 10 * wordBytes 0) a11
  stp (args + 11 * wordBytes 0) a12
  stp (args + 12 * wordBytes 0) a13
  stp (args + 13 * wordBytes 0) a14
  stp (args + 14 * wordBytes 0) a15
  stp (args + 15 * wordBytes 0) a16
  applyLoop f 16 args 0

end ZrLean.Low.Apply
