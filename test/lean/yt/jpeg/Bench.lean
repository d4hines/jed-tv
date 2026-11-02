import JpegTest
open Yt.Jpeg

/-- Host decode times (not part of the transcript). -/
def main : IO Unit := do
  for (name, w, h, jpg, _, _) in JpegTest.Fixtures.all do
    if !(name.startsWith "yt-" || name == "prog420" || name == "s444") then continue
    let file := JpegTest.b64 jpg
    let reps := 20
    let t0 ← IO.monoNanosNow
    let mut acc := 0
    for _ in [0:reps] do
      match decode file with
      | .ok img => acc := acc + img.rgb.size
      | .error _ => pure ()
    let t1 ← IO.monoNanosNow
    let t2 ← IO.monoNanosNow
    for _ in [0:reps] do
      match decode file with
      | .ok img => acc := acc + (scaleNearest img 288 162).size
      | .error _ => pure ()
    let t3 ← IO.monoNanosNow
    IO.println s!"{name} {w}x{h}: decode {(t1 - t0) / reps / 1000} us, decode+scale to 288x162 {(t3 - t2) / reps / 1000} us ({acc / reps / 2} bytes)"
