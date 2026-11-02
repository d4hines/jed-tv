import Yt.App.Model
import Yt.Ui.Text
import Yt.Mp4.Demux
import Yt.Json.YouTube
import Yt.Aac.Decoder
import Yt.H264.Stream

/-! The player's engine, driven by the event loop. Each track (video, and
audio when available) downloads its header (init + sidx) and then fragments
two ahead by ranged GETs, which the loop carries out (`wants`/`onData`).
Video samples go to the Lean H.264 decoder (`Yt.H264.Stream`, with its NEON
kernels); a picture is shown when the clock reaches it, and while pictures
come late the decoder skips non-reference ones to catch up.
Audio is decoded in Lean (`Yt.Aac`) and written to the device while less
than `audioLead` ms are queued, so the loop never blocks on it; once audio
plays, its position (frames written minus queued) is the clock. -/
namespace Yt.App.Player
open Yt.Platform Yt.Platform.Tv Yt.Ui

/-- One MP4 track's downloads and demuxed samples. -/
structure Track where
  url : String
  initRange : UInt64 × UInt64
  indexRange : UInt64 × UInt64
  init : Option Mp4.Init := none
  index : Option Mp4.Index := none
  nextSeg : Nat := 0
  inflight : Bool := false
  /-- Bumped on seek: downloads started before it are discarded. -/
  gen : Nat := 0
  segs : Array (ByteArray × Array Mp4.Sample) := #[]
  sample : Nat := 0
  deriving Inhabited

def Track.ofFormat (f : Json.YouTube.Format) : Track :=
  { url := f.url, initRange := f.initRange, indexRange := f.indexRange }

/-- A ranged download a track wants: (fragment, or none for the header;
first and last byte). -/
def Track.want (t : Track) : Option (Option Nat × UInt64 × UInt64) :=
  if t.inflight then none
  else match t.index with
    | none => some (none, t.initRange.1, t.indexRange.2)
    | some idx =>
      if t.segs.size ≥ 2 then none
      else (idx.segments[t.nextSeg]?).map fun r => (some t.nextSeg, r.offset, r.offset + r.size.toUInt64 - 1)

def Track.allIn (t : Track) : Bool :=
  match t.index with | some idx => t.nextSeg ≥ idx.segments.size | none => false

/-- The current sample (and its fragment's bytes), dropping used-up fragments. -/
partial def Track.current (t : Track) : Track × Option (ByteArray × Mp4.Sample) :=
  match t.segs[0]? with
  | none => (t, none)
  | some (buf, samples) =>
    match samples[t.sample]? with
    | some s => (t, some (buf, s))
    | none => Track.current { t with segs := t.segs.extract 1 t.segs.size, sample := 0 }

def Track.advance (t : Track) : Track := { t with sample := t.sample + 1 }

/-- Presentation time (ms) of a sample. -/
def sampleMs (init : Mp4.Init) (smp : Mp4.Sample) : UInt64 :=
  let v := smp.pts - init.editOffset
  if Yt.Fmt.isNeg64 v || init.timescale == 0 then 0 else v * 1000 / init.timescale.toUInt64

/-- The index of the last fragment starting at or before `ms`, and its start (ms). -/
def Track.segmentAt (t : Track) (ms : UInt64) : Nat × UInt64 := Id.run do
  let some idx := t.index | return (0, 0)
  let ts := idx.timescale.toUInt64
  if ts == 0 then return (0, 0)
  let mut k := 0
  for i in [0:idx.segments.size] do
    if (idx.segments[i]!.start * 1000) / ts ≤ ms then k := i
  return (k, (idx.segments[k]!.start * 1000) / ts)

/-- The track's duration (ms) by its index (the end of the last fragment), or 0. -/
def Track.indexMs (t : Track) : UInt64 := Id.run do
  let some idx := t.index | return 0
  let some last := idx.segments.back? | return 0
  if idx.timescale == 0 then return 0
  return (last.start + last.duration.toUInt64) * 1000 / idx.timescale.toUInt64

def Track.restartAt (t : Track) (k : Nat) : Track :=
  { t with segs := #[], sample := 0, nextSeg := k, inflight := false, gen := t.gen + 1 }

/-- A header download finished: parse init and index. -/
def Track.onHeader (t : Track) (body : ByteArray) : Except String Track := do
  let initLen := (t.initRange.2 - t.initRange.1 + 1).toNat
  let init ← Mp4.parseInit (body.extract 0 initLen)
  let idx ← Mp4.parseSidx (body.extract (t.indexRange.1 - t.initRange.1).toNat body.size) t.indexRange.1
  return { t with init, index := idx, inflight := false }

def Track.onFragment (t : Track) (i : Nat) (body : ByteArray) : Except String Track := do
  let some init := t.init | throw "fragment before header"
  let samples ← Mp4.parseFragments body init
  return { t with segs := t.segs.push (body, samples), nextSeg := i + 1, inflight := false }

/-- Milliseconds of audio kept queued in the device. -/
def audioLead : UInt64 := 300

structure State where
  videoId : String := ""
  title : String
  durationMs : UInt64
  video : Track
  audio : Option Track := none
  dec : Option H264.Stream := none
  aac : Option Aac.Decoder := none
  out : Option Audio := none
  /-- Frames written since the audio clock was (re)anchored at `audioBase`. -/
  written : UInt64 := 0
  audioBase : UInt64 := 0
  audioRate : UInt64 := 0
  /-- A received picture not shown yet, and its time (ms). -/
  pending : Bool := false
  pendingMs : UInt64 := 0
  flushed : Bool := false
  /-- Without audio: media time `base` at monotonic `ref` (ms). -/
  ref : UInt64 := 0
  base : UInt64 := 0
  paused : Bool := false
  pausedMedia : UInt64 := 0
  overlayUntil : UInt64 := 0
  frames : Nat := 0
  dropped : Nat := 0
  /-- The last picture shown was late: the decoder skips non-reference
  pictures until the video is back on time. -/
  behind : Bool := false
  ended : Bool := false
  error : Option String := none
  /-- Where to start (ms), e.g. where the phone was. -/
  startMs : UInt64 := 0
  /-- After a seek: pictures and audio before this media time (ms) are
  decoded but not shown or played, so seeks land exactly. -/
  skipUntil : UInt64 := 0
  /-- In-app volume 0..100 (attenuation only; PID 1 owns the system volume). -/
  gain : Nat := 100
  muted : Bool := false
  deriving Inhabited

def State.new (title : String) (lengthS : UInt64) (v : Json.YouTube.Format) (a : Option Json.YouTube.Format) : State :=
  { title, durationMs := lengthS * 1000, video := .ofFormat v, audio := a.map .ofFormat }

/-- The duration (ms): the video's index once known (exact), else the
/player reply's whole seconds. -/
def State.duration (s : State) : UInt64 :=
  let i := s.video.indexMs
  if i > 0 then i else s.durationMs

/-- Media time now (ms): the audio position once audio plays, else the
monotonic clock. -/
def State.media (s : State) (now : UInt64) : BaseIO UInt64 := do
  if s.paused then return s.pausedMedia
  if let some out := s.out then
    if s.written > 0 && s.audioRate > 0 then
      let queued := match ← out.queued with | .ok q => q.toUInt64 | .error _ => 0
      let played := if s.written > queued then s.written - queued else 0
      return s.audioBase + played * 1000 / s.audioRate
  return if now ≥ s.ref then s.base + (now - s.ref) else s.base

/-- Downloads wanted now: (track 0 video / 1 audio, generation, fragment, range). -/
def State.wants (s : State) : Array (Nat × Nat × Option Nat × UInt64 × UInt64) :=
  if s.ended || s.error.isSome then #[] else
  let v := (s.video.want.map fun (g, a, b) => #[(0, s.video.gen, g, a, b)]).getD #[]
  match s.audio with
  | some t => v ++ ((t.want.map fun (g, a, b) => #[(1, t.gen, g, a, b)]).getD #[])
  | none => v

def State.markInflight (s : State) (track : Nat) : State :=
  if track == 0 then { s with video := { s.video with inflight := true } }
  else { s with audio := s.audio.map fun t => { t with inflight := true } }

/-- Open the video decoder (Yt.H264) for the video track's codec. -/
def openVideo (t : Track) : BaseIO (Except String H264.Stream) := do
  match t.init.map (fun (i : Mp4.Init) => i.codec) with
  | some (Mp4.Codec.avc _ _ nls _ _ sps pps _) =>
    match H264.Stream.open nls (sps ++ pps) with
    | .ok d => return .ok d
    | .error e => return .error ("decoder open failed: " ++ e)
  | _ => return .error "not an H.264 track"

/-- A download finished (`track`, `gen` as in `wants`). -/
def State.onData (s : State) (track gen : Nat) (seg : Option Nat) (body : ByteArray) : BaseIO State := do
  if track == 0 then
    if gen != s.video.gen then return s
    match seg with
    | none =>
      match s.video.onHeader body with
      | .error e => return { s with error := "video: " ++ e }
      | .ok t =>
        match ← openVideo t with
        | .error e => return { s with error := e }
        | .ok d =>
          let n := match t.index with | some i => i.segments.size | none => 0
          Sys.log ("player: video, " ++ toString n ++ " fragments")
          let k := if s.startMs > 0 then (t.segmentAt s.startMs).1 else 0
          return { s with video := { t with nextSeg := k }, dec := some d, ref := (← Sys.monotonicMs) + 300,
                          base := s.startMs, skipUntil := s.startMs }
    | some i =>
      match s.video.onFragment i body with
      | .ok t => return { s with video := t }
      | .error e => return { s with error := "video: " ++ e }
  else
    let some a := s.audio | return s
    if gen != a.gen then return s
    match seg with
    | none =>
      match a.onHeader body with
      | .error e => Sys.log ("player: no audio: " ++ e); return { s with audio := none }
      | .ok t =>
        let asc := match t.init.map (fun (i : Mp4.Init) => i.codec) with
          | some (Mp4.Codec.aac _ _ _ asc) => some asc | _ => none
        match asc.map Aac.Decoder.init with
        | some (.ok dec) =>
          match ← Audio.open dec.sampleRate dec.channels with
          | .ok out =>
            Sys.log ("player: audio " ++ toString dec.sampleRate ++ " Hz, " ++ toString dec.channels ++ " ch")
            let t := if s.startMs > 0 then { t with nextSeg := (t.segmentAt s.startMs).1 } else t
            return { s with audio := some t, aac := some dec, out := some out, audioRate := dec.sampleRate.toUInt64 }
          | .error e => Sys.log ("player: audio device: errno " ++ toString e); return { s with audio := none }
        | some (.error e) => Sys.log ("player: no audio: " ++ e.message); return { s with audio := none }
        | none => Sys.log "player: no audio: not AAC"; return { s with audio := none }
    | some i =>
      match a.onFragment i body with
      | .ok t => return { s with audio := some t }
      | .error e => Sys.log ("player: audio stopped: " ++ e); return { s with audio := none }

/-- Feed the video decoder until it has a picture for us (or needs data). -/
partial def State.decode (s : State) : BaseIO State := do
  let some dec := s.dec | return s
  if s.pending then return s
  let (dec, got) := dec.receive
  if got then return { s with dec := some dec, pending := true, pendingMs := (dec.frame.map (·.pts)).getD 0 }
  if dec.flushed then return { s with dec := some dec, ended := true }
  let (t, cur) := s.video.current
  let s := { s with video := t, dec := some dec }
  match cur, t.init with
  | some (buf, smp), some init =>
    match dec.send buf smp.offset smp.size.toNat (sampleMs init smp) s.behind with
    | .ok dec => State.decode { s with video := t.advance, dec := some dec }
    | .error e => return { s with error := "video decode error: " ++ e }
  | _, _ =>
    if t.allIn && !s.flushed then State.decode { s with flushed := true, dec := some dec.flush }
    else return s

/-- Scale 16-bit little-endian samples `[i, n)` by `gain`/100 (two's complement in UInt16). -/
partial def scaleLoop (b : ByteArray) (gain i n : Nat) : ByteArray :=
  if i + 1 ≥ n then b else
  let v := b[i]!.toNat ||| (b[i + 1]!.toNat <<< 8)
  let w := if v ≥ 0x8000 then (0x10000 - ((0x10000 - v) * gain / 100)) % 0x10000 else v * gain / 100
  scaleLoop ((b.set! i w.toUInt8).set! (i + 1) (w >>> 8).toUInt8) gain (i + 2) n

def scalePcm (pcm : ByteArray) (gain : Nat) : ByteArray := scaleLoop pcm gain 0 pcm.size

/-- Decode and queue audio while less than `audioLead` ms are queued. -/
partial def State.feedAudio (s : State) : BaseIO State := do
  if s.paused then return s
  let (some a, some dec, some out) := (s.audio, s.aac, s.out) | return s
  let queued := match ← out.queued with | .ok q => q.toUInt64 | .error _ => 0
  if queued * 1000 ≥ audioLead * s.audioRate then return s
  let (t, cur) := a.current
  match cur, t.init with
  | some (buf, smp), some init =>
    let (dec, r) := dec.decode (smp.data buf)
    let s := { s with audio := some t.advance, aac := some dec }
    match r with
    | .ok pcm =>
      -- before a seek target: decoded (for the overlap) but not played
      if sampleMs init smp < s.skipUntil then State.feedAudio s else
      let pcm := if s.muted then scalePcm pcm 0 else if s.gain < 100 then scalePcm pcm s.gain else pcm
      let s := if s.written == 0 then { s with audioBase := sampleMs init smp } else s
      match ← out.write pcm 0 (pcm.size / 2) with
      | .ok () =>
        State.feedAudio { s with written := s.written + (pcm.size / (2 * dec.channels)).toUInt64 }
      | .error e => Sys.log ("player: audio write: errno " ++ toString e); return { s with audio := none }
    | .error e => Sys.log ("player: audio frame: " ++ e.message); State.feedAudio s
  | _, _ => return { s with audio := some t }

def fmtTime (ms : UInt64) : String :=
  let s := (ms / 1000).toNat
  let m := s / 60
  let ss := s % 60
  toString m ++ ":" ++ (if ss < 10 then "0" else "") ++ toString ss

/-- Log progress every 120 pictures (shown, or dropped before a seek target). -/
def State.progress (s : State) (media : UInt64) : BaseIO State := do
  if (s.frames + s.dropped) % 120 == 0 then
    Sys.log ("player: " ++ toString s.frames ++ " frames shown, " ++ toString s.dropped ++ " dropped, media " ++
      fmtTime media ++ ", audio frames " ++ toString s.written.toNat)
  return s

/-- Draw the pending picture letterboxed, with the overlay when wanted. -/
def State.draw (s : State) (d : Display) (f : Fonts) (now media : UInt64) : BaseIO Unit := do
  let some pic := s.dec.bind (·.frame) | return
  let fw := (← d.width).toNat
  let fh := (← d.height).toNat
  let vw := pic.width
  let vh := pic.height
  if vw == 0 || vh == 0 then return
  let (w, h) := if fw * vh ≥ fh * vw then ((vw * fh) / vh, fh) else (fw, (vh * fw) / vw)
  let x := (fw - w) / 2
  let y := (fh - h) / 2
  if y > 0 then
    d.fill 0 0 fw y 0
    d.fill 0 (y + h) fw (fh - y - h) 0
  if x > 0 then
    d.fill 0 y x h 0
    d.fill (x + w) y (fw - x - w) h 0
  d.blitYuv pic.y pic.cb pic.cr vw vh x y w h
  if s.paused || now < s.overlayUntil then
    d.fill 0 (fh - 120) fw 120 0x101219
    let _ ← f.body.draw d 48 (fh - 112) (f.body.ellipsize s.title.toUTF8 (fw - 300)) 0xF5F5FA
    let dur := s.duration
    let t := (if s.paused then "‖  " else "▶  ") ++ fmtTime (if dur > 0 then min media dur else media) ++ " / " ++ fmtTime dur
    let _ ← f.small.draw d 48 (fh - 60) t.toUTF8 0xAFB4C3
    let barX := 300
    let barW := fw - barX - 48
    d.fill barX (fh - 48) barW 6 0x3A3E4C
    if dur > 0 then
      d.fill barX (fh - 48) (min barW ((media * barW.toUInt64 / dur).toNat)) 6 0xAA1826

/-- Show the pending picture if its time has come; how long to wait (ms). -/
def State.present (s : State) (d : Display) (f : Fonts) : BaseIO (State × Nat) := do
  let now ← Sys.monotonicMs
  if !s.pending || s.paused then return (s, 15)
  let media ← s.media now
  if s.pendingMs < s.skipUntil then
    return ({ s with pending := false, dropped := s.dropped + 1 }, 0)
  if s.pendingMs > media then return (s, min 15 (s.pendingMs - media).toNat)
  -- a late picture is shown all the same (decoding it was the cost); being
  -- late makes the decoder skip non-reference pictures to catch up
  let late := s.frames > 0 && s.pendingMs + 80 < media
  match ← d.acquire with
  | .ok () =>
    s.draw d f now media
    let _ ← d.present
  | .error _ => pure ()
  return (← { s with pending := false, frames := s.frames + 1, behind := late }.progress media, 0)

/-- Redraw now (pause, overlay changes). -/
def State.redraw (s : State) (d : Display) (f : Fonts) : BaseIO Unit := do
  if !s.pending then return
  let now ← Sys.monotonicMs
  match ← d.acquire with
  | .ok () => s.draw d f now (← s.media now); let _ ← d.present
  | .error _ => pure ()

/-- Restart both tracks at the fragments around `target` (ms). -/
def State.seekTo (s : State) (target : UInt64) : BaseIO State := do
  let dur := s.duration
  let target := if dur > 0 then min target dur else target
  let (k, startMs) := s.video.segmentAt (if dur > 0 then min target (dur - 1) else target)
  let video := s.video.restartAt k
  let dec ← match ← openVideo video with | .ok d => pure (some d) | .error _ => pure none
  let audio := s.audio.map fun a => a.restartAt (a.segmentAt startMs).1
  if let some out := s.out then out.reset
  let now ← Sys.monotonicMs
  let landing := max target startMs
  let s := { s with video, audio, dec, aac := s.aac.map Aac.Decoder.reset, written := 0, flushed := false,
                    pending := false, ended := false, overlayUntil := now + 3000, skipUntil := landing }
  Sys.log ("player: seek to " ++ fmtTime landing)
  return if s.paused then { s with pausedMedia := landing } else { s with ref := now + 300, base := landing }

/-- A remote key for the player. -/
def State.key (s : State) (a : Action) : BaseIO State := do
  let now ← Sys.monotonicMs
  let media ← s.media now
  match a with
  | .ok | .playPause =>
    if s.paused then
      Sys.log ("player: resumed at " ++ fmtTime s.pausedMedia)
      return { s with paused := false, ref := now, base := s.pausedMedia, written := 0, overlayUntil := now + 3000 }
    else
      if let some out := s.out then out.reset
      Sys.log ("player: paused at " ++ fmtTime media)
      return { s with paused := true, pausedMedia := media }
  | .left | .rewind => s.seekTo (if media > 10000 then media - 10000 else 0)
  | .right | .fastForward => s.seekTo (media + 10000)
  | .digit n => s.seekTo (s.duration * n.toUInt64 / 10)
  | _ => return { s with overlayUntil := now + 3000 }

def State.close (s : State) : BaseIO Unit := do
  if let some out := s.out then out.close

end Yt.App.Player
