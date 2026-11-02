import ZrAsm.Neon

/-!
# ZrAsm.Yuv: the YUV 4:2:0 to RGB row conversion in NEON

`zr_yuv420_row_neon` converts rows for the Lean apps' blit (`ZrTv.Video`,
assembled into the runtime's `tv.S`; BT.709 limited range, 10-bit fixed
point; first written for the Rust player's yuv.rs), whole blocks of 16
pixels (8 chroma samples) of one row:

    void zr_yuv420_row_neon(const uint8_t *y, const uint8_t *u, const uint8_t *v,
                            uint32_t *out, size_t blocks, const struct zr_yuv_k *k);

`k` (built by `ZrTv.Video.putConsts`) holds, in this order: the `int16` coefficients
`KY KRV KGU KGV KBU` (padded to 8), the `int32` constants `CR CG CB` and the
alpha bits, and the `uint32` multipliers `1 << rs`, `1 << gs`, `1 << bs`
(padded to 4). Per pixel with luma `Y` and the chroma pair's `U`, `V`,

    R = clamp(((Y - 16) * KY + 512 + (V - 128) * KRV) >> 10, 0, 255)

(and G, B likewise) and packs `(R << rs) | (G << gs) | (B << bs) | alpha`.
Here the constant offsets are folded into `CR = 512 - 16 KY - 128 KRV`,
`CG = 512 - 16 KY + 128 KGU + 128 KGV`, `CB = 512 - 16 KY - 128 KBU`, so
`R = clamp((Y KY + V KRV + CR) >> 10, 0, 255)`: the same integers, all in
32-bit lanes. The clamp of `x >> 10` to 0..255 is `vqshlu.s32 #14` (negative
to 0, `x >= 2^18` to all ones) then `vshr.u32 #24`. The fields are placed
by multiply-accumulating onto the alpha bits, which equals an OR because
the blit only calls this when the four fields are disjoint (`layoutOk`).

Even and odd pixels are split by `vld2.8`, so the chroma sample `i` lines up
with luma lanes `i` of both halves, and joined again by `vst2.32`.
Registers: q0-q2 the constants, q3 the B chroma term, q4 `Y * KY`, q5 a
channel, q6/q7 the even/odd pixels, q8/q9 the loaded bytes, q10-q13 them
widened (even Y, odd Y, U, V), q14/q15 the R and G chroma terms. q4-q7 are
callee-saved (d8-d15) and pushed.

Not interpreted by `ZrAsm.Sem` (NEON is stuck there); the Lean app's
blit is checked by playback in the QEMU demo (test/check.py, demo/run.py).
-/

namespace ZrAsm.Yuv

open ZrAsm Asm Reg VReg

/-- One channel of four pixels: `dst += clamp((ys + c) >> 10, 0, 255) * mul`. -/
def channel (dst ys c : VReg) (mul : Scalar) : Asm Unit := do
  vadd .i32 q5 ys c
  vqshlu .s32 q5 q5 14
  vshr .u32 q5 q5 24
  vmla .i32 dst q5 mul

/-- Four pixels into `dst` from the luma lanes `y` and the chroma terms in q14, q15, q3. -/
def pixels (dst y : VReg) : Asm Unit := do
  vmull .s16 q4 y d0[0]           -- Y * KY
  vdup 32 dst d3[1]               -- alpha
  channel dst q4 q14 d4[0]        -- R << rs
  channel dst q4 q15 d4[1]        -- G << gs
  channel dst q4 q3 d5[0]         -- B << bs

/-- Eight pixels (four chroma samples) from the luma lanes `ye`, `yo` and chroma `u`, `v`. -/
def half (ye yo u v : VReg) : Asm Unit := do
  vdup 32 q14 d2[0]               -- CR + KRV * V
  vmlal .s16 q14 v d0[1]
  vdup 32 q15 d2[1]               -- CG - KGU * U - KGV * V
  vmlsl .s16 q15 u d0[2]
  vmlsl .s16 q15 v d0[3]
  vdup 32 q3 d3[0]                -- CB + KBU * U
  vmlal .s16 q3 u d1[0]
  pixels q6 ye
  pixels q7 yo
  vst2 32 [q6, q7] r3 .wb         -- even/odd interleaved

def row : Routine := routine "zr_yuv420_row_neon" do
  ldr r12 (mem sp 4)              -- k
  vld1 16 [q0] r12 .wb            -- KY KRV KGU KGV KBU
  vld1 32 [q1] r12 .wb            -- CR CG CB alpha
  vld1 32 [q2] r12                -- 1 << rs, 1 << gs, 1 << bs
  ldr r12 sp                      -- blocks
  vpush (drange 8 15)
  lbl 1
  vld2 8 [d16, d17] r0 .wb        -- Y: 8 even, 8 odd pixels
  vld1 8 [d18] r1 .wb             -- U
  vld1 8 [d19] r2 .wb             -- V
  vmovl .u8 q10 d16
  vmovl .u8 q11 d17
  vmovl .u8 q12 d18
  vmovl .u8 q13 d19
  half d20 d22 d24 d26
  half d21 d23 d25 d27
  subs r12 r12 1
  bne (back 1)
  vpop (drange 8 15)
  bx lr

/-- Nearest-neighbour horizontal scaling of one row of pixels, for the Lean
video blit (ZrTv.Video):

    void zr_u32_scale_row(uint32_t *dst, const uint32_t *src, size_t n,
                          size_t w, size_t q, size_t r);

writes `dst[x] = src[x * sw / w]` for `x < n` (`n ≥ 1`), where the caller
passes `q = sw / w` and `r = sw % w`. The source index `sx` and the
remainder `rem` of `x * sw / w` advance by `q` and `r`, and a carry when
`rem` reaches `w` (one subtraction suffices: `rem + r < 2 w`), so every index
is exact without a division. Plain ARM (no NEON), so `ZrAsm.Sem` runs it:
`test/yuv/ScaleRow.lean` compares it with the formula. The loop label is a
`.L` name because Sem does not model numeric local labels. -/
def scaleRow : Routine := routine "zr_u32_scale_row" do
  push [r4, r5, r6, r7]
  ldr r4 (mem sp 16)              -- q
  ldr r5 (mem sp 20)              -- r
  mov r6 0                        -- sx
  mov r7 0                        -- rem
  label ".Lzr_u32_scale_row_loop"
  ldr ip (mem r1 (r6.lsl 2))
  str ip (post r0 4)
  add r6 r6 r4
  add r7 r7 r5
  cmp r7 r3
  sub r7 r7 r3 .cs
  add r6 r6 1 .cs
  subs r2 r2 1
  b ".Lzr_u32_scale_row_loop" .ne
  pop [r4, r5, r6, r7]
  bx lr

/-- ZrTv's assembly (runtime build, `tv.S`): the row kernel and the scaler,
for `ZrTv.Video.blit`. -/
def tvText : String :=
  "/* ZrTv's video routines, written in the ZrAsm Lean DSL\n" ++
  " * (src/lean/zr/ZrAsm/Yuv.lean). Generated by zr-asm-gen; do not edit. */\n" ++
  printItems (.syntaxUnified :: .fpu "neon" :: row.items ++ scaleRow.items)

end ZrAsm.Yuv
