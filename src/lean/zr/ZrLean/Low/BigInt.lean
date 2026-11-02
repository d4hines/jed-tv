import ZrLean.Low.BigNat

/-! Ints. A small Int is lean.h's scalar `lean_box((unsigned)n)` for `n` in
[LEAN_MIN_SMALL_INT, LEAN_MAX_SMALL_INT]: [-2^30, 2^30) on 32 bits, [-2^31,
2^31) on 64. Any other Int is a big-number object (BigNat's layout: two
UInt64 words at +8 and +16) holding the value in 128-bit two's complement,
so Ints are bounded to [-2^127, 2^127), as Nats are to 2^128: a result
outside stops the program (fatal code 10), never a wrong value. Values are
normalised (an object never holds a small value), so a nonnegative big Int
is the same object as the big Nat of its value: `Int.ofNat` and `Int.toNat`
of big values reuse it, and the existing `lean_int*_of_big_int` (the low
bits) are right for negative values too.

Values are carried as `lo`/`hi` words, a small value sign-extended; no Lean
Int is used. These are the out-of-line paths of HeaderInt and its shared
helpers. -/
namespace ZrLean.Low.BigInt
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc ZrLean.Low.Once

set_option compiler.extract_closed false

/-- Value bits of a small Int: 31 on 32 bits, 32 on 64. -/
@[inline] def smallBits (z : USize) : UInt64 := if wordBytes z == 4 then 31 else 32

/-- The value of the small Int `a`, sign-extended to 64 bits. -/
@[inline] def unbox (a : USize) : UInt64 :=
  let v := (a >>> 1).toUInt64
  if v >>> (smallBits 0 - 1) != 0 then v ||| (0xffffffffffffffff <<< smallBits 0) else v

/-- Whether the 64-bit two's complement `v` is a small Int. -/
@[inline] def fits (v : UInt64) : Bool :=
  v + ((1 : UInt64) <<< (smallBits 0 - 1)) < ((1 : UInt64) <<< smallBits 0)

/-- `lean_box((unsigned)v)` of a small `v`. -/
@[inline] def box (v : UInt64) : USize := (v.toUInt32.toUSize <<< 1) ||| 1

/-- A small Int's word with its sign flipped: words ordered as their values. -/
@[inline] def key (a : USize) : USize := a ^^^ ((1 : USize) <<< (smallBits 0).toUSize)

/-- The high word extending the low word `l`'s sign. -/
@[inline] def ext (l : UInt64) : UInt64 := if l >>> 63 != 0 then 0xffffffffffffffff else 0
@[inline] def isNeg (h : UInt64) : Bool := h >>> 63 != 0

@[inline] def lo (a : USize) : BaseIO UInt64 :=
  if a &&& 1 != 0 then pure (unbox a) else ld64 (a + 8)
@[inline] def hi (a : USize) : BaseIO UInt64 :=
  if a &&& 1 != 0 then pure (ext (unbox a)) else ld64 (a + 16)

/-- `-(h:l)`, high word (the low word is `0 - l`). -/
@[inline] def negHi (l h : UInt64) : UInt64 := ~~~h + (if l == 0 then 1 else 0)
/-- The magnitude of `h:l` (as unsigned 128 bits: 2^127 for -2^127). -/
@[inline] def magLo (l h : UInt64) : UInt64 := if isNeg h then 0 - l else l
@[inline] def magHi (l h : UInt64) : UInt64 := if isNeg h then negHi l h else h

/-- Arithmetic shift right of a 64-bit value by `k` < 64. -/
@[inline] def sar (v k : UInt64) : UInt64 := if isNeg v then ~~~((~~~v) >>> k) else v >>> k
/-- Arithmetic shift right of `h:l` by `k` < 128. -/
@[inline] def sarLo (l h k : UInt64) : UInt64 :=
  if isNeg h then ~~~(BigNat.shrLo (~~~l) (~~~h) k) else BigNat.shrLo l h k
@[inline] def sarHi (h k : UInt64) : UInt64 :=
  if isNeg h then ~~~(BigNat.shrHi (~~~h) k) else BigNat.shrHi h k

@[inline] def overflow : BaseIO USize := fatal 10

/-- The Int `h:l`: a scalar when small, else a fresh object. -/
@[export zr_low_int_of]
def ofWords (l h : UInt64) : BaseIO USize := do
  if h == ext l && fits l then pure (box l) else BigNat.object l h

/-- The Int of the 64-bit two's complement `v`. -/
@[inline] def ofInt64 (v : UInt64) : BaseIO USize :=
  if fits v then pure (box v) else ofWords v (ext v)

/-- The Int with magnitude `mh:ml` (unsigned) and sign `neg`. -/
@[inline] def ofMag (neg : Bool) (ml mh : UInt64) : BaseIO USize :=
  if neg then
    if mh > 0x8000000000000000 || (mh == 0x8000000000000000 && ml != 0) then overflow
    else ofWords (0 - ml) (negHi ml mh)
  else if isNeg mh then overflow
  else ofWords ml mh

/-- `Int.ofNat` of a big Nat (owned): the same object. -/
@[export zr_low_int_big_of_nat]
def ofNat (a : USize) : BaseIO USize := do
  if isNeg (← ld64 (a + 16)) then overflow else pure a

/-- `Int.negSucc a`, `-(a + 1) = ~a`, of a big Nat (owned). -/
@[export zr_low_int_big_neg_succ_of_nat]
def negSuccOfNat (a : USize) : BaseIO USize := do
  let l ← ld64 (a + 8)
  let h ← ld64 (a + 16)
  dec a
  if isNeg h then overflow else ofWords (~~~l) (~~~h)

@[export zr_low_int_big_neg]
def neg (a : USize) : BaseIO USize := do
  let l ← lo a; let h ← hi a
  ofMag (!isNeg h) (magLo l h) (magHi l h)

@[export zr_low_int_big_add]
def add (a b : USize) : BaseIO USize := do
  let al ← lo a; let ah ← hi a; let bl ← lo b; let bh ← hi b
  let l := al + bl
  let h := ah + bh + (if l < al then 1 else 0)
  if isNeg ah == isNeg bh && isNeg h != isNeg ah then overflow else ofWords l h

@[export zr_low_int_big_sub]
def sub (a b : USize) : BaseIO USize := do
  let al ← lo a; let ah ← hi a; let bl ← lo b; let bh ← hi b
  let l := al - bl
  let h := ah - bh - (if al < bl then 1 else 0)
  if isNeg ah != isNeg bh && isNeg h != isNeg ah then overflow else ofWords l h

/-- Product of the magnitudes, as BigNat.mulWords, with sign `neg`. -/
@[inline] def mulMag (neg : Bool) (al ah bl bh : UInt64) : BaseIO USize := do
  if ah != 0 && bh != 0 then overflow
  else
    let h := BigNat.mulHi al bl
    let x := if ah != 0 then ah else al
    let y := if ah != 0 then bl else bh
    let cross := x * y
    if BigNat.mulHi x y != 0 || h + cross < h then overflow
    else ofMag neg (al * bl) (h + cross)

@[export zr_low_int_big_mul]
def mul (a b : USize) : BaseIO USize := do
  let al ← lo a; let ah ← hi a; let bl ← lo b; let bh ← hi b
  mulMag (isNeg ah != isNeg bh) (magLo al ah) (magHi al ah) (magLo bl bh) (magHi bl bh)

/-! Division. `mode`: 0 `Int.tdiv` (`/` rounding toward zero, lean_int_div),
1 `Int.tmod`, 2 `Int.ediv` (`/`), 3 `Int.emod` (`%`). From the magnitudes'
quotient `q` and remainder `r` (divisor `d`): T-rounding signs `q` by both
signs and `r` by the dividend's; Euclidean rounding takes a negative
dividend's quotient one further from zero, and its remainder to `d - r`,
when `r` is not 0. -/

def divFinish (mode : UInt8) (negA negD : Bool) (ql qh rl rh dl dh : UInt64) : BaseIO USize :=
  let away := negA && (rl ||| rh) != 0
  if mode == 0 then ofMag (negA != negD) ql qh
  else if mode == 1 then ofMag negA rl rh
  else if mode == 2 then
    if away then ofMag (negA != negD) (ql + 1) (qh + (if ql + 1 == 0 then 1 else 0))
    else ofMag (negA != negD) ql qh
  else if away then ofMag false (dl - rl) (dh - rh - (if dl < rl then 1 else 0))
  else ofMag false rl rh

/-- BigNat.divLoop, keeping the original divisor `d0h:d0l` for divFinish. -/
partial def divLoop (nl nh dl dh ql qh : UInt64) (left : USize) (mode : UInt8) (negA negD : Bool)
    (d0l d0h : UInt64) : BaseIO USize := do
  if left == 0 then divFinish mode negA negD ql qh nl nh d0l d0h
  else
    let take := !BigNat.lt nl nh dl dh
    let nlo := if take then nl - dl else nl
    let nhi := if take then nh - dh - (if nl < dl then 1 else 0) else nh
    let qlo := (ql <<< 1) ||| (if take then 1 else 0)
    let qhi := (qh <<< 1) ||| (ql >>> 63)
    divLoop nlo nhi (BigNat.shrLo dl dh 1) (BigNat.shrHi dh 1) qlo qhi (left - 1) mode negA negD d0l d0h

/-- `a` (borrowed) divided by `b` in `mode`; by 0: quotient 0, remainder `a`. -/
@[export zr_low_int_big_div_mod]
def divMod (a b : USize) (mode : UInt8) : BaseIO USize := do
  let bl ← lo b; let bh ← hi b
  if (bl ||| bh) == 0 then
    if mode &&& 1 != 0 then inc a; pure a else pure 1
  else
    let al ← lo a; let ah ← hi a
    let negA := isNeg ah; let negD := isNeg bh
    let nl := magLo al ah; let nh := magHi al ah
    let dl := magLo bl bh; let dh := magHi bl bh
    if nh == 0 && dh == 0 then divFinish mode negA negD (nl / dl) 0 (nl % dl) 0 dl dh
    else if BigNat.lt nl nh dl dh then divFinish mode negA negD 0 0 nl nh dl dh
    else
      let s := BigNat.clz dl dh - BigNat.clz nl nh
      divLoop nl nh (BigNat.shlLo dl s) (BigNat.shlHi dl dh s) 0 0 (s.toUSize + 1) mode negA negD dl dh

@[export zr_low_int_big_eq]
def eq (a b : USize) : BaseIO Bool := do pure ((← lo a) == (← lo b) && (← hi a) == (← hi b))

/-- Signed `a < b`: the high words compared with their signs flipped. -/
@[export zr_low_int_big_lt]
def less (a b : USize) : BaseIO Bool := do
  pure (BigNat.lt (← lo a) ((← hi a) ^^^ 0x8000000000000000) (← lo b) ((← hi b) ^^^ 0x8000000000000000))

@[export zr_low_int_big_le]
def lessEq (a b : USize) : BaseIO Bool := do pure (!(← less b a))

/-- `Int.toNat` of a big Int (owned): 0 if negative, else the same object or
a scalar. -/
@[export zr_low_int_big_to_nat]
def toNat (a : USize) : BaseIO USize := do
  let l ← ld64 (a + 8)
  let h ← ld64 (a + 16)
  if isNeg h then dec a; pure 1
  else if h == 0 && l ≤ BigNat.maxSmall 0 then dec a; pure (BigNat.boxed l.toUSize)
  else pure a

/-- `Int.natAbs` of a big Int (borrowed). -/
@[export zr_low_int_big_nat_abs]
def natAbs (a : USize) : BaseIO USize := do
  let l ← ld64 (a + 8)
  let h ← ld64 (a + 16)
  if isNeg h then BigNat.ofWords (0 - l) (negHi l h)
  else if h == 0 && l ≤ BigNat.maxSmall 0 then pure (BigNat.boxed l.toUSize)
  else inc a; pure a

end ZrLean.Low.BigInt
