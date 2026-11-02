/-!
# Doom.Audio.Sfx

DMX digital sound effect lumps (`DS*`), parsed the way Chocolate Doom's
`i_sdlsound.c` `CacheSFX` does:

* bytes 0-1: format, must be 3 (`03 00`);
* bytes 2-3: sample rate (u16le);
* bytes 4-7: sample count including padding (u32le);
* then unsigned 8-bit mono samples.

A sound is rejected if the lump is shorter than 8 bytes, the format is not
3, the count exceeds the lump (`length > lumplen - 8`) or the count is at
most 48 (DMX ignores such short sounds). DMX skips 16 padding bytes at each
end, so playback uses samples `[16, length - 16)` of the data, i.e. lump
bytes `[24, 24 + length - 32)`.

The lump bytes are shared, not copied.
-/

namespace Doom.Audio.Sfx

structure Sfx where
  /-- The whole lump. -/
  data : ByteArray
  /-- Byte offset of the first played sample (24). -/
  offset : Nat
  /-- Number of played samples (header count minus 32). -/
  length : Nat
  /-- Sample rate in Hz. -/
  rate : Nat
deriving Inhabited

/-- `NORM_PITCH`. -/
def normPitch : Nat := 128

def parse (lump : ByteArray) : Except String Sfx := do
  if lump.size < 8 then throw "sfx lump shorter than 8 bytes"
  if lump.get! 0 != 3 || lump.get! 1 != 0 then throw "sfx lump is not DMX format 3"
  let rate := (lump.get! 2).toNat ||| ((lump.get! 3).toNat <<< 8)
  let length := (lump.get! 4).toNat ||| ((lump.get! 5).toNat <<< 8) |||
    ((lump.get! 6).toNat <<< 16) ||| ((lump.get! 7).toNat <<< 24)
  if length > lump.size - 8 then throw s!"sfx length {length} exceeds lump ({lump.size} bytes)"
  if length <= 48 then throw s!"sfx too short ({length} samples)"
  if rate == 0 then throw "sfx sample rate is 0"
  return { data := lump, offset := 8 + 16, length := length - 32, rate }

/-- Duration in milliseconds. -/
def Sfx.durationMs (s : Sfx) : Nat := s.length * 1000 / s.rate

end Doom.Audio.Sfx
