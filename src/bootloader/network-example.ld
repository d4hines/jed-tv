ENTRY(leanboot_network_example)
SECTIONS {
  . = 0;
  .text : { KEEP(*(.text.leanboot_network_example)) *(.text .text.*) }
  .rodata : { *(.rodata .rodata.*) }
  .forbidden : { *(.data .data.* .bss .bss.* COMMON .got .got.*) }
  ASSERT(SIZEOF(.forbidden) == 0, "Network example must be position independent")
  /DISCARD/ : { *(.ARM.exidx* .ARM.extab* .comment .note* .eh_frame*) }
}
