import Doom.Playsim.Defs
import Doom.Playsim.Flags
import Doom.Playsim.GameState
import Doom.Playsim.MapUtil
import Doom.Playsim.Sound

/-!
# Doom.Playsim.MobjBase

`P_SpawnMobj` and `P_RemoveMobj` (`p_mobj.c`): the parts of mobj life-cycle
that do not run state actions.
-/

namespace Doom.Playsim.MobjBase

open Doom.Playsim.Defs
open Doom.Playsim.Flags
open Doom.Playsim.GameState
open Doom.Playsim.MapUtil
open Doom.Playsim.Mobj
open Doom.Playsim.Sound
open Doom.Playsim.Thinker

def ONFLOORZ : Int32 := Int32.minValue
def ONCEILINGZ : Int32 := Int32.maxValue

/-- `P_SpawnMobj`. Does not run the spawn state's action. -/
def spawnMobj (x y z : Int32) (typeId : Int32) : M Nat := do
  let gs ← get
  let info := gs.mobjinfo[typeId.toNatClampNeg]!
  let reactiontime := if gs.gameskill != sk_nightmare then info.reactiontime else 0
  let (r, rng) := Doom.Playsim.Random.pRandom gs.rng
  let st := gs.states[info.spawnstate.toNat]!
  let mi := gs.mobjs.size
  let mo : Mobj := {
    Mobj.empty with
    typeId, x, y
    radius := info.radius
    height := info.height
    flags := info.flags
    health := info.spawnhealth
    reactiontime
    lastlook := r % 4
    state := info.spawnstate
    tics := st.tics
    sprite := st.sprite
    frame := st.frame
  }
  set { gs with mobjs := gs.mobjs.push mo, rng }
  setThingPosition mi
  let gs ← get
  let mo := gs.mobjs[mi]!
  let sec := gs.sectors[moSector gs mo]!
  let z' := if z == ONFLOORZ then sec.floorheight
            else if z == ONCEILINGZ then sec.ceilingheight - info.height
            else z
  let tid := gs.traceIdCounter
  set {
    gs with
    mobjs := gs.mobjs.modify mi fun m => { m with floorz := sec.floorheight, ceilingz := sec.ceilingheight, z := z', traceId := tid }
    thinkers := gs.thinkers.push (thRef TK_MOBJ mi)
    traceIdCounter := tid + 1 }
  return mi

/-- `P_RemoveMobj`. -/
def removeMobj (mi : Nat) : M Unit := do
  let gs ← get
  let mo := gs.mobjs[mi]!
  if (mo.flags &&& MF_SPECIAL) != 0 && (mo.flags &&& MF_DROPPED) == 0 &&
      mo.typeId != MT_INV && mo.typeId != MT_INS then
    let h := gs.iquehead
    let iquehead := (h + 1) &&& 127
    let iquetail := if iquehead == gs.iquetail then (gs.iquetail + 1) &&& 127 else gs.iquetail
    set {
      gs with
      itemrespawnque := gs.itemrespawnque.set! h
        { x := mo.spawnX, y := mo.spawnY, angle := mo.spawnAngle, type_ := mo.spawnType,
          options := mo.spawnOptions }
      itemrespawntime := gs.itemrespawntime.set! h gs.leveltime.toInt32
      iquehead, iquetail }
  unsetThingPosition mi
  stopSoundMo mi
  modifyMo mi fun m => { m with func := THF_REMOVED }

end Doom.Playsim.MobjBase
