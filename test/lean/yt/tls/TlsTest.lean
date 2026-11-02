import TlsTest.X509
import TlsTest.Trace
import TlsTest.Replay
import TlsTest.Fuzz
import TlsTest.Hrr
import TlsTest.Handshake
namespace TlsTest

@[export zr_test_transcript] def transcript (k : Nat) : ByteArray := Id.run do
  let mut o := ByteArray.empty
  o := line o ("tls transcript " ++ toString k)
  o := x509Tests o
  o := rfc8448Tests o
  o := replayTests o
  o := fuzzTests o
  o := hrrTests o
  o
end TlsTest
