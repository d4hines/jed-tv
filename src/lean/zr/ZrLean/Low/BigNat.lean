import ZrLean.Low.Rc
import ZrLean.Low.Once

/-! Bounded big Nats, as two scalar UInt64 words. No Lean Nat is used to
implement a Nat operation: object pointers and boxed scalars are USize bits. -/
namespace ZrLean.Low.BigNat
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc ZrLean.Low.Once

@[never_extract, extern "zr_prim_bignat_max_small"]
opaque maxSmall (z : USize) : UInt64
@[never_extract, extern "zr_prim_bignat_mpz_header"]
opaque mpzHeader (z : USize) : UInt32

@[inline] def lo (o : USize) : BaseIO UInt64 :=
  if o &&& 1 != 0 then pure (o >>> 1).toUInt64 else ld64 (o + 8)
@[inline] def hi (o : USize) : BaseIO UInt64 :=
  if o &&& 1 != 0 then pure 0 else ld64 (o + 16)
@[inline] def lt (al ah bl bh : UInt64) : Bool := ah < bh || (ah == bh && al < bl)
@[inline] def zero (al ah : UInt64) : Bool := al ||| ah == 0
@[inline] def boxed (v : USize) : USize := (v <<< 1) ||| 1

@[never_extract, extern "zr_prim_bignat_object_bytes"]
opaque objectBytes (z : USize) : USize

/-- A fresh object holding the words `h:l` (big Ints share it: BigInt). -/
@[inline] def object (l h : UInt64) : BaseIO USize := do
  countAllocBytes (objectBytes 0)
  let o ← Alloc.alloc (Alloc.theHeap 0) (objectBytes 0)
  setHeader o (mpzHeader 0)
  st64 (o + 8) l
  st64 (o + 16) h
  pure o

@[export zr_low_nat_of]
def ofWords (l h : UInt64) : BaseIO USize := do
  if h == 0 && l ≤ maxSmall 0 then pure (boxed l.toUSize) else object l h

@[inline] def overflow : BaseIO USize := fatal 4

@[inline] def addWords (al ah bl bh : UInt64) : BaseIO USize := do
  let l := al + bl
  let c : UInt64 := if l < al then 1 else 0
  let h := ah + bh
  if h < ah || h + c < h then overflow else ofWords l (h + c)

@[export zr_low_nat_big_add]
def add (a b : USize) : BaseIO USize := do
  addWords (← lo a) (← hi a) (← lo b) (← hi b)

@[export zr_low_nat_big_sub]
def sub (a b : USize) : BaseIO USize := do
  let al ← lo a; let ah ← hi a; let bl ← lo b; let bh ← hi b
  if lt al ah bl bh then pure 1
  else ofWords (al - bl) (ah - bh - (if al < bl then 1 else 0))

/-- High half of the full 64 by 64 product, without a compiler 128-bit type. -/
@[inline] def mulHi (a b : UInt64) : UInt64 :=
  let a0 := a &&& 0xffffffff; let a1 := a >>> 32
  let b0 := b &&& 0xffffffff; let b1 := b >>> 32
  let p00 := a0 * b0; let p01 := a0 * b1; let p10 := a1 * b0
  let mid := (p00 >>> 32) + (p01 &&& 0xffffffff) + (p10 &&& 0xffffffff)
  a1 * b1 + (p01 >>> 32) + (p10 >>> 32) + (mid >>> 32)

@[inline] def mulWords (al ah bl bh : UInt64) : BaseIO USize := do
  if ah != 0 && bh != 0 then overflow
  else
    let h := mulHi al bl
    let x := if ah != 0 then ah else al
    let y := if ah != 0 then bl else bh
    let cross := x * y
    if mulHi x y != 0 || h + cross < h then overflow
    else ofWords (al * bl) (h + cross)

@[export zr_low_nat_big_mul]
def mul (a b : USize) : BaseIO USize := do
  mulWords (← lo a) (← hi a) (← lo b) (← hi b)

@[export zr_low_nat_overflow_mul]
def overflowMul (a b : USize) : BaseIO USize := mulWords a.toUInt64 0 b.toUInt64 0

@[inline] def shlLo (l : UInt64) (s : UInt64) : UInt64 := if s ≥ 64 then 0 else l <<< s
@[inline] def shlHi (l h s : UInt64) : UInt64 :=
  if s == 0 then h else if s ≥ 64 then l <<< (s - 64) else (h <<< s) ||| (l >>> (64 - s))
@[inline] def shrLo (l h s : UInt64) : UInt64 :=
  if s == 0 then l else if s ≥ 64 then h >>> (s - 64) else (l >>> s) ||| (h <<< (64 - s))
@[inline] def shrHi (h s : UInt64) : UInt64 := if s ≥ 64 then 0 else h >>> s
@[inline] def clz (l h : UInt64) : UInt64 :=
  if h != 0 then 63 - h.log2 else if l != 0 then 127 - l.log2 else 128

partial def divLoop (nl nh dl dh ql qh : UInt64) (left : USize) (rem : UInt8) : BaseIO USize := do
  if left == 0 then
    if rem != 0 then ofWords nl nh else ofWords ql qh
  else
    let take := !lt nl nh dl dh
    let nlo := if take then nl - dl else nl
    let nhi := if take then nh - dh - (if nl < dl then 1 else 0) else nh
    let qlo := (ql <<< 1) ||| (if take then 1 else 0)
    let qhi := (qh <<< 1) ||| (ql >>> 63)
    divLoop nlo nhi (shrLo dl dh 1) (shrHi dh 1) qlo qhi (left - 1) rem

@[inline] def divWords (nl nh dl dh : UInt64) (rem : UInt8) : BaseIO USize := do
  if nh == 0 && dh == 0 then ofWords (if rem != 0 then nl % dl else nl / dl) 0
  else if lt nl nh dl dh then
    if rem != 0 then ofWords nl nh else pure 1
  else
    let s := clz dl dh - clz nl nh
    divLoop nl nh (shlLo dl s) (shlHi dl dh s) 0 0 (s.toUSize + 1) rem

@[export zr_low_nat_big_div]
def div (a b : USize) : BaseIO USize := do
  let bl ← lo b; let bh ← hi b
  if zero bl bh then pure 1 else divWords (← lo a) (← hi a) bl bh 0

@[export zr_low_nat_big_mod]
def mod (a b : USize) : BaseIO USize := do
  let bl ← lo b; let bh ← hi b
  if zero bl bh then inc a; pure a else divWords (← lo a) (← hi a) bl bh 1

partial def gcdLoop (al ah bl bh shift : UInt64) : BaseIO USize := do
  if zero al ah then ofWords (shlLo bl shift) (shlHi bl bh shift)
  else if zero bl bh then ofWords (shlLo al shift) (shlHi al ah shift)
  else if al &&& 1 == 0 then
    if bl &&& 1 == 0 then gcdLoop (shrLo al ah 1) (shrHi ah 1) (shrLo bl bh 1) (shrHi bh 1) (shift + 1)
    else gcdLoop (shrLo al ah 1) (shrHi ah 1) bl bh shift
  else if bl &&& 1 == 0 then gcdLoop al ah (shrLo bl bh 1) (shrHi bh 1) shift
  else if lt al ah bl bh then gcdLoop al ah (bl - al) (bh - ah - (if bl < al then 1 else 0)) shift
  else gcdLoop (al - bl) (ah - bh - (if al < bl then 1 else 0)) bl bh shift

@[export zr_low_nat_gcd]
def gcd (a b : USize) : BaseIO USize := do
  gcdLoop (← lo a) (← hi a) (← lo b) (← hi b) 0

@[export zr_low_nat_big_eq]
def eq (a b : USize) : BaseIO Bool := do pure ((← lo a) == (← lo b) && (← hi a) == (← hi b))
@[export zr_low_nat_big_lt]
def less (a b : USize) : BaseIO Bool := do pure (lt (← lo a) (← hi a) (← lo b) (← hi b))
@[export zr_low_nat_big_le]
def lessEq (a b : USize) : BaseIO Bool := do pure (!lt (← lo b) (← hi b) (← lo a) (← hi a))

@[export zr_low_nat_big_land]
def land (a b : USize) : BaseIO USize := do ofWords ((← lo a) &&& (← lo b)) ((← hi a) &&& (← hi b))
@[export zr_low_nat_big_lor]
def lor (a b : USize) : BaseIO USize := do ofWords ((← lo a) ||| (← lo b)) ((← hi a) ||| (← hi b))
@[export zr_low_nat_big_xor]
def xor (a b : USize) : BaseIO USize := do ofWords ((← lo a) ^^^ (← lo b)) ((← hi a) ^^^ (← hi b))

@[export zr_low_nat_shiftl]
def shiftl (a b : USize) : BaseIO USize := do
  let l ← lo a; let h ← hi a
  if zero l h then pure 1
  else if b &&& 1 == 0 || b >>> 1 > 0xffffffff then fatal 5
  else
    let s := (b >>> 1).toUInt64
    if s ≥ 128 || clz l h < s then overflow
    else ofWords (shlLo l s) (shlHi l h s)

@[export zr_low_nat_big_shiftr]
def shiftr (a b : USize) : BaseIO USize := do
  if b &&& 1 == 0 || b >>> 1 ≥ 128 then pure 1
  else
    let s := (b >>> 1).toUInt64
    ofWords (shrLo (← lo a) (← hi a) s) (shrHi (← hi a) s)

partial def powLoop (bl bh al ah : UInt64) (e : UInt32) : BaseIO USize := do
  -- Multiplication shares the same overflow checks, but keeps intermediate
  -- words in scalars, so exponentiation has just its final allocation.
  let cross := if ah != 0 then ah * bl else al * bh
  let h := mulHi al bl
  let x := if ah != 0 then ah else al
  let y := if ah != 0 then bl else bh
  let nl := if e &&& 1 != 0 then al * bl else al
  let nh := if e &&& 1 != 0 then h + cross else ah
  if e &&& 1 != 0 && ((ah != 0 && bh != 0) || mulHi x y != 0 || h + cross < h) then overflow
  else if e == 1 then ofWords nl nh
  else
    if bh != 0 then overflow
    else powLoop (bl * bl) (mulHi bl bl) nl nh (e / 2)

@[export zr_low_nat_pow]
def pow (a b : USize) : BaseIO USize := do
  if b &&& 1 == 0 || b >>> 1 > 0xffffffff then
    if a &&& 1 != 0 && a >>> 1 ≤ 1 then pure a else fatal 6
  else if b >>> 1 == 0 then pure 3
  else powLoop (← lo a) (← hi a) 1 0 (b >>> 1).toUInt32

@[export zr_low_big_usize_to_nat]
def fromUSize (a : USize) : BaseIO USize := ofWords a.toUInt64 0
@[export zr_low_big_uint64_to_nat]
def fromUInt64 (a : UInt64) : BaseIO USize := ofWords a 0

@[never_extract, extern "zr_prim_bignat_literals"]
opaque literals (z : USize) : USize

partial def findLiteral (base : USize) (l h : UInt64) (i n : USize) : BaseIO USize := do
  if i == n then pure 0
  else if (← ld64 (base + i * 24)) == l && (← ld64 (base + i * 24 + 8)) == h then
    ldp (base + i * 24 + 16)
  else findLiteral base l h (i + 1) n

def literal (l h : UInt64) : BaseIO USize := do
  if h == 0 && l ≤ maxSmall 0 then pure (boxed l.toUSize)
  else
    let state := literals 0
    let base ← ldp state
    let n ← ldp (state + wordBytes 0)
    let cap ← ldp (state + 2 * wordBytes 0)
    let old ← findLiteral base l h 0 n
    if old != 0 then pure old
    else
      let o ← ofWords l h
      RcCold.markPersistent o
      let p ← if n == cap then do
          let next := if cap == 0 then 4 else cap * 2
          let p ← Alloc.memAlloc (next * 24) 8
          if p == 0 then fatal 0
          else
            if n != 0 then
              copy p base (n * 24)
              Alloc.memFree base (cap * 24) 8
            stp state p
            stp (state + 2 * wordBytes 0) next
            pure p
        else pure base
      st64 (p + n * 24) l
      st64 (p + n * 24 + 8) h
      stp (p + n * 24 + 16) o
      stp (state + wordBytes 0) (n + 1)
      pure o

@[export zr_low_big_unsigned_to_nat]
def unsignedLiteral (n : UInt32) : BaseIO USize := literal n.toUInt64 0

partial def parseLoop (s : USize) (l h : UInt64) : BaseIO USize := do
  let c ← ld8 s
  if c == 0 then literal l h
  else if c < 48 || c > 57 then fatal 7
  else
    let l0 := l * 10
    let carry := mulHi l 10
    let h0 := h * 10
    let digit := (c - 48).toUInt64
    let carry1 : UInt64 := if l0 + digit < l0 then 1 else 0
    if mulHi h 10 != 0 || h0 + carry < h0 || h0 + carry + carry1 < h0 + carry then overflow
    else parseLoop (s + 1) (l0 + digit) (h0 + carry + carry1)

@[export zr_low_cstr_to_nat]
def parse (s : USize) : BaseIO USize := parseLoop s 0 0

@[export zr_low_nat_get]
def get (a out : USize) : BaseIO UInt8 := do
  st64 out (← lo a)
  st64 (out + 8) (← hi a)
  pure 0

@[export zr_low_uint8_of_big_nat]
def toUInt8 (a : USize) : BaseIO UInt8 := do pure (← lo a).toUInt8
@[export zr_low_uint16_of_big_nat]
def toUInt16 (a : USize) : BaseIO UInt16 := do pure (← lo a).toUInt16
@[export zr_low_uint32_of_big_nat]
def toUInt32 (a : USize) : BaseIO UInt32 := do pure (← lo a).toUInt32
@[export zr_low_uint64_of_big_nat]
def toUInt64 (a : USize) : BaseIO UInt64 := lo a
@[export zr_low_usize_of_big_nat]
def toUSize (a : USize) : BaseIO USize := do pure (← lo a).toUSize

@[extern "zr_prim_bignat_bits_int8"]
opaque bitsInt8 (v : UInt64) : Int8
@[export zr_low_int8_of_big_int]
def toInt8 (a : USize) : BaseIO Int8 := do pure (bitsInt8 (← lo a))

@[extern "zr_prim_bignat_bits_int16"]
opaque bitsInt16 (v : UInt64) : Int16
@[export zr_low_int16_of_big_int]
def toInt16 (a : USize) : BaseIO Int16 := do pure (bitsInt16 (← lo a))

@[extern "zr_prim_bignat_bits_int32"]
opaque bitsInt32 (v : UInt64) : Int32
@[export zr_low_int32_of_big_int]
def toInt32 (a : USize) : BaseIO Int32 := do pure (bitsInt32 (← lo a))

@[extern "zr_prim_bignat_bits_int64"]
opaque bitsInt64 (v : UInt64) : Int64
@[export zr_low_int64_of_big_int]
def toInt64 (a : USize) : BaseIO Int64 := do pure (bitsInt64 (← lo a))

end ZrLean.Low.BigNat
