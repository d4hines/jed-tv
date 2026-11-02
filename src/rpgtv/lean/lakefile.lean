import Lake
open Lake DSL
package rpgTv where
  buildDir := "../../../assets/build/lake/rpgtv"
lean_lib Rpg
@[default_target] lean_lib RpgTv
