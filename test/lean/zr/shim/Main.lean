import ShimTest
def main : IO Unit := do
  let o ← IO.getStdout
  o.write (ShimTest.transcript 0)
