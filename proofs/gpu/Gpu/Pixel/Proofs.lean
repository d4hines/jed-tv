import Gpu.Pixel.Spec

/-!
# Pixel formats and blending: proofs

Main results:
* `unpack_pack`, `pack_unpack` (C1): packing as `0xAARRGGBB` and
  unpacking are inverse to each other, for every colour and every word.
* `fromPanel_toPanel_rgb`, `fromPanel_toPanel_bgr` (C1): writing a colour
  to a panel word and reading it back recovers the colour, for the RGB
  and the BGR layouts.
* `blendNat_le_255` (C2): the blend never leaves `0..255`, so it fits in
  8 bits without wrapping (`blendChannel_toNat`).
* `blendNat_alpha_full`, `blendNat_alpha_zero` (C2): full alpha gives
  the source, zero alpha keeps the destination.
* Negative example: a colour written in BGR and read as RGB comes back
  with red and blue swapped.

The bit-level facts are proved by turning shifts and ORs into plain
arithmetic (a byte shifted left by `k` is the byte times `2^k`; ORing
values that occupy different bit ranges is adding them) and then letting
`omega` do the arithmetic.  This avoids `bv_decide`, which would add a
native-computation axiom to the proof.
-/

namespace Gpu.Pixel

/-! ## Bits as arithmetic -/

/-- ORing a value shifted left by `n` bits with a value below `2^n` is
the same as adding them: the two occupy different bits. -/
theorem or_eq_add_mul {x y n : Nat} (hy : y < 2 ^ n) : x * 2 ^ n ||| y = x * 2 ^ n + y := by
  rw [← Nat.shiftLeft_eq]
  exact (Nat.shiftLeft_add_eq_or_of_lt hy x).symm

/-- `or_eq_add_mul` for 8 bits. -/
theorem or_add_8 {x y : Nat} (hy : y < 256) : x * 256 ||| y = x * 256 + y :=
  or_eq_add_mul (n := 8) hy

/-- `or_eq_add_mul` for 16 bits. -/
theorem or_add_16 {x y : Nat} (hy : y < 65536) : x * 65536 ||| y = x * 65536 + y :=
  or_eq_add_mul (n := 16) hy

/-- `or_eq_add_mul` for 24 bits. -/
theorem or_add_24 {x y : Nat} (hy : y < 16777216) : x * 16777216 ||| y = x * 16777216 + y :=
  or_eq_add_mul (n := 24) hy

/-- Four bytes placed at bit offsets 24, 16, 8 and 0 with OR add up. -/
theorem or_bytes4 {a r g b : Nat} (hr : r < 256) (hg : g < 256) (hb : b < 256) :
    a * 16777216 ||| r * 65536 ||| g * 256 ||| b = a * 16777216 + r * 65536 + g * 256 + b := by
  rw [Nat.or_assoc, Nat.or_assoc, or_add_8 hb, or_add_16 (by omega), or_add_24 (by omega)]
  omega

/-- Three bytes placed at bit offsets 16, 8 and 0 with OR add up. -/
theorem or_bytes3 {r g b : Nat} (hg : g < 256) (hb : b < 256) :
    r * 65536 ||| g * 256 ||| b = r * 65536 + g * 256 + b := by
  rw [Nat.or_assoc, or_add_8 hb, or_add_16 (by omega)]
  omega

/-- The same with the bytes ORed in the opposite order (offsets 0, 8, 16). -/
theorem or_bytes3_rev {r g b : Nat} (hr : r < 256) (hg : g < 256) :
    r ||| g * 256 ||| b * 65536 = b * 65536 + g * 256 + r := by
  rw [Nat.or_comm, Nat.or_comm r, ← Nat.or_assoc]
  exact or_bytes3 hg hr

/-- A byte shifted left by 24 bits, as a number (it fits in 32 bits). -/
theorem toNat_shl24 (x : UInt8) : (x.toUInt32 <<< 24).toNat = x.toNat * 16777216 := by
  have := x.toNat_lt
  simp only [UInt32.toNat_shiftLeft, UInt8.toNat_toUInt32, UInt32.toNat_ofNat, Nat.shiftLeft_eq,
    Nat.reduceMod, Nat.reducePow]
  rw [Nat.mod_eq_of_lt (by omega)]

/-- A byte shifted left by 16 bits, as a number. -/
theorem toNat_shl16 (x : UInt8) : (x.toUInt32 <<< 16).toNat = x.toNat * 65536 := by
  have := x.toNat_lt
  simp only [UInt32.toNat_shiftLeft, UInt8.toNat_toUInt32, UInt32.toNat_ofNat, Nat.shiftLeft_eq,
    Nat.reduceMod, Nat.reducePow]
  rw [Nat.mod_eq_of_lt (by omega)]

/-- A byte shifted left by 8 bits, as a number. -/
theorem toNat_shl8 (x : UInt8) : (x.toUInt32 <<< 8).toNat = x.toNat * 256 := by
  have := x.toNat_lt
  simp only [UInt32.toNat_shiftLeft, UInt8.toNat_toUInt32, UInt32.toNat_ofNat, Nat.shiftLeft_eq,
    Nat.reduceMod, Nat.reducePow]
  rw [Nat.mod_eq_of_lt (by omega)]

/-- A byte shifted left by 0 bits is the byte. -/
theorem toNat_shl0 (x : UInt8) : (x.toUInt32 <<< 0).toNat = x.toNat := by
  have := x.toNat_lt
  simp only [UInt32.toNat_shiftLeft, UInt8.toNat_toUInt32, UInt32.toNat_ofNat, Nat.shiftLeft_eq,
    Nat.reduceMod, Nat.reducePow, Nat.mul_one]
  rw [Nat.mod_eq_of_lt (by omega)]

/-- The packed word, as a number: `a·2^24 + r·2^16 + g·2^8 + b`. -/
theorem pack_toNat (c : Rgba) :
    (pack c).toNat = c.a.toNat * 16777216 + c.r.toNat * 65536 + c.g.toNat * 256 + c.b.toNat := by
  have hr := c.r.toNat_lt
  have hg := c.g.toNat_lt
  have hb := c.b.toNat_lt
  simp only [pack, UInt32.toNat_or, toNat_shl24, toNat_shl16, toNat_shl8, UInt8.toNat_toUInt32]
  exact or_bytes4 (by omega) (by omega) (by omega)

/-- The RGB panel word, as a number: `r·2^16 + g·2^8 + b`. -/
theorem toPanel_rgb_toNat (c : Rgb) :
    (toPanel rgbLayout c).toNat = c.r.toNat * 65536 + c.g.toNat * 256 + c.b.toNat := by
  have hg := c.g.toNat_lt
  have hb := c.b.toNat_lt
  simp only [toPanel, rgbLayout, UInt32.toNat_or, toNat_shl16, toNat_shl8, toNat_shl0]
  exact or_bytes3 (by omega) (by omega)

/-- The BGR panel word, as a number: `b·2^16 + g·2^8 + r`. -/
theorem toPanel_bgr_toNat (c : Rgb) :
    (toPanel bgrLayout c).toNat = c.b.toNat * 65536 + c.g.toNat * 256 + c.r.toNat := by
  have hg := c.g.toNat_lt
  have hr := c.r.toNat_lt
  simp only [toPanel, bgrLayout, UInt32.toNat_or, toNat_shl16, toNat_shl8, toNat_shl0]
  exact or_bytes3_rev (by omega) (by omega)

/-! ## C1: packing -/

/-- **C1.** Unpacking a packed colour gives the colour back. -/
theorem unpack_pack (c : Rgba) : unpack (pack c) = c := by
  obtain ⟨r, g, b, a⟩ := c
  have hr := r.toNat_lt
  have hg := g.toNat_lt
  have hb := b.toNat_lt
  have ha := a.toNat_lt
  have hp : (pack ⟨r, g, b, a⟩).toNat
      = a.toNat * 16777216 + r.toNat * 65536 + g.toNat * 256 + b.toNat := pack_toNat _
  simp only [unpack, Rgba.mk.injEq]
  refine ⟨?_, ?_, ?_, ?_⟩ <;> apply UInt8.toNat_inj.mp <;>
    simp only [UInt32.toNat_toUInt8, UInt32.toNat_shiftRight, Nat.shiftRight_eq_div_pow,
      UInt32.toNat_ofNat, Nat.reduceMod, Nat.reducePow, hp] <;> omega

/-- **C1.** Packing an unpacked word gives the word back. -/
theorem pack_unpack (v : UInt32) : pack (unpack v) = v := by
  apply UInt32.toNat_inj.mp
  have hv := v.toNat_lt
  rw [pack_toNat]
  simp only [unpack, UInt32.toNat_toUInt8, UInt32.toNat_shiftRight, Nat.shiftRight_eq_div_pow,
    UInt32.toNat_ofNat, Nat.reduceMod, Nat.reducePow]
  omega

/-- **C1 (RGB panel).** A colour sent to an RGB panel word and read back is unchanged. -/
theorem fromPanel_toPanel_rgb (c : Rgb) : fromPanel rgbLayout (toPanel rgbLayout c) = c := by
  obtain ⟨r, g, b⟩ := c
  have hr := r.toNat_lt
  have hg := g.toNat_lt
  have hb := b.toNat_lt
  have hp : (toPanel ⟨16, 8, 0⟩ ⟨r, g, b⟩).toNat
      = r.toNat * 65536 + g.toNat * 256 + b.toNat := toPanel_rgb_toNat _
  simp only [fromPanel, rgbLayout, Rgb.mk.injEq]
  refine ⟨?_, ?_, ?_⟩ <;> apply UInt8.toNat_inj.mp <;>
    simp only [UInt32.toNat_toUInt8, UInt32.toNat_shiftRight, Nat.shiftRight_eq_div_pow,
      UInt32.toNat_ofNat, Nat.reduceMod, Nat.reducePow, hp] <;> omega

/-- **C1 (BGR panel).** A colour sent to a BGR panel word and read back is unchanged. -/
theorem fromPanel_toPanel_bgr (c : Rgb) : fromPanel bgrLayout (toPanel bgrLayout c) = c := by
  obtain ⟨r, g, b⟩ := c
  have hr := r.toNat_lt
  have hg := g.toNat_lt
  have hb := b.toNat_lt
  have hp : (toPanel ⟨0, 8, 16⟩ ⟨r, g, b⟩).toNat
      = b.toNat * 65536 + g.toNat * 256 + r.toNat := toPanel_bgr_toNat _
  simp only [fromPanel, bgrLayout, Rgb.mk.injEq]
  refine ⟨?_, ?_, ?_⟩ <;> apply UInt8.toNat_inj.mp <;>
    simp only [UInt32.toNat_toUInt8, UInt32.toNat_shiftRight, Nat.shiftRight_eq_div_pow,
      UInt32.toNat_ofNat, Nat.reduceMod, Nat.reducePow, hp] <;> omega

/-- **C1 (end to end).** A packed `0xAARRGGBB` word converted to either
panel layout and read back gives the colour's red, green and blue. -/
theorem fromPanel_packedToPanel (L : PanelLayout) (hL : L = rgbLayout ∨ L = bgrLayout)
    (v : UInt32) : fromPanel L (packedToPanel L v) = (unpack v).rgb := by
  unfold packedToPanel
  rcases hL with rfl | rfl
  · exact fromPanel_toPanel_rgb _
  · exact fromPanel_toPanel_bgr _

/-! ## C2: blending -/

/-- **C2 (range).** When all inputs are at most 255, so is the blend.
Hence it fits in 8 bits. -/
theorem blendNat_le_255 {src dst a : Nat} (hs : src ≤ 255) (hd : dst ≤ 255) (ha : a ≤ 255) :
    blendNat src dst a ≤ 255 := by
  unfold blendNat
  have h1 : src * a ≤ 255 * a := Nat.mul_le_mul_right a hs
  have h2 : dst * (255 - a) ≤ 255 * (255 - a) := Nat.mul_le_mul_right _ hd
  omega

/-- **C2 (full alpha).** With alpha 255 the result is the source. -/
theorem blendNat_alpha_full (src dst : Nat) : blendNat src dst 255 = src := by
  unfold blendNat
  omega

/-- **C2 (zero alpha).** With alpha 0 the result is the destination. -/
theorem blendNat_alpha_zero (src dst : Nat) : blendNat src dst 0 = dst := by
  unfold blendNat
  omega

/-- The 8-bit blend is exactly the `Nat` blend: no wrapping happens. -/
theorem blendChannel_toNat (src dst a : UInt8) :
    (blendChannel src dst a).toNat = blendNat src.toNat dst.toNat a.toNat := by
  unfold blendChannel
  apply UInt8.toNat_ofNat_of_lt'
  have := blendNat_le_255 (Nat.le_of_lt_succ src.toNat_lt) (Nat.le_of_lt_succ dst.toNat_lt)
    (Nat.le_of_lt_succ a.toNat_lt)
  exact Nat.lt_succ_of_le this

/-- Full alpha on 8-bit channels: the source wins. -/
theorem blendChannel_alpha_full (src dst : UInt8) : blendChannel src dst 255 = src := by
  apply UInt8.toNat_inj.mp
  rw [blendChannel_toNat]
  show blendNat src.toNat dst.toNat 255 = src.toNat
  exact blendNat_alpha_full _ _

/-- Zero alpha on 8-bit channels: the destination is untouched. -/
theorem blendChannel_alpha_zero (src dst : UInt8) : blendChannel src dst 0 = dst := by
  apply UInt8.toNat_inj.mp
  rw [blendChannel_toNat]
  show blendNat src.toNat dst.toNat 0 = dst.toNat
  exact blendNat_alpha_zero _ _

/-- An opaque source colour replaces the destination's red, green and blue. -/
theorem blendOver_opaque (src dst : Rgba) (h : src.a = 255) :
    (blendOver src dst).rgb = src.rgb := by
  simp only [blendOver, Rgba.rgb, h, blendChannel_alpha_full]

/-- A fully transparent source colour leaves the destination unchanged. -/
theorem blendOver_transparent (src dst : Rgba) (h : src.a = 0) :
    blendOver src dst = dst := by
  obtain ⟨r, g, b, a⟩ := dst
  simp only [blendOver, h, blendChannel_alpha_zero]

/-! ## Hypotheses are satisfiable: concrete instances -/

example : unpack (pack ⟨0x12, 0x34, 0x56, 0x78⟩) = ⟨0x12, 0x34, 0x56, 0x78⟩ :=
  unpack_pack _

example : pack (unpack 0xDEADBEEF) = 0xDEADBEEF := pack_unpack _

example : fromPanel rgbLayout (toPanel rgbLayout ⟨0x12, 0x34, 0x56⟩) = ⟨0x12, 0x34, 0x56⟩ :=
  fromPanel_toPanel_rgb _

example : fromPanel bgrLayout (toPanel bgrLayout ⟨0x12, 0x34, 0x56⟩) = ⟨0x12, 0x34, 0x56⟩ :=
  fromPanel_toPanel_bgr _

example : fromPanel bgrLayout (packedToPanel bgrLayout 0x78123456) = (unpack 0x78123456).rgb :=
  fromPanel_packedToPanel bgrLayout (Or.inr rfl) _

example : blendNat 200 100 77 ≤ 255 :=
  blendNat_le_255 (by decide) (by decide) (by decide)

example : blendNat 200 100 255 = 200 := blendNat_alpha_full _ _
example : blendNat 200 100 0 = 100 := blendNat_alpha_zero _ _
example : blendChannel 200 100 255 = 200 := blendChannel_alpha_full _ _
example : blendChannel 200 100 0 = 100 := blendChannel_alpha_zero _ _
example : (blendOver ⟨255, 0, 0, 255⟩ ⟨0, 0, 255, 255⟩).rgb = ⟨255, 0, 0⟩ :=
  blendOver_opaque _ _ rfl
example : blendOver ⟨255, 0, 0, 0⟩ ⟨0, 0, 255, 255⟩ = ⟨0, 0, 255, 255⟩ :=
  blendOver_transparent _ _ rfl

/-! ## The spec rejects bad behaviour -/

/-- **Negative example.** A colour written for a BGR panel but read with
the RGB layout comes back with red and blue swapped: the layouts are not
interchangeable. -/
theorem bgr_read_as_rgb_swaps :
    fromPanel rgbLayout (toPanel bgrLayout ⟨0x12, 0x34, 0x56⟩) = ⟨0x56, 0x34, 0x12⟩ ∧
    fromPanel rgbLayout (toPanel bgrLayout ⟨0x12, 0x34, 0x56⟩) ≠ ⟨0x12, 0x34, 0x56⟩ := by
  decide

/-- The same for a packed word: converting for a BGR panel and reading as
RGB is not the identity on the colour. -/
theorem packed_bgr_read_as_rgb_differs :
    fromPanel rgbLayout (packedToPanel bgrLayout 0x78123456) ≠ (unpack 0x78123456).rgb := by
  decide

/-- Half alpha is neither the source nor the destination: the blend really mixes. -/
theorem blend_half_mixes : blendNat 255 0 128 ≠ 255 ∧ blendNat 255 0 128 ≠ 0 := by
  decide

end Gpu.Pixel
