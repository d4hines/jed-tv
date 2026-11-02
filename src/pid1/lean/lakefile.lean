import Lake
open Lake DSL
package tv where
  buildDir := "../../../assets/build/lake/tv"
@[default_target]
lean_lib TV
