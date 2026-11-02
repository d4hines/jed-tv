import Yt.Tls.Client
import TlsTest.Common
import TlsTest.Trace
import TlsTest.Chains
open Yt.Crypto Yt.X509 Yt.Tls
namespace TlsTest

/-- Outcome tally: how many runs ended connected / still handshaking /
failed. The point is that none of them panics or crashes (the host run's
stderr must stay empty). -/
structure Tally where
  ok : Nat := 0
  pending : Nat := 0
  failed : Nat := 0

def Tally.add (t : Tally) (s : Status) : Tally :=
  match s with
  | .connected | .closed => { t with ok := t.ok + 1 }
  | .handshaking => { t with pending := t.pending + 1 }
  | .failed _ => { t with failed := t.failed + 1 }

def Tally.str (t : Tally) : String :=
  "connected " ++ toString t.ok ++ ", handshaking " ++ toString t.pending ++ ", failed " ++ toString t.failed

def mutate (b : ByteArray) (p : Nat) (x : UInt8) : ByteArray := b.set! p (b[p]! ^^^ x)

def fuzzTests (o : ByteArray) : ByteArray := Id.run do
  let mut o := o
  let ch := ofHex Rfc8448.S3.clientHello
  let priv := ofHex Rfc8448.S3.clientPriv
  let (c0, _) := startWithClientHello rfc8448Config ch priv
  -- 1. the decrypted server flight (EE, Certificate, CertificateVerify,
  --    Finished) mutated byte by byte, fed to the handshake layer directly
  let c1 := (feed c0 (ofHex Rfc8448.S3.serverHelloRecord)).client
  let flight := ofHex Rfc8448.S3.serverHsPayload
  let run (c : Client) (msgs : ByteArray) : Status :=
    match drainHandshake { c with hsBuf := msgs } with
    | .ok c => c.status
    | .error f => .failed f.msg
  o := line o ("fuzz: unmodified flight " ++ statusName (run c1 flight))
  let mut t : Tally := {}
  for p in [0:flight.size] do
    for x in [0x01, 0x80, 0xff] do
      t := t.add (run c1 (mutate flight p x))
  o := line o ("fuzz: flight with one byte changed (" ++ toString (3 * flight.size) ++ " runs): " ++ t.str)
  t := {}
  for n in [0:flight.size] do
    t := t.add (run c1 (slice flight 0 n))
  o := line o ("fuzz: flight truncated at every length: " ++ t.str)
  -- 2. the plaintext ServerHello record mutated
  let sh := ofHex Rfc8448.S3.serverHelloRecord
  t := {}
  for p in [0:sh.size] do
    for x in [0x01, 0x80, 0xff] do
      t := t.add (feed c0 (mutate sh p x)).status
  o := line o ("fuzz: ServerHello record with one byte changed: " ++ t.str)
  -- 3. a real certificate mutated and truncated: parsing must just fail
  let leaf := (hexLines Chains.youtubeEcdsa)[0]!
  let mut parsed := 0
  let mut rejected := 0
  for p in [0:leaf.size] do
    if p % 3 == 0 then
      match Cert.parse (mutate leaf p 0x55) with
      | some _ => parsed := parsed + 1
      | none => rejected := rejected + 1
  for n in [0:leaf.size] do
    match Cert.parse (slice leaf 0 n) with
    | some _ => parsed := parsed + 1
    | none => rejected := rejected + 1
  o := line o ("fuzz: certificate mutations/truncations: " ++ toString parsed ++ " still parse, " ++ toString rejected ++ " rejected")
  -- 4. records of garbage
  t := {}
  let mut g := Sha256.hash (ascii "garbage")
  for _ in [0:64] do
    g := Sha256.hash g
    t := t.add (feed c0 g).status
    t := t.add (feed c1 ((ByteArray.empty.push 23 |>.push 3 |>.push 3 |>.push 0 |>.push 32) ++ g)).status
  o := line o ("fuzz: garbage input: " ++ t.str)
  o

end TlsTest
