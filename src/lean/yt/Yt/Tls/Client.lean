import Yt.Tls.Keys
import Yt.X509.Verify
import Yt.Crypto.X25519
/-! A TLS 1.3 client (RFC 8446) as a pure, caller-driven state machine: no
sockets, clocks or randomness of its own. The caller supplies entropy and
the UNIX time in `Config`, writes the bytes the client returns to the
transport, and feeds back whatever bytes arrive.

    let (c, hello) := Tls.start cfg            -- send `hello`
    let r := Tls.feed c received               -- send r.send, use r.data
    let (c, recs) := Tls.send r.client bytes   -- once r.status = connected
    let (c, bye) := Tls.close c                -- close_notify

Supported: TLS 1.3 only; TLS_AES_128_GCM_SHA256, TLS_CHACHA20_POLY1305_SHA256
and TLS_AES_256_GCM_SHA384; x25519 key exchange; HelloRetryRequest
carrying a cookie (a request for any other group fails, x25519 being the
only group offered); SNI; ALPN; server certificates signed with
ecdsa_secp256r1_sha256, ecdsa_secp384r1_sha384 or rsa_pss_rsae_sha256/384/512
(certificate chains may also use rsa_pkcs1_sha256/384/512); an empty
client Certificate if the server asks for one; KeyUpdate; NewSessionTicket
is accepted and ignored. Not supported: resumption/PSK, 0-RTT, client
certificates, TLS 1.2 and below, post-handshake authentication,
record_size_limit and max_fragment_length. -/
namespace Yt.Tls
open Yt.Crypto Yt.X509

namespace Alert
def closeNotify : UInt8 := 0
def unexpectedMessage : UInt8 := 10
def badRecordMac : UInt8 := 20
def recordOverflow : UInt8 := 22
def handshakeFailure : UInt8 := 40
def badCertificate : UInt8 := 42
def certificateExpired : UInt8 := 45
def illegalParameter : UInt8 := 47
def unknownCa : UInt8 := 48
def decodeError : UInt8 := 50
def decryptError : UInt8 := 51
def protocolVersion : UInt8 := 70
def internalError : UInt8 := 80
def userCanceled : UInt8 := 90
def missingExtension : UInt8 := 109
def unsupportedExtension : UInt8 := 110
def noApplicationProtocol : UInt8 := 120
end Alert

namespace Ext
def serverName : Nat := 0
def supportedGroups : Nat := 10
def signatureAlgorithms : Nat := 13
def alpn : Nat := 16
def preSharedKey : Nat := 41
def supportedVersions : Nat := 43
def cookie : Nat := 44
def keyShare : Nat := 51
end Ext

/-- Signature schemes offered, in preference order. The rsa_pkcs1 schemes
are for certificate signatures only (RFC 8446 section 4.2.3). -/
def offeredSchemes : Array Nat := #[0x0403, 0x0503, 0x0804, 0x0805, 0x0806, 0x0401, 0x0501, 0x0601]

/-- The 32-byte random of a HelloRetryRequest. -/
def hrrRandom : ByteArray := ofHex "cf21ad74e59a6111be1d8c021e65b891c2a211167abb8c5e079e09e2c8a8339c"

/-- Cipher suites in default preference order. ChaCha20-Poly1305 first: in
pure Lean it is about 3x faster than table-driven AES-GCM (host benchmark)
and uses no secret-indexed tables. -/
def defaultSuites : Array Suite := #[.chacha20, .aes128gcm, .aes256gcm]

structure Config where
  /-- DNS name of the server (SNI and certificate check) -/
  serverName : String
  /-- ALPN protocols to offer (empty: no ALPN extension) -/
  alpn : Array String := #["http/1.1"]
  /-- current UNIX time in seconds, for certificate validity -/
  unixTime : UInt64
  /-- at least 32 bytes from the OS random source -/
  entropy : ByteArray
  anchors : Array Anchor := defaultAnchors
  policy : Policy := {}
  /-- cipher suites in preference order -/
  suites : Array Suite := defaultSuites
  /-- digests of certificate signatures verified on earlier connections
  (`Client.sigCache` of a previous client); lets repeated handshakes to
  the same hosts skip the chain's public-key arithmetic -/
  sigCache : Array ByteArray := #[]
deriving Inhabited

inductive Phase where
  | waitServerHello | waitEncryptedExtensions | waitCertOrRequest | waitCertificate
  | waitCertVerify | waitFinished | connected | closed | failed
deriving BEq, Inhabited

inductive Status where
  | handshaking
  | connected
  /-- the server sent close_notify -/
  | closed
  | failed (reason : String)
deriving Inhabited, BEq

structure Client where
  cfg : Config
  phase : Phase := .waitServerHello
  error : String := ""
  priv : ByteArray := .empty
  random : ByteArray := .empty
  sessionId : ByteArray := .empty
  pub : ByteArray := .empty
  /-- built by `start` (so a HelloRetryRequest can rebuild it) -/
  canRebuild : Bool := false
  offeredSuites : Array Nat := #[]
  offeredExts : Array Nat := #[]
  offeredSchemes : Array Nat := #[]
  offeredAlpn : Array ByteArray := #[]
  transcript : ByteArray := .empty
  hrr : Bool := false
  suite : Suite := .aes128gcm
  master : ByteArray := .empty
  cHs : ByteArray := .empty
  sHs : ByteArray := .empty
  certs : Array ByteArray := #[]
  leafKey : PubKey := .unsupported
  /-- `Config.sigCache` plus this handshake's verified chain signatures:
  pass it on in the next connection's `Config` -/
  sigCache : Array ByteArray := #[]
  certRequested : Bool := false
  certReqContext : ByteArray := .empty
  alpn : String := ""
  inBuf : ByteArray := .empty
  hsBuf : ByteArray := .empty
  rd : Option Cipher := none
  wr : Option Cipher := none
  /-- send a dummy change_cipher_spec before the second flight
  (middlebox compatibility, RFC 8446 appendix D.4) -/
  compat : Bool := false
  sentClose : Bool := false
  /-- bytes to send, collected during one call -/
  out : ByteArray := .empty
  /-- application data received during one call -/
  appData : ByteArray := .empty
deriving Inhabited

structure Fail where
  alert : UInt8
  msg : String

abbrev M := Except Fail

def fail (alert : UInt8) (msg : String) : M α := throw ⟨alert, msg⟩

/-! ## Wire helpers -/

@[inline] def u8 (b : ByteArray) (p : Nat) : Nat := b[p]!.toNat
@[inline] def u16 (b : ByteArray) (p : Nat) : Nat := (b[p]!.toNat <<< 8) ||| b[p+1]!.toNat
@[inline] def u24 (b : ByteArray) (p : Nat) : Nat :=
  (b[p]!.toNat <<< 16) ||| (b[p+1]!.toNat <<< 8) ||| b[p+2]!.toNat

/-- A length-prefixed vector at `p` (prefix of `w` bytes) ending by `lim`:
its contents' start and end. -/
def vec (b : ByteArray) (p lim w : Nat) : M (Nat × Nat) := do
  if p + w > lim then fail Alert.decodeError "truncated length"
  let n := if w == 1 then u8 b p else if w == 2 then u16 b p else u24 b p
  if p + w + n > lim then fail Alert.decodeError "truncated vector"
  return (p + w, p + w + n)

def vec8 (body : ByteArray) : ByteArray := (ByteArray.empty.push body.size.toUInt8) ++ body
def vec16 (body : ByteArray) : ByteArray := pushU16BE ByteArray.empty body.size ++ body
def ext (type : Nat) (body : ByteArray) : ByteArray := pushU16BE ByteArray.empty type ++ vec16 body
def hsMsg (type : UInt8) (body : ByteArray) : ByteArray := pushU24BE (ByteArray.empty.push type) body.size ++ body

/-- A plaintext record (legacy version `ver`). -/
def plainRecord (type : UInt8) (ver : Nat) (body : ByteArray) : ByteArray :=
  pushU16BE (pushU16BE (ByteArray.empty.push type) ver) body.size ++ body

/-- Extensions of an extension block `[p, lim)`: type and contents range;
duplicates are rejected. -/
def exts (b : ByteArray) (p lim : Nat) : M (Array (Nat × Nat × Nat)) := do
  let (s, e) ← vec b p lim 2
  if e != lim then fail Alert.decodeError "trailing bytes after extensions"
  let mut out : Array (Nat × Nat × Nat) := #[]
  let mut q := s
  while q < e do
    if q + 4 > e then fail Alert.decodeError "truncated extension"
    let t := u16 b q
    let (cs, ce) ← vec b (q + 2) e 2
    if out.any (fun x => x.1 == t) then fail Alert.illegalParameter "duplicate extension"
    out := out.push (t, cs, ce)
    q := ce
  return out

/-! ## ClientHello -/

/-- Is `s` a plausible DNS host name (what SNI may carry)? -/
def validHostName (s : ByteArray) : Bool := Id.run do
  if s.size == 0 || s.size > 253 then return false
  for c in s do
    let ok := (c ≥ 97 && c ≤ 122) || (c ≥ 48 && c ≤ 57) || c == 45 || c == 46
    if !ok then return false
  return true

def hostBytes (cfg : Config) : ByteArray := stripDot (lowerAscii cfg.serverName.toUTF8)

def buildClientHello (cfg : Config) (random sid pub cookie : ByteArray) : ByteArray := Id.run do
  let host := hostBytes cfg
  let mut suites := ByteArray.empty
  for s in cfg.suites do suites := pushU16BE suites s.code
  let mut sigs := ByteArray.empty
  for s in offeredSchemes do sigs := pushU16BE sigs s
  let mut e := ext Ext.serverName (vec16 ((ByteArray.empty.push 0) ++ vec16 host))
  e := e ++ ext Ext.supportedGroups (vec16 (pushU16BE .empty 0x001d))
  e := e ++ ext Ext.signatureAlgorithms (vec16 sigs)
  e := e ++ ext Ext.supportedVersions (vec8 (pushU16BE .empty 0x0304))
  e := e ++ ext Ext.keyShare (vec16 (pushU16BE .empty 0x001d ++ vec16 pub))
  if cfg.alpn.size > 0 then
    let mut names := ByteArray.empty
    for a in cfg.alpn do names := names ++ vec8 a.toUTF8
    e := e ++ ext Ext.alpn (vec16 names)
  if cookie.size > 0 then e := e ++ ext Ext.cookie (vec16 cookie)
  let body := pushU16BE .empty 0x0303 ++ random ++ vec8 sid ++ vec16 suites ++ vec8 (ByteArray.empty.push 0) ++ vec16 e
  hsMsg 1 body

/-- Read back what a ClientHello message offers. -/
def offers (c : Client) (ch : ByteArray) : M Client := do
  if ch.size < 4 || ch[0]! != 1 || u24 ch 1 + 4 != ch.size then fail Alert.internalError "bad ClientHello"
  let lim := ch.size
  let random := slice ch 6 32
  let (ss, se) ← vec ch 38 lim 1
  let (cs, ce) ← vec ch se lim 2
  let mut suites := #[]
  for i in [0:(ce - cs) / 2] do suites := suites.push (u16 ch (cs + 2 * i))
  let (_, me) ← vec ch ce lim 1
  let xs ← exts ch me lim
  let mut schemes := #[]
  let mut alpn := #[]
  for (t, s, e) in xs do
    if t == Ext.signatureAlgorithms then
      let (ls, le) ← vec ch s e 2
      for i in [0:(le - ls) / 2] do schemes := schemes.push (u16 ch (ls + 2 * i))
    if t == Ext.alpn then
      let (ls, le) ← vec ch s e 2
      let mut q := ls
      while q < le do
        let (ns, ne) ← vec ch q le 1
        alpn := alpn.push (slice ch ns (ne - ns))
        q := ne
  return { c with random := random, sessionId := slice ch ss (se - ss), offeredSuites := suites,
                  offeredExts := xs.map (·.1), offeredSchemes := schemes, offeredAlpn := alpn }

/-! ## Records -/

def Client.alertRecord (c : Client) (desc : UInt8) (level : UInt8 := 2) : Client :=
  let body := (ByteArray.empty.push level).push desc
  match c.wr with
  | some w => let (w, r) := w.seal 21 body; { c with wr := some w, out := c.out ++ r }
  | none => { c with out := c.out ++ plainRecord 21 0x0303 body }

/-- Queue a handshake message under the current write protection. -/
def Client.sendHandshake (c : Client) (msg : ByteArray) : Client :=
  match c.wr with
  | some w => let (w, r) := w.seal 22 msg; { c with wr := some w, out := c.out ++ r }
  | none => { c with out := c.out ++ plainRecord 22 0x0303 msg }

def Client.status (c : Client) : Status :=
  match c.phase with
  | .connected => .connected
  | .closed => .closed
  | .failed => .failed c.error
  | _ => .handshaking

/-- The transcript hash under the negotiated suite. -/
def Client.th (c : Client) : ByteArray := c.suite.hash.hash c.transcript

/-! ## Handshake messages -/

def handleServerHello (c : Client) (msg : ByteArray) : M Client := do
  let lim := msg.size
  if lim < 4 + 38 then fail Alert.decodeError "short ServerHello"
  let p := 4
  if u16 msg p != 0x0303 then fail Alert.protocolVersion "not TLS 1.3"
  let random := slice msg (p + 2) 32
  let (ss, se) ← vec msg (p + 34) lim 1
  if !(bytesEqAt msg ss c.sessionId 0 c.sessionId.size) || se - ss != c.sessionId.size then
    fail Alert.illegalParameter "session id not echoed"
  if se + 3 > lim then fail Alert.decodeError "short ServerHello"
  let code := u16 msg se
  let some suite := Suite.ofCode? code | fail Alert.illegalParameter "unknown cipher suite"
  if !(c.offeredSuites.contains code) then fail Alert.illegalParameter "cipher suite not offered"
  if c.hrr && suite != c.suite then fail Alert.illegalParameter "cipher suite changed after HelloRetryRequest"
  if u8 msg (se + 2) != 0 then fail Alert.illegalParameter "compression"
  let xs ← exts msg (se + 3) lim
  let isHrr := random == hrrRandom
  let mut version := false
  let mut serverKey := ByteArray.empty
  let mut cookie := ByteArray.empty
  for (t, s, e) in xs do
    if t == Ext.supportedVersions then
      if e - s != 2 || u16 msg s != 0x0304 then fail Alert.illegalParameter "supported_versions"
      version := true
    else if t == Ext.keyShare then
      if isHrr then
        if e - s != 2 then fail Alert.decodeError "key_share"
        -- x25519 was already offered; we offer no other group
        fail Alert.illegalParameter "HelloRetryRequest asks for an unsupported group"
      if e - s < 4 || u16 msg s != 0x001d then fail Alert.illegalParameter "key_share group"
      let (ks, ke) ← vec msg (s + 2) e 2
      if ke != e || ke - ks != 32 then fail Alert.illegalParameter "x25519 key length"
      serverKey := slice msg ks 32
    else if t == Ext.cookie && isHrr then
      let (ks, ke) ← vec msg s e 2
      if ke != e || ke == ks then fail Alert.decodeError "cookie"
      cookie := slice msg ks (ke - ks)
    else if c.offeredExts.contains t then
      fail Alert.illegalParameter "extension not allowed in ServerHello"
    else fail Alert.unsupportedExtension "unsolicited extension"
  if !version then fail Alert.protocolVersion "server does not speak TLS 1.3"
  if isHrr then
    if c.hrr then fail Alert.unexpectedMessage "second HelloRetryRequest"
    if cookie.size == 0 then fail Alert.illegalParameter "HelloRetryRequest changes nothing"
    if !c.canRebuild then fail Alert.internalError "cannot rebuild ClientHello"
    let h := suite.hash
    -- transcript: message_hash(ClientHello1) ‖ HelloRetryRequest ‖ ClientHello2
    let ch1 := c.transcript
    let mh := hsMsg 254 (h.hash ch1)
    let ch2 := buildClientHello c.cfg c.random c.sessionId c.pub cookie
    let c := { c with hrr := true, suite := suite, transcript := mh ++ msg ++ ch2,
                      offeredExts := c.offeredExts.push Ext.cookie }
    let c := if c.compat then { c with out := c.out ++ plainRecord 20 0x0303 (ByteArray.empty.push 1) } else c
    return { c with out := c.out ++ plainRecord 22 0x0303 ch2 }
  if serverKey.size == 0 then fail Alert.missingExtension "no key_share"
  let shared := X25519.x25519 c.priv serverKey
  if ctEq shared (zeros 32) then fail Alert.illegalParameter "x25519 shared secret is zero"
  let h := suite.hash
  -- the private key is not needed after this point
  let c := { c with suite := suite, transcript := c.transcript ++ msg, priv := .empty }
  let th := c.th
  let early := hkdfExtract h ByteArray.empty (zeros h.outLen)
  let empty := h.hash ByteArray.empty
  let hs := hkdfExtract h (deriveSecret h early "derived" empty) shared
  let cHs := deriveSecret h hs "c hs traffic" th
  let sHs := deriveSecret h hs "s hs traffic" th
  let master := hkdfExtract h (deriveSecret h hs "derived" empty) (zeros h.outLen)
  return { c with cHs := cHs, sHs := sHs, master := master, phase := .waitEncryptedExtensions,
                  rd := some (Cipher.ofSecret suite sHs), wr := some (Cipher.ofSecret suite cHs) }

def handleEncryptedExtensions (c : Client) (msg : ByteArray) : M Client := do
  let xs ← exts msg 4 msg.size
  let mut alpn := ""
  for (t, s, e) in xs do
    if !(c.offeredExts.contains t) then fail Alert.unsupportedExtension "unsolicited extension"
    if t == Ext.keyShare || t == Ext.supportedVersions || t == Ext.preSharedKey || t == Ext.cookie
        || t == Ext.signatureAlgorithms then
      fail Alert.illegalParameter "extension not allowed in EncryptedExtensions"
    if t == Ext.serverName && e != s then fail Alert.decodeError "server_name response not empty"
    if t == Ext.alpn then
      let (ls, le) ← vec msg s e 2
      let (ns, ne) ← vec msg ls le 1
      if le != e || ne != le || ne == ns then fail Alert.decodeError "ALPN response"
      let name := slice msg ns (ne - ns)
      if !(c.offeredAlpn.contains name) then fail Alert.illegalParameter "ALPN protocol not offered"
      alpn := (String.fromUTF8? name).getD ""
  return { c with transcript := c.transcript ++ msg, alpn := alpn, phase := .waitCertOrRequest }

def handleCertificateRequest (c : Client) (msg : ByteArray) : M Client := do
  let (cs, ce) ← vec msg 4 msg.size 1
  let _ ← exts msg ce msg.size
  return { c with transcript := c.transcript ++ msg, certRequested := true,
                  certReqContext := slice msg cs (ce - cs), phase := .waitCertificate }

def chainAlert (e : String) : UInt8 :=
  if e == "leaf certificate expired or not yet valid" then Alert.certificateExpired
  else if e == "no valid path to a trust anchor" then Alert.unknownCa
  else Alert.badCertificate

def handleCertificate (c : Client) (msg : ByteArray) : M Client := do
  let lim := msg.size
  let (cs, ce) ← vec msg 4 lim 1
  if ce != cs then fail Alert.decodeError "server certificate_request_context not empty"
  let (ls, le) ← vec msg ce lim 3
  if le != lim then fail Alert.decodeError "trailing bytes after certificate list"
  let mut certs : Array ByteArray := #[]
  let mut q := ls
  while q < le do
    let (ds, de) ← vec msg q le 3
    if de == ds then fail Alert.decodeError "empty certificate"
    let (_, xe) ← vec msg de le 2
    certs := certs.push (slice msg ds (de - ds))
    q := xe
  if certs.size == 0 then fail Alert.decodeError "empty certificate list"
  match verifyServerCached certs c.cfg.serverName c.cfg.unixTime c.cfg.anchors c.cfg.policy c.cfg.sigCache with
  | .error e => fail (chainAlert e) e
  | .ok (key, cache) => return { c with transcript := c.transcript ++ msg, certs := certs, leafKey := key,
                                         sigCache := cache, phase := .waitCertVerify }

/-- Check a CertificateVerify signature over `content` with the leaf key. -/
def verifyScheme (c : Client) (scheme : Nat) (content sig : ByteArray) : Bool :=
  match scheme, c.leafKey with
  | 0x0403, .p256 pt =>
    match Der.ecdsaSig sig with
    | some (r, s) => Ec.verify Ec.p256 pt (Sha256.hash content) r s
    | none => false
  | 0x0503, .p384 pt =>
    match Der.ecdsaSig sig with
    | some (r, s) => Ec.verify Ec.p384 pt (Sha512.hash384 content) r s
    | none => false
  | 0x0804, .rsa n e => rsaBits n ≥ c.cfg.policy.minRsaBits && Rsa.verifyPss ⟨n, e⟩ .sha256 (Sha256.hash content) sig
  | 0x0805, .rsa n e => rsaBits n ≥ c.cfg.policy.minRsaBits && Rsa.verifyPss ⟨n, e⟩ .sha384 (Sha512.hash384 content) sig
  | 0x0806, .rsa n e => rsaBits n ≥ c.cfg.policy.minRsaBits && Rsa.verifyPss ⟨n, e⟩ .sha512 (Sha512.hash content) sig
  | _, _ => false

def handleCertificateVerify (c : Client) (msg : ByteArray) : M Client := do
  let lim := msg.size
  if lim < 8 then fail Alert.decodeError "short CertificateVerify"
  let scheme := u16 msg 4
  let (ss, se) ← vec msg 6 lim 2
  if se != lim then fail Alert.decodeError "trailing bytes after CertificateVerify"
  if !(c.offeredSchemes.contains scheme) || scheme == 0x0401 || scheme == 0x0501 || scheme == 0x0601 then
    fail Alert.illegalParameter "signature scheme not offered"
  let mut content := ByteArray.emptyWithCapacity 200
  for _ in [0:64] do content := content.push 0x20
  content := content ++ ascii "TLS 1.3, server CertificateVerify" |>.push 0
  content := content ++ c.th
  if !(verifyScheme c scheme content (slice msg ss (se - ss))) then
    fail Alert.decryptError "CertificateVerify signature does not verify"
  return { c with transcript := c.transcript ++ msg, phase := .waitFinished }

def handleFinished (c : Client) (msg : ByteArray) : M Client := do
  let h := c.suite.hash
  if msg.size != 4 + h.outLen then fail Alert.decodeError "Finished length"
  let fk := expandLabel h c.sHs "finished" ByteArray.empty h.outLen
  let want := hmac h fk c.th
  if !(ctEq want (slice msg 4 h.outLen)) then fail Alert.decryptError "server Finished does not verify"
  let c := { c with transcript := c.transcript ++ msg }
  let th := c.th
  let cAp := deriveSecret h c.master "c ap traffic" th
  let sAp := deriveSecret h c.master "s ap traffic" th
  -- second flight: [change_cipher_spec] [Certificate] Finished
  let c := if c.compat then { c with out := c.out ++ plainRecord 20 0x0303 (ByteArray.empty.push 1) } else c
  let c := if c.certRequested then
      let cert := hsMsg 11 (vec8 c.certReqContext ++ pushU24BE .empty 0)
      { (c.sendHandshake cert) with transcript := c.transcript ++ cert }
    else c
  let cfk := expandLabel h c.cHs "finished" ByteArray.empty h.outLen
  let fin := hsMsg 20 (hmac h cfk c.th)
  let c := c.sendHandshake fin
  return { c with transcript := c.transcript ++ fin, phase := .connected,
                  rd := some (Cipher.ofSecret c.suite sAp), wr := some (Cipher.ofSecret c.suite cAp),
                  cHs := .empty, sHs := .empty, master := .empty }

def handleKeyUpdate (c : Client) (msg : ByteArray) : M Client := do
  if msg.size != 5 then fail Alert.decodeError "KeyUpdate length"
  let req := msg[4]!
  if req > 1 then fail Alert.illegalParameter "KeyUpdate request"
  let c := { c with rd := c.rd.map Cipher.update }
  if req == 1 then
    let c := c.sendHandshake (hsMsg 24 (ByteArray.empty.push 0))
    return { c with wr := c.wr.map Cipher.update }
  return c

/-- Dispatch one complete handshake message; whether it changed keys (so
it must end its record). -/
def handleMessage (c : Client) (msg : ByteArray) : M (Client × Bool) := do
  let t := msg[0]!
  match c.phase, t with
  | .waitServerHello, 2 => do
    let c ← handleServerHello c msg
    return (c, true)
  | .waitEncryptedExtensions, 8 => return (← handleEncryptedExtensions c msg, false)
  | .waitCertOrRequest, 13 => return (← handleCertificateRequest c msg, false)
  | .waitCertOrRequest, 11 => return (← handleCertificate c msg, false)
  | .waitCertificate, 11 => return (← handleCertificate c msg, false)
  | .waitCertVerify, 15 => return (← handleCertificateVerify c msg, false)
  | .waitFinished, 20 => return (← handleFinished c msg, true)
  | .connected, 4 => return (c, false)
  | .connected, 24 => return (← handleKeyUpdate c msg, true)
  | _, _ => fail Alert.unexpectedMessage ("unexpected handshake message " ++ toString t.toNat)

/-- Largest handshake message accepted (certificate chains). -/
def maxHandshake : Nat := 0x20000

partial def drainHandshake (c : Client) : M Client := do
  let b := c.hsBuf
  if b.size < 4 then return c
  let len := u24 b 1
  if len > maxHandshake then fail Alert.decodeError "handshake message too large"
  if b.size < 4 + len then return c
  let msg := slice b 0 (4 + len)
  let rest := slice b (4 + len) (b.size - 4 - len)
  let (c, rekey) ← handleMessage { c with hsBuf := rest } msg
  if rekey && rest.size > 0 then fail Alert.unexpectedMessage "handshake data after a key change"
  if c.phase == .closed || c.phase == .failed then return c
  drainHandshake c

def handleAlert (c : Client) (body : ByteArray) : M Client := do
  if body.size != 2 then fail Alert.decodeError "alert length"
  let desc := body[1]!
  if desc == Alert.closeNotify then return { c with phase := .closed }
  if desc == Alert.userCanceled then return c
  throw ⟨255, "server sent alert " ++ toString desc.toNat⟩

def inHandshake (c : Client) : Bool :=
  c.phase != .connected && c.phase != .closed && c.phase != .failed

/-- Process the record at `pos` (complete, body length `len`). -/
def handleRecord (c : Client) (b : ByteArray) (pos len : Nat) : M Client := do
  let type := b[pos]!
  if type == 20 then
    if len == 1 && b[pos + 5]! == 1 && inHandshake c then return c
    fail Alert.unexpectedMessage "change_cipher_spec"
  match c.rd with
  | none =>
    if len == 0 then fail Alert.decodeError "empty record"
    let body := slice b (pos + 5) len
    if type == 22 then drainHandshake { c with hsBuf := c.hsBuf ++ body }
    else if type == 21 then handleAlert c body
    else fail Alert.unexpectedMessage "unprotected record"
  | some r =>
    if type != 23 then fail Alert.unexpectedMessage "unprotected record after key exchange"
    let some (r, inner, body) := r.unseal b pos len | fail Alert.badRecordMac "record does not decrypt"
    if body.size > 16384 then fail Alert.recordOverflow "record too large"
    let c := { c with rd := some r }
    if inner == 22 then
      if body.size == 0 then fail Alert.decodeError "empty handshake record"
      drainHandshake { c with hsBuf := c.hsBuf ++ body }
    else if inner == 21 then
      if c.hsBuf.size > 0 then fail Alert.unexpectedMessage "alert inside a handshake message"
      handleAlert c body
    else if inner == 23 then
      if c.phase != .connected then fail Alert.unexpectedMessage "application data during handshake"
      if c.hsBuf.size > 0 then fail Alert.unexpectedMessage "application data inside a handshake message"
      return { c with appData := c.appData ++ body }
    else fail Alert.unexpectedMessage "unknown content type"

partial def drainRecords (c : Client) (b : ByteArray) (pos : Nat) : M (Client × Nat) := do
  if c.phase == .closed || c.phase == .failed then return (c, b.size)
  if b.size < pos + 5 then return (c, pos)
  let len := u16 b (pos + 3)
  if len > 16384 + 256 then fail Alert.recordOverflow "record too large"
  if b.size < pos + 5 + len then return (c, pos)
  let c ← handleRecord c b pos len
  drainRecords c b (pos + 5 + len)

/-! ## Public interface -/

structure Output where
  client : Client
  /-- bytes to write to the transport -/
  send : ByteArray
  /-- application data received -/
  data : ByteArray
  status : Status
deriving Inhabited

def Client.failWith (c : Client) (f : Fail) : Client :=
  let c := if f.alert == 255 then c else c.alertRecord f.alert
  { c with phase := .failed, error := f.msg, inBuf := .empty, hsBuf := .empty }

/-- Set up a client from a ready ClientHello message and its x25519
private key. `start` is the normal entry point; this one exists so that
foreign traces (RFC 8448) can be replayed exactly. -/
def startWithClientHello (cfg : Config) (ch priv : ByteArray) (canRebuild := false) (compat := false) :
    Client × ByteArray :=
  let c : Client := { cfg := cfg, priv := priv, canRebuild := canRebuild, compat := compat,
                      pub := X25519.publicKey priv, sigCache := cfg.sigCache }
  match offers c ch with
  | .error f => let c := c.failWith f; (c, c.out)
  | .ok c => ({ c with transcript := ch }, plainRecord 22 0x0301 ch)

/-- Begin a handshake: the client and the ClientHello record to send. -/
def start (cfg : Config) : Client × ByteArray :=
  let host := hostBytes cfg
  if cfg.entropy.size < 32 then
    ({ cfg := cfg, phase := .failed, error := "need at least 32 bytes of entropy" }, .empty)
  else if !(validHostName host) then
    ({ cfg := cfg, phase := .failed, error := "server name is not a DNS name" }, .empty)
  else if cfg.suites.size == 0 then
    ({ cfg := cfg, phase := .failed, error := "no cipher suites" }, .empty)
  else
    let prk := hkdfExtract .sha256 (ascii "yt-tls entropy") cfg.entropy
    let priv := hkdfExpand .sha256 prk (ascii "x25519") 32
    let random := hkdfExpand .sha256 prk (ascii "client random") 32
    let sid := hkdfExpand .sha256 prk (ascii "session id") 32
    let pub := X25519.publicKey priv
    -- the entropy is not kept in the client state
    let cfg := { cfg with entropy := .empty }
    startWithClientHello cfg (buildClientHello cfg random sid pub .empty) priv true true

/-- Feed bytes received from the transport. -/
def feed (c : Client) (input : ByteArray) : Output :=
  if c.phase == .failed || c.phase == .closed then ⟨c, .empty, .empty, c.status⟩ else
  let c := { c with out := .empty, appData := .empty }
  let b := c.inBuf ++ input
  match drainRecords c b 0 with
  | .ok (c, pos) =>
    let c := { c with inBuf := slice b pos (b.size - pos) }
    ⟨{ c with out := .empty, appData := .empty }, c.out, c.appData, c.status⟩
  | .error f =>
    let c := c.failWith f
    ⟨{ c with out := .empty }, c.out, .empty, c.status⟩

/-- Encrypt application data (only once connected): the records to send. -/
def send (c : Client) (data : ByteArray) : Client × ByteArray := Id.run do
  if c.phase != .connected || c.sentClose then return (c, .empty)
  let some w := c.wr | return (c, .empty)
  let mut w := w
  let mut out := ByteArray.empty
  let mut p := 0
  while p < data.size do
    let n := min 16384 (data.size - p)
    let (w', r) := w.seal 23 (slice data p n)
    w := w'
    out := out ++ r
    p := p + n
  ({ c with wr := some w }, out)

/-- Send close_notify; no more data can be sent afterwards. -/
def close (c : Client) : Client × ByteArray :=
  if c.sentClose || c.phase == .failed then (c, .empty) else
  let c := c.alertRecord Alert.closeNotify 1
  ({ c with sentClose := true, out := .empty }, c.out)

/-- The negotiated ALPN protocol ("" if none). -/
def Client.protocol (c : Client) : String := c.alpn

/-- The negotiated cipher suite's name. -/
def Client.cipherSuite (c : Client) : String := c.suite.name

end Yt.Tls
