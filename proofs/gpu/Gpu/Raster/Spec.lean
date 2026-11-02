/-!
# Rasterization fill rule: specification

A rasterizer decides, for each pixel centre, which triangle covers it.
Two triangles that share an edge must not both draw a pixel centre that
sits exactly on that edge (it would be drawn twice: wrong for blending),
and they must not both skip it (a visible gap).  The *top-left rule*
settles these ties.

## Coordinate scheme

All coordinates are integers in *doubled* screen units: the value `v`
stands for `v / 2` pixels.  Pixel `(i, j)` has its centre at
`(2 * i + 1, 2 * j + 1)`, an odd/odd point.  Triangle vertices can sit on
any half-pixel position (any integer in doubled units).  This way the
edge functions are exact integers and no fractions are needed.

The screen is y-down: larger `y` is lower on the screen.

## Winding

A triangle is listed so that every edge function is positive at its
third vertex (`IsPositiveWinding`).  In the usual y-up maths frame this
is a counter-clockwise triangle; on a y-down screen it looks clockwise.
With this winding, the inside of the triangle is where all three edge
functions are `≥ 0`.
-/

namespace Gpu.Raster

/-- A point in doubled screen units (see the file header). -/
structure Point where
  x : Int
  y : Int
deriving Repr, DecidableEq

/-- The edge function of the edge from `a` to `b`, evaluated at `p`:
the 2D cross product `(b - a) × (p - a)`.  It is `0` exactly when `p` is
on the line through `a` and `b`, and positive on the inside side of the
edge for a positive-winding triangle. -/
def edgeFn (a b p : Point) : Int :=
  (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)

/-- A top edge is exactly horizontal and points to the right (`b.x > a.x`).
For a positive-winding triangle the inside is below such an edge, so it
is the topmost edge of the triangle. -/
def isTopEdge (a b : Point) : Bool :=
  b.y = a.y ∧ b.x > a.x

/-- A left edge points up the screen (`b.y < a.y`).  For a positive-winding
triangle the inside is to the right of such an edge. -/
def isLeftEdge (a b : Point) : Bool :=
  b.y < a.y

/-- The tie-break of the top-left rule: a point exactly on an edge counts
as inside only if the edge is a top edge or a left edge. -/
def isTopLeft (a b : Point) : Bool :=
  isTopEdge a b || isLeftEdge a b

/-- The edge test with the top-left rule: `p` passes the edge `a → b` if
it is strictly on the inside side, or exactly on the edge and the edge
is top or left. -/
def edgePasses (a b p : Point) : Bool :=
  edgeFn a b p > 0 || (edgeFn a b p = 0 && isTopLeft a b)

/-- The naive edge test with no tie-break: on the edge counts as inside. -/
def edgePassesNaive (a b p : Point) : Bool :=
  edgeFn a b p ≥ 0

/-- A triangle given by its three vertices, in order. -/
structure Triangle where
  a : Point
  b : Point
  c : Point
deriving Repr, DecidableEq

/-- The vertex order makes every edge function positive at the opposite
vertex; the inside is then where all three edge functions are `≥ 0`. -/
def IsPositiveWinding (t : Triangle) : Prop :=
  edgeFn t.a t.b t.c > 0 ∧ edgeFn t.b t.c t.a > 0 ∧ edgeFn t.c t.a t.b > 0

instance (t : Triangle) : Decidable (IsPositiveWinding t) :=
  inferInstanceAs (Decidable (_ ∧ _ ∧ _))

/-- `p` is covered by `t` under the top-left rule: it passes all three
edge tests `a → b`, `b → c`, `c → a`. -/
def covers (t : Triangle) (p : Point) : Bool :=
  edgePasses t.a t.b p && edgePasses t.b t.c p && edgePasses t.c t.a p

/-- `p` is covered by `t` under the naive rule (all three edge functions `≥ 0`). -/
def coversNaive (t : Triangle) (p : Point) : Bool :=
  edgePassesNaive t.a t.b p && edgePassesNaive t.b t.c p && edgePassesNaive t.c t.a p

/-- `p` lies on the line through `a` and `b`, strictly between them:
the edge function is `0` and `p` is on the `b` side of `a` and on the `a`
side of `b` (two dot products are positive).  This rules out `p = a`,
`p = b` and `a = b`. -/
def OnOpenSegment (a b p : Point) : Prop :=
  edgeFn a b p = 0 ∧
  (b.x - a.x) * (p.x - a.x) + (b.y - a.y) * (p.y - a.y) > 0 ∧
  (a.x - b.x) * (p.x - b.x) + (a.y - b.y) * (p.y - b.y) > 0

instance : ∀ a b p, Decidable (OnOpenSegment a b p) := fun _ _ _ =>
  inferInstanceAs (Decidable (_ ∧ _ ∧ _))

/-! ## Unit tests

Triangle `tri1` has vertices at pixel centres `(0,0)`, `(4,0)`, `(0,4)`,
that is doubled coordinates `(1,1)`, `(9,1)`, `(1,9)`.  Triangle `tri2`
shares the edge from `(9,1)` to `(1,1)` and lies above it.  The point
`onEdge` is the pixel centre `(2,0)`, doubled `(5,1)`, on the shared edge. -/

def tri1 : Triangle := ⟨⟨1, 1⟩, ⟨9, 1⟩, ⟨1, 9⟩⟩
def tri2 : Triangle := ⟨⟨9, 1⟩, ⟨1, 1⟩, ⟨9, -7⟩⟩
def onEdge : Point := ⟨5, 1⟩

#guard edgeFn ⟨0, 0⟩ ⟨10, 0⟩ ⟨0, 10⟩ == 100
#guard edgeFn ⟨0, 0⟩ ⟨10, 0⟩ ⟨5, 0⟩ == 0
#guard edgeFn ⟨0, 0⟩ ⟨10, 0⟩ ⟨0, -10⟩ == -100

#guard isTopEdge ⟨1, 1⟩ ⟨9, 1⟩ == true
#guard isTopEdge ⟨9, 1⟩ ⟨1, 1⟩ == false
#guard isLeftEdge ⟨1, 9⟩ ⟨1, 1⟩ == true
#guard isLeftEdge ⟨1, 1⟩ ⟨1, 9⟩ == false
#guard isTopLeft ⟨1, 1⟩ ⟨9, 1⟩ == true
#guard isTopLeft ⟨9, 1⟩ ⟨1, 1⟩ == false

#guard edgePasses ⟨1, 1⟩ ⟨9, 1⟩ onEdge == true
#guard edgePasses ⟨9, 1⟩ ⟨1, 1⟩ onEdge == false
#guard edgePassesNaive ⟨1, 1⟩ ⟨9, 1⟩ onEdge == true
#guard edgePassesNaive ⟨9, 1⟩ ⟨1, 1⟩ onEdge == true

#guard decide (IsPositiveWinding tri1) == true
#guard decide (IsPositiveWinding tri2) == true
#guard decide (IsPositiveWinding ⟨⟨1, 1⟩, ⟨1, 9⟩, ⟨9, 1⟩⟩) == false

#guard covers tri1 ⟨3, 3⟩ == true
#guard covers tri1 ⟨9, 9⟩ == false
#guard covers tri1 onEdge == true
#guard covers tri2 onEdge == false
#guard coversNaive tri1 onEdge == true
#guard coversNaive tri2 onEdge == true

#guard decide (OnOpenSegment ⟨1, 1⟩ ⟨9, 1⟩ onEdge) == true
#guard decide (OnOpenSegment ⟨1, 1⟩ ⟨9, 1⟩ ⟨1, 1⟩) == false
#guard decide (OnOpenSegment ⟨1, 1⟩ ⟨9, 1⟩ ⟨11, 1⟩) == false
#guard decide (OnOpenSegment ⟨1, 1⟩ ⟨9, 1⟩ ⟨5, 3⟩) == false

end Gpu.Raster
