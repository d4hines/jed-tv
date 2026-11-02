/* QEMU virt: RAM starts at 0x40000000. QEMU puts the DTB at the base of RAM
 * when an ELF is loaded above it (QEMU sizes it at 1 MiB unpacked), so the
 * image starts 2 MiB in. */
ENTRY(_start)

SECTIONS
{
    . = 0x40200000;
    __image_start = .;
    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }
    .rodata : ALIGN(8) { *(.rodata .rodata.*) }
    .data : ALIGN(4096) { *(.data .data.*) }
    .bss (NOLOAD) : ALIGN(64) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(64);
        __bss_end = .;
    }
    /* 4 cores x 256 KiB stacks, top of stack N = __stacks_start + (N+1)*256K */
    .stacks (NOLOAD) : ALIGN(4096) {
        __stacks_start = .;
        . += 4 * 256 * 1024;
        __stacks_end = .;
    }
    . = ALIGN(2 * 1024 * 1024);
    __heap_start = .;
    __image_end = .;
    /DISCARD/ : { *(.comment) *(.note .note.*) *(.eh_frame*) }
}
