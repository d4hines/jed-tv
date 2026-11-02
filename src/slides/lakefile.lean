import Lake
open Lake DSL
package slides where
  buildDir := "../../assets/build/lake/slides"
lean_lib Embedded where
  srcDir := "../../assets/build/lean-slides/generated"
  roots := #[`SlideAssets]
@[default_target] lean_lib Slides
/-- The TV app. Its host checks (`check`, `preview`) are test/slides. -/
lean_exe slides where root := `Main
