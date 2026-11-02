import Yt.Tls.Client
import TlsTest.Common
import TlsTest.Rfc8448
open Yt.Crypto Yt.X509 Yt.Tls
namespace TlsTest

def statusName : Status → String
  | .handshaking => "handshaking"
  | .connected => "connected"
  | .closed => "closed"
  | .failed r => "failed: " ++ r

def secretOf (c : Option Cipher) : ByteArray := match c with | some x => x.secret | none => .empty

/-- RFC 8448 section 3 as a deterministic end-to-end test: the client is
started from the trace's ClientHello and x25519 key, fed the server's
records, and must produce the trace's client records byte for byte. The
server certificate (RSA-1024, CN=rsa, no subjectAltName) is the trust
anchor, at a time inside its validity, with name checking off. -/
def rfc8448Config : Config :=
  let certMsg := ofHex Rfc8448.S3.certificateMsg
  let certDer := slice certMsg 11 (u24 certMsg 8)
  { serverName := "server", unixTime := 1546300800, entropy := .empty,
    anchors := (Anchor.ofDer certDer).toArray, policy := { minRsaBits := 1024, checkName := false } }

def rfc8448Tests (o : ByteArray) : ByteArray := Id.run do
  let mut o := o
  let open3 := Rfc8448.S3.clientHello
  let (c0, chRec) := startWithClientHello rfc8448Config (ofHex open3) (ofHex Rfc8448.S3.clientPriv)
  o := check o "rfc8448 ClientHello record" chRec Rfc8448.S3.clientHelloRecord
  let r1 := feed c0 (ofHex Rfc8448.S3.serverHelloRecord)
  o := line o ("rfc8448 after ServerHello: " ++ statusName r1.status ++ ", sends " ++ toString r1.send.size)
  o := check o "rfc8448 s hs traffic" (secretOf r1.client.rd) Rfc8448.S3.sHsTraffic
  o := check o "rfc8448 c hs traffic" (secretOf r1.client.wr) Rfc8448.S3.cHsTraffic
  let r2 := feed r1.client (ofHex Rfc8448.S3.serverHsRecord)
  o := line o ("rfc8448 after server flight: " ++ statusName r2.status ++ " " ++ r2.client.cipherSuite)
  o := check o "rfc8448 client Finished record" r2.send Rfc8448.S3.clientFinishedRecord
  o := check o "rfc8448 s ap traffic" (secretOf r2.client.rd) Rfc8448.S3.sApTraffic
  o := check o "rfc8448 c ap traffic" (secretOf r2.client.wr) Rfc8448.S3.cApTraffic
  let r3 := feed r2.client (ofHex Rfc8448.S3.ticketRecord)
  o := line o ("rfc8448 after NewSessionTicket: " ++ statusName r3.status ++ ", sends " ++ toString r3.send.size)
  let mut payload := ByteArray.empty
  for i in [0:50] do payload := payload.push i.toUInt8
  let (c4, appRec) := send r3.client payload
  o := check o "rfc8448 client application_data record" appRec Rfc8448.S3.clientAppRecord
  let r5 := feed c4 (ofHex Rfc8448.S3.serverAppRecord)
  o := checkB o "rfc8448 server application data decrypts" (r5.data == payload)
  let (c6, alertRec) := close r5.client
  o := check o "rfc8448 client close_notify record" alertRec Rfc8448.S3.clientAlertRecord
  let r7 := feed c6 (ofHex Rfc8448.S3.serverAlertRecord)
  o := line o ("rfc8448 after server close_notify: " ++ statusName r7.status)
  -- the whole server side at once, and one byte at a time
  let all := ofHex Rfc8448.S3.serverHelloRecord ++ ofHex Rfc8448.S3.serverHsRecord
    ++ ofHex Rfc8448.S3.ticketRecord ++ ofHex Rfc8448.S3.serverAppRecord
  let ra := feed c0 all
  o := checkB o "rfc8448 one feed: Finished and data" (ra.send == ofHex Rfc8448.S3.clientFinishedRecord && ra.data == payload)
  let mut c := c0
  let mut sent := ByteArray.empty
  let mut data := ByteArray.empty
  for i in [0:all.size] do
    let r := feed c (slice all i 1)
    c := r.client
    sent := sent ++ r.send
    data := data ++ r.data
  o := checkB o "rfc8448 byte-at-a-time: Finished and data" (sent == ofHex Rfc8448.S3.clientFinishedRecord && data == payload)
  -- failures: tampered record, wrong anchor, bad Finished
  let bad := (ofHex Rfc8448.S3.serverHsRecord)
  let bad := bad.set! 100 (bad[100]! ^^^ 1)
  let rb := feed r1.client bad
  o := line o ("rfc8448 tampered record: " ++ statusName rb.status ++ " alert " ++ toHex rb.send)
  let (cw, _) := startWithClientHello { rfc8448Config with anchors := defaultAnchors } (ofHex open3) (ofHex Rfc8448.S3.clientPriv)
  let rw := feed (feed cw (ofHex Rfc8448.S3.serverHelloRecord)).client (ofHex Rfc8448.S3.serverHsRecord)
  o := line o ("rfc8448 untrusted certificate: " ++ statusName rw.status ++ ", alert record " ++ toString rw.send.size ++ " bytes")
  let (cx, _) := startWithClientHello { rfc8448Config with unixTime := 1800000000 } (ofHex open3) (ofHex Rfc8448.S3.clientPriv)
  let rx := feed (feed cx (ofHex Rfc8448.S3.serverHelloRecord)).client (ofHex Rfc8448.S3.serverHsRecord)
  o := line o ("rfc8448 expired certificate: " ++ statusName rx.status)
  let (cs, _) := startWithClientHello { rfc8448Config with policy := { checkName := false } } (ofHex open3) (ofHex Rfc8448.S3.clientPriv)
  let rs := feed (feed cs (ofHex Rfc8448.S3.serverHelloRecord)).client (ofHex Rfc8448.S3.serverHsRecord)
  o := line o ("rfc8448 RSA-1024 under the default 2048-bit policy: " ++ statusName rs.status)
  let rp := feed c0 (ofHex Rfc8448.S3.serverHsRecord)
  o := line o ("rfc8448 encrypted record before ServerHello: " ++ statusName rp.status)
  o

end TlsTest
