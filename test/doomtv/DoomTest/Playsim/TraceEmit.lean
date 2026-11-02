import Doom.Harness.Fnv
import DoomTest.Harness.TraceFormat
import Doom.Playsim.GameState
import Doom.Playsim.Thinker
/-!
# Doom.Playsim.TraceEmit

Build a `TicRecord` from `GameState` at the post-`G_Ticker` dump point.
-/

namespace Doom.Playsim.TraceEmit

open Doom.Harness.Fnv
open Doom.Harness.TraceFormat
open Doom.Playsim.GameState
open Doom.Playsim.Thinker

private def i32bits (x : Int32) : UInt32 := x.toUInt32

/-- FNV of all sectors' floor/ceiling (§5.4). -/
def sectorsDigest (gs : GameState) : UInt64 :=
  Id.run do
    let mut acc := ByteArray.emptyWithCapacity (gs.sectors.size * 8)
    for sec in gs.sectors do
      acc := pushU32LE acc (i32bits sec.floorheight)
      acc := pushU32LE acc (i32bits sec.ceilingheight)
    pure (fnv1a64 acc)

def emitPlayer (gs : GameState) (playerIndex : Nat) : Except String PlayerRec := do
  match gs.players[playerIndex]? with
  | none => throw "emitPlayer: missing"
  | some p =>
    let (moTraceId, x, y, z, momx, momy, momz, angle) ←
      if p.mo < 0 then
        pure ((0xffffffff : UInt32), (0 : Int32), 0, 0, 0, 0, 0, (0 : UInt32))
      else
        match gs.mobjs[p.mo.toNatClampNeg]? with
        | none => throw "emitPlayer: bad mo"
        | some mo =>
          pure (mo.traceId, mo.x, mo.y, mo.z, mo.momx, mo.momy, mo.momz, mo.angle)
    let ammo (i : Nat) : Int32 := match p.ammo[i]? with | some v => v | none => 0
    let wo (i : Nat) : Int32 := match p.weaponowned[i]? with | some v => v | none => 0
    pure {
      playerIndex := playerIndex.toUInt32
      moTraceId := moTraceId
      x := i32bits x
      y := i32bits y
      z := i32bits z
      momx := i32bits momx
      momy := i32bits momy
      momz := i32bits momz
      angle := angle
      viewz := i32bits p.viewz
      health := i32bits p.health
      armorpoints := i32bits p.armorpoints
      readyweapon := i32bits p.readyweapon
      pendingweapon := i32bits p.pendingweapon
      ammo0 := i32bits (ammo 0)
      ammo1 := i32bits (ammo 1)
      ammo2 := i32bits (ammo 2)
      ammo3 := i32bits (ammo 3)
      weaponowned0 := i32bits (wo 0)
      weaponowned1 := i32bits (wo 1)
      weaponowned2 := i32bits (wo 2)
      weaponowned3 := i32bits (wo 3)
      weaponowned4 := i32bits (wo 4)
      weaponowned5 := i32bits (wo 5)
      weaponowned6 := i32bits (wo 6)
      weaponowned7 := i32bits (wo 7)
      weaponowned8 := i32bits (wo 8)
      cmdForwardmove := i32bits p.cmd.forwardmove
      cmdSidemove := i32bits p.cmd.sidemove
      cmdAngleturn := i32bits p.cmd.angleturn
      cmdButtons := p.cmd.buttons
    }

/-- Payload `func` of a thinker list entry (§6.2). -/
def thinkerFunc (gs : GameState) (r : Nat) : UInt32 × UInt32 :=
  let k := thKind r
  let i := thIdx r
  if k == TK_MOBJ then let m := gs.mobjs[i]!; (m.traceId, m.func)
  else if k == TK_DOOR then let d := gs.verticalDoors[i]!; (d.traceId, d.func)
  else if k == TK_CEILING then let c := gs.ceilings[i]!; (c.traceId, c.func)
  else if k == TK_FLOOR then let f := gs.floors[i]!; (f.traceId, f.func)
  else if k == TK_PLAT then let p := gs.plats[i]!; (p.traceId, p.func)
  else if k == TK_FLASH then (gs.lightFlashes[i]!.traceId, THF_LIGHTFLASH)
  else if k == TK_STROBE then (gs.strobes[i]!.traceId, THF_STROBEFLASH)
  else if k == TK_GLOW then (gs.glows[i]!.traceId, THF_GLOW)
  else (gs.fireFlickers[i]!.traceId, THF_FIREFLICKER)

def emitThinker (gs : GameState) (r : Nat) : ThinkerRec :=
  let (traceId, func) := thinkerFunc gs r
  if func == THF_MOBJ then
    let mo := gs.mobjs[thIdx r]!
    { traceId, func
      mobj := some {
        x := i32bits mo.x, y := i32bits mo.y, z := i32bits mo.z
        momx := i32bits mo.momx, momy := i32bits mo.momy, momz := i32bits mo.momz
        angle := mo.angle, state := mo.state, tics := i32bits mo.tics
        health := i32bits mo.health, flags := mo.flags, type_ := i32bits mo.typeId } }
  else { traceId, func, mobj := none }

/-- Emit one tic record after `G_Ticker` (§5; non-level tics carry no state). -/
def emitTicRecord (gs : GameState) : Except String TicRecord := do
  if gs.gamestate != gamestateLevel then
    return { gametic := gs.gametic, inLevel := 0, leveltime := 0
             rndindex := gs.rng.rndindex, prndindex := gs.rng.prndindex
             players := #[], thinkers := #[], sectors := #[], sectorsDigest := emptyFnv }
  let mut players : Array PlayerRec := #[]
  for i in [0:4] do
    if gs.playeringame[i]! then
      players := players.push (← emitPlayer gs i)
  let thinkers := gs.thinkers.map (emitThinker gs)
  let mut sectors : Array SectorRec := #[]
  for s in [0:gs.sectors.size] do
    let sec := gs.sectors[s]!
    if sec.specialdata != -1 then
      sectors := sectors.push { sectorIndex := s.toUInt32, floorheight := i32bits sec.floorheight
                                ceilingheight := i32bits sec.ceilingheight }
  return { gametic := gs.gametic, inLevel := 1, leveltime := gs.leveltime
           rndindex := gs.rng.rndindex, prndindex := gs.rng.prndindex
           players, thinkers, sectors, sectorsDigest := sectorsDigest gs }

end Doom.Playsim.TraceEmit
