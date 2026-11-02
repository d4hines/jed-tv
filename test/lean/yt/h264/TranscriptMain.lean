import Yt.Host.H264Kernels
import H264Test.Transcript
def main : IO Unit := do
  let o ← IO.getStdout
  o.write (H264Test.transcript 0)
