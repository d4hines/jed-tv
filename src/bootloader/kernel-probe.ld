ENTRY(leanboot_kernel_probe_start)
SECTIONS {
  . = 0;
  .text : { KEEP(*(.text.leanboot_kernel_probe_start)) *(.text*) }
  .rodata : { *(.rodata*) }
  .forbidden : { *(.data*) *(.bss*) *(.got*) }
  ASSERT(SIZEOF(.forbidden) == 0, "Writable globals are forbidden")
  /DISCARD/ : { *(.ARM.exidx*) *(.ARM.extab*) *(.comment*) *(.note*) }
}
