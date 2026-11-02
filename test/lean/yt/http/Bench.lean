import HttpTest
open Yt.Http

/-- Host timings (not part of the transcript). -/
def main : IO Unit := do
  let raw := HttpTest.b64 HttpTest.Fixtures.searchResponse
  let t0 ← IO.monoNanosNow
  let mut size := 0
  for _ in [0:20] do
    size := size + (((Parser.new).feed raw).finish).body.size
  let t1 ← IO.monoNanosNow
  IO.println s!"search response {raw.size} bytes, one feed: {(t1 - t0) / 20 / 1000} us (body {size / 20})"
  let t0 ← IO.monoNanosNow
  let mut p := Parser.new
  for i in [0:raw.size] do
    p := p.feedSlice raw i (i + 1)
  p := p.finish
  let t1 ← IO.monoNanosNow
  IO.println s!"same, byte by byte: {(t1 - t0) / 1000} us (body {p.body.size}, {(t1 - t0) / raw.size} ns/byte)"
  let t0 ← IO.monoNanosNow
  let mut q := Parser.new
  let mut i := 0
  while i < raw.size do
    q := q.feedSlice raw i (i + 1460)
    i := i + 1460
  q := q.finish
  let t1 ← IO.monoNanosNow
  IO.println s!"same, 1460-byte segments: {(t1 - t0) / 1000} us (body {q.body.size})"
