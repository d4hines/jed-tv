import Lake
open Lake DSL System

-- Only the two libraries the TV plant needs; the upstream lakefile also builds
-- the desktop tools and the macOS dynamic library. The package keeps the
-- upstream name because Lean puts it in module initializer symbols, which
-- c/cityjsb_abi.c names (initialize_vibecheck_CityJsb_Plant).
package vibecheck where
  buildDir := "../../../../assets/build/lake/cityflight"
@[default_target] lean_lib JsbExact
@[default_target] lean_lib CityJsb

-- Host-only: the differential test and timing of the generic model against
-- the `Float` model (`lake exe jsbdiff`, source in Tools/JsbDiff.lean). It
-- needs the jx_* sin/cos externs of JsbExact/Scalar.lean at link time, which
-- c/jxmath_host.c provides (demo/flight.nix writes its own for the TV).
-- `lake build JsbExact CityJsb`, which the TV build runs, does not touch this.
target jxmath.o pkg : FilePath := do
  let oFile := pkg.buildDir / "c" / "jxmath_host.o"
  let srcJob ← inputTextFile <| pkg.dir / ".." / "c" / "jxmath_host.c"
  buildO oFile srcJob #[] #["-O2", "-fno-builtin"] "cc"

lean_exe jsbdiff where
  root := `Tools.JsbDiff
  moreLinkObjs := #[jxmath.o]
