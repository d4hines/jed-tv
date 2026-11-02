/*
 * C variadic entry points for the Rust translation of FFmpeg (AArch64,
 * AAPCS64 / Linux). Stable Rust can't define `...` functions, so each one
 * here builds a va_list exactly as clang's va_start would and calls the
 * translated body `<name>__va(fixed..., va_list)`.
 *
 * Frame (240 bytes):  [sp]      x29, x30
 *                     [sp+16]   va_list { __stack, __gr_top, __vr_top, __gr_offs, __vr_offs }
 *                     [sp+48]   x0-x7 save area   (__gr_top = sp+112)
 *                     [sp+112]  q0-q7 save area   (__vr_top = sp+240)
 * All fixed arguments are int/pointer, so every FP register may hold a vararg.
 */

/* K fixed int/pointer args (K <= 7); the va_list goes by reference in xK. */
.macro  vararg_tramp name, k
        .section .text.\name,"ax",%progbits
        .global \name
        .type   \name, %function
        .p2align 2
\name:
        sub     sp, sp, #240
        stp     x29, x30, [sp]
        mov     x29, sp
        stp     x0, x1, [sp, #48]
        stp     x2, x3, [sp, #64]
        stp     x4, x5, [sp, #80]
        stp     x6, x7, [sp, #96]
        stp     q0, q1, [sp, #112]
        stp     q2, q3, [sp, #144]
        stp     q4, q5, [sp, #176]
        stp     q6, q7, [sp, #208]
        add     x9, sp, #240
        str     x9, [sp, #16]           // __stack: first stacked vararg
        add     x9, sp, #112
        str     x9, [sp, #24]           // __gr_top
        add     x9, sp, #240
        str     x9, [sp, #32]           // __vr_top
        mov     w9, #(-(64 - 8 * \k))
        mov     w10, #-128
        stp     w9, w10, [sp, #40]      // __gr_offs, __vr_offs
        add     x\k, sp, #16
        bl      \name\()__va
        ldp     x29, x30, [sp]
        add     sp, sp, #240
        ret
        .size   \name, . - \name
.endm

vararg_tramp av_log, 3
vararg_tramp av_log_once, 5
vararg_tramp av_strlcatf, 3
vararg_tramp av_asprintf, 1
vararg_tramp av_bprintf, 2
vararg_tramp avpriv_request_sample, 2
vararg_tramp avpriv_report_missing_feature, 2
vararg_tramp avpriv_open, 2
vararg_tramp av_sscanf, 2

        .section .note.GNU-stack,"",%progbits
