import ZrAsm.Ffmpeg.Common

/-!
# H.264 chroma motion compensation (FFmpeg libavcodec/arm/h264cmc_neon.S)

Copyright (c) 2008 Mans Rullgard <mans@mansr.com>

This file is part of FFmpeg. FFmpeg is free software; you can redistribute
it and/or modify it under the terms of the GNU Lesser General Public License
as published by the Free Software Foundation; either version 2.1 of the
License, or (at your option) any later version. FFmpeg is distributed in the
hope that it will be useful, but WITHOUT ANY WARRANTY; without even the
implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
the GNU Lesser General Public License for more details.

FFmpeg's file as Lean: each GNU macro is a Lean function of the same name and
parameters, `.ifc` is `if`, and the numeric labels are FFmpeg's. The
`h264_chroma_mc*` macros also take a `codec` (`rv40`, `vc1`) for decoders
not configured (CONFIG_RV40_DECODER, CONFIG_VC1DSP are 0, so neither
those instances nor the `rv40bias` table are assembled); only the H.264
instances are written here. `test/ffmpeg/AsmCheck.lean` checks the result
against FFmpeg's original byte for byte.
-/

namespace ZrAsm.Ffmpeg.H264Cmc

open ZrAsm Asm Reg VReg

/-- `chroma_mc8(uint8_t *dst, const uint8_t *src, ptrdiff_t stride, int h, int x, int y)` -/
def h264_chroma_mc8 (type : String) : Asm Unit := do
  function s!"ff_{type}_h264_chroma_mc8_neon" (global := true) do
    push [r4, r5, r6, r7, lr]
    ldrd r4 r5 (mem sp 20)
    if type == "avg" then
      mov lr r0
    pld r1
    pld (mem r1 r2)
    muls r7 r4 r5
    rsb r6 r7 (r5.lsl 3)
    rsb r12 r7 (r4.lsl 3)
    sub r4 r7 (r4.lsl 3)
    sub r4 r4 (r5.lsl 3)
    add r4 r4 64
    beq (fwd 2)
    vdup 8 d0 r4
    vdup 8 d1 r12
    vld1 8 [d4, d5] r1 r2
    vdup 8 d2 r6
    vdup 8 d3 r7
    vext 8 d5 d4 d5 1
    lbl 1
    vld1 8 [d6, d7] r1 r2
    vmull .u8 q8 d4 d0
    vmlal .u8 q8 d5 d1
    vext 8 d7 d6 d7 1
    vld1 8 [d4, d5] r1 r2
    vmlal .u8 q8 d6 d2
    pld r1
    vext 8 d5 d4 d5 1
    vmlal .u8 q8 d7 d3
    vmull .u8 q9 d6 d0
    subs r3 r3 2
    vmlal .u8 q9 d7 d1
    vmlal .u8 q9 d4 d2
    vmlal .u8 q9 d5 d3
    pld (mem r1 r2)
    vrshrn .u16 d16 q8 6
    vrshrn .u16 d17 q9 6
    if type == "avg" then
      vld1 8 [d20] (al lr 64) r2
      vld1 8 [d21] (al lr 64) r2
      vrhadd .u8 q8 q8 q10
    vst1 8 [d16] (al r0 64) r2
    vst1 8 [d17] (al r0 64) r2
    bgt (back 1)
    pop [r4, r5, r6, r7, pc]
    lbl 2
    adds r12 r12 r6
    vdup 8 d0 r4
    beq (fwd 5)
    tst r6 r6
    vdup 8 d1 r12
    beq (fwd 4)
    vld1 8 [d4] r1 r2
    lbl 3
    vld1 8 [d6] r1 r2
    vmull .u8 q8 d4 d0
    vmlal .u8 q8 d6 d1
    vld1 8 [d4] r1 r2
    vmull .u8 q9 d6 d0
    vmlal .u8 q9 d4 d1
    pld r1
    vrshrn .u16 d16 q8 6
    vrshrn .u16 d17 q9 6
    pld (mem r1 r2)
    if type == "avg" then
      vld1 8 [d20] (al lr 64) r2
      vld1 8 [d21] (al lr 64) r2
      vrhadd .u8 q8 q8 q10
    subs r3 r3 2
    vst1 8 [d16] (al r0 64) r2
    vst1 8 [d17] (al r0 64) r2
    bgt (back 3)
    pop [r4, r5, r6, r7, pc]
    lbl 4
    vld1 8 [d4, d5] r1 r2
    vld1 8 [d6, d7] r1 r2
    vext 8 d5 d4 d5 1
    vext 8 d7 d6 d7 1
    pld r1
    subs r3 r3 2
    vmull .u8 q8 d4 d0
    vmlal .u8 q8 d5 d1
    vmull .u8 q9 d6 d0
    vmlal .u8 q9 d7 d1
    pld (mem r1 r2)
    vrshrn .u16 d16 q8 6
    vrshrn .u16 d17 q9 6
    if type == "avg" then
      vld1 8 [d20] (al lr 64) r2
      vld1 8 [d21] (al lr 64) r2
      vrhadd .u8 q8 q8 q10
    vst1 8 [d16] (al r0 64) r2
    vst1 8 [d17] (al r0 64) r2
    bgt (back 4)
    pop [r4, r5, r6, r7, pc]
    lbl 5
    vld1 8 [d4] r1 r2
    vld1 8 [d5] r1 r2
    pld r1
    subs r3 r3 2
    vmull .u8 q8 d4 d0
    vmull .u8 q9 d5 d0
    pld (mem r1 r2)
    vrshrn .u16 d16 q8 6
    vrshrn .u16 d17 q9 6
    if type == "avg" then
      vld1 8 [d20] (al lr 64) r2
      vld1 8 [d21] (al lr 64) r2
      vrhadd .u8 q8 q8 q10
    vst1 8 [d16] (al r0 64) r2
    vst1 8 [d17] (al r0 64) r2
    bgt (back 5)
    pop [r4, r5, r6, r7, pc]

/-- `chroma_mc4(uint8_t *dst, const uint8_t *src, ptrdiff_t stride, int h, int x, int y)` -/
def h264_chroma_mc4 (type : String) : Asm Unit := do
  function s!"ff_{type}_h264_chroma_mc4_neon" (global := true) do
    push [r4, r5, r6, r7, lr]
    ldrd r4 r5 (mem sp 20)
    if type == "avg" then
      mov lr r0
    pld r1
    pld (mem r1 r2)
    muls r7 r4 r5
    rsb r6 r7 (r5.lsl 3)
    rsb r12 r7 (r4.lsl 3)
    sub r4 r7 (r4.lsl 3)
    sub r4 r4 (r5.lsl 3)
    add r4 r4 64
    beq (fwd 2)
    vdup 8 d0 r4
    vdup 8 d1 r12
    vld1 8 [d4] r1 r2
    vdup 8 d2 r6
    vdup 8 d3 r7
    vext 8 d5 d4 d5 1
    vtrn 32 d4 d5
    vtrn 32 d0 d1
    vtrn 32 d2 d3
    lbl 1
    vld1 8 [d6] r1 r2
    vext 8 d7 d6 d7 1
    vtrn 32 d6 d7
    vmull .u8 q8 d4 d0
    vmlal .u8 q8 d6 d2
    vld1 8 [d4] r1 r2
    vext 8 d5 d4 d5 1
    vtrn 32 d4 d5
    pld r1
    vmull .u8 q9 d6 d0
    vmlal .u8 q9 d4 d2
    vadd .i16 d16 d16 d17
    vadd .i16 d17 d18 d19
    vrshrn .u16 d16 q8 6
    subs r3 r3 2
    pld (mem r1 r2)
    if type == "avg" then
      vld1 32 [d20[0]] (al lr 32) r2
      vld1 32 [d20[1]] (al lr 32) r2
      vrhadd .u8 d16 d16 d20
    vst1 32 [d16[0]] (al r0 32) r2
    vst1 32 [d16[1]] (al r0 32) r2
    bgt (back 1)
    pop [r4, r5, r6, r7, pc]
    lbl 2
    adds r12 r12 r6
    vdup 8 d0 r4
    beq (fwd 5)
    tst r6 r6
    vdup 8 d1 r12
    vtrn 32 d0 d1
    beq (fwd 4)
    vext 32 d1 d0 d1 1
    vld1 32 [d4[0]] r1 r2
    lbl 3
    vld1 32 [d4[1]] r1 r2
    vmull .u8 q8 d4 d0
    vld1 32 [d4[0]] r1 r2
    vmull .u8 q9 d4 d1
    vadd .i16 d16 d16 d17
    vadd .i16 d17 d18 d19
    pld r1
    vrshrn .u16 d16 q8 6
    if type == "avg" then
      vld1 32 [d20[0]] (al lr 32) r2
      vld1 32 [d20[1]] (al lr 32) r2
      vrhadd .u8 d16 d16 d20
    subs r3 r3 2
    pld (mem r1 r2)
    vst1 32 [d16[0]] (al r0 32) r2
    vst1 32 [d16[1]] (al r0 32) r2
    bgt (back 3)
    pop [r4, r5, r6, r7, pc]
    lbl 4
    vld1 8 [d4] r1 r2
    vld1 8 [d6] r1 r2
    vext 8 d5 d4 d5 1
    vext 8 d7 d6 d7 1
    vtrn 32 d4 d5
    vtrn 32 d6 d7
    vmull .u8 q8 d4 d0
    vmull .u8 q9 d6 d0
    subs r3 r3 2
    vadd .i16 d16 d16 d17
    vadd .i16 d17 d18 d19
    pld r1
    vrshrn .u16 d16 q8 6
    if type == "avg" then
      vld1 32 [d20[0]] (al lr 32) r2
      vld1 32 [d20[1]] (al lr 32) r2
      vrhadd .u8 d16 d16 d20
    pld r1
    vst1 32 [d16[0]] (al r0 32) r2
    vst1 32 [d16[1]] (al r0 32) r2
    bgt (back 4)
    pop [r4, r5, r6, r7, pc]
    lbl 5
    vld1 32 [d4[0]] r1 r2
    vld1 32 [d4[1]] r1 r2
    vmull .u8 q8 d4 d0
    subs r3 r3 2
    pld r1
    vrshrn .u16 d16 q8 6
    if type == "avg" then
      vld1 32 [d20[0]] (al lr 32) r2
      vld1 32 [d20[1]] (al lr 32) r2
      vrhadd .u8 d16 d16 d20
    pld r1
    vst1 32 [d16[0]] (al r0 32) r2
    vst1 32 [d16[1]] (al r0 32) r2
    bgt (back 5)
    pop [r4, r5, r6, r7, pc]

def h264_chroma_mc2 (type : String) : Asm Unit := do
  function s!"ff_{type}_h264_chroma_mc2_neon" (global := true) do
    push [r4, r5, r6, lr]
    ldr r4 (mem sp 16)
    ldr lr (mem sp 20)
    pld r1
    pld (mem r1 r2)
    orrs r5 r4 lr
    beq (fwd 2)
    mul r5 r4 lr
    rsb r6 r5 (lr.lsl 3)
    rsb r12 r5 (r4.lsl 3)
    sub r4 r5 (r4.lsl 3)
    sub r4 r4 (lr.lsl 3)
    add r4 r4 64
    vdup 8 d0 r4
    vdup 8 d2 r12
    vdup 8 d1 r6
    vdup 8 d3 r5
    vtrn 16 q0 q1
    lbl 1
    vld1 32 [d4[0]] r1 r2
    vld1 32 [d4[1]] r1 r2
    vrev64 32 d5 d4
    vld1 32 [d5[1]] r1
    vext 8 q3 q2 q2 1
    vtrn 16 q2 q3
    vmull .u8 q8 d4 d0
    vmlal .u8 q8 d5 d1
    if type == "avg" then
      vld1 16 [d18[0]] (al r0 16) r2
      vld1 16 [d18[1]] (al r0 16)
      sub r0 r0 r2
    vtrn 32 d16 d17
    vadd .i16 d16 d16 d17
    vrshrn .u16 d16 q8 6
    if type == "avg" then
      vrhadd .u8 d16 d16 d18
    vst1 16 [d16[0]] (al r0 16) r2
    vst1 16 [d16[1]] (al r0 16) r2
    subs r3 r3 2
    bgt (back 1)
    pop [r4, r5, r6, pc]
    lbl 2
    if type == "put" then
      ldrh r5 (post r1 r2)
      strh r5 (post r0 r2)
      ldrh r6 (post r1 r2)
      strh r6 (post r0 r2)
    else
      vld1 16 [d16[0]] r1 r2
      vld1 16 [d16[1]] r1 r2
      vld1 16 [d18[0]] (al r0 16) r2
      vld1 16 [d18[1]] (al r0 16)
      sub r0 r0 r2
      vrhadd .u8 d16 d16 d18
      vst1 16 [d16[0]] (al r0 16) r2
      vst1 16 [d16[1]] (al r0 16) r2
    subs r3 r3 2
    bgt (back 2)
    pop [r4, r5, r6, pc]

def file : AsmFile where
  name := "h264cmc_neon"
  copyright := ["Copyright (c) 2008 Mans Rullgard <mans@mansr.com>"]
  module := "ZrAsm.Ffmpeg.H264Cmc"
  body := do
    h264_chroma_mc8 "put"
    h264_chroma_mc8 "avg"
    h264_chroma_mc4 "put"
    h264_chroma_mc4 "avg"
    h264_chroma_mc2 "put"
    h264_chroma_mc2 "avg"

end ZrAsm.Ffmpeg.H264Cmc
