import Yt.Crypto.Hash
import Yt.Crypto.Gcm
import Yt.Crypto.ChaCha
/-! TLS 1.3 cipher suites, key schedule (RFC 8446 section 7) and record
protection (section 5.2). -/
namespace Yt.Tls
open Yt.Crypto

inductive Suite where
  | aes128gcm | aes256gcm | chacha20
deriving BEq, Inhabited

namespace Suite
def code : Suite → Nat
  | .aes128gcm => 0x1301 | .aes256gcm => 0x1302 | .chacha20 => 0x1303
def ofCode? : Nat → Option Suite
  | 0x1301 => some .aes128gcm | 0x1302 => some .aes256gcm | 0x1303 => some .chacha20 | _ => none
def hash : Suite → HashAlg
  | .aes256gcm => .sha384 | _ => .sha256
def keyLen : Suite → Nat
  | .aes128gcm => 16 | _ => 32
def name : Suite → String
  | .aes128gcm => "TLS_AES_128_GCM_SHA256"
  | .aes256gcm => "TLS_AES_256_GCM_SHA384"
  | .chacha20 => "TLS_CHACHA20_POLY1305_SHA256"
end Suite

/-- HKDF-Expand-Label(secret, label, context, len). -/
def expandLabel (h : HashAlg) (secret : ByteArray) (label : String) (ctx : ByteArray) (len : Nat) : ByteArray :=
  let l := ("tls13 " ++ label).toUTF8
  let info := ((ByteArray.emptyWithCapacity (4 + l.size + ctx.size)).push (len >>> 8).toUInt8
    |>.push len.toUInt8 |>.push l.size.toUInt8) ++ l ++ (ByteArray.empty.push ctx.size.toUInt8) ++ ctx
  hkdfExpand h secret info len

/-- Derive-Secret(secret, label, messages) given the transcript hash. -/
def deriveSecret (h : HashAlg) (secret : ByteArray) (label : String) (thash : ByteArray) : ByteArray :=
  expandLabel h secret label thash h.outLen

/-- One direction's record protection state. -/
structure Cipher where
  suite : Suite
  key : ByteArray
  gcm : Gcm.Key
  iv : ByteArray
  seq : UInt64
  /-- the traffic secret (for KeyUpdate) -/
  secret : ByteArray
deriving Inhabited

def Cipher.ofSecret (s : Suite) (secret : ByteArray) : Cipher :=
  let h := s.hash
  let key := expandLabel h secret "key" ByteArray.empty s.keyLen
  let iv := expandLabel h secret "iv" ByteArray.empty 12
  let gcm := match s with | .chacha20 => default | _ => Gcm.init key
  ⟨s, key, gcm, iv, 0, secret⟩

/-- The next generation of traffic secret and keys (KeyUpdate). -/
def Cipher.update (c : Cipher) : Cipher :=
  Cipher.ofSecret c.suite (expandLabel c.suite.hash c.secret "traffic upd" ByteArray.empty c.suite.hash.outLen)

def Cipher.nonce (c : Cipher) : ByteArray := Id.run do
  let mut n := c.iv
  for i in [0:8] do
    n := n.set! (11 - i) (n[11 - i]! ^^^ (c.seq >>> (8 * i).toUInt64).toUInt8)
  n

/-- Protect one record: `content ‖ type` into a TLSCiphertext record. -/
def Cipher.seal (c : Cipher) (type : UInt8) (content : ByteArray) : Cipher × ByteArray :=
  let inner := content.push type
  let len := inner.size + 16
  let hdr := (ByteArray.emptyWithCapacity 5).push 23 |>.push 3 |>.push 3 |>.push (len >>> 8).toUInt8 |>.push len.toUInt8
  let n := c.nonce
  let ct := match c.suite with
    | .chacha20 => ChaCha.encrypt c.key n hdr inner
    | _ => Gcm.encrypt c.gcm n hdr inner
  ({ c with seq := c.seq + 1 }, hdr ++ ct)

/-- Remove protection from the record whose header starts at `off` in `b`
(body length `len`): the inner content type and content. -/
def Cipher.unseal (c : Cipher) (b : ByteArray) (off len : Nat) : Option (Cipher × UInt8 × ByteArray) := do
  let hdr := slice b off 5
  let n := c.nonce
  let pt ← match c.suite with
    | .chacha20 => ChaCha.decryptAt c.key n hdr b (off + 5) len
    | _ => Gcm.decryptAt c.gcm n hdr b (off + 5) len
  -- strip zero padding; the last nonzero byte is the content type
  let mut i := pt.size
  while i > 0 && pt[i - 1]! == 0 do i := i - 1
  if i == 0 then none
  some ({ c with seq := c.seq + 1 }, pt[i - 1]!, slice pt 0 (i - 1))

end Yt.Tls
