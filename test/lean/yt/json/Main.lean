import JsonTest
def main : IO Unit := do
  (← IO.getStdout).write (JsonTest.transcript 0)
