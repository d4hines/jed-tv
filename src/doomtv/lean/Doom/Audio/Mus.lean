import Doom.Audio.Midi

/-!
# Doom.Audio.Mus

DMX MUS lumps: a parser, a stand-alone 140 Hz sequencer, and the conversion
to MIDI events that Chocolate Doom performs before playback (`mus2mid.c`,
then reading the result back with `midifile.c`).

MUS layout: `id[4]` (not checked, as in Chocolate), `scorelength:u16le`,
`scorestart:u16le`, `primarychannels`, `secondarychannels`,
`instrumentcount` (u16le each), instrument list, then the score at
`scorestart`. Each event starts with a descriptor byte: bit 7 = "last event
of this group", bits 4-6 = type, bits 0-3 = channel (15 = percussion).
Types: 0 release key (note), 1 press key (note, bit 7 => a volume byte
follows), 2 pitch wheel (value), 3 system event (controller 10..14),
4 change controller (controller, value; controller 0 = instrument),
6 score end; 5 and 7 are invalid. After a "last" event a delay follows,
7 bits per byte, high bit = continue, in ticks of 1/140 s.

`toMidi` reproduces `mus2mid` exactly (one track, 70 ticks per beat at the
default 500000 us per beat = 140 Hz): MUS channels are mapped to MIDI
channels in order of first use (skipping 9; MUS 15 -> MIDI 9), each newly
used channel first gets an "all notes off" controller event, controller
numbers go through `controller_map`, controller values >= 0x80 become 0x7F,
pitch wheel values are `value * 64`, and the per-channel note velocity is
remembered across events. Chocolate keeps that velocity table in a static
array that is never reset between songs; `toMidi` takes it as input and
returns the updated table so a player can reproduce that too.
-/

namespace Doom.Audio.Mus


def musRelease : UInt8 := 0
def musPress : UInt8 := 1
def musPitch : UInt8 := 2
def musSystem : UInt8 := 3
def musController : UInt8 := 4
def musScoreEnd : UInt8 := 6

/-- A decoded MUS event. `delay` is the number of ticks after this event before
the next one (nonzero only on the last event of a group). -/
structure Event where
  channel : UInt8
  kind : UInt8
  /-- Note / pitch value / controller number. -/
  a : UInt8 := 0
  /-- Volume (press, if `hasVolume`) / controller value. -/
  b : UInt8 := 0
  hasVolume : Bool := false
  delay : Nat := 0
deriving Inhabited, Repr

structure Header where
  scoreLength : Nat
  scoreStart : Nat
  primaryChannels : Nat
  secondaryChannels : Nat
  instrumentCount : Nat
  instruments : Array Nat
deriving Inhabited, Repr

structure Song where
  header : Header
  events : Array Event
deriving Inhabited

private def u16 (b : ByteArray) (i : Nat) : Nat := (b.get! i).toNat ||| ((b.get! (i + 1)).toNat <<< 8)

/--
Decode a MUS lump. Mirrors `mus2mid`'s reading: a missing pitch-wheel value
is tolerated (the event is dropped); any other truncation, an invalid system
controller, an invalid controller number or event type 5/7 is an error.
-/
def parse (data : ByteArray) : Except String Song := do
  if data.size < 14 then throw "MUS lump too small"
  let header : Header :=
    { scoreLength := u16 data 4, scoreStart := u16 data 6, primaryChannels := u16 data 8,
      secondaryChannels := u16 data 10, instrumentCount := u16 data 12, instruments := #[] }
  let header := { header with instruments := Id.run do
      let mut a := #[]
      for i in [0:header.instrumentCount] do
        if 16 + 2 * i + 1 < data.size then a := a.push (u16 data (16 + 2 * i))
      return a }
  let mut pos := header.scoreStart
  if pos > data.size then throw "MUS score start beyond lump"
  let mut events : Array Event := #[]
  let mut ended := false
  let rd (p : Nat) : Except String UInt8 :=
    if h : p < data.size then pure data[p] else throw "unexpected end of MUS data"
  -- the score cannot have more events than bytes
  for _ in [0:data.size + 1] do
    if ended then break
    -- a group of events
    let mut last := false
    while !last && !ended do
      let d ← rd pos
      pos := pos + 1
      let ch := d &&& 0x0f
      let kind := (d >>> 4) &&& 7
      last := d &&& 0x80 != 0
      match kind with
      | 0 =>
        let k ← rd pos; pos := pos + 1
        events := events.push { channel := ch, kind := musRelease, a := k }
      | 1 =>
        let k ← rd pos; pos := pos + 1
        if k &&& 0x80 != 0 then
          let v ← rd pos; pos := pos + 1
          events := events.push { channel := ch, kind := musPress, a := k, b := v, hasVolume := true }
        else
          events := events.push { channel := ch, kind := musPress, a := k }
      | 2 =>
        if h : pos < data.size then
          events := events.push { channel := ch, kind := musPitch, a := data[pos] }
          pos := pos + 1
        else
          -- mus2mid: `break` without writing anything
          events := events.push { channel := ch, kind := 0xff }
      | 3 =>
        let c ← rd pos; pos := pos + 1
        if c < 10 || c > 14 then throw s!"invalid MUS system event {c}"
        events := events.push { channel := ch, kind := musSystem, a := c }
      | 4 =>
        let c ← rd pos; pos := pos + 1
        let v ← rd pos; pos := pos + 1
        if c > 9 then throw s!"invalid MUS controller {c}"
        events := events.push { channel := ch, kind := musController, a := c, b := v }
      | 6 =>
        events := events.push { channel := ch, kind := musScoreEnd }
        ended := true
      | _ => throw s!"invalid MUS event type {kind}"
    if !ended then
      let mut delay := 0
      let mut more := true
      while more do
        let w ← rd pos
        pos := pos + 1
        delay := delay * 128 + (w &&& 0x7f).toNat
        more := w &&& 0x80 != 0
      if events.size > 0 then
        events := events.modify (events.size - 1) fun e => { e with delay := e.delay + delay }
  if !ended then throw "MUS score has no end"
  return { header, events }

/-- `controller_map` of `mus2mid.c` (MUS controller 0..14 -> MIDI controller). -/
def controllerMap : Array UInt8 :=
  #[0x00, 0x20, 0x01, 0x07, 0x0A, 0x0B, 0x5B, 0x5D, 0x40, 0x43, 0x78, 0x7B, 0x7E, 0x7F, 0x79]

/-- Chocolate's initial velocity table (all 127). -/
def initialVelocities : Array UInt8 := Array.replicate 16 127

/--
`mus2mid` + `MIDI_LoadFile`: the events of the single MIDI track Chocolate
plays for this song, and the updated (static) velocity table.
Fails exactly where `mus2mid` would (including on events after a delay that
does not fit the 4-byte variable-length limit of `midifile.c`).
-/
def toMidi (song : Song) (velocities : Array UInt8 := initialVelocities) :
    Except String (Midi.File × Array UInt8) := do
  let mut chanMap : Array Int := Array.replicate 16 (-1)
  let mut vel := velocities
  let mut queued : Nat := 0
  let mut out : Array Midi.Event := #[]
  let mut pendingDelay : Nat := 0
  for e in song.events do
    queued := queued + pendingDelay
    pendingDelay := 0
    -- GetMIDIChannel runs for every descriptor, before the event switch
    let musCh := e.channel.toNat
    let mut ch : Nat := 9
    if musCh != 15 then
      if chanMap[musCh]! == -1 then
        let mx := chanMap.foldl (fun m x => if x > m then x else m) (-1)
        let r := mx + 1
        let r := if r == 9 then 10 else r
        chanMap := chanMap.set! musCh r
        out := out.push { delta := queued, kind := Midi.evController, channel := r.toNat.toUInt8, p1 := 0x7b, p2 := 0 }
        queued := 0
      ch := chanMap[musCh]!.toNat
    let c8 := ch.toUInt8
    if e.kind == musRelease then
      out := out.push { delta := queued, kind := Midi.evNoteOff, channel := c8, p1 := e.a &&& 0x7f, p2 := 0 }
      queued := 0
    else if e.kind == musPress then
      if e.hasVolume then vel := vel.set! ch (e.b &&& 0x7f)
      out := out.push { delta := queued, kind := Midi.evNoteOn, channel := c8, p1 := e.a &&& 0x7f, p2 := vel[ch]! }
      queued := 0
    else if e.kind == musPitch then
      let wheel := e.a.toNat * 64
      out := out.push { delta := queued, kind := Midi.evPitchBend, channel := c8,
                        p1 := (wheel &&& 0x7f).toUInt8, p2 := ((wheel >>> 7) &&& 0x7f).toUInt8 }
      queued := 0
    else if e.kind == musSystem then
      out := out.push { delta := queued, kind := Midi.evController, channel := c8,
                        p1 := controllerMap[e.a.toNat]! &&& 0x7f, p2 := 0 }
      queued := 0
    else if e.kind == musController then
      if e.a == 0 then
        out := out.push { delta := queued, kind := Midi.evProgramChange, channel := c8, p1 := e.b &&& 0x7f }
      else
        let v := if e.b &&& 0x80 != 0 then 0x7f else e.b
        out := out.push { delta := queued, kind := Midi.evController, channel := c8,
                          p1 := controllerMap[e.a.toNat]! &&& 0x7f, p2 := v }
      queued := 0
    else
      pure ()  -- score end, or a dropped truncated pitch wheel
    pendingDelay := e.delay
  out := out.push { delta := queued, kind := Midi.evMeta, metaType := Midi.metaEndOfTrack }
  if out.any (·.delta >= (1 <<< 28)) then throw "MUS delay too long for midifile.c"
  return ({ format := 0, division := 0x46, tracks := #[out] }, vel)

/-! ## Stand-alone sequencer (140 Hz ticks) -/

/-- Plays a decoded MUS song one tick at a time, looping at the score end. -/
structure Sequencer where
  song : Song
  /-- Index of the next event. -/
  pos : Nat := 0
  /-- Ticks to wait before `pos` is due. -/
  wait : Nat := 0
  looping : Bool := true
  finished : Bool := false
deriving Inhabited

def Sequencer.start (song : Song) (looping : Bool := true) : Sequencer := { song, looping }

/--
Advance one tick (1/140 s): returns the events due at this tick, in score
order. On the score end the sequencer restarts at the first event (if
looping; the restart events are delivered on the next tick) or finishes.
-/
def Sequencer.tick (s : Sequencer) : Sequencer × Array Event := Id.run do
  if s.finished || s.song.events.isEmpty then return (s, #[])
  if s.wait > 0 then return ({ s with wait := s.wait - 1 }, #[])
  let mut s := s
  let mut due : Array Event := #[]
  let mut go := true
  while go do
    match s.song.events[s.pos]? with
    | none =>
      s := { s with finished := true }
      go := false
    | some e =>
      due := due.push e
      if e.kind == musScoreEnd then
        if s.looping then s := { s with pos := 0, wait := 0 }
        else s := { s with finished := true }
        go := false
      else
        s := { s with pos := s.pos + 1 }
        if e.delay > 0 then
          s := { s with wait := e.delay - 1 }
          go := false
  return (s, due)

end Doom.Audio.Mus
