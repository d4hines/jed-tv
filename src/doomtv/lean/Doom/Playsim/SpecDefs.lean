import Doom.Playsim.Fixed

/-! # Doom.Playsim.SpecDefs — `p_spec.h` enums and constants. -/

namespace Doom.Playsim.SpecDefs

open Doom.Playsim.Fixed

-- vldoor_e
def vld_normal : Int32 := 0
def vld_close30ThenOpen : Int32 := 1
def vld_close : Int32 := 2
def vld_open : Int32 := 3
def vld_raiseIn5Mins : Int32 := 4
def vld_blazeRaise : Int32 := 5
def vld_blazeOpen : Int32 := 6
def vld_blazeClose : Int32 := 7
-- ceiling_e
def lowerToFloor : Int32 := 0
def raiseToHighest : Int32 := 1
def lowerAndCrush : Int32 := 2
def crushAndRaise : Int32 := 3
def fastCrushAndRaise : Int32 := 4
def silentCrushAndRaise : Int32 := 5
-- floor_e
def lowerFloor : Int32 := 0
def lowerFloorToLowest : Int32 := 1
def turboLower : Int32 := 2
def raiseFloor : Int32 := 3
def raiseFloorToNearest : Int32 := 4
def raiseToTexture : Int32 := 5
def lowerAndChange : Int32 := 6
def raiseFloor24 : Int32 := 7
def raiseFloor24AndChange : Int32 := 8
def raiseFloorCrush : Int32 := 9
def raiseFloorTurbo : Int32 := 10
def donutRaise : Int32 := 11
def raiseFloor512 : Int32 := 12
-- stair_e
def build8 : Int32 := 0
def turbo16 : Int32 := 1
-- plattype_e
def perpetualRaise : Int32 := 0
def downWaitUpStay : Int32 := 1
def raiseAndChange : Int32 := 2
def raiseToNearestAndChange : Int32 := 3
def blazeDWUS : Int32 := 4
-- plat_e
def plat_up : Int32 := 0
def plat_down : Int32 := 1
def plat_waiting : Int32 := 2
def plat_in_stasis : Int32 := 3
-- result_e
def res_ok : Nat := 0
def res_crushed : Nat := 1
def res_pastdest : Nat := 2
-- bwhere_e
def bw_top : Int32 := 0
def bw_middle : Int32 := 1
def bw_bottom : Int32 := 2

def GLOWSPEED : Int32 := 8
def PLATWAIT : Int32 := 3
def PLATSPEED : Int32 := FRACUNIT
def VDOORSPEED : Int32 := 2 * FRACUNIT
def VDOORWAIT : Int32 := 150
def CEILSPEED : Int32 := FRACUNIT
def FLOORSPEED : Int32 := FRACUNIT
def TICRATE : Int32 := 35
def BUTTONTIME : Int32 := 35
/-- e6y: `floor->crush` of stair steps is uninitialized in vanilla. -/
def STAIRS_UNINITIALIZED_CRUSH_FIELD_VALUE : Int32 := 10

end Doom.Playsim.SpecDefs
