import Gpu.Pixel.Spec
import Gpu.Pixel.Proofs

/-!
# Topic C: pixel formats and blending

Packing colours into 32-bit words, panel layouts, and 8-bit source-over
blending.  See `Gpu.Pixel.Spec` for the definitions and
`Gpu.Pixel.Proofs` for the theorems `unpack_pack`, `pack_unpack`,
`fromPanel_toPanel_rgb`, `fromPanel_toPanel_bgr`, `blendNat_le_255`,
`blendNat_alpha_full`, `blendNat_alpha_zero`.
-/
