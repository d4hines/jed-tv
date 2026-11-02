/* An offline function library, not a board reset/boot entry. */
ENTRY(leanboot_update_install)
SECTIONS {
  . = 0;
  .text : ALIGN(4) { *(.text .text.*) }
  .rodata : ALIGN(4) { *(.rodata .rodata.*) }
  .forbidden : { *(.data .data.* .bss .bss.* COMMON .got .got.*) }
  ASSERT(SIZEOF(.forbidden) == 0, "Updater must not allocate writable globals")
  /DISCARD/ : { *(.ARM.exidx* .ARM.extab* .comment .note* .eh_frame*) }
}
