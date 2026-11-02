.syntax unified
.arm
.global _start
.global capture_args
_start:
    mov r0, #17
    mov r1, #34
    ldr r2, =probe_words
    mov r3, #68
    blx capture_args
    mov r0, #0
    mov r7, #1
    svc #0
.thumb
.thumb_func
capture_args:
    bx lr
.align 2
probe_words:
    .word 0x11223344, 0x55667788, 0x99aabbcc, 0xddeeff00
.section .note.GNU-stack,"",%progbits
