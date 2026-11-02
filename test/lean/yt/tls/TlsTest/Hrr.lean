import Yt.Tls.Client
import TlsTest.Common
import TlsTest.Trace
open Yt.Crypto Yt.X509 Yt.Tls
namespace TlsTest

/-- A HelloRetryRequest from the "server": the client's session id echoed,
the given suite and extensions. -/
def hrrRecord (c : Client) (suite : Nat) (exts : ByteArray) : ByteArray :=
  let body := pushU16BE .empty 0x0303 ++ hrrRandom ++ vec8 c.sessionId ++ pushU16BE .empty suite
    ++ (ByteArray.empty.push 0) ++ vec16 exts
  plainRecord 22 0x0303 (hsMsg 2 body)

def versions13 : ByteArray := ext 43 (pushU16BE .empty 0x0304)

/-- HelloRetryRequest handling, synthetically (no available server sends a
cookie-only HRR to an x25519-only client): the second ClientHello must be
the first plus the cookie, preceded by change_cipher_spec, the transcript
must restart from message_hash(ClientHello1), and the illegal variants
must fail. -/
def hrrTests (o : ByteArray) : ByteArray := Id.run do
  let mut o := o
  let cfg : Config := { serverName := "www.youtube.com", unixTime := 1793522018, entropy := Sha256.hash (ascii "hrr test") }
  let (c, hello) := start cfg
  let ch1 := slice hello 5 (hello.size - 5)
  let cookie := ascii "a cookie from the server"
  let r := feed c (hrrRecord c 0x1303 (versions13 ++ ext 44 (vec16 cookie)))
  let ch2 := buildClientHello cfg r.client.random r.client.sessionId r.client.pub cookie
  o := line o ("hrr cookie: " ++ statusName r.status)
  o := checkB o "hrr: change_cipher_spec then ClientHello2 with the cookie"
    (r.send == plainRecord 20 0x0303 (ByteArray.empty.push 1) ++ plainRecord 22 0x0303 ch2)
  let hrrRec := hrrRecord c 0x1303 (versions13 ++ ext 44 (vec16 cookie))
  let hrrMsg := slice hrrRec 5 (hrrRec.size - 5)
  o := checkB o "hrr: transcript is message_hash(CH1) ++ HRR ++ CH2"
    (r.client.transcript == hsMsg 254 (Sha256.hash ch1) ++ hrrMsg ++ ch2)
  o := checkB o "hrr: ClientHello2 differs from ClientHello1 only by the cookie extension"
    (ch2.size == ch1.size + 4 + 2 + cookie.size)
  let r2 := feed r.client (hrrRecord c 0x1303 (versions13 ++ ext 44 (vec16 cookie)))
  o := line o ("hrr twice: " ++ statusName r2.status)
  let rk := feed c (hrrRecord c 0x1303 (versions13 ++ ext 51 (pushU16BE .empty 0x001d)))
  o := line o ("hrr asking for x25519 again: " ++ statusName rk.status)
  let rp := feed c (hrrRecord c 0x1303 (versions13 ++ ext 51 (pushU16BE .empty 0x0017)))
  o := line o ("hrr asking for P-256: " ++ statusName rp.status)
  let rn := feed c (hrrRecord c 0x1303 versions13)
  o := line o ("hrr without cookie or group: " ++ statusName rn.status)
  let rs := feed c (hrrRecord c 0x1304 (versions13 ++ ext 44 (vec16 cookie)))
  o := line o ("hrr with an unknown suite: " ++ statusName rs.status)
  let rv := feed c (hrrRecord c 0x1303 (ext 44 (vec16 cookie)))
  o := line o ("hrr without supported_versions: " ++ statusName rv.status)
  -- after an HRR choosing ChaCha20, a ServerHello with another suite fails
  let shBody := pushU16BE .empty 0x0303 ++ zeros 32 ++ vec8 c.sessionId ++ pushU16BE .empty 0x1301
    ++ (ByteArray.empty.push 0) ++ vec16 (versions13 ++ ext 51 (pushU16BE .empty 0x001d ++ vec16 (X25519.publicKey (zeros 32 |>.set! 0 7))))
  let rc := feed r.client (plainRecord 22 0x0303 (hsMsg 2 shBody))
  o := line o ("ServerHello changing the suite after hrr: " ++ statusName rc.status)
  o

end TlsTest
