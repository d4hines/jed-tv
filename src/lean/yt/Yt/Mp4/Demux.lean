import Yt.Json.Fmt

/-! Minimal fragmented-MP4 demuxer for YouTube DASH files (H.264 video, AAC
audio): a port of `src/mp4/src/lib.rs` with the same results and the same
error messages.

A YouTube MP4 format is `ftyp`, `moov` (the "init" range), `sidx` (the
"index" range), then `moof`+`mdat` fragments. `initRange`/`indexRange` in
the /player reply give the byte ranges, so a player needs:
  1. `parseInit` on the init range (codec config, timescale, defaults),
  2. `parseSidx` on the index range (where each fragment is, and when),
  3. `parseFragments` on each fragment's bytes (the samples).
All parsing is bounds-checked; malformed input yields `.error msg`.

Differences from the Rust API, none in the parsed values:
- a `Sample` gives `offset`/`size` into the fragment buffer instead of a
  copy of its bytes (`Sample.data` copies them out when needed);
- signed 64-bit values (`Init.editOffset`, `Sample.pts`) are `UInt64` in
  two's complement (our runtime has no `Int64` arithmetic); use
  `Yt.Fmt.isNeg64`/`Yt.Fmt.decI64`, or `Sample.ptsMinus`. -/
namespace Yt.Mp4
open Yt.Fmt

abbrev Result (α : Type) := Except String α

/-- A big-endian reader over `buf[lo, lo+len)`; `p` is relative to `lo`, as
in Rust's `Rd` over a sub-slice (error messages report it). -/
structure Rd where
  buf : ByteArray
  lo : Nat
  len : Nat
  p : Nat

namespace Rd
def new (buf : ByteArray) (lo hi : Nat) : Rd := ⟨buf, lo, hi - lo, 0⟩
@[inline] def left (r : Rd) : Nat := r.len - r.p
/-- Absolute position of the cursor in `buf`. -/
@[inline] def abs (r : Rd) : Nat := r.lo + r.p
@[inline] def stop (r : Rd) : Nat := r.lo + r.len

def take (r : Rd) (n : Nat) : Result (Nat × Rd) :=
  if r.p + n ≤ r.len then .ok (r.lo + r.p, { r with p := r.p + n })
  else .error ("truncated: need " ++ toString n ++ " bytes at " ++ toString r.p)

@[inline] def skip (r : Rd) (n : Nat) : Result Rd := do
  let (_, r) ← r.take n
  pure r

@[inline] def u8 (r : Rd) : Result (UInt8 × Rd) := do
  let (a, r) ← r.take 1
  pure (byteAt r.buf a, r)

@[inline] def u16 (r : Rd) : Result (UInt16 × Rd) := do
  let (a, r) ← r.take 2
  pure (((byteAt r.buf a).toUInt16 <<< 8) ||| (byteAt r.buf (a+1)).toUInt16, r)

@[inline] def u24 (r : Rd) : Result (UInt32 × Rd) := do
  let (a, r) ← r.take 3
  pure (((byteAt r.buf a).toUInt32 <<< 16) ||| ((byteAt r.buf (a+1)).toUInt32 <<< 8)
        ||| (byteAt r.buf (a+2)).toUInt32, r)

@[inline] def be32 (b : ByteArray) (a : Nat) : UInt32 :=
  ((byteAt b a).toUInt32 <<< 24) ||| ((byteAt b (a+1)).toUInt32 <<< 16)
  ||| ((byteAt b (a+2)).toUInt32 <<< 8) ||| (byteAt b (a+3)).toUInt32

@[inline] def u32 (r : Rd) : Result (UInt32 × Rd) := do
  let (a, r) ← r.take 4
  pure (be32 r.buf a, r)

@[inline] def u64 (r : Rd) : Result (UInt64 × Rd) := do
  let (a, r) ← r.take 8
  pure (((be32 r.buf a).toUInt64 <<< 32) ||| (be32 r.buf (a+4)).toUInt64, r)

/-- version + 24-bit flags of a FullBox. -/
@[inline] def full (r : Rd) : Result (UInt8 × UInt32 × Rd) := do
  let (v, r) ← r.u8
  let (f, r) ← r.u24
  pure (v, f, r)
end Rd

/-- One box: its type (four bytes, big-endian), its payload
`buf[body, stop)` and where it starts, all absolute positions in `buf`. -/
structure Bx where
  ty : UInt32
  body : Nat
  start : Nat
  stop : Nat

def fourcc (s : String) : UInt32 := Rd.be32 s.toUTF8 0

def tyName (ty : UInt32) : String :=
  lossy ⟨#[(ty >>> 24).toUInt8, (ty >>> 16).toUInt8, (ty >>> 8).toUInt8, ty.toUInt8]⟩

/-- The box starting at `p` among the boxes laid out back to back in
`buf[p, hi)` (Rust's `boxes` iterator, one step). -/
def boxAt (buf : ByteArray) (p hi : Nat) : Result Bx := do
  let rd := Rd.new buf p hi
  let (size32, rd) ← rd.u32
  let (ta, rd) ← rd.take 4
  let ty := Rd.be32 buf ta
  let (hdr, size) ← (if size32 == 0 then pure (8, hi - p)
    else if size32 == 1 then do
      let (s, _) ← rd.u64
      pure (16, s.toNat)
    else pure (8, size32.toNat) : Result (Nat × Nat))
  if size < hdr || size > hi - p then
    throw ("box " ++ tyName ty ++ " size " ++ toString size ++ " out of bounds")
  pure ⟨ty, p + hdr, p, p + size⟩

/-- First box of type `ty` in `buf[lo, hi)`, failing on any malformed box
met before it. -/
partial def find (buf : ByteArray) (lo hi : Nat) (ty : UInt32) : Result (Option Bx) := do
  if lo ≥ hi then return none
  let bx ← boxAt buf lo hi
  if bx.ty == ty then return some bx
  find buf bx.stop hi ty

def need (buf : ByteArray) (lo hi : Nat) (ty : UInt32) : Result Bx := do
  match ← find buf lo hi ty with
  | some b => pure b
  | none => throw ("missing box " ++ tyName ty)

/-- Rust's `for bx in boxes(b) { let bx = bx?; ... }` over `buf[lo, hi)`:
lazy, so an error from `f` comes before one from a later malformed box. -/
partial def forBoxes {σ : Type} (buf : ByteArray) (lo hi : Nat) (s : σ)
    (f : Bx → σ → Result σ) : Result σ := do
  if lo ≥ hi then return s
  let bx ← boxAt buf lo hi
  let s ← f bx s
  forBoxes buf bx.stop hi s f

inductive Codec where
  /-- H.264. `avcc` is the raw avcC record (decoder "extradata"). -/
  | avc (width height : UInt16) (nalLengthSize profile level : UInt8)
        (sps pps : Array ByteArray) (avcc : ByteArray)
  /-- AAC. `asc` is the AudioSpecificConfig. -/
  | aac (channels : UInt16) (sampleRate : UInt32) (objectType : UInt8) (asc : ByteArray)
  | other (fourcc : UInt32)
  deriving Inhabited

structure Defaults where
  duration : UInt32 := 0
  size : UInt32 := 0
  flags : UInt32 := 0
  deriving Inhabited

structure Init where
  trackId : UInt32
  timescale : UInt32
  codec : Codec
  defaults : Defaults
  /-- Media time where presentation starts (first non-empty `elst` edit),
  in the track timescale (signed, two's complement). Subtract it from
  sample pts to get presentation time. -/
  editOffset : UInt64
  deriving Inhabited

def parseAvc (buf : ByteArray) (body stop : Nat) : Result Codec := do
  let r := Rd.new buf body stop
  let r ← r.skip (8 + 16)
  let (width, r) ← r.u16
  let (height, r) ← r.u16
  let r ← r.skip (4 + 4 + 4 + 2 + 32 + 2 + 2)
  let bx ← need buf r.abs stop (fourcc "avcC")
  let a := Rd.new buf bx.body bx.stop
  let (_version, a) ← a.u8
  let (profile, a) ← a.u8
  let (_compat, a) ← a.u8
  let (level, a) ← a.u8
  let (nls, a) ← a.u8
  let nls := (nls &&& 3) + 1
  let (nsps, a) ← a.u8
  let mut a := a
  let mut sps := #[]
  for _ in [0:(nsps &&& 0x1f).toNat] do
    let (n, a1) ← a.u16
    let (s, a2) ← a1.take n.toNat
    sps := sps.push (buf.extract s (s + n.toNat))
    a := a2
  let (npps, a3) ← a.u8
  a := a3
  let mut pps := #[]
  for _ in [0:npps.toNat] do
    let (n, a1) ← a.u16
    let (s, a2) ← a1.take n.toNat
    pps := pps.push (buf.extract s (s + n.toNat))
    a := a2
  pure (.avc width height nls profile level sps pps (buf.extract bx.body bx.stop))

/-- MPEG-4 descriptor header: tag and variable-length size; the reader of
its payload and the reader after it. -/
def descriptor (r0 : Rd) : Result (UInt8 × Rd × Rd) := do
  let (tag, r1) ← r0.u8
  let mut len := 0
  let mut r := r1
  for _ in [0:4] do
    let (c, r1) ← r.u8
    r := r1
    len := len * 128 + (c &&& 0x7f).toNat
    if c &&& 0x80 == 0 then break
  let (s, r2) ← r.take len
  pure (tag, Rd.new r2.buf s (s + len), r2)

def parseMp4a (buf : ByteArray) (body stop : Nat) : Result Codec := do
  let r := Rd.new buf body stop
  let r ← r.skip (8 + 8)
  let (channels, r) ← r.u16
  let r ← r.skip (2 + 2 + 2)
  let (sr, r) ← r.u32
  let sampleRate := sr >>> 16
  let bx ← need buf r.abs stop (fourcc "esds")
  let e := Rd.new buf bx.body bx.stop
  let (_, _, e) ← e.full
  let (tag, es, _) ← descriptor e
  if tag != 3 then throw ("esds: expected ES_Descriptor, got tag " ++ toString tag.toNat)
  let es ← es.skip 2
  let (flags, es) ← es.u8
  let es ← if flags &&& 0x80 != 0 then es.skip 2 else pure es
  let es ← if flags &&& 0x40 != 0 then do
      let (n, es) ← es.u8
      es.skip n.toNat
    else pure es
  let es ← if flags &&& 0x20 != 0 then es.skip 2 else pure es
  let (tag, dc, _) ← descriptor es
  if tag != 4 then throw ("esds: expected DecoderConfigDescriptor, got tag " ++ toString tag.toNat)
  let dc ← dc.skip 13
  let (tag, dsi, _) ← descriptor dc
  if tag != 5 then throw ("esds: expected DecoderSpecificInfo, got tag " ++ toString tag.toNat)
  let asc := buf.extract dsi.lo dsi.stop
  let objectType := if asc.size > 0 then byteAt asc 0 >>> 3 else 0
  pure (.aac channels sampleRate objectType asc)

/-- Parse the init range (`ftyp` + `moov`). Expects a single track, as YouTube serves. -/
def parseInit (b : ByteArray) : Result Init := do
  let moov ← need b 0 b.size (fourcc "moov")
  let trak ← need b moov.body moov.stop (fourcc "trak")
  let tkhdB ← need b trak.body trak.stop (fourcc "tkhd")
  let (v, _, tkhd) ← (Rd.new b tkhdB.body tkhdB.stop).full
  let tkhd ← tkhd.skip (if v == 1 then 16 else 8)
  let (trackId, _) ← tkhd.u32
  let mut editOffset : UInt64 := 0
  if let some edts ← find b trak.body trak.stop (fourcc "edts") then
    if let some elst ← find b edts.body edts.stop (fourcc "elst") then
      let (v, _, r) ← (Rd.new b elst.body elst.stop).full
      let (count, r) ← r.u32
      let mut r := r
      for _ in [0:count.toNat] do
        let (mediaTime, r1) ← (if v == 1 then do
            let r ← r.skip 8
            r.u64
          else do
            let r ← r.skip 4
            let (t, r) ← r.u32
            pure (sext32 t, r) : Result (UInt64 × Rd))
        r ← r1.skip 4  -- media_rate
        if !isNeg64 mediaTime then
          editOffset := mediaTime
          break
  let mdia ← need b trak.body trak.stop (fourcc "mdia")
  let mdhdB ← need b mdia.body mdia.stop (fourcc "mdhd")
  let (v, _, mdhd) ← (Rd.new b mdhdB.body mdhdB.stop).full
  let mdhd ← mdhd.skip (if v == 1 then 16 else 8)
  let (timescale, _) ← mdhd.u32
  if timescale == 0 then throw "timescale 0"
  let minf ← need b mdia.body mdia.stop (fourcc "minf")
  let stbl ← need b minf.body minf.stop (fourcc "stbl")
  let stsdB ← need b stbl.body stbl.stop (fourcc "stsd")
  let (_, _, stsd) ← (Rd.new b stsdB.body stsdB.stop).full
  let (n, stsd) ← stsd.u32
  if n < 1 then throw "stsd has no entries"
  if stsd.abs ≥ stsd.stop then throw "empty stsd"
  let entry ← boxAt b stsd.abs stsd.stop
  let codec ← (if entry.ty == fourcc "avc1" || entry.ty == fourcc "avc3" then parseAvc b entry.body entry.stop
    else if entry.ty == fourcc "mp4a" then parseMp4a b entry.body entry.stop
    else pure (.other entry.ty))
  let mut defaults : Defaults := {}
  if let some mvex ← find b moov.body moov.stop (fourcc "mvex") then
    if let some trex ← find b mvex.body mvex.stop (fourcc "trex") then
      let (_, _, r) ← (Rd.new b trex.body trex.stop).full
      let r ← r.skip 8  -- track_ID, default_sample_description_index
      let (d, r) ← r.u32
      let (s, r) ← r.u32
      let (f, _) ← r.u32
      defaults := { duration := d, size := s, flags := f }
  pure { trackId, timescale, codec, defaults, editOffset }

/-- One `sidx` reference: a fragment's absolute byte range and time span. -/
structure SegmentRef where
  offset : UInt64
  size : UInt32
  /-- Earliest presentation time, in the sidx timescale. -/
  start : UInt64
  duration : UInt32
  startsWithSap : Bool
  deriving Inhabited

structure Index where
  timescale : UInt32
  segments : Array SegmentRef
  deriving Inhabited

/-- Parse the index range. `fileOffset` is where `b` starts in the file
(`indexRange.start`), since segment offsets are relative to the sidx end. -/
def parseSidx (b : ByteArray) (fileOffset : UInt64) : Result Index := do
  let sidx ← need b 0 b.size (fourcc "sidx")
  let (v, _, r) ← (Rd.new b sidx.body sidx.stop).full
  let r ← r.skip 4  -- reference_ID
  let (timescale, r) ← r.u32
  let (t, first, r) ← (if v == 0 then do
      let (t, r) ← r.u32
      let (f, r) ← r.u32
      pure (t.toUInt64, f.toUInt64, r)
    else do
      let (t, r) ← r.u64
      let (f, r) ← r.u64
      pure (t, f, r) : Result (UInt64 × UInt64 × Rd))
  let r ← r.skip 2
  let (count, r) ← r.u16
  let mut offset := fileOffset + sidx.stop.toUInt64 + first
  let mut t := t
  let mut r := r
  let mut segments := Array.mkEmpty count.toNat
  for _ in [0:count.toNat] do
    let (a, r1) ← r.u32
    if a &&& 0x80000000 != 0 then throw "hierarchical sidx not supported"
    let size := a &&& 0x7fffffff
    let (duration, r2) ← r1.u32
    let (sap, r3) ← r2.u32
    r := r3
    segments := segments.push { offset, size, start := t, duration,
                                startsWithSap := sap &&& 0x80000000 != 0 }
    offset := offset + size.toUInt64
    t := t + duration.toUInt64
  pure { timescale, segments }

/-- A coded sample. Times are in the track timescale; `pts = dts +
composition offset` (signed, two's complement). Its bytes are
`buf[offset, offset + size)` of the buffer given to `parseFragments`. -/
structure Sample where
  dts : UInt64
  pts : UInt64
  duration : UInt32
  keyframe : Bool
  offset : Nat
  size : UInt32
  deriving Inhabited

/-- The sample's bytes, copied out of the fragment buffer. -/
def Sample.data (s : Sample) (buf : ByteArray) : ByteArray :=
  buf.extract s.offset (s.offset + s.size.toNat)

def parseTraf (file : ByteArray) (moofStart : Nat) (traf : Bx) (init : Init)
    (out : Array Sample) : Result (Array Sample) := do
  let tfhd ← need file traf.body traf.stop (fourcc "tfhd")
  let (_, fl, t) ← (Rd.new file tfhd.body tfhd.stop).full
  let (track, t) ← t.u32
  if track != init.trackId then return out
  let (base, t) ← if fl &&& 0x01 != 0 then t.u64 else pure (moofStart.toUInt64, t)
  let t ← if fl &&& 0x02 != 0 then t.skip 4 else pure t
  let (dur, t) ← if fl &&& 0x08 != 0 then t.u32 else pure (init.defaults.duration, t)
  let (size, t) ← if fl &&& 0x10 != 0 then t.u32 else pure (init.defaults.size, t)
  let (flags, _) ← if fl &&& 0x20 != 0 then t.u32 else pure (init.defaults.flags, t)
  let dts ← match ← find file traf.body traf.stop (fourcc "tfdt") with
    | some bx => do
      let (v, _, r) ← (Rd.new file bx.body bx.stop).full
      if v == 1 then do let (x, _) ← r.u64; pure x
      else do let (x, _) ← r.u32; pure x.toUInt64
    | none => pure (if h : out.size > 0 then
        let s := out[out.size - 1]
        s.dts + s.duration.toUInt64 else 0)
  let (_, _, out) ← forBoxes file traf.body traf.stop (dts, base, out) fun trun (dts, dataPos, out) => do
    if trun.ty != fourcc "trun" then return (dts, dataPos, out)
    let (v, tf, r) ← (Rd.new file trun.body trun.stop).full
    let (count, r) ← r.u32
    let (r, dataPos) ← if tf &&& 0x01 != 0 then do
        let (o, r) ← r.u32
        pure (r, base + sext32 o)
      else pure (r, dataPos)
    let (firstFlags, r) ← if tf &&& 0x04 != 0 then do
        let (f, r) ← r.u32
        pure (some f, r)
      else pure (none, r)
    let p0 := tf &&& 0x100 != 0
    let p1 := tf &&& 0x200 != 0
    let p2 := tf &&& 0x400 != 0
    let p3 := tf &&& 0x800 != 0
    let perLen := 4 * ((if p0 then 1 else 0) + (if p1 then 1 else 0) + (if p2 then 1 else 0)
      + (if p3 then 1 else 0))
    -- Rust: `count as usize * per_len` overflowing usize (64-bit) also fails.
    if perLen > 0 && count.toNat * perLen > r.left then
      throw ("trun claims " ++ toString count.toNat ++ " samples but has " ++ toString r.left ++ " bytes")
    let mut r := r
    let mut dts := dts
    let mut dataPos := dataPos
    let mut out := out
    for i in [0:count.toNat] do
      let (d, r1) ← if p0 then r.u32 else pure (dur, r)
      let (s, r2) ← if p1 then r1.u32 else pure (size, r1)
      let (f, r3) ← if p2 then r2.u32
        else pure (if i == 0 then firstFlags.getD flags else flags, r2)
      let (cto, r4) ← if p3 then do
          let (c, r4) ← r3.u32
          pure (if v == 0 then c.toUInt64 else sext32 c, r4)
        else pure (0, r3)
      r := r4
      let start := dataPos.toNat
      if start + s.toNat > file.size then
        throw ("sample " ++ toString i ++ " [" ++ toString start ++ "+" ++ toString s.toNat
               ++ "] outside the fragment")
      out := out.push { dts, pts := dts + cto, duration := d, keyframe := f &&& 0x00010000 == 0,
                        offset := start, size := s }
      dataPos := dataPos + s.toUInt64
      dts := dts + d.toUInt64
    pure (dts, dataPos, out)
  pure out

/-- Parse every `moof`+`mdat` pair in `b` (the bytes of one or more fragments). -/
def parseFragments (b : ByteArray) (init : Init) : Result (Array Sample) :=
  forBoxes b 0 b.size #[] fun bx out => do
    if bx.ty != fourcc "moof" then return out
    forBoxes b bx.body bx.stop out fun traf out => do
      if traf.ty != fourcc "traf" then return out
      parseTraf b bx.start traf init out

/-- Split an AVC sample `buf[start, stop)` (length-prefixed NAL units) into
the NAL units' (offset, size) in `buf`. -/
def avcNals (buf : ByteArray) (start stop : Nat) (nalLengthSize : UInt8) :
    Result (Array (Nat × Nat)) := do
  let mut r := Rd.new buf start stop
  let mut nals := #[]
  while r.left > 0 do
    let mut n := 0
    for _ in [0:nalLengthSize.toNat] do
      let (c, r1) ← r.u8
      r := r1
      n := n * 256 + c.toNat
    let (a, r1) ← r.take n
    r := r1
    nals := nals.push (a, n)
  pure nals

end Yt.Mp4
