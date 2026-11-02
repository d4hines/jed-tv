import Gpu.Depth
import Gpu.Raster
import Gpu.Pixel
import Gpu.Handoff
import Gpu.Audit

/-!
# Gpu: machine-checked facts about GPU pipeline operations

Four topics, each with a `Spec` file (definitions, unit tests) and a
`Proofs` file (theorems, concrete instances, negative examples):

* `Gpu.Depth`: the depth test; drawing order does not matter.
* `Gpu.Raster`: the top-left fill rule; shared edges are drawn exactly once.
* `Gpu.Pixel`: colour packing, panel layouts, 8-bit blending.
* `Gpu.Handoff`: a sequence lock between renderer and presenter; no torn frames.

`Gpu.Audit` prints the axioms of every main theorem.
-/
