import ZrAsm.Ffmpeg.Common

/-!
# Start code candidate search (FFmpeg libavcodec/arm/startcode_armv6.S)

Copyright (c) 2013 RISC OS Open Ltd
Author: Ben Avison <bavison@riscosopen.org>

This file is part of FFmpeg. FFmpeg is free software; you can redistribute
it and/or modify it under the terms of the GNU Lesser General Public License
as published by the Free Software Foundation; either version 2.1 of the
License, or (at your option) any later version. FFmpeg is distributed in the
hope that it will be useful, but WITHOUT ANY WARRANTY; without even the
implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
the GNU Lesser General Public License for more details.

FFmpeg's file as Lean: each GNU macro is a Lean function of the same name and
parameters, `.if`/`.ifc` is `if`, and the numeric labels are FFmpeg's.
`test/ffmpeg/AsmCheck.lean` checks the result against FFmpeg's original byte
for byte.

The `.req` register names are Lean definitions, `ldr PATTERN, =0x80008000` is a
literal-pool load (`ldrConst`), and the Thumb `it` lines are left out: in ARM mode
they assemble to nothing.
-/

namespace ZrAsm.Ffmpeg.StartcodeArmv6

open ZrAsm Asm Reg VReg

-- The register names (`.req`), APCS a1-a4 = r0-r3, v1-v6 = r4-r9.
def RESULT : Reg := r0
def BUF : Reg := r0
def SIZE : Reg := r1
def PATTERN : Reg := r2
def PTR : Reg := r3
def DAT0 : Reg := r4
def DAT1 : Reg := r5
def DAT2 : Reg := r6
def DAT3 : Reg := r7
def TMP0 : Reg := r8
def TMP1 : Reg := r9
def TMP2 : Reg := ip
def TMP3 : Reg := lr

def PRELOAD_DISTANCE : Nat := 4

def innerloop4 : Asm Unit := do
  ldr DAT0 (post PTR 4)
  subs SIZE SIZE 4  -- C flag survives rest of macro
  sub TMP0 DAT0 (PATTERN.lsr 14)
  bic TMP0 TMP0 DAT0
  ands TMP0 TMP0 PATTERN

def innerloop16 (decrement : Option Nat) (do_preload : Bool := false) : Asm Unit := do
  ldmia PTR [DAT0, DAT1, DAT2, DAT3]
  if do_preload then
    pld (mem PTR (.int (PRELOAD_DISTANCE*32)))
  if let some n := decrement then
    subs SIZE SIZE (.nat n)  -- C flag survives rest of macro  -- C flag survives rest of macro
  sub TMP0 DAT0 (PATTERN.lsr 14)
  sub TMP1 DAT1 (PATTERN.lsr 14)
  bic TMP0 TMP0 DAT0
  bic TMP1 TMP1 DAT1
  sub TMP2 DAT2 (PATTERN.lsr 14)
  sub TMP3 DAT3 (PATTERN.lsr 14)
  ands TMP0 TMP0 PATTERN
  bic TMP2 TMP2 DAT2
  ands TMP1 TMP1 PATTERN .eq
  bic TMP3 TMP3 DAT3
  ands TMP2 TMP2 PATTERN .eq
  ands TMP3 TMP3 PATTERN .eq

-- int ff_startcode_find_candidate_armv6(const uint8_t *buf, int size)
def ff_startcode_find_candidate_armv6 : Asm Unit := function "ff_startcode_find_candidate_armv6" (global := true) do
  push [r4, r5, r6, r7, r8, r9, lr]  -- v1-v6, lr
  mov PTR BUF
  -- Ensure there are at least (PRELOAD_DISTANCE+2) complete cachelines to go
  -- before using code that does preloads
  cmp SIZE (.nat ((PRELOAD_DISTANCE+3)*32 - 1))
  blo (fwd 60)
  -- Get to word-alignment, 1 byte at a time
  tst PTR 3
  beq (fwd 2)
  lbl 1
  ldrb DAT0 (post PTR 1)
  sub SIZE SIZE 1
  teq DAT0 0
  beq (fwd 90)
  tst PTR 3
  bne (back 1)
  lbl 2  -- Get to 4-word alignment, 1 word at a time
  ldrConst PATTERN 0x80008000
  setend (be := true)
  tst PTR 12
  beq (fwd 4)
  lbl 3
  innerloop4
  bne (fwd 91)
  tst PTR 12
  bne (back 3)
  lbl 4  -- Get to cacheline (8-word) alignment
  tst PTR 16
  beq (fwd 5)
  innerloop16 (some 16)
  bne (fwd 93)
  lbl 5  -- Check complete cachelines, with preloading
  -- We need to stop when there are still (PRELOAD_DISTANCE+1)
  -- complete cachelines to go
  sub SIZE SIZE (.nat ((PRELOAD_DISTANCE+2)*32))
  lbl 6
  innerloop16 none (do_preload := true)
  bne (fwd 93)
  innerloop16 (some 32)
  bne (fwd 93)
  bcs (back 6)
  -- Preload trailing part-cacheline, if any
  tst SIZE 31
  beq (fwd 7)
  pld (mem PTR (.int ((PRELOAD_DISTANCE+1)*32)))
  -- Check remaining data without doing any more preloads. First
  -- do in chunks of 4 words:
  lbl 7
  adds SIZE SIZE (.nat ((PRELOAD_DISTANCE+2)*32 - 16))
  bmi (fwd 9)
  lbl 8
  innerloop16 (some 16)
  bne (fwd 93)
  bcs (back 8)
  -- Then in words:
  lbl 9
  adds SIZE SIZE (.nat (16 - 4))
  bmi (fwd 11)
  lbl 10
  innerloop4
  bne (fwd 91)
  bcs (back 10)
  lbl 11
  setend (be := false)
  -- Check second byte of final halfword
  ldrb DAT0 (mem PTR (-1))
  teq DAT0 0
  beq (fwd 90)
  -- Check any remaining bytes
  tst SIZE 3
  beq (fwd 13)
  lbl 12
  ldrb DAT0 (post PTR 1)
  sub SIZE SIZE 1
  teq DAT0 0
  beq (fwd 90)
  tst SIZE 3
  bne (back 12)
  -- No candidate found
  lbl 13
  sub RESULT PTR BUF
  b (fwd 99)
  lbl 60  -- Small buffer - simply check by looping over bytes
  subs SIZE SIZE 1
  bcc (fwd 99)
  lbl 61
  ldrb DAT0 (post PTR 1)
  subs SIZE SIZE 1
  teq DAT0 0
  beq (fwd 90)
  bcs (back 61)
  -- No candidate found
  sub RESULT PTR BUF
  b (fwd 99)
  lbl 90  -- Found a candidate at the preceding byte
  sub RESULT PTR BUF
  sub RESULT RESULT 1
  b (fwd 99)
  lbl 91  -- Found a candidate somewhere in the preceding 4 bytes
  sub RESULT PTR BUF
  sub RESULT RESULT 4
  sub TMP0 DAT0 0x20000
  bics TMP0 TMP0 DAT0
  ldrb DAT0 (mem PTR (-3)) .pl
  add RESULT RESULT 2 .pl
  bpl (fwd 92)
  teq RESULT 0
  beq (fwd 98)  -- don't look back a byte if found at first byte in buffer
  ldrb DAT0 (mem PTR (-5))
  lbl 92
  teq DAT0 0
  sub RESULT RESULT 1 .eq
  b (fwd 98)
  lbl 93  -- Found a candidate somewhere in the preceding 16 bytes
  sub RESULT PTR BUF
  sub RESULT RESULT 16
  teq TMP0 0
  beq (fwd 95)  -- not in first 4 bytes
  sub TMP0 DAT0 0x20000
  bics TMP0 TMP0 DAT0
  ldrb DAT0 (mem PTR (-15)) .pl
  add RESULT RESULT 2 .pl
  bpl (fwd 94)
  teq RESULT 0
  beq (fwd 98)  -- don't look back a byte if found at first byte in buffer
  ldrb DAT0 (mem PTR (-17))
  lbl 94
  teq DAT0 0
  sub RESULT RESULT 1 .eq
  b (fwd 98)
  lbl 95
  add RESULT RESULT 4
  teq TMP1 0
  beq (fwd 96)  -- not in next 4 bytes
  sub TMP1 DAT1 0x20000
  bics TMP1 TMP1 DAT1
  ldrb DAT0 (mem PTR (-13)) .mi
  ldrb DAT0 (mem PTR (-11)) .pl
  add RESULT RESULT 2 .pl
  teq DAT0 0
  sub RESULT RESULT 1 .eq
  b (fwd 98)
  lbl 96
  add RESULT RESULT 4
  teq TMP2 0
  beq (fwd 97)  -- not in next 4 bytes
  sub TMP2 DAT2 0x20000
  bics TMP2 TMP2 DAT2
  ldrb DAT0 (mem PTR (-9)) .mi
  ldrb DAT0 (mem PTR (-7)) .pl
  add RESULT RESULT 2 .pl
  teq DAT0 0
  sub RESULT RESULT 1 .eq
  b (fwd 98)
  lbl 97
  add RESULT RESULT 4
  sub TMP3 DAT3 0x20000
  bics TMP3 TMP3 DAT3
  ldrb DAT0 (mem PTR (-5)) .mi
  ldrb DAT0 (mem PTR (-3)) .pl
  add RESULT RESULT 2 .pl
  teq DAT0 0
  sub RESULT RESULT 1 .eq
  -- drop through to 98f
  lbl 98
  setend (be := false)
  lbl 99
  pop [r4, r5, r6, r7, r8, r9, pc]

def file : AsmFile where
  name := "startcode_armv6"
  copyright := ["Copyright (c) 2013 RISC OS Open Ltd", "Author: Ben Avison <bavison@riscosopen.org>"]
  module := "ZrAsm.Ffmpeg.StartcodeArmv6"
  body := do
    ff_startcode_find_candidate_armv6

end ZrAsm.Ffmpeg.StartcodeArmv6
