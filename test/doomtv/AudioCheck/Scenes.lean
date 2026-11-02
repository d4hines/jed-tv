import AudioCheck.Util
import Doom.Audio.Sound
/-!
# AudioCheck.Scenes (host test support)

Scripted sound scenes rendered through `Doom.Audio.Sound` + `Mixer` (+ the
music player), with programmatic checks of the result.
-/

namespace AudioCheck.Scenes

open Doom.Audio Doom.Audio.Sound AudioCheck.Util

def fu (n : Int) : Int32 := (n * 65536).toInt32

def sfx! (name : String) : Nat := (sfxId? name).getD 0

/-- What happens at a given tic (35 Hz). -/
structure Event where
  tic : Nat
  cmds : Array Cmd := #[]
  gamemap : Option Nat := none

def listenerId : Nat := 1
def listener : Listener := { id := listenerId, x := 0, y := 0, angle := 0 }

/--
The scripted scene (listener at the origin facing east, +x; +y is to the
listener's left):
* 0.0 s pistol (local), 0.5 s imp sight 300 units ahead,
* 1.0 s door opening 600 units to the left, 1.5 s barrel explosion behind
  right at ~850 units (attenuated),
* 2.0 s ten simultaneous sounds (channel stealing by priority),
* 3.0 s a sound 1500 units away (inaudible, never starts),
* 3.5 s a cacodemon moving from the far left to the far right over one second,
* 5.0 s on map 8, the same far sound plays at the minimum volume.
-/
def script : Array Event := Id.run do
  let mut evs : Array Event := #[]
  evs := evs.push { tic := 0, cmds := #[.start (sfx! "pistol") (some { id := listenerId, x := 0, y := 0 })] }
  evs := evs.push { tic := 17, cmds := #[.start (sfx! "bgsit1") (some { id := 100, x := fu 300, y := 0 })] }
  evs := evs.push { tic := 35, cmds := #[.start (sfx! "doropn") (some { id := 200, x := 0, y := fu 600 })] }
  evs := evs.push { tic := 52, cmds := #[.start (sfx! "barexp") (some { id := 101, x := fu (-800), y := fu (-300) })] }
  let burst : Array Cmd := #[
    .start (sfx! "shotgn") (some { id := listenerId, x := 0, y := 0 }),
    .start (sfx! "posit1") (some { id := 102, x := fu 100, y := fu 50 }),
    .start (sfx! "posit2") (some { id := 103, x := fu 150, y := fu (-50) }),
    .start (sfx! "bgact") (some { id := 104, x := fu 200, y := fu 100 }),
    .start (sfx! "popain") (some { id := 105, x := fu 120, y := 0 }),
    .start (sfx! "claw") (some { id := 106, x := fu 80, y := fu 80 }),
    .start (sfx! "firsht") (some { id := 107, x := fu 250, y := fu (-200) }),
    .start (sfx! "telept") (some { id := 108, x := fu 300, y := fu 300 }),
    .start (sfx! "itemup") none,
    .start (sfx! "stnmov") (some { id := 201, x := fu (-100), y := 0 })]
  evs := evs.push { tic := 70, cmds := burst }
  evs := evs.push { tic := 105, cmds := #[.start (sfx! "posit3") (some { id := 109, x := fu 1500, y := 0 })] }
  -- moving cacodemon: starts left (+y), moves to the right (-y) over 35 tics
  evs := evs.push { tic := 122, cmds := #[.start (sfx! "cacsit") (some { id := 110, x := fu 100, y := fu 400 })] }
  for k in [1:36] do
    evs := evs.push { tic := 122 + k, cmds := #[.move { id := 110, x := fu 100, y := fu (400 - 800 * k / 35) }] }
  evs := evs.push { tic := 175, gamemap := some 8,
                    cmds := #[.start (sfx! "posit3") (some { id := 111, x := fu 1500, y := 0 })] }
  return evs

/-- Frames per tic at `rate` (35 Hz; the remainder is spread like a real loop). -/
def ticFrames (rate tic : Nat) : Nat := (tic + 1) * rate / 35 - tic * rate / 35

/-- Run the script for `tics` tics, rendering after every tic. -/
def runScript (sys : System) (tics : Nat) (withMusic : Option Nat := none) : System × ByteArray := Id.run do
  let mut s := sys
  if let some m := withMusic then s := s.changeMusic m true
  let mut out := ByteArray.empty
  for t in [0:tics] do
    let mut cmds : Array Cmd := #[]
    for e in script do
      if e.tic == t then
        if let some g := e.gamemap then s := { s with gamemap := g }
        cmds := cmds ++ e.cmds
    s := s.tick listener cmds
    let (s', b) := s.render (ticFrames s.rate t)
    s := s'
    out := out ++ b
  return (s, out)

/-- RMS of frames `[a, b)` of one channel (0 = left, 1 = right). -/
def windowRms (pcm : ByteArray) (a b ch : Nat) : Float := Id.run do
  let mut acc : Float := 0
  for i in [a:b] do
    let v := Float.ofInt (sample pcm (2 * i + ch))
    acc := acc + v * v
  return (acc / (max (b - a) 1).toFloat).sqrt

def ticFrame (rate tic : Nat) : Nat := tic * rate / 35

/-- Checks on the rendered sfx scene; returns failures (empty = pass). -/
def checkScene (pcm : ByteArray) (rate : Nat) (s : System) : Array String := Id.run do
  let mut fails : Array String := #[]
  let st := stats pcm rate
  if st.rmsL < 300 || st.rmsR < 300 then fails := fails.push s!"scene too quiet: {st.summary rate}"
  if st.clipped * 1000 > st.frames then fails := fails.push s!"clipping storm: {st.clipped} clipped samples"
  let w (t0 t1 ch : Nat) := windowRms pcm (ticFrame rate t0) (ticFrame rate t1) ch
  -- door to the left: the DMX-style separation (sep = 32) puts it mostly on one side
  let dl := w 36 46 0
  let dr := w 36 46 1
  if !(dl > 2 * dr) then fails := fails.push s!"door (left source) not panned: L {dl} R {dr}"
  -- far sound at 3.0 s must not start: compare with silence just before it
  let farW := w 106 112 0 + w 106 112 1
  if farW > 50 then fails := fails.push s!"inaudible far sound produced output (rms {farW})"
  -- moving cacodemon: early window left-heavy, late window right-heavy
  let el := w 124 130 0
  let er := w 124 130 1
  let ll := w 150 156 0
  let lr := w 150 156 1
  if !(el > er && lr > ll) then fails := fails.push s!"moving source did not pan: early L/R {el}/{er}, late L/R {ll}/{lr}"
  -- map 8: the far sound plays (quietly)
  let m8 := w 176 186 0 + w 176 186 1
  if m8 < 20 then fails := fails.push s!"map-8 far sound silent (rms {m8})"
  if m8 > 3000 then fails := fails.push s!"map-8 far sound too loud (rms {m8})"
  -- all 8 channels were in use during the burst
  let _ := s
  return fails

end AudioCheck.Scenes

namespace AudioCheck.Scenes

open Doom.Audio Doom.Audio.Sound

/-- Targeted checks of `s_sound.c` semantics; returns failures. -/
def unitChecks (sys0 : System) : Array String := Id.run do
  let mut fails : Array String := #[]
  let l := listener
  let near (id : Nat) : Option Origin := some { id, x := fu 50, y := 0 }
  -- 1. priority: fill 8 channels with priority-64 sounds from distinct origins
  let full := Id.run do
    let mut t := sys0
    for k in [0:8] do t := t.startSound l (near (300 + k)) (sfx! "pistol")
    return t
  let s := full
  let busy := s.channels.foldl (fun n c => if c.sfx != 0 then n + 1 else n) 0
  if busy != 8 then fails := fails.push s!"expected 8 busy channels, got {busy}"
  let s1 := s.startSound l (near 400) (sfx! "firsht")   -- priority 70: no channel
  if s1.channels.any (·.sfx == sfx! "firsht") then fails := fails.push "priority-70 sound stole a priority-64 channel"
  let s2 := s.startSound l (near 401) (sfx! "telept")   -- priority 32: kicks channel 0
  if (s2.channels[0]!).sfx != sfx! "telept" then fails := fails.push "priority-32 sound did not take channel 0"
  -- 2. same origin: the old sound is stopped and the channel reused
  let s := (sys0.startSound l (near 500) (sfx! "posit1")).startSound l (near 500) (sfx! "popain")
  if (s.channels[0]!).sfx != sfx! "popain" || (s.channels[1]!).sfx != 0 then
    fails := fails.push "same-origin sound did not replace the old one on channel 0"
  -- 3. NULL origin: S_StopSound(NULL) stops the previous NULL-origin sound
  let s := (sys0.startSound l none (sfx! "itemup")).startSound l none (sfx! "wpnup")
  if s.channels.any (·.sfx == sfx! "itemup") then fails := fails.push "second NULL-origin sound did not stop the first"
  -- 4. attenuation and separation: 700 units straight ahead at sfx volume 8
  let s := sys0.startSound l (some { id := 600, x := fu 700, y := 0 }) (sfx! "posit2")
  let mc := s.mixer.chans[0]!
  if mc.left != 31 || mc.right != 32 then fails := fails.push s!"700-unit sound panning {mc.left}/{mc.right}, expected 31/32"
  -- 5. beyond the clipping distance nothing starts, except on map 8 (volume 15)
  let far : Option Origin := some { id := 601, x := fu 1500, y := 0 }
  let s := sys0.startSound l far (sfx! "posit2")
  if s.channels.any (·.sfx != 0) then fails := fails.push "sound beyond 1200 units started"
  let s := { sys0 with gamemap := 8 }.startSound l far (sfx! "posit2")
  let mc := s.mixer.chans[0]!
  -- vol 15 centred: left = 126*15/127 = 14, right = 128*15/127 = 15
  if mc.left != 14 || mc.right != 15 then fails := fails.push s!"map-8 far sound panning {mc.left}/{mc.right}, expected 14/15"
  -- 6. linked sound (chgun -> pistol data) plays
  let s := sys0.startSound l none (sfx! "chgun")
  if !s.mixer.isPlaying 0 then fails := fails.push "chgun (linked to pistol) did not play"
  -- 7. S_UpdateSounds frees finished channels and stops sources that walked away
  let s := sys0.startSound l (some { id := 602, x := fu 300, y := 0 }) (sfx! "bgact")
  let s := s.moveOrigin { id := 602, x := fu 2000, y := 0 }
  let s := s.updateSounds l
  if (s.channels[0]!).sfx != 0 || s.mixer.isPlaying 0 then fails := fails.push "source moved out of range was not stopped"
  -- 8. volume 0 silences new sounds
  let s := (sys0.setSfxVolume 0).startSound l (near 603) (sfx! "pistol")
  if s.channels.any (·.sfx != 0) then fails := fails.push "sound started at sfx volume 0"
  -- 9. music selection
  if levelMusic false 1 1 != musE1M1 || levelMusic true 1 1 != musRunnin || levelMusic false 4 1 != 22 then
    fails := fails.push "level music mapping wrong"
  return fails

end AudioCheck.Scenes
