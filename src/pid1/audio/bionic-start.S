.syntax unified
.arm
.text
.global _start
_start:
    mov r0, sp
    mov r1, #0
    ldr r2, 1f
.Lmain:
    add r2, pc, r2
    ldr r3, 2f
.Lstructors:
    add r3, pc, r3
    bl __libc_init
    b .
1:  .word main-(.Lmain+8)
2:  .word structors-(.Lstructors+8)
.data
.align 2
structors:
    .word 0,0,0
