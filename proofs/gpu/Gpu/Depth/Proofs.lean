import Gpu.Depth.Spec

/-!
# Depth test: proofs

Main results:
* `draw_nearestWins` (A1): drawing any list of fragments onto an empty
  framebuffer leaves every pixel holding a nearest fragment.
* `draw_depth_perm` (A2, depth): drawing the fragments in any other order
  gives the same depth at every pixel.
* `draw_perm_of_noDepthTies` (A2, colour): if no two fragments at the
  same pixel share a depth, any order gives exactly the same framebuffer.
* Negative examples: a tie case where the colour does depend on order,
  and a framebuffer where a farther fragment won, which fails the spec.
-/

namespace Gpu.Depth

/-! ## One pixel at a time

`draw` changes the framebuffer as a whole, but each pixel only ever
depends on its own old value and the fragments that target it.  The
lemmas below reduce `draw` to a fold on one pixel. -/

/-- Drawing a list is the same as folding `stepPixel` on each pixel. -/
theorem draw_apply (frags : List Fragment) (fb : Framebuffer) (p : Nat) :
    draw frags fb p = frags.foldl (stepPixel p) (fb p) := by
  induction frags generalizing fb with
  | nil => rfl
  | cons f rest ih =>
    simp only [draw, List.foldl_cons] at *
    exact ih (writeFragment fb f)

/-- A pixel ends up empty exactly when it started empty and no fragment targets it. -/
theorem foldl_stepPixel_eq_none_iff (frags : List Fragment) (p : Nat) (s0 : Option Sample) :
    frags.foldl (stepPixel p) s0 = none ↔ s0 = none ∧ ∀ f ∈ frags, f.pixel ≠ p := by
  induction frags generalizing s0 with
  | nil => simp
  | cons f rest ih =>
    simp only [List.foldl_cons, ih, List.mem_cons, forall_eq_or_imp]
    unfold stepPixel
    by_cases hp : p = f.pixel
    · cases s0 with
      | none => simp [hp, passesDepthTest]
      | some s =>
        simp only [hp, passesDepthTest, true_and]
        by_cases hd : f.depth < s.depth <;> simp [hd]
    · simp [hp, Ne.symm hp]

/-- Whatever a pixel holds at the end either was there at the start or is
the depth and colour of some fragment that targets the pixel. -/
theorem foldl_stepPixel_eq_some (frags : List Fragment) (p : Nat) (s0 : Option Sample)
    (s : Sample) (h : frags.foldl (stepPixel p) s0 = some s) :
    s0 = some s ∨ ∃ f ∈ frags, f.pixel = p ∧ f.depth = s.depth ∧ f.colour = s.colour := by
  induction frags generalizing s0 with
  | nil => simp at h; exact Or.inl h
  | cons f rest ih =>
    simp only [List.foldl_cons] at h
    rcases ih _ h with h1 | ⟨g, hg, hgp, hgd, hgc⟩
    · unfold stepPixel at h1
      split at h1
      · right
        rename_i hc
        cases h1
        exact ⟨f, List.mem_cons_self .., hc.1.symm, rfl, rfl⟩
      · exact Or.inl h1
    · exact Or.inr ⟨g, List.mem_cons_of_mem _ hg, hgp, hgd, hgc⟩

/-- Whatever a pixel holds at the end is no farther than what it held at
the start and no farther than any fragment that targets the pixel. -/
theorem foldl_stepPixel_nearest (frags : List Fragment) (p : Nat) (s0 : Option Sample)
    (s : Sample) (h : frags.foldl (stepPixel p) s0 = some s) :
    (∀ t, s0 = some t → s.depth ≤ t.depth) ∧ ∀ f ∈ frags, f.pixel = p → s.depth ≤ f.depth := by
  induction frags generalizing s0 with
  | nil =>
    simp at h
    subst h
    simp
  | cons f rest ih =>
    simp only [List.foldl_cons] at h
    have ⟨ih1, ih2⟩ := ih _ h
    refine ⟨?_, ?_⟩
    · intro t ht
      subst ht
      unfold stepPixel at ih1
      split at ih1
      · rename_i hc
        have : s.depth ≤ f.depth := ih1 _ rfl
        simp only [passesDepthTest, decide_eq_true_eq] at hc
        omega
      · exact ih1 _ rfl
    · intro g hg hgp
      rcases List.mem_cons.mp hg with rfl | hg
      · unfold stepPixel at ih1
        split at ih1
        · exact ih1 _ rfl
        · rename_i hc
          cases s0 with
          | none => simp [passesDepthTest] at hc; exact absurd hgp.symm hc
          | some t =>
            simp only [passesDepthTest, decide_eq_true_eq, not_and, Nat.not_lt] at hc
            have := ih1 t rfl
            have := hc hgp.symm
            omega
      · exact ih2 g hg hgp

/-! ## A1: nearest wins -/

/-- **A1.** After drawing any list of fragments onto an empty framebuffer,
every pixel holds a nearest fragment among those that target it, or is
empty if none does. -/
theorem draw_nearestWins (frags : List Fragment) :
    NearestWins frags (draw frags Framebuffer.empty) where
  empty_iff p := by
    rw [draw_apply, foldl_stepPixel_eq_none_iff]
    simp [Framebuffer.empty]
  came_from_fragment p s h := by
    rw [draw_apply] at h
    rcases foldl_stepPixel_eq_some _ _ _ _ h with h1 | h1
    · simp [Framebuffer.empty] at h1
    · exact h1
  nearest p s h := by
    rw [draw_apply] at h
    exact (foldl_stepPixel_nearest _ _ _ _ h).2

/-! ## A2: the drawing order does not matter -/

/-- **A2 (depth).** Drawing the same fragments in any other order gives
the same depth at every pixel, whatever the starting framebuffer. -/
theorem draw_depth_perm {l₁ l₂ : List Fragment} (hperm : l₁.Perm l₂) (fb : Framebuffer)
    (p : Nat) : depthAt (draw l₁ fb) p = depthAt (draw l₂ fb) p := by
  unfold depthAt
  rw [draw_apply, draw_apply]
  have mem : ∀ f, f ∈ l₁ ↔ f ∈ l₂ := fun f => hperm.mem_iff
  -- The two results are either both empty or both hold the same depth.
  have key : ∀ {m₁ m₂ : List Fragment}, (∀ f, f ∈ m₁ ↔ f ∈ m₂) →
      ∀ s₁ s₂, m₁.foldl (stepPixel p) (fb p) = some s₁ →
        m₂.foldl (stepPixel p) (fb p) = some s₂ → s₁.depth ≤ s₂.depth := by
    intro m₁ m₂ hmem s₁ s₂ h₁ h₂
    have ⟨n₁, n₂⟩ := foldl_stepPixel_nearest _ _ _ _ h₁
    rcases foldl_stepPixel_eq_some _ _ _ _ h₂ with h | ⟨g, hg, hgp, hgd, -⟩
    · exact n₁ _ h
    · rw [← hgd]; exact n₂ g ((hmem g).mpr hg) hgp
  cases h₁ : l₁.foldl (stepPixel p) (fb p) with
  | none =>
    rw [foldl_stepPixel_eq_none_iff] at h₁
    have h₂ : l₂.foldl (stepPixel p) (fb p) = none := by
      rw [foldl_stepPixel_eq_none_iff]
      exact ⟨h₁.1, fun f hf => h₁.2 f ((mem f).mpr hf)⟩
    rw [h₂]
  | some s₁ =>
    cases h₂ : l₂.foldl (stepPixel p) (fb p) with
    | none =>
      rw [foldl_stepPixel_eq_none_iff] at h₂
      have : l₁.foldl (stepPixel p) (fb p) = none := by
        rw [foldl_stepPixel_eq_none_iff]
        exact ⟨h₂.1, fun f hf => h₂.2 f ((mem f).mp hf)⟩
      rw [this] at h₁
      cases h₁
    | some s₂ =>
      have a := key mem s₁ s₂ h₁ h₂
      have b := key (fun f => (mem f).symm) s₂ s₁ h₂ h₁
      simp only [Option.map_some]
      congr 1
      omega

/-- A fragment aimed at another pixel changes nothing. -/
theorem stepPixel_of_ne {p : Nat} {f : Fragment} (h : p ≠ f.pixel) (s : Option Sample) :
    stepPixel p s f = s := by
  simp [stepPixel, h]

/-- A fragment aimed at an empty pixel is stored. -/
theorem stepPixel_none {p : Nat} {f : Fragment} (h : p = f.pixel) :
    stepPixel p none f = some ⟨f.depth, f.colour⟩ := by
  simp [stepPixel, h, passesDepthTest]

/-- A fragment aimed at a filled pixel replaces it only if strictly nearer. -/
theorem stepPixel_some {p : Nat} {f : Fragment} (h : p = f.pixel) (t : Sample) :
    stepPixel p (some t) f = if f.depth < t.depth then some ⟨f.depth, f.colour⟩ else some t := by
  simp [stepPixel, h, passesDepthTest]

/-- Two fragments that do not tie at a shared pixel can be drawn in either
order on one pixel with the same result. -/
theorem stepPixel_comm (p : Nat) (s : Option Sample) (x y : Fragment)
    (hxy : x.pixel = y.pixel → x.depth ≠ y.depth) :
    stepPixel p (stepPixel p s x) y = stepPixel p (stepPixel p s y) x := by
  by_cases hx : p = x.pixel <;> by_cases hy : p = y.pixel
  · have hne : x.depth ≠ y.depth := hxy (hx ▸ hy)
    cases s with
    | none =>
      rw [stepPixel_none hx, stepPixel_none hy, stepPixel_some hx, stepPixel_some hy]
      by_cases h1 : y.depth < x.depth
      · simp [h1, Nat.lt_asymm h1]
      · have h2 : x.depth < y.depth := by omega
        simp [h1, h2]
    | some t =>
      rw [stepPixel_some hx, stepPixel_some hy]
      by_cases h1 : x.depth < t.depth <;> by_cases h2 : y.depth < t.depth <;>
        simp only [h1, h2, ite_true, ite_false] <;>
        rw [stepPixel_some hx, stepPixel_some hy]
      · by_cases h3 : y.depth < x.depth
        · simp [h3, Nat.lt_asymm h3]
        · have h4 : x.depth < y.depth := by omega
          simp [h3, h4]
      · have h3 : ¬ y.depth < x.depth := by omega
        simp [h3, h1]
      · have h3 : ¬ x.depth < y.depth := by omega
        simp [h3, h2]
      · simp [h1, h2]
  · simp only [stepPixel_of_ne hy]
  · simp only [stepPixel_of_ne hx]
  · simp only [stepPixel_of_ne hx, stepPixel_of_ne hy]

/-- Two fragments that do not tie at a shared pixel can be drawn in either
order with the same result. -/
theorem writeFragment_comm (fb : Framebuffer) (x y : Fragment)
    (hxy : x.pixel = y.pixel → x.depth ≠ y.depth) :
    writeFragment (writeFragment fb x) y = writeFragment (writeFragment fb y) x := by
  funext p
  exact stepPixel_comm p (fb p) x y hxy

/-- In a pairwise-related list, any two members are equal or related one way or the other. -/
theorem Pairwise.rel_of_mem {α : Type} {R : α → α → Prop} {l : List α} (h : l.Pairwise R)
    {x y : α} (hx : x ∈ l) (hy : y ∈ l) : x = y ∨ R x y ∨ R y x := by
  induction l with
  | nil => cases hx
  | cons a t ih =>
    rw [List.pairwise_cons] at h
    rcases List.mem_cons.mp hx with hxa | hxt
    · rcases List.mem_cons.mp hy with hya | hyt
      · exact Or.inl (hxa.trans hya.symm)
      · exact Or.inr (Or.inl (hxa ▸ h.1 y hyt))
    · rcases List.mem_cons.mp hy with hya | hyt
      · exact Or.inr (Or.inr (hya ▸ h.1 x hxt))
      · exact ih h.2 hxt hyt

/-- **A2 (colour).** If no two fragments at the same pixel share a depth,
drawing the fragments in any order gives exactly the same framebuffer,
depths and colours included. -/
theorem draw_perm_of_noDepthTies {l₁ l₂ : List Fragment} (hperm : l₁.Perm l₂)
    (hties : NoDepthTies l₁) (fb : Framebuffer) : draw l₁ fb = draw l₂ fb := by
  unfold draw
  apply hperm.foldl_eq'
  intro x hx y hy z
  rcases Pairwise.rel_of_mem hties hx hy with rfl | h | h
  · rfl
  · exact writeFragment_comm z x y h
  · exact (writeFragment_comm z y x h).symm

/-! ## Hypotheses are satisfiable: concrete instances of every main theorem -/

/-- A1 on a concrete list. -/
example : NearestWins exampleFrags (draw exampleFrags Framebuffer.empty) :=
  draw_nearestWins exampleFrags

/-- The reordered list used below. -/
def exampleFragsSwapped : List Fragment :=
  [⟨0, 2, 0x00FF00⟩, ⟨0, 5, 0xFF0000⟩, ⟨1, 9, 0x0000FF⟩]

/-- The two example lists are permutations of each other. -/
theorem exampleFrags_perm : exampleFrags.Perm exampleFragsSwapped :=
  List.Perm.swap _ _ _

/-- A2 (depth) on a concrete permutation. -/
example : depthAt (draw exampleFrags Framebuffer.empty) 0
    = depthAt (draw exampleFragsSwapped Framebuffer.empty) 0 :=
  draw_depth_perm exampleFrags_perm _ _

/-- A2 (colour) on a concrete permutation, with the tie rule discharged by `decide`. -/
example : draw exampleFrags Framebuffer.empty = draw exampleFragsSwapped Framebuffer.empty :=
  draw_perm_of_noDepthTies exampleFrags_perm (by decide) _

/-! ## The spec rejects bad behaviour -/

/-- Two fragments at pixel 0 with the same depth 5 but different colours. -/
def tieFrags : List Fragment := [⟨0, 5, 0xAAAAAA⟩, ⟨0, 5, 0xBBBBBB⟩]

/-- The same two fragments in the other order. -/
def tieFragsSwapped : List Fragment := [⟨0, 5, 0xBBBBBB⟩, ⟨0, 5, 0xAAAAAA⟩]

theorem tieFrags_perm : tieFrags.Perm tieFragsSwapped := List.Perm.swap _ _ _

/-- **Ties break order independence.** With two fragments at the same
pixel and depth, the drawing order decides the colour.  This is why
`draw_perm_of_noDepthTies` needs its tie rule. -/
theorem tie_colour_depends_on_order :
    colourAt (draw tieFrags Framebuffer.empty) 0
      ≠ colourAt (draw tieFragsSwapped Framebuffer.empty) 0 := by
  decide

/-- Even so, the depth agrees (A2 depth needs no tie rule). -/
example : depthAt (draw tieFrags Framebuffer.empty) 0
    = depthAt (draw tieFragsSwapped Framebuffer.empty) 0 :=
  draw_depth_perm tieFrags_perm _ _

/-- A nearer fragment (depth 1) and a farther one (depth 7) at pixel 0. -/
def nearAndFar : List Fragment := [⟨0, 1, 0xFF0000⟩, ⟨0, 7, 0x0000FF⟩]

/-- A wrong framebuffer: pixel 0 holds the farther fragment. -/
def farWonFb : Framebuffer :=
  fun p => if p = 0 then some ⟨7, 0x0000FF⟩ else none

/-- **Negative example.** A framebuffer where the farther fragment won
does not satisfy `NearestWins`. -/
theorem farWon_not_nearestWins : ¬ NearestWins nearAndFar farWonFb := by
  intro h
  have := h.nearest 0 ⟨7, 0x0000FF⟩ rfl ⟨0, 1, 0xFF0000⟩ (by simp [nearAndFar]) rfl
  simp at this

/-- And the correct result does satisfy it, with the nearer one winning. -/
example : draw nearAndFar Framebuffer.empty 0 = some ⟨1, 0xFF0000⟩ := by decide

end Gpu.Depth
