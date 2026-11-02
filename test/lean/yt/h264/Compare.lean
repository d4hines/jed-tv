import Host.Compare

/-! `h264compare FIXTURE.mp4 SAMPLES [--no-deblock]`: Host.compareMp4 (the
Lean decoder against FFmpeg on the first SAMPLES samples). -/

def main (args : List String) : IO UInt32 := do
  let noDeblock := args.contains "--no-deblock"
  let [fixture, nStr] := args.filter (· != "--no-deblock")
    | IO.eprintln "usage: h264compare FIXTURE.mp4 SAMPLES [--no-deblock]"; return 2
  match ← Host.compareMp4 fixture nStr.toNat! noDeblock with
  | (some r, same) => IO.println r; IO.println s!"{same} frames identical before frame {same}"; return 1
  | (none, same) => IO.println s!"all {same} frames identical to ffmpeg"; return 0
