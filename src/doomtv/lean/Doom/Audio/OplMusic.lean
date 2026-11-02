import Doom.Audio.Opl
import Doom.Audio.Midi
import Doom.Audio.Genmidi
import Doom.Audio.OplMusicTables

/-!
# Doom.Audio.OplMusic

A port of Chocolate Doom's OPL music driver: `i_oplmusic.c` (DMX voice
allocation, GENMIDI instruments, frequency/volume tables, pitch bend,
percussion, the Doom 1.666 / Doom 2 1.666 / 1.9 driver variants, music
volume, OPL3 stereo option), together with the parts of `opl.c`,
`opl_sdl.c` and `opl_queue.c` that decide *when* things happen: register
writes go through `OPL3_WriteRegBuffered`, track events are timer callbacks
in a binary heap keyed by microseconds, and output is produced exactly like
`OPL_Mix_Callback` (generate up to the next callback, rounding the sample
count up, then advance time by the truncated number of microseconds and run
the due callbacks). Rendering the same song with the same buffer sizes
therefore produces the same samples as Chocolate Doom (validated by
`AudioCheck/c/music_ref.c`).

Copyright (C) 1993-1996 Id Software, Inc.; Copyright (C) 2005-2014 Simon
Howard (GPL-2.0-or-later). Lean port distributed under the same license.

Hot path: only `Opl.Chip.renderStream`; everything here runs per MIDI event.
-/

namespace Doom.Audio.OplMusic

open Doom.Audio

/-- `opl_driver_ver_t`. -/
inductive DriverVer where
  | doom1_1_666
  | doom2_1_666
  | doom_1_9
deriving BEq, Inhabited, Repr

def DriverVer.rank : DriverVer → Nat
  | .doom1_1_666 => 0
  | .doom2_1_666 => 1
  | .doom_1_9 => 2

def numVoicesOpl2 : Nat := 9
def voiceOperators0 : Array Nat := #[0x00, 0x01, 0x02, 0x08, 0x09, 0x0a, 0x10, 0x11, 0x12]
def voiceOperators1 : Array Nat := #[0x03, 0x04, 0x05, 0x0b, 0x0c, 0x0d, 0x13, 0x14, 0x15]

def regsTremolo : Nat := 0x20
def regsLevel : Nat := 0x40
def regsAttack : Nat := 0x60
def regsSustain : Nat := 0x80
def regsWaveform : Nat := 0xE0
def regsFreq1 : Nat := 0xA0
def regsFreq2 : Nat := 0xB0
def regsFeedback : Nat := 0xC0

/-- `NULL` instrument / channel. -/
def noInstr : Nat := 0xFFFF
def noChan : Nat := 0xFF
/-- Queue callback id for `RestartSong` (others are track numbers). -/
def cbRestart : Nat := 0xFFFF
def opsSecond : UInt64 := 1000000

structure Voice where
  index : Nat
  op1 : Nat
  op2 : Nat
  array : Nat
  currentInstr : Nat := noInstr
  currentInstrVoice : Nat := 0
  channel : Nat := noChan
  key : Nat := 0
  note : Nat := 0
  freq : Nat := 0
  noteVolume : Nat := 0
  carVolume : Nat := 0
  modVolume : Nat := 0
  regPan : Nat := 0
  priority : Nat := 0
deriving Inhabited

structure ChanData where
  instrument : Nat := 0
  volume : Nat := 0
  volumeBase : Nat := 0
  pan : Nat := 0
  bend : Int := 0
deriving Inhabited

structure QEntry where
  time : UInt64
  cb : Nat
deriving Inhabited

structure Config where
  driver : DriverVer := .doom_1_9
  /-- DMXOPTION `-opl3`: 18 voices and OPL3 stereo panning. -/
  opl3 : Bool := false
  /-- DMXOPTION `-reverse`: correct (non-DMX) stereo orientation. -/
  stereoCorrect : Bool := false
deriving Inhabited, Repr

structure State where
  chip : Opl.Chip
  gm : Genmidi.Genmidi
  cfg : Config
  numVoices : Nat
  voices : Array Voice
  freeList : Array Nat
  allocList : Array Nat
  channels : Array ChanData
  tracks : Array (Array Midi.Event) := #[]
  trackPos : Array Nat := #[]
  numTracks : Nat := 0
  runningTracks : Nat := 0
  looping : Bool := false
  ticksPerBeat : Nat := 1
  usPerBeat : Nat := 500000
  startMusicVolume : Nat := 0
  currentMusicVolume : Nat := 0
  /-- Output (mixing) sample rate. -/
  rate : Nat
  currentTime : UInt64 := 0
  pauseOffset : UInt64 := 0
  paused : Bool := false
  queue : Array QEntry := #[]
  /-- Frames rendered so far (for the optional register-write log). -/
  framesOut : Nat := 0
  logWrites : Bool := false
  /-- `(frame, reg, value)` of every write that reached the chip (if `logWrites`). -/
  writeLog : Array (Nat × Nat × Nat) := #[]
deriving Inhabited

def emptyChip : Opl.Chip := default

/-! ## Chip access -/

@[inline] def State.withChip (s : State) (f : Opl.Chip → Opl.Chip) : State :=
  let chip := s.chip
  let s := { s with chip := emptyChip }
  { s with chip := f chip }

/-- `OPL_WriteRegister` through the SDL driver: timer registers are handled by
the driver itself; everything else is a buffered chip write. -/
def State.write (s : State) (reg val : Nat) : State :=
  if reg == 2 || reg == 3 || reg == 4 then s
  else
    let s := if s.logWrites then { s with writeLog := s.writeLog.push (s.framesOut, reg, val) } else s
    s.withChip (·.writeRegBuffered reg.toUInt32 val.toUInt32)

@[inline] def State.modVoice (s : State) (i : Nat) (f : Voice → Voice) : State :=
  let vs := s.voices
  let s := { s with voices := #[] }
  { s with voices := vs.modify i f }

@[inline] def State.modChannel (s : State) (i : Nat) (f : ChanData → ChanData) : State :=
  let cs := s.channels
  let s := { s with channels := #[] }
  { s with channels := cs.modify i f }

def State.instr (s : State) (i : Nat) : Genmidi.Instr := s.gm.instrs[i]!
def State.voice (s : State) (i : Nat) : Voice := s.voices[i]!
def State.chan (s : State) (i : Nat) : ChanData := s.channels[i]!

def vmt (i : Nat) : Nat := (volumeMappingTable[min i 127]!).toNat

/-! ## Callback queue (`opl_queue.c`) -/

def maxQueue : Nat := 64

def queuePush (q : Array QEntry) (time : UInt64) (cb : Nat) : Array QEntry := Id.run do
  if q.size >= maxQueue then return q
  let mut q := q.push { time, cb }
  let mut id := q.size - 1
  while id > 0 do
    let parent := (id - 1) / 2
    if time >= (q[parent]!).time then break
    q := q.set! id q[parent]!
    id := parent
  q := q.set! id { time, cb }
  return q

/-- Pop the minimum; returns its callback id. -/
def queuePop (q : Array QEntry) : Array QEntry × Nat := Id.run do
  let top := (q[0]!).cb
  let n := q.size - 1
  let entry := q[n]!
  let mut q := q.pop
  if n == 0 then return (q, top)
  let mut i := 0
  let mut go := true
  while go do
    let c1 := i * 2 + 1
    let c2 := i * 2 + 2
    let next :=
      if c1 < n && (q[c1]!).time < entry.time then
        if c2 < n && (q[c2]!).time < (q[c1]!).time then c2 else c1
      else if c2 < n && (q[c2]!).time < entry.time then c2
      else n
    if next == n then go := false
    else
      q := q.set! i q[next]!
      i := next
  q := q.set! i entry
  return (q, top)

/-- `OPL_SDL_SetCallback`. -/
def State.setCallback (s : State) (us : UInt64) (cb : Nat) : State :=
  { s with queue := queuePush s.queue (s.currentTime - s.pauseOffset + us) cb }

/-- `OPL_Queue_AdjustCallbacks` (single-precision float, as in C). -/
def State.adjustCallbacks (s : State) (factor : Float32) : State :=
  let t := s.currentTime
  { s with queue := s.queue.map fun e =>
      let offset : Int64 := (e.time - t).toInt64
      { e with time := t + ((offset.toFloat32 / factor).toInt64).toUInt64 } }

/-! ## Voices -/

/-- `InitVoices`. -/
def initVoices (numVoices : Nat) : Array Voice × Array Nat := Id.run do
  let mut vs : Array Voice := #[]
  let mut free : Array Nat := #[]
  for i in [0:numVoices] do
    vs := vs.push { index := i % numVoicesOpl2, op1 := voiceOperators0[i % numVoicesOpl2]!,
                    op2 := voiceOperators1[i % numVoicesOpl2]!, array := (i / numVoicesOpl2) <<< 8 }
    free := free.push i
  return (vs, free)

/-- `VoiceKeyOff`. -/
def State.voiceKeyOff (s : State) (vi : Nat) : State :=
  let v := s.voice vi
  s.write ((regsFreq2 + v.index) ||| v.array) (v.freq >>> 8)

/-- `GetFreeVoice`: the voice index, or `none`. -/
def State.getFreeVoice (s : State) : State × Option Nat :=
  if s.freeList.isEmpty then (s, none)
  else
    let r := s.freeList[0]!
    ({ s with freeList := s.freeList.eraseIdx! 0, allocList := s.allocList.push r }, some r)

/-- `ReleaseVoice` (including the Doom 2 1.666 out-of-range "crash" emulation
and the 1.666 double-voice release). -/
partial def State.releaseVoice (s : State) (index : Nat) : State :=
  if index >= s.allocList.size then
    { s with allocList := #[], freeList := #[] }
  else
    let vi := s.allocList[index]!
    let s := s.voiceKeyOff vi
    let double := (s.voice vi).currentInstrVoice != 0
    let s := s.modVoice vi fun v => { v with channel := noChan, note := 0 }
    let s := { s with allocList := s.allocList.eraseIdx! index, freeList := s.freeList.push vi }
    if double && s.cfg.driver.rank < DriverVer.doom_1_9.rank then s.releaseVoice index else s

/-- `LoadOperatorData`: returns the level written (stored as the voice volume). -/
def State.loadOperatorData (s : State) (op : Nat) (data : Genmidi.Op) (maxLevel : Bool) : State × Nat :=
  let level := data.scale.toNat ||| (if maxLevel then 0x3f else data.level.toNat)
  let s := s.write (regsLevel + op) level
  let s := s.write (regsTremolo + op) data.tremolo.toNat
  let s := s.write (regsAttack + op) data.attack.toNat
  let s := s.write (regsSustain + op) data.sustain.toNat
  let s := s.write (regsWaveform + op) data.waveform.toNat
  (s, level)

/-- `SetVoiceInstrument`. -/
def State.setVoiceInstrument (s : State) (vi : Nat) (instr : Nat) (instrVoice : Nat) : State :=
  let v := s.voice vi
  if v.currentInstr == instr && v.currentInstrVoice == instrVoice then s
  else
    let data := (s.instr instr).voice instrVoice
    let modulating := (data.feedback &&& 1) == 0
    let (s, car) := s.loadOperatorData (v.op2 ||| v.array) data.carrier true
    let (s, md) := s.loadOperatorData (v.op1 ||| v.array) data.modulator (!modulating)
    let s := s.write ((regsFeedback + v.index) ||| v.array) (data.feedback.toNat ||| v.regPan)
    s.modVoice vi fun v =>
      { v with currentInstr := instr, currentInstrVoice := instrVoice, carVolume := car, modVolume := md,
               priority := 0x0f - (data.carrier.attack.toNat >>> 4) + 0x0f - (data.carrier.sustain.toNat &&& 0x0f) }

/-- `SetVoiceVolume`. -/
def State.setVoiceVolume (s : State) (vi : Nat) (volume : Nat) : State :=
  let s := s.modVoice vi fun v => { v with noteVolume := volume }
  let v := s.voice vi
  let ov := (s.instr v.currentInstr).voice v.currentInstrVoice
  let midiVolume := 2 * (vmt (s.chan v.channel).volume + 1)
  let fullVolume := (vmt v.noteVolume * midiVolume) >>> 9
  let carVolume := 0x3f - fullVolume
  if carVolume != (v.carVolume &&& 0x3f) then
    let newCar := carVolume ||| (v.carVolume &&& 0xc0)
    let s := s.modVoice vi fun v => { v with carVolume := newCar }
    let s := s.write ((regsLevel + v.op2) ||| v.array) newCar
    if (ov.feedback &&& 1) != 0 && ov.modulator.level != 0x3f then
      let mv := ov.modulator.level.toNat
      let mv := if mv < carVolume then carVolume else mv
      let mv := mv ||| (v.modVolume &&& 0xc0)
      if mv != v.modVolume then
        let s := s.modVoice vi fun v => { v with modVolume := mv }
        s.write ((regsLevel + v.op1) ||| v.array) (mv ||| (ov.modulator.scale.toNat &&& 0xc0))
      else s
    else s
  else s

/-- `SetVoicePan`. -/
def State.setVoicePan (s : State) (vi : Nat) (pan : Nat) : State :=
  let s := s.modVoice vi fun v => { v with regPan := pan }
  let v := s.voice vi
  let ov := (s.instr v.currentInstr).voice v.currentInstrVoice
  s.write ((regsFeedback + v.index) ||| v.array) (ov.feedback.toNat ||| pan)

/-- `SetChannelVolume`. -/
def State.setChannelVolume (s : State) (ch : Nat) (volume : Nat) (clipStart : Bool) : State := Id.run do
  let mut s := s.modChannel ch fun c => { c with volumeBase := volume }
  let mut vol := volume
  if vol > s.currentMusicVolume then vol := s.currentMusicVolume
  if clipStart && vol > s.startMusicVolume then vol := s.startMusicVolume
  s := s.modChannel ch fun c => { c with volume := vol }
  for i in [0:s.numVoices] do
    if (s.voice i).channel == ch then
      s := s.setVoiceVolume i (s.voice i).noteVolume
  return s

/-- `I_OPL_SetMusicVolume` (0..127). -/
def State.setMusicVolume (s : State) (volume : Nat) : State := Id.run do
  if s.currentMusicVolume == volume then return s
  let mut s := { s with currentMusicVolume := volume }
  for i in [0:16] do
    if i == 15 then s := s.setChannelVolume i volume false
    else s := s.setChannelVolume i (s.chan i).volumeBase false
  return s

/-- `SetChannelPan`. -/
def State.setChannelPan (s : State) (ch : Nat) (pan0 : Nat) : State := Id.run do
  let pan := if s.cfg.stereoCorrect then 144 - pan0 else pan0
  if !s.cfg.opl3 then return s
  let regPan := if pan >= 96 then 0x10 else if pan <= 48 then 0x20 else 0x30
  if (s.chan ch).pan == regPan then return s
  let mut s := s.modChannel ch fun c => { c with pan := regPan }
  for i in [0:s.numVoices] do
    if (s.voice i).channel == ch then s := s.setVoicePan i regPan
  return s

/-- Release every allocated voice on `ch` matching `pred` (`KeyOffEvent` /
`AllNotesOff` loops, which re-examine the same index after a release). -/
partial def State.releaseMatching (s : State) (pred : Voice → Bool) (i : Nat := 0) : State :=
  if i >= s.allocList.size then s
  else if pred (s.voice s.allocList[i]!) then (s.releaseVoice i).releaseMatching pred i
  else s.releaseMatching pred (i + 1)

def State.allNotesOff (s : State) (ch : Nat) : State := s.releaseMatching (·.channel == ch)

/-- `TrackChannelForEvent`: MIDI 9 <-> MUS-style 15 swap. -/
def trackChannel (midiCh : Nat) : Nat := if midiCh == 9 then 15 else if midiCh == 15 then 9 else midiCh

/-- `ReplaceExistingVoice` (Doom 1.9). -/
def State.replaceExistingVoice (s : State) : State := Id.run do
  let mut result := 0
  for i in [0:s.allocList.size] do
    let vi := s.voice s.allocList[i]!
    let vr := s.voice s.allocList[result]!
    if vi.currentInstrVoice != 0 || vi.channel >= vr.channel then result := i
  return s.releaseVoice result

/-- `ReplaceExistingVoiceDoom1`. -/
def State.replaceExistingVoiceDoom1 (s : State) : State := Id.run do
  let mut result := 0
  for i in [0:s.allocList.size] do
    if (s.voice s.allocList[i]!).channel > (s.voice s.allocList[result]!).channel then result := i
  return s.releaseVoice result

/-- `ReplaceExistingVoiceDoom2`. -/
def State.replaceExistingVoiceDoom2 (s : State) (ch : Nat) : State := Id.run do
  let mut result := 0
  let mut priority := 0x8000
  for i in [0:s.allocList.size - 3] do
    let v := s.voice s.allocList[i]!
    if v.priority < priority && v.channel >= ch then
      priority := v.priority
      result := i
  return s.releaseVoice result

/-- `FrequencyForVoice`. -/
def State.frequencyForVoice (s : State) (vi : Nat) : Nat := Id.run do
  let v := s.voice vi
  let instr := s.instr v.currentInstr
  let gv := instr.voice v.currentInstrVoice
  let mut note : Int := v.note
  if (instr.flags &&& Genmidi.flagFixed) == 0 then note := note + gv.baseNoteOffset
  while note < 0 do note := note + 12
  while note > 95 do note := note - 12
  let mut fi : Int := 64 + 32 * note + (s.chan v.channel).bend
  if v.currentInstrVoice != 0 then fi := fi + ((instr.fineTuning.toNat / 2 : Nat) : Int) - 64
  if fi < 0 then fi := 0
  let idx := fi.toNat
  if idx < 284 then return (frequencyCurve[idx]!).toNat
  let sub := (idx - 284) % (12 * 32)
  let octave := min 7 ((idx - 284) / (12 * 32))
  return (frequencyCurve[sub + 284]!).toNat ||| (octave <<< 10)

/-- `UpdateVoiceFrequency`. -/
def State.updateVoiceFrequency (s : State) (vi : Nat) : State :=
  let freq := s.frequencyForVoice vi
  let v := s.voice vi
  if v.freq != freq then
    let s := s.write ((regsFreq1 + v.index) ||| v.array) (freq &&& 0xff)
    let s := s.write ((regsFreq2 + v.index) ||| v.array) ((freq >>> 8) ||| 0x20)
    s.modVoice vi fun v => { v with freq := freq }
  else s

/-- `VoiceKeyOn`. -/
def State.voiceKeyOn (s : State) (ch instr instrVoice0 note key volume : Nat) : State :=
  let instrVoice := if !s.cfg.opl3 && s.cfg.driver == .doom1_1_666 then 0 else instrVoice0
  match s.getFreeVoice with
  | (s, none) => s
  | (s, some vi) =>
    let ins := s.instr instr
    let n := if (ins.flags &&& Genmidi.flagFixed) != 0 then ins.fixedNote.toNat else note
    let pan := (s.chan ch).pan
    let s := s.modVoice vi fun v => { v with channel := ch, key := key, note := n, regPan := pan }
    let s := s.setVoiceInstrument vi instr instrVoice
    let s := s.setVoiceVolume vi volume
    let s := s.modVoice vi fun v => { v with freq := 0 }
    s.updateVoiceFrequency vi

/-- `KeyOffEvent`. -/
def State.keyOffEvent (s : State) (midiCh key : Nat) : State :=
  let ch := trackChannel midiCh
  s.releaseMatching fun v => v.channel == ch && v.key == key

/-- `KeyOnEvent`. -/
def State.keyOnEvent (s : State) (midiCh p1 p2 : Nat) : State := Id.run do
  let key := p1
  let volume := p2
  if volume == 0 then return s.keyOffEvent midiCh key
  let ch := trackChannel midiCh
  let mut note := p1
  let mut instr := 0
  if midiCh == 9 then
    if key < 35 || key > 81 then return s
    instr := Genmidi.numInstrs + (key - 35)
    note := 60
  else
    instr := (s.chan ch).instrument
  let double := ((s.instr instr).flags &&& Genmidi.flag2Voice) != 0
  let mut s := s
  match s.cfg.driver with
  | .doom1_1_666 =>
    let voicenum : Nat := if !s.cfg.opl3 then 1 else (if double then 2 else 1)
    -- while (voice_alloced_num > num_opl_voices - voicenum)
    let mut guard := 0
    while (s.allocList.size : Int) > (s.numVoices : Int) - voicenum && guard < 64 do
      s := s.replaceExistingVoiceDoom1
      guard := guard + 1
    if double then s := s.voiceKeyOn ch instr 1 note key volume
    s := s.voiceKeyOn ch instr 0 note key volume
  | .doom2_1_666 =>
    if s.allocList.size == s.numVoices then s := s.replaceExistingVoiceDoom2 ch
    if s.allocList.size + 1 == s.numVoices && double then s := s.replaceExistingVoiceDoom2 ch
    if double then s := s.voiceKeyOn ch instr 1 note key volume
    s := s.voiceKeyOn ch instr 0 note key volume
  | .doom_1_9 =>
    if s.freeList.isEmpty then s := s.replaceExistingVoice
    s := s.voiceKeyOn ch instr 0 note key volume
    if double then s := s.voiceKeyOn ch instr 1 note key volume
  return s

/-- `ControllerEvent`. -/
def State.controllerEvent (s : State) (midiCh p1 p2 : Nat) : State :=
  let ch := trackChannel midiCh
  if p1 == 0x07 then s.setChannelVolume ch p2 true
  else if p1 == 0x0a then s.setChannelPan ch p2
  else if p1 == 0x7b then s.allNotesOff ch
  else s

/-- `PitchBendEvent`: updated voices move to the end of the allocated list. -/
def State.pitchBendEvent (s : State) (midiCh p2 : Nat) : State := Id.run do
  let ch := trackChannel midiCh
  let mut s := s.modChannel ch fun c => { c with bend := (p2 : Int) - 64 }
  let mut updated : Array Nat := #[]
  let mut notUpdated : Array Nat := #[]
  for vi in s.allocList do
    if (s.voice vi).channel == ch then
      s := s.updateVoiceFrequency vi
      updated := updated.push vi
    else notUpdated := notUpdated.push vi
  return { s with allocList := notUpdated ++ updated }

/-- `MetaSetTempo`. -/
def State.metaSetTempo (s : State) (tempo : Nat) : State :=
  let factor := s.usPerBeat.toUInt32.toFloat32 / tempo.toUInt32.toFloat32
  { s.adjustCallbacks factor with usPerBeat := tempo }

/-- `ProcessEvent`. -/
def State.processEvent (s : State) (e : Midi.Event) : State :=
  let ch := e.channel.toNat
  if e.kind == Midi.evNoteOff then s.keyOffEvent ch e.p1.toNat
  else if e.kind == Midi.evNoteOn then s.keyOnEvent ch e.p1.toNat e.p2.toNat
  else if e.kind == Midi.evController then s.controllerEvent ch e.p1.toNat e.p2.toNat
  else if e.kind == Midi.evProgramChange then
    s.modChannel (trackChannel ch) fun c => { c with instrument := e.p1.toNat % Genmidi.numInstrs }
  else if e.kind == Midi.evPitchBend then s.pitchBendEvent ch e.p2.toNat
  else if e.kind == Midi.evMeta && e.metaType == Midi.metaSetTempo && e.data.size == 3 then
    s.metaSetTempo (((e.data.get! 0).toNat <<< 16) ||| ((e.data.get! 1).toNat <<< 8) ||| (e.data.get! 2).toNat)
  else s

/-! ## Tracks and timing -/

/-- `InitChannel`. -/
def State.initChannel (s : State) (ch : Nat) : State :=
  let vol := min s.currentMusicVolume 100
  s.modChannel ch fun _ => { instrument := 0, volume := vol, volumeBase := 100, pan := 0x30, bend := 0 }

/-- `ScheduleTrack`. -/
def State.scheduleTrack (s : State) (t : Nat) : State :=
  let pos := s.trackPos[t]!
  let track := s.tracks[t]!
  let nticks : Nat := match track[pos]? with | some e => e.delta | none => 0
  let us : UInt64 := (nticks.toUInt64 * s.usPerBeat.toUInt64) / (max s.ticksPerBeat 1).toUInt64
  s.setCallback us t

/-- `RestartSong`. -/
def State.restartSong (s : State) : State := Id.run do
  let mut s := { s with runningTracks := s.numTracks, startMusicVolume := s.currentMusicVolume }
  for t in [0:s.numTracks] do
    s := { s with trackPos := s.trackPos.set! t 0 }
    s := s.scheduleTrack t
  for ch in [0:16] do
    s := s.initChannel ch
  return s

/-- `TrackTimerCallback`. -/
def State.trackTimerCallback (s : State) (t : Nat) : State :=
  let pos := s.trackPos[t]!
  match (s.tracks[t]!)[pos]? with
  | none => s
  | some e =>
    let s := { s with trackPos := s.trackPos.set! t (pos + 1) }
    let s := s.processEvent e
    if e.isEndOfTrack then
      let s := { s with runningTracks := s.runningTracks - 1 }
      if s.runningTracks == 0 && s.looping then s.setCallback 5000 cbRestart else s
    else s.scheduleTrack t

def State.invoke (s : State) (cb : Nat) : State :=
  if cb == cbRestart then s.restartSong else s.trackTimerCallback cb

/-- `AdvanceTime`: move the clock by `n` output samples and run due callbacks. -/
partial def State.advanceTime (s : State) (n : Nat) : State :=
  let us := (n.toUInt64 * opsSecond) / s.rate.toUInt64
  let s := { s with currentTime := s.currentTime + us,
                    pauseOffset := if s.paused then s.pauseOffset + us else s.pauseOffset }
  let rec run (s : State) : State :=
    if s.queue.isEmpty then s
    else if s.currentTime >= (s.queue[0]!).time + s.pauseOffset then
      let (q, cb) := queuePop s.queue
      run ({ s with queue := q }.invoke cb)
    else s
  run s

/-- Generate `n` frames of chip output onto `out`. -/
def State.fill (s : State) (out : ByteArray) (n : Nat) : State × ByteArray :=
  let chip := s.chip
  let s := { s with chip := emptyChip }
  let (chip, out) := chip.renderStream out n
  ({ s with chip := chip, framesOut := s.framesOut + n }, out)

/--
`OPL_Mix_Callback` for one output buffer of `frames` frames: appends
`frames * 4` bytes (s16le stereo) to `out`.
-/
partial def State.mixBuffer (s : State) (out : ByteArray) (frames : Nat) (filled : Nat := 0) : State × ByteArray :=
  if filled >= frames then (s, out)
  else
    let remaining := frames - filled
    let n : Nat :=
      if s.paused || s.queue.isEmpty then remaining
      else
        let next := (s.queue[0]!).time + s.pauseOffset
        let ns := (next - s.currentTime) * s.rate.toUInt64
        let ns := (ns + opsSecond - 1) / opsSecond
        if ns > remaining.toUInt64 then remaining else ns.toNat
    let (s, out) := s.fill out n
    let s := s.advanceTime n
    s.mixBuffer out frames (filled + n)

/-! ## Public driver API -/

/-- `OPL_InitRegisters`. -/
def State.initRegisters (s : State) (opl3 : Bool) : State := Id.run do
  let mut s := s
  for r in [regsLevel:regsLevel + 22] do s := s.write r 0x3f
  for r in [regsAttack:regsWaveform + 22] do s := s.write r 0
  for r in [1:regsLevel] do s := s.write r 0
  s := s.write 0x04 0x60
  s := s.write 0x04 0x80
  s := s.write 0x01 0x20
  if opl3 then
    s := s.write 0x105 1
    for r in [regsLevel:regsLevel + 22] do s := s.write (r ||| 0x100) 0x3f
    for r in [regsAttack:regsWaveform + 22] do s := s.write (r ||| 0x100) 0
    for r in [1:regsLevel] do s := s.write (r ||| 0x100) 0
  s := s.write 0x08 0x40
  if opl3 then s := s.write 0x105 1
  return s

/-- `OPL3_Reset` + `I_OPL_InitMusic` (+ the initial `I_OPL_SetMusicVolume`). -/
def State.init (gm : Genmidi.Genmidi) (rate : Nat) (cfg : Config := {}) (musicVolume : Nat := 64)
    (logWrites : Bool := false) : State :=
  let numVoices := if cfg.opl3 then 2 * numVoicesOpl2 else numVoicesOpl2
  let (voices, free) := initVoices numVoices
  let s : State := { chip := Opl.Chip.reset rate, gm, cfg, numVoices, voices, freeList := free,
                     allocList := #[], channels := Array.replicate 16 {}, rate, logWrites }
  let s := s.initRegisters cfg.opl3
  s.setMusicVolume musicVolume

/-- `I_OPL_StopSong`. -/
def State.stopSong (s : State) : State := Id.run do
  let mut s := { s with queue := #[] }
  for ch in [0:16] do s := s.allNotesOff ch
  return { s with tracks := #[], trackPos := #[], numTracks := 0 }

/-- `I_OPL_PlaySong`. -/
def State.playSong (s : State) (file : Midi.File) (looping : Bool) : State := Id.run do
  let n := file.tracks.size
  let mut s := { s with tracks := file.tracks, trackPos := Array.replicate n 0, numTracks := n,
                        runningTracks := n, looping := looping, ticksPerBeat := file.ticksPerBeat,
                        usPerBeat := 500000, startMusicVolume := s.currentMusicVolume }
  for t in [0:n] do s := s.scheduleTrack t
  for ch in [0:16] do s := s.initChannel ch
  return { s with paused := false }

/-- `I_OPL_PauseSong`: stop the clock and key off melodic voices. -/
def State.pauseSong (s : State) : State := Id.run do
  let mut s := { s with paused := true }
  for i in [0:s.numVoices] do
    let v := s.voice i
    if v.channel != noChan && v.currentInstr < Genmidi.numInstrs then s := s.voiceKeyOff i
  return s

/-- `I_OPL_ResumeSong`. -/
def State.resumeSong (s : State) : State := { s with paused := false }

/-- `I_OPL_MusicIsPlaying`. -/
def State.isPlaying (s : State) : Bool := s.numTracks > 0

/-- Change the output rate (chip resampler and callback clock). -/
def State.setRate (s : State) (rate : Nat) : State :=
  if rate == s.rate || rate == 0 then s
  else { s.withChip (·.setRate rate) with rate := rate }

end Doom.Audio.OplMusic
