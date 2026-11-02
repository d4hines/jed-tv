import Gpu.Depth.Spec
import Gpu.Depth.Proofs

/-!
# Topic A: the depth test

Drawing order does not matter for a depth-tested framebuffer.
See `Gpu.Depth.Spec` for the definitions and `Gpu.Depth.Proofs` for the
theorems `draw_nearestWins`, `draw_depth_perm`, `draw_perm_of_noDepthTies`.
-/
