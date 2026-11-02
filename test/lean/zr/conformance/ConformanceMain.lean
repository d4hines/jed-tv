import ZrLeanTest.Conformance

/-! Host driver: the conformance transcript on upstream Lean's own runtime
(`zr-build conformance --record` records its output). Not part of the
runtime's generated C. -/

def main (args : List String) : IO Unit := do
  -- `k` stays 0; taking it from the command line keeps it opaque to the compiler.
  let k := if args.length > 100 then 1 else 0
  let out ← IO.getStdout
  out.write (ZrLean.Conformance.transcript k)
  out.flush
