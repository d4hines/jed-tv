import Doom.Playsim.Defs
import Doom.Playsim.FinaleText
import Doom.Playsim.GameState
import Doom.Playsim.Sound

/-!
# Doom.Playsim.Finale

`f_finale.c` logic: `F_StartFinale`, `F_Ticker` (text timing, Doom II
"press a button" exit, the MAP30 cast sequence) and `F_CastResponder`.
-/

namespace Doom.Playsim.Finale

open Doom.Playsim.Defs
open Doom.Playsim.FinaleText
open Doom.Playsim.GameState
open Doom.Playsim.Sound

def F_STAGE_TEXT : Int32 := 0
def F_STAGE_ARTSCREEN : Int32 := 1
def F_STAGE_CAST : Int32 := 2
def TEXTSPEED : Int32 := 3
def TEXTWAIT : Int32 := 250

/-- Finale text id for the current map: index into `finaleTexts` (E1..E4 = 0..3,
C1..C6 = 4..9), or -1. -/
def finaleTextIndex (gs : GameState) : Int32 :=
  if gs.res.gamemode != gm_commercial then
    if gs.gamemap == 8 && gs.gameepisode >= 1 && gs.gameepisode <= 4 then gs.gameepisode - 1 else -1
  else
    let m := gs.gamemap
    if m == 6 then 4 else if m == 11 then 5 else if m == 20 then 6 else if m == 30 then 7
    else if m == 15 then 8 else if m == 31 then 9 else -1

/-- The finale text after DEHACKED replacement (`DEH_String(finaletext)`). -/
def finaleText (gs : GameState) (idx : Int32) : String :=
  match finaleTexts[idx.toNatClampNeg]? with
  | none => ""
  | some (key, orig) =>
    match gs.res.dehBex.find? (·.1 == key) with
    | some (_, v) => v
    | none => orig

/-- `F_StartFinale`. -/
def startFinale : M Unit := do
  modify fun gs => { gs with gameaction := ga_nothing, gamestate := gamestateFinale
                             viewactive := false, automapactive := false }
  if (← get).res.gamemode != gm_commercial then changeMusic mus_victor true
  else changeMusic mus_read_m true
  modify fun gs => { gs with finale := { finalestage := F_STAGE_TEXT, finalecount := 0
                                         textIndex := finaleTextIndex gs } }

/-- `castorder[]` mobj types. -/
def castorder : Array Int32 := #[MT_POSSESSED, MT_SHOTGUY, MT_CHAINGUY, MT_TROOP, MT_SERGEANT,
  MT_SKULL, MT_HEAD, MT_KNIGHT, MT_BRUISER, MT_BABY, MT_PAIN, MT_UNDEAD, MT_FATSO, MT_VILE,
  MT_SPIDER, MT_CYBORG, MT_PLAYER]

@[inline] def modF (f : FinaleState → FinaleState) : M Unit :=
  modify fun gs => { gs with finale := f gs.finale }

/-- `F_StartCast`. -/
def startCast : M Unit := do
  let gs ← get
  let st := gs.mobjinfo[castorder[0]!.toNatClampNeg]!.seestate
  set { gs with wipegamestate := -1
                finale := { gs.finale with
                            castnum := 0, caststate := st
                            casttics := gs.states[st.toNat]!.tics, castdeath := false
                            finalestage := F_STAGE_CAST, castframes := 0, castonmelee := 0
                            castattacking := false } }
  changeMusic mus_evil true

/-- Sound for a cast state transition (`F_CastTicker` switch). -/
def castSfx (st : UInt32) : Nat :=
  if st == S_PLAY_ATK1 then sfx_dshtgn else if st == S_POSS_ATK2 then sfx_pistol
  else if st == S_SPOS_ATK2 then sfx_shotgn else if st == S_VILE_ATK2 then sfx_vilatk
  else if st == S_SKEL_FIST2 then sfx_skeswg else if st == S_SKEL_FIST4 then sfx_skepch
  else if st == S_SKEL_MISS2 then sfx_skeatk
  else if st == S_FATT_ATK8 || st == S_FATT_ATK5 || st == S_FATT_ATK2 then sfx_firsht
  else if st == S_CPOS_ATK2 || st == S_CPOS_ATK3 || st == S_CPOS_ATK4 then sfx_shotgn
  else if st == S_TROO_ATK3 then sfx_claw else if st == S_SARG_ATK2 then sfx_sgtatk
  else if st == S_BOSS_ATK2 || st == S_BOS2_ATK2 || st == S_HEAD_ATK2 then sfx_firsht
  else if st == S_SKULL_ATK2 then sfx_sklatk
  else if st == S_SPID_ATK2 || st == S_SPID_ATK3 then sfx_shotgn
  else if st == S_BSPI_ATK2 then sfx_plasma
  else if st == S_CYBER_ATK2 || st == S_CYBER_ATK4 || st == S_CYBER_ATK6 then sfx_rlaunc
  else if st == S_PAIN_ATK3 then sfx_sklatk else 0

/-- `F_CastTicker`. -/
def castTicker : M Unit := do
  modF fun f => { f with casttics := f.casttics - 1 }
  let gs ← get
  if gs.finale.casttics > 0 then return
  let info (n : Int32) := gs.mobjinfo[castorder[n.toNatClampNeg]!.toNatClampNeg]!
  let cs := gs.states[gs.finale.caststate.toNat]!
  let mut stop := false
  if cs.tics == -1 || cs.nextstate == S_NULL then
    let n := if gs.finale.castnum + 1 >= castorder.size.toInt32 then 0 else gs.finale.castnum + 1
    if (info n).seesound != 0 then startSoundNull (info n).seesound.toNatClampNeg
    modF fun f => { f with castnum := n, castdeath := false, caststate := (info n).seestate, castframes := 0 }
  else
    if gs.finale.caststate == S_PLAY_ATK1 then stop := true
    else
      let st := cs.nextstate
      modF fun f => { f with caststate := st, castframes := f.castframes + 1 }
      let sfx := castSfx st
      if sfx != 0 then startSoundNull sfx
  let gs ← get
  let f := gs.finale
  let inf := info f.castnum
  if !stop && f.castframes == 12 then
    let st := if f.castonmelee != 0 then inf.meleestate else inf.missilestate
    let melee := f.castonmelee ^^^ 1
    let st := if st == S_NULL then (if melee != 0 then inf.meleestate else inf.missilestate) else st
    modF fun f => { f with castattacking := true, caststate := st, castonmelee := melee }
  let f := (← get).finale
  if stop || (f.castattacking && (f.castframes == 24 || f.caststate == inf.seestate)) then
    modF fun f => { f with castattacking := false, castframes := 0, caststate := inf.seestate }
  let gs ← get
  let t := gs.states[gs.finale.caststate.toNat]!.tics
  modF fun f => { f with casttics := if t == -1 then 15 else t }

/-- `F_Ticker`. -/
def fTicker : M Unit := do
  let gs ← get
  if gs.res.gamemode == gm_commercial && gs.finale.finalecount > 50 then
    let mut any := false
    for i in [0:4] do
      if gs.players[i]!.cmd.buttons != 0 then any := true
    if any then
      if gs.gamemap == 30 then startCast
      else modify fun gs => { gs with gameaction := ga_worlddone }
  modF fun f => { f with finalecount := f.finalecount + 1 }
  let gs ← get
  if gs.finale.finalestage == F_STAGE_CAST then
    castTicker
    return
  if gs.res.gamemode == gm_commercial then return
  let len := (finaleText gs gs.finale.textIndex).utf8ByteSize.toInt32
  if gs.finale.finalestage == F_STAGE_TEXT && gs.finale.finalecount > len * TEXTSPEED + TEXTWAIT then
    modify fun gs => { gs with wipegamestate := -1
                               finale := { gs.finale with finalecount := 0, finalestage := F_STAGE_ARTSCREEN } }
    if gs.gameepisode == 3 then startMusic mus_bunny

/-- `F_CastResponder` for a key press during the cast. Returns true if eaten. -/
def castResponder : M Bool := do
  let gs ← get
  if gs.finale.finalestage != F_STAGE_CAST then return false
  if gs.finale.castdeath then return true
  let inf := gs.mobjinfo[castorder[gs.finale.castnum.toNatClampNeg]!.toNatClampNeg]!
  modF fun f => { f with castdeath := true, caststate := inf.deathstate
                         casttics := gs.states[inf.deathstate.toNat]!.tics, castframes := 0
                         castattacking := false }
  if inf.deathsound != 0 then startSoundNull inf.deathsound.toNatClampNeg
  return true

end Doom.Playsim.Finale
