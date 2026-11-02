import Lake
open Lake DSL

/-- Host checks of the Slides app (../../src/slides): the parser, layout and
renderer (`check`) and PPM renders of every slide (`preview`). -/
package «slides-test»

require slides from "../../src/slides"

lean_exe check where root := `Check
lean_exe preview where root := `Preview
