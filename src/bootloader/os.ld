ENTRY(leanboot_os_start)
SECTIONS
{
  . = 0;
  .text : ALIGN(4) { KEEP(*(.text.leanboot_os_start)) *(.text .text.*) }
  .rodata : ALIGN(4) { *(.rodata .rodata.*) }
  .forbidden : { *(.data .data.* .bss .bss.* COMMON .got .got.*) }
  ASSERT(SIZEOF(.forbidden) == 0, "OS loader cannot use globals, BSS or GOT")
  /DISCARD/ : { *(.ARM.exidx* .ARM.extab* .comment .note* .eh_frame*) }
}
