/-!
# Doom.Audio.Genmidi

Parser for the DMX `GENMIDI` lump (OPL2 instrument bank), as read by
Chocolate Doom's `i_oplmusic.c` `LoadInstrumentTable`.

Layout: 8-byte header `#OPL_II#`, then 175 instruments of 36 bytes
(128 melodic + 47 percussion for keys 35..81), then 175 names of 32 bytes.
An instrument is `flags:u16le, fine_tuning:u8, fixed_note:u8` followed by two
16-byte voices: modulator operator (6 bytes: tremolo/AM-VIB-EG-KSR-MULT,
attack, sustain, waveform, scale (KSL bits), level), feedback, carrier
operator (6 bytes), unused, `base_note_offset:s16le`.

DMX does not check the header; neither does this parser (only the size).
-/

namespace Doom.Audio.Genmidi

def numInstrs : Nat := 128
def numPercussion : Nat := 47
def headerLen : Nat := 8
def instrLen : Nat := 36
def flagFixed : UInt32 := 0x0001
def flag2Voice : UInt32 := 0x0004

structure Op where
  tremolo : UInt32
  attack : UInt32
  sustain : UInt32
  waveform : UInt32
  scale : UInt32
  level : UInt32
deriving Inhabited, Repr, BEq

structure Voice where
  modulator : Op
  feedback : UInt32
  carrier : Op
  /-- Signed. -/
  baseNoteOffset : Int
deriving Inhabited, Repr, BEq

structure Instr where
  flags : UInt32
  fineTuning : UInt32
  fixedNote : UInt32
  voice0 : Voice
  voice1 : Voice
deriving Inhabited, Repr, BEq

/-- Parsed bank. Instruments `0..127` are melodic, `128..174` percussion. -/
structure Genmidi where
  instrs : Array Instr
  names : Array String
deriving Inhabited

def Instr.voice (i : Instr) (n : Nat) : Voice := if n == 0 then i.voice0 else i.voice1

private def u8 (b : ByteArray) (i : Nat) : UInt32 := (b.get! i).toUInt32

private def parseOp (b : ByteArray) (o : Nat) : Op :=
  { tremolo := u8 b o, attack := u8 b (o + 1), sustain := u8 b (o + 2),
    waveform := u8 b (o + 3), scale := u8 b (o + 4), level := u8 b (o + 5) }

private def parseVoice (b : ByteArray) (o : Nat) : Voice :=
  let raw := (u8 b (o + 14) ||| (u8 b (o + 15) <<< 8)).toNat
  { modulator := parseOp b o, feedback := u8 b (o + 6), carrier := parseOp b (o + 7),
    baseNoteOffset := if raw >= 0x8000 then (raw : Int) - 0x10000 else raw }

/-- Total size of a well-formed lump (11908 bytes). -/
def lumpSize : Nat := headerLen + (numInstrs + numPercussion) * (instrLen + 32)

/-- Parse a `GENMIDI` lump. -/
def parse (b : ByteArray) : Except String Genmidi := do
  if b.size < headerLen + (numInstrs + numPercussion) * instrLen then
    throw s!"GENMIDI too small: {b.size} bytes"
  let mut instrs : Array Instr := Array.mkEmpty (numInstrs + numPercussion)
  for i in [0:numInstrs + numPercussion] do
    let o := headerLen + i * instrLen
    instrs := instrs.push
      { flags := u8 b o ||| (u8 b (o + 1) <<< 8), fineTuning := u8 b (o + 2), fixedNote := u8 b (o + 3),
        voice0 := parseVoice b (o + 4), voice1 := parseVoice b (o + 20) }
  let mut names : Array String := #[]
  let nameBase := headerLen + (numInstrs + numPercussion) * instrLen
  if b.size >= lumpSize then
    for i in [0:numInstrs + numPercussion] do
      let o := nameBase + i * 32
      let raw := (b.extract o (o + 32)).toList.takeWhile (· != 0)
      names := names.push (String.ofList (raw.map fun c => Char.ofNat c.toNat))
  return { instrs, names }

end Doom.Audio.Genmidi
