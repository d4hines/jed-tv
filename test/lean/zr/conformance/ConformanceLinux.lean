import ZrLeanTest.Conformance

/-!
The conformance transcript on our runtime under Linux (`zr-build conformance`).
The per-app DSL `main` (`zr-asm-gen app`) calls this module's initializer,
then `zr_conformance_main`. System calls go through ZrTv.Linux, linked from
the runtime build.

Normally the transcript goes to stdout and Lean's panic messages to stderr.
As PID 1 (zr-qemu32), stderr is redirected to `/panics`. After the
transcript, the panic messages follow on the console between markers, and
then the machine powers off.
-/

namespace ZrLeanTest.Linux

@[extern "zr_tv_sys"] opaque sys (nr a0 a1 a2 a3 : USize) : BaseIO USize
@[extern "zr_tv_write"] opaque writeRaw (fd : USize) (b : @& ByteArray) (off len : USize) : BaseIO USize
@[extern "zr_tv_read"] opaque read (fd n : USize) : BaseIO (Except Nat ByteArray)
@[extern "zr_tv_open"] opaque openPath (path : @& String) (flags mode : USize) : BaseIO USize

def isErr (r : USize) : Bool := r > (0 : USize) - 4096

partial def putAll (fd : USize) (b : ByteArray) (off : USize) : BaseIO Unit := do
  if off < b.size.toUSize then
    let w ← writeRaw fd b off (b.size.toUSize - off)
    if w != 0 && !isErr w then putAll fd b (off + w)

def put (fd : USize) (s : String) : BaseIO Unit := putAll fd s.toUTF8 0

/-- Copy everything left in `src` to stdout. -/
partial def copyOut (src : USize) : BaseIO Unit := do
  match ← read src 512 with
  | .ok b =>
    if b.size > 0 then
      putAll 1 b 0
      copyOut src
  | .error _ => pure ()

@[export zr_conformance_main]
def main (argc : UInt32) : BaseIO UInt32 := do
  let pid1 := (← sys 20 0 0 0 0) == 1                    -- getpid
  let mut panics : USize := 0
  if pid1 then
    panics ← openPath "/panics" (0o100 ||| 0o1000 ||| 2) 0o600  -- O_CREAT | O_TRUNC | O_RDWR
    let _ ← sys 63 panics 2 0 0                             -- dup2(panics, stderr)
  -- `k` stays 0; taking it from argc keeps it opaque to the compiler.
  let out := ZrLean.Conformance.transcript (if argc > 100 then 1 else 0)
  if !pid1 then
    putAll 1 out 0
    return 0
  put 1 "\n==lean-transcript==\n"
  putAll 1 out 0
  put 1 "==lean-panics==\n"
  let _ ← sys 19 panics 0 0 0                               -- lseek(panics, 0, SEEK_SET)
  copyOut panics
  put 1 "==lean-end==\n"
  let _ ← sys 36 0 0 0 0                                    -- sync
  let _ ← sys 88 0xfee1dead 672274793 0x4321fedc 0          -- reboot(RB_POWER_OFF)
  return 0

end ZrLeanTest.Linux
