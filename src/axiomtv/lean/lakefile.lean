import Lake
open Lake DSL

/-- Axiom: First Light (github.com/petergpt/lean-3d-game-axiom) on the TV.
The game is the upstream Lean code with Raylib replaced by the TV platform
layer in ../gfx (Rust) through ../native/lean_glue.c. Built for the TV by
tools/lean-arm/build-exe.sh. -/
package axiomtv where
  leanOptions := #[⟨`autoImplicit, false⟩]

lean_lib Axiom

/-- The TV app. -/
lean_exe axiomtv where root := `AxiomTV

/-- The upstream acceptance tests, run on the host. -/
lean_exe tests where root := `Tests
