import ZrAsm.Neon

/-!
# The few C library pieces an app needs, in the DSL

Apps link without a C library (`zr-build app`: `-nostdlib`):
what the compiled C still refers to is here, linked into each app with its
`main` (`zr-asm-gen app`).

- `_start`: the process entry. argc, argv and envp from the initial stack,
  `main(argc, argv, envp)`, then `exit_group` with its result. No thread
  pointer or other C runtime setup: the runtime has no thread-local storage,
  stdio or atexit.
- `memcpy`, `memmove`: NEON block copies (`vld1.8`/`vst1.8`: byte
  elements, so any alignment), bytes for the tail.
- `__aeabi_uldivmod`: unsigned 64-bit division with remainder (ARM EABI:
  numerator r1:r0, denominator r3:r2; quotient r1:r0, remainder r3:r2),
  restoring division after aligning the denominator's leading one with the
  numerator's. The caller never divides by zero (lean.h checks). Checked
  in the ARM model against `Nat` division: test/libc/Uldivmod.lean.
-/

namespace ZrAsm.Libc
open Reg Asm VReg

private def movr (rd rm : Reg) : Asm Unit := mov rd (.reg rm (.lsl 0))

/-- The process entry: sp points at argc, then argv[], NULL, envp[]. -/
def start : Routine := routine "_start" do
  mov r11 0
  mov lr 0
  ldr r0 (mem sp 0)                   -- argc
  add r1 sp 4                         -- argv
  add r2 r1 (r0.lsl 2)
  add r2 r2 4                         -- envp = argv + argc + 1
  bl "main"
  mov r7 248                          -- exit_group(main's result)
  ins (.svc 0)

/-- `void *memcpy(void *dst, const void *src, size_t n)`. -/
def memcpy : Routine := routine "memcpy" do
  movr ip r0                          -- dst cursor; r0 is the result
  cmp r2 64
  blo (fwd 2)
  lbl 1                               -- 64 bytes at a time
  vld1 8 [q0, q1] r1 .wb
  vld1 8 [q2, q3] r1 .wb
  sub r2 r2 64
  vst1 8 [q0, q1] ip .wb
  vst1 8 [q2, q3] ip .wb
  cmp r2 64
  bhs (back 1)
  lbl 2                               -- 16 at a time
  cmp r2 16
  blo (fwd 3)
  vld1 8 [q0] r1 .wb
  sub r2 r2 16
  vst1 8 [q0] ip .wb
  b (back 2)
  lbl 3                               -- 8 (byte elements: any alignment)
  cmp r2 8
  blo (fwd 4)
  vld1 8 [d0] r1 .wb
  sub r2 r2 8
  vst1 8 [d0] ip .wb
  lbl 4                               -- the last 0-7 bytes
  subs r2 r2 1
  bx lr .cc
  ldrb r3 (post r1 1)
  strb r3 (post ip 1)
  b (back 4)

/-- `void *memmove(void *dst, const void *src, size_t n)`: forward (memcpy)
unless dst lies inside src .. src + n, then backward, 16 bytes at a time
(each block is loaded before it is stored, and the blocks still to read lie
below the ones written). -/
def memmove : Routine := routine "memmove" do
  sub r3 r0 r1
  cmp r3 r2
  b "memcpy" .cs                      -- dst - src ≥ n (unsigned): no harmful overlap
  add r1 r1 r2
  add ip r0 r2
  lbl 1
  cmp r2 16
  blo (fwd 2)
  sub r1 r1 16
  sub ip ip 16
  vld1 8 [q0] r1
  sub r2 r2 16
  vst1 8 [q0] ip
  b (back 1)
  lbl 2
  subs r2 r2 1
  bx lr .cc
  ldrb r3 (pre r1 (-1))
  strb r3 (pre ip (-1))
  b (back 2)

/-- `__aeabi_uldivmod`: (r1:r0, r3:r2) ↦ quotient r1:r0, remainder r3:r2. -/
def uldivmod : Routine := routine "__aeabi_uldivmod" do
  push [r4, r5, r6, r7, r8, lr]
  mov r4 0                            -- quotient r5:r4
  mov r5 0
  -- a denominator above the numerator: quotient 0, remainder the numerator
  cmp r3 r1
  cmp r2 r0 .eq
  b ".Luldivmod_done" .hi
  -- shift = clz64(d) - clz64(n) ≥ 0
  cmp r3 0
  ins (.clz r6 r3) .ne
  ins (.clz r6 r2) .eq
  add r6 r6 32 .eq
  cmp r1 0
  ins (.clz r7 r1) .ne
  ins (.clz r7 r0) .eq
  add r7 r7 32 .eq
  sub r6 r6 r7
  -- d <<= shift
  subs r7 r6 32
  mov r3 (.regShift r2 .lsl r7) .pl
  mov r2 0 .pl
  b ".Luldivmod_shifted" .pl
  rsb r7 r6 32                        -- 32 - shift (a register shift by 32 gives 0)
  mov r3 (.regShift r3 .lsl r6)
  orr r3 r3 (.regShift r2 .lsr r7)
  mov r2 (.regShift r2 .lsl r6)
  label ".Luldivmod_shifted"
  add r6 r6 1                         -- shift + 1 quotient bits
  label ".Luldivmod_loop"
  adds r4 r4 r4                       -- q <<= 1
  ins (.dp .adc false r5 r5 r5)
  subs r7 r0 r2                       -- rem - d; carry: rem ≥ d
  ins (.dp .sbc true r8 r1 r3)
  mov r0 (.reg r7 (.lsl 0)) .cs       -- rem -= d, q |= 1
  mov r1 (.reg r8 (.lsl 0)) .cs
  orr r4 r4 1 .cs
  movs r3 (r3.lsr 1)                  -- d >>= 1
  mov r2 (.reg r2 .rrx)
  subs r6 r6 1
  b ".Luldivmod_loop" .ne
  label ".Luldivmod_done"
  movr r2 r0                          -- remainder r3:r2, quotient r1:r0
  movr r3 r1
  movr r0 r4
  movr r1 r5
  pop [r4, r5, r6, r7, r8, pc]

/-- What apps need from a C library, linked with their `main`. -/
def routines : List Routine := [start, memcpy, memmove, uldivmod]

end ZrAsm.Libc
