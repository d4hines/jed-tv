import Yt.X509.Cert
import Yt.X509.Roots
import Yt.Crypto.Rsa
import Yt.Crypto.Ec
/-! Server certificate chain validation for TLS: path building from the
leaf through the presented intermediates to a trust anchor, checking
signatures, validity at a caller-supplied UNIX time, basicConstraints
(cA, pathLenConstraint), keyUsage, extKeyUsage (serverAuth), unknown
critical extensions, and the server name against subjectAltName dNSNames
(with left-most-label wildcards; no common-name fallback). Name chaining
compares the DER of Names byte for byte, as webpki does. Not implemented:
revocation (CRL/OCSP), name constraints (a CA carrying them as a critical
extension is rejected as unknown-critical), policy constraints, IP-address
names. -/
namespace Yt.X509
open Yt.Crypto

/-- A trust anchor: subject Name and public key. -/
structure Anchor where
  subject : ByteArray
  key : PubKey
deriving Inhabited

def Anchor.ofDer (der : ByteArray) : Option Anchor :=
  (Cert.parse der).map fun c => ⟨c.subject, c.key⟩

/-- The compiled-in anchors (GTS Root R1 and R4, see `Roots`). -/
def defaultAnchors : Array Anchor := Roots.all.filterMap Anchor.ofDer

structure Policy where
  minRsaBits : Nat := 2048
  maxIntermediates : Nat := 6
  /-- check the server name against the leaf (tests of foreign traces only) -/
  checkName : Bool := true
deriving Inhabited

def rsaBits (n : ByteArray) : Nat := (Rsa.PublicKey.mk n ByteArray.empty).bits

/-- Verify `sig` over `msg` with `key` and the certificate signature
algorithm `alg`. -/
def verifySig (policy : Policy) (key : PubKey) (alg : SigAlg) (msg sig : ByteArray) : Bool :=
  match key, alg with
  | .rsa n e, .rsaPkcs1 h => rsaBits n ≥ policy.minRsaBits && Rsa.verifyPkcs1 ⟨n, e⟩ h (h.hash msg) sig
  | .p256 pt, .ecdsa h =>
    match Der.ecdsaSig sig with
    | some (r, s) => Ec.verify Ec.p256 pt (h.hash msg) r s
    | none => false
  | .p384 pt, .ecdsa h =>
    match Der.ecdsaSig sig with
    | some (r, s) => Ec.verify Ec.p384 pt (h.hash msg) r s
    | none => false
  | _, _ => false

def lowerAscii (b : ByteArray) : ByteArray := Id.run do
  let mut r := b
  for i in [0:b.size] do
    let c := b[i]!
    if c ≥ 65 && c ≤ 90 then r := r.set! i (c + 32)
  r

def countDots (b : ByteArray) (from_ : Nat) : Nat := Id.run do
  let mut n : Nat := 0
  for i in [from_:b.size] do
    if b[i]! == 46 then n := n + 1
  n

/-- Does the dNSName `pattern` cover `name` (both lower-case ASCII)?
A wildcard is only `*` as the entire left-most label, it matches exactly
one non-empty label, and at least two labels must follow it. -/
def nameMatches (pattern name : ByteArray) : Bool := Id.run do
  if pattern.size == 0 || name.size == 0 then return false
  for c in name do
    if c == 42 then return false
  if pattern == name then return true
  if pattern.size < 3 || pattern[0]! != 42 || pattern[1]! != 46 then return false
  -- suffix ".rest" of the pattern must have at least one more dot inside
  if countDots pattern 2 < 1 then return false
  let suffixLen := pattern.size - 1
  if name.size ≤ suffixLen then return false
  let labelLen := name.size - suffixLen
  for i in [0:labelLen] do
    let c := name[i]!
    if c == 46 || c == 42 then return false
  return bytesEqAt name labelLen pattern 1 suffixLen

def validAt (c : Cert) (now : UInt64) : Bool := c.notBefore ≤ now && now ≤ c.notAfter

def ekuAllows (c : Cert) (allowAny : Bool) : Bool :=
  match c.eku with
  | none => true
  | some ids => ids.any (fun o => o == Oid.serverAuth || (allowAny && o == Oid.anyEku))

/-- May `ca` issue certificates `below` intermediates above the leaf? -/
def caOk (c : Cert) (below : Nat) (now : UInt64) : Bool :=
  validAt c now && c.isCA && !c.unknownCritical
    && (match c.keyUsage with | some ku => ku &&& 0x20 != 0 | none => true)
    && ekuAllows c true
    && (match c.pathLen with | some n => below ≤ n | none => true)

/-- Cache key of one certificate-signature verification: SHA-256 over the
issuer key, the algorithm, the signed bytes and the signature. A verified
signature stays verified, so an application that keeps these digests
(see `verifyServerCached`) can skip the public-key arithmetic for chains
it has seen before; validity, name and constraint checks still run. -/
def sigDigest (key : PubKey) (alg : SigAlg) (msg sig : ByteArray) : ByteArray :=
  let lp (b : ByteArray) : ByteArray := pushU32BE ByteArray.empty b.size.toUInt32 ++ b
  let k := match key with
    | .rsa n e => (ByteArray.empty.push 1) ++ lp n ++ lp e
    | .p256 pt => (ByteArray.empty.push 2) ++ lp pt
    | .p384 pt => (ByteArray.empty.push 3) ++ lp pt
    | .unsupported => ByteArray.empty.push 0
  let a : UInt8 := match alg with
    | .rsaPkcs1 .sha256 => 1 | .rsaPkcs1 .sha384 => 2 | .rsaPkcs1 .sha512 => 3
    | .ecdsa .sha256 => 4 | .ecdsa .sha384 => 5 | .ecdsa .sha512 => 6 | .unsupported => 0
  Sha256.hash (ascii "yt-x509 signature v1" ++ k ++ (ByteArray.empty.push a) ++ lp msg ++ lp sig)

/-- `verifySig` through a cache of digests; the digest is added on success. -/
def verifySigCached (policy : Policy) (cache : Array ByteArray) (key : PubKey) (alg : SigAlg)
    (msg sig : ByteArray) : Bool × Array ByteArray :=
  let d := sigDigest key alg msg sig
  if cache.contains d then (true, cache)
  else if verifySig policy key alg msg sig then (true, cache.push d)
  else (false, cache)

/-- Can `cur` be chained to an anchor through `pool`? `below` counts the
intermediates already between `cur` and the leaf. Threads the signature
cache. -/
partial def chains (policy : Policy) (anchors : Array Anchor) (pool : Array Cert) (now : UInt64)
    (cur : Cert) (below : Nat) (cache : Array ByteArray) : Bool × Array ByteArray := Id.run do
  let mut cache := cache
  for a in anchors do
    if a.subject == cur.issuer then
      let (ok, c) := verifySigCached policy cache a.key cur.sigAlg cur.tbs cur.sig
      cache := c
      if ok then return (true, cache)
  if below ≥ policy.maxIntermediates then return (false, cache)
  for cand in pool do
    if cand.subject == cur.issuer && caOk cand below now then
      let (ok, c) := verifySigCached policy cache cand.key cur.sigAlg cur.tbs cur.sig
      cache := c
      if ok then
        let (ok2, c2) := chains policy anchors pool now cand (below + 1) cache
        cache := c2
        if ok2 then return (true, cache)
  return (false, cache)

def stripDot (b : ByteArray) : ByteArray :=
  if b.size > 0 && b[b.size - 1]! == 46 then slice b 0 (b.size - 1) else b

/-- Validate the server's chain (DER, leaf first) for `serverName` at UNIX
time `now`, given digests of signatures verified before (`sigDigest`):
the leaf's public key and the digest list extended with this chain's
signatures (keep at most the last `cacheLimit`). -/
def verifyServerCached (certs : Array ByteArray) (serverName : String) (now : UInt64)
    (anchors : Array Anchor := defaultAnchors) (policy : Policy := {}) (cache : Array ByteArray := #[])
    (cacheLimit : Nat := 32) : Except String (PubKey × Array ByteArray) := do
  if certs.size == 0 then throw "no certificate"
  let some leaf := Cert.parse certs[0]! | throw "unparsable leaf certificate"
  let mut pool : Array Cert := #[]
  for i in [1:certs.size] do
    -- unparsable extra certificates are ignored (they cannot be on the path)
    if let some c := Cert.parse certs[i]! then pool := pool.push c
  if !(validAt leaf now) then throw "leaf certificate expired or not yet valid"
  if leaf.unknownCritical then throw "leaf has an unknown critical extension"
  if leaf.isCA then throw "leaf is a CA certificate"
  if !(ekuAllows leaf false) then throw "leaf not valid for serverAuth"
  match leaf.keyUsage with
  | some ku => if ku &&& 1 == 0 then throw "leaf keyUsage lacks digitalSignature"
  | none => pure ()
  match leaf.key with
  | .unsupported => throw "unsupported leaf key"
  | .rsa n _ => if rsaBits n < policy.minRsaBits then throw "leaf RSA key too small"
  | _ => pure ()
  if policy.checkName then
    let name := stripDot (lowerAscii serverName.toUTF8)
    if !(leaf.dnsNames.any fun p => nameMatches (lowerAscii p) name) then
      throw "certificate does not match the server name"
  let (ok, cache) := chains policy anchors pool now leaf 0 cache
  if !ok then throw "no valid path to a trust anchor"
  -- keep the newest `cacheLimit` (Array.extract is not in the runtime shim)
  let mut kept : Array ByteArray := #[]
  for i in [cache.size - min cache.size cacheLimit:cache.size] do kept := kept.push cache[i]!
  return (leaf.key, kept)

/-- Validate the server's chain (DER, leaf first) for `serverName` at UNIX
time `now`; the leaf's public key on success. -/
def verifyServer (certs : Array ByteArray) (serverName : String) (now : UInt64)
    (anchors : Array Anchor := defaultAnchors) (policy : Policy := {}) : Except String PubKey :=
  (verifyServerCached certs serverName now anchors policy).map (·.1)

end Yt.X509
