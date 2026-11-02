import Yt.Aac.Bits
import Yt.Aac.HuffData

/-! Two-level Huffman lookup tables for the AAC codebooks.

A table indexes the next `bits` bits of the stream. Entries are 32-bit words:
a leaf is `(length <<< 16) ||| symbol`; a pointer to a second-level table is
`0x80000000 ||| (offset <<< 8) ||| subBits`, and that table's leaves store the
length beyond the first `bits`. An all-zero entry is not a codeword. -/

namespace Yt.Aac

structure HuffTable where
  bits : UInt32
  data : ByteArray
  deriving Inhabited

namespace HuffTable

private def maxLen (lens : Array Nat) : Nat :=
  lens.foldl (fun m l => if l > m then l else m) 0

private partial def fill (t : ByteArray) (i n : USize) (v : UInt32) : ByteArray :=
  if n == 0 then t else fill (st32 t i v) (i + 1) (n - 1) v

/-- Build the table for codewords `codes[i]` of length `lens[i]` (symbol `i`). -/
def build (lens codes : Array Nat) (primary : Nat := 10) : HuffTable := Id.run do
  let n := lens.size
  let pb := min primary (maxLen lens)
  let pbU := pb.toUInt32
  let psize : UInt32 := (1 : UInt32) <<< pbU
  -- Longest code per first-level prefix, for codes longer than `pb`.
  let mut gmax := zeros32 psize.toNat
  for i in [0:n] do
    let l := (lens[i]!).toUInt32
    let c := (codes[i]!).toUInt32
    if l > pbU then
      let p := (c >>> (l - pbU)).toUSize
      if l > ld32 gmax p then gmax := st32 gmax p l
  -- Second-level table offsets.
  let mut off := zeros32 psize.toNat
  let mut total : UInt32 := psize
  for p in [0:psize.toNat] do
    let g := ld32 gmax p.toUSize
    if g > 0 then
      off := st32 off p.toUSize total
      total := total + ((1 : UInt32) <<< (g - pbU))
  let mut t := zeros32 total.toNat
  for p in [0:psize.toNat] do
    let g := ld32 gmax p.toUSize
    if g > 0 then
      t := st32 t p.toUSize ((0x80000000 : UInt32) ||| (ld32 off p.toUSize <<< 8) ||| (g - pbU))
  for i in [0:n] do
    let l := (lens[i]!).toUInt32
    let c := (codes[i]!).toUInt32
    let sym := i.toUInt32
    if l ≤ pbU then
      t := fill t (c <<< (pbU - l)).toUSize ((1 : UInt32) <<< (pbU - l)).toUSize ((l <<< 16) ||| sym)
    else
      let p := (c >>> (l - pbU)).toUSize
      let sb := ld32 gmax p - pbU
      let rest := l - pbU
      let suf := c &&& (((1 : UInt32) <<< rest) - 1)
      let lo := ld32 off p + (suf <<< (sb - rest))
      t := fill t lo.toUSize ((1 : UInt32) <<< (sb - rest)).toUSize ((rest <<< 16) ||| sym)
  return { bits := pbU, data := t }

/-- Decode the codeword at bit `pos`: `(length <<< 16) ||| symbol`, or 0 when
the bits are not a codeword. -/
@[inline] def decode (t : HuffTable) (d : ByteArray) (pos : USize) : UInt32 :=
  let w := peek32 d pos
  let e := ld32 t.data (w >>> (32 - t.bits)).toUSize
  if e &&& 0x80000000 == 0 then e
  else
    let sb := e &&& 0xFF
    let off := (e >>> 8) &&& 0x7FFFFF
    let e2 := ld32 t.data (off + ((w <<< t.bits) >>> (32 - sb))).toUSize
    if e2 == 0 then 0 else e2 + (t.bits <<< 16)

end HuffTable

/-- All AAC codebooks: `spec[i]` is spectral codebook `i + 1`, `scf` the
scalefactor codebook. -/
structure Books where
  spec : Array HuffTable
  scf : HuffTable
  deriving Inhabited

open HuffData in
/-- The codebooks, built on first use. -/
def books (_ : Unit) : Books :=
  { spec := #[HuffTable.build cb1Len cb1Code, HuffTable.build cb2Len cb2Code,
              HuffTable.build cb3Len cb3Code, HuffTable.build cb4Len cb4Code,
              HuffTable.build cb5Len cb5Code, HuffTable.build cb6Len cb6Code,
              HuffTable.build cb7Len cb7Code, HuffTable.build cb8Len cb8Code,
              HuffTable.build cb9Len cb9Code, HuffTable.build cb10Len cb10Code,
              HuffTable.build cb11Len cb11Code]
    scf := HuffTable.build scfLen scfCode }

end Yt.Aac
