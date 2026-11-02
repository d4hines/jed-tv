/-! Byte helpers for the crypto, X.509 and TLS libraries. Everything here
links against our runtime (../../RULES.md): plain loops over `ByteArray`
and scalars, no standard-library calls that are not inlined. -/
namespace Yt.Crypto

/-- Byte `i` of `b`, or 0 past the end (no panic). -/
@[inline] def getD (b : ByteArray) (i : Nat) : UInt8 := if i < b.size then b[i]! else 0

/-- `n` zero bytes. -/
def zeros (n : Nat) : ByteArray := Id.run do
  let mut b := ByteArray.emptyWithCapacity n
  for _ in [0:n] do b := b.push 0
  b

/-- Bytes `off` up to `off + len` of `b` (zero-padded past the end). -/
def slice (b : ByteArray) (off len : Nat) : ByteArray :=
  if off + len ≤ b.size then b.copySlice off (ByteArray.emptyWithCapacity len) 0 len
  else Id.run do
    let mut r := ByteArray.emptyWithCapacity len
    for i in [0:len] do r := r.push (getD b (off + i))
    r

@[inline] def getU32BE (b : ByteArray) (i : Nat) : UInt32 :=
  (b[i]!.toUInt32 <<< 24) ||| (b[i+1]!.toUInt32 <<< 16) ||| (b[i+2]!.toUInt32 <<< 8) ||| b[i+3]!.toUInt32

@[inline] def getU32LE (b : ByteArray) (i : Nat) : UInt32 :=
  b[i]!.toUInt32 ||| (b[i+1]!.toUInt32 <<< 8) ||| (b[i+2]!.toUInt32 <<< 16) ||| (b[i+3]!.toUInt32 <<< 24)

@[inline] def getU64BE (b : ByteArray) (i : Nat) : UInt64 :=
  ((getU32BE b i).toUInt64 <<< 32) ||| (getU32BE b (i+4)).toUInt64

@[inline] def getU16BE (b : ByteArray) (i : Nat) : Nat :=
  (b[i]!.toNat <<< 8) ||| b[i+1]!.toNat

@[inline] def pushU32BE (b : ByteArray) (x : UInt32) : ByteArray :=
  b.push (x >>> 24).toUInt8 |>.push (x >>> 16).toUInt8 |>.push (x >>> 8).toUInt8 |>.push x.toUInt8

@[inline] def pushU32LE (b : ByteArray) (x : UInt32) : ByteArray :=
  b.push x.toUInt8 |>.push (x >>> 8).toUInt8 |>.push (x >>> 16).toUInt8 |>.push (x >>> 24).toUInt8

@[inline] def pushU64BE (b : ByteArray) (x : UInt64) : ByteArray :=
  pushU32BE (pushU32BE b (x >>> 32).toUInt32) x.toUInt32

@[inline] def pushU16BE (b : ByteArray) (x : Nat) : ByteArray :=
  b.push (x >>> 8).toUInt8 |>.push x.toUInt8

@[inline] def pushU24BE (b : ByteArray) (x : Nat) : ByteArray :=
  b.push (x >>> 16).toUInt8 |>.push (x >>> 8).toUInt8 |>.push x.toUInt8

/-- Big-endian encoding of `x` in `len` bytes (`len ≤ 8`). -/
def beBytes (x : UInt64) (len : Nat) : ByteArray := Id.run do
  let mut r := ByteArray.emptyWithCapacity len
  for i in [0:len] do
    r := r.push (x >>> (8 * (len - 1 - i)).toUInt64).toUInt8
  r

/-- `a xor b` over the length of `a` (`b` zero-padded). -/
def xorBytes (a b : ByteArray) : ByteArray := Id.run do
  let mut r := a
  for i in [0:a.size] do r := r.set! i (a[i]! ^^^ getD b i)
  r

/-- Equality that inspects every byte (no early exit on the contents). -/
def ctEq (a b : ByteArray) : Bool := Id.run do
  if a.size != b.size then return false
  let mut d : UInt8 := 0
  for i in [0:a.size] do d := d ||| (a[i]! ^^^ b[i]!)
  d == 0

/-- Byte equality of `a[aOff, aOff+len)` and `b[bOff, bOff+len)`. -/
def bytesEqAt (a : ByteArray) (aOff : Nat) (b : ByteArray) (bOff len : Nat) : Bool := Id.run do
  if a.size < aOff + len || b.size < bOff + len then return false
  for i in [0:len] do
    if a[aOff + i]! != b[bOff + i]! then return false
  true

@[inline] def hexDigit (d : UInt8) : Char :=
  Char.ofNat (if d < 10 then 48 + d.toNat else 87 + d.toNat)

/-- Lower-case hexadecimal. -/
def toHex (b : ByteArray) : String := Id.run do
  let mut s := ""
  for x in b do
    s := (s.push (hexDigit (x >>> 4))).push (hexDigit (x &&& 15))
  s

@[inline] def hexVal (c : UInt8) : UInt8 :=
  if c ≥ 48 && c ≤ 57 then c - 48
  else if c ≥ 97 && c ≤ 102 then c - 87
  else if c ≥ 65 && c ≤ 70 then c - 55
  else 255

/-- Parse hexadecimal, skipping anything that is not a hex digit
(whitespace, colons). An odd trailing digit is dropped. -/
def ofHex (s : String) : ByteArray := Id.run do
  let u := s.toUTF8
  let mut r := ByteArray.emptyWithCapacity (u.size / 2)
  let mut hi : UInt8 := 0
  let mut pending : Bool := false
  for c in u do
    let v := hexVal c
    if v != 255 then
      if pending then
        r := r.push ((hi <<< 4) ||| v)
        pending := false
      else
        hi := v
        pending := true
  r

/-- The ASCII bytes of a string (its UTF-8 encoding). -/
@[inline] def ascii (s : String) : ByteArray := s.toUTF8

end Yt.Crypto
