import Yt.Http.Message
import Yt.Json.YouTube
import HttpTest.Fixtures

/-! The HTTP library: URL parsing, requests byte-identical to the ones the
Rust client sent (recorded by capture.py), and the response parser fed the
recorded responses split at every boundary (and byte by byte), synthetic
responses split at every pair of boundaries, error cases and the CONNECT
tunnel. -/
namespace HttpTest
open Yt.Http Yt.Fmt

def n (x : Nat) : String := toString x
def b64 (s : String) : ByteArray := (base64Decode s.toUTF8).getD ByteArray.empty
def ok (b : Bool) : String := if b then "ok" else "FAIL"

/-! ## URLs -/

def urlText : Except String Url → String
  | .ok u => "tls=" ++ (if u.tls then "1" else "0") ++ " host=" ++ u.host ++ " port=" ++ n u.port.toNat
      ++ " path=" ++ u.path ++ " hostHeader=" ++ u.hostHeader
  | .error _ => "error"

def urlCases : Array (String × String) := #[
  ("https://www.youtube.com/youtubei/v1/player?prettyPrint=false",
   "tls=1 host=www.youtube.com port=443 path=/youtubei/v1/player?prettyPrint=false hostHeader=www.youtube.com"),
  ("http://127.0.0.1:8090/youtubei/v1/search?prettyPrint=false",
   "tls=0 host=127.0.0.1 port=8090 path=/youtubei/v1/search?prettyPrint=false hostHeader=127.0.0.1:8090"),
  ("https://i.ytimg.com/vi/aqz-KE-bpKQ/mqdefault.jpg",
   "tls=1 host=i.ytimg.com port=443 path=/vi/aqz-KE-bpKQ/mqdefault.jpg hostHeader=i.ytimg.com"),
  ("http://example.com", "tls=0 host=example.com port=80 path=/ hostHeader=example.com"),
  ("http://example.com:80/", "tls=0 host=example.com port=80 path=/ hostHeader=example.com"),
  ("https://example.com:80/", "tls=1 host=example.com port=80 path=/ hostHeader=example.com:80"),
  ("HTTPS://Example.com/A?b=C", "tls=1 host=Example.com port=443 path=/A?b=C hostHeader=Example.com"),
  ("https://h?q=1", "tls=1 host=h port=443 path=/?q=1 hostHeader=h"),
  ("https://h/p#frag", "tls=1 host=h port=443 path=/p hostHeader=h"),
  ("https://h#frag", "tls=1 host=h port=443 path=/ hostHeader=h"),
  ("http://[::1]:8090/x", "tls=0 host=::1 port=8090 path=/x hostHeader=[::1]:8090"),
  ("http://[2001:db8::2]/", "tls=0 host=2001:db8::2 port=80 path=/ hostHeader=[2001:db8::2]"),
  ("http://h:65535/", "tls=0 host=h port=65535 path=/ hostHeader=h:65535"),
  ("http://h:65536/", "error"), ("http://h:/", "error"), ("http://h:8x/", "error"),
  ("ftp://h/", "error"), ("https://", "error"), ("http:///p", "error"), ("http://u@h/", "error"),
  ("http://h/a b", "error"), ("http://[::1/", "error"), ("www.youtube.com", "error")]

def urls : String := Id.run do
  let mut good := 0
  let mut o := ""
  for (u, want) in urlCases do
    let got := urlText (Url.parse u)
    if got == want then good := good + 1
    else o := o ++ "  " ++ u ++ "\n    want " ++ want ++ "\n    got  " ++ got ++ "\n"
  return "http urls: " ++ n good ++ " of " ++ n urlCases.size ++ " as expected\n" ++ o

/-! ## Requests -/

/-- `b` without its `Connection: close` header line: the recorded requests
are the Rust client's, which closed every connection; ours are persistent. -/
def dropClose (b : ByteArray) : ByteArray := Id.run do
  let line := "Connection: close\r\n".toUTF8
  for i in [0:b.size] do
    if i + line.size ≤ b.size && b.extract i (i + line.size) == line then
      return b.extract 0 i ++ b.extract (i + line.size) b.size
  return b

def requests : String := Id.run do
  let ua := Yt.Json.YouTube.userAgent
  let search := match Url.parse "https://www.youtube.com/youtubei/v1/search?prettyPrint=false" with
    | .ok u => request "POST" u #[("Content-Type", "application/json"), ("User-Agent", ua)]
        (Yt.Json.YouTube.searchBody "big buck bunny")
    | .error _ => ByteArray.empty
  let item : Yt.Json.YouTube.SearchItem := { id := "PVGeM40dABA", title := "", channel := "", length := "" }
  let thumb := match Url.parse item.thumbUrl with
    | .ok u => request "GET" u #[] ByteArray.empty
    | .error _ => ByteArray.empty
  let loc := match Url.parse "http://127.0.0.1:8090/videoplayback?id=1" with
    | .ok u => request "GET" u #[rangeHeader 0 65535] ByteArray.empty
    | .error _ => ByteArray.empty
  let post0 := match Url.parse "http://h/p" with
    | .ok u => request "POST" u #[] ByteArray.empty
    | .error _ => ByteArray.empty
  let conn := match Url.parse "https://rr4---sn-o097znzd.googlevideo.com/videoplayback?x=1" with
    | .ok u => connectRequest u
    | .error _ => ByteArray.empty
  let conn6 := match Url.parse "https://[2001:db8::2]:8443/" with
    | .ok u => connectRequest u
    | .error _ => ByteArray.empty
  "http request: search POST " ++ ok (search == dropClose (b64 Fixtures.searchRequest))
    ++ " (" ++ n search.size ++ " bytes, identical to the recorded Rust-format request incl. the JSON body, but persistent)\n"
    ++ "http request: thumbnail GET " ++ ok (thumb == dropClose (b64 Fixtures.thumbRequest)) ++ "\n"
    ++ "http request: non-default port + Range " ++ ok (str loc ==
      "GET /videoplayback?id=1 HTTP/1.1\r\nHost: 127.0.0.1:8090\r\nRange: bytes=0-65535\r\n\r\n") ++ "\n"
    ++ "http request: empty POST has Content-Length: 0 " ++ ok (str post0 ==
      "POST /p HTTP/1.1\r\nHost: h\r\nContent-Length: 0\r\n\r\n") ++ "\n"
    ++ "http request: CONNECT " ++ ok (str conn ==
      "CONNECT rr4---sn-o097znzd.googlevideo.com:443 HTTP/1.1\r\nHost: rr4---sn-o097znzd.googlevideo.com:443\r\n\r\n"
      && str conn6 == "CONNECT [2001:db8::2]:8443 HTTP/1.1\r\nHost: [2001:db8::2]:8443\r\n\r\n") ++ "\n"

/-! ## Responses -/

/-- What a parse ended with, for comparisons. -/
structure Outcome where
  phaseDone : Bool
  error : String
  head : Option Head
  body : ByteArray
  leftover : ByteArray

def Outcome.same (a b : Outcome) : Bool :=
  a.phaseDone == b.phaseDone && a.error == b.error && a.head == b.head && a.body == b.body
    && a.leftover == b.leftover

def outcome (p : Parser) : Outcome :=
  { phaseDone := p.isDone, error := (p.error?).getD "", head := p.head?, body := p.body, leftover := p.leftover }

/-- Feed `data` cut at the given points, then EOF. -/
def feedCuts (isHead : Bool) (data : ByteArray) (cuts : Array Nat) : Outcome := Id.run do
  let mut p := Parser.new isHead
  let mut a := 0
  for c in cuts do
    p := p.feedSlice data a c
    a := c
  p := (p.feedSlice data a data.size).finish
  return outcome p

/-- Byte by byte, taking the body after every byte (streaming use). -/
def feedBytes (data : ByteArray) : Outcome := Id.run do
  let mut p := Parser.new
  let mut body := ByteArray.empty
  for i in [0:data.size] do
    p := p.feedSlice data i (i + 1)
    let (b, p2) := p.takeBody
    body := body ++ b
    p := p2
  p := p.finish
  return { outcome p with body := body ++ p.body }

def headText (h : Head) : String := Id.run do
  let mut s := "HTTP/1." ++ n h.minor ++ " " ++ n h.status ++ " " ++ h.reason
  for (k, v) in h.headers do s := s ++ " | " ++ k ++ ": " ++ v
  return s

/-- Every 2-way split of `data` must give the same outcome as one feed. -/
def allSplits (isHead : Bool) (data : ByteArray) (points : Array Nat) : Nat × Nat := Id.run do
  let ref := feedCuts isHead data #[]
  let mut good := 0
  for k in points do
    if (feedCuts isHead data #[k]).same ref then good := good + 1
  return (good, points.size)

def range (a b : Nat) : Array Nat := Id.run do
  let mut r := #[]
  for i in [a:b] do r := r.push i
  return r

def recorded (name : String) (raw body : ByteArray) (wantStatus : Nat) (exhaustive : Bool) : String := Id.run do
  let ref := feedCuts false raw #[]
  let mut o := "http " ++ name ++ ": " ++ n raw.size ++ " bytes; "
  match ref.head with
  | none => return o ++ "FAIL no head: " ++ ref.error ++ "\n"
  | some h =>
    o := o ++ (if ref.phaseDone && h.status == wantStatus && ref.body == body then
        "complete, body (" ++ n body.size ++ " bytes) identical to Python's decoding"
      else "FAIL " ++ ref.error) ++ "\n  head: " ++ headText h ++ "\n"
  -- Split points: all of them, or the head plus 16 bytes around every
  -- chunk-size line.
  let points ← if exhaustive then pure (range 0 (raw.size + 1)) else do
    let mut pts := range 0 600
    let mut i := 0
    while i + 1 < raw.size do
      if byteAt raw i == 13 && byteAt raw (i + 1) == 10 then
        -- a CRLF inside the body region marks a chunk boundary
        if i > 600 then
          for j in [i - 16:min (i + 24) (raw.size + 1)] do pts := pts.push j  -- (Array.append is not linked)
      i := i + 1
    pure (pts.push raw.size)
  let (good, total) := allSplits false raw points
  o := o ++ "  2-way splits: " ++ n good ++ " of " ++ n total ++ " identical to the single feed\n"
  let bytes := feedBytes raw
  o := o ++ "  byte-by-byte with takeBody: " ++ ok (bytes.same ref) ++ "\n"
  return o

structure Synth where
  name : String
  data : String
  isHead : Bool := false
  status : Nat
  body : String
  leftover : String := ""
  error : String := ""

def synths : Array Synth := #[
  { name := "100-continue, gzip+chunked, extension, trailer",
    data := "HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 200 OK\r\nTransfer-Encoding: gzip, chunked\r\nX-A:  spaced value \r\n\r\n5;ext=1\r\nhello\r\nA\r\n0123456789\r\n0\r\nTrailer: x\r\n\r\n",
    status := 200, body := "hello0123456789" },
  { name := "Content-Length, bare LF line ends, pipelined bytes after",
    data := "HTTP/1.1 206 Partial Content\nContent-Range: bytes 0-3/10\ncontent-length: 4\n\nabcdEXTRA",
    status := 206, body := "abcd", leftover := "EXTRA" },
  { name := "close-delimited HTTP/1.0",
    data := "HTTP/1.0 200 OK\r\nServer: x\r\n\r\nthe body runs until the connection closes\r\n\r\n",
    status := 200, body := "the body runs until the connection closes\r\n\r\n" },
  { name := "204 without body, then leftover",
    data := "HTTP/1.1 204 No Content\r\nContent-Length: 7\r\n\r\nleft", status := 204, body := "", leftover := "left" },
  { name := "HEAD request with Content-Length",
    data := "HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\n", isHead := true, status := 200, body := "" },
  { name := "Content-Length: 0", data := "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n", status := 200, body := "" },
  { name := "chunked, uppercase hex, no trailer CRLF before EOF",
    data := "HTTP/1.1 200 OK\r\nTransfer-Encoding: Chunked\r\n\r\n1F\r\n0123456789abcdef0123456789abcde\r\n0\r\n",
    status := 200, body := "0123456789abcdef0123456789abcde" },
  { name := "error: EOF inside Content-Length body",
    data := "HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nabc", status := 200, body := "abc",
    error := "HTTP: connection closed with 7 body bytes missing" },
  { name := "error: bad chunk size",
    data := "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nzz\r\n", status := 200, body := "",
    error := "HTTP: bad chunk size: zz" },
  { name := "error: chunk not followed by CRLF",
    data := "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\nabX\r\n", status := 200, body := "ab",
    error := "HTTP: chunk not followed by CRLF" },
  { name := "error: bad Content-Length",
    data := "HTTP/1.1 200 OK\r\nContent-Length: 1x\r\n\r\nabc", status := 200, body := "",
    error := "HTTP: bad Content-Length: 1x" },
  { name := "error: bad status line", data := "HTTP/2 200 OK\r\n\r\n", status := 0, body := "",
    error := "HTTP: bad status line: HTTP/2 200 OK" },
  { name := "error: EOF in headers", data := "HTTP/1.1 200 OK\r\nServer: x", status := 0, body := "",
    error := "HTTP: connection closed in the headers" },
  { name := "error: EOF before any byte", data := "", status := 0, body := "",
    error := "HTTP: connection closed before a response" }]

def synth (s : Synth) : String := Id.run do
  let data := s.data.toUTF8
  let ref := feedCuts s.isHead data #[]
  let status := match ref.head with | some h => h.status | none => 0
  let good := status == s.status && ref.body == s.body.toUTF8 && ref.leftover == s.leftover.toUTF8
    && ref.error == s.error && (s.error != "" || ref.phaseDone)
  -- Every pair of cut points.
  let mut same := 0
  let mut total := 0
  for a in [0:data.size + 1] do
    for b in [a:data.size + 1] do
      total := total + 1
      if (feedCuts s.isHead data #[a, b]).same ref then same := same + 1
  return "http synthetic " ++ s.name ++ ": " ++ ok good
    ++ (if good then "" else " (status " ++ n status ++ ", body " ++ lossy ref.body ++ ", error " ++ ref.error ++ ")")
    ++ "; " ++ n same ++ " of " ++ n total ++ " 3-way splits identical\n"

def headLimit : String :=
  let big := "HTTP/1.1 200 OK\r\nX: ".toUTF8 ++ zeros 70000 ++ "\r\n\r\n".toUTF8
  let r := feedCuts false big #[]
  "http head limit (70 KB header): " ++ ok (r.error == "HTTP: response head larger than 64 KiB") ++ "\n"

def tunnelText : TunnelResult → String
  | .needMore _ => "needMore"
  | .established rest => "established rest=" ++ lossy rest
  | .failed r => "failed " ++ r

/-- Feed the proxy's bytes in two parts. -/
def tunnelSplit (data : ByteArray) (k : Nat) : String :=
  match ({} : Tunnel).feed (data.extract 0 k) with
  | .needMore t => tunnelText (t.feed (data.extract k data.size))
  -- Complete in the first part: what follows belongs to the tunnel.
  | .established rest => "early " ++ tunnelText (.established (rest ++ data.extract k data.size))
  | r => if k == data.size then tunnelText r else "early " ++ tunnelText r

def tunnels : String := Id.run do
  let cases : Array (String × String) := #[
    ("HTTP/1.1 200 Connection established\r\n\r\n", "established rest="),
    ("HTTP/1.0 200 OK\r\nProxy-Agent: p\r\n\r\n\x16\x03", "established rest=\x16\x03"),
    ("HTTP/1.1 407 Proxy Authentication Required\r\nProxy-Authenticate: Basic\r\n\r\n",
     "failed HTTP tunnel failed: 407 Proxy Authentication Required"),
    ("SSH-2.0-OpenSSH\r\n\r\n", "failed HTTP tunnel failed: bad status line")]
  let mut o := ""
  let mut idx := 0
  for (d, want) in cases do
    idx := idx + 1
    let data := d.toUTF8
    let mut good := 0
    for k in [0:data.size + 1] do
      let got := tunnelSplit data k
      -- A complete header in the first part answers right away.
      if got == want || got == "early " ++ want then good := good + 1
    o := o ++ "http tunnel case " ++ n idx ++ ": " ++ n good ++ " of " ++ n (data.size + 1)
      ++ " splits give the expected result: " ++ ok (good == data.size + 1) ++ "\n"
  let long := ("HTTP/1.1 200 OK\r\nX: ".toUTF8 ++ zeros 9000)
  o := o ++ "http tunnel overlong header: " ++ tunnelText (({} : Tunnel).feed long) ++ "\n"
  return o

@[export zr_test_transcript] def transcript (k : Nat) : ByteArray :=
  let search := b64 Fixtures.searchResponse
  let thumb := b64 Fixtures.thumbResponse
  let o := urls ++ requests
    ++ recorded "recorded thumbnail response (Content-Length)" (thumb.extract k thumb.size) (b64 Fixtures.thumbBody) 200 true
    ++ recorded "recorded search response (chunked)" (search.extract k search.size) (b64 Fixtures.searchBody) 200 false
    ++ synths.foldl (fun o s => o ++ synth s) "" ++ headLimit ++ tunnels
  o.toUTF8

end HttpTest
