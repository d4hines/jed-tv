/*
 * C variadic entry points for the Rust translation of FFmpeg (32-bit ARM,
 * AAPCS-VFP; variadic arguments always use core registers and the stack).
 * Stable Rust can't define `...` functions, so each one here builds a
 * va_list (`struct { void *__ap; }`) exactly as clang's va_start would and
 * calls the translated body `<name>__va(fixed..., va_list)`.
 */
        .syntax unified
        .arm

/* K fixed int/pointer args (K <= 3): spill r0-r3 next to the caller's stack
 * arguments (8-byte aligned, as va_start does), pass &first-vararg in rK. */
.macro  vararg_tramp name, k
        .section .text.\name,"ax",%progbits
        .global \name
        .type   \name, %function
        .p2align 2
\name:
        push    {r0-r3}
        push    {r4, lr}
        add     r\k, sp, #(8 + 4 * \k)
        bl      \name\()__va
        pop     {r4, lr}
        add     sp, sp, #16
        bx      lr
        .size   \name, . - \name
.endm

vararg_tramp av_log, 3
vararg_tramp av_strlcatf, 3
vararg_tramp av_asprintf, 1
vararg_tramp av_bprintf, 2
vararg_tramp avpriv_request_sample, 2
vararg_tramp avpriv_report_missing_feature, 2
vararg_tramp avpriv_open, 2
vararg_tramp av_sscanf, 2

/* av_log_once(avcl, initial_level, subsequent_level, state, fmt, ...):
 * fmt is the first stack argument, the varargs follow it. */
        .section .text.av_log_once,"ax",%progbits
        .global av_log_once
        .type   av_log_once, %function
        .p2align 2
av_log_once:
        push    {r4, lr}
        ldr     r12, [sp, #8]           @ fmt
        add     r4, sp, #12             @ va_list -> first vararg
        sub     sp, sp, #8
        str     r12, [sp]
        str     r4, [sp, #4]
        bl      av_log_once__va
        add     sp, sp, #8
        pop     {r4, pc}
        .size   av_log_once, . - av_log_once

        .section .note.GNU-stack,"",%progbits
