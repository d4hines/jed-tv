import Yt.Json.Fmt

/-! URLs for the HTTP client: `http://` and `https://` only, as
src/ytplay/src/http.rs's `parse_url`, with these additions: the scheme is
case-insensitive; the authority also ends at `?` or `#` (so
`https://h?q=1` requests `/?q=1`); a `#fragment` is not sent; bracketed
IPv6 literals (`http://[::1]:8090/`) work; userinfo (`user@host`), empty
hosts, bad ports and spaces/control characters are rejected. -/
namespace Yt.Http
open Yt.Fmt

structure Url where
  tls : Bool
  /-- Host name or address, without brackets for IPv6. -/
  host : String
  port : UInt16
  /-- Path and query, starting with `/`; no fragment. -/
  path : String
  deriving Inhabited, BEq

namespace Url

def defaultPort (tls : Bool) : UInt16 := if tls then 443 else 80

/-- The host as it appears in a URL authority (IPv6 in brackets). -/
def hostLiteral (u : Url) : String :=
  if contains u.host.toUTF8 ":".toUTF8 then "[" ++ u.host ++ "]" else u.host

/-- `Host` header value: the port is included when it is not the default
(http.rs: virtual hosts and our replay server need it). -/
def hostHeader (u : Url) : String :=
  if u.port == defaultPort u.tls then u.hostLiteral else u.hostLiteral ++ ":" ++ toString u.port.toNat

/-- `host:port`, as in a CONNECT request. -/
def authority (u : Url) : String := u.hostLiteral ++ ":" ++ toString u.port.toNat

/-- The URL again (scheme, authority, path). -/
def toString (u : Url) : String :=
  (if u.tls then "https://" else "http://") ++ u.hostHeader ++ u.path

def parsePort (b : ByteArray) : Option UInt16 := Id.run do
  if b.size == 0 || b.size > 5 then return none
  let mut x : Nat := 0
  for c in b do
    if c < 48 || c > 57 then return none
    x := x * 10 + (c - 48).toNat
  if x > 65535 then return none
  return some x.toUInt16

def parseBytes (s : ByteArray) : Except String Url := do
  for c in s do
    if c ≤ 32 || c == 127 then throw "URL: space or control character"
  let (tls, rest) ←
    if s.size ≥ 8 && eqIgnoreCaseAt s 0 8 "https://".toUTF8 then pure (true, 8)
    else if s.size ≥ 7 && eqIgnoreCaseAt s 0 7 "http://".toUTF8 then pure (false, 7)
    else throw ("unsupported URL: " ++ lossy s)
  -- The authority runs to the first '/', '?' or '#'.
  let mut e := rest
  while e < s.size && byteAt s e != 47 && byteAt s e != 63 && byteAt s e != 35 do e := e + 1
  let auth := s.extract rest e
  if contains auth "@".toUTF8 then throw "URL: userinfo is not supported"
  -- Path and query, without the fragment.
  let mut f := e
  while f < s.size && byteAt s f != 35 do f := f + 1
  let pq := s.extract e f
  let path := if pq.size == 0 then "/" else if byteAt pq 0 == 63 then "/" ++ str pq else str pq
  let (host, portB) ←
    if byteAt auth 0 == 91 then do
      let mut k := 1
      while k < auth.size && byteAt auth k != 93 do k := k + 1
      if k ≥ auth.size then throw "URL: unclosed '['"
      let h := auth.extract 1 k
      if k + 1 == auth.size then pure (h, none)
      else if byteAt auth (k + 1) == 58 then pure (h, some (auth.extract (k + 2) auth.size))
      else throw "URL: junk after ']'"
    else do
      -- rsplit_once(':'), as http.rs.
      let mut k := auth.size
      while k > 0 && byteAt auth (k - 1) != 58 do k := k - 1
      if k == 0 then pure (auth, none)
      else pure (auth.extract 0 (k - 1), some (auth.extract k auth.size))
  if host.size == 0 then throw "URL: empty host"
  let port ← match portB with
    | none => pure (defaultPort tls)
    | some p => match parsePort p with
      | some x => pure x
      | none => throw ("URL: bad port " ++ lossy p)
  pure { tls, host := str host, port, path }

/-- Parse an `http://` or `https://` URL. -/
def parse (s : String) : Except String Url := parseBytes s.toUTF8

end Url
end Yt.Http
