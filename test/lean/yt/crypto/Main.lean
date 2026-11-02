import CryptoTest
def main : IO Unit := do
  let o ← IO.getStdout
  o.write (CryptoTest.transcript 0)
