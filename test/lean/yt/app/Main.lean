import AppTest
def main : IO Unit := do
  let o ← IO.getStdout
  o.write (AppTest.transcript 0)
  unless AppTest.KeepAlive.allPass do throw (IO.userError "keepalive eligibility regression")
