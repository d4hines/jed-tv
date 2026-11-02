import ZrAsm
import ZrAsm.Ffmpeg
import ZrAsm.Yuv

/-!
# zr-asm-gen: writes the assembly files written in Lean

    zr-asm-gen OUT                  OUT/runtime.S (the Lean runtime's entry points)
                                    and OUT/tv.S (ZrTv's video routines, `ZrAsm.Yuv`)
    zr-asm-gen app OUT.S INIT ENTRY an app's main (INIT(1), then ENTRY(argc)),
                                    with `_start`, memcpy, memmove and 64-bit
                                    division (`ZrAsm.Libc`: apps link no libc)
    zr-asm-gen transcript OUT.S INIT  a package's ARM transcript binary: the same
                                    main with `ZrAsm.Linux.transcriptEntry` as ENTRY
    zr-asm-gen timed OUT.S INIT FN STAGES  a timed binary: FN on each stage
                                    (comma-separated Nats), `ZrAsm.Linux.timedEntry`
    zr-asm-gen ffmpeg-arm OUT       OUT/<name>.S and OUT/SOURCES: FFmpeg's 32-bit
                                    ARM H.264 assembly (`ZrAsm.Ffmpeg`)
-/

open ZrAsm

def main (args : List String) : IO UInt32 := do
  match args with
  | ["ffmpeg-arm", out] =>
    IO.FS.createDirAll out
    for f in Ffmpeg.files do
      IO.FS.writeFile s!"{out}/{f.name}.S" f.text
    IO.FS.writeFile s!"{out}/SOURCES" (String.join (Ffmpeg.files.map (s!"{·.name}.S\n")))
    return 0
  | ["timed", out, init, fn, stages] =>
    let stages := (stages.splitOn ",").filterMap String.toNat?
    IO.FS.writeFile out (".fpu neon\n" ++ Routine.file ("A timed binary: " ++ init ++ ", then " ++ fn ++
      " on stages " ++ toString stages ++ "; and the C library pieces it needs.") (Linux.timedRoutines init fn stages ++ Libc.routines))
    return 0
  | ["transcript", out, init] =>
    IO.FS.writeFile out (".fpu neon\n" ++ Routine.file ("A package's ARM transcript binary: " ++ init ++
      ", then its transcript; and the C library pieces it needs.") (Linux.transcriptRoutines init ++ Libc.routines))
    return 0
  | ["app", out, init, entry] =>
    IO.FS.writeFile out (".fpu neon\n" ++ Routine.file ("A Lean app's entry point: " ++ init ++ ", then " ++ entry ++
      "; and the C library pieces it needs.") (Linux.appRoutines init entry ++ Libc.routines))
    return 0
  | [out] =>
    IO.FS.createDirAll out
    IO.FS.writeFile s!"{out}/runtime.S" (Routine.file "The Lean runtime's entry points (branch stubs), storage and Linux system calls." (Runtime.routines ++ Linux.runtimeRoutines))
    IO.FS.writeFile s!"{out}/tv.S" Yuv.tvText
    return 0
  | _ =>
    IO.eprintln "usage: zr-asm-gen OUT | zr-asm-gen app OUT.S INIT ENTRY | zr-asm-gen transcript OUT.S INIT | zr-asm-gen timed OUT.S INIT FN STAGES | zr-asm-gen ffmpeg-arm OUT"
    return 2
