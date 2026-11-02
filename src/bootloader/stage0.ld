ENTRY(leanboot_bundle_start)
SECTIONS {
  . = 0;
  .text : { KEEP(*(.text.leanboot_bundle_start)) *(.text*) }
  .rodata : { *(.rodata*) }
  .forbidden : { *(.data*) *(.bss*) *(.got*) }
  ASSERT(SIZEOF(.forbidden) == 0, "Writable globals are forbidden")
  ASSERT(. <= 0x2000, "Stage0 overlaps the bundle header")
  /DISCARD/ : { *(.ARM.exidx*) *(.ARM.extab*) *(.comment*) *(.note*) }
}
