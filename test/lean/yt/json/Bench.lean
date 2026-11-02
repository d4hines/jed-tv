import JsonTest
open Yt.Json Yt.Json.YouTube

def time (label : String) (reps : Nat) (f : Unit → Nat) : IO Unit := do
  let t0 ← IO.monoNanosNow
  let mut acc := 0
  for _ in [0:reps] do acc := acc + f ()
  let t1 ← IO.monoNanosNow
  IO.println s!"{label}: {(t1 - t0) / reps / 1000} us (check {acc / reps})"

/-- Host timings (not part of the transcript). -/
def main : IO Unit := do
  let player := JsonTest.Fixtures.player9Json.toUTF8
  let search := JsonTest.Fixtures.searchJson.toUTF8
  -- ~1 MB: four search replies in one array.
  let big := "[".toUTF8 ++ search ++ ",".toUTF8 ++ search ++ ",".toUTF8 ++ search ++ ",".toUTF8 ++ search ++ "]".toUTF8
  for (name, b) in [("player 85KB", player), ("search 253KB", search), ("search x4 1MB", big)] do
    IO.println s!"{name} ({b.size} bytes):"
    time "  Doc.parse (validate + tape)" 20 fun _ => match Doc.parse b with | .ok d => d.count | .error _ => 0
    time "  parsePlayer (parse + extract)" 20 fun _ => match parsePlayer b with | .ok p => p.formats.size | .error _ => 0
    time "  parseSearch (parse + extract)" 20 fun _ => match parseSearch b with | .ok v => v.size | .error _ => 0
    time "  Json.parse (full tree)" 20 fun _ => match Json.parse b with | .ok (.obj kvs) => kvs.size | .ok (.arr xs) => xs.size | _ => 0
