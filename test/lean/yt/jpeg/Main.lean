import JpegTest
def main : IO Unit := do
  (← IO.getStdout).write (JpegTest.transcript 0)
