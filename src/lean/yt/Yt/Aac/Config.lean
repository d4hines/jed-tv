import Yt.Aac.Bits
import Yt.Aac.Tables

/-! AudioSpecificConfig (ISO/IEC 14496-3 1.6.2.1) for AAC-LC. -/

namespace Yt.Aac

inductive DecodeError where
  /-- The bitstream violates the syntax or a limit. -/
  | invalid (what : String)
  /-- Valid but outside this decoder: HE-AAC (SBR/PS), other object types,
  960-sample frames, coupling channels, more than two channels. -/
  | unsupported (what : String)
  deriving Inhabited, BEq

def DecodeError.message : DecodeError → String
  | .invalid w => "invalid AAC data: " ++ w
  | .unsupported w => "unsupported AAC feature: " ++ w

structure Config where
  /-- Audio object type (2 = AAC-LC). -/
  objectType : Nat
  /-- Sampling-frequency index (0..12) selecting the band tables. -/
  rateIndex : Nat
  sampleRate : Nat
  channelConfig : Nat
  channels : Nat
  deriving Inhabited

private def readAot (r : BitReader) : Nat × BitReader :=
  let (a, r) := r.readNat 5
  if a == 31 then
    let (b, r) := r.readNat 6
    (32 + b, r)
  else (a, r)

private def readRate (r : BitReader) : Nat × Nat × BitReader :=
  let (i, r) := r.readNat 4
  if i == 15 then
    let (f, r) := r.readNat 24
    (Tables.rateIndex f, f, r)
  else (i, Tables.sampleRates.getD i 0, r)

/-- Parse an AudioSpecificConfig. HE-AAC (explicit SBR/PS signalling, or a
backward-compatible SBR sync extension) is reported as unsupported. -/
def parseAsc (asc : ByteArray) : Except DecodeError Config := do
  let r : BitReader := { data := asc, pos := 0 }
  let (aot, r) := readAot r
  let (ri, rate, r) := readRate r
  let (cc, r) := r.readNat 4
  if aot == 5 || aot == 29 then
    throw (.unsupported "HE-AAC (SBR/PS signalled in AudioSpecificConfig)")
  if aot != 2 then
    throw (.unsupported ("audio object type " ++ toString aot ++ " (only AAC-LC, type 2)"))
  if ri > 12 || rate == 0 then throw (.invalid "sampling frequency index")
  -- GASpecificConfig
  let (frameLen960, r) := r.readBool
  if frameLen960 then throw (.unsupported "960-sample frames")
  let (dependsOnCore, r) := r.readBool
  let r := if dependsOnCore then r.skip 14 else r
  let (_extension, r) := r.readBool
  if cc == 0 then throw (.unsupported "program_config_element channel layout")
  if cc > 2 then throw (.unsupported ("channel configuration " ++ toString cc ++ " (mono/stereo only)"))
  if r.overrun then throw (.invalid "AudioSpecificConfig truncated")
  -- Backward-compatible SBR signalling: syncExtensionType 0x2b7.
  if r.bitsLeft ≥ 16 then
    let (sync, r) := r.read 11
    if sync == 0x2b7 then
      let (ext, r) := readAot r
      if ext == 5 then
        let (sbr, _) := r.readBool
        if sbr then throw (.unsupported "HE-AAC (SBR sync extension in AudioSpecificConfig)")
  return { objectType := aot, rateIndex := ri, sampleRate := rate, channelConfig := cc, channels := cc }

end Yt.Aac
