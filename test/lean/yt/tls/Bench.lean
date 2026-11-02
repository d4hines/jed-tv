import Yt.Tls.Client
import TlsTest.Chains
import TlsTest.Replay
/-! Host-only micro-benchmarks (upstream runtime): record-sized AEAD
throughput, hashing, and the public-key operations of a handshake. Each
result feeds a checksum that is tested before the clock is read, so the
work cannot move outside the timed region. -/
open Yt.Crypto Yt.X509 Yt.Tls TlsTest

def timeIt (name : String) (iters : Nat) (bytesPer : Nat) (f : Nat → UInt8) : IO Unit := do
  let t0 ← IO.monoNanosNow
  let mut acc : UInt8 := 0
  for i in [0:iters] do acc := acc ^^^ f i
  if acc == 42 then IO.print ""
  let t1 ← IO.monoNanosNow
  let ns := (t1 - t0).toFloat
  let per := ns / iters.toFloat
  if bytesPer > 0 then
    let mbs := (bytesPer * iters).toFloat / (ns / 1.0e9) / 1.0e6
    IO.println s!"{name}: {mbs.toString} MB/s ({iters} x {bytesPer} bytes)"
  else
    IO.println s!"{name}: {(per / 1.0e6).toString} ms/op ({iters} ops)"

def main (args : List String) : IO Unit := do
  if args == ["loop-p384"] then
    let yt := hexLines Chains.youtubeEcdsa
    let some we2 := Cert.parse yt[1]! | throw (IO.userError "parse")
    let r4 := defaultAnchors[1]!
    timeIt "loop P-384 verify" 400 0 fun _ => if verifySig {} r4.key we2.sigAlg we2.tbs we2.sig then 1 else 0
    return
  let rec16 ← IO.getRandomBytes 16384
  let key16 ← IO.getRandomBytes 16
  let key32 ← IO.getRandomBytes 32
  let nonce ← IO.getRandomBytes 12
  let aad := ByteArray.mk #[23, 3, 3, 64, 17]
  let g128 := Gcm.init key16
  let g256 := Gcm.init key32
  timeIt "AES-128-GCM encrypt" 64 16384 fun i => (Gcm.encrypt g128 nonce aad (rec16.set! 0 i.toUInt8))[7]!
  let ct := Gcm.encrypt g128 nonce aad rec16
  timeIt "AES-128-GCM decrypt" 64 16384 fun _ => ((Gcm.decrypt g128 nonce aad ct).getD .empty)[7]!
  timeIt "AES-256-GCM encrypt" 64 16384 fun i => (Gcm.encrypt g256 nonce aad (rec16.set! 0 i.toUInt8))[7]!
  timeIt "ChaCha20-Poly1305 encrypt" 64 16384 fun i => (ChaCha.encrypt key32 nonce aad (rec16.set! 0 i.toUInt8))[7]!
  let cc := ChaCha.encrypt key32 nonce aad rec16
  timeIt "ChaCha20-Poly1305 decrypt" 64 16384 fun _ => ((ChaCha.decrypt key32 nonce aad cc).getD .empty)[7]!
  let aes := Aes.expand key16
  timeIt "AES-128 block (ECB, 1 block)" 20000 16 fun i => (Aes.encryptBlock aes (rec16.set! 0 i.toUInt8))[3]!
  timeIt "SHA-256" 64 16384 fun i => (Sha256.hash (rec16.set! 0 i.toUInt8))[0]!
  timeIt "SHA-384" 64 16384 fun i => (Sha512.hash384 (rec16.set! 0 i.toUInt8))[0]!
  -- public key
  let sk ← IO.getRandomBytes 32
  timeIt "X25519 scalar multiplication" 20 0 fun i => (X25519.publicKey (sk.set! 0 i.toUInt8))[0]!
  let yt := hexLines Chains.youtubeEcdsa
  let ytr := hexLines Chains.youtubeRsa
  let some leaf := Cert.parse yt[0]! | throw (IO.userError "parse")
  let some we2 := Cert.parse yt[1]! | throw (IO.userError "parse")
  let some rleaf := Cert.parse ytr[0]! | throw (IO.userError "parse")
  let some wr2 := Cert.parse ytr[1]! | throw (IO.userError "parse")
  let r1 := defaultAnchors[0]!
  let r4 := defaultAnchors[1]!
  let pol : Policy := {}
  let b (x : Bool) : UInt8 := if x then 1 else 0
  timeIt "ECDSA P-256 verify (leaf by WE2)" 20 0 fun _ => b (verifySig pol we2.key leaf.sigAlg leaf.tbs leaf.sig)
  timeIt "ECDSA P-384 verify (WE2 by GTS R4)" 10 0 fun _ => b (verifySig pol r4.key we2.sigAlg we2.tbs we2.sig)
  timeIt "RSA-2048 PKCS#1 verify (leaf by WR2)" 20 0 fun _ => b (verifySig pol wr2.key rleaf.sigAlg rleaf.tbs rleaf.sig)
  timeIt "RSA-4096 PKCS#1 verify (WR2 by GTS R1)" 10 0 fun _ => b (verifySig pol r1.key wr2.sigAlg wr2.tbs wr2.sig)
  timeIt "chain validation, ECDSA chain" 5 0 fun _ =>
    match verifyServer yt "www.youtube.com" Chains.captureTime with | .ok _ => 1 | .error _ => 0
  timeIt "chain validation, RSA chain" 5 0 fun _ =>
    match verifyServer ytr "www.youtube.com" Chains.captureTime with | .ok _ => 1 | .error _ => 0
  let cold := Id.run do
    let (c, _) := start { serverName := LiveYoutube.serverName, unixTime := LiveYoutube.unixTime,
                          entropy := ofHex LiveYoutube.entropy, suites := #[.aes128gcm, .chacha20, .aes256gcm] }
    (feed c (ofHex LiveYoutube.received)).client.sigCache
  timeIt "full client handshake, warm signature cache" 5 0 fun _ =>
    let (c, _) := start { serverName := LiveYoutube.serverName, unixTime := LiveYoutube.unixTime,
                          entropy := ofHex LiveYoutube.entropy, suites := #[.aes128gcm, .chacha20, .aes256gcm],
                          sigCache := cold }
    let r := feed c (ofHex LiveYoutube.received)
    r.send[0]!
  timeIt "full client handshake (replayed www.youtube.com session)" 5 0 fun _ =>
    let (c, _) := start { serverName := LiveYoutube.serverName, unixTime := LiveYoutube.unixTime,
                          entropy := ofHex LiveYoutube.entropy, suites := #[.aes128gcm, .chacha20, .aes256gcm] }
    let r := feed c (ofHex LiveYoutube.received)
    r.send[0]!
