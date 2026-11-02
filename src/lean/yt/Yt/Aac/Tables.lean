/-! Static AAC-LC tables (ISO/IEC 14496-3): sampling rates, scalefactor-band
offsets for 1024- and 128-line windows, and TNS band limits. Array literals of
small naturals compile to static data. -/

namespace Yt.Aac.Tables

def sampleRates : Array Nat :=
  #[96000, 88200, 64000, 48000, 44100, 32000, 24000, 22050, 16000, 12000, 11025, 8000, 7350]

def swb96Long : Array Nat := #[
  0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 64, 72, 80, 88, 96, 108, 120, 132,
  144, 156, 172, 188, 212, 240, 276, 320, 384, 448, 512, 576, 640, 704, 768, 832, 896, 960, 1024]

def swb64Long : Array Nat := #[
  0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 64, 72, 80, 88, 100, 112, 124, 140,
  156, 172, 192, 216, 240, 268, 304, 344, 384, 424, 464, 504, 544, 584, 624, 664, 704, 744, 784,
  824, 864, 904, 944, 984, 1024]

def swb48Long : Array Nat := #[
  0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 48, 56, 64, 72, 80, 88, 96, 108, 120, 132, 144, 160,
  176, 196, 216, 240, 264, 292, 320, 352, 384, 416, 448, 480, 512, 544, 576, 608, 640, 672, 704,
  736, 768, 800, 832, 864, 896, 928, 1024]

def swb32Long : Array Nat := #[
  0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 48, 56, 64, 72, 80, 88, 96, 108, 120, 132, 144, 160,
  176, 196, 216, 240, 264, 292, 320, 352, 384, 416, 448, 480, 512, 544, 576, 608, 640, 672, 704,
  736, 768, 800, 832, 864, 896, 928, 960, 992, 1024]

def swb24Long : Array Nat := #[
  0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 52, 60, 68, 76, 84, 92, 100, 108, 116, 124, 136,
  148, 160, 172, 188, 204, 220, 240, 260, 284, 308, 336, 364, 396, 432, 468, 508, 552, 600, 652,
  704, 768, 832, 896, 960, 1024]

def swb16Long : Array Nat := #[
  0, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 100, 112, 124, 136, 148, 160, 172, 184, 196, 212,
  228, 244, 260, 280, 300, 320, 344, 368, 396, 424, 456, 492, 532, 572, 616, 664, 716, 772, 832,
  896, 960, 1024]

def swb8Long : Array Nat := #[
  0, 12, 24, 36, 48, 60, 72, 84, 96, 108, 120, 132, 144, 156, 172, 188, 204, 220, 236, 252, 268,
  288, 308, 328, 348, 372, 396, 420, 448, 476, 508, 544, 580, 620, 664, 712, 764, 820, 880, 944,
  1024]

def swb64Short : Array Nat := #[0, 4, 8, 12, 16, 20, 24, 32, 40, 48, 64, 92, 128]
def swb48Short : Array Nat := #[0, 4, 8, 12, 16, 20, 28, 36, 44, 56, 68, 80, 96, 112, 128]
def swb24Short : Array Nat := #[0, 4, 8, 12, 16, 20, 24, 28, 36, 44, 52, 64, 76, 92, 108, 128]
def swb16Short : Array Nat := #[0, 4, 8, 12, 16, 20, 24, 28, 32, 40, 48, 60, 72, 88, 108, 128]
def swb8Short : Array Nat := #[0, 4, 8, 12, 16, 20, 24, 28, 36, 44, 52, 60, 72, 88, 108, 128]

/-- Scalefactor-band offsets of 1024-line windows for sampling-frequency index `i`. -/
def swbLong (i : Nat) : Array Nat :=
  if i ≤ 1 then swb96Long else if i == 2 then swb64Long else if i ≤ 4 then swb48Long
  else if i == 5 then swb32Long else if i ≤ 7 then swb24Long else if i ≤ 10 then swb16Long
  else swb8Long

/-- Scalefactor-band offsets of 128-line windows for sampling-frequency index `i`. -/
def swbShort (i : Nat) : Array Nat :=
  if i ≤ 2 then swb64Short else if i ≤ 5 then swb48Short else if i ≤ 7 then swb24Short
  else if i ≤ 10 then swb16Short else swb8Short

/-- Highest band TNS may filter (AAC-LC), 1024-line windows. -/
def tnsMaxBandsLong : Array Nat := #[31, 31, 34, 40, 42, 51, 46, 46, 42, 42, 42, 39, 39]
/-- Highest band TNS may filter, 128-line windows. -/
def tnsMaxBandsShort : Array Nat := #[9, 9, 10, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14]

/-- Sampling-frequency index for an explicit rate (ISO/IEC 14496-3 Table 4.82). -/
def rateIndex (rate : Nat) : Nat :=
  if rate ≥ 92017 then 0 else if rate ≥ 75132 then 1 else if rate ≥ 55426 then 2
  else if rate ≥ 46009 then 3 else if rate ≥ 37566 then 4 else if rate ≥ 27713 then 5
  else if rate ≥ 23004 then 6 else if rate ≥ 18783 then 7 else if rate ≥ 13856 then 8
  else if rate ≥ 11502 then 9 else if rate ≥ 9391 then 10 else 11

end Yt.Aac.Tables
