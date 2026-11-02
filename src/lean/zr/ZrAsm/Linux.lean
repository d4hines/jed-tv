import ZrAsm.Stubs

/-!
# Linux userspace pieces of the runtime and of apps, in the DSL

`zr_sys6`, the ARM EABI system call (the number in r7, `svc #0`), is the
only way ZrLow and ZrTv code reach the kernel. The app entry `main` records
argc/argv/envp (read by ZrTv.Linux) and runs the app: its root module's
initializer and exported entry, named per app (`zr-asm-gen app`, run by
`zr-build app`).
-/

namespace ZrAsm.Linux
open Reg Asm

private def movr (rd rm : Reg) : Asm Unit := mov rd (.reg rm (.lsl 0))

/-- `size_t zr_sys6(nr, a0, a1, a2, a3, a4, a5)`, AAPCS: nr and a0-a2 in
r0-r3, a3-a5 on the stack; returns the kernel's raw result (-errno on
failure). Saves r4, r5, r7 (callee-saved) and keeps sp 8-byte aligned. -/
def sys6 : Routine := routine "zr_sys6" do
  push [r4, r5, r7, lr]
  movr r7 r0
  movr r0 r1
  movr r1 r2
  movr r2 r3
  ldr r3 (mem sp 16)
  ldr r4 (mem sp 20)
  ldr r5 (mem sp 24)
  ins (.svc 0)
  pop [r4, r5, r7, pc]

/-- The address of the Linux platform's state words (ZrLean.Low.Linux). -/
def heapAddr : Routine := routine "zr_linux_state_addr" do
  loadSym r0 "zr_linux_state"
  bx lr

/-- The runtime's Linux pieces (linked into every program). -/
def runtimeRoutines : List Routine :=
  [sys6, heapAddr, storage "zr_linux_state" 16, storage "zr_fw_heap_head" 4] ++
  stubs [("zr_rt_mem_alloc", "zr_low_linux_mem_alloc"), ("zr_rt_mem_free", "zr_low_linux_mem_free"),
         ("zr_rt_fatal", "zr_low_linux_fatal"), ("zr_rt_report_panic", "zr_low_linux_report_panic")]

/-- `int main(argc, argv, envp)`: record the arguments, initialize the app's
root module (`init(1)`) and run `entry(argc)`. -/
def appMain (init entry : String) : Routine := routine "main" do
  push [r4, lr]
  loadSym r3 "zr_args"
  str r0 (mem r3 0)
  str r1 (mem r3 4)
  str r2 (mem r3 8)
  movr r4 r0
  mov r0 (.imm 1 0)
  bl init
  movr r0 r4
  bl entry
  pop [r4, pc]

def argsAddr : Routine := routine "zr_tv_args_addr" do
  loadSym r0 "zr_args"
  bx lr

/-- An app's entry (linked into apps only). -/
def appRoutines (init entry : String) : List Routine := [appMain init entry, argsAddr, storage "zr_args" 12]

/-- `write(1, r1, r2)` until all `r2` bytes at `r1` are written or it fails. -/
def transcriptPut : Routine := routine "zr_transcript_put" do
  push [r7, lr]
  lbl 1
  cmp r2 0
  beq (fwd 2)
  mov r0 1
  mov r7 4                            -- write
  ins (.svc 0)
  cmp r0 0
  ble (fwd 2)
  add r1 r1 (.reg r0 (.lsl 0))
  sub r2 r2 (.reg r0 (.lsl 0))
  b (back 1)
  lbl 2
  pop [r7, pc]

/-- The entry of a package's ARM transcript binary (`zr-build transcript`):
the package's `zr_test_transcript k` (k = 0, taken from argc so that the
compiler cannot fold it) to stdout between `==lean-transcript==` and
`==lean-end==` lines, then, as PID 1 (zr-qemu32), `sync` and power off. The
result is a `ByteArray`: its size at +8, its bytes from +16. -/
def transcriptEntry : Routine := routine "zr_transcript_entry" do
  push [r4, r5, r6, r7, r8, lr]
  cmp r0 100
  mov r0 1                            -- boxed 0
  mov r0 3 .hi                        -- boxed 1 when argc > 100
  bl "zr_test_transcript"
  movr r4 r0
  loadSym r1 "zr_transcript_begin"
  mov r2 21
  bl "zr_transcript_put"
  add r1 r4 16
  ldr r2 (mem r4 8)
  bl "zr_transcript_put"
  loadSym r1 "zr_transcript_end"
  mov r2 13
  bl "zr_transcript_put"
  mov r7 20                           -- getpid
  ins (.svc 0)
  cmp r0 1
  bne (fwd 1)
  mov r7 36                           -- sync
  ins (.svc 0)
  loadImm r0 0xfee1dead               -- reboot(RB_POWER_OFF)
  loadImm r1 672274793
  loadImm r2 0x4321fedc
  mov r7 88
  ins (.svc 0)
  lbl 1
  mov r0 0
  pop [r4, r5, r6, r7, r8, pc]

/-- A transcript binary's `main` and entry: `init`, then the transcript. -/
def transcriptRoutines (init : String) : List Routine :=
  appRoutines init "zr_transcript_entry" ++ [transcriptEntry, transcriptPut,
    roString "zr_transcript_begin" "\n==lean-transcript==\n", roString "zr_transcript_end" "==lean-end==\n"]

/-- Eight hex digits of `r0` and a space to stdout (clobbers r0-r3). -/
def timedHex : Routine := routine "zr_timed_hex" do
  push [r4, r7, r8, lr]
  movr r4 r0
  loadSym r1 "zr_timed_text"
  mov r2 8
  lbl 1
  mov r3 (.reg r4 (.lsr 28))
  cmp r3 10
  add r3 r3 48 .cc                    -- '0'..'9' (unsigned <)
  add r3 r3 87 .cs                    -- 'a'..'f'
  strb r3 (post r1 1)
  mov r4 (.reg r4 (.lsl 4))
  subs r2 r2 1
  bne (back 1)
  mov r3 32
  strb r3 (mem r1 0)
  loadSym r1 "zr_timed_text"
  mov r2 9
  bl "zr_transcript_put"
  pop [r4, r7, r8, pc]

/-- `clock_gettime(CLOCK_MONOTONIC)` into the two words at r0 (seconds, nanoseconds). -/
def timedNow : Routine := routine "zr_timed_now" do
  push [r7, lr]
  movr r1 r0
  mov r0 1                            -- CLOCK_MONOTONIC
  loadImm r7 263                      -- clock_gettime
  ins (.svc 0)
  pop [r7, pc]

/-- The entry of a timed binary (`zr-build timed`): for each of `stages`, the
monotonic clock, then `fn stage` (a boxed small `Nat`), then the clock once
more; written to stdout as `==timed==` and hex words: per stage its start
(seconds, nanoseconds) and `fn`'s result, then the end time. As PID 1
(zr-qemu32), `sync` and power off. -/
def timedEntry (fn : String) (stages : List Nat) : Routine := routine "zr_timed_entry" do
  push [r4, r5, r6, r7, r8, lr]
  loadSym r4 "zr_timed_buf"
  for s in stages do
    movr r0 r4
    bl "zr_timed_now"
    loadImm r0 (2 * s + 1).toUInt32
    bl fn
    str r0 (mem r4 8)
    add r4 r4 12
  movr r0 r4
  bl "zr_timed_now"
  loadSym r1 "zr_timed_begin"
  mov r2 11
  bl "zr_transcript_put"
  loadSym r4 "zr_timed_buf"
  for _ in [0:3 * stages.length + 2] do
    ldr r0 (post r4 4)
    bl "zr_timed_hex"
  loadSym r1 "zr_timed_newline"
  mov r2 1
  bl "zr_transcript_put"
  mov r7 20                           -- getpid
  ins (.svc 0)
  cmp r0 1
  bne (fwd 1)
  mov r7 36                           -- sync
  ins (.svc 0)
  loadImm r0 0xfee1dead               -- reboot(RB_POWER_OFF)
  loadImm r1 672274793
  loadImm r2 0x4321fedc
  mov r7 88
  ins (.svc 0)
  lbl 1
  mov r0 0
  pop [r4, r5, r6, r7, r8, pc]

/-- A timed binary's `main` and entry: `init`, then `fn` on each of `stages`. -/
def timedRoutines (init fn : String) (stages : List Nat) : List Routine :=
  appRoutines init "zr_timed_entry" ++ [timedEntry fn stages, timedNow, timedHex, transcriptPut,
    storage "zr_timed_buf" (4 * (3 * stages.length + 2)), storage "zr_timed_text" 12,
    roString "zr_timed_begin" "\n==timed== ", roString "zr_timed_newline" "\n"]

end ZrAsm.Linux
