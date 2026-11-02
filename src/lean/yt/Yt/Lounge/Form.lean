import Yt.Json.Fmt

/-! Text helpers for the Lounge protocol: `application/x-www-form-urlencoded`
encoding exactly as Python's `urllib.parse.urlencode` (what the host
prototype `research/dial-lounge/lounge.py` sends), decimal seconds <-> integer
milliseconds (no floating point on our runtime), and small validators.
Everything is pure and links against our runtime (../../RULES.md). -/
namespace Yt.Lounge
open Yt.Fmt

@[inline] def hexUpper (n : UInt8) : UInt8 := if n < 10 then 48 + n else 55 + n

/-- `quote_plus` with no safe characters: ASCII letters, digits and `_.-~`
stay, space becomes `+`, every other byte becomes `%XX` (upper case). -/
def pctEncodeInto (o : ByteArray) (s : String) : ByteArray := Id.run do
  let mut o := o
  for c in s.toUTF8 do
    if (48 ≤ c && c ≤ 57) || (65 ≤ c && c ≤ 90) || (97 ≤ c && c ≤ 122) || c == 95 || c == 46
        || c == 45 || c == 126 then
      o := o.push c
    else if c == 32 then o := o.push 43
    else
      o := ((o.push 37).push (hexUpper (c >>> 4))).push (hexUpper (c &&& 15))
  return o

def pctEncode (s : String) : String := str (pctEncodeInto ByteArray.empty s)

/-- `k1=v1&k2=v2...`, each side percent-encoded. -/
def formEncode (kvs : Array (String × String)) : String := Id.run do
  let mut o := ByteArray.emptyWithCapacity 256
  let mut first := true
  for (k, v) in kvs do
    if !first then o := o.push 38
    first := false
    o := pctEncodeInto o k
    o := o.push 61
    o := pctEncodeInto o v
  return str o

/-- Decimal digits of a `Nat` (small values: ids, counters). -/
def natStr (n : Nat) : String := dec n.toUInt64

/-- A non-negative decimal integer, or none. -/
def parseNat? (s : String) : Option Nat := Id.run do
  let b := s.toUTF8
  if b.size == 0 || b.size > 18 then return none
  let mut x : Nat := 0
  for c in b do
    if c < 48 || c > 57 then return none
    x := x * 10 + (c - 48).toNat
  return some x

/-- Seconds as sent by the phone (`"12"`, `"12.5"`, `"0.000"`) to integer
milliseconds. Digits past the third decimal are dropped; a sign, exponent or
anything else gives none. -/
def parseSecondsMs? (s : String) : Option UInt64 := Id.run do
  let b := s.toUTF8
  if b.size == 0 || b.size > 24 then return none
  let mut whole : UInt64 := 0
  let mut frac : UInt64 := 0
  let mut fracDigits : Nat := 0
  let mut seenDot := false
  let mut digits := 0
  for c in b do
    if c == 46 then
      if seenDot then return none
      seenDot := true
    else if c < 48 || c > 57 then return none
    else
      digits := digits + 1
      if seenDot then
        if fracDigits < 3 then
          frac := frac * 10 + (c - 48).toUInt64
          fracDigits := fracDigits + 1
      else
        whole := whole * 10 + (c - 48).toUInt64
  if digits == 0 then return none
  let scale : UInt64 := if fracDigits == 0 then 1000 else if fracDigits == 1 then 100
    else if fracDigits == 2 then 10 else 1
  return some (whole * 1000 + frac * scale)

/-- Milliseconds as decimal seconds with three decimals: 12345 -> "12.345". -/
def secondsStr (ms : UInt64) : String :=
  let f := ms % 1000
  let pad := if f < 10 then "00" else if f < 100 then "0" else ""
  dec (ms / 1000) ++ "." ++ pad ++ dec f

/-- Only ASCII letters, digits, `-` and `_`, 1 to `max` bytes: screen IDs,
pairing codes, video IDs (anything we put back into a request or hand to
the app). -/
def isToken (s : String) (max : Nat) : Bool := Id.run do
  let b := s.toUTF8
  if b.size == 0 || b.size > max then return false
  for c in b do
    if !((48 ≤ c && c ≤ 57) || (65 ≤ c && c ≤ 90) || (97 ≤ c && c ≤ 122) || c == 45 || c == 95) then
      return false
  return true

/-- `b` without leading and trailing ASCII whitespace, as a String. -/
def trimmed (b : ByteArray) : String := Id.run do
  let mut i := 0
  let mut j := b.size
  while i < j && (byteAt b i == 32 || byteAt b i == 10 || byteAt b i == 13 || byteAt b i == 9) do
    i := i + 1
  while j > i && (byteAt b (j - 1) == 32 || byteAt b (j - 1) == 10 || byteAt b (j - 1) == 13
      || byteAt b (j - 1) == 9) do
    j := j - 1
  return str (b.extract i j)

/-- Split on one byte (ASCII separator), keeping empty pieces out. -/
def splitOn (s : String) (sep : UInt8) : Array String := Id.run do
  let b := s.toUTF8
  let mut out : Array String := #[]
  let mut start := 0
  for i in [0:b.size + 1] do
    if i == b.size || byteAt b i == sep then
      if i > start then out := out.push (str (b.extract start i))
      start := i + 1
  return out

/-- A version-4 UUID string from 16 bytes of entropy (fewer are zero-padded):
for a fresh `Identity.deviceId`. -/
def uuidFromBytes (e : ByteArray) : String := Id.run do
  let hex := "0123456789abcdef".toUTF8
  let mut o := ByteArray.emptyWithCapacity 36
  for i in [0:16] do
    let b0 := byteAt e i
    let b := if i == 6 then (b0 &&& 0x0F) ||| 0x40 else if i == 8 then (b0 &&& 0x3F) ||| 0x80 else b0
    if i == 4 || i == 6 || i == 8 || i == 10 then o := o.push 45
    o := (o.push (byteAt hex (b >>> 4).toNat)).push (byteAt hex (b &&& 15).toNat)
  return str o

end Yt.Lounge
