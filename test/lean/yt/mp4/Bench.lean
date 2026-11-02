import Mp4Test
open Yt.Mp4

/-- Host timing of the demuxer on the fixtures (not part of the transcript). -/
def main : IO Unit := do
  for (name, b64) in [("140.m4a", Mp4Test.Fixtures.m140), ("136.mp4", Mp4Test.Fixtures.m136)] do
    let file := Mp4Test.decode b64
    let t0 ← IO.monoNanosNow
    let mut total := 0
    for _ in [0:10] do
      let .ok init := parseInit file | throw (IO.userError "init")
      let .ok index := parseSidx file 0 | throw (IO.userError "sidx")
      for s in index.segments do
        let frag := file.extract s.offset.toNat (s.offset.toNat + s.size.toNat)
        let .ok v := parseFragments frag init | throw (IO.userError "frag")
        total := total + v.size
    let t1 ← IO.monoNanosNow
    IO.println s!"{name}: init+sidx+all fragments ({total / 10} samples) {(t1 - t0) / 10 / 1000} us per pass (incl. fragment copies)"
