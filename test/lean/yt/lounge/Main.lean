import LoungeTest
def main : IO Unit := do
  let o ← IO.getStdout
  o.write (LoungeTest.transcript 0)
