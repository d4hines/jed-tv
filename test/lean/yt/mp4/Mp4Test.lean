import Yt.Mp4.Demux
import Mp4Test.Fixtures

/-! The MP4 port against the Rust demuxer: the same text dump as
src/mp4/examples/dump.rs (init, index, every sample with an FNV-1a hash of
its bytes, and the results on truncated and bit-flipped input), compared
line by line with the Rust output, plus the Rust unit tests. -/
namespace Mp4Test
open Yt.Mp4 Yt.Fmt

/-- An output buffer of text lines. -/
abbrev Out := ByteArray
@[inline] def put (o : Out) (s : String) : Out := o ++ s.toUTF8
@[inline] def u (x : UInt64) : String := dec x
@[inline] def n (x : Nat) : String := toString x

def hexList (v : Array ByteArray) : String := Id.run do
  let mut s := ""
  for i in [0:v.size] do
    if i > 0 then s := s ++ ","
    s := s ++ hex v[i]!
  s

def codecText : Codec → String
  | .avc w h nls p l sps pps avcc =>
    "avc " ++ n w.toNat ++ "x" ++ n h.toNat ++ " nls" ++ n nls.toNat ++ " p" ++ n p.toNat ++ " l" ++ n l.toNat
      ++ " sps[" ++ hexList sps ++ "] pps[" ++ hexList pps ++ "] avcc[" ++ hex avcc ++ "]"
  | .aac ch sr ot asc =>
    "aac ch" ++ n ch.toNat ++ " sr" ++ n sr.toNat ++ " ot" ++ n ot.toNat ++ " asc[" ++ hex asc ++ "]"
  | .other t => "other " ++ hex ⟨#[(t >>> 24).toUInt8, (t >>> 16).toUInt8, (t >>> 8).toUInt8, t.toUInt8]⟩

def initLine : Result Init → String
  | .ok i => "ok track" ++ n i.trackId.toNat ++ " ts" ++ n i.timescale.toNat ++ " edit" ++ decI64 i.editOffset
      ++ " " ++ codecText i.codec ++ " def" ++ n i.defaults.duration.toNat ++ "," ++ n i.defaults.size.toNat
      ++ "," ++ n i.defaults.flags.toNat
  | .error e => "err " ++ e

def sidxLine : Result Index → String
  | .ok x => Id.run do
    let mut h : UInt64 := 0xcbf29ce484222325
    for s in x.segments do
      let t := (u s.offset ++ "," ++ n s.size.toNat ++ "," ++ u s.start ++ "," ++ n s.duration.toNat ++ ","
        ++ (if s.startsWithSap then "1" else "0") ++ ";").toUTF8
      for b in t do h := (h ^^^ b.toUInt64) * 0x100000001b3
    "ok ts" ++ n x.timescale.toNat ++ " n" ++ n x.segments.size ++ " h" ++ hex64 h
  | .error e => "err " ++ e

def fragLine : Result (Array Sample) → String
  | .ok v =>
    if h : v.size > 0 then
      let s := v[v.size - 1]
      "ok n" ++ n v.size ++ " last" ++ u s.dts ++ "," ++ decI64 s.pts ++ "," ++ n s.duration.toNat ++ ","
        ++ (if s.keyframe then "1" else "0") ++ "," ++ n s.size.toNat
    else "ok n0"
  | .error e => "err " ++ e

def be32 (b : ByteArray) (p : Nat) : Nat := (Rd.be32 b p).toNat

/-- The dump of src/mp4/examples/dump.rs for `file`. -/
def dump (file : ByteArray) : Except String Out := do
  let mut o : Out := ByteArray.emptyWithCapacity (1 <<< 20)
  let initR := parseInit file
  o := put o ("init " ++ initLine initR ++ "\n")
  let init ← initR
  let index ← parseSidx file 0
  o := put o ("index ts" ++ n index.timescale.toNat ++ " n" ++ n index.segments.size ++ "\n")
  let mut k := 0
  for i in [0:index.segments.size] do
    let s := index.segments[i]!
    o := put o ("seg " ++ n i ++ " off" ++ u s.offset ++ " size" ++ n s.size.toNat ++ " start" ++ u s.start
      ++ " dur" ++ n s.duration.toNat ++ " sap" ++ (if s.startsWithSap then "1" else "0") ++ "\n")
    let frag := file.extract s.offset.toNat (s.offset.toNat + s.size.toNat)
    for x in ← parseFragments frag init do
      o := put o ("s " ++ n k ++ " dts" ++ u x.dts ++ " pts" ++ decI64 x.pts ++ " dur" ++ n x.duration.toNat
        ++ " key" ++ (if x.keyframe then "1" else "0") ++ " size" ++ n x.size.toNat ++ " fnv"
        ++ hex64 (fnv1a64 frag x.offset (x.offset + x.size.toNat)) ++ "\n")
      k := k + 1
  -- Malformed input.
  let mut sidxAt := 0
  while file.extract (sidxAt + 4) (sidxAt + 8) != "sidx".toUTF8 do
    sidxAt := sidxAt + be32 file sidxAt
  let sidxLen := be32 file sidxAt
  let sidx := file.extract sidxAt (sidxAt + sidxLen)
  for j in [0:sidxAt + 1] do
    o := put o ("init-trunc " ++ n j ++ " " ++ initLine (parseInit (file.extract 0 j)) ++ "\n")
  let mut b := file.extract 0 sidxAt
  for j in [0:sidxAt] do
    b := b.set! j (b[j]! ^^^ 0xff)
    o := put o ("init-flip " ++ n j ++ " " ++ initLine (parseInit b) ++ "\n")
    b := b.set! j (b[j]! ^^^ 0xff)
  for j in [0:sidxLen + 1] do
    o := put o ("sidx-trunc " ++ n j ++ " " ++ sidxLine (parseSidx (sidx.extract 0 j) sidxAt.toUInt64) ++ "\n")
  b := sidx
  for j in [0:sidxLen] do
    b := b.set! j (b[j]! ^^^ 0xff)
    o := put o ("sidx-flip " ++ n j ++ " " ++ sidxLine (parseSidx b sidxAt.toUInt64) ++ "\n")
    b := b.set! j (b[j]! ^^^ 0xff)
  let s0 := index.segments[0]!
  let frag := file.extract s0.offset.toNat (s0.offset.toNat + s0.size.toNat)
  let moofLen := be32 frag 0
  let mut ks : Array Nat := #[]
  for j in [0:moofLen + 17] do ks := ks.push j
  let mut q := moofLen + 16
  while q < frag.size do
    ks := ks.push q
    q := q + 4099
  ks := ks.push frag.size
  for j in ks do
    o := put o ("frag-trunc " ++ n j ++ " " ++ fragLine (parseFragments (frag.extract 0 j) init) ++ "\n")
  pure o

/-- Line `i` (0-based) of `b`. -/
def lineAt (b : ByteArray) (i : Nat) : String := Id.run do
  let mut p := 0
  let mut k := 0
  while k < i && p < b.size do
    if b[p]! == 10 then k := k + 1
    p := p + 1
  let mut q := p
  while q < b.size && b[q]! != 10 do q := q + 1
  lossy (b.extract p q)

def countLines (b : ByteArray) : Nat := Id.run do
  let mut c := 0
  for x in b do
    if x == 10 then c := c + 1
  return c

/-- Compare the dump of `file` with the Rust reference text. -/
def check (name : String) (file : ByteArray) (want : ByteArray) : String :=
  match dump file with
  | .error e => "mp4 " ++ name ++ ": FAIL dump error: " ++ e ++ "\n"
  | .ok got =>
    let summary := "mp4 " ++ name ++ ": " ++ n file.size ++ " bytes, dump " ++ n (countLines got)
      ++ " lines, "
    if got == want then
      summary ++ "identical to the Rust reference (fnv " ++ hex64 (fnv1a64 got 0 got.size) ++ ")\n"
    else Id.run do
      let mut i := 0
      let mut p := 0
      while p < got.size && p < want.size && got[p]! == want[p]! do
        if got[p]! == 10 then i := i + 1
        p := p + 1
      summary ++ "FAIL first difference at line " ++ n (i + 1) ++ "\n  rust: " ++ lineAt want i
        ++ "\n  lean: " ++ lineAt got i ++ "\n"

def bx (ty : String) (body : ByteArray) : ByteArray :=
  let n := body.size + 8
  ⟨#[(n >>> 24).toUInt8, (n >>> 16).toUInt8, (n >>> 8).toUInt8, n.toUInt8]⟩ ++ ty.toUTF8 ++ body

/-- The unit tests of src/mp4/src/lib.rs. -/
def unitTests : String := Id.run do
  let mut o := ""
  -- rejects_box_larger_than_buffer
  let b := (bx "moov" (zeros 4)).set! 3 200
  o := o ++ "unit rejects_box_larger_than_buffer: " ++ (match parseInit b with
    | .error e => "ok (" ++ e ++ ")" | .ok _ => "FAIL") ++ "\n"
  -- rejects_truncated_input_without_panicking
  let b := bx "sidx" ⟨Array.replicate 56 0xff⟩
  let mut errs := 0
  for k in [0:64] do
    if let .error _ := parseSidx (b.extract 0 (min k b.size)) 0 then errs := errs + 1
  o := o ++ "unit rejects_truncated_input_without_panicking: ok (" ++ n errs ++ " of 64 rejected)\n"
  -- splits_length_prefixed_nals
  let s : ByteArray := ⟨#[0, 0, 0, 2, 0x65, 1, 0, 0, 0, 1, 0x41]⟩
  let r1 := match avcNals s 0 s.size 4 with
    | .ok v => v.size == 2 && s.extract v[0]!.1 (v[0]!.1 + v[0]!.2) == ⟨#[0x65, 1]⟩
        && s.extract v[1]!.1 (v[1]!.1 + v[1]!.2) == ⟨#[0x41]⟩
    | .error _ => false
  let r2 := match avcNals s 0 5 4 with | .error _ => true | .ok _ => false
  o := o ++ "unit splits_length_prefixed_nals: " ++ (if r1 && r2 then "ok" else "FAIL") ++ "\n"
  o

def decode (s : String) : ByteArray := (base64Decode s.toUTF8).getD ByteArray.empty

@[export zr_test_transcript] def transcript (k : Nat) : ByteArray :=
  let m140 := decode Fixtures.m140
  let m136 := decode Fixtures.m136
  let o := check "140.m4a" (m140.extract k m140.size) Fixtures.d140.toUTF8
    ++ check "136.mp4" (m136.extract k m136.size) Fixtures.d136.toUTF8 ++ unitTests
  o.toUTF8

end Mp4Test
