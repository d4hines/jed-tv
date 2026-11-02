/* Linked at zero for relocation testing. This is not a TV load address. */
ENTRY(leanboot_main)
SECTIONS
{
  . = 0;
  .text : ALIGN(4) { KEEP(*(.text.leanboot_main)) *(.text .text.*) }
  .rodata : ALIGN(4) { *(.rodata .rodata.*) }
  .forbidden : { *(.data .data.* .bss .bss.* COMMON .got .got.*) }
  ASSERT(SIZEOF(.forbidden) == 0, "RAM probe cannot use globals, BSS or GOT")
  /DISCARD/ : { *(.ARM.exidx* .ARM.extab* .comment .note* .eh_frame*) }
}
