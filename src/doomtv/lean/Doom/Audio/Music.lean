import Doom.Audio.OplMusic
import Doom.Audio.Mus

/-!
# Doom.Audio.Music

High-level OPL music player: what the game's music calls (`I_RegisterSong`,
`I_PlaySong`, `I_StopSong`, `I_PauseSong`, `I_ResumeSong`,
`I_SetMusicVolume`) do in Chocolate Doom with the OPL backend, plus
rendering of the music stream.

Song lumps may be MIDI (`MThd`, under 96 KiB, as all Freedoom music is) or
MUS (anything else; converted with `Mus.toMidi`, i.e. `mus2mid`). A lump that
fails to load plays nothing, as in Chocolate.

Rendering: `render p frames rate` produces `frames` interleaved s16le stereo
frames. Each call is treated as one audio-callback buffer, so with the same
buffer sizes the output is sample-identical to Chocolate Doom's
`OPL_Mix_Callback` (SDL calls it once per buffer).
-/

namespace Doom.Audio.Music

open Doom.Audio

structure Player where
  st : OplMusic.State
  /-- `mus2mid`'s static per-channel velocity table (never reset in Chocolate). -/
  velocities : Array UInt8 := Mus.initialVelocities
  /-- Error of the last `start`, if the lump could not be loaded. -/
  lastError : Option String := none
deriving Inhabited

/--
A player at output rate `rate` (48000 on the TV, 44100 fallback) with music
volume `volume` (0..127; the menu's 0..15 times 8, default 64).
-/
def Player.init (gm : Genmidi.Genmidi) (rate : Nat := 48000) (cfg : OplMusic.Config := {})
    (volume : Nat := 64) (logWrites : Bool := false) : Player :=
  { st := OplMusic.State.init gm rate cfg volume logWrites }

/-- `I_RegisterSong`: the MIDI events Chocolate would play for this lump. -/
def Player.register (p : Player) (lump : ByteArray) : Player × Except String Midi.File :=
  if Midi.isMidLump lump then (p, Midi.parse lump)
  else
    match Mus.parse lump >>= (Mus.toMidi · p.velocities) with
    | .ok (f, vel) => ({ p with velocities := vel }, .ok f)
    | .error e => (p, .error e)

/-- Stop the current song (`S_StopMusic`'s driver calls: resume if paused, stop). -/
def Player.stop (p : Player) : Player :=
  let st := if p.st.paused then p.st.resumeSong else p.st
  { p with st := st.stopSong }

/--
Stop any current song and start `lump` (`S_ChangeMusic`'s driver calls).
An optional replacement GENMIDI bank invalidates the voices' cached
instruments.
-/
def Player.start (p : Player) (lump : ByteArray) (looping : Bool := true)
    (genmidi : Option Genmidi.Genmidi := none) : Player :=
  let p := p.stop
  let p := match genmidi with
    | some gm =>
      let voices := p.st.voices.map fun (v : OplMusic.Voice) => { v with currentInstr := OplMusic.noInstr }
      { p with st := { p.st with gm := gm, voices := voices } }
    | none => p
  match p.register lump with
  | (p, .ok file) => { p with st := p.st.playSong file looping, lastError := none }
  | (p, .error e) => { p with lastError := some e }

def Player.pause (p : Player) : Player := { p with st := p.st.pauseSong }
def Player.resume (p : Player) : Player := { p with st := p.st.resumeSong }
/-- Music volume 0..127. -/
def Player.setVolume (p : Player) (volume : Nat) : Player := { p with st := p.st.setMusicVolume (min volume 127) }
def Player.isPlaying (p : Player) : Bool := p.st.isPlaying
def Player.isPaused (p : Player) : Bool := p.st.paused

/-- Render one buffer of `frames` frames at `rate` Hz (s16le stereo, `frames * 4` bytes). -/
def Player.render (p : Player) (frames : Nat) (rate : Nat := 48000) : Player × ByteArray :=
  let st := p.st
  let p := { p with st := default }
  let st := st.setRate rate
  let (st, out) := st.mixBuffer (ByteArray.emptyWithCapacity (frames * 4)) frames
  ({ p with st := st }, out)

end Doom.Audio.Music
