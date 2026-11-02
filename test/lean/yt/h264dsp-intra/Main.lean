import Yt.Host.H264Kernels
import H264DspTest.Transcript
def main : IO Unit := do
  let o ← IO.getStdout
  o.write (H264DspTest.transcript 0)
