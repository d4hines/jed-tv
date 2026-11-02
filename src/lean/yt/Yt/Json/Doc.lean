import Yt.Json.Fmt

/-! JSON over UTF-8 bytes, read in place.

`Doc.parse` validates the whole input (RFC 8259 grammar, UTF-8) in one
pass and records a *tape*: two `Nat`s per value, its start offset in the
source and either the tape index just past it (objects, arrays: so lookups
skip whole subtrees) or where it ends (strings, numbers). No string is
decoded and no tree is built: a ~1 MB YouTube response costs one array of
4 words per value. The accessors then decode only what is asked for
(`str?`, `u64?`, `get?`, ...). `Doc.toJson` materializes a `Json` tree when
a whole value is wanted.

Semantics, chosen to agree with Python's `json.loads` on bytes and with
serde_json where the app relies on it:
- strings: all escapes; `😀` pairs combine; a lone surrogate
  escape decodes to U+FFFD (Python keeps it as a lone surrogate, which
  UTF-8 cannot carry; serde_json rejects it);
- numbers keep their source text (`raw`, `Json.num`): exact. `u64?` is
  serde's `as_u64` (non-negative integer literal that fits in 64 bits);
- duplicate keys: `get?` returns the last, like Python and serde_json;
- a leading UTF-8 BOM is skipped (Python does that for bytes input);
- nesting deeper than `maxDepth` (1000) is rejected (Python: RecursionError);
- NaN/Infinity are rejected (Python accepts them by default). -/
namespace Yt.Json
open Yt.Fmt

/-- A value in a `Doc`: its index on the tape. -/
structure Val where
  i : Nat
  deriving Inhabited, BEq

inductive Kind where
  | obj | arr | str | num | bool | null
  deriving Inhabited, BEq

structure Doc where
  src : ByteArray
  /-- Value `k` is at `tape[2k]` (start offset) and `tape[2k+1]` (aux):
  object/array: index of the first value after it; string: 2 * offset of
  the closing quote + 1 if it has escapes; number: end offset; literal: 0. -/
  tape : Array Nat
  deriving Inhabited

def maxDepth : Nat := 1000

@[inline] def isWs (c : UInt8) : Bool := c == 32 || c == 10 || c == 13 || c == 9
@[inline] def isDigit (c : UInt8) : Bool := 48 ≤ c && c ≤ 57
@[inline] def isHex (c : UInt8) : Bool :=
  isDigit c || (97 ≤ c && c ≤ 102) || (65 ≤ c && c ≤ 70)

partial def skipWs (s : ByteArray) (i : Nat) : Nat :=
  if h : i < s.size then (if isWs s[i] then skipWs s (i + 1) else i) else i

/-- From inside a string at `i`: 2 * (offset of the closing quote) + 1 if an
escape was seen, or 0 if the string is malformed (unterminated, raw control
character, bad escape). -/
partial def scanStr (s : ByteArray) (i : Nat) (esc : Nat) : Nat :=
  if h : i < s.size then
    let c := s[i]
    if c == 34 then 2 * i + esc
    else if c == 92 then
      let e := byteAt s (i + 1)
      if e == 117 then
        if isHex (byteAt s (i + 2)) && isHex (byteAt s (i + 3)) && isHex (byteAt s (i + 4))
            && isHex (byteAt s (i + 5)) then scanStr s (i + 6) 1 else 0
      else if e == 34 || e == 92 || e == 47 || e == 98 || e == 102 || e == 110 || e == 114
          || e == 116 then scanStr s (i + 2) 1
      else 0
    else if c < 32 then 0
    else scanStr s (i + 1) esc
  else 0

partial def skipDigits (s : ByteArray) (i : Nat) : Nat :=
  if h : i < s.size then (if isDigit s[i] then skipDigits s (i + 1) else i) else i

/-- End of the number starting at `i`, or 0 if it is malformed. -/
def scanNum (s : ByteArray) (i : Nat) : Nat :=
  let j := if byteAt s i == 45 then i + 1 else i
  let c := byteAt s j
  if !isDigit c then 0 else
  let j := if c == 48 then j + 1 else skipDigits s j
  let j := if byteAt s j == 46 then
      (if isDigit (byteAt s (j + 1)) then skipDigits s (j + 1) else 0)
    else j
  if j == 0 then 0 else
  let c := byteAt s j
  if c == 101 || c == 69 then
    let k := if byteAt s (j + 1) == 43 || byteAt s (j + 1) == 45 then j + 2 else j + 1
    if isDigit (byteAt s k) then skipDigits s k else 0
  else j

/-- Does `s` continue at `i` with the bytes of `w`? -/
partial def litFrom (s w : ByteArray) (i k : Nat) : Bool :=
  if k ≥ w.size then true
  else if byteAt s (i + k) == byteAt w k then litFrom s w i (k + 1) else false

@[inline] def lit (s : ByteArray) (i : Nat) (w : String) : Bool :=
  i + w.utf8ByteSize ≤ s.size && litFrom s w.toUTF8 i 0

def errAt (i : Nat) (msg : String) : Except String (Array Nat) :=
  .error ("JSON: " ++ msg ++ " at byte " ++ toString i)

/-- The parser: one loop over the input. `st`: 0 value expected, 1 after a
value, 2 key expected, 3 key or `}` (after `{`), 4 value or `]` (after `[`).
`stack` holds the tape indices of the open containers. -/
partial def run (s : ByteArray) (i : Nat) (tape : Array Nat) (stack : Array Nat) (st : UInt8) :
    Except String (Array Nat) :=
  let i := skipWs s i
  if st == 1 then
    if stack.size == 0 then
      if i == s.size then .ok tape else errAt i "trailing characters"
    else
      let top := stack[stack.size - 1]!
      let isObj := byteAt s tape[2 * top]! == 123
      let c := byteAt s i
      if c == 44 then run s (i + 1) tape stack (if isObj then 2 else 0)
      else if (isObj && c == 125) || (!isObj && c == 93) then
        run s (i + 1) (tape.set! (2 * top + 1) (tape.size / 2)) stack.pop 1
      else if i ≥ s.size then errAt i "unexpected end of input"
      else errAt i (if isObj then "expected ',' or '}'" else "expected ',' or ']'")
  else if i ≥ s.size then errAt i "unexpected end of input"
  else
    let c := byteAt s i
    if st == 3 && c == 125 || st == 4 && c == 93 then
      let top := stack[stack.size - 1]!
      run s (i + 1) (tape.set! (2 * top + 1) (tape.size / 2)) stack.pop 1
    else if st == 2 || st == 3 then
      if c != 34 then errAt i "expected a string key" else
      let r := scanStr s (i + 1) 0
      if r == 0 then errAt i "malformed string" else
      let j := skipWs s (r / 2 + 1)
      if byteAt s j != 58 then errAt j "expected ':'" else
      run s (j + 1) ((tape.push i).push r) stack 0
    else if c == 34 then
      let r := scanStr s (i + 1) 0
      if r == 0 then errAt i "malformed string" else
      run s (r / 2 + 1) ((tape.push i).push r) stack 1
    else if c == 123 || c == 91 then
      if stack.size ≥ maxDepth then errAt i "nesting too deep" else
      -- Read the index from the pushed tape: reading `tape.size` after the
      -- push would keep the old tape alive and make the push copy it.
      let tape := (tape.push i).push 0
      run s (i + 1) tape (stack.push (tape.size / 2 - 1)) (if c == 123 then 3 else 4)
    else if c == 45 || isDigit c then
      let j := scanNum s i
      if j == 0 then errAt i "malformed number" else
      run s j ((tape.push i).push j) stack 1
    else if c == 116 && lit s i "true" then run s (i + 4) ((tape.push i).push 0) stack 1
    else if c == 102 && lit s i "false" then run s (i + 5) ((tape.push i).push 0) stack 1
    else if c == 110 && lit s i "null" then run s (i + 4) ((tape.push i).push 0) stack 1
    else errAt i "unexpected character"

/-- Validate and index `src` (UTF-8 JSON). -/
def Doc.parse (src : ByteArray) : Except String Doc :=
  if !ByteArray.validateUTF8 src then .error "JSON: invalid UTF-8" else
  let start := if byteAt src 0 == 0xEF && byteAt src 1 == 0xBB && byteAt src 2 == 0xBF then 3 else 0
  -- A rough capacity guess: one value per 8 bytes.
  match run src start (Array.emptyWithCapacity (src.size / 4)) #[] 0 with
  | .ok tape => .ok ⟨src, tape⟩
  | .error e => .error e

namespace Doc

/-- The top-level value. -/
@[inline] def root (_ : Doc) : Val := ⟨0⟩
@[inline] def start (d : Doc) (v : Val) : Nat := d.tape[2 * v.i]!
@[inline] def aux (d : Doc) (v : Val) : Nat := d.tape[2 * v.i + 1]!
/-- Number of values in the document. -/
@[inline] def count (d : Doc) : Nat := d.tape.size / 2

def kind (d : Doc) (v : Val) : Kind :=
  let c := byteAt d.src (d.start v)
  if c == 123 then .obj else if c == 91 then .arr else if c == 34 then .str
  else if c == 116 || c == 102 then .bool else if c == 110 then .null else .num

/-- The value after `v` and everything inside it. -/
@[inline] def next (d : Doc) (v : Val) : Val :=
  let c := byteAt d.src (d.start v)
  if c == 123 || c == 91 then ⟨d.aux v⟩ else ⟨v.i + 1⟩

/-- Offset just past value `v` in the source. -/
partial def endOf (d : Doc) (v : Val) : Nat :=
  let s := d.start v
  let c := byteAt d.src s
  if c == 34 then d.aux v / 2 + 1
  else if c == 123 || c == 91 then
    -- Past the last child (or the opening bracket), then whitespace, then
    -- the closing bracket.
    let stop := d.aux v
    let last := Id.run do
      let mut k := v.i + 1
      let mut last := v.i
      while k < stop do
        let val := if c == 123 then k + 1 else k
        last := val
        k := (d.next ⟨val⟩).i
      return last
    let p := if last == v.i then s + 1 else endOf d ⟨last⟩
    skipWs d.src p + 1
  else if c == 116 then s + 4 else if c == 102 then s + 5 else if c == 110 then s + 4
  else d.aux v

/-- The source text of `v`. -/
def raw (d : Doc) (v : Val) : ByteArray := d.src.extract (d.start v) (d.endOf v)

@[inline] def isNull (d : Doc) (v : Val) : Bool := byteAt d.src (d.start v) == 110

def bool? (d : Doc) (v : Val) : Option Bool :=
  let c := byteAt d.src (d.start v)
  if c == 116 then some true else if c == 102 then some false else none

/-- Push the UTF-8 encoding of code point `cp`. -/
def pushUtf8 (o : ByteArray) (cp : UInt32) : ByteArray :=
  let hi : UInt32 := 0x80
  let m : UInt32 := 0x3F
  if cp < 0x80 then o.push cp.toUInt8
  else if cp < 0x800 then (o.push ((0xC0 : UInt32) ||| (cp >>> 6)).toUInt8).push (hi ||| (cp &&& m)).toUInt8
  else if cp < 0x10000 then
    ((o.push ((0xE0 : UInt32) ||| (cp >>> 12)).toUInt8).push (hi ||| ((cp >>> 6) &&& m)).toUInt8).push
      (hi ||| (cp &&& m)).toUInt8
  else
    (((o.push ((0xF0 : UInt32) ||| (cp >>> 18)).toUInt8).push (hi ||| ((cp >>> 12) &&& m)).toUInt8).push
      (hi ||| ((cp >>> 6) &&& m)).toUInt8).push (hi ||| (cp &&& m)).toUInt8

@[inline] def hexVal (c : UInt8) : UInt32 :=
  if c ≤ 57 then (c - 48).toUInt32 else if c ≤ 70 then (c - 55).toUInt32 else (c - 87).toUInt32

@[inline] def hex4 (s : ByteArray) (i : Nat) : UInt32 :=
  (hexVal (byteAt s i) <<< 12) ||| (hexVal (byteAt s (i+1)) <<< 8) ||| (hexVal (byteAt s (i+2)) <<< 4)
  ||| hexVal (byteAt s (i+3))

/-- Decode the (validated) string body `s[i, e)` with escapes, appending to `o`. -/
partial def unescape (s : ByteArray) (i e : Nat) (o : ByteArray) : ByteArray :=
  if i ≥ e then o else
  -- Copy the run up to the next backslash in one slice.
  let j := Id.run do
    let mut j := i
    while j < e && byteAt s j != 92 do j := j + 1
    return j
  let o := if j > i then s.copySlice i o o.size (j - i) false else o
  if j ≥ e then o else
  let c := byteAt s (j + 1)
  if c == 117 then
    let u := hex4 s (j + 2)
    if 0xD800 ≤ u && u ≤ 0xDBFF && byteAt s (j + 6) == 92 && byteAt s (j + 7) == 117 then
      let l := hex4 s (j + 8)
      if 0xDC00 ≤ l && l ≤ 0xDFFF then
        unescape s (j + 12) e (pushUtf8 o ((0x10000 : UInt32) + ((u - 0xD800) <<< 10) + (l - 0xDC00)))
      else unescape s (j + 6) e (pushUtf8 o 0xFFFD)
    else if 0xD800 ≤ u && u ≤ 0xDFFF then unescape s (j + 6) e (pushUtf8 o 0xFFFD)
    else unescape s (j + 6) e (pushUtf8 o u)
  else
    let b : UInt8 := if c == 98 then 8 else if c == 102 then 12 else if c == 110 then 10
      else if c == 114 then 13 else if c == 116 then 9 else c
    unescape s (j + 2) e (o.push b)

/-- The UTF-8 bytes of string `v` (escapes decoded). -/
def bytes? (d : Doc) (v : Val) : Option ByteArray :=
  let s := d.start v
  if byteAt d.src s != 34 then none else
  let a := d.aux v
  if a % 2 == 0 then some (d.src.extract (s + 1) (a / 2))
  else some (unescape d.src (s + 1) (a / 2) (ByteArray.emptyWithCapacity (a / 2 - s)))

/-- String `v`. -/
def str? (d : Doc) (v : Val) : Option String :=
  match d.bytes? v with
  | some b => String.fromUTF8? b
  | none => none

/-- Is `v` the string `key` (UTF-8 bytes)? No allocation unless it has escapes. -/
def strEq (d : Doc) (v : Val) (key : ByteArray) : Bool :=
  let s := d.start v
  if byteAt d.src s != 34 then false else
  let a := d.aux v
  if a % 2 == 1 then (d.bytes? v).getD ByteArray.empty == key
  else if a / 2 - (s + 1) != key.size then false
  else Id.run do
    for k in [0:key.size] do
      if byteAt d.src (s + 1 + k) != byteAt key k then return false
    return true

/-- serde_json's `as_u64`: a non-negative integer literal (no fraction, no
exponent, no sign) that fits in 64 bits. -/
def u64? (d : Doc) (v : Val) : Option UInt64 :=
  let s := d.start v
  if !isDigit (byteAt d.src s) then none else
  let e := d.aux v
  Id.run do
    let mut x : Nat := 0
    for k in [s:e] do
      let c := byteAt d.src k
      if !isDigit c then return none
      x := x * 10 + (c - 48).toNat
      if x ≥ 18446744073709551616 then return none
    return some x.toUInt64

/-- Members of object `v` as (key, value) pairs, in document order. -/
def members (d : Doc) (v : Val) : Array (Val × Val) := Id.run do
  if byteAt d.src (d.start v) != 123 then return #[]
  let stop := d.aux v
  let mut k := v.i + 1
  let mut out := #[]
  while k < stop do
    let key : Val := ⟨k⟩
    let val : Val := ⟨k + 1⟩
    out := out.push (key, val)
    k := (d.next val).i
  return out

/-- Elements of array `v`, in order. -/
def elems (d : Doc) (v : Val) : Array Val := Id.run do
  if byteAt d.src (d.start v) != 91 then return #[]
  let stop := d.aux v
  let mut k := v.i + 1
  let mut out := #[]
  while k < stop do
    out := out.push ⟨k⟩
    k := (d.next ⟨k⟩).i
  return out

/-- Number of elements (array) or members (object); 0 otherwise. -/
def size (d : Doc) (v : Val) : Nat := Id.run do
  let c := byteAt d.src (d.start v)
  if c != 123 && c != 91 then return 0
  let stop := d.aux v
  let mut k := v.i + 1
  let mut n := 0
  while k < stop do
    n := n + 1
    k := if c == 123 then (d.next ⟨k + 1⟩).i else (d.next ⟨k⟩).i
  return n

/-- The last member of the object ending before tape index `stop`, from
`k` on, whose key is `key`. (A recursive function, not a `while` with two
mutable variables, which would allocate a tuple per member.) -/
partial def findKey (d : Doc) (key : ByteArray) (stop k : Nat) (found : Option Val) : Option Val :=
  if k ≥ stop then found
  else findKey d key stop (d.next ⟨k + 1⟩).i (if d.strEq ⟨k⟩ key then some ⟨k + 1⟩ else found)

/-- Member `key` of object `v` (the last one if repeated). -/
def getB? (d : Doc) (v : Val) (key : ByteArray) : Option Val :=
  if byteAt d.src (d.start v) != 123 then none else findKey d key (d.aux v) (v.i + 1) none

@[inline] def get? (d : Doc) (v : Val) (key : String) : Option Val := d.getB? v key.toUTF8

/-- Element `n` of array `v`. -/
def at? (d : Doc) (v : Val) (n : Nat) : Option Val := Id.run do
  if byteAt d.src (d.start v) != 91 then return none
  let stop := d.aux v
  let mut k := v.i + 1
  let mut j := 0
  while k < stop do
    if j == n then return some ⟨k⟩
    j := j + 1
    k := (d.next ⟨k⟩).i
  return none

/-- Follow object keys from `v`. -/
def path? (d : Doc) (v : Val) (keys : Array String) : Option Val := Id.run do
  let mut cur := v
  for k in keys do
    match d.get? cur k with
    | some x => cur := x
    | none => return none
  return some cur

end Doc
end Yt.Json
