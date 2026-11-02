import DoomTest.Harness.Real
/-!
# fb-test

E2E framebuffer gate: run DEMO1 through gametic 1050, dump fb_35 / fb_70 /
fb_350 / fb_700 / fb_1050. Pairwise dump+match of PPM+FNV pairs. Also one
E1M1 scroll sample at tic 35.
-/

open Doom.Harness.Real

private def defaultRoot : IO System.FilePath := do
  let cwd ← IO.currentDir
  match cwd.components.getLast? with
  | some "lean" => pure (cwd.parent.getD cwd)
  | _ => pure cwd

private def comparePair (label : String) (ref cand : System.FilePath) : IO Bool := do
  if !(← cand.pathExists) then
    IO.eprintln s!"fb-test FAIL: missing {label}"
    return false
  let rb ← IO.FS.readBinFile ref
  let cb ← IO.FS.readBinFile cand
  if rb != cb then
    IO.eprintln s!"fb-test FAIL: {label} mismatch (ref {rb.size} cand {cb.size})"
    return false
  pure true

def main (_args : List String) : IO UInt32 := do
  let root ← defaultRoot
  let iwad := root / "fixtures" / "wads" / "doom1.wad"
  let refDir := root / "fixtures" / "fb" / "demo1"
  let e1Ref := root / "fixtures" / "fb" / "e1m1"
  let outDir := root / ".agent_tmp" / "fb_cand"
  let e1Out := root / ".agent_tmp" / "fb_e1m1_cand"
  let traceBase := root / ".agent_tmp" / "fb_trace"
  IO.FS.createDirAll outDir
  IO.FS.createDirAll e1Out
  let demo1Names := [
    "fb_35.ppm", "fb_35.fnv", "fb_70.ppm", "fb_70.fnv",
    "fb_350.ppm", "fb_350.fnv", "fb_700.ppm", "fb_700.fnv",
    "fb_1050.ppm", "fb_1050.fnv"]
  for name in demo1Names do
    try IO.FS.removeFile (outDir / name) catch _ => pure ()
  for name in ["fb_35.ppm", "fb_35.fnv"] do
    try IO.FS.removeFile (e1Out / name) catch _ => pure ()
  match ← runReal iwad "DEMO1" 1051 traceBase (some outDir) #[35, 70, 350, 700, 1050] with
  | Except.error e => do
    IO.eprintln s!"fb-test FAIL: {e}"
    pure 1
  | Except.ok () =>
    let mut ok := true
    for name in demo1Names do
      let pass ← comparePair name (refDir / name) (outDir / name)
      ok := ok && pass
    match ← runReal iwad (root / "fixtures" / "e1m1" / "e1m1demo.lmp").toString 70
        (root / ".agent_tmp" / "fb_e1m1_trace") (some e1Out) #[35] with
    | Except.error e => do
      IO.eprintln s!"fb-test FAIL: e1m1 {e}"
      pure 1
    | Except.ok () =>
      let ok35 ← comparePair "e1m1 fb_35.ppm" (e1Ref / "fb_35.ppm") (e1Out / "fb_35.ppm")
      let ok35f ← comparePair "e1m1 fb_35.fnv" (e1Ref / "fb_35.fnv") (e1Out / "fb_35.fnv")
      if ok && ok35 && ok35f then
        IO.println "fb-test PASS: DEMO1 35/70/350/700/1050 and E1M1 tic 35 ppm+fnv match"
        pure 0
      else
        pure 1
