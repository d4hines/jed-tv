import AacTest.Transcript
def main : IO Unit := do
  let o ← IO.getStdout
  o.write (AacTest.transcript 0)
