import Doom.Playsim.Angle
import Doom.Playsim.Core
import Doom.Playsim.Defs
import Doom.Playsim.GameState
import Doom.Playsim.MapUtil
import Doom.Playsim.MobjBase
import Doom.Playsim.MobjThink
import Doom.Playsim.Movers
import Doom.Playsim.Player
import Doom.Playsim.Sound
import Doom.Playsim.SpecUtil
import Doom.Playsim.Switch
import Doom.Playsim.User

/-!
# Doom.Playsim.Tick

`P_Ticker` (`p_tick.c`): players, `P_RunThinkers`, `P_UpdateSpecials`,
`P_RespawnSpecials`; and the level-time `ST_Ticker` / `HU_Ticker`.
-/

namespace Doom.Playsim.Tick

open Doom.Playsim.Angle
open Doom.Playsim.Core
open Doom.Playsim.Defs
open Doom.Playsim.Fixed
open Doom.Playsim.Flags
open Doom.Playsim.GameState
open Doom.Playsim.MapUtil
open Doom.Playsim.MobjBase
open Doom.Playsim.MobjThink
open Doom.Playsim.Movers
open Doom.Playsim.Player
open Doom.Playsim.Random
open Doom.Playsim.Sound
open Doom.Playsim.SpecUtil
open Doom.Playsim.Switch
open Doom.Playsim.Thinker
open Doom.Playsim.User

private def setArr {α : Type} (arr : Array α) (i : Nat) (v : α) : Array α :=
  if h : i < arr.size then arr.set i v else arr

def CF_GODMODE : Int32 := 2

def ST_TICRATE : Int32 := 35
def ST_NUMPAINFACES : Int32 := 5
def ST_NUMSTRAIGHTFACES : Int32 := 3
def ST_FACESTRIDE : Int32 := 8
def ST_TURNOFFSET : Int32 := 3
def ST_OUCHOFFSET : Int32 := 5
def ST_EVILGRINOFFSET : Int32 := 6
def ST_RAMPAGEOFFSET : Int32 := 7
def ST_GODFACE : Int32 := 40
def ST_DEADFACE : Int32 := 41
def ST_EVILGRINCOUNT : Int32 := 70
def ST_STRAIGHTFACECOUNT : Int32 := 17
def ST_TURNCOUNT : Int32 := 35
def ST_RAMPAGEDELAY : Int32 := 70
def ST_MUCHPAIN : Int32 := 20

/-- `ST_calcPainOffset`. -/
def calcPainOffset (st : StFaceState) (health0 : Int32) : StFaceState × Int32 :=
  let health := if health0 > 100 then (100 : Int32) else health0
  if health != st.painOldhealth then
    let lastcalc := ST_FACESTRIDE * (((100 - health) * ST_NUMPAINFACES) / 101)
    ({ st with painOffset := lastcalc, painOldhealth := health }, lastcalc)
  else
    (st, st.painOffset)

/-- `ST_updateFaceWidget`. -/
def updateFaceWidget (gs : GameState) (plyr : Player) (stRandom : Int32) : StFaceState :=
  Id.run do
    let mut st := gs.stFace
    if st.priority < 10 then
      if plyr.health == 0 then
        st := { st with priority := 9, faceindex := ST_DEADFACE, facecount := 1 }
    if st.priority < 9 then
      if plyr.bonuscount != 0 then
        let mut doevilgrin := false
        let mut owned := st.oldweaponsowned
        let mut i : Nat := 0
        while i < NUMWEAPONS do
          let cur := match plyr.weaponowned[i]? with | some v => v | none => (0 : Int32)
          let old := match owned[i]? with | some v => v | none => (0 : Int32)
          if old != cur then
            doevilgrin := true
            owned := setArr owned i cur
          i := i + 1
        st := { st with oldweaponsowned := owned }
        if doevilgrin then
          let (st1, pain) := calcPainOffset st plyr.health
          st := {
            st1 with
            priority := 8
            facecount := ST_EVILGRINCOUNT
            faceindex := pain + ST_EVILGRINOFFSET
          }
    if st.priority < 8 then
      if plyr.damagecount != 0 && plyr.attacker >= 0 && plyr.attacker != plyr.mo then
        match gs.mobjs[plyr.mo.toNatClampNeg]?, gs.mobjs[plyr.attacker.toNatClampNeg]? with
        | some pmo, some att =>
          st := { st with priority := 7 }
          if plyr.health - st.oldhealth > ST_MUCHPAIN then
            let (st1, pain) := calcPainOffset st plyr.health
            st := { st1 with facecount := ST_TURNCOUNT, faceindex := pain + ST_OUCHOFFSET }
          else
            let badguyangle := pointToAngle2 pmo.x pmo.y att.x att.y
            let turnRight :=
              if badguyangle > pmo.angle then
                (badguyangle - pmo.angle) > ANG180
              else
                (pmo.angle - badguyangle) <= ANG180
            let diffang :=
              if badguyangle > pmo.angle then badguyangle - pmo.angle
              else pmo.angle - badguyangle
            let (st1, pain) := calcPainOffset st plyr.health
            let face :=
              if diffang < ANG45 then pain + ST_RAMPAGEOFFSET
              else if turnRight then pain + ST_TURNOFFSET
              else pain + ST_TURNOFFSET + 1
            st := { st1 with facecount := ST_TURNCOUNT, faceindex := face }
        | _, _ => pure ()
    if st.priority < 7 then
      if plyr.damagecount != 0 then
        if plyr.health - st.oldhealth > ST_MUCHPAIN then
          let (st1, pain) := calcPainOffset st plyr.health
          st := {
            st1 with
            priority := 7
            facecount := ST_TURNCOUNT
            faceindex := pain + ST_OUCHOFFSET
          }
        else
          let (st1, pain) := calcPainOffset st plyr.health
          st := {
            st1 with
            priority := 6
            facecount := ST_TURNCOUNT
            faceindex := pain + ST_RAMPAGEOFFSET
          }
    if st.priority < 6 then
      if plyr.attackdown then
        if st.lastattackdown == -1 then
          st := { st with lastattackdown := ST_RAMPAGEDELAY }
        else
          let next := st.lastattackdown - 1
          if next == 0 then
            let (st1, pain) := calcPainOffset st plyr.health
            st := {
              st1 with
              priority := 5
              faceindex := pain + ST_RAMPAGEOFFSET
              facecount := 1
              lastattackdown := 1
            }
          else
            st := { st with lastattackdown := next }
      else
        st := { st with lastattackdown := -1 }
    if st.priority < 5 then
      let god :=
        (plyr.cheats &&& CF_GODMODE) != 0 ||
          (match plyr.powers[pw_invulnerability]? with
            | some n => n != 0
            | none => false)
      if god then
        st := { st with priority := 4, faceindex := ST_GODFACE, facecount := 1 }
    if st.facecount == 0 then
      let (st1, pain) := calcPainOffset st plyr.health
      st := {
        st1 with
        faceindex := pain + (stRandom % (3 : Int32))
        facecount := ST_STRAIGHTFACECOUNT
        priority := 0
      }
    { st with facecount := st.facecount - 1 }


/-- Is this thinker entry marked for removal? -/
@[inline] def thRemoved (gs : GameState) (r : Nat) : Bool :=
  let k := thKind r
  let i := thIdx r
  if k == TK_MOBJ then gs.mobjs[i]!.func == THF_REMOVED
  else if k == TK_DOOR then gs.verticalDoors[i]!.func == THF_REMOVED
  else if k == TK_CEILING then gs.ceilings[i]!.func == THF_REMOVED
  else if k == TK_FLOOR then gs.floors[i]!.func == THF_REMOVED
  else if k == TK_PLAT then gs.plats[i]!.func == THF_REMOVED
  else false

/-- Run one thinker (`currentthinker->function.acp1`), skipping NULL functions. -/
def runOne (r : Nat) : M Unit := do
  let k := thKind r
  let i := thIdx r
  if k == TK_MOBJ then mobjThinker i
  else if k == TK_DOOR then verticalDoorThinker i
  else if k == TK_CEILING then
    if (← get).ceilings[i]!.func == THF_MOVECEILING then moveCeilingThinker i
  else if k == TK_FLOOR then moveFloorThinker i
  else if k == TK_PLAT then
    if (← get).plats[i]!.func == THF_PLATRAISE then platRaiseThinker i
  else if k == TK_FLASH then lightFlashThinker i
  else if k == TK_STROBE then strobeFlashThinker i
  else if k == TK_GLOW then glowThinker i
  else if k == TK_FIRE then fireFlickerThinker i

/-- `P_RunThinkers`: run in list order (including thinkers added during the
pass) and drop entries marked removed when the walk reaches them. -/
def runThinkers : M Unit := do
  let mut i := 0
  let mut w := 0
  while i < (← get).thinkers.size do
    let gs ← get
    let r := gs.thinkers[i]!
    if !thRemoved gs r then
      runOne r
      if w != i then modify fun gs => { gs with thinkers := gs.thinkers.set! w r }
      w := w + 1
    i := i + 1
  modify fun gs => { gs with thinkers := gs.thinkers.shrink w }

/-- `P_UpdateSpecials`. -/
def updateSpecials : M Unit := do
  let gs ← get
  if gs.levelTimer then
    set { gs with levelTimeCount := gs.levelTimeCount - 1 }
    if gs.levelTimeCount - 1 == 0 then gExitLevel
  -- animated flats/textures: take the arrays out of the state to update in place
  let gs ← get
  let lt := gs.leveltime.toInt32
  let anims := gs.picAnims
  let mut flats := gs.flattranslation
  let mut texs := gs.texturetranslation
  set { gs with flattranslation := #[], texturetranslation := #[] }
  for anim in anims do
    let mut i := anim.basepic
    while i < anim.basepic + anim.numpics do
      let pic := anim.basepic + ((lt / anim.speed + i) % anim.numpics)
      let ix := i.toNatClampNeg
      if anim.istexture then
        if texs[ix]! != pic then texs := texs.set! ix pic
      else
        if flats[ix]! != pic then flats := flats.set! ix pic
      i := i + 1
  modify fun gs => { gs with flattranslation := flats, texturetranslation := texs }
  -- scrolling walls (special 48)
  let gs ← get
  if gs.lineSpecials.size > 0 then
    let specials := gs.lineSpecials
    let mut sides := gs.level.sides
    let lines := gs.level.lines
    set { gs with level := { gs.level with sides := #[] } }
    for l in specials do
      let ld := lines[l]!
      if ld.special == 48 then
        sides := sides.modify ld.sidenum0.toNatClampNeg fun s => { s with textureoffset := s.textureoffset + FRACUNIT }
    modify fun gs => { gs with level := { gs.level with sides } }
  -- buttons
  for bi in [0:16] do
    let b := (← get).buttons[bi]!
    if b.btimer != 0 then
      modify fun gs => { gs with buttons := gs.buttons.modify bi fun b => { b with btimer := b.btimer - 1 } }
      if b.btimer - 1 == 0 then
        let side := (← get).level.lines[b.line.toNatClampNeg]!.sidenum0.toNatClampNeg
        setSideTexture side b.where_ b.btexture
        startSound (.buttonField bi b.soundorg) sfx_swtchn
        modify fun gs => { gs with buttons := gs.buttons.set! bi {} }

/-- `P_RespawnSpecials` (altdeath item respawn). -/
def respawnSpecials : M Unit := do
  let gs ← get
  if gs.deathmatchMode != 2 then return
  if gs.iquehead == gs.iquetail then return
  if gs.leveltime.toInt32 - gs.itemrespawntime[gs.iquetail]! < 30 * 35 then return
  let mt := gs.itemrespawnque[gs.iquetail]!
  let x := mt.x <<< 16
  let y := mt.y <<< 16
  let ss := pointInSubsector gs.level x y
  let mo ← spawnMobj x y gs.sectors[gs.level.subsectors[ss]!.sector.toNat]!.floorheight MT_IFOG
  startSoundMo mo sfx_itmbk
  let gs ← get
  match gs.mobjinfo.findIdx? (·.doomednum == mt.type_) with
  | none =>
    iError s!"P_RespawnSpecials: Failed to find mobj type with doomednum {mt.type_} when respawning thing. This would cause a buffer overrun in vanilla Doom"
  | some i =>
    let z := if (gs.mobjinfo[i]!.flags &&& MF_SPAWNCEILING) != 0 then ONCEILINGZ else ONFLOORZ
    let mo ← spawnMobj x y z i.toInt32
    modifyMo mo fun m => {
      m with
      spawnX := mt.x, spawnY := mt.y, spawnAngle := mt.angle
      spawnType := mt.type_, spawnOptions := mt.options, angle := ANG45 * (mt.angle / 45).toUInt32 }
    modify fun gs => { gs with iquetail := (gs.iquetail + 1) &&& 127 }

/-- `P_Ticker`. -/
def pTicker : M Unit := do
  let gs ← get
  if gs.paused then return
  if !gs.netgame && gs.menuactive && !gs.demoplayback && gs.players[gs.consoleplayer]!.viewz != 1 then
    return
  for i in [0:4] do
    if (← get).playeringame[i]! then playerThink i
  runThinkers
  updateSpecials
  respawnSpecials
  modify fun gs => { gs with leveltime := gs.leveltime + 1 }

/-- `ST_Ticker`: one `M_Random`, then the face widget. -/
def stTicker : M Unit := do
  let stRandom ← mRandomM
  let gs ← get
  let plyr := gs.players[gs.consoleplayer]!
  let st := updateFaceWidget gs plyr stRandom
  set { gs with stFace := { st with oldhealth := plyr.health } }

def HU_MSGTIMEOUT : Int32 := 140

/-- `HU_Ticker` (message widget only). -/
def huTicker : M Unit := modify fun (gs : GameState) => Id.run do
  let mut hu := gs.hu
  if hu.messageCounter != 0 then
    hu := { hu with messageCounter := hu.messageCounter - 1 }
    if hu.messageCounter == 0 then hu := { hu with messageOn := false }
  let mut gs := { gs with hu }
  let plyr := gs.players[gs.consoleplayer]!
  match plyr.message with
  | some msg =>
    gs := { gs with players := gs.players.modify gs.consoleplayer fun p => { p with message := none }
                    hu := { hu with messageText := msg, messageOn := true, messageCounter := HU_MSGTIMEOUT } }
  | none => pure ()
  return gs

end Doom.Playsim.Tick
