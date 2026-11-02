import Yt.Tls.Client
import TlsTest.Common
import TlsTest.Trace
import TlsTest.LiveYoutube
import TlsTest.LiveYtimg
import TlsTest.LiveGooglevideo
open Yt.Crypto Yt.X509 Yt.Tls
namespace TlsTest

def leafKind (c : Client) : String :=
  match c.leafKey with
  | .p256 _ => "ecdsa-p256" | .p384 _ => "ecdsa-p384"
  | .rsa n _ => "rsa-" ++ toString (rsaBits n) | .unsupported => "none"

/-- Up to the first CRLF. -/
def firstLine (b : ByteArray) : String := Id.run do
  let mut s := ""
  for i in [0:min b.size 200] do
    let c := b[i]!
    if c == 13 || c == 10 then break
    s := s.push (Char.ofNat c.toNat)
  s

/-- Replay a session recorded by `tlsprobe` against a live host: with the
recorded entropy and time the client must emit exactly the recorded bytes
(ClientHello, change_cipher_spec + Finished, the HTTP request record),
validate the real certificate chain against the compiled-in roots, and
decrypt any recorded application data. -/
def replay (o : ByteArray) (name serverName : String) (time : UInt64) (entropy received sent request : String) : ByteArray := Id.run do
  let mut o := o
  -- the suite order the sessions were recorded with
  let cfg : Config := { serverName := serverName, unixTime := time, entropy := ofHex entropy,
                        suites := #[.aes128gcm, .chacha20, .aes256gcm] }
  let (c, hello) := start cfg
  let r := feed c (ofHex received)
  let (_, req) := send r.client request.toUTF8
  o := line o (name ++ ": " ++ statusName r.status ++ " " ++ r.client.cipherSuite ++ " alpn " ++ r.client.protocol
    ++ " leaf " ++ leafKind r.client ++ " certs " ++ toString r.client.certs.size)
  o := checkB o (name ++ " client bytes identical to the live session") (hello ++ r.send ++ req == ofHex sent)
  if r.data.size > 0 then
    o := line o (name ++ " first response line: " ++ firstLine r.data ++ " (" ++ toString r.data.size ++ " bytes)")
  -- again with the signature cache from the first run: same bytes, cache unchanged
  let (cw, hw) := start { cfg with sigCache := r.client.sigCache }
  let rw := feed cw (ofHex received)
  let (_, reqw) := send rw.client request.toUTF8
  o := checkB o (name ++ " warm signature cache (" ++ toString r.client.sigCache.size ++ " entries): same bytes")
    (hw ++ rw.send ++ reqw == ofHex sent && rw.client.sigCache.size == r.client.sigCache.size)
  -- the same session long after the chain expires fails
  let (cl, _) := start { cfg with unixTime := 4102444800 }
  o := line o (name ++ " in 2100: " ++ statusName (feed cl (ofHex received)).status)
  o

def replayTests (o : ByteArray) : ByteArray := Id.run do
  let mut o := o
  o := replay o "live www.youtube.com" LiveYoutube.serverName LiveYoutube.unixTime LiveYoutube.entropy
    LiveYoutube.received LiveYoutube.sent LiveYoutube.request
  o := replay o "live i.ytimg.com" LiveYtimg.serverName LiveYtimg.unixTime LiveYtimg.entropy
    LiveYtimg.received LiveYtimg.sent LiveYtimg.request
  o := replay o "live redirector.googlevideo.com" LiveGooglevideo.serverName LiveGooglevideo.unixTime
    LiveGooglevideo.entropy LiveGooglevideo.received LiveGooglevideo.sent LiveGooglevideo.request
  o

/-- Host-side inspection helper: the decrypted application data of a fixture. -/
def replayData (serverName : String) (time : UInt64) (entropy received : String) : ByteArray :=
  let (c, _) := start { serverName := serverName, unixTime := time, entropy := ofHex entropy,
                        suites := #[.aes128gcm, .chacha20, .aes256gcm] }
  (feed c (ofHex received)).data

end TlsTest
