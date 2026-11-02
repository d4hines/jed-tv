import Gpu.Raster.Spec
import Gpu.Raster.Proofs

/-!
# Topic B: watertight rasterization (the top-left fill rule)

Two triangles that share an edge draw a pixel centre on that edge
exactly once.  See `Gpu.Raster.Spec` for the definitions and
`Gpu.Raster.Proofs` for the theorems `edgePasses_shared_exactly_once`,
`covers_shared_exactly_once`, `covers_shared_no_gap`.
-/
