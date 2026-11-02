import HttpTest
def main : IO Unit := do
  (← IO.getStdout).write (HttpTest.transcript 0)
