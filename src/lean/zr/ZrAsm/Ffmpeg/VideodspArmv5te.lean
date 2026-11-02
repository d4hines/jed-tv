import ZrAsm.Ffmpeg.Common

/-!
# Prefetch (FFmpeg libavcodec/arm/videodsp_armv5te.S)

Copyright (c) 2004 AGAWA Koji <i (AT) atty (DOT) jp>

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

namespace ZrAsm.Ffmpeg.VideodspArmv5te

open ZrAsm Asm Reg VReg

def ff_prefetch_arm : Asm Unit := function "ff_prefetch_arm" (global := true) do
  lbl 1
  subs r2 r2 1
  pld r0
  add r0 r0 r1
  bne (back 1)
  bx lr

def file : AsmFile where
  name := "videodsp_armv5te"
  copyright := ["Copyright (c) 2004 AGAWA Koji <i (AT) atty (DOT) jp>"]
  module := "ZrAsm.Ffmpeg.VideodspArmv5te"
  body := do
    ff_prefetch_arm

end ZrAsm.Ffmpeg.VideodspArmv5te
