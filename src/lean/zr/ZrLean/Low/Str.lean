/-
Strings, List/Array conversion, `ByteArray` equality and data, `Nat.log2`
and `panic!`: the out-of-line `lean.h` functions apps need beyond the
router's firmware. The router only linked C versions of these into its tests
(test/leanrt/csrc/zr_runtime_testing.c in dande-router); these are the same
algorithms in ZrLow Lean, checked by the conformance transcript.

Strings are `lean_string_object`s: header, `m_size` (bytes including the
terminating NUL), `m_capacity`, `m_length` (code points), then the bytes.
-/
import ZrLean.Low.HeaderSupport
import ZrLean.Low.Runtime
import ZrLean.Low.BigNat

namespace ZrLean.Low.Str
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc

/-! ### Lists and arrays -/

/-- The length of the list `it` (`box 0` is nil, a cons has tag 1 and fields
head, tail). -/
partial def listLength (it n : USize) : BaseIO USize := do
  if it &&& 1 != 0 then pure n
  else listLength (← ldp (it + ctorObjs 0 + wordBytes 0)) (n + 1)

partial def listFill (r it i n : USize) : BaseIO Unit := do
  if i < n then
    let head ← ldp (it + ctorObjs 0)
    inc head
    stp (r + arrData 0 + i * wordBytes 0) head
    listFill r (← ldp (it + ctorObjs 0 + wordBytes 0)) (i + 1) n

/-- `lean_array_mk`: `List.toArray` (the list is consumed). -/
@[export zr_low_array_mk]
def arrayMk (l : USize) : BaseIO USize := do
  let n ← listLength l 0
  let r ← allocArray n n
  listFill r l 0 n
  dec l
  pure r

partial def consFrom (a i l : USize) : BaseIO USize := do
  if i == 0 then pure l
  else
    let x ← ldp (a + arrData 0 + (i - 1) * wordBytes 0)
    inc x
    let c ← HeaderObj.allocCtor 1 2 0
    stp (c + ctorObjs 0) x
    stp (c + ctorObjs 0 + wordBytes 0) l
    consFrom a (i - 1) c

/-- `lean_array_to_list`: `Array.toList` (the array is consumed). -/
@[export zr_low_array_to_list]
def arrayToList (a : USize) : BaseIO USize := do
  let l ← consFrom a (← ldp (a + arrSizeOff 0)) 1
  dec a
  pure l

/-! ### Scalar arrays -/

/-- Whether the `n` bytes at `p` and `q` are equal. -/
partial def bytesEq (p q i n : USize) : BaseIO Bool := do
  if i ≥ n then pure true
  else if (← ld8 (p + i)) != (← ld8 (q + i)) then pure false
  else bytesEq p q (i + 1) n

/-- `lean_sarray_eq_cold`: same element size and size (checked inline), so
compare the bytes. -/
@[export zr_low_sarray_eq_cold]
def sarrayEqCold (a b : USize) : BaseIO UInt8 := do
  let len := (← elemSize a) * (← ldp (a + sarrSizeOff 0))
  pure (if ← bytesEq (a + sarrData 0) (b + sarrData 0) 0 len then 1 else 0)

partial def boxBytes (src dst i n : USize) : BaseIO Unit := do
  if i < n then
    stp (dst + i * wordBytes 0) (((← ld8 (src + i)).toUSize <<< 1) ||| 1)
    boxBytes src dst (i + 1) n

/-- `lean_byte_array_data`: `ByteArray.data`, the bytes as an `Array UInt8`
(`a` consumed). -/
@[export zr_low_byte_array_data]
def byteArrayData (a : USize) : BaseIO USize := do
  let sz ← ldp (a + sarrSizeOff 0)
  let r ← allocArray sz sz
  boxBytes (a + sarrData 0) (r + arrData 0) 0 sz
  dec a
  pure r

/-! ### Nats -/

/-- `lean_nat_log2`: `Nat.log2` (0 for 0), `a` borrowed. -/
@[export zr_low_nat_log2]
def natLog2 (a : USize) : BaseIO USize := do
  let l ← BigNat.lo a
  let h ← BigNat.hi a
  if l ||| h == 0 then pure 1
  else pure (BigNat.boxed (127 - BigNat.clz l h).toUSize)

/-! ### Panics -/

/-- `lean_panic_fn`: report `msg` (consumed), return `default` (owned). -/
@[export zr_low_panic_fn]
def panicFn (default msg : USize) : BaseIO USize := do
  let _ ← Runtime.leanPanic (msg + strData 0) ((← ldp (msg + strSizeOff 0)) - 1)
  dec msg
  pure default

/-- `lean_panic_fn_borrowed`: the same with `default` borrowed. -/
@[export zr_low_panic_fn_borrowed]
def panicFnBorrowed (default msg : USize) : BaseIO USize := do
  inc default
  panicFn default msg

/-! ### Strings -/

@[inline] def lenOff (z : USize) : USize := strCapOff z + wordBytes z

/-- The code points among the `n` bytes at `b`: bytes that do not continue
a sequence. -/
partial def utf8Len (b i n k : USize) : BaseIO USize := do
  if i < n then
    let c ← ld8 (b + i)
    utf8Len b (i + 1) n (if c &&& 0xC0 != 0x80 then k + 1 else k)
  else pure k

/-- `lean_copy_string`: a copy of `s` (consumed) with capacity `cap`. -/
def copyString (s cap : USize) : BaseIO USize := do
  let sz ← ldp (s + strSizeOff 0)
  let r ← HeaderSupport.allocString sz cap (← ldp (s + lenOff 0))
  copyLinearMark s r
  copy (r + strData 0) (s + strData 0) sz
  decRef s
  pure r

/-- `lean_copy_string_nonlinear`: the copy of a shared string. Upstream
aborts here for a `String.markLinear` string only when the environment says
LEAN_ABORT_ON_NONLINEAR; with no environment this just copies. -/
@[export zr_low_copy_string_nonlinear]
def copyStringNonlinear (s cap : USize) : BaseIO USize := copyString s cap

/-- Room for `extra` more bytes in the exclusive string `o`. -/
def ensureCapacity (o extra : USize) : BaseIO USize := do
  let sz ← ldp (o + strSizeOff 0)
  let cap ← ldp (o + strCapOff 0)
  if sz + extra ≤ cap then pure o
  else
    let r ← HeaderSupport.allocString sz (cap + sz + extra) (← ldp (o + lenOff 0))
    copyLinearMark o r
    copy (r + strData 0) (o + strData 0) sz
    freeObject o
    pure r

/-- Write the UTF-8 encoding of the scalar value `c` at `d`; its length. -/
def encode (d : USize) (c : UInt32) : BaseIO USize := do
  if c < 0x80 then
    st8 d c.toUInt8
    pure 1
  else if c < 0x800 then
    st8 d ((0xC0 : UInt32) ||| (c >>> 6)).toUInt8
    st8 (d + 1) ((0x80 : UInt32) ||| (c &&& 0x3F)).toUInt8
    pure 2
  else if c < 0x10000 then
    st8 d ((0xE0 : UInt32) ||| (c >>> 12)).toUInt8
    st8 (d + 1) ((0x80 : UInt32) ||| ((c >>> 6) &&& 0x3F)).toUInt8
    st8 (d + 2) ((0x80 : UInt32) ||| (c &&& 0x3F)).toUInt8
    pure 3
  else
    st8 d ((0xF0 : UInt32) ||| (c >>> 18)).toUInt8
    st8 (d + 1) ((0x80 : UInt32) ||| ((c >>> 12) &&& 0x3F)).toUInt8
    st8 (d + 2) ((0x80 : UInt32) ||| ((c >>> 6) &&& 0x3F)).toUInt8
    st8 (d + 3) ((0x80 : UInt32) ||| (c &&& 0x3F)).toUInt8
    pure 4

/-- Decode one scalar from a valid UTF-8 string. The caller supplies its
leading-byte address, so continuation bytes are always within the string. -/
def decode (d : USize) : BaseIO UInt32 := do
  let a := (← ld8 d).toUInt32
  if a < 0x80 then pure a else
    let b := (← ld8 (d + 1)).toUInt32 &&& 0x3f
    if a < 0xe0 then pure (((a &&& 0x1f) <<< 6) ||| b) else
      let c := (← ld8 (d + 2)).toUInt32 &&& 0x3f
      if a < 0xf0 then pure (((a &&& 0xf) <<< 12) ||| (b <<< 6) ||| c) else
        let e := (← ld8 (d + 3)).toUInt32 &&& 0x3f
        pure (((a &&& 7) <<< 18) ||| (b <<< 12) ||| (c <<< 6) ||| e)

private partial def scalarStart (bytes i : USize) : BaseIO USize := do
  if i == 0 || (← ld8 (bytes + i)) &&& 0xc0 != 0x80 then pure i
  else scalarStart bytes (i - 1)

private partial def charsFrom (bytes endPos tail : USize) : BaseIO USize := do
  if endPos == 0 then pure tail else
    let start ← scalarStart bytes (endPos - 1)
    let char ← HeaderObj.boxUInt32 (← decode (bytes + start))
    let cons ← HeaderObj.allocCtor 1 2 0
    stp (cons + ctorObjs 0) char
    stp (cons + ctorObjs 0 + wordBytes 0) tail
    charsFrom bytes start cons

/-- `lean_string_data`: String.toList consumes its string and returns the
Unicode scalars in order. Embedded NUL is an ordinary scalar. -/
@[export zr_low_string_data]
def data (s : USize) : BaseIO USize := do
  let result ← charsFrom (s + strData 0) ((← ldp (s + strSizeOff 0)) - 1) 1
  dec s
  pure result

private partial def charsSize (list bytes : USize) : BaseIO USize := do
  if list &&& 1 != 0 then pure bytes else
    let char ← HeaderObj.unboxUInt32 (← ldp (list + ctorObjs 0))
    let size : USize := if char < 0x80 then 1 else if char < 0x800 then 2 else if char < 0x10000 then 3 else 4
    charsSize (← ldp (list + ctorObjs 0 + wordBytes 0)) (bytes + size)

private partial def writeChars (list dst : USize) : BaseIO Unit := do
  if list &&& 1 == 0 then
    let size ← encode dst (← HeaderObj.unboxUInt32 (← ldp (list + ctorObjs 0)))
    writeChars (← ldp (list + ctorObjs 0 + wordBytes 0)) (dst + size)

/-- `lean_string_mk`: String.ofList consumes its scalar list. It preserves a
shared list and copies its UTF-8 bytes into a new string. -/
@[export zr_low_string_mk]
def mk (list : USize) : BaseIO USize := do
  let bytes ← charsSize list 0
  let count ← listLength list 0
  let result ← HeaderSupport.allocString (bytes + 1) (bytes + 1) count
  writeChars list (result + strData 0)
  st8 (result + strData 0 + bytes) 0
  dec list
  pure result

/-- `lean_string_push`: `String.push` (`s` owned, `c` a scalar value). -/
@[export zr_low_string_push]
def push (s : USize) (c : UInt32) : BaseIO USize := do
  let sz ← ldp (s + strSizeOff 0)
  let r ← if ← isExclusive s then ensureCapacity s 5 else copyStringNonlinear s ((sz + 5) * 2)
  let n ← encode (r + strData 0 + sz - 1) c
  stp (r + strSizeOff 0) (sz + n)
  stp (r + lenOff 0) ((← ldp (r + lenOff 0)) + 1)
  st8 (r + strData 0 + sz + n - 1) 0
  pure r

/-- `lean_string_append`: `s1 ++ s2` (`s1` owned, `s2` borrowed). -/
@[export zr_low_string_append]
def append (s1 s2 : USize) : BaseIO USize := do
  let sz1 ← ldp (s1 + strSizeOff 0)
  let sz2 ← ldp (s2 + strSizeOff 0)
  let newLen := (← ldp (s1 + lenOff 0)) + (← ldp (s2 + lenOff 0))
  let newSz := sz1 + sz2 - 1
  let r ← if ← isExclusive s1 then ensureCapacity s1 (sz2 - 1) else copyStringNonlinear s1 (newSz * 2)
  -- when s1 is exclusive it cannot be s2 (the caller holds s2)
  move (r + strData 0 + sz1 - 1) (s2 + strData 0) (sz2 - 1)
  stp (r + strSizeOff 0) newSz
  stp (r + lenOff 0) newLen
  st8 (r + strData 0 + newSz - 1) 0
  pure r

/-- `lean_string_eq_cold`: byte equality (`lean.h` checked the sizes). -/
@[export zr_low_string_eq_cold]
def eqCold (s1 s2 : USize) : BaseIO UInt8 := do
  let n := (← ldp (s1 + strSizeOff 0)) - 1
  pure (if ← bytesEq (s1 + strData 0) (s2 + strData 0) 0 n then 1 else 0)

/-- The first difference of the `n` bytes at `a` and `b`: 0 less, 1 none,
2 greater. -/
partial def cmpLoop (a b i n : USize) : BaseIO UInt8 := do
  if i ≥ n then pure 1
  else
    let x ← ld8 (a + i)
    let y ← ld8 (b + i)
    if x != y then pure (if x < y then 0 else 2) else cmpLoop a b (i + 1) n

/-- Lexicographic byte order (code point order, for UTF-8) as an `Ordering`
constructor index: lt 0, eq 1, gt 2. -/
def compareCore (s1 s2 : USize) : BaseIO UInt8 := do
  let n1 := (← ldp (s1 + strSizeOff 0)) - 1
  let n2 := (← ldp (s2 + strSizeOff 0)) - 1
  let r ← cmpLoop (s1 + strData 0) (s2 + strData 0) 0 (if n1 < n2 then n1 else n2)
  if r != 1 then pure r
  else pure (if n1 < n2 then 0 else if n1 > n2 then 2 else 1)

/-- `lean_string_lt`. -/
@[export zr_low_string_lt]
def lt (s1 s2 : USize) : BaseIO UInt8 := do
  pure (if (← compareCore s1 s2) == 0 then 1 else 0)

/-- `lean_string_compare`. -/
@[export zr_low_string_compare]
def compare (s1 s2 : USize) : BaseIO UInt8 := compareCore s1 s2

/-- `lean_string_from_utf8_unchecked`: `String.fromUTF8` after validation
(`a` an owned `ByteArray` of UTF-8). -/
@[export zr_low_string_from_utf8_unchecked]
def fromUtf8Unchecked (a : USize) : BaseIO USize := do
  let b := a + sarrData 0
  let n ← ldp (a + sarrSizeOff 0)
  let r ← HeaderSupport.allocString (n + 1) (n + 1) (← utf8Len b 0 n 0)
  copy (r + strData 0) b n
  st8 (r + strData 0 + n) 0
  dec a
  pure r

/-- Bytes `i + 2 .. i + k` at `b` are continuation bytes. -/
partial def contOk (b i j k : USize) : BaseIO Bool := do
  if j > k then pure true
  else if (← ld8 (b + i + j)) &&& 0xC0 != 0x80 then pure false
  else contOk b i (j + 1) k

/-- Well-formed UTF-8 from byte `i` on (Unicode table 3-7: no overlong
forms, no surrogates, nothing above U+10FFFF). -/
partial def validate (b i n : USize) : BaseIO UInt8 := do
  if i ≥ n then pure 1
  else
    let c ← ld8 (b + i)
    if c < 0x80 then validate b (i + 1) n
    else
      let k : USize :=
        if c ≥ 0xC2 && c ≤ 0xDF then 1
        else if c ≥ 0xE0 && c ≤ 0xEF then 2
        else if c ≥ 0xF0 && c ≤ 0xF4 then 3
        else 0
      let lo : UInt8 := if c == 0xE0 then 0xA0 else if c == 0xF0 then 0x90 else 0x80
      let hi : UInt8 := if c == 0xED then 0x9F else if c == 0xF4 then 0x8F else 0xBF
      if k == 0 then pure 0
      else if n - i - 1 < k then pure 0
      else
        let c1 ← ld8 (b + i + 1)
        if c1 < lo || c1 > hi then pure 0
        else if !(← contOk b i 2 k) then pure 0
        else validate b (i + k + 1) n

/-- `lean_string_validate_utf8`: `String.validateUTF8` (`a` borrowed). -/
@[export zr_low_string_validate_utf8]
def validateUtf8 (a : USize) : BaseIO UInt8 :=
  do validate (a + sarrData 0) 0 (← ldp (a + sarrSizeOff 0))

end ZrLean.Low.Str
