import Gpu.Raster.Spec

/-!
# Rasterization fill rule: proofs

Main results:
* `edgePasses_shared_exactly_once` (B1, edge test only): on the line of a
  shared edge, exactly one of the two directions of the edge passes.
* `covers_shared_exactly_once` (B1): two triangles that share an edge
  cover a point strictly inside that edge exactly once.
* `covers_shared_no_gap` (B2): such a point is covered by at least one.
* `naive_covers_twice`: with the naive rule, a concrete point on a shared
  edge is covered by both triangles.
-/

namespace Gpu.Raster

/-! ## Edge functions -/

/-- Reversing an edge negates its edge function. -/
theorem edgeFn_swap (a b p : Point) : edgeFn b a p = - edgeFn a b p := by
  unfold edgeFn
  simp only [Int.mul_sub, Int.sub_mul]
  have h1 := Int.mul_comm b.x a.y
  have h2 := Int.mul_comm b.y a.x
  have h3 := Int.mul_comm a.x a.y
  have h4 := Int.mul_comm b.x b.y
  omega

/-- Rotating the vertices of a triangle does not change what it covers,
so stating B1 with the shared edge as the first edge loses nothing. -/
theorem covers_rotate (a b c : Point) (p : Point) :
    covers ⟨a, b, c⟩ p = covers ⟨b, c, a⟩ p := by
  simp only [covers]
  cases edgePasses a b p <;> cases edgePasses b c p <;> cases edgePasses c a p <;> rfl

/-- A point strictly inside a segment forces its endpoints to differ. -/
theorem OnOpenSegment.ne {a b p : Point} (h : OnOpenSegment a b p) : a ≠ b := by
  intro hab
  subst hab
  have := h.2.1
  simp at this

/-- A point strictly inside the segment `a → b` is also strictly inside `b → a`. -/
theorem OnOpenSegment.symm {a b p : Point} (h : OnOpenSegment a b p) : OnOpenSegment b a p := by
  obtain ⟨h0, h1, h2⟩ := h
  refine ⟨?_, h2, h1⟩
  rw [edgeFn_swap, h0]
  rfl

/-! ## B1 for one edge -/

/-- For two distinct points, exactly one direction of the edge between
them is top-or-left.  (If `a = b` neither is, which is why `a ≠ b` matters.) -/
theorem isTopLeft_swap {a b : Point} (hab : a ≠ b) : isTopLeft b a = !isTopLeft a b := by
  have hne : a.x ≠ b.x ∨ a.y ≠ b.y := by
    by_cases hx : a.x = b.x
    · right
      intro hy
      exact hab (by cases a; cases b; simp_all)
    · exact Or.inl hx
  simp only [isTopLeft, isTopEdge, isLeftEdge]
  rcases Int.lt_trichotomy a.y b.y with hy | hy | hy
  · simp [hy, Int.not_lt.mpr (Int.le_of_lt hy), Int.ne_of_gt hy]
  · rcases hne with hx | hx
    · rcases Int.lt_or_gt_of_ne hx with hx | hx
      · simp [hy, hx, Int.not_lt.mpr (Int.le_of_lt hx)]
      · simp [hy, hx, Int.not_lt.mpr (Int.le_of_lt hx)]
    · exact absurd hy hx
  · simp [hy, Int.not_lt.mpr (Int.le_of_lt hy), Int.ne_of_gt hy]

/-- **B1 (edge test).** If `p` is on the line of the edge `a → b` and
`a ≠ b`, then `p` passes the edge test for `a → b` exactly when it fails
the edge test for `b → a`. -/
theorem edgePasses_shared_exactly_once {a b p : Point} (hab : a ≠ b)
    (hline : edgeFn a b p = 0) : edgePasses b a p = !edgePasses a b p := by
  have hline' : edgeFn b a p = 0 := by
    rw [edgeFn_swap, hline]
    rfl
  simp only [edgePasses, hline, hline', isTopLeft_swap hab]
  simp

/-! ## B1 and B2 for whole triangles -/

/-- **B1 (coverage).** Triangle `⟨a, b, c⟩` and triangle `⟨b, a, d⟩` share
the edge between `a` and `b`, in opposite directions.  If `p` is strictly
inside that edge and passes the other two edge tests of both triangles,
then exactly one of the two triangles covers `p`. -/
theorem covers_shared_exactly_once {a b c d p : Point}
    (hp : OnOpenSegment a b p)
    (h1 : edgePasses b c p = true) (h2 : edgePasses c a p = true)
    (h3 : edgePasses a d p = true) (h4 : edgePasses d b p = true) :
    covers ⟨b, a, d⟩ p = !covers ⟨a, b, c⟩ p := by
  simp only [covers, h1, h2, h3, h4, Bool.and_true]
  exact edgePasses_shared_exactly_once hp.ne hp.1

/-- **B2 (no gaps).** Under the same conditions, at least one of the two
triangles covers `p`. -/
theorem covers_shared_no_gap {a b c d p : Point}
    (hp : OnOpenSegment a b p)
    (h1 : edgePasses b c p = true) (h2 : edgePasses c a p = true)
    (h3 : edgePasses a d p = true) (h4 : edgePasses d b p = true) :
    ¬ (covers ⟨a, b, c⟩ p = false ∧ covers ⟨b, a, d⟩ p = false) := by
  rw [covers_shared_exactly_once hp h1 h2 h3 h4]
  cases covers ⟨a, b, c⟩ p <;> simp

/-- The same, phrased positively: one of them covers `p`. -/
theorem covers_shared_or {a b c d p : Point}
    (hp : OnOpenSegment a b p)
    (h1 : edgePasses b c p = true) (h2 : edgePasses c a p = true)
    (h3 : edgePasses a d p = true) (h4 : edgePasses d b p = true) :
    (covers ⟨a, b, c⟩ p || covers ⟨b, a, d⟩ p) = true := by
  rw [covers_shared_exactly_once hp h1 h2 h3 h4]
  cases covers ⟨a, b, c⟩ p <;> rfl

/-! ## Hypotheses are satisfiable: concrete instances

`tri1 = ⟨(1,1), (9,1), (1,9)⟩` and `tri2 = ⟨(9,1), (1,1), (9,-7)⟩` share
the edge between `(1,1)` and `(9,1)`; `onEdge = (5,1)` is a pixel centre
strictly inside it.  Every hypothesis is discharged by `decide`. -/

example : OnOpenSegment ⟨1, 1⟩ ⟨9, 1⟩ onEdge := by decide
example : IsPositiveWinding tri1 ∧ IsPositiveWinding tri2 := by decide

/-- B1 (edge test) on the concrete shared edge. -/
example : edgePasses ⟨9, 1⟩ ⟨1, 1⟩ onEdge = !edgePasses ⟨1, 1⟩ ⟨9, 1⟩ onEdge :=
  edgePasses_shared_exactly_once (by decide) (by decide)

/-- B1 (coverage) on the concrete triangles: `tri2` covers `onEdge` exactly when `tri1` does not. -/
example : covers tri2 onEdge = !covers tri1 onEdge :=
  covers_shared_exactly_once (by decide) (by decide) (by decide) (by decide) (by decide)

/-- B2 on the concrete triangles. -/
example : ¬ (covers tri1 onEdge = false ∧ covers tri2 onEdge = false) :=
  covers_shared_no_gap (by decide) (by decide) (by decide) (by decide) (by decide)

/-- Concretely, it is `tri1` (whose copy of the edge is a top edge) that draws the point. -/
example : covers tri1 onEdge = true ∧ covers tri2 onEdge = false := by decide

/-! ## The spec rejects bad behaviour -/

/-- **Negative example.** With the naive rule (no tie-break), both
triangles cover the shared-edge point: it would be drawn twice. -/
theorem naive_covers_twice : coversNaive tri1 onEdge = true ∧ coversNaive tri2 onEdge = true := by
  decide

/-- So the naive rule does not have the exactly-once property that the
top-left rule has. -/
theorem naive_not_exactly_once : ¬ (coversNaive tri2 onEdge = !coversNaive tri1 onEdge) := by
  decide

/-- Dropping the `a ≠ b` condition would make B1 false: on a degenerate
edge (`a = b`) neither direction passes, so there is a gap. -/
theorem degenerate_edge_gap :
    edgePasses ⟨1, 1⟩ ⟨1, 1⟩ ⟨1, 1⟩ = false ∧ edgePasses ⟨1, 1⟩ ⟨1, 1⟩ ⟨1, 1⟩ = false := by
  decide

end Gpu.Raster
