import Yt.Host.H264Kernels
import H264InterTest.Transcript
def main : IO Unit := do
  let o ← IO.getStdout
  o.write (H264InterTest.transcript 0)
