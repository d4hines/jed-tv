import Yt.Aac.Decoder

/-! Host-only: time the table generators (first use of the decoder). -/
open Yt.Aac

def time (name : String) (f : Unit → IO Nat) : IO Unit := do
  let t0 ← IO.monoNanosNow
  let n ← f ()
  let t1 ← IO.monoNanosNow
  IO.println s!"{name}: {(t1 - t0) / 1000} us ({n})"

def main (args : List String) : IO Unit := do
  let n := (args.headD "1024").toNat!
  time "zeroBlock" fun _ => pure (zeroBlock ()).size
  time "pi" fun _ => pure (Math.piQ62 ()).toNat
  time "sine n" fun _ => pure (Math.sineWindow n).size
  time "kbd n α=4" fun _ => pure (Math.kbdWindow n 4).size
  time "mdct 2n" fun _ => pure (Mdct.make (2 * n)).tw.size
  time "pow43" fun _ => pure (Math.pow43Table (8 * n + 16)).1.size
  time "books" fun _ => pure (books ()).spec.size
  time "sharedTables (rest)" fun _ => pure (sharedTables ()).tns3.size
  time "sharedTables (again)" fun _ => pure (sharedTables ()).tns4.size
