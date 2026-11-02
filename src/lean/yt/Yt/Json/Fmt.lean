/-! Byte and text helpers shared by the format libraries (JSON, HTTP, MP4,
JPEG): decimal and hex formatting, ASCII case folding, base64 and FNV-1a.
Everything links against our runtime (../../RULES.md): no `Init` calls
that are not inlined, no `Int`/`Int64` (their runtime functions are
missing), `Nat` only for sizes and offsets. -/
namespace Yt.Fmt

/-- Byte `i` of `b`, or 0 past the end (no panic path). -/
@[inline] def byteAt (b : ByteArray) (i : Nat) : UInt8 :=
  if h : i < b.size then b[i] else 0

/-- `b[start, stop)` (clamped). -/
@[inline] def slice (b : ByteArray) (start stop : Nat) : ByteArray :=
  b.extract start stop

/-- `b` as a `String` if it is valid UTF-8, else "" (`String.fromUTF8!`
would link upstream's panic-message formatting). -/
@[inline] def str (b : ByteArray) : String :=
  match String.fromUTF8? b with
  | some s => s
  | none => ""

/-- A byte array of `n` zero bytes, built by doubling (memcpy), not byte by byte. -/
def zeros (n : Nat) : ByteArray := Id.run do
  let mut b := ByteArray.emptyWithCapacity n
  b := b.push 0
  while b.size * 2 ≤ n do
    b := b ++ b
  if b.size < n then
    b := b ++ b.extract 0 (n - b.size)
  if n == 0 then ByteArray.empty else b

/-- Append the decimal digits of `x`. -/
def pushDec (out : ByteArray) (x : UInt64) : ByteArray := Id.run do
  if x == 0 then return out.push 48
  let mut digits := ByteArray.emptyWithCapacity 20
  let mut v := x
  while v != 0 do
    digits := digits.push (48 + (v % 10).toUInt8)
    v := v / 10
  let mut o := out
  let mut i := digits.size
  while i > 0 do
    i := i - 1
    o := o.push (byteAt digits i)
  return o

/-- Decimal digits of `x`. -/
@[inline] def decBytes (x : UInt64) : ByteArray := pushDec (ByteArray.emptyWithCapacity 20) x

/-- Decimal string of `x`. -/
def dec (x : UInt64) : String := str (decBytes x)

/-- `x` read as a two's-complement signed 64-bit value. -/
@[inline] def isNeg64 (x : UInt64) : Bool := x >>> 63 != 0

/-- Decimal string of the two's-complement signed value `x` (Rust `i64`). -/
def decI64 (x : UInt64) : String :=
  if isNeg64 x then "-" ++ dec ((0 : UInt64) - x) else dec x

/-- Sign-extend a 32-bit two's-complement value to 64 bits (Rust `as i32 as i64`). -/
@[inline] def sext32 (x : UInt32) : UInt64 :=
  if x >>> 31 != 0 then x.toUInt64 ||| 0xFFFFFFFF00000000 else x.toUInt64

@[inline] def hexDigit (n : UInt8) : UInt8 := if n < 10 then 48 + n else 87 + n

/-- Lowercase hex of `b`. -/
def hex (b : ByteArray) : String := Id.run do
  let mut o := ByteArray.emptyWithCapacity (2 * b.size)
  for x in b do
    o := (o.push (hexDigit (x >>> 4))).push (hexDigit (x &&& 15))
  str o

/-- Lowercase hex of `x`, 16 digits. -/
def hex64 (x : UInt64) : String := Id.run do
  let mut o := ByteArray.emptyWithCapacity 16
  let mut s : UInt64 := 60
  for _ in [0:16] do
    o := o.push (hexDigit ((x >>> s) &&& 15).toUInt8)
    s := s - 4
  str o

@[inline] def lower (c : UInt8) : UInt8 := if 65 ≤ c && c ≤ 90 then c + 32 else c

/-- ASCII case-insensitive equality of `a[ao, ao+n)` and `b`. -/
def eqIgnoreCaseAt (a : ByteArray) (ao : Nat) (n : Nat) (b : ByteArray) : Bool := Id.run do
  if n != b.size || ao + n > a.size then return false
  for i in [0:n] do
    if lower (byteAt a (ao + i)) != lower (byteAt b i) then return false
  return true

/-- ASCII case-insensitive equality. -/
@[inline] def eqIgnoreCase (a b : ByteArray) : Bool := eqIgnoreCaseAt a 0 a.size b

/-- Does `a` start with `p`? -/
def startsWith (a p : ByteArray) : Bool := Id.run do
  if p.size > a.size then return false
  for i in [0:p.size] do
    if byteAt a i != byteAt p i then return false
  return true

/-- Does `a` contain `p`? (naive search; for short needles) -/
def contains (a p : ByteArray) : Bool := Id.run do
  if p.size == 0 then return true
  if p.size > a.size then return false
  for s in [0:a.size - p.size + 1] do
    let mut ok := true
    for i in [0:p.size] do
      if ok && byteAt a (s + i) != byteAt p i then ok := false
    if ok then return true
  return false

/-- `b` as UTF-8 text with every maximal invalid subsequence replaced by
U+FFFD, like Rust's `String::from_utf8_lossy`. -/
def lossy (b : ByteArray) : String := Id.run do
  if let some s := String.fromUTF8? b then return s
  let mut s := ""
  let mut i := 0
  while i < b.size do
    let c := byteAt b i
    if c < 0x80 then
      s := s.push (Char.ofNat c.toNat)
      i := i + 1
    else
      -- Expected length and the allowed range of the second byte.
      let (n, lo, hi) : Nat × UInt8 × UInt8 :=
        if 0xC2 ≤ c && c ≤ 0xDF then (2, 0x80, 0xBF)
        else if c == 0xE0 then (3, 0xA0, 0xBF)
        else if c == 0xED then (3, 0x80, 0x9F)
        else if 0xE1 ≤ c && c ≤ 0xEF then (3, 0x80, 0xBF)
        else if c == 0xF0 then (4, 0x90, 0xBF)
        else if 0xF1 ≤ c && c ≤ 0xF3 then (4, 0x80, 0xBF)
        else if c == 0xF4 then (4, 0x80, 0x8F)
        else (1, 0, 0)
      let mut k := 1
      let mut cp : Nat := if n == 2 then (c &&& 0x1F).toNat else if n == 3 then (c &&& 0x0F).toNat
        else (c &&& 0x07).toNat
      while k < n && i + k < b.size do
        let d := byteAt b (i + k)
        let ok := if k == 1 then lo ≤ d && d ≤ hi else 0x80 ≤ d && d ≤ 0xBF
        if !ok then break
        cp := cp * 64 + (d &&& 0x3F).toNat
        k := k + 1
      if n > 1 && k == n then s := s.push (Char.ofNat cp)
      else s := s.push '\uFFFD'
      i := i + k
  return s

@[inline] def b64Val (c : UInt8) : UInt32 :=
  if 65 ≤ c && c ≤ 90 then (c - 65).toUInt32
  else if 97 ≤ c && c ≤ 122 then (c - 71).toUInt32
  else if 48 ≤ c && c ≤ 57 then (c + 4).toUInt32
  else if c == 43 then 62 else if c == 47 then 63
  else if c == 61 then 64 else 65

/-- Decode `s[i..]` (state: `acc` holds `n` sextets; `pad` once `=` was seen). -/
partial def b64Loop (s out : ByteArray) (i : Nat) (acc n : UInt32) (pad : Bool) : Option ByteArray :=
  if h : i < s.size then
    let c := s[i]
    let v := b64Val c
    if v < 64 then
      if pad then none
      else if n == 3 then
        let a := (acc <<< 6) ||| v
        b64Loop s (((out.push (a >>> 16).toUInt8).push (a >>> 8).toUInt8).push a.toUInt8) (i + 1) 0 0 false
      else b64Loop s out (i + 1) ((acc <<< 6) ||| v) (n + 1) false
    else if v == 64 then b64Loop s out (i + 1) acc n true
    else if c == 10 || c == 13 || c == 32 || c == 9 then b64Loop s out (i + 1) acc n pad
    else none
  else if n == 2 then some (out.push (acc >>> 4).toUInt8)
  else if n == 3 then some ((out.push (acc >>> 10).toUInt8).push (acc >>> 2).toUInt8)
  else if n == 1 then none
  else some out

/-- Decode standard base64 (`=` padding; ASCII whitespace ignored). -/
def base64Decode (s : ByteArray) : Option ByteArray :=
  b64Loop s (ByteArray.emptyWithCapacity (s.size / 4 * 3 + 3)) 0 0 0 false

/-- FNV-1a 64 of `b[start, stop)`. -/
def fnv1a64 (b : ByteArray) (start stop : Nat) : UInt64 := Id.run do
  let mut h : UInt64 := 0xcbf29ce484222325
  for i in [start:stop] do
    h := (h ^^^ (byteAt b i).toUInt64) * 0x100000001b3
  return h

end Yt.Fmt
