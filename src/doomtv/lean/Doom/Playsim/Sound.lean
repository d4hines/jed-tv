import Doom.Playsim.Defs
import Doom.Playsim.Fixed
import Doom.Playsim.GameState
import Doom.Playsim.Random

/-!
# Doom.Playsim.Sound

`S_StartSound` / `S_StopSound` / `S_ChangeMusic` (`s_sound.c`) as the playsim
sees them: the pitch-variation `M_Random` draw (taken only when the sound is
audible to the console player, exactly as the oracle decides it) and the
per-tic audio event list in `GameState.soundEvents`.
-/

namespace Doom.Playsim.Sound

open Doom.Playsim.Defs
open Doom.Playsim.Fixed
open Doom.Playsim.GameState
open Doom.Playsim.Random

/-- The `origin` pointer passed to `S_StartSound`. -/
inductive SndOrg where
  | none
  | mobj (idx : Nat)
  /-- `&sector->soundorg`. -/
  | sector (idx : Nat)
  /-- `&buttonlist[i].soundorg` from `P_UpdateSpecials` (vanilla bug: the
  pointer's address, not the pointer, is passed). -/
  | buttonField (i : Nat) (sectorIdx : Int32)

def S_CLIPPING_DIST : Int32 := 1200 * FRACUNIT
def S_CLOSE_DIST : Int32 := 200 * FRACUNIT
def S_ATTENUATOR : Int32 := 1000
/-- `S_Init(sfxVolume * 8)` with the default `sfxVolume = 8`. -/
def snd_SfxVolume : Int32 := 64

/-- `S_AdjustSoundParams` return value (volume > 0) for a source at `(sx, sy)`. -/
def adjustAudible (gamemap : Int32) (lx ly sx sy : Int32) : Bool :=
  let adx := wabs (lx - sx)
  let ady := wabs (ly - sy)
  let approx := adx + ady - ((if adx < ady then adx else ady) >>> 1)
  if gamemap != 8 && approx > S_CLIPPING_DIST then false
  else if approx < S_CLOSE_DIST then snd_SfxVolume > 0
  else if gamemap == 8 then
    let d := if approx > S_CLIPPING_DIST then S_CLIPPING_DIST else approx
    let v : Int32 := 15 + ((snd_SfxVolume - 15) * ((S_CLIPPING_DIST - d) >>> (16 : Int32))) / S_ATTENUATOR
    v > 0
  else
    let v : Int32 := (snd_SfxVolume * ((S_CLIPPING_DIST - approx) >>> (16 : Int32))) / S_ATTENUATOR
    v > 0

/-- `S_StartSound(origin, sfx_id)`. -/
def startSound (org : SndOrg) (sfx : Nat) : M Unit := do
  let gs ← get
  let cp := gs.players[gs.consoleplayer]!
  let listener? := if cp.mo >= 0 then some gs.mobjs[cp.mo.toNatClampNeg]! else Option.none
  let (origin, audible) : SoundOrigin × Bool :=
    match org with
    | .none => (.none, true)
    | .mobj i =>
      let mo := gs.mobjs[i]!
      let ev := SoundOrigin.mobj i.toUInt32 mo.x mo.y
      if cp.mo == i.toInt32 then (ev, true)
      else match listener? with
        | some l => (ev, adjustAudible gs.gamemap l.x l.y mo.x mo.y)
        | Option.none => (ev, true)
    | .sector s =>
      let sec := gs.level.sectors[s]!
      let ev := SoundOrigin.sector sec.soundorgX sec.soundorgY
      match listener? with
      | some l => (ev, adjustAudible gs.gamemap l.x l.y sec.soundorgX sec.soundorgY)
      | Option.none => (ev, true)
    | .buttonField i secIdx =>
      -- The C "mobj" overlays the next button: x/y are the halves of
      -- `buttonlist[i+1].soundorg` (0,0 when that slot is empty).
      let ev :=
        if secIdx >= 0 then
          let sec := gs.level.sectors[secIdx.toNatClampNeg]!
          SoundOrigin.sector sec.soundorgX sec.soundorgY
        else SoundOrigin.none
      let nextBusy := i + 1 < gs.buttons.size && gs.buttons[i + 1]!.soundorg >= 0
      match listener? with
      | some l =>
        if nextBusy then (ev, gs.gamemap == 8)
        else (ev, adjustAudible gs.gamemap l.x l.y 0 0)
      | Option.none => (ev, true)
  if audible && sfx != sfx_itemup && sfx != sfx_tink then
    let (_, rng) := mRandom gs.rng
    set { gs with rng, soundEvents := gs.soundEvents.push (.start sfx origin true) }
  else
    set { gs with soundEvents := gs.soundEvents.push (.start sfx origin audible) }

@[inline] def startSoundMo (mobjIdx : Nat) (sfx : Nat) : M Unit := startSound (.mobj mobjIdx) sfx
@[inline] def startSoundSec (secIdx : Nat) (sfx : Nat) : M Unit := startSound (.sector secIdx) sfx
@[inline] def startSoundNull (sfx : Nat) : M Unit := startSound .none sfx

/-- `S_StopSound(mobj)` (called by `P_RemoveMobj`). -/
def stopSoundMo (mobjIdx : Nat) : M Unit := modify fun gs =>
  let mo := gs.mobjs[mobjIdx]!
  { gs with soundEvents := gs.soundEvents.push (.stop (.mobj mobjIdx.toUInt32 mo.x mo.y)) }

/-- `S_ChangeMusic(musicnum, looping)`: no-op if that music already plays. -/
def changeMusic (mus : Nat) (looping : Bool) : M Unit := modify fun gs =>
  if gs.musPlaying == mus then gs
  else { gs with musPlaying := mus, soundEvents := gs.soundEvents.push (.music mus looping) }

/-- `S_StartMusic(m)`. -/
def startMusic (mus : Nat) : M Unit := changeMusic mus false

/-- `S_Start`: level music (`s_sound.c`). -/
def sStart : M Unit := do
  let gs ← get
  let mnum : Nat :=
    if gs.res.gamemode == gm_commercial then
      mus_runnin + gs.gamemap.toNatClampNeg - 1
    else
      let spmus : Array Nat := #[mus_e3m4, mus_e3m2, mus_e3m3, mus_e1m5, mus_e2m7,
        mus_e2m4, mus_e2m6, mus_e2m5, mus_e1m9]
      if gs.gameepisode < 4 then
        mus_e1m1 + (gs.gameepisode.toNatClampNeg - 1) * 9 + gs.gamemap.toNatClampNeg - 1
      else spmus[gs.gamemap.toNatClampNeg - 1]!
  changeMusic mnum true

end Doom.Playsim.Sound
