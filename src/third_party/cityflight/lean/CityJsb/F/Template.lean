import JsbExact.F.Convert
import CityJsb.Template

/-! `CityJsb.template` (the generated in-flight c172p state) as an `F.State`: the same numbers copied once at
start-up, so the generated file stays the single source. -/

namespace CityJsb.F

def template : JsbExact.F.State := JsbExact.F.State.ofGeneric CityJsb.template

end CityJsb.F
