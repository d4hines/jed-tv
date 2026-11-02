/-!
# Doom.Audio.Midi

Standard MIDI file reader, a port of Chocolate Doom's `midifile.c` (as used
by `i_oplmusic.c`), working on an in-memory lump instead of a temp file.

Faithful quirks:
* the header chunk size must be 6, format 0 or 1, at least one track;
* each track is read event by event until the end-of-track meta event; the
  `MTrk` chunk length is ignored (the next track header is read from wherever
  the previous track's events ended);
* running status reuses the last status byte, including `0xF0`/`0xF7`/`0xFF`;
* variable-length quantities are limited to 4 bytes;
* any other status byte, or running out of data, is a load failure.
-/

namespace Doom.Audio.Midi

def evNoteOff : UInt8 := 0x80
def evNoteOn : UInt8 := 0x90
def evAftertouch : UInt8 := 0xA0
def evController : UInt8 := 0xB0
def evProgramChange : UInt8 := 0xC0
def evChanAftertouch : UInt8 := 0xD0
def evPitchBend : UInt8 := 0xE0
def evSysex : UInt8 := 0xF0
def evSysexSplit : UInt8 := 0xF7
def evMeta : UInt8 := 0xFF

def metaEndOfTrack : UInt8 := 0x2F
def metaSetTempo : UInt8 := 0x51

/--
One event. Channel events use `kind` (high nibble), `channel`, `p1`, `p2`;
meta events use `metaType` and `data`; sysex keeps only its kind.
-/
structure Event where
  delta : Nat
  kind : UInt8
  channel : UInt8 := 0
  p1 : UInt8 := 0
  p2 : UInt8 := 0
  metaType : UInt8 := 0
  data : ByteArray := .empty
deriving Inhabited

def Event.isEndOfTrack (e : Event) : Bool := e.kind == evMeta && e.metaType == metaEndOfTrack

structure File where
  format : Nat
  /-- Raw big-endian `time_division` field. -/
  division : Nat
  tracks : Array (Array Event)
deriving Inhabited

/-- `MIDI_GetFileTimeDivision`: ticks per beat (SMPTE divisions converted). -/
def File.ticksPerBeat (f : File) : Nat :=
  if f.division >= 0x8000 then
    let r : Int := (f.division : Int) - 0x10000
    -- C: (-(result / 256)) * (result & 0xFF), truncating division
    let hi : Int := -(r.tdiv 256)
    (hi * ((f.division % 256 : Nat) : Int)).toNat
  else f.division

private structure Reader where
  data : ByteArray
  pos : Nat

private abbrev R := StateT Reader (Except String)

private def byte : R UInt8 := do
  let r ← get
  if h : r.pos < r.data.size then
    set { r with pos := r.pos + 1 }
    return r.data[r.pos]
  else throw "unexpected end of MIDI data"

private def be32 : R Nat := do
  let a ← byte; let b ← byte; let c ← byte; let d ← byte
  return (a.toNat <<< 24) ||| (b.toNat <<< 16) ||| (c.toNat <<< 8) ||| d.toNat

private def be16 : R Nat := do
  let a ← byte; let b ← byte
  return (a.toNat <<< 8) ||| b.toNat

/-- `ReadVariableLength`: at most 4 bytes. -/
private def vlq : R Nat := do
  let mut v := 0
  for _ in [0:4] do
    let b ← byte
    v := (v <<< 7) ||| (b.toNat &&& 0x7f)
    if b &&& 0x80 == 0 then return v
  throw "variable-length value too long"

private def bytes (n : Nat) : R ByteArray := do
  let r ← get
  if r.pos + n <= r.data.size then
    set { r with pos := r.pos + n }
    return r.data.extract r.pos (r.pos + n)
  else throw "unexpected end of MIDI data"

private def readEvent (last : UInt8) : R (Event × UInt8) := do
  let delta ← vlq
  let b ← byte
  let (ty, last) ←
    if b &&& 0x80 == 0 then do
      -- running status: re-read this byte as a parameter
      modify fun r => { r with pos := r.pos - 1 }
      pure (last, last)
    else pure (b, b)
  let hi := ty &&& 0xf0
  if hi == evNoteOff || hi == evNoteOn || hi == evAftertouch || hi == evController || hi == evPitchBend then
    let p1 ← byte
    let p2 ← byte
    return ({ delta, kind := hi, channel := ty &&& 0x0f, p1, p2 }, last)
  else if hi == evProgramChange || hi == evChanAftertouch then
    let p1 ← byte
    return ({ delta, kind := hi, channel := ty &&& 0x0f, p1 }, last)
  else if ty == evSysex || ty == evSysexSplit then
    let len ← vlq
    let _ ← bytes len
    return ({ delta, kind := ty }, last)
  else if ty == evMeta then
    let mt ← byte
    let len ← vlq
    let d ← bytes len
    return ({ delta, kind := evMeta, metaType := mt, data := d }, last)
  else throw s!"unknown MIDI event type 0x{ty.toNat}"

private partial def readTrackEvents (last : UInt8) (acc : Array Event) : R (Array Event) := do
  let (e, last) ← readEvent last
  let acc := acc.push e
  if e.isEndOfTrack then return acc else readTrackEvents last acc

private def readTrack : R (Array Event) := do
  let id ← bytes 4
  if id.toList != "MTrk".toUTF8.toList then throw "expected MTrk chunk"
  let _len ← be32
  readTrackEvents 0 #[]

/-- `MIDI_LoadFile` on lump bytes. -/
def parse (data : ByteArray) : Except String File := do
  let go : R File := do
    let id ← bytes 4
    if id.toList != "MThd".toUTF8.toList then throw "expected MThd chunk"
    let size ← be32
    if size != 6 then throw s!"invalid MIDI header size {size}"
    let format ← be16
    let ntracks ← be16
    let division ← be16
    if (format != 0 && format != 1) || ntracks < 1 then throw "only type 0/1 MIDI files supported"
    let mut tracks := #[]
    for _ in [0:ntracks] do
      tracks := tracks.push (← readTrack)
    return { format, division, tracks }
  (go.run { data, pos := 0 }).map (·.1)

/-- `IsMid` + size limit from `I_OPL_RegisterSong` (`MAXMIDLENGTH` = 96 KiB). -/
def isMidLump (data : ByteArray) : Bool :=
  data.size > 4 && data.size < 96 * 1024 && data.extract 0 4 == "MThd".toUTF8

end Doom.Audio.Midi
