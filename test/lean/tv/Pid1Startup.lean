import TV.Pid1

-- Stage failures must stop later operations, while success preserves order.
def main : IO Unit := do
  for failAt in [none, some 0, some 1, some 2] do
    let calls ← IO.mkRef (#[] : Array UInt32)
    let result ← TV.Pid1.startup fun stage => do
      calls.modify (·.push stage.code)
      return if failAt == some stage.code then 1 else 0
    let expected := match failAt with
      | none => #[0, 1, 2]
      | some 0 => #[0]
      | some 1 => #[0, 1]
      | _ => #[0, 1, 2]
    unless (← calls.get) == expected do throw (IO.userError "startup order/failure boundary")
    unless result == (failAt.map (· + 1)).getD 0 do throw (IO.userError "startup failure result")
  IO.println "Lean PID1 startup order and every failure boundary: PASS"
