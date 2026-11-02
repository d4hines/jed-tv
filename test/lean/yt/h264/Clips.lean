import Host.Compare

/-! `h264clips [DIR]`: the Lean decoder's feature clips: small libx264 encodes
(fragmented MP4, like YouTube's) that each exercise one part of the syntax,
generated into DIR (default assets/build/lean-h264/clips, git-ignored) when
missing, then compared frame by frame with FFmpeg (Host.compareMp4). Needs
ffmpeg with libx264 on PATH. The CAVLC clips (baseline, cavlc_main) are
generated but not compared until CAVLC is implemented. -/

def cif := "testsrc2=size=352x288:rate=25:duration=3"

/-- (name, lavfi source, encoder options, compared). -/
def clips : List (String × String × Array String × Bool) := [
  ("default", cif, #["-preset", "medium", "-x264-params", "weightp=2:weightb=1"], true),
  ("temporal", cif, #["-preset", "medium", "-x264-params", "direct=temporal:weightb=1"], true),
  ("slices", cif, #["-preset", "medium", "-x264-params", "slices=4"], true),
  ("cqm", cif, #["-preset", "medium", "-x264-params", "cqm=jvt"], true),
  ("pyramid", cif, #["-preset", "medium", "-x264-params", "b-pyramid=normal:bframes=4:ref=4"], true),
  ("nodct8", cif, #["-profile:v", "main", "-preset", "medium"], true),
  ("oddsize", "testsrc2=size=350x286:rate=25:duration=2", #["-preset", "medium"], true),
  ("motion", "mandelbrot=size=640x360:rate=25", #["-t", "3", "-preset", "slow", "-crf", "30"], true),
  ("baseline", cif, #["-profile:v", "baseline", "-preset", "medium"], false),
  ("cavlc_main", cif, #["-profile:v", "main", "-preset", "medium", "-x264-params", "cabac=0:bframes=3"], false)]

def main (args : List String) : IO UInt32 := do
  let dir : System.FilePath := args.headD "../../../../assets/build/lean-h264/clips"
  IO.FS.createDirAll dir
  let mut ok := true
  for (name, src, opts, compared) in clips do
    let path := dir / s!"{name}.mp4"
    if !(← path.pathExists) then
      let r ← IO.Process.output { cmd := "ffmpeg", args := #["-v", "error", "-y", "-f", "lavfi", "-i", src,
        "-pix_fmt", "yuv420p", "-c:v", "libx264"] ++ opts ++
        #["-movflags", "frag_keyframe+empty_moov+default_base_moof", path.toString] }
      if r.exitCode != 0 then IO.eprintln s!"{name}: {r.stderr}"; return 1
    if compared then
      match ← Host.compareMp4 path.toString 100000 false with
      | (some r, _) => IO.println s!"{name}: {r}"; ok := false
      | (none, same) => IO.println s!"{name}: all {same} frames identical"
  return if ok then 0 else 1
