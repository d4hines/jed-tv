import ZrAsm.Ffmpeg.Common

/-!
# Half-pel (and qpel mc00) copy and average (FFmpeg libavcodec/arm/hpeldsp_neon.S)

ARM NEON optimised DSP functions
Copyright (c) 2008 Mans Rullgard <mans@mansr.com>

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
-/

namespace ZrAsm.Ffmpeg.HpelDsp

open ZrAsm Asm Reg VReg

-- `pixfunc` defines three macros by rounding mode, which the `pixels*` macros use:

/-- `avg rd, rn, rm`: the halving add, rounding (`vrhadd.u8`) or not (`vhadd.u8`). -/
def avgOp (rnd : Bool) (rd rn : VReg) (rm : VSrc) : Asm Unit :=
  if rnd then vrhadd .u8 rd rn rm else vhadd .u8 rd rn rm

/-- `shrn rd, rn, #n`: the narrowing shift, rounding (`vrshrn.u16`) or not (`vshrn.u16`). -/
def shrnOp (rnd : Bool) (rd rn : VReg) (n : Nat) : Asm Unit :=
  if rnd then vrshrn .u16 rd rn n else vshrn .u16 rd rn n

/-- `NRND insn`: the instruction only in the no-rounding variant. -/
def nrnd (rnd : Bool) (insn : Asm Unit) : Asm Unit :=
  unless rnd do insn

def pixels16 (rnd avg : Bool) : Asm Unit := do
  if avg then
    mov r12 r0
  lbl 1
  vld1 8 [q0] r1 r2
  vld1 8 [q1] r1 r2
  vld1 8 [q2] r1 r2
  pld (mem r1 (r2.lsl 2))
  vld1 8 [q3] r1 r2
  pld r1
  pld (mem r1 r2)
  pld (mem r1 (r2.lsl 1))
  if avg then
    vld1 8 [q8] (al r12 128) r2
    vrhadd .u8 q0 q0 q8
    vld1 8 [q9] (al r12 128) r2
    vrhadd .u8 q1 q1 q9
    vld1 8 [q10] (al r12 128) r2
    vrhadd .u8 q2 q2 q10
    vld1 8 [q11] (al r12 128) r2
    vrhadd .u8 q3 q3 q11
  subs r3 r3 4
  vst1 64 [q0] (al r0 128) r2
  vst1 64 [q1] (al r0 128) r2
  vst1 64 [q2] (al r0 128) r2
  vst1 64 [q3] (al r0 128) r2
  bne (back 1)
  bx lr

def pixels16_x2 (rnd avg : Bool) : Asm Unit := do
  lbl 1
  vld1 8 [d0, d1, d2] r1 r2
  vld1 8 [d4, d5, d6] r1 r2
  pld r1
  pld (mem r1 r2)
  subs r3 r3 2
  vext 8 q1 q0 q1 1
  avgOp rnd q0 q0 q1
  vext 8 q3 q2 q3 1
  avgOp rnd q2 q2 q3
  if avg then
    vld1 8 [q1] (al r0 128) r2
    vld1 8 [q3] (al r0 128)
    vrhadd .u8 q0 q0 q1
    vrhadd .u8 q2 q2 q3
    sub r0 r0 r2
  vst1 8 [q0] (al r0 128) r2
  vst1 8 [q2] (al r0 128) r2
  bne (back 1)
  bx lr

def pixels16_y2 (rnd avg : Bool) : Asm Unit := do
  sub r3 r3 2
  vld1 8 [q0] r1 r2
  vld1 8 [q1] r1 r2
  lbl 1
  subs r3 r3 2
  avgOp rnd q2 q0 q1
  vld1 8 [q0] r1 r2
  avgOp rnd q3 q0 q1
  vld1 8 [q1] r1 r2
  pld r1
  pld (mem r1 r2)
  if avg then
    vld1 8 [q8] (al r0 128) r2
    vld1 8 [q9] (al r0 128)
    vrhadd .u8 q2 q2 q8
    vrhadd .u8 q3 q3 q9
    sub r0 r0 r2
  vst1 8 [q2] (al r0 128) r2
  vst1 8 [q3] (al r0 128) r2
  bne (back 1)
  avgOp rnd q2 q0 q1
  vld1 8 [q0] r1 r2
  avgOp rnd q3 q0 q1
  if avg then
    vld1 8 [q8] (al r0 128) r2
    vld1 8 [q9] (al r0 128)
    vrhadd .u8 q2 q2 q8
    vrhadd .u8 q3 q3 q9
    sub r0 r0 r2
  vst1 8 [q2] (al r0 128) r2
  vst1 8 [q3] (al r0 128) r2
  bx lr

def pixels16_xy2 (rnd avg : Bool) : Asm Unit := do
  sub r3 r3 2
  vld1 8 [d0, d1, d2] r1 r2
  vld1 8 [d4, d5, d6] r1 r2
  nrnd rnd (vmovImm .i16 q13 1)
  pld r1
  pld (mem r1 r2)
  vext 8 q1 q0 q1 1
  vext 8 q3 q2 q3 1
  vaddl .u8 q8 d0 d2
  vaddl .u8 q10 d1 d3
  vaddl .u8 q9 d4 d6
  vaddl .u8 q11 d5 d7
  lbl 1
  subs r3 r3 2
  vld1 8 [d0, d1, d2] r1 r2
  vadd .u16 q12 q8 q9
  pld r1
  nrnd rnd (vadd .u16 q12 q12 q13)
  vext 8 q15 q0 q1 1
  vadd .u16 q1 q10 q11
  shrnOp rnd d28 q12 2
  nrnd rnd (vadd .u16 q1 q1 q13)
  shrnOp rnd d29 q1 2
  if avg then
    vld1 8 [q8] (al r0 128)
    vrhadd .u8 q14 q14 q8
  vaddl .u8 q8 d0 d30
  vld1 8 [d2, d3, d4] r1 r2
  vaddl .u8 q10 d1 d31
  vst1 8 [q14] (al r0 128) r2
  vadd .u16 q12 q8 q9
  pld (mem r1 r2)
  nrnd rnd (vadd .u16 q12 q12 q13)
  vext 8 q2 q1 q2 1
  vadd .u16 q0 q10 q11
  shrnOp rnd d30 q12 2
  nrnd rnd (vadd .u16 q0 q0 q13)
  shrnOp rnd d31 q0 2
  if avg then
    vld1 8 [q9] (al r0 128)
    vrhadd .u8 q15 q15 q9
  vaddl .u8 q9 d2 d4
  vaddl .u8 q11 d3 d5
  vst1 8 [q15] (al r0 128) r2
  bgt (back 1)
  vld1 8 [d0, d1, d2] r1 r2
  vadd .u16 q12 q8 q9
  nrnd rnd (vadd .u16 q12 q12 q13)
  vext 8 q15 q0 q1 1
  vadd .u16 q1 q10 q11
  shrnOp rnd d28 q12 2
  nrnd rnd (vadd .u16 q1 q1 q13)
  shrnOp rnd d29 q1 2
  if avg then
    vld1 8 [q8] (al r0 128)
    vrhadd .u8 q14 q14 q8
  vaddl .u8 q8 d0 d30
  vaddl .u8 q10 d1 d31
  vst1 8 [q14] (al r0 128) r2
  vadd .u16 q12 q8 q9
  nrnd rnd (vadd .u16 q12 q12 q13)
  vadd .u16 q0 q10 q11
  shrnOp rnd d30 q12 2
  nrnd rnd (vadd .u16 q0 q0 q13)
  shrnOp rnd d31 q0 2
  if avg then
    vld1 8 [q9] (al r0 128)
    vrhadd .u8 q15 q15 q9
  vst1 8 [q15] (al r0 128) r2
  bx lr

def pixels8 (rnd avg : Bool) : Asm Unit := do
  lbl 1
  vld1 8 [d0] r1 r2
  vld1 8 [d1] r1 r2
  vld1 8 [d2] r1 r2
  pld (mem r1 (r2.lsl 2))
  vld1 8 [d3] r1 r2
  pld r1
  pld (mem r1 r2)
  pld (mem r1 (r2.lsl 1))
  if avg then
    vld1 8 [d4] (al r0 64) r2
    vrhadd .u8 d0 d0 d4
    vld1 8 [d5] (al r0 64) r2
    vrhadd .u8 d1 d1 d5
    vld1 8 [d6] (al r0 64) r2
    vrhadd .u8 d2 d2 d6
    vld1 8 [d7] (al r0 64) r2
    vrhadd .u8 d3 d3 d7
    sub r0 r0 (r2.lsl 2)
  subs r3 r3 4
  vst1 8 [d0] (al r0 64) r2
  vst1 8 [d1] (al r0 64) r2
  vst1 8 [d2] (al r0 64) r2
  vst1 8 [d3] (al r0 64) r2
  bne (back 1)
  bx lr

def pixels8_x2 (rnd avg : Bool) : Asm Unit := do
  lbl 1
  vld1 8 [q0] r1 r2
  vext 8 d1 d0 d1 1
  vld1 8 [q1] r1 r2
  vext 8 d3 d2 d3 1
  pld r1
  pld (mem r1 r2)
  subs r3 r3 2
  vswp d1 d2
  avgOp rnd q0 q0 q1
  if avg then
    vld1 8 [d4] (al r0 64) r2
    vld1 8 [d5] (al r0 64)
    vrhadd .u8 q0 q0 q2
    sub r0 r0 r2
  vst1 8 [d0] (al r0 64) r2
  vst1 8 [d1] (al r0 64) r2
  bne (back 1)
  bx lr

def pixels8_y2 (rnd avg : Bool) : Asm Unit := do
  sub r3 r3 2
  vld1 8 [d0] r1 r2
  vld1 8 [d1] r1 r2
  lbl 1
  subs r3 r3 2
  avgOp rnd d4 d0 d1
  vld1 8 [d0] r1 r2
  avgOp rnd d5 d0 d1
  vld1 8 [d1] r1 r2
  pld r1
  pld (mem r1 r2)
  if avg then
    vld1 8 [d2] (al r0 64) r2
    vld1 8 [d3] (al r0 64)
    vrhadd .u8 q2 q2 q1
    sub r0 r0 r2
  vst1 8 [d4] (al r0 64) r2
  vst1 8 [d5] (al r0 64) r2
  bne (back 1)
  avgOp rnd d4 d0 d1
  vld1 8 [d0] r1 r2
  avgOp rnd d5 d0 d1
  if avg then
    vld1 8 [d2] (al r0 64) r2
    vld1 8 [d3] (al r0 64)
    vrhadd .u8 q2 q2 q1
    sub r0 r0 r2
  vst1 8 [d4] (al r0 64) r2
  vst1 8 [d5] (al r0 64) r2
  bx lr

def pixels8_xy2 (rnd avg : Bool) : Asm Unit := do
  sub r3 r3 2
  vld1 8 [q0] r1 r2
  vld1 8 [q1] r1 r2
  nrnd rnd (vmovImm .i16 q11 1)
  pld r1
  pld (mem r1 r2)
  vext 8 d4 d0 d1 1
  vext 8 d6 d2 d3 1
  vaddl .u8 q8 d0 d4
  vaddl .u8 q9 d2 d6
  lbl 1
  subs r3 r3 2
  vld1 8 [q0] r1 r2
  pld r1
  vadd .u16 q10 q8 q9
  vext 8 d4 d0 d1 1
  nrnd rnd (vadd .u16 q10 q10 q11)
  vaddl .u8 q8 d0 d4
  shrnOp rnd d5 q10 2
  vld1 8 [q1] r1 r2
  vadd .u16 q10 q8 q9
  pld (mem r1 r2)
  if avg then
    vld1 8 [d7] (al r0 64)
    vrhadd .u8 d5 d5 d7
  nrnd rnd (vadd .u16 q10 q10 q11)
  vst1 8 [d5] (al r0 64) r2
  shrnOp rnd d7 q10 2
  if avg then
    vld1 8 [d5] (al r0 64)
    vrhadd .u8 d7 d7 d5
  vext 8 d6 d2 d3 1
  vaddl .u8 q9 d2 d6
  vst1 8 [d7] (al r0 64) r2
  bgt (back 1)
  vld1 8 [q0] r1 r2
  vadd .u16 q10 q8 q9
  vext 8 d4 d0 d1 1
  nrnd rnd (vadd .u16 q10 q10 q11)
  vaddl .u8 q8 d0 d4
  shrnOp rnd d5 q10 2
  vadd .u16 q10 q8 q9
  if avg then
    vld1 8 [d7] (al r0 64)
    vrhadd .u8 d5 d5 d7
  nrnd rnd (vadd .u16 q10 q10 q11)
  vst1 8 [d5] (al r0 64) r2
  shrnOp rnd d7 q10 2
  if avg then
    vld1 8 [d5] (al r0 64)
    vrhadd .u8 d7 d7 d5
  vst1 8 [d7] (al r0 64) r2
  bx lr

/-- `pixfunc pfx, name, suf, rnd, avg`: the function `ff_<pfx><name><suf>_neon`
whose body is the macro `name`. -/
def pixfunc (pfx name : String) (pixels : Bool → Bool → Asm Unit) (suf := "") (rnd := true)
    (avg := false) : Asm Unit :=
  function s!"ff_{pfx}{name}{suf}_neon" (global := true) do
    pixels rnd avg

/-- Both rounding modes: `ff_<pfx><name>_neon` and `ff_<pfx><name>_no_rnd_neon`. -/
def pixfunc2 (pfx name : String) (pixels : Bool → Bool → Asm Unit) (avg := false) : Asm Unit := do
  pixfunc pfx name pixels (rnd := true) (avg := avg)
  pixfunc pfx name pixels "_no_rnd" (rnd := false) (avg := avg)

-- The `mc00` entry points set the height and fall through into the function after them.
def ff_put_h264_qpel16_mc00_neon : Asm Unit := function "ff_put_h264_qpel16_mc00_neon" (global := true) do
  mov r3 16

def ff_avg_h264_qpel16_mc00_neon : Asm Unit := function "ff_avg_h264_qpel16_mc00_neon" (global := true) do
  mov r3 16

def ff_put_h264_qpel8_mc00_neon : Asm Unit := function "ff_put_h264_qpel8_mc00_neon" (global := true) do
  mov r3 8

def ff_avg_h264_qpel8_mc00_neon : Asm Unit := function "ff_avg_h264_qpel8_mc00_neon" (global := true) do
  mov r3 8

def file : AsmFile where
  name := "hpeldsp_neon"
  copyright := ["ARM NEON optimised DSP functions", "Copyright (c) 2008 Mans Rullgard <mans@mansr.com>"]
  module := "ZrAsm.Ffmpeg.HpelDsp"
  body := do
    ff_put_h264_qpel16_mc00_neon
    pixfunc "put_" "pixels16" pixels16 (avg := false)
    pixfunc2 "put_" "pixels16_x2" pixels16_x2 (avg := false)
    pixfunc2 "put_" "pixels16_y2" pixels16_y2 (avg := false)
    pixfunc2 "put_" "pixels16_xy2" pixels16_xy2 (avg := false)
    ff_avg_h264_qpel16_mc00_neon
    pixfunc "avg_" "pixels16" pixels16 (avg := true)
    pixfunc2 "avg_" "pixels16_x2" pixels16_x2 (avg := true)
    pixfunc2 "avg_" "pixels16_y2" pixels16_y2 (avg := true)
    pixfunc2 "avg_" "pixels16_xy2" pixels16_xy2 (avg := true)
    ff_put_h264_qpel8_mc00_neon
    pixfunc "put_" "pixels8" pixels8 (avg := false)
    pixfunc2 "put_" "pixels8_x2" pixels8_x2 (avg := false)
    pixfunc2 "put_" "pixels8_y2" pixels8_y2 (avg := false)
    pixfunc2 "put_" "pixels8_xy2" pixels8_xy2 (avg := false)
    ff_avg_h264_qpel8_mc00_neon
    pixfunc "avg_" "pixels8" pixels8 (avg := true)
    pixfunc "avg_" "pixels8_x2" pixels8_x2 (avg := true)
    pixfunc "avg_" "pixels8_y2" pixels8_y2 (avg := true)
    pixfunc "avg_" "pixels8_xy2" pixels8_xy2 (avg := true)

end ZrAsm.Ffmpeg.HpelDsp
