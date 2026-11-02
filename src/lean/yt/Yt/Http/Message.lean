import Yt.Http.Url

/-! HTTP/1.1 messages without sockets: the app writes `request ...` to its
connection and feeds whatever bytes arrive to a `Parser`, a pure state
machine that accepts arbitrary chunk boundaries.

Requests are byte-identical to src/ytplay/src/http.rs's (`Content-Length`
when there is a body or the method is POST) except that they leave out its
`Connection: close`: connections are persistent (`Yt.Net.Fetch`). The response parser handles what http.rs handled
(status line, headers, `Content-Length`, `chunked`, close-delimited) and
also: interim 1xx responses are skipped (except 101), HEAD requests and
204/304 responses have no body, chunk extensions and trailers, bare-LF
line ends, and limits (64 KiB of head, 4 KiB per chunk-size or trailer
line). Malformed framing is an error rather than a guess: an unparsable
`Content-Length`, a bad chunk size, a chunk not followed by CRLF, or EOF
before the declared end of the body.

`Tunnel` handles an HTTP proxy's answer to `connectRequest` (http.rs's
`TV_HTTP_PROXY` path). It accepts any `HTTP/1.x 2xx` (http.rs required
exactly `HTTP/1.1 200 `). -/
namespace Yt.Http
open Yt.Fmt

/-- A request: `METHOD path HTTP/1.1`, `Host`, the given headers,
`Content-Length` if `body` is non-empty or the method is POST, then the
body. -/
def request (method : String) (u : Url) (headers : Array (String × String)) (body : ByteArray) :
    ByteArray := Id.run do
  let mut s := method ++ " " ++ u.path ++ " HTTP/1.1\r\nHost: " ++ u.hostHeader ++ "\r\n"
  for (k, v) in headers do
    s := s ++ k ++ ": " ++ v ++ "\r\n"
  if body.size > 0 || method == "POST" then
    s := s ++ "Content-Length: " ++ toString body.size ++ "\r\n"
  s := s ++ "\r\n"
  return s.toUTF8 ++ body

/-- `Range` header for bytes `start..=stop` (inclusive, as in initRange). -/
def rangeHeader (start stop : UInt64) : String × String :=
  ("Range", "bytes=" ++ dec start ++ "-" ++ dec stop)

/-- The CONNECT request http.rs sends to an HTTP proxy for `u`. -/
def connectRequest (u : Url) : ByteArray :=
  ("CONNECT " ++ u.authority ++ " HTTP/1.1\r\nHost: " ++ u.authority ++ "\r\n\r\n").toUTF8

/-- Status line and headers of a response. -/
structure Head where
  /-- `x` in `HTTP/1.x`. -/
  minor : Nat
  status : Nat
  reason : String
  /-- Header fields as received (name, value), values trimmed. -/
  headers : Array (String × String)
  deriving Inhabited, BEq

/-- The first header named `name` (ASCII case-insensitive), as http.rs. -/
def Head.header? (h : Head) (name : String) : Option String := Id.run do
  let n := name.toUTF8
  for (k, v) in h.headers do
    if eqIgnoreCase k.toUTF8 n then return some v
  return none

inductive Phase where
  | status | headers | length | chunkSize | chunkData | chunkEnd | trailers | close | done | failed
  deriving Inhabited, BEq

structure Parser where
  phase : Phase := .status
  /-- Bytes of an incomplete line (line phases), or bytes after the end of
  the message (`done`). -/
  buf : ByteArray := ByteArray.empty
  /-- Bytes of status line + headers seen so far (for the 64 KiB limit). -/
  headBytes : Nat := 0
  head : Option Head := none
  /-- Headers being collected. -/
  fields : Array (String × String) := #[]
  pending : Option (Nat × Nat × String) := none
  /-- Body bytes received and not yet taken (`takeBody`). -/
  body : ByteArray := ByteArray.empty
  /-- Total body bytes received. -/
  received : Nat := 0
  /-- Bytes left in a `Content-Length` body or in the current chunk. -/
  remaining : Nat := 0
  isHead : Bool := false
  error : String := ""
  deriving Inhabited

namespace Parser

def maxHead : Nat := 65536
def maxLine : Nat := 4096

/-- A parser for the response to a request; `isHead` for HEAD requests. -/
def new (isHead : Bool := false) : Parser := { isHead }

def fail (p : Parser) (msg : String) : Parser :=
  { p with phase := .failed, error := "HTTP: " ++ msg, buf := ByteArray.empty }

def isDone (p : Parser) : Bool := p.phase == .done
def error? (p : Parser) : Option String := if p.phase == .failed then some p.error else none
/-- Status line and headers, once complete. -/
def head? (p : Parser) : Option Head := p.head
/-- Bytes after the end of the response (pipelined data; normally empty). -/
def leftover (p : Parser) : ByteArray := if p.phase == .done then p.buf else ByteArray.empty

/-- Take the body bytes received so far (to stream a large body). -/
def takeBody (p : Parser) : ByteArray × Parser := (p.body, { p with body := ByteArray.empty })

@[inline] def isSpace (c : UInt8) : Bool := c == 32 || c == 9

/-- `b[i, j)` without leading/trailing spaces and tabs. -/
def trim (b : ByteArray) (i j : Nat) : ByteArray := Id.run do
  let mut i := i
  let mut j := j
  while i < j && isSpace (byteAt b i) do i := i + 1
  while j > i && isSpace (byteAt b (j - 1)) do j := j - 1
  return b.extract i j

def parseStatus (line : ByteArray) : Option (Nat × Nat × String) := Id.run do
  -- HTTP/1.x SP 3DIGIT [SP reason]
  if line.size < 12 || !startsWith line "HTTP/1.".toUTF8 then return none
  let m := byteAt line 7
  if m < 48 || m > 57 || byteAt line 8 != 32 then return none
  let d0 := byteAt line 9
  let d1 := byteAt line 10
  let d2 := byteAt line 11
  if !(isDigit d0 && isDigit d1 && isDigit d2) || d0 == 48 then return none
  if line.size > 12 && byteAt line 12 != 32 then return none
  let code := (d0 - 48).toNat * 100 + (d1 - 48).toNat * 10 + (d2 - 48).toNat
  return some ((m - 48).toNat, code, lossy (trim line 12 line.size))
where isDigit (c : UInt8) : Bool := 48 ≤ c && c ≤ 57

def parseDec (b : ByteArray) : Option Nat := Id.run do
  if b.size == 0 || b.size > 19 then return none
  let mut x := 0
  for c in b do
    if c < 48 || c > 57 then return none
    x := x * 10 + (c - 48).toNat
  return some x

/-- Chunk size: hex digits, then optional `;extensions`. -/
def parseChunkSize (line : ByteArray) : Option Nat := Id.run do
  let mut e := 0
  while e < line.size && byteAt line e != 59 do e := e + 1
  let h := trim line 0 e
  if h.size == 0 || h.size > 15 then return none
  let mut x := 0
  for c in h do
    let v := if 48 ≤ c && c ≤ 57 then c - 48 else if 97 ≤ c && c ≤ 102 then c - 87
      else if 65 ≤ c && c ≤ 70 then c - 55 else 255
    if v == 255 then return none
    x := x * 16 + v.toNat
  return some x

/-- Is the last coding of a Transfer-Encoding value `chunked` ("gzip, chunked")? -/
def lastCodingChunked (b : ByteArray) : Bool := Id.run do
  let mut k := b.size
  while k > 0 && byteAt b (k - 1) != 44 do k := k - 1
  return eqIgnoreCase (trim b k b.size) "chunked".toUTF8

/-- Headers complete: pick the body framing (RFC 9112 6.3). -/
def startBody (p : Parser) (h : Head) : Parser :=
  if 100 ≤ h.status && h.status < 200 && h.status != 101 then
    -- Interim response: the real one follows.
    { p with phase := .status, fields := #[], pending := none }
  else
    let p := { p with head := some h, fields := #[] }
    if p.isHead || h.status < 200 || h.status == 204 || h.status == 304 then { p with phase := .done }
    else
      let chunked := match h.header? "transfer-encoding" with
        | some te => lastCodingChunked te.toUTF8
        | none => false
      if chunked then { p with phase := .chunkSize }
      else match h.header? "content-length" with
        | some v => match parseDec (trim v.toUTF8 0 v.utf8ByteSize) with
          | some n => if n == 0 then { p with phase := .done } else { p with phase := .length, remaining := n }
          | none => p.fail ("bad Content-Length: " ++ v)
        | none => { p with phase := .close }

/-- Handle one complete line (without its CR LF) in a line phase. -/
def line (p : Parser) (l : ByteArray) : Parser :=
  match p.phase with
  | .status =>
    match parseStatus l with
    | some s => { p with phase := .headers, pending := some s }
    | none => p.fail ("bad status line: " ++ lossy (l.extract 0 80))
  | .headers =>
    if l.size == 0 then
      match p.pending with
      | some (minor, status, reason) => p.startBody { minor, status, reason, headers := p.fields }
      | none => p.fail "no status line"
    else Id.run do
      let mut c := 0
      while c < l.size && byteAt l c != 58 do c := c + 1
      -- Lines without ':' are ignored, as http.rs does.
      if c ≥ l.size then return p
      return { p with fields := p.fields.push (lossy (trim l 0 c), lossy (trim l (c + 1) l.size)) }
  | .chunkSize =>
    match parseChunkSize l with
    | some 0 => { p with phase := .trailers }
    | some n => { p with phase := .chunkData, remaining := n }
    | none => p.fail ("bad chunk size: " ++ lossy (l.extract 0 40))
  | .chunkEnd => if l.size == 0 then { p with phase := .chunkSize } else p.fail "chunk not followed by CRLF"
  | .trailers => if l.size == 0 then { p with phase := .done } else p
  | _ => p

@[inline] def isLinePhase (ph : Phase) : Bool :=
  ph == .status || ph == .headers || ph == .chunkSize || ph == .chunkEnd || ph == .trailers

/-- Feed `data[i, stop)`. -/
partial def feedFrom (p : Parser) (data : ByteArray) (i stop : Nat) : Parser :=
  if i ≥ stop then p else
  match p.phase with
  | .failed => p
  | .done => { p with buf := data.copySlice i p.buf p.buf.size (stop - i) false }
  | .close =>
    { p with body := data.copySlice i p.body p.body.size (stop - i) false, received := p.received + (stop - i) }
  | .length | .chunkData =>
    let n := min p.remaining (stop - i)
    let p := { p with body := data.copySlice i p.body p.body.size n false, received := p.received + n,
                      remaining := p.remaining - n }
    let p := if p.remaining == 0 then { p with phase := if p.phase == .length then .done else .chunkEnd } else p
    feedFrom p data (i + n) stop
  | ph =>
    -- A line phase: find the end of the line.
    let j := Id.run do
      let mut j := i
      while j < stop && byteAt data j != 10 do j := j + 1
      return j
    let inHead := ph == .status || ph == .headers
    let headBytes := if inHead then p.headBytes + (j - i) + 1 else p.headBytes
    if inHead && headBytes > maxHead then p.fail "response head larger than 64 KiB"
    else if !inHead && p.buf.size + (j - i) > maxLine then p.fail "chunk-size or trailer line too long"
    else if j ≥ stop then
      { p with buf := data.copySlice i p.buf p.buf.size (stop - i) false, headBytes := headBytes - 1 }
    else
      let l := if p.buf.size == 0 then data.extract i j else data.copySlice i p.buf p.buf.size (j - i) false
      let l := if l.size > 0 && byteAt l (l.size - 1) == 13 then l.extract 0 (l.size - 1) else l
      let p := line { p with buf := ByteArray.empty, headBytes } l
      feedFrom p data (j + 1) stop

/-- Feed the bytes `data[start, stop)` received from the connection. -/
@[inline] def feedSlice (p : Parser) (data : ByteArray) (start stop : Nat) : Parser :=
  feedFrom p data start (min stop data.size)

/-- Feed bytes received from the connection. -/
@[inline] def feed (p : Parser) (data : ByteArray) : Parser := feedFrom p data 0 data.size

/-- The connection was closed by the peer. -/
def finish (p : Parser) : Parser :=
  match p.phase with
  | .close => { p with phase := .done }
  -- After the last chunk, a missing trailer section is tolerated (http.rs
  -- stopped reading at the zero-size chunk).
  | .trailers => { p with phase := .done, buf := ByteArray.empty }
  | .done | .failed => p
  | .status => if p.buf.size == 0 && p.headBytes == 0 then p.fail "connection closed before a response"
      else p.fail "connection closed in the status line"
  | .headers => p.fail "connection closed in the headers"
  | .length => p.fail ("connection closed with " ++ toString p.remaining ++ " body bytes missing")
  | _ => p.fail "connection closed inside a chunked body"

end Parser

/-- A complete response. -/
structure Response where
  head : Head
  body : ByteArray
  deriving Inhabited

def Response.status (r : Response) : Nat := r.head.status
def Response.header? (r : Response) (name : String) : Option String := r.head.header? name

/-- Parse a whole response (everything received until the peer closed). -/
def parseResponse (bytes : ByteArray) (isHead : Bool := false) : Except String Response :=
  let p := ((Parser.new isHead).feed bytes).finish
  match p.error?, p.head? with
  | some e, _ => .error e
  | none, some h => .ok { head := h, body := p.body }
  | none, none => .error "HTTP: no response"

/-- The proxy's answer to `connectRequest`. -/
structure Tunnel where
  buf : ByteArray := ByteArray.empty
  deriving Inhabited

inductive TunnelResult where
  /-- Header not complete yet. -/
  | needMore (t : Tunnel)
  /-- 2xx: the connection is now a tunnel; `rest` are bytes that followed
  the proxy's header (normally none: the TLS server does not speak first). -/
  | established (rest : ByteArray)
  | failed (reason : String)
  deriving Inhabited

/-- Feed bytes from the proxy. The header may be at most 8192 bytes (http.rs). -/
def Tunnel.feed (t : Tunnel) (data : ByteArray) : TunnelResult := Id.run do
  let start := if t.buf.size ≥ 3 then t.buf.size - 3 else 0
  let b := t.buf ++ data
  -- Find CR LF CR LF.
  let mut i := start
  while i + 3 < b.size && !(byteAt b i == 13 && byteAt b (i+1) == 10 && byteAt b (i+2) == 13
      && byteAt b (i+3) == 10) do
    i := i + 1
  if i + 3 ≥ b.size then
    return if b.size ≥ 8192 then .failed "HTTP tunnel: proxy header too long" else .needMore ⟨b⟩
  if i + 4 > 8192 then return .failed "HTTP tunnel: proxy header too long"
  let mut e := 0
  while e < i && byteAt b e != 13 do e := e + 1
  match Parser.parseStatus (b.extract 0 e) with
  | some (_, status, reason) =>
    if 200 ≤ status && status < 300 then return .established (b.extract (i + 4) b.size)
    return .failed ("HTTP tunnel failed: " ++ toString status ++ " " ++ reason)
  | none => return .failed "HTTP tunnel failed: bad status line"

end Yt.Http
