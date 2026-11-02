//! Entry, MMU/caches, stacks, BSS and exception vectors.
//!
//! QEMU `-kernel <elf>` on virt enters `_start` at EL1 with the MMU off on
//! CPU 0; the other CPUs stay powered off until PSCI CPU_ON (see smp.rs),
//! which enters `_secondary_entry` with x0 = core index. Both paths enable
//! FP/SIMD, then turn on the MMU with one shared identity map made of 1 GiB
//! blocks: device memory below 1 GiB (flash, GIC, UART, virtio-mmio),
//! normal write-back cacheable memory for RAM. The table is a constant in
//! .rodata, so no code runs with caches off except these few instructions.

use core::arch::global_asm;

const BLOCK: u64 = 0b01; // level-1 block descriptor
const AF: u64 = 1 << 10;
const ATTR_DEVICE: u64 = 0 << 2; // MAIR attr0 = Device-nGnRE
const ATTR_NORMAL: u64 = 1 << 2; // MAIR attr1 = Normal WB RW-allocate
const INNER_SHAREABLE: u64 = 3 << 8;
const PXN_UXN: u64 = (1 << 53) | (1 << 54);
const DEVICE: u64 = BLOCK | AF | ATTR_DEVICE | PXN_UXN;
const NORMAL: u64 = BLOCK | AF | ATTR_NORMAL | INNER_SHAREABLE;

#[repr(C, align(4096))]
pub struct Table(pub [u64; 512]);

/// Level-1 table for a 39-bit VA space (T0SZ=25): entry n maps n GiB.
#[unsafe(no_mangle)]
pub static BOOT_L1: Table = {
    let mut t = [0u64; 512];
    t[0] = DEVICE; // 0x0000_0000: flash, GIC, PL011, RTC, virtio-mmio
    t[1] = NORMAL | 0x4000_0000; // RAM (QEMU virt RAM base)
    t[2] = NORMAL | 0x8000_0000; // RAM above 1 GiB (we run with 1430 MiB)
    Table(t)
};

global_asm!(
    r#"
.equ MAIR_VALUE, 0xff04
// T0SZ=25 | IRGN0=WBWA | ORGN0=WBWA | SH0=inner | TG0=4K | EPD1 | IPS=36 bits
.equ TCR_VALUE, 0x100803519

.section .text.boot, "ax"
.global _start
_start:
    mrs     x1, mpidr_el1
    and     x1, x1, #0xff
    cbz     x1, 1f
0:  wfe                         // unexpected secondary: park it
    b       0b
1:  mov     x0, #0              // core 0
    b       common_entry

.global _secondary_entry
_secondary_entry:               // x0 = core index (PSCI context id)
    b       common_entry

common_entry:
    mov     x19, x0
    // FP/SIMD on: the aarch64-unknown-none target uses NEON registers.
    mov     x1, #(3 << 20)
    msr     cpacr_el1, x1
    isb
    // MMU: attributes, translation control, identity table.
    ldr     x1, =MAIR_VALUE
    msr     mair_el1, x1
    ldr     x1, =TCR_VALUE
    msr     tcr_el1, x1
    adrp    x1, BOOT_L1
    add     x1, x1, :lo12:BOOT_L1
    msr     ttbr0_el1, x1
    isb
    tlbi    vmalle1
    ic      iallu
    dsb     nsh
    isb
    mrs     x1, sctlr_el1
    orr     x1, x1, #(1 << 0)       // M: MMU on
    orr     x1, x1, #(1 << 2)       // C: data cache on
    orr     x1, x1, #(1 << 12)      // I: instruction cache on
    bic     x1, x1, #(1 << 1)       // A: allow unaligned access to normal memory
    bic     x1, x1, #(1 << 19)      // WXN off
    msr     sctlr_el1, x1
    isb
    // Stack: 256 KiB per core.
    adrp    x1, __stacks_start
    add     x1, x1, :lo12:__stacks_start
    add     x2, x19, #1
    lsl     x2, x2, #18
    add     sp, x1, x2
    // Exception vectors.
    adrp    x1, exception_vectors
    add     x1, x1, :lo12:exception_vectors
    msr     vbar_el1, x1
    isb
    cbnz    x19, 3f
    // Core 0 clears BSS (now through the cache) and enters Rust.
    adrp    x1, __bss_start
    add     x1, x1, :lo12:__bss_start
    adrp    x2, __bss_end
    add     x2, x2, :lo12:__bss_end
2:  cmp     x1, x2
    b.hs    4f
    stp     xzr, xzr, [x1], #16
    b       2b
4:  bl      rust_main
    b       0b
3:  mov     x0, x19
    bl      rust_secondary
    b       0b

// Vector table: 16 entries of 0x80 bytes. Every exception is fatal in this
// spike (IRQs stay masked; interrupts are used only as WFI wake-ups).
.macro VENTRY idx
.balign 0x80
    stp     x29, x30, [sp, #-16]!
    mov     x0, #\idx
    mrs     x1, esr_el1
    mrs     x2, elr_el1
    mrs     x3, far_el1
    mov     x4, x30
    bl      exception_fatal
5:  wfe
    b       5b
.endm

.section .text.vectors, "ax"
.balign 0x800
exception_vectors:
    VENTRY 0
    VENTRY 1
    VENTRY 2
    VENTRY 3
    VENTRY 4
    VENTRY 5
    VENTRY 6
    VENTRY 7
    VENTRY 8
    VENTRY 9
    VENTRY 10
    VENTRY 11
    VENTRY 12
    VENTRY 13
    VENTRY 14
    VENTRY 15
"#
);

#[unsafe(no_mangle)]
extern "C" fn exception_fatal(kind: u64, esr: u64, elr: u64, far: u64, lr: u64) -> ! {
    const KINDS: [&str; 4] = ["sync", "irq", "fiq", "serror"];
    const FROM: [&str; 4] = ["EL1t", "EL1h", "EL0/64", "EL0/32"];
    crate::println!(
        "EXCEPTION {} from {}: ESR={:#x} (EC={:#x}) ELR={:#x} FAR={:#x} LR={:#x}",
        KINDS[(kind & 3) as usize], FROM[(kind >> 2) as usize], esr, esr >> 26, elr, far, lr
    );
    crate::psci::system_off();
}
