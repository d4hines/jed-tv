.syntax unified
.arch armv7-a
.arm
.section .text
.global _start
_start:
    cpsid if
    movw r0, #0x0c04
    movt r0, #0x1f00
    mov r1, #0
    str r1, [r0]
    dsb sy
    movw r0, #0x6000
    movt r0, #0x1f00
    mov r1, #1
    mov r2, #0
    strh r1, [r0, #0x10]
    strh r2, [r0, #0x14]
    dsb sy
    strb r1, [r0]
    dsb sy
1:
    b 1b
