import Yt.Crypto.Util
/-! A strict DER reader over a `ByteArray` (offsets, no copies): single-byte
tags, definite minimal lengths, minimal INTEGER encodings. Enough for
X.509 certificates, SubjectPublicKeyInfo and ECDSA signatures. -/
namespace Yt.X509.Der
open Yt.Crypto

/-- One element: its tag, the extent of its contents, and the offset just
past it. -/
structure Tlv where
  tag : UInt8
  off : Nat
  len : Nat
  stop : Nat
deriving Inhabited

namespace Tag
def boolean : UInt8 := 0x01
def integer : UInt8 := 0x02
def bitString : UInt8 := 0x03
def octetString : UInt8 := 0x04
def null : UInt8 := 0x05
def oid : UInt8 := 0x06
def utf8String : UInt8 := 0x0c
def printableString : UInt8 := 0x13
def ia5String : UInt8 := 0x16
def utcTime : UInt8 := 0x17
def generalizedTime : UInt8 := 0x18
def sequence : UInt8 := 0x30
def set : UInt8 := 0x31
/-- `[n]` constructed, context-specific. -/
def ctx (n : UInt8) : UInt8 := 0xa0 ||| n
/-- `[n]` primitive, context-specific. -/
def ctxPrim (n : UInt8) : UInt8 := 0x80 ||| n
end Tag

/-- The element starting at `pos`, which must end by `lim`. -/
def read (b : ByteArray) (pos lim : Nat) : Option Tlv := Id.run do
  if pos + 2 > lim || lim > b.size then return none
  let tag := b[pos]!
  if tag &&& 0x1f == 0x1f then return none
  let l0 := b[pos + 1]!
  if l0 < 0x80 then
    let off := pos + 2
    if off + l0.toNat > lim then return none
    return some ⟨tag, off, l0.toNat, off + l0.toNat⟩
  let nb := (l0 &&& 0x7f).toNat
  if nb == 0 || nb > 4 || pos + 2 + nb > lim then return none
  if b[pos + 2]! == 0 then return none
  let mut len := 0
  for i in [0:nb] do len := len * 256 + b[pos + 2 + i]!.toNat
  if len < 0x80 then return none
  let off := pos + 2 + nb
  if off + len > lim then return none
  return some ⟨tag, off, len, off + len⟩

/-- The element at `pos` if it has tag `t`. -/
def expect (b : ByteArray) (pos lim : Nat) (t : UInt8) : Option Tlv :=
  match read b pos lim with
  | some e => if e.tag == t then some e else none
  | none => none

/-- The whole encoding of an element (tag and length included). -/
def raw (b : ByteArray) (pos : Nat) (e : Tlv) : ByteArray := slice b pos (e.stop - pos)

def contents (b : ByteArray) (e : Tlv) : ByteArray := slice b e.off e.len

/-- A nonnegative INTEGER's magnitude, big-endian without the sign byte;
`none` for negative or non-minimal encodings. -/
def uint (b : ByteArray) (e : Tlv) : Option ByteArray := Id.run do
  if e.tag != Tag.integer || e.len == 0 then return none
  let b0 := b[e.off]!
  if b0 ≥ 0x80 then return none
  if e.len > 1 && b0 == 0 && b[e.off + 1]! < 0x80 then return none
  if b0 == 0 then return some (slice b (e.off + 1) (e.len - 1))
  return some (contents b e)

/-- A BIT STRING with no unused bits: its bytes. -/
def bits (b : ByteArray) (e : Tlv) : Option ByteArray :=
  if e.tag != Tag.bitString || e.len == 0 || b[e.off]! != 0 then none
  else some (slice b (e.off + 1) (e.len - 1))

/-- Elements of a constructed value, in order (`none` if any is malformed). -/
def children (b : ByteArray) (e : Tlv) : Option (Array (Nat × Tlv)) := Id.run do
  let mut out := #[]
  let mut p := e.off
  let lim := e.off + e.len
  while p < lim do
    match read b p lim with
    | some c =>
      out := out.push (p, c)
      p := c.stop
    | none => return none
  return some out

/-- An ECDSA-Sig-Value `SEQUENCE { r INTEGER, s INTEGER }` filling all of `sig`. -/
def ecdsaSig (sig : ByteArray) : Option (ByteArray × ByteArray) := do
  let sq ← expect sig 0 sig.size Tag.sequence
  if sq.stop != sig.size then none
  let re ← expect sig sq.off sq.stop Tag.integer
  let se ← expect sig re.stop sq.stop Tag.integer
  if se.stop != sq.stop then none
  let r ← uint sig re
  let s ← uint sig se
  some (r, s)

end Yt.X509.Der
