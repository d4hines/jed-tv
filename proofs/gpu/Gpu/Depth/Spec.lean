/-!
# Depth test: specification

A GPU draws many fragments (candidate pixels) and keeps, for each
screen pixel, the one nearest to the viewer.  The depth test does this
one fragment at a time: a fragment replaces what the pixel holds only
if it is strictly nearer.

This file defines the data, the depth test, the `draw` loop, and the
property `NearestWins` that a correct result must have.  The proofs are
in `Gpu.Depth.Proofs`.
-/

namespace Gpu.Depth

/-- A fragment is a candidate for one pixel: which pixel, how far away
(smaller `depth` = nearer the viewer), and which colour. -/
structure Fragment where
  pixel : Nat
  depth : Nat
  colour : UInt32
deriving Repr, DecidableEq

/-- What a framebuffer pixel stores once something has been drawn to it:
the depth and the colour of the fragment that won. -/
structure Sample where
  depth : Nat
  colour : UInt32
deriving Repr, DecidableEq

/-- A framebuffer maps each pixel index to what it holds: `none` means
nothing has been drawn there yet. -/
def Framebuffer := Nat → Option Sample

/-- The framebuffer with nothing drawn. -/
def Framebuffer.empty : Framebuffer := fun _ => none

/-- The depth test.  A fragment passes if the pixel is empty, or if the
fragment is strictly nearer than what the pixel holds.  A fragment at
the same depth as the stored one fails (so the first one drawn wins ties). -/
def passesDepthTest (f : Fragment) (stored : Option Sample) : Bool :=
  match stored with
  | none => true
  | some s => decide (f.depth < s.depth)

/-- What one pixel holds after one fragment has been processed: the
pixel changes only if the fragment targets it and passes the depth test. -/
def stepPixel (p : Nat) (stored : Option Sample) (f : Fragment) : Option Sample :=
  if p = f.pixel ∧ passesDepthTest f stored then some ⟨f.depth, f.colour⟩ else stored

/-- Draw one fragment into the framebuffer: only its target pixel can
change, and only if it passes the depth test. -/
def writeFragment (fb : Framebuffer) (f : Fragment) : Framebuffer :=
  fun p => stepPixel p (fb p) f

/-- Draw a list of fragments, one after another, in list order. -/
def draw (frags : List Fragment) (fb : Framebuffer) : Framebuffer :=
  frags.foldl writeFragment fb

/-- The depth a pixel holds, if any. -/
def depthAt (fb : Framebuffer) (p : Nat) : Option Nat :=
  (fb p).map Sample.depth

/-- The colour a pixel holds, if any. -/
def colourAt (fb : Framebuffer) (p : Nat) : Option UInt32 :=
  (fb p).map Sample.colour

/-- `NearestWins frags fb` says what a correct result of drawing `frags`
onto an empty framebuffer must look like:
* a pixel is empty exactly when no fragment targets it;
* whatever a pixel holds came from a fragment that targets that pixel;
* no fragment that targets the pixel is nearer than what it holds.
Together: each pixel holds a nearest fragment among those that target it. -/
structure NearestWins (frags : List Fragment) (fb : Framebuffer) : Prop where
  /-- A pixel is empty exactly when no fragment targets it. -/
  empty_iff : ∀ p, fb p = none ↔ ∀ f ∈ frags, f.pixel ≠ p
  /-- Whatever a pixel holds is the depth and colour of some fragment that targets it. -/
  came_from_fragment : ∀ p s, fb p = some s →
    ∃ f ∈ frags, f.pixel = p ∧ f.depth = s.depth ∧ f.colour = s.colour
  /-- No fragment that targets the pixel is nearer than what the pixel holds. -/
  nearest : ∀ p s, fb p = some s → ∀ f ∈ frags, f.pixel = p → s.depth ≤ f.depth

/-- No two fragments in the list target the same pixel at the same depth.
This is the tie rule: with it, the colour of each pixel does not depend
on the drawing order. -/
def NoDepthTies (frags : List Fragment) : Prop :=
  frags.Pairwise fun f g => f.pixel = g.pixel → f.depth ≠ g.depth

/-! ## Unit tests -/

/-- Three fragments: two at pixel 0 (depths 5 and 2), one at pixel 1. -/
def exampleFrags : List Fragment :=
  [⟨0, 5, 0xFF0000⟩, ⟨0, 2, 0x00FF00⟩, ⟨1, 9, 0x0000FF⟩]

#guard passesDepthTest ⟨0, 3, 0⟩ none == true
#guard passesDepthTest ⟨0, 3, 0⟩ (some ⟨5, 0⟩) == true
#guard passesDepthTest ⟨0, 5, 0⟩ (some ⟨5, 0⟩) == false
#guard passesDepthTest ⟨0, 7, 0⟩ (some ⟨5, 0⟩) == false

#guard stepPixel 0 none ⟨0, 3, 7⟩ == some ⟨3, 7⟩
#guard stepPixel 1 none ⟨0, 3, 7⟩ == none
#guard stepPixel 0 (some ⟨2, 9⟩) ⟨0, 3, 7⟩ == some ⟨2, 9⟩

#guard writeFragment Framebuffer.empty ⟨4, 1, 2⟩ 4 == some ⟨1, 2⟩
#guard writeFragment Framebuffer.empty ⟨4, 1, 2⟩ 5 == none

#guard draw exampleFrags Framebuffer.empty 0 == some ⟨2, 0x00FF00⟩
#guard draw exampleFrags Framebuffer.empty 1 == some ⟨9, 0x0000FF⟩
#guard draw exampleFrags Framebuffer.empty 2 == none
#guard depthAt (draw exampleFrags Framebuffer.empty) 0 == some 2
#guard colourAt (draw exampleFrags Framebuffer.empty) 0 == some 0x00FF00
#guard depthAt Framebuffer.empty 0 == none

/-- `NoDepthTies` is decidable on concrete lists, so it can be checked by `decide`. -/
instance : DecidablePred NoDepthTies := fun frags =>
  inferInstanceAs (Decidable (frags.Pairwise _))

example : NoDepthTies exampleFrags := by decide
example : ¬ NoDepthTies [⟨0, 5, 1⟩, ⟨0, 5, 2⟩] := by decide

end Gpu.Depth
