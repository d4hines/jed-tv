/-!
# Pixel formats and blending: specification

Colours travel through a GPU pipeline packed into 32-bit words.  This
file defines:

* the common `0xAARRGGBB` packing of an 8-bit-per-channel colour into a
  `UInt32` and its inverse;
* "panel layouts": the display panel wants red, green and blue at given
  bit offsets, which differ between panels (RGB vs BGR);
* 8-bit "source over" alpha blending, one channel at a time.

The proofs are in `Gpu.Pixel.Proofs`.
-/

namespace Gpu.Pixel

/-- A colour with 8-bit red, green, blue and alpha (opacity) channels. -/
structure Rgba where
  r : UInt8
  g : UInt8
  b : UInt8
  a : UInt8
deriving Repr, DecidableEq

/-- A colour without alpha, as a display panel receives it. -/
structure Rgb where
  r : UInt8
  g : UInt8
  b : UInt8
deriving Repr, DecidableEq

/-! ## Packing as 0xAARRGGBB -/

/-- Pack a colour into one 32-bit word in `0xAARRGGBB` order: alpha in
the top byte, then red, green, and blue in the bottom byte. -/
def pack (c : Rgba) : UInt32 :=
  (c.a.toUInt32 <<< 24) ||| (c.r.toUInt32 <<< 16) ||| (c.g.toUInt32 <<< 8) ||| c.b.toUInt32

/-- Unpack a `0xAARRGGBB` word back into its four channels. -/
def unpack (v : UInt32) : Rgba :=
  ⟨(v >>> 16).toUInt8, (v >>> 8).toUInt8, v.toUInt8, (v >>> 24).toUInt8⟩

/-! ## Panel layouts -/

/-- Where a panel wants each colour channel inside a 32-bit word: the bit
offset of red, green and blue.  Each channel occupies 8 bits from its offset. -/
structure PanelLayout where
  redShift : UInt32
  greenShift : UInt32
  blueShift : UInt32
deriving Repr, DecidableEq

/-- The usual layout: red at bit 16, green at 8, blue at 0 (`0x00RRGGBB`). -/
def rgbLayout : PanelLayout := ⟨16, 8, 0⟩

/-- The swapped layout some panels use: blue at bit 16, green at 8, red at 0 (`0x00BBGGRR`). -/
def bgrLayout : PanelLayout := ⟨0, 8, 16⟩

/-- Put a colour into a panel word according to `L`. -/
def toPanel (L : PanelLayout) (c : Rgb) : UInt32 :=
  (c.r.toUInt32 <<< L.redShift) ||| (c.g.toUInt32 <<< L.greenShift) ||| (c.b.toUInt32 <<< L.blueShift)

/-- Read a colour out of a panel word according to `L`. -/
def fromPanel (L : PanelLayout) (v : UInt32) : Rgb :=
  ⟨(v >>> L.redShift).toUInt8, (v >>> L.greenShift).toUInt8, (v >>> L.blueShift).toUInt8⟩

/-- Drop the alpha channel. -/
def Rgba.rgb (c : Rgba) : Rgb := ⟨c.r, c.g, c.b⟩

/-- Convert a packed `0xAARRGGBB` word to a panel word under layout `L`. -/
def packedToPanel (L : PanelLayout) (v : UInt32) : UInt32 :=
  toPanel L (unpack v).rgb

/-! ## Source-over blending in 8 bits -/

/-- One channel of "source over" blending with 8-bit alpha `a`, in plain
natural numbers:  `(src * a + dst * (255 - a) + 127) / 255`.
The `+ 127` rounds to nearest.  Inputs are meant to be `≤ 255`. -/
def blendNat (src dst a : Nat) : Nat :=
  (src * a + dst * (255 - a) + 127) / 255

/-- The same blend on 8-bit channels.  The intermediate sum is computed
in `Nat` (so there is no overflow); `blendNat_le_255` shows the result
fits back into 8 bits without wrapping. -/
def blendChannel (src dst a : UInt8) : UInt8 :=
  UInt8.ofNat (blendNat src.toNat dst.toNat a.toNat)

/-- Blend a source colour over a destination colour using the source alpha,
channel by channel.  The result keeps the destination alpha. -/
def blendOver (src dst : Rgba) : Rgba :=
  ⟨blendChannel src.r dst.r src.a, blendChannel src.g dst.g src.a,
   blendChannel src.b dst.b src.a, dst.a⟩

/-! ## Unit tests -/

#guard pack ⟨0x12, 0x34, 0x56, 0x78⟩ == 0x78123456
#guard unpack 0x78123456 == ⟨0x12, 0x34, 0x56, 0x78⟩
#guard unpack (pack ⟨1, 2, 3, 4⟩) == ⟨1, 2, 3, 4⟩
#guard pack (unpack 0xDEADBEEF) == 0xDEADBEEF

#guard toPanel rgbLayout ⟨0x12, 0x34, 0x56⟩ == 0x123456
#guard toPanel bgrLayout ⟨0x12, 0x34, 0x56⟩ == 0x563412
#guard fromPanel rgbLayout 0x123456 == ⟨0x12, 0x34, 0x56⟩
#guard fromPanel bgrLayout 0x563412 == ⟨0x12, 0x34, 0x56⟩
#guard (unpack 0x78123456).rgb == ⟨0x12, 0x34, 0x56⟩
#guard packedToPanel rgbLayout 0x78123456 == 0x123456
#guard packedToPanel bgrLayout 0x78123456 == 0x563412

#guard blendNat 200 100 255 == 200
#guard blendNat 200 100 0 == 100
#guard blendNat 255 0 128 == 128
#guard blendNat 0 255 128 == 127
#guard blendNat 255 255 77 == 255
#guard blendChannel 200 100 255 == 200
#guard blendChannel 200 100 0 == 100
#guard blendChannel 255 0 128 == 128
#guard blendOver ⟨255, 0, 0, 255⟩ ⟨0, 0, 255, 255⟩ == ⟨255, 0, 0, 255⟩
#guard blendOver ⟨255, 0, 0, 0⟩ ⟨0, 0, 255, 255⟩ == ⟨0, 0, 255, 255⟩
#guard blendOver ⟨255, 0, 0, 128⟩ ⟨0, 0, 255, 255⟩ == ⟨128, 0, 127, 255⟩

end Gpu.Pixel
