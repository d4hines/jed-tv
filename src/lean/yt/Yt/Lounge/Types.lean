import Yt.Lounge.Form

/-! Public types of the Lounge screen (see Session.lean for the protocol). -/
namespace Yt.Lounge

/-- Who this screen is. Persist `screenId` (once learned, `Session.identity`)
and `deviceId` across launches, or every paired phone has to re-link. -/
structure Identity where
  /-- From `generate_screen_id`; none on a fresh install. -/
  screenId : Option String := none
  /-- A random UUID (`uuidFromBytes`), fixed for this install. Use the same
  UUID as the DIAL UDN when PID 1 runs DIAL. -/
  deviceId : String
  /-- Shown on the phone ("Link with TV code" list, cast menu). -/
  name : String
  deriving Inhabited, BEq

/-- Settings that rarely change. Defaults are what the host prototype used
against live YouTube on 2026-11-01. -/
structure Config where
  /-- The screen app label. `lb-v4` (official TV app's label) is what
  SmartTube sends; yt-cast-receiver sends `ytcr`. -/
  app : String := "lb-v4"
  theme : String := "cl"
  capabilities : String := "dsp,mic,dpa"
  userAgent : String := "dande-tv-lounge/0"
  brand : String := "TCL"
  model : String := "32Q3K"
  /-- Send `deviceInfo`, `cst` and `obfuscatedGaiaId` on bind, with
  `dialAdditionalDataSupportLevel` "supported" (needed only when DIAL serves
  the full additionalData set; see research/dial-lounge/README.md). -/
  deviceInfo : Bool := false
  /-- Fetch a 12-digit "Link with TV code" while no phone is connected. -/
  showCode : Bool := true
  /-- Refresh the code this often while unpaired (codes expire). -/
  codeRefreshMs : UInt64 := 300000
  /-- Treat a long poll without any bytes for this long as dead (the server
  sends `noop` about every 30 s). -/
  pollStallMs : UInt64 := 75000
  deriving Inhabited

/-- What a request is for (the session matches responses by request id). -/
inductive Tag where
  | generateScreenId | tokenBatch | bindOpen | pairingCode | registerCode | poll | send | terminate
  deriving Inhabited, BEq

def Tag.name : Tag → String
  | .generateScreenId => "generateScreenId" | .tokenBatch => "tokenBatch" | .bindOpen => "bindOpen"
  | .pairingCode => "pairingCode" | .registerCode => "registerCode" | .poll => "poll"
  | .send => "send" | .terminate => "terminate"

/-- An HTTPS request for the app's fetch layer. One request per connection
is fine. For `streaming` requests (the long poll), hand the body bytes to
`Session.onChunk` as they arrive and call `Session.onPollEnd` when the
response ends or fails. At most one poll is current: when the session emits
a new `.poll`, close any previous poll connection (its id is no longer
current and its bytes would be ignored anyway). -/
structure Request where
  id : Nat
  tag : Tag
  method : String
  /-- Absolute `https://www.youtube.com/...` URL with the query. -/
  url : String
  /-- Extra headers (`Http.request` adds Host, Connection, Content-Length). -/
  headers : Array (String × String)
  body : ByteArray
  streaming : Bool
  deriving Inhabited

/-- YouTube player state numbers, as real screens report them. -/
inductive PlayState where
  | unstarted | ended | playing | paused | buffering
  deriving Inhabited, BEq

def PlayState.code : PlayState → String
  | .unstarted => "-1" | .ended => "0" | .playing => "1" | .paused => "2" | .buffering => "3"

/-- The app's player, for reporting to phones. -/
structure PlayerState where
  videoId : Option String := none
  state : PlayState := .unstarted
  currentTimeMs : UInt64 := 0
  /-- 0 if not known yet (then duration fields are omitted). -/
  durationMs : UInt64 := 0
  /-- Buffered up to (0: report the duration, as the reference screens do). -/
  loadedMs : UInt64 := 0
  /-- 0..100. -/
  volume : Nat := 100
  muted : Bool := false
  deriving Inhabited, BEq

/-- What the phone asks the app to do. `next`/`previous` arrive as
`playVideo` of the neighbouring queue item (the queue lives in the
session). -/
inductive Command where
  | playVideo (videoId : String) (startMs : UInt64) (listId : Option String)
  | play
  | pause
  | seek (ms : UInt64)
  | stop
  | setVolume (volume : Nat) (muted : Bool)
  | remoteConnected (name : String)
  | remoteDisconnected (name : String)
  /-- `dpadCommand` (key name as sent, e.g. "UP", "ENTER"). -/
  | dpad (key : String)
  deriving Inhabited, BEq

def Command.describe : Command → String
  | .playVideo v t l => "playVideo " ++ v ++ " from " ++ secondsStr t ++ "s" ++
      (match l with | some x => " list " ++ x | none => "")
  | .play => "play" | .pause => "pause" | .seek t => "seek " ++ secondsStr t ++ "s" | .stop => "stop"
  | .setVolume v m => "setVolume " ++ natStr v ++ (if m then " muted" else "")
  | .remoteConnected n => "remoteConnected " ++ n
  | .remoteDisconnected n => "remoteDisconnected " ++ n
  | .dpad k => "dpad " ++ k

end Yt.Lounge
