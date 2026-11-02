ENTRY(leanboot_gate_run)
SECTIONS {
  . = 0;
  .text : {
    KEEP(*(.text.leanboot_gate_run))
    *(.text*)
  }
  .rodata : { *(.rodata*) }
  .forbidden : { *(.data*) *(.bss*) *(.got*) }
  ASSERT(SIZEOF(.forbidden) == 0, "Writable globals are forbidden")
  ASSERT(leanboot_gate_run == 0, "Relocated entry offset changed")
  ASSERT(. < 0x40000, "Gate exceeds private code reservation")
  /DISCARD/ : { *(.ARM.exidx*) *(.ARM.extab*) *(.comment*) *(.note*) }
}
