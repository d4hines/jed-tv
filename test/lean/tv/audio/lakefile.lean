import Lake
open Lake DSL
package audioTests where
  buildDir := "../../../../assets/build/lake/audio-tests"
require tv from "../../../../src/pid1/lean"
lean_lib BackendTests
lean_lib DeviceTests
lean_exe audioTests where
  root := `Main
