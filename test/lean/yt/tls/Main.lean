import TlsTest
def main : IO Unit := do
  let o ← IO.getStdout
  o.write (TlsTest.transcript 0)
