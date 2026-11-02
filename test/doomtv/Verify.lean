import DoomTest.Harness.Real
/-!
# verify

Playsim identity against Chocolate Doom: play a demo through the engine,
write its per-tic digest stream, and compare it byte for byte with the
oracle's (`DDIG` v1: 16-byte header, then one 8-byte FNV digest per tic).
-/

open Doom.Harness.Real

private def usage : String :=
  "usage: verify --iwad PATH --demo NAME --ref-digest REF.dig --out-dir DIR [--tics N] [--trace] [--timer MIN]"

structure VerifyArgs where
  iwad : System.FilePath := ""
  demo : String := ""
  refDigest : System.FilePath := ""
  outDir : System.FilePath := ""
  tics : Nat := 1000000000
  trace : Bool := false
  timer : Nat := 0
  bench : Bool := false
  api : Bool := false
  exits : Array (Nat × Bool) := #[]
  flow : Bool := false
  sounds : Bool := false

private def parseArgs : List String → VerifyArgs → Except String VerifyArgs
  | [], a =>
    if a.iwad.toString == "" || a.demo == "" || a.refDigest.toString == "" ||
        a.outDir.toString == "" then .error usage else .ok a
  | "--iwad" :: p :: rest, a => parseArgs rest { a with iwad := p }
  | "--demo" :: p :: rest, a => parseArgs rest { a with demo := p }
  | "--ref-digest" :: p :: rest, a => parseArgs rest { a with refDigest := p }
  | "--out-dir" :: p :: rest, a => parseArgs rest { a with outDir := p }
  | "--trace" :: rest, a => parseArgs rest { a with trace := true }
  | "--bench" :: rest, a => parseArgs rest { a with bench := true }
  | "--api" :: rest, a => parseArgs rest { a with api := true }
  | "--flow" :: rest, a => parseArgs rest { a with flow := true }
  | "--sounds" :: rest, a => parseArgs rest { a with sounds := true }
  | "--exits" :: spec :: rest, a =>
    let ex := (spec.splitOn ",").filterMap fun item =>
      match item.splitOn ":" with
      | [t] => t.toNat?.map (·, false)
      | [t, k] => t.toNat?.map (·, k == "s")
      | _ => none
    parseArgs rest { a with exits := ex.toArray }
  | "--timer" :: n :: rest, a =>
    match n.toNat? with
    | some n => parseArgs rest { a with timer := n }
    | none => .error s!"invalid --timer {n}"
  | "--tics" :: n :: rest, a =>
    match n.toNat? with
    | some n => parseArgs rest { a with tics := n }
    | none => .error s!"invalid --tics {n}"
  | other :: _, _ => .error s!"unknown argument {other}\n{usage}"

/-- First tic whose digest differs, or `none` when the streams are identical. -/
def firstDivergence (ref cand : ByteArray) : Option Nat := Id.run do
  let header := 16
  if ref.size < header || cand.size < header then return some 0
  let n := (min ref.size cand.size - header) / 8
  for t in [0:n] do
    for k in [0:8] do
      if ref[header + 8 * t + k]! != cand[header + 8 * t + k]! then return some t
  if ref.size != cand.size then some n else none

def main (args : List String) : IO UInt32 := do
  match parseArgs args {} with
  | .error e => IO.eprintln e; pure 2
  | .ok va =>
    IO.FS.createDirAll va.outDir
    let candBase := va.outDir / "candidate"
    let ref ← IO.FS.readBinFile va.refDigest
    let refTics := (ref.size - 16) / 8
    if va.bench then
      let t0 ← IO.monoNanosNow
      match ← runDemo va.iwad va.demo refTics candBase false (timelimit := va.timer.toInt32) (digest := false) with
      | .error e => IO.eprintln e; return 1
      | .ok r =>
        let ns := (← IO.monoNanosNow) - t0
        IO.println s!"{va.demo}: {r.tics} tics, engine only {ns / 1000000} ms ({ns / 1000 / (max r.tics 1)} us/tic)"
        return 0
    let t0 ← IO.monoMsNow
    let run := if va.api then runApi va.iwad va.demo (min va.tics refTics) candBase
      else runDemo va.iwad va.demo (min va.tics refTics) candBase va.trace
        (timelimit := va.timer.toInt32) (exits := va.exits) (logFlow := va.flow) (logSounds := va.sounds)
    match ← run with
    | .error e => IO.eprintln s!"{va.demo}: engine error: {e}"; pure 1
    | .ok r =>
      let ms := (← IO.monoMsNow) - t0
      let cand ← IO.FS.readBinFile (va.outDir / "candidate.dig")
      let refCut := ref.extract 0 (16 + 8 * (min va.tics refTics))
      match firstDivergence refCut cand with
      | none =>
        IO.println s!"{va.demo}: {r.tics} tics identical to the oracle ({ms} ms)"
        pure 0
      | some t =>
        IO.eprintln s!"{va.demo}: digest diverges at tic {t} (ran {r.tics} of {refTics}){match r.error with | some e => s!"; {e}" | none => ""}"
        pure 1
