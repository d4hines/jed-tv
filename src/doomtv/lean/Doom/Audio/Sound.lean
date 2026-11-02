import Doom.Audio.Mixer
import Doom.Audio.Music
import Doom.Audio.Sfx
import Doom.Audio.Genmidi
import Doom.Playsim.Angle
import Doom.Playsim.Fixed
import Doom.Playsim.Tables

/-!
# Doom.Audio.Sound

The game-facing sound logic of Chocolate Doom's `s_sound.c` (with the tables
of `sounds.c`) as a pure state machine driving `Mixer` and `Music`:

* `startSound` = `S_StartSound` (link volume/pitch, audibility and stereo
  separation from `S_AdjustSoundParams`, including the map-8 boss-level rule
  that never clips and keeps a minimum volume of 15, the "same position =
  centre" rule, `S_StopSound(origin)` and `S_GetChannel`'s channel choice by
  origin and priority);
* `stopSound` = `S_StopSound`, `updateSounds` = `S_UpdateSounds` (per tic);
* `changeMusic` / `startLevel` = `S_ChangeMusic` / `S_Start` (Doom 1
  `d_e1m1`... with the episode 4 table, Doom 2 `d_runnin`..., `D_INTROA`
  for OPL);
* `setSfxVolume` / `setMusicVolume` take the menu's 0..15 (internally 0..127
  as `x * 8`), `pauseSound` / `resumeSound`.

Inputs are plain values: a `Listener` (the console player's mobj id,
position and angle) and `Cmd`s (`start sfx origin`, `stop origin`,
`move origin`). Origins are identified by a caller-chosen `Nat` id that must
be unique among things that can make sounds (mobjs and sector sound
origins); `none` is the C `NULL` origin. Moving sources are tracked through
`move` commands (the C code reads the live mobj position each tic).

Pitch: `S_StartSound` draws `M_Random` for pitch variation; this module
never touches the playsim RNG. A caller may pass the drawn value as `rnd` to
reproduce the pitch; the mixer only applies pitch when created with
`pitchShift` (Chocolate's Doom default is off).
-/

namespace Doom.Audio.Sound

open Doom.Audio
open Doom.Playsim.Fixed (FRACUNIT fixedMul wabs)
open Doom.Playsim.Angle (pointToAngle2)
open Doom.Playsim.Tables (finesine)

/-! ## Tables (`sounds.c`) -/

structure SfxInfo where
  name : String
  priority : Nat
  link : Option Nat := none
  pitch : Int := -1
  volume : Int := -1
deriving Inhabited, Repr

private def snd (name : String) (priority : Nat) : SfxInfo := { name, priority }

/-- `S_sfx[]` (index = `sfxenum_t`). -/
def sfxTable : Array SfxInfo := #[
  snd "none" 0, snd "pistol" 64, snd "shotgn" 64, snd "sgcock" 64, snd "dshtgn" 64,
  snd "dbopn" 64, snd "dbcls" 64, snd "dbload" 64, snd "plasma" 64, snd "bfg" 64,
  snd "sawup" 64, snd "sawidl" 118, snd "sawful" 64, snd "sawhit" 64, snd "rlaunc" 64,
  snd "rxplod" 70, snd "firsht" 70, snd "firxpl" 70, snd "pstart" 100, snd "pstop" 100,
  snd "doropn" 100, snd "dorcls" 100, snd "stnmov" 119, snd "swtchn" 78, snd "swtchx" 78,
  snd "plpain" 96, snd "dmpain" 96, snd "popain" 96, snd "vipain" 96, snd "mnpain" 96,
  snd "pepain" 96, snd "slop" 78, snd "itemup" 78, snd "wpnup" 78, snd "oof" 96,
  snd "telept" 32, snd "posit1" 98, snd "posit2" 98, snd "posit3" 98, snd "bgsit1" 98,
  snd "bgsit2" 98, snd "sgtsit" 98, snd "cacsit" 98, snd "brssit" 94, snd "cybsit" 92,
  snd "spisit" 90, snd "bspsit" 90, snd "kntsit" 90, snd "vilsit" 90, snd "mansit" 90,
  snd "pesit" 90, snd "sklatk" 70, snd "sgtatk" 70, snd "skepch" 70, snd "vilatk" 70,
  snd "claw" 70, snd "skeswg" 70, snd "pldeth" 32, snd "pdiehi" 32, snd "podth1" 70,
  snd "podth2" 70, snd "podth3" 70, snd "bgdth1" 70, snd "bgdth2" 70, snd "sgtdth" 70,
  snd "cacdth" 70, snd "skldth" 70, snd "brsdth" 32, snd "cybdth" 32, snd "spidth" 32,
  snd "bspdth" 32, snd "vildth" 32, snd "kntdth" 32, snd "pedth" 32, snd "skedth" 32,
  snd "posact" 120, snd "bgact" 120, snd "dmact" 120, snd "bspact" 100, snd "bspwlk" 100,
  snd "vilact" 100, snd "noway" 78, snd "barexp" 60, snd "punch" 64, snd "hoof" 70,
  snd "metal" 70, { name := "chgun", priority := 64, link := some 1, pitch := 150, volume := 0 },
  snd "tink" 60, snd "bdopn" 100, snd "bdcls" 100, snd "itmbk" 100, snd "flame" 32,
  snd "flamst" 32, snd "getpow" 60, snd "bospit" 70, snd "boscub" 70, snd "bossit" 70,
  snd "bospn" 70, snd "bosdth" 70, snd "manatk" 70, snd "mandth" 70, snd "sssit" 70,
  snd "ssdth" 70, snd "keenpn" 70, snd "keendt" 70, snd "skeact" 70, snd "skesit" 70,
  snd "skeatk" 70, snd "radio" 60]

def numSfx : Nat := sfxTable.size

/-- `S_music[]` names (index = `musicenum_t`, 0 = `mus_None`). -/
def musicNames : Array String := #["",
  "e1m1", "e1m2", "e1m3", "e1m4", "e1m5", "e1m6", "e1m7", "e1m8", "e1m9",
  "e2m1", "e2m2", "e2m3", "e2m4", "e2m5", "e2m6", "e2m7", "e2m8", "e2m9",
  "e3m1", "e3m2", "e3m3", "e3m4", "e3m5", "e3m6", "e3m7", "e3m8", "e3m9",
  "inter", "intro", "bunny", "victor", "introa",
  "runnin", "stalks", "countd", "betwee", "doom", "the_da", "shawn", "ddtblu", "in_cit", "dead",
  "stlks2", "theda2", "doom2", "ddtbl2", "runni2", "dead2", "stlks3", "romero", "shawn2", "messag",
  "count2", "ddtbl3", "ampie", "theda3", "adrian", "messg2", "romer2", "tense", "shawn3", "openin",
  "evil", "ultima", "read_m", "dm2ttl", "dm2int"]

def numMusic : Nat := musicNames.size
def musE1M1 : Nat := 1
def musInter : Nat := 28
def musIntro : Nat := 29
def musIntroa : Nat := 32
def musRunnin : Nat := 33

def sfxSawup : Nat := 10
def sfxSawhit : Nat := 13
def sfxItemup : Nat := 32
def sfxTink : Nat := 87

/-- Look up an sfx id by name (e.g. `"pistol"`). -/
def sfxId? (name : String) : Option Nat := sfxTable.findIdx? (·.name == name)
/-- Look up a music id by name (e.g. `"e1m1"`). -/
def musicId? (name : String) : Option Nat := musicNames.findIdx? (·  == name)

/-! ## Constants (`s_sound.c`) -/

def clippingDist : Int32 := 1200 * FRACUNIT
def closeDist : Int32 := 200 * FRACUNIT
/-- `S_ATTENUATOR = (S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS`. -/
def attenuator : Int := 1000
def stereoSwing : Int32 := 96 * FRACUNIT
def normPitch : Int := 128
def normSep : Int := 128

/-! ## State -/

structure Listener where
  /-- Origin id of the console player's mobj. -/
  id : Nat
  x : Int32
  y : Int32
  angle : UInt32
deriving Inhabited, Repr

structure Origin where
  id : Nat
  x : Int32
  y : Int32
deriving Inhabited, Repr

inductive Cmd where
  /-- `S_StartSound(origin, sfx)`; `rnd` = the `M_Random()` value drawn for pitch, if known. -/
  | start (sfx : Nat) (origin : Option Origin) (rnd : Option Nat := none)
  /-- `S_StopSound(origin)`. -/
  | stop (origin : Option Nat)
  /-- A sound origin moved. -/
  | move (origin : Origin)
deriving Inhabited, Repr

structure Channel where
  /-- `sfxinfo` (0 = free). -/
  sfx : Nat := 0
  origin : Option Nat := none
  ox : Int32 := 0
  oy : Int32 := 0
  handle : Int := -1
  pitch : Nat := 128
deriving Inhabited, Repr

structure Config where
  /-- `snd_channels`. -/
  channels : Nat := 8
  /-- Output rate (48000 on the TV, 44100 fallback). -/
  rate : Nat := 48000
  /-- Menu volumes, 0..15. -/
  sfxVolume : Nat := 8
  musicVolume : Nat := 8
  /-- `snd_pitchshift` (Doom default off). -/
  pitchShift : Bool := false
  opl : OplMusic.Config := {}
deriving Inhabited

structure System where
  /-- Playable sample data per sfx id (links resolved; `none` if missing/invalid). -/
  sfxData : Array (Option Sfx.Sfx)
  /-- Music lump per music id. -/
  musicLumps : Array (Option ByteArray)
  introaPresent : Bool
  channels : Array Channel
  mixer : Mixer.Mixer
  music : Music.Player
  sfxVolume : Nat
  musicVolume : Nat
  /-- `snd_SfxVolume` (0..127). -/
  sndSfxVolume : Int
  musPlaying : Option Nat := none
  musPaused : Bool := false
  /-- `gamemap` (for the map-8 rule). -/
  gamemap : Nat := 1
  rate : Nat
deriving Inhabited

def upper (s : String) : String := s.map Char.toUpper

/--
`S_Init` + precaching: build the system from a lump lookup (by name, e.g. a
WAD directory). Needs `GENMIDI`; missing or invalid sound lumps simply never
play (vanilla would abort).
-/
def System.create (lump : String → Option ByteArray) (cfg : Config := {}) : Except String System := do
  let gmBytes ← match lump "GENMIDI" with
    | some b => pure b
    | none => throw "GENMIDI lump not found"
  let gm ← Genmidi.parse gmBytes
  let sfxData := sfxTable.mapIdx fun i info =>
    if i == 0 then none
    else
      let target := match info.link with | some l => sfxTable[l]! | none => info
      (lump ("DS" ++ upper target.name)).bind fun b => (Sfx.parse b).toOption
  let musicLumps := musicNames.mapIdx fun i n => if i == 0 then none else lump ("D_" ++ upper n)
  return { sfxData, musicLumps, introaPresent := (lump "D_INTROA").isSome,
           channels := Array.replicate cfg.channels {},
           mixer := Mixer.Mixer.init cfg.channels cfg.rate cfg.pitchShift,
           music := Music.Player.init gm cfg.rate cfg.opl (min cfg.musicVolume 15 * 8),
           sfxVolume := min cfg.sfxVolume 15, musicVolume := min cfg.musicVolume 15,
           sndSfxVolume := (min cfg.sfxVolume 15 * 8 : Nat), rate := cfg.rate }

@[inline] def System.modChannel (s : System) (i : Nat) (f : Channel → Channel) : System :=
  let cs := s.channels
  let s := { s with channels := #[] }
  { s with channels := cs.modify i f }

/-- `I_SoundIsPlaying(handle)`. -/
def System.handlePlaying (s : System) (h : Int) : Bool :=
  h >= 0 && s.mixer.isPlaying h.toNat

/-- `S_StopChannel`. -/
def System.stopChannel (s : System) (cnum : Nat) : System :=
  let c := s.channels[cnum]!
  if c.sfx == 0 then s
  else
    let s := if s.handlePlaying c.handle then { s with mixer := s.mixer.stop c.handle.toNat } else s
    s.modChannel cnum fun c => { c with sfx := 0, origin := none }

/-- `S_StopSound`: stop the first channel playing from `origin` (`none` = `NULL`). -/
def System.stopSound (s : System) (origin : Option Nat) : System := Id.run do
  for i in [0:s.channels.size] do
    let c := s.channels[i]!
    if c.sfx != 0 && c.origin == origin then return s.stopChannel i
  return s

/-- `S_GetChannel`. -/
def System.getChannel (s : System) (origin : Option Nat) (sfx : Nat) : System × Option Nat := Id.run do
  let n := s.channels.size
  let mut s := s
  let mut cnum := n
  for i in [0:n] do
    let c := s.channels[i]!
    if c.sfx == 0 then
      cnum := i
      break
    else if origin.isSome && c.origin == origin then
      s := s.stopChannel i
      cnum := i
      break
  if cnum == n then
    -- look for a channel playing something of lower or equal priority
    for i in [0:n] do
      if (sfxTable[(s.channels[i]!).sfx]!).priority >= (sfxTable[sfx]!).priority then
        cnum := i
        break
    if cnum == n then return (s, none)
    s := s.stopChannel cnum
  s := s.modChannel cnum fun c => { c with sfx := sfx, origin := origin }
  return (s, some cnum)

/--
`S_AdjustSoundParams`: `(audible, vol, sep)` for a source at `(sx, sy)`.
-/
def adjustSoundParams (gamemap : Nat) (sndSfxVolume : Int) (l : Listener) (sx sy : Int32) :
    Bool × Int × Int :=
  let adx := wabs (l.x - sx)
  let ady := wabs (l.y - sy)
  let approx := adx + ady - ((if adx < ady then adx else ady) >>> 1)
  if gamemap != 8 && approx > clippingDist then (false, 0, 0)
  else
    let angle := pointToAngle2 l.x l.y sx sy
    let angle := if angle > l.angle then angle - l.angle else angle + (0xffffffff - l.angle)
    let fine := (angle >>> 19).toNat
    let sep : Int := 128 - (fixedMul stereoSwing (finesine[fine]!) >>> 16).toInt
    let vol : Int :=
      if approx < closeDist then sndSfxVolume
      else if gamemap == 8 then
        let a := if approx > clippingDist then clippingDist else approx
        15 + ((sndSfxVolume - 15) * ((clippingDist - a) >>> 16).toInt).tdiv attenuator
      else
        (sndSfxVolume * ((clippingDist - approx) >>> 16).toInt).tdiv attenuator
    (vol > 0, vol, sep)

/-- `CheckVolumeSeparation` of `i_sound.c`. -/
def clampVolSep (vol sep : Int) : Int × Int :=
  (if vol < 0 then 0 else if vol > 127 then 127 else vol,
   if sep < 0 then 0 else if sep > 254 then 254 else sep)

/-- `I_StartSound`: start the mixer channel; the handle or -1. -/
def System.iStartSound (s : System) (sfx : Nat) (cnum : Nat) (vol sep : Int) (pitch : Nat) : System × Int :=
  match s.sfxData[sfx]? with
  | some (some data) =>
    if cnum < s.mixer.numChannels then
      let (v, sp) := clampVolSep vol sep
      ({ s with mixer := s.mixer.start cnum data v sp pitch }, cnum)
    else (s, -1)
  | _ =>
    -- invalid or missing lump: whatever played on this mixer channel is released
    if cnum < s.mixer.numChannels then ({ s with mixer := s.mixer.stop cnum }, -1) else (s, -1)

/-- `S_StartSound`. -/
def System.startSound (s : System) (l : Listener) (origin : Option Origin) (sfxId : Nat)
    (rnd : Option Nat := none) : System := Id.run do
  if sfxId < 1 || sfxId >= numSfx then return s
  let info := sfxTable[sfxId]!
  let mut volume := s.sndSfxVolume
  let mut pitch : Int := normPitch
  if info.link.isSome then
    volume := volume + info.volume
    pitch := info.pitch
    if volume < 1 then return s
    if volume > s.sndSfxVolume then volume := s.sndSfxVolume
  let mut sep : Int := normSep
  match origin with
  | some o =>
    if o.id != l.id then
      let (rc, v, sp) := adjustSoundParams s.gamemap s.sndSfxVolume l o.x o.y
      volume := v
      sep := if o.x == l.x && o.y == l.y then normSep else sp
      if !rc then return s
  | none => pure ()
  match rnd with
  | some r =>
    if sfxId >= sfxSawup && sfxId <= sfxSawhit then pitch := pitch + 8 - ((r % 256) &&& 15 : Nat)
    else if sfxId != sfxItemup && sfxId != sfxTink then pitch := pitch + 16 - ((r % 256) &&& 31 : Nat)
  | none => pure ()
  let pitchN : Nat := if pitch < 0 then 0 else if pitch > 255 then 255 else pitch.toNat
  let oid := origin.map (·.id)
  let mut s := s.stopSound oid
  let (s', cnum?) := s.getChannel oid sfxId
  s := s'
  match cnum? with
  | none => return s
  | some cnum =>
    let (ox, oy) := match origin with | some o => (o.x, o.y) | none => (0, 0)
    s := s.modChannel cnum fun c => { c with pitch := pitchN, ox := ox, oy := oy }
    let (s', h) := s.iStartSound sfxId cnum volume sep pitchN
    return s'.modChannel cnum fun c => { c with handle := h }

/-- `S_UpdateSounds(listener)`. -/
def System.updateSounds (s : System) (l : Listener) : System := Id.run do
  let mut s := s
  for cnum in [0:s.channels.size] do
    let c := s.channels[cnum]!
    if c.sfx == 0 then continue
    if s.handlePlaying c.handle then
      let info := sfxTable[c.sfx]!
      -- a linked sound whose volume offset silences it is stopped; otherwise the
      -- volume and separation come entirely from S_AdjustSoundParams
      if info.link.isSome && s.sndSfxVolume + info.volume < 1 then
        s := s.stopChannel cnum
        continue
      match c.origin with
      | some oid =>
        if oid != l.id then
          let (audible, v, sp) := adjustSoundParams s.gamemap s.sndSfxVolume l c.ox c.oy
          if !audible then s := s.stopChannel cnum
          else
            let (v, sp) := clampVolSep v sp
            s := { s with mixer := s.mixer.update c.handle.toNat v sp }
      | none => pure ()
    else
      s := s.stopChannel cnum
  return s

/-- Record a new position for an origin (all channels playing from it). -/
def System.moveOrigin (s : System) (o : Origin) : System := Id.run do
  let mut s := s
  for i in [0:s.channels.size] do
    if (s.channels[i]!).origin == some o.id then
      s := s.modChannel i fun c => { c with ox := o.x, oy := o.y }
  return s

/-- One game tic: apply the commands in order, then `S_UpdateSounds`. -/
def System.tick (s : System) (l : Listener) (cmds : Array Cmd) : System := Id.run do
  let mut s := s
  for c in cmds do
    s := match c with
      | .start sfx origin rnd => s.startSound l origin sfx rnd
      | .stop origin => s.stopSound origin
      | .move o => s.moveOrigin o
  return s.updateSounds l

/-! ## Music -/

/-- `S_StopMusic`. -/
def System.stopMusic (s : System) : System :=
  match s.musPlaying with
  | none => s
  | some _ => { s with music := s.music.stop, musPlaying := none }

/-- `S_ChangeMusic(musicnum, looping)`. -/
def System.changeMusic (s : System) (musicnum0 : Nat) (looping : Bool) : System :=
  let musicnum := if musicnum0 == musIntro && s.introaPresent then musIntroa else musicnum0
  if musicnum == 0 || musicnum >= numMusic then s
  else if s.musPlaying == some musicnum then s
  else
    let s := s.stopMusic
    match s.musicLumps[musicnum]! with
    | some lump => { s with music := s.music.start lump looping, musPlaying := some musicnum }
    | none => s

/-- `S_StartMusic`. -/
def System.startMusic (s : System) (musicnum : Nat) : System := s.changeMusic musicnum false

/-- Music for a level (`S_Start`'s choice). -/
def levelMusic (commercial : Bool) (episode map : Nat) : Nat :=
  if commercial then musRunnin + map - 1
  else if episode < 4 then musE1M1 + (episode - 1) * 9 + map - 1
  else #[22, 20, 21, 5, 16, 13, 15, 14, 9][map - 1]!

/-- `S_Start`: stop all channels, then start (looping) the level's music. -/
def System.startLevel (s : System) (commercial : Bool) (episode map : Nat) : System := Id.run do
  let mut s := { s with gamemap := map }
  for cnum in [0:s.channels.size] do
    if (s.channels[cnum]!).sfx != 0 then s := s.stopChannel cnum
  s := { s with musPaused := false }
  return s.changeMusic (levelMusic commercial episode map) true

/-- `S_PauseSound`. -/
def System.pauseSound (s : System) : System :=
  if s.musPlaying.isSome && !s.musPaused then { s with music := s.music.pause, musPaused := true } else s

/-- `S_ResumeSound`. -/
def System.resumeSound (s : System) : System :=
  if s.musPlaying.isSome && s.musPaused then { s with music := s.music.resume, musPaused := false } else s

/-- Sound effects volume, menu units 0..15. -/
def System.setSfxVolume (s : System) (v : Nat) : System :=
  let v := min v 15
  { s with sfxVolume := v, sndSfxVolume := (v * 8 : Nat) }

/-- Music volume, menu units 0..15. -/
def System.setMusicVolume (s : System) (v : Nat) : System :=
  let v := min v 15
  { s with musicVolume := v, music := s.music.setVolume (v * 8) }

/-- Render `frames` frames of mixed output (sfx + music), s16le stereo. -/
def System.render (s : System) (frames : Nat) : System × ByteArray :=
  let music := s.music
  let mixer := s.mixer
  let s := { s with music := default, mixer := default }
  let (music, mbuf) := music.render frames s.rate
  let (mixer, out) := mixer.mix frames mbuf
  ({ s with music := music, mixer := mixer }, out)

end Doom.Audio.Sound
