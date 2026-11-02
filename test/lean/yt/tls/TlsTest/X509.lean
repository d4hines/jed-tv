import Yt.X509.Verify
import TlsTest.Common
import TlsTest.Chains
open Yt.Crypto Yt.X509
namespace TlsTest

def keyName : PubKey → String
  | .rsa n _ => "rsa" ++ toString (rsaBits n)
  | .p256 _ => "p256"
  | .p384 _ => "p384"
  | .unsupported => "unsupported"

def result (r : Except String PubKey) : String :=
  match r with
  | .ok k => "ok " ++ keyName k
  | .error e => "rejected: " ++ e

def x509Tests (o : ByteArray) : ByteArray := Id.run do
  let mut o := o
  let t := Chains.captureTime
  let yt := hexLines Chains.youtubeEcdsa
  let ytr := hexLines Chains.youtubeRsa
  let gv := hexLines Chains.googlevideoEcdsa
  let im := hexLines Chains.ytimgEcdsa
  o := line o ("anchors parsed " ++ toString defaultAnchors.size)
  -- parsing details
  match Cert.parse yt[0]!, Cert.parse yt[1]! with
  | some leaf, some we2 =>
    o := checkB o "leaf validity 1791400062..1798657661" (leaf.notBefore == 1791400062 && leaf.notAfter == 1798657661)
    o := checkB o "WE2 validity 1702458000..1866290400" (we2.notBefore == 1702458000 && we2.notAfter == 1866290400)
    o := checkB o "leaf extensions" (leaf.dnsNames.size == 65 && !leaf.isCA && leaf.keyUsage == some 1
      && leaf.eku.isSome && !leaf.unknownCritical)
    o := checkB o "WE2 extensions" (we2.isCA && we2.pathLen == some 0 && we2.keyUsage == some 0x61)
  | _, _ => o := line o "parse FAIL"
  o := line o ("www.youtube.com ecdsa chain: " ++ result (verifyServer yt "www.youtube.com" t))
  o := line o ("www.youtube.com rsa chain: " ++ result (verifyServer ytr "www.youtube.com" t))
  o := line o ("googlevideo chain rr3---sn-a5mekn6s.googlevideo.com: " ++ result (verifyServer gv "rr3---sn-a5mekn6s.googlevideo.com" t))
  o := line o ("i.ytimg.com chain: " ++ result (verifyServer im "i.ytimg.com" t))
  o := line o ("names youtube.com: " ++ result (verifyServer yt "youtube.com" t))
  o := line o ("names WWW.YouTube.COM.: " ++ result (verifyServer yt "WWW.YouTube.COM." t))
  o := line o ("names a.b.youtube.com: " ++ result (verifyServer yt "a.b.youtube.com" t))
  o := line o ("names googlevideo.com on youtube leaf: " ++ result (verifyServer yt "x.googlevideo.com" t))
  o := line o ("names evil.example: " ++ result (verifyServer yt "evil.example" t))
  o := line o ("time before notBefore: " ++ result (verifyServer yt "www.youtube.com" 1791400061))
  o := line o ("time after notAfter: " ++ result (verifyServer yt "www.youtube.com" 1798657662))
  o := line o ("time after WE2 expiry: " ++ result (verifyServer yt "www.youtube.com" 1866290401))
  let bad := yt.set! 0 ((yt[0]!).set! (yt[0]!.size - 1) ((yt[0]!)[yt[0]!.size - 1]! ^^^ 1))
  o := line o ("tampered leaf signature: " ++ result (verifyServer bad "www.youtube.com" t))
  let badTbs := yt.set! 0 ((yt[0]!).set! 200 ((yt[0]!)[200]! ^^^ 1))
  o := line o ("tampered leaf body: " ++ result (verifyServer badTbs "www.youtube.com" t))
  o := line o ("missing intermediate: " ++ result (verifyServer #[yt[0]!] "www.youtube.com" t))
  o := line o ("leaf and cross-sign only: " ++ result (verifyServer #[yt[0]!, yt[2]!] "www.youtube.com" t))
  o := line o ("only R1 trusted, ecdsa chain: " ++ result (verifyServer yt "www.youtube.com" t #[defaultAnchors[0]!]))
  o := line o ("only R1 trusted, rsa chain: " ++ result (verifyServer ytr "www.youtube.com" t #[defaultAnchors[0]!]))
  o := line o ("no anchors: " ++ result (verifyServer yt "www.youtube.com" t #[]))
  o := line o ("intermediate as leaf: " ++ result (verifyServer #[yt[1]!, yt[2]!] "www.youtube.com" t))
  o := line o ("reordered extras: " ++ result (verifyServer #[yt[0]!, yt[2]!, yt[1]!] "www.youtube.com" t))
  -- signature cache: hits for the same chain, no help for a tampered one
  match verifyServerCached yt "www.youtube.com" t with
  | .ok (_, cache) =>
    o := line o ("cache after ecdsa chain: " ++ toString cache.size ++ " entries")
    o := line o ("cached, same chain: " ++ result ((verifyServerCached yt "www.youtube.com" t defaultAnchors {} cache).map (·.1)))
    o := line o ("cached, tampered leaf signature: " ++ result ((verifyServerCached bad "www.youtube.com" t defaultAnchors {} cache).map (·.1)))
    o := line o ("cached, expired: " ++ result ((verifyServerCached yt "www.youtube.com" 1798657662 defaultAnchors {} cache).map (·.1)))
    o := line o ("cached, other name: " ++ result ((verifyServerCached yt "evil.example" t defaultAnchors {} cache).map (·.1)))
  | .error e => o := line o ("cache FAIL " ++ e)
  -- wildcard rules
  let nm (p n : String) := if nameMatches p.toUTF8 n.toUTF8 then "y" else "n"
  o := line o ("nameMatches " ++ nm "*.youtube.com" "www.youtube.com" ++ nm "*.youtube.com" "youtube.com"
    ++ nm "*.youtube.com" "a.b.youtube.com" ++ nm "*.com" "x.com" ++ nm "*.co.uk" "x.co.uk"
    ++ nm "www.youtube.com" "www.youtube.com" ++ nm "w*.youtube.com" "www.youtube.com"
    ++ nm "*.youtube.com" "*.youtube.com" ++ nm "" "" ++ nm "*." "a.")
  o

end TlsTest
