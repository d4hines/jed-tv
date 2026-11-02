import Lake
open Lake DSL

-- Only the two libraries the TV plant needs; the upstream lakefile also builds
-- the desktop tools and the macOS dynamic library. The package keeps the
-- upstream name because Lean puts it in module initializer symbols, which
-- c/cityjsb_abi.c names (initialize_vibecheck_CityJsb_Plant).
package vibecheck where
  buildDir := "../../../../assets/build/lake/cityflight"
lean_lib JsbExact
lean_lib CityJsb
