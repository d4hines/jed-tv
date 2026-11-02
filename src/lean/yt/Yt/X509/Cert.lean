import Yt.X509.Der
import Yt.Crypto.Hash
/-! X.509 v3 certificate parsing (RFC 5280), keeping what server
authentication needs: the signed bytes and signature, names (as raw DER
for byte comparison), validity as UNIX seconds, the public key, and the
basicConstraints, keyUsage, extKeyUsage and subjectAltName extensions.
Unknown critical extensions are recorded so validation can reject them. -/
namespace Yt.X509
open Yt.Crypto Yt.X509.Der

-- DER contents of the object identifiers we recognise.
namespace Oid
def rsaEncryption := ofHex "2a864886f70d010101"
def sha256WithRSA := ofHex "2a864886f70d01010b"
def sha384WithRSA := ofHex "2a864886f70d01010c"
def sha512WithRSA := ofHex "2a864886f70d01010d"
def ecPublicKey := ofHex "2a8648ce3d0201"
def prime256v1 := ofHex "2a8648ce3d030107"
def secp384r1 := ofHex "2b81040022"
def ecdsaSha256 := ofHex "2a8648ce3d040302"
def ecdsaSha384 := ofHex "2a8648ce3d040303"
def ecdsaSha512 := ofHex "2a8648ce3d040304"
def basicConstraints := ofHex "551d13"
def keyUsage := ofHex "551d0f"
def extKeyUsage := ofHex "551d25"
def subjectAltName := ofHex "551d11"
def serverAuth := ofHex "2b06010505070301"
def anyEku := ofHex "551d2500"
end Oid

inductive PubKey where
  | rsa (n e : ByteArray)
  /-- uncompressed point 04 ‖ X ‖ Y -/
  | p256 (pt : ByteArray)
  | p384 (pt : ByteArray)
  | unsupported
deriving Inhabited

inductive SigAlg where
  | rsaPkcs1 (h : HashAlg)
  | ecdsa (h : HashAlg)
  | unsupported
deriving Inhabited

structure Cert where
  /-- the signed TBSCertificate, tag and length included -/
  tbs : ByteArray
  sigAlg : SigAlg
  sig : ByteArray
  issuer : ByteArray
  subject : ByteArray
  notBefore : UInt64
  notAfter : UInt64
  key : PubKey
  /-- basicConstraints cA -/
  isCA : Bool := false
  pathLen : Option Nat := none
  /-- keyUsage bits (bit i = KeyUsage bit i, digitalSignature = 0,
  keyCertSign = 5) if the extension is present -/
  keyUsage : Option UInt32 := none
  /-- extKeyUsage purposes if present -/
  eku : Option (Array ByteArray) := none
  dnsNames : Array ByteArray := #[]
  unknownCritical : Bool := false
deriving Inhabited

/-- Days since 1970-01-01 of a proleptic Gregorian date (year ≥ 1970). -/
def daysFromCivil (y m d : Nat) : Nat :=
  let y := if m ≤ 2 then y - 1 else y
  let era := y / 400
  let yoe := y - era * 400
  let mp := if m > 2 then m - 3 else m + 9
  let doy := (153 * mp + 2) / 5 + d - 1
  let doe := yoe * 365 + yoe / 4 - yoe / 100 + doy
  era * 146097 + doe - 719468

/-- UTCTime `YYMMDDHHMMSSZ` or GeneralizedTime `YYYYMMDDHHMMSSZ` as UNIX
seconds (dates before 1970 clamp to 0). -/
def parseTime (b : ByteArray) (e : Tlv) : Option UInt64 := Id.run do
  let digit (i : Nat) : Option Nat :=
    let c := b[e.off + i]!
    if c ≥ 48 && c ≤ 57 then some (c - 48).toNat else none
  let two (i : Nat) : Option Nat := do
    let a ← digit i
    let c ← digit (i + 1)
    some (a * 10 + c)
  let (yr, o) ← if e.tag == Tag.utcTime && e.len == 13 then
      match two 0 with
      | some yy => (some (if yy ≥ 50 then 1900 + yy else 2000 + yy), 2)
      | none => (none, 0)
    else if e.tag == Tag.generalizedTime && e.len == 15 then
      match two 0, two 2 with
      | some a, some c => (some (a * 100 + c), 4)
      | _, _ => (none, 0)
    else (none, 0)
  let some y := yr | return none
  if b[e.off + e.len - 1]! != 90 then return none  -- 'Z'
  let some mo := two o | return none
  let some d := two (o + 2) | return none
  let some h := two (o + 4) | return none
  let some mi := two (o + 6) | return none
  let some s := two (o + 8) | return none
  if mo < 1 || mo > 12 || d < 1 || d > 31 || h > 23 || mi > 59 || s > 59 then return none
  if y < 1970 then return some 0
  let days := daysFromCivil y mo d
  return some (days.toUInt64 * 86400 + (h * 3600 + mi * 60 + s).toUInt64)

/-- An AlgorithmIdentifier's OID contents and whether its parameters are
absent or NULL. -/
def algId (b : ByteArray) (e : Tlv) : Option (ByteArray × Nat) := do
  if e.tag != Tag.sequence then none
  let o ← expect b e.off e.stop Tag.oid
  some (contents b o, o.stop)

def parseSigAlg (b : ByteArray) (e : Tlv) : SigAlg := Id.run do
  let some (oid, p) := algId b e | return .unsupported
  let paramsNullOrAbsent : Bool :=
    p == e.stop || (match read b p e.stop with
      | some n => n.tag == Tag.null && n.len == 0 && n.stop == e.stop
      | none => false)
  if oid == Oid.ecdsaSha256 then return (if p == e.stop then .ecdsa .sha256 else .unsupported)
  if oid == Oid.ecdsaSha384 then return (if p == e.stop then .ecdsa .sha384 else .unsupported)
  if oid == Oid.ecdsaSha512 then return (if p == e.stop then .ecdsa .sha512 else .unsupported)
  if !paramsNullOrAbsent then return .unsupported
  if oid == Oid.sha256WithRSA then return .rsaPkcs1 .sha256
  if oid == Oid.sha384WithRSA then return .rsaPkcs1 .sha384
  if oid == Oid.sha512WithRSA then return .rsaPkcs1 .sha512
  return .unsupported

/-- SubjectPublicKeyInfo. -/
def parseSpki (b : ByteArray) (e : Tlv) : PubKey := Id.run do
  if e.tag != Tag.sequence then return .unsupported
  let some a := expect b e.off e.stop Tag.sequence | return .unsupported
  let some (oid, p) := algId b a | return .unsupported
  let some kb := expect b a.stop e.stop Tag.bitString | return .unsupported
  if kb.stop != e.stop then return .unsupported
  let some key := bits b kb | return .unsupported
  if oid == Oid.rsaEncryption then
    let some sq := expect key 0 key.size Tag.sequence | return .unsupported
    let some ne := expect key sq.off sq.stop Tag.integer | return .unsupported
    let some ee := expect key ne.stop sq.stop Tag.integer | return .unsupported
    if ee.stop != sq.stop || sq.stop != key.size then return .unsupported
    let some n := uint key ne | return .unsupported
    let some ex := uint key ee | return .unsupported
    return .rsa n ex
  if oid == Oid.ecPublicKey then
    let some c := expect b p a.stop Tag.oid | return .unsupported
    let curve := contents b c
    if curve == Oid.prime256v1 && key.size == 65 && key[0]! == 4 then return .p256 key
    if curve == Oid.secp384r1 && key.size == 97 && key[0]! == 4 then return .p384 key
  return .unsupported

/-- KeyUsage BIT STRING (unused bits allowed) as a bit set. -/
def parseKeyUsage (b : ByteArray) : Option UInt32 := do
  let e ← expect b 0 b.size Tag.bitString
  if e.len < 2 || e.len > 3 || b[e.off]! > 7 then none
  let mut v : UInt32 := 0
  let b1 := b[e.off + 1]!
  for i in [0:8] do
    if (b1 >>> (7 - i).toUInt8) &&& 1 == 1 then v := v ||| ((1 : UInt32) <<< i.toUInt32)
  if e.len == 3 && b[e.off + 2]! &&& 0x80 != 0 then v := v ||| 0x100
  some v

/-- Apply one extension to the certificate being built. -/
def applyExt (c : Cert) (oid : ByteArray) (critical : Bool) (v : ByteArray) : Option Cert := do
  if oid == Oid.basicConstraints then
    let sq ← expect v 0 v.size Tag.sequence
    if sq.stop != v.size then none
    let mut p := sq.off
    let mut ca := false
    let mut pl : Option Nat := none
    if p < sq.stop then
      if let some e := read v p sq.stop then
        if e.tag == Tag.boolean then
          if e.len != 1 then none
          ca := v[e.off]! == 0xff
          p := e.stop
    if p < sq.stop then
      let e ← expect v p sq.stop Tag.integer
      let n ← uint v e
      if n.size > 1 || e.stop != sq.stop then none
      pl := some (if n.size == 0 then 0 else n[0]!.toNat)
    else if p != sq.stop then none
    some { c with isCA := ca, pathLen := pl }
  else if oid == Oid.keyUsage then
    let ku ← parseKeyUsage v
    some { c with keyUsage := some ku }
  else if oid == Oid.extKeyUsage then
    let sq ← expect v 0 v.size Tag.sequence
    let kids ← children v sq
    let mut ids : Array ByteArray := #[]
    for (_, k) in kids do
      if k.tag != Tag.oid then none
      ids := ids.push (contents v k)
    some { c with eku := some ids }
  else if oid == Oid.subjectAltName then
    let sq ← expect v 0 v.size Tag.sequence
    let kids ← children v sq
    let mut names := c.dnsNames
    for (_, k) in kids do
      if k.tag == Tag.ctxPrim 2 then names := names.push (contents v k)
    some { c with dnsNames := names }
  else
    some (if critical then { c with unknownCritical := true } else c)

/-- Parse a DER certificate. -/
def Cert.parse (der : ByteArray) : Option Cert := do
  let top ← expect der 0 der.size Tag.sequence
  if top.stop != der.size then none
  let tbsE ← expect der top.off top.stop Tag.sequence
  let algE ← expect der tbsE.stop top.stop Tag.sequence
  let sigE ← expect der algE.stop top.stop Tag.bitString
  if sigE.stop != top.stop then none
  let sig ← bits der sigE
  -- TBSCertificate
  let mut p := tbsE.off
  let lim := tbsE.stop
  let mut version := 0
  let v0 ← read der p lim
  if v0.tag == Tag.ctx 0 then
    let vi ← expect der v0.off v0.stop Tag.integer
    let vb ← uint der vi
    if vb.size != 1 || vi.stop != v0.stop then none
    version := vb[0]!.toNat
    p := v0.stop
  let serial ← expect der p lim Tag.integer
  let innerAlg ← expect der serial.stop lim Tag.sequence
  if raw der serial.stop innerAlg != raw der tbsE.stop algE then none
  let issuerE ← expect der innerAlg.stop lim Tag.sequence
  let validity ← expect der issuerE.stop lim Tag.sequence
  let subjectE ← expect der validity.stop lim Tag.sequence
  let spkiE ← expect der subjectE.stop lim Tag.sequence
  let nbE ← read der validity.off validity.stop
  let naE ← read der nbE.stop validity.stop
  if naE.stop != validity.stop then none
  let nb ← parseTime der nbE
  let na ← parseTime der naE
  let mut c : Cert := {
    tbs := raw der top.off tbsE
    sigAlg := parseSigAlg der algE
    sig := sig
    issuer := raw der innerAlg.stop issuerE
    subject := raw der validity.stop subjectE
    notBefore := nb
    notAfter := na
    key := parseSpki der spkiE }
  p := spkiE.stop
  -- skip issuerUniqueID [1] / subjectUniqueID [2]
  while p < lim do
    let e ← read der p lim
    if e.tag == Tag.ctxPrim 1 || e.tag == Tag.ctxPrim 2 || e.tag == 0xa1 || e.tag == 0xa2 then p := e.stop
    else break
  if p < lim then
    let x ← expect der p lim (Tag.ctx 3)
    if x.stop != lim || version != 2 then none
    let exts ← expect der x.off x.stop Tag.sequence
    if exts.stop != x.stop then none
    let kids ← children der exts
    let mut seen : Array ByteArray := #[]
    for (_, ext) in kids do
      if ext.tag != Tag.sequence then none
      let oidE ← expect der ext.off ext.stop Tag.oid
      let oid := contents der oidE
      if seen.contains oid then none
      seen := seen.push oid
      let mut q := oidE.stop
      let mut critical := false
      let nx ← read der q ext.stop
      if nx.tag == Tag.boolean then
        if nx.len != 1 then none
        critical := der[nx.off]! == 0xff
        q := nx.stop
      let val ← expect der q ext.stop Tag.octetString
      if val.stop != ext.stop then none
      c ← applyExt c oid critical (contents der val)
  some c

end Yt.X509
