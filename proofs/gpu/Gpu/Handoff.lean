import Gpu.Handoff.Spec
import Gpu.Handoff.Proofs

/-!
# Topic D: renderer-to-presenter frame handoff (a sequence lock)

A reader that passes the sequence-counter check never sees a torn
frame.  See `Gpu.Handoff.Spec` for the model and `Gpu.Handoff.Proofs`
for the theorems `no_torn_frames`, `freshness`, `counter_identifies_frame`.
-/
