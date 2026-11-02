import Mp4Test
def main : IO Unit := do
  (← IO.getStdout).write (Mp4Test.transcript 0)
