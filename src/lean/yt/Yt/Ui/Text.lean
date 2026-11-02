import Yt.Platform
import Yt.Ui.FontData

/-! Text for the UI: glyph blobs rasterized at build time
(`tools/make-font.py`, layout documented there), UTF-8 decoding,
measuring, ellipsizing and drawing through the display's alpha-mask
primitive. Text is UTF-8 bytes (`ByteArray`); see RULES.md. -/
namespace Yt.Ui
open Yt.Platform.Tv

/-! ### Base64 -/

@[inline] def b64Value (c : UInt8) : UInt32 :=
  if c ≥ 65 && c ≤ 90 then (c - 65).toUInt32
  else if c ≥ 97 && c ≤ 122 then (c - 71).toUInt32
  else if c ≥ 48 && c ≤ 57 then (c + 4).toUInt32
  else if c == 43 then 62 else if c == 47 then 63 else 64

/-- Decode standard base64 (padding and anything else ignored). -/
def base64Decode (s : ByteArray) : ByteArray := Id.run do
  let mut out := ByteArray.emptyWithCapacity (s.size / 4 * 3)
  let mut acc : UInt32 := 0
  let mut bits : UInt32 := 0
  for c in s do
    let v := b64Value c
    if v < 64 then
      acc := (acc <<< 6) ||| v
      bits := bits + 6
      if bits ≥ 8 then
        bits := bits - 8
        out := out.push (acc >>> bits).toUInt8
        acc := acc &&& ((1 <<< bits) - 1)
  out

/-! ### UTF-8 -/

/-- The code point at byte `i` of `s` and the byte after it (malformed
bytes decode as U+FFFD, one byte at a time). -/
def utf8At (s : ByteArray) (i : Nat) : UInt32 × Nat :=
  let b0 := s[i]!.toUInt32
  let cont (k : Nat) : UInt32 := s[i + k]!.toUInt32 &&& 0x3F
  let ok (k : Nat) : Bool := i + k < s.size && s[i + k]! &&& 0xC0 == 0x80
  if b0 < 0x80 then (b0, i + 1)
  else if b0 &&& 0xE0 == 0xC0 && ok 1 then (((b0 &&& 0x1F) <<< 6) ||| cont 1, i + 2)
  else if b0 &&& 0xF0 == 0xE0 && ok 1 && ok 2 then
    (((b0 &&& 0x0F) <<< 12) ||| (cont 1 <<< 6) ||| cont 2, i + 3)
  else if b0 &&& 0xF8 == 0xF0 && ok 1 && ok 2 && ok 3 then
    (((b0 &&& 0x07) <<< 18) ||| (cont 1 <<< 12) ||| (cont 2 <<< 6) ||| cont 3, i + 4)
  else (0xFFFD, i + 1)

/-! ### Fonts -/

/-- A decoded glyph blob. -/
structure Font where
  blob : ByteArray
  count : Nat
  size : Nat
  ascent : Nat
  lineHeight : Nat

@[inline] def u16le (b : ByteArray) (i : Nat) : Nat := b[i]!.toNat ||| (b[i+1]!.toNat <<< 8)
@[inline] def u32le (b : ByteArray) (i : Nat) : Nat :=
  u16le b i ||| (u16le b (i + 2) <<< 16)
/-- `x` plus the signed byte at `i` of `b`, if not negative. -/
@[inline] def addSigned (x : Nat) (b : ByteArray) (i : Nat) : Option Nat :=
  let v := b[i]!.toNat
  if v < 128 then some (x + v) else if x ≥ 256 - v then some (x - (256 - v)) else none

def Font.ofBase64 (s : String) : Font :=
  let b := base64Decode s.toUTF8
  { blob := b, count := u32le b 4, size := u16le b 8, ascent := u16le b 10, lineHeight := u16le b 12 }

/-- Header bytes before the records, and the record size. -/
def headerSize : Nat := 16
def recordSize : Nat := 16

@[inline] def Font.recordAt (f : Font) (k : Nat) : Nat := headerSize + recordSize * k
@[inline] def Font.masksAt (f : Font) : Nat := headerSize + recordSize * f.count

/-- The record index of code point `cp` in `[lo, hi)` (binary search). -/
partial def Font.search (f : Font) (cp lo hi : Nat) : Option Nat :=
  if lo ≥ hi then none
  else
    let mid := (lo + hi) / 2
    let c := u32le f.blob (f.recordAt mid)
    if c == cp then some mid
    else if c < cp then f.search cp (mid + 1) hi else f.search cp lo mid

/-- The record of `cp`, or of '?' if the font lacks it. -/
def Font.find (f : Font) (cp : UInt32) : Nat :=
  match f.search cp.toNat 0 f.count with
  | some k => f.recordAt k
  | none => f.recordAt ((f.search 63 0 f.count).getD 0)

@[inline] def Font.advance (f : Font) (r : Nat) : Nat := f.blob[r + 12]!.toNat

/-- Width in pixels of `s` (UTF-8). -/
partial def Font.measure (f : Font) (s : ByteArray) (i : Nat := 0) (w : Nat := 0) : Nat :=
  if i ≥ s.size then w
  else
    let (cp, j) := utf8At s i
    f.measure s j (w + f.advance (f.find cp))

/-- Draw `s` with its line top at `y`; the x after the last glyph. Glyphs
past `maxX` are not drawn. -/
partial def Font.draw (f : Font) (d : Display) (x y : Nat) (s : ByteArray) (rgb : UInt32)
    (maxX : Nat := 100000) (i : Nat := 0) : BaseIO Nat := do
  if i ≥ s.size then return x
  let (cp, j) := utf8At s i
  let r := f.find cp
  let adv := f.advance r
  if x + adv > maxX then return x
  let w := f.blob[r + 8]!.toNat
  let h := f.blob[r + 9]!.toNat
  if w > 0 && h > 0 then
    if let (some gx, some gy) := (addSigned x f.blob (r + 10), addSigned y f.blob (r + 11)) then
      d.blitMask gx gy f.blob (f.masksAt + u32le f.blob (r + 4)) w h w rgb
  f.draw d (x + adv) y s rgb maxX j

/-- `s`, shortened with "…" to fit `width` pixels. -/
def Font.ellipsize (f : Font) (s : ByteArray) (width : Nat) : ByteArray := Id.run do
  if f.measure s ≤ width then return s
  let dots : ByteArray := ⟨#[0xE2, 0x80, 0xA6]⟩
  let budget := width - min width (f.measure dots)
  let mut i := 0
  let mut w := 0
  while i < s.size do
    let (cp, j) := utf8At s i
    let a := f.advance (f.find cp)
    if w + a > budget then break
    w := w + a
    i := j
  return s.extract 0 i ++ dots

/-- The UI's fonts, decoded once at startup. -/
structure Fonts where
  small : Font
  body : Font
  large : Font

def Fonts.load : Fonts :=
  { small := Font.ofBase64 FontData.sans22, body := Font.ofBase64 FontData.sans28,
    large := Font.ofBase64 FontData.sans40 }

end Yt.Ui
