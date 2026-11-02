import Yt.Net.Conn
import Yt.Http.Message
import Yt.Http.Url
import Yt.Tls.Client

/-! HTTP(S) requests over `Conn`, several at once, driven by the event loop:
connect (directly to an IPv4 address, or to the CONNECT proxy) -> the
proxy's tunnel -> TLS 1.3 (`Yt.Tls`, end to end inside the app) -> an
HTTP/1.1 request and its response (`Yt.Http.Parser`). The app has no DNS
resolver: names are resolved by the proxy (TV_HTTP_PROXY on the TV).

Connections are persistent: after a response that ends by its framing
(`Content-Length` or chunked, HTTP/1.1, no `Connection: close`) the
connection, with its tunnel and TLS session, waits in `idle` for the next
request to the same origin. A TLS handshake costs the TV's CPU a fifth of a
second or more, and search checks, thumbnails and media fragments each go
to the same few hosts. A reused connection that fails before any response
byte (the server closed it while idle) is retried once on a new one. -/
namespace Yt.Net
open Yt.Platform Yt.Http Yt.Fmt

abbrev Ip := UInt8 × UInt8 × UInt8 × UInt8

inductive Phase where
  | connecting | tunnel | tls | http
  deriving BEq, Inhabited

/-- One request in flight. -/
structure Fetch where
  id : Nat
  url : Url
  request : ByteArray
  conn : Conn
  viaProxy : Bool
  phase : Phase := .connecting
  tunnel : Tunnel := {}
  tls : Option Tls.Client := none
  sent : Bool := false
  parser : Parser := Parser.new
  /-- Deliver body bytes as they arrive (long polls) instead of only at the end. -/
  streaming : Bool := false
  /-- The connection came from the idle pool. -/
  reused : Bool := false
  deriving Inhabited

/-- An open connection between requests. -/
structure Idle where
  origin : String
  conn : Conn
  tls : Option Tls.Client
  sinceMs : UInt64
  deriving Inhabited

/-- Idle connections live this long (servers close theirs after a while). -/
def idleMs : UInt64 := 20000
/-- At most this many idle connections are kept. -/
def maxIdle : Nat := 8

/-- What makes connections interchangeable: scheme, host and port. -/
def _root_.Yt.Http.Url.origin (u : Url) : String := (if u.tls then "https://" else "http://") ++ u.authority

/-- A finished request: the response, or why it failed. -/
abbrev Result := Except String Response

structure Fetcher where
  proxy : Option (Ip × UInt16) := none
  fetches : Array Fetch := #[]
  nextId : Nat := 1
  /-- TLS: certificate signatures already verified (shared by connections). -/
  sigCache : Array ByteArray := #[]
  /-- Persistent connections waiting for a request, oldest first. -/
  idle : Array Idle := #[]
  /-- Connections opened and requests sent on a reused one (diagnostics). -/
  opened : Nat := 0
  reuses : Nat := 0

/-- `host:port` with an IPv4 host. -/
def parseHostPort (s : String) : Option (Ip × UInt16) := do
  let b := s.toUTF8
  let mut colon := b.size
  for i in [0:b.size] do
    if b[i]! == 58 then colon := i
  guard (colon < b.size)
  let ip ← parseIPv4 (← String.fromUTF8? (b.extract 0 colon))
  let mut port := 0
  for i in [colon + 1:b.size] do
    let c := b[i]!
    guard (c ≥ 48 && c ≤ 57)
    port := port * 10 + (c - 48).toNat
  guard (port > 0 && port < 65536)
  return (ip, port.toUInt16)

/-- A request `id` on a new connection: directly to an IPv4 literal,
otherwise through the proxy (which resolves names). -/
def Fetcher.connect (f : Fetcher) (id : Nat) (u : Url) (req : ByteArray) (streaming : Bool) :
    BaseIO (Except String Fetch) := do
  let target : Option (Ip × UInt16 × Bool) :=
    match parseIPv4 u.host, f.proxy with
    | some ip, _ => some (ip, u.port, false)
    | none, some (pip, pport) => some (pip, pport, true)
    | none, none => none
  let some (ip, port, viaProxy) := target
    | return .error ("no proxy to resolve " ++ u.host ++ " (set TV_HTTP_PROXY)")
  match ← Conn.open ip port with
  | .error e => return .error ("connect failed, errno " ++ toString e)
  | .ok conn => return .ok { id, url := u, request := req, conn, viaProxy, streaming }

/-- Send the HTTP request (through TLS when there is a session). -/
def Fetch.sendRequest (fe : Fetch) : Fetch :=
  match fe.tls with
  | some c =>
    let (c, rec) := Tls.send c fe.request
    { fe with tls := some c, conn := fe.conn.send rec, sent := true, phase := .http }
  | none => { fe with conn := fe.conn.send fe.request, sent := true, phase := .http }

/-- Start a request; its id, or why it cannot start. It goes out at once on
an idle connection to the same origin if there is one. -/
def Fetcher.start (f : Fetcher) (method : String) (url : String) (headers : Array (String × String))
    (body : ByteArray := .empty) (streaming : Bool := false) : BaseIO (Fetcher × Except String Nat) := do
  let u ← match Url.parse url with
    | .ok u => pure u
    | .error e => return (f, .error e)
  let id := f.nextId
  let req := request method u headers body
  let origin := u.origin
  -- the most recently parked connection is the least likely to be closed
  let mut found : Option Nat := none
  for i in [0:f.idle.size] do
    if f.idle[i]!.origin == origin then found := some i
  if let some i := found then
    let c := f.idle[i]!
    let idle := f.idle.extract 0 i ++ f.idle.extract (i + 1) f.idle.size
    let fe : Fetch := { id, url := u, request := req, conn := c.conn, viaProxy := false, streaming,
                        tls := c.tls, phase := .http, reused := true }
    return ({ f with idle, fetches := f.fetches.push fe.sendRequest, nextId := id + 1, reuses := f.reuses + 1 }, .ok id)
  match ← f.connect id u req streaming with
  | .error e => return (f, .error e)
  | .ok fe => return ({ f with fetches := f.fetches.push fe, nextId := id + 1, opened := f.opened + 1 }, .ok id)

/-- Scan comma-separated Connection tokens with only HTTP spaces/tabs removed.
Use byte operations so this path runs on the app's owned Lean runtime. -/
private def hasCloseToken (value : ByteArray) : Bool := Id.run do
  let mut start := 0
  for stop in [0:value.size + 1] do
    if stop == value.size || value[stop]! == 44 then
      if eqIgnoreCase (Parser.trim value start stop) "close".toUTF8 then return true
      start := stop + 1
  return false

/-- The response ended by its framing on a healthy HTTP/1.1 connection that
the server keeps open: the connection can carry the next request. -/
def Fetch.reusable (fe : Fetch) : Bool :=
  !fe.streaming && fe.parser.isDone && fe.parser.leftover.size == 0 && !fe.conn.eof &&
    fe.conn.state == .connected && fe.conn.inbox.size == 0 &&
    (match fe.url.tls, fe.tls with
      | false, none => true
      | true, some c => c.phase == .connected && !c.sentClose
      | _, _ => false) &&
    match fe.parser.head? with
    | some h => h.minor ≥ 1 && !h.headers.any (fun (name, value) =>
        eqIgnoreCase name.toUTF8 "Connection".toUTF8 && hasCloseToken value.toUTF8)
    | none => false

/-- Nothing of a response arrived; a reused connection may have been closed
by the server while idle. This alone does not prove whether it ran the request. -/
def Fetch.noResponse (fe : Fetch) : Bool :=
  fe.parser.head?.isNone && fe.parser.headBytes == 0 && fe.parser.buf.size == 0

/-- TLS close_notify ends the response stream even if TCP stays open. -/
def Fetch.streamEnded (fe : Fetch) : Bool :=
  fe.conn.eof || (match fe.tls with | some c => c.phase == .closed | none => false)

/-- The connection to the server is up (directly or tunnelled): start TLS or
send the request. -/
def Fetch.onTransport (fe : Fetch) (sigCache : Array ByteArray) : BaseIO Fetch := do
  if fe.url.tls then
    let entropy ← match ← Sys.getrandom 32 with
      | .ok e => pure e
      | .error _ => pure .empty
    let now := (← Sys.clockNs 0) / 1000000000
    let (c, hello) := Tls.start { serverName := fe.url.host, unixTime := now, entropy, sigCache }
    return { fe with tls := some c, conn := fe.conn.send hello, phase := .tls }
  else
    return fe.sendRequest

/-- Feed decrypted (or plain) response bytes to the HTTP parser. -/
def Fetch.onData (fe : Fetch) (data : ByteArray) : Fetch :=
  if data.size == 0 then fe else { fe with parser := fe.parser.feed data }

/-- Advance with what arrived; `some result` when finished. -/
def Fetch.advance (fe : Fetch) (sigCache : Array ByteArray) : BaseIO (Fetch × Option Result) := do
  let mut fe := fe
  match fe.conn.state with
  | .failed e => return (fe, some (.error ("connection failed, errno " ++ toString e)))
  | .connecting => return (fe, none)
  | _ => pure ()
  if fe.phase == .connecting then
    fe ← if fe.viaProxy then pure { fe with conn := fe.conn.send (connectRequest fe.url), phase := .tunnel }
         else fe.onTransport sigCache
  let (conn, input) := fe.conn.take fe.conn.inbox.size
  fe := { fe with conn }
  let mut input := input
  if fe.phase == .tunnel && input.size > 0 then
    match fe.tunnel.feed input with
    | .needMore t => fe := { fe with tunnel := t }; input := .empty
    | .failed r => return (fe, some (.error ("proxy: " ++ r)))
    | .established rest => fe ← fe.onTransport sigCache; input := rest
  if fe.phase == .tls || (fe.phase == .http && fe.tls.isSome) then
    if let some c := fe.tls then
      if input.size > 0 then
        let o := Tls.feed c input
        fe := { fe with tls := some o.client, conn := fe.conn.send o.send }
        match o.status with
        | .failed r => return (fe, some (.error ("TLS: " ++ r)))
        | .connected => if !fe.sent then fe := fe.sendRequest
        | _ => pure ()
        fe := fe.onData o.data
  else if fe.phase == .http then
    fe := fe.onData input
  -- close_notify ends the TLS byte stream even when TCP stays open. Feed any
  -- final response bytes first; an incomplete response then fails like EOF.
  if fe.streamEnded then fe := { fe with conn := { fe.conn with eof := true } }
  if fe.parser.isDone then
    match fe.parser.head? with
    | some h => return (fe, some (.ok { head := h, body := fe.parser.body }))
    | none => return (fe, some (.error "HTTP: no response head"))
  if let some e := fe.parser.error? then return (fe, some (.error ("HTTP: " ++ e)))
  if fe.conn.eof then
    if fe.phase != .http then return (fe, some (.error "connection closed early"))
    let p := fe.parser.finish
    match p.error?, p.head? with
    | some e, _ => return (fe, some (.error ("HTTP: " ++ e)))
    | none, some h => return (fe, some (.ok { head := h, body := p.body }))
    | none, none => return (fe, some (.error "HTTP: no response"))
  return (fe, none)

/-- Wait up to `timeoutMs` for network activity (this is also the event
loop's idle wait); the requests that finished, and body bytes of streaming
requests that arrived (a streaming request's final result carries only the
bytes not delivered as chunks). -/
def Fetcher.step (f : Fetcher) (timeoutMs : Nat) :
    BaseIO (Fetcher × Array (Nat × Result) × Array (Nat × ByteArray)) := do
  let now := (← Sys.clockNs 1) / 1000000
  let mut idle : Array Idle := #[]
  for c in f.idle do
    if now - c.sinceMs < idleMs then idle := idle.push c else let _ ← c.conn.close
  if f.fetches.isEmpty then
    let _ ← Sys.poll .empty timeoutMs.toUSize
    return ({ f with idle }, #[], #[])
  let conns ← pollAll (f.fetches.map (·.conn)) timeoutMs
  let mut keep : Array Fetch := #[]
  let mut done : Array (Nat × Result) := #[]
  let mut chunks : Array (Nat × ByteArray) := #[]
  let mut cache := f.sigCache
  let mut opened := f.opened
  for i in [0:f.fetches.size] do
    let fe := { f.fetches[i]! with conn := conns[i]! }
    let (fe0, r) ← fe.advance cache
    let mut fe := fe0
    -- streaming: hand over the body bytes parsed so far
    if fe.streaming && r.isNone then
      let (b, p) := fe.parser.takeBody
      if b.size > 0 then chunks := chunks.push (fe.id, b)
      fe := { fe with parser := p }
    -- push queued bytes out now rather than waiting for the next poll
    if fe.conn.state == .connected then fe := { fe with conn := ← fe.conn.flush }
    if let some c := fe.tls then
      if c.sigCache.size > cache.size then cache := c.sigCache
    match r with
    | some res =>
      match res with
      | .ok _ =>
        if fe.reusable then idle := idle.push { origin := fe.url.origin, conn := fe.conn, tls := fe.tls, sinceMs := now }
        else let _ ← fe.conn.close
        done := done.push (fe.id, res)
      | .error e =>
        let _ ← fe.conn.close
        if fe.reused && fe.noResponse then
          match ← f.connect fe.id fe.url fe.request fe.streaming with
          | .ok fresh => keep := keep.push fresh; opened := opened + 1
          | .error e2 => done := done.push (fe.id, .error e2)
        else done := done.push (fe.id, .error e)
    | none => keep := keep.push fe
  while idle.size > maxIdle do
    let _ ← idle[0]!.conn.close
    idle := idle.extract 1 idle.size
  return ({ f with fetches := keep, sigCache := cache, idle, opened }, done, chunks)

def Fetcher.stats (f : Fetcher) : String :=
  toString f.opened ++ " connections opened, " ++ toString f.reuses ++ " requests on a kept one"

/-- Abandon a request. -/
def Fetcher.cancel (f : Fetcher) (id : Nat) : BaseIO Fetcher := do
  let mut keep : Array Fetch := #[]
  for fe in f.fetches do
    if fe.id == id then let _ ← fe.conn.close else keep := keep.push fe
  return { f with fetches := keep }

end Yt.Net
