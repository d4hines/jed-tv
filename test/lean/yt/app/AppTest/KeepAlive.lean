import Yt.Net.Fetch

namespace AppTest.KeepAlive
open Yt Yt.Net Yt.Http

private def parsed (response : String) : Parser := Parser.new.feed response.toUTF8

private def fetch (tls : Bool := false) : Fetch :=
  let url := (Url.parse (if tls then "https://example.test/" else "http://example.test/")).toOption.getD default
  { id := 1, url, request := .empty, viaProxy := false, sent := true,
    conn := { fd := 17, state := .connected }, phase := .http,
    parser := parsed "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n",
    tls := if tls then some { cfg := default, phase := .connected } else none }

def cases : Array (String × Bool) := Id.run do
  let plain := fetch
  let secure := fetch true
  let mut result := #[
    ("plain framed response", plain.reusable),
    ("connected TLS", secure.reusable),
    ("missing TLS session", !({ secure with tls := none } : Fetch).reusable),
    ("unexpected TLS session", !({ plain with tls := secure.tls } : Fetch).reusable)]
  for phase in [Tls.Phase.waitServerHello, .waitEncryptedExtensions, .waitCertOrRequest,
      .waitCertificate, .waitCertVerify, .waitFinished, .closed, .failed] do
    let client : Tls.Client := { cfg := default, phase }
    result := result.push ("nonconnected TLS phase", !({ secure with tls := some client } : Fetch).reusable)
  let closing : Tls.Client := { cfg := default, phase := .connected, sentClose := true }
  result := result.push ("local close_notify", !({ secure with tls := some closing } : Fetch).reusable)
  let closed : Tls.Client := { cfg := default, phase := .closed }
  result := result ++ #[
    ("peer close_notify ends response stream", ({ secure with tls := some closed } : Fetch).streamEnded),
    ("connected TLS stream stays open", !secure.streamEnded)]
  for (header, allowed) in [
      ("Connection: close", false), ("Connection: keep-alive, close", false),
      ("cOnNeCtIoN: ClOsE, upgrade", false), ("Connection: keep-alive, \tCLOSE\t", false),
      ("Connection: keep-alive\r\nConnection: close", false),
      ("Connection: keep-alive, xclose", true), ("Connection: closeish", true),
      ("Connection: keep-alive", true), ("X-Connection: close", true)] do
    let p := parsed ("HTTP/1.1 200 OK\r\nContent-Length: 0\r\n" ++ header ++ "\r\n\r\n")
    result := result.push (header, ({ plain with parser := p } : Fetch).reusable == allowed)
  result := result ++ #[
    ("HTTP 1.0", !({ plain with parser := parsed "HTTP/1.0 200 OK\r\nContent-Length: 0\r\n\r\n" } : Fetch).reusable),
    ("streaming response", !({ plain with streaming := true } : Fetch).reusable),
    ("transport EOF", !({ plain with conn := { plain.conn with eof := true } } : Fetch).reusable),
    ("transport closed", !({ plain with conn := { plain.conn with state := .closed } } : Fetch).reusable),
    ("transport failed", !({ plain with conn := { plain.conn with state := .failed 32 } } : Fetch).reusable),
    ("unread transport bytes", !({ plain with conn := { plain.conn with inbox := ByteArray.mk #[0] } } : Fetch).reusable),
    ("incomplete response", !({ plain with parser := parsed "HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\n" } : Fetch).reusable),
    ("trailing response bytes", !({ plain with parser := parsed "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\nx" } : Fetch).reusable)]
  return result

def allPass : Bool := cases.all (·.2)

def transcript : ByteArray :=
  let passed := cases.foldl (fun n (_, ok) => if ok then n + 1 else n) 0
  let failures := cases.foldl (fun out (name, ok) => if ok then out else out ++ "FAIL: " ++ name ++ "\n") ""
  ("keepalive eligibility: " ++ toString passed ++ " of " ++ toString cases.size ++ " passed\n" ++ failures).toUTF8

end AppTest.KeepAlive
