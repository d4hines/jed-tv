import Yt.Tls.Client
import TlsTest.Common
import TlsTest.LiveYoutube
import TlsTest.LiveGooglevideo
open Yt.Crypto Yt.Tls
namespace TlsTest

/-! Handshake workload for instruction counting on 32-bit ARM (`zr-build
timed test/lean/yt/tls tls_test_handshake 0,1,2,3 -- -icount shift=0`): one
client handshake replayed from a recorded live session, from ClientHello to
the decrypted response. Stage 0: www.youtube.com with an empty signature
cache (the app's first connection), 1: the same with the cache warm (every
later connection to that host), 2 and 3: redirector.googlevideo.com (the
media requests) cold and warm. The warm caches are module constants, built
before the timed calls. Returns a checksum of what the client sent. -/

def handshake (serverName : String) (time : UInt64) (entropy received : String) (cache : Array ByteArray) :
    Client × ByteArray :=
  let (c, hello) := start { serverName, unixTime := time, entropy := ofHex entropy,
                            suites := #[.aes128gcm, .chacha20, .aes256gcm], sigCache := cache }
  let r := feed c (ofHex received)
  (r.client, hello ++ r.send ++ r.data)

def sum (b : ByteArray) : UInt32 := b.foldl (fun h x => (h ^^^ x.toUInt32) * 16777619) 2166136261

def yt (cache : Array ByteArray) : Client × ByteArray :=
  handshake LiveYoutube.serverName LiveYoutube.unixTime LiveYoutube.entropy LiveYoutube.received cache
def gv (cache : Array ByteArray) : Client × ByteArray :=
  handshake LiveGooglevideo.serverName LiveGooglevideo.unixTime LiveGooglevideo.entropy LiveGooglevideo.received cache
def ytCache : Array ByteArray := (yt #[]).1.sigCache
def gvCache : Array ByteArray := (gv #[]).1.sigCache

/-- `stage` is opaque to the compiler, so no stage's handshake is hoisted out of the timed call. -/
@[export tls_test_handshake]
def handshakeBench (stage : Nat) : UInt32 :=
  let none : Array ByteArray := if stage == 99 then ytCache else #[]
  match stage with
  | 0 => sum (yt none).2
  | 1 => sum (yt ytCache).2
  | 2 => sum (gv none).2
  | _ => sum (gv gvCache).2

end TlsTest
