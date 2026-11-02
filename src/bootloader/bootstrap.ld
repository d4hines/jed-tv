ENTRY(leanboot_bootstrap_probe)
SECTIONS
{
  . = 0;
  .text : ALIGN(4) { KEEP(*(.text.leanboot_bootstrap_probe)) *(.text .text.*) }
  .rodata : ALIGN(4) { *(.rodata .rodata.*) }
  .forbidden : { *(.data .data.* .bss .bss.* COMMON .got .got.*) }
  ASSERT(SIZEOF(.forbidden) == 0, "Bootstrap probe cannot use globals, BSS or GOT")
  /DISCARD/ : { *(.ARM.exidx* .ARM.extab* .comment .note* .eh_frame*) }
}
