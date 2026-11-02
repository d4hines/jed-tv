import Yt.Aac.Decoder
import AacFixture
import AacTest.Adts

/-! The AAC transcript: decode the embedded fixture streams and print frame
counts, tool statistics and PCM hashes (plus table checksums), so the host
(upstream Lean) and our runtime on 32-bit ARM can be compared byte for
byte. -/

namespace AacTest
open Yt.Aac

def line (o : ByteArray) (s : String) : ByteArray := o ++ (s ++ "\n").toUTF8

def statsLine (s : Stats) : String :=
  "frames=" ++ toString s.frames ++ " short=" ++ toString s.short ++ " kbd=" ++ toString s.kbd ++
  " tns=" ++ toString s.tns ++ " pulse=" ++ toString s.pulse ++ " ms=" ++ toString s.ms ++
  " is=" ++ toString s.intensity ++ " pns=" ++ toString s.noise

/-- Decode one ADTS stream; one line per stream plus a hash every 20 frames. -/
def runStream (o : ByteArray) (name : String) (adts : ByteArray) : ByteArray := Id.run do
  let (asc, aus) := adtsSplit adts
  let mut o := o
  match Decoder.init asc with
  | .error e => return line o (name ++ ": init error: " ++ e.message)
  | .ok d0 =>
    let mut d := d0
    let mut h := fnvInit
    let mut errs := 0
    let mut bytes := 0
    for i in [0:aus.size] do
      let (d1, r) := d.decode aus[i]!
      d := d1
      match r with
      | .ok pcm =>
        h := fnv h pcm
        bytes := bytes + pcm.size
      | .error e =>
        errs := errs + 1
        if errs ≤ 3 then o := line o ("  " ++ name ++ " frame " ++ toString i ++ ": " ++ e.message)
      if (i + 1) % 20 == 0 then
        o := line o ("  " ++ name ++ " @" ++ toString (i + 1) ++ " " ++ hex32 h)
    o := line o (name ++ ": rate=" ++ toString d.config.sampleRate ++ " ch=" ++ toString d.config.channels ++
      " aus=" ++ toString aus.size ++ " errors=" ++ toString errs ++ " pcm_bytes=" ++ toString bytes ++
      " fnv=" ++ hex32 h ++ " " ++ statsLine d.stats)
    return o

/-- Checksums of the generated tables (they must not depend on the platform). -/
def tableLines (o : ByteArray) : ByteArray := Id.run do
  let t := sharedTables ()
  let mut o := o
  o := line o ("pow43 " ++ hex32 (fnv (fnv fnvInit t.consts.pow43M) t.consts.pow43E) ++
    " frac " ++ hex32 (fnv fnvInit t.consts.frac))
  o := line o ("windows " ++ hex32 (fnv fnvInit t.win.sineLong) ++ " " ++ hex32 (fnv fnvInit t.win.sineShort) ++
    " " ++ hex32 (fnv fnvInit t.win.kbdLong) ++ " " ++ hex32 (fnv fnvInit t.win.kbdShort))
  o := line o ("mdct " ++ hex32 (fnv (fnv fnvInit t.long.tw) t.long.fft) ++ " " ++
    hex32 (fnv (fnv fnvInit t.short.tw) t.short.fft) ++ " tns " ++ hex32 (fnv (fnv fnvInit t.tns3) t.tns4))
  let mut hb := fnvInit
  for b in t.books.spec do hb := fnv hb b.data
  o := line o ("huffman " ++ hex32 (fnv hb t.books.scf.data))
  -- A few exact values: sin window ends, KBD middle, x^(4/3) samples.
  o := line o ("sine[0]=" ++ toString (ld32 t.win.sineLong 0) ++ " sine[1023]=" ++ toString (ld32 t.win.sineLong 1023) ++
    " kbd[512]=" ++ toString (ld32 t.win.kbdLong 512) ++ " pow43M[3]=" ++ toString (ld32 t.consts.pow43M 3) ++
    " pow43M[8191]=" ++ toString (ld32 t.consts.pow43M 8191) ++ " E[8191]=" ++ toString (ld8 t.consts.pow43E 8191))
  return o

/-- Error handling: bad configs and truncated / garbage access units. -/
def errorLines (o : ByteArray) : ByteArray := Id.run do
  let mut o := o
  let cases : Array (String × ByteArray) := #[
    ("he-aac", ⟨#[0x2B, 0x92, 0x08, 0x00]⟩), ("main", ⟨#[0x0A, 0x10]⟩), ("5.1", ⟨#[0x11, 0xB0]⟩),
    ("960", ⟨#[0x12, 0x14]⟩), ("sync-sbr", ⟨#[0x13, 0x10, 0x56, 0xE5, 0x98]⟩),
    -- Apple's encoder, HE-AAC in MP4: LC core config + SBR sync extension
    ("he-aac-mp4", ⟨#[0x13, 0x90, 0x56, 0xE5, 0xA0]⟩), ("lc", ⟨#[0x12, 0x10]⟩)]
  for (n, asc) in cases do
    match Decoder.init asc with
    | .ok d => o := line o ("asc " ++ n ++ ": ok rate=" ++ toString d.config.sampleRate ++ " ch=" ++ toString d.config.channels)
    | .error e => o := line o ("asc " ++ n ++ ": " ++ e.message)
  match Decoder.init ⟨#[0x12, 0x10]⟩ with
  | .error _ => pure ()
  | .ok d =>
    let aus : Array (String × ByteArray) := #[("empty", .empty), ("zeros", ⟨#[0, 0, 0, 0]⟩),
      ("ones", ⟨#[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]⟩), ("end-only", ⟨#[0xE0]⟩),
      ("cpe-trunc", ⟨#[0x21, 0x00, 0x80]⟩)]
    for (n, au) in aus do
      match (d.decode au).2 with
      | .ok pcm => o := line o ("au " ++ n ++ ": ok " ++ toString pcm.size)
      | .error e => o := line o ("au " ++ n ++ ": " ++ e.message)
  return o

/-- Robustness: decode corrupted copies of a stream's frames (one flipped bit
per frame, then every frame truncated to 2/3 of its length) on a decoder that
keeps running; count outcomes and hash whatever PCM comes out. -/
def fuzzLines (o : ByteArray) (adts : ByteArray) : ByteArray := Id.run do
  let (asc, aus) := adtsSplit adts
  let .ok d0 := Decoder.init asc | return line o "fuzz: init failed"
  let mut o := o
  let mut d := d0
  let mut rnd : UInt32 := 12345
  let mut ok := 0
  let mut bad := 0
  let mut h := fnvInit
  for au in aus do
    rnd := rnd * 1103515245 + 12345
    let bit := (rnd >>> 8).toNat % (8 * au.size)
    let b := ld8 au (bit / 8).toUSize
    let au := st8 au (bit / 8).toUSize (b ^^^ ((1 : UInt8) <<< (7 - bit % 8).toUInt8))
    let (d1, r) := d.decode au
    d := d1
    match r with
    | .ok pcm => ok := ok + 1; h := fnv h pcm
    | .error _ => bad := bad + 1
  o := line o ("fuzz bitflip: ok=" ++ toString ok ++ " errors=" ++ toString bad ++ " fnv=" ++ hex32 h)
  ok := 0
  bad := 0
  for au in aus do
    let (d1, r) := d.decode (au.extract 0 (2 * au.size / 3))
    d := d1
    match r with
    | .ok pcm => ok := ok + 1; h := fnv h pcm
    | .error _ => bad := bad + 1
  o := line o ("fuzz truncate: ok=" ++ toString ok ++ " errors=" ++ toString bad ++ " fnv=" ++ hex32 h)
  return o

@[export zr_test_transcript] def transcript (k : Nat) : ByteArray := Id.run do
  let mut o := line ByteArray.empty ("aac transcript " ++ toString k)
  o := tableLines o
  o := errorLines o
  for (name, hex) in AacFixture.streams do
    o := runStream o name (unhex hex)
    if name == "yt140" || name == "s44" then o := fuzzLines o (unhex hex)
  return o

end AacTest
