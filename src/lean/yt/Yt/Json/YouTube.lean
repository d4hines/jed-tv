import Yt.Json.Value

/-! What the app needs from YouTube's InnerTube JSON (src/ytplay/src/yt.rs,
research/yt-stage0/PROTOCOL.md): the two request bodies and the fields of
/player and /search replies, with yt.rs's exact rules (`as_u64` numbers,
`str::parse::<u64>` for numeric strings, `max_by_key` keeping the last
maximum). Works on a `Doc`, so only the extracted strings are decoded.

One deliberate difference: yt.rs's serde_json `Value` keeps object keys
sorted, so its recursive search walk visits sibling keys alphabetically;
`parseSearch` walks in document order (Python's dict order), which is the
order YouTube lists results in. -/
namespace Yt.Json.YouTube
open Yt.Json Yt.Fmt

def userAgent : String :=
  "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15"

/-- Body of POST /youtubei/v1/player (VISIONOS client), byte-identical to
yt.rs's `player_body` (serde_json: sorted keys, compact). -/
def playerBody (video : String) (visitor : Option String) : ByteArray :=
  let client : Array (String × Json) := #[
    ("clientName", .str "VISIONOS"), ("clientVersion", .str "1.02"),
    ("deviceMake", .str "Apple"), ("deviceModel", .str "RealityDevice17,1"),
    ("hl", .str "en"), ("osName", .str "visionOS"), ("osVersion", .str "26.5.23O471"),
    ("userAgent", .str userAgent)]
  let client := match visitor with
    | some v => client.push ("visitorData", .str v)
    | none => client
  Json.render (.obj #[("contentCheckOk", .bool true), ("context", .obj #[("client", .obj client)]),
                      ("racyCheckOk", .bool true), ("videoId", .str video)])

/-- Body of POST /youtubei/v1/search (WEB client), as yt.rs's `search`. -/
def searchBody (query : String) : ByteArray :=
  Json.render (.obj #[("context", .obj #[("client", .obj #[
      ("clientName", .str "WEB"), ("clientVersion", .str "2.20260708.00.00"),
      ("gl", .str "US"), ("hl", .str "en")])]), ("query", .str query)])

/-- Rust's `str::parse::<u64>`: optional `+`, then only decimal digits, no overflow. -/
def parseU64 (b : ByteArray) : Option UInt64 := Id.run do
  let s := if byteAt b 0 == 43 then 1 else 0
  if s ≥ b.size then return none
  let mut x : Nat := 0
  for i in [s:b.size] do
    let c := byteAt b i
    if !isDigit c then return none
    x := x * 10 + (c - 48).toNat
    if x ≥ 18446744073709551616 then return none
  return some x.toUInt64

structure Format where
  itag : UInt32
  url : String
  mime : String
  width : UInt32
  height : UInt32
  fps : UInt32
  bitrate : UInt64
  contentLength : UInt64
  /-- Inclusive byte ranges (start, end) of the init (`moov`) and index (`sidx`). -/
  initRange : UInt64 × UInt64
  indexRange : UInt64 × UInt64
  deriving Inhabited

structure Player where
  /-- `playabilityStatus.status` ("" if absent): "OK", "LOGIN_REQUIRED", ... -/
  status : String
  /-- `playabilityStatus.reason` ("" if absent). -/
  reason : String
  /-- `responseContext.visitorData`: send it back in the second request. -/
  visitorData : Option String
  title : String
  lengthS : UInt64
  /-- `streamingData.adaptiveFormats` entries that have an `itag` and a direct `url`. -/
  formats : Array Format
  deriving Inhabited

def Player.playable (p : Player) : Bool := p.status == "OK"

section
variable (d : Doc)

def strAt (v : Option Val) : Option String := v.bind d.str?
def u64At (v : Option Val) : Option UInt64 := v.bind d.u64?
/-- A numeric string field parsed like Rust's `s.parse::<u64>()`, or 0. -/
def u64StrAt (v : Option Val) : UInt64 := ((v.bind d.bytes?).bind parseU64).getD 0
def range (v : Option Val) : UInt64 × UInt64 :=
  match v with
  | some r => (u64StrAt d (d.get? r "start"), u64StrAt d (d.get? r "end"))
  | none => (0, 0)

def format? (f : Val) : Option Format := do
  let itag ← u64At d (d.get? f "itag")
  let url ← strAt d (d.get? f "url")
  let u := fun k => ((u64At d (d.get? f k)).getD 0).toUInt32
  pure { itag := itag.toUInt32, url, mime := (strAt d (d.get? f "mimeType")).getD "",
         width := u "width", height := u "height", fps := u "fps",
         bitrate := (u64At d (d.get? f "bitrate")).getD 0,
         contentLength := u64StrAt d (d.get? f "contentLength"),
         initRange := range d (d.get? f "initRange"), indexRange := range d (d.get? f "indexRange") }

/-- The fields of a parsed /player reply. -/
def playerOf : Player :=
  let r := d.root
  let ps := d.get? r "playabilityStatus"
  let fmts := match d.path? r #["streamingData", "adaptiveFormats"] with
    | some a => (d.elems a).foldl (fun acc f => match format? d f with
        | some x => acc.push x
        | none => acc) #[]
    | none => #[]
  let vd := d.get? r "videoDetails"
  { status := (strAt d (ps.bind (d.get? · "status"))).getD "",
    reason := (strAt d (ps.bind (d.get? · "reason"))).getD "",
    visitorData := strAt d (d.path? r #["responseContext", "visitorData"]),
    title := (strAt d (vd.bind (d.get? · "title"))).getD "",
    lengthS := u64StrAt d (vd.bind (d.get? · "lengthSeconds")),
    formats := fmts }
end

/-- Parse a /player reply body. -/
def parsePlayer (body : ByteArray) : Except String Player := do
  pure (playerOf (← Doc.parse body))

/-- yt.rs's `pick_video`: H.264 in MP4, width ≤ 1280, height ≤ `maxHeight`,
fps ≤ 30 (all > 0); the highest (height, bitrate), last one on ties. -/
def pickVideo (fs : Array Format) (maxHeight : UInt32) : Option Format := Id.run do
  let mut best : Option Format := none
  for f in fs do
    if startsWith f.mime.toUTF8 "video/mp4".toUTF8 && contains f.mime.toUTF8 "avc1".toUTF8
        && f.width > 0 && f.width ≤ 1280 && f.height > 0 && f.height ≤ maxHeight
        && f.fps > 0 && f.fps ≤ 30 then
      match best with
      | some b => if f.height > b.height || (f.height == b.height && f.bitrate ≥ b.bitrate) then
          best := some f
      | none => best := some f
  return best

/-- Why the app cannot play a /player reply, or `none` if it can: YouTube's
own verdict for this client, an H.264 stream it can decode (`pickVideo`),
and the stream's `sidx` index, which the player seeks with (live streams
have none). -/
def unplayableReason (p : Player) : Option String :=
  if !p.playable then some (p.status ++ " " ++ p.reason)
  else match pickVideo p.formats 720 with
    | none => some "no H.264 stream up to 720p30"
    | some v => if v.indexRange.2 == 0 then some "no seekable stream (live?)" else none

/-- yt.rs's `pick_audio`: AAC in MP4 (itag 140 in practice), highest bitrate,
last one on ties. -/
def pickAudio (fs : Array Format) : Option Format := Id.run do
  let mut best : Option Format := none
  for f in fs do
    if startsWith f.mime.toUTF8 "audio/mp4".toUTF8 && contains f.mime.toUTF8 "mp4a".toUTF8 then
      match best with
      | some b => if f.bitrate ≥ b.bitrate then best := some f
      | none => best := some f
  return best

/-- One search result. -/
structure SearchItem where
  id : String
  title : String
  channel : String
  length : String
  deriving Inhabited

/-- YouTube's fixed-URL 320x180 thumbnail (16:9, no letterbox bars). -/
def SearchItem.thumbUrl (i : SearchItem) : String :=
  "https://i.ytimg.com/vi/" ++ i.id ++ "/mqdefault.jpg"

/-- yt.rs's `text_of`: `simpleText`, else the concatenated `runs[].text`. -/
def textOf (d : Doc) (v : Option Val) : String :=
  match v with
  | none => ""
  | some v =>
    match strAt d (d.get? v "simpleText") with
    | some s => s
    | none =>
      match d.get? v "runs" with
      | some r =>
        if d.kind r != .arr then "" else
        (d.elems r).foldl (fun s x => s ++ (strAt d (d.get? x "text")).getD "") ""
      | none => ""

mutual
/-- yt.rs's `collect_videos`, in document order: at each object, its
`videoRenderer` first, then every member value. -/
partial def collect (d : Doc) (v : Val) (out : Array SearchItem) : Array SearchItem :=
  let k := d.kind v
  if k == .obj then
    let out := match d.get? v "videoRenderer" with
      | some r => match strAt d (d.get? r "videoId") with
        | some id => out.push { id, title := textOf d (d.get? r "title"),
                                channel := textOf d (d.get? r "ownerText"),
                                length := textOf d (d.get? r "lengthText") }
        | none => out
      | none => out
    collectChildren d true (d.aux v) (v.i + 1) out
  else if k == .arr then collectChildren d false (d.aux v) (v.i + 1) out
  else out

/-- `collect` over the members (values) or elements from tape index `i`. -/
partial def collectChildren (d : Doc) (isObj : Bool) (stop i : Nat) (out : Array SearchItem) :
    Array SearchItem :=
  if i ≥ stop then out else
  let v : Val := if isObj then ⟨i + 1⟩ else ⟨i⟩
  collectChildren d isObj stop (d.next v).i (collect d v out)
end

/-- The videos of a parsed /search reply: live streams (no length) dropped,
then duplicates (by id) dropped, as yt.rs's `search`. -/
def searchOf (d : Doc) : Array SearchItem := Id.run do
  let all := collect d d.root #[]
  let mut out : Array SearchItem := #[]
  for i in all do
    if i.length.isEmpty then continue
    if out.any (·.id == i.id) then continue
    out := out.push i
  return out

/-- Parse a /search reply body. -/
def parseSearch (body : ByteArray) : Except String (Array SearchItem) := do
  pure (searchOf (← Doc.parse body))

end Yt.Json.YouTube
