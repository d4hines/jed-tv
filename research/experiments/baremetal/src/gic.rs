//! Minimal GIC driver, v2 (TCG default for cortex-a55; also what a GIC-400
//! SoC has) and v3 (QEMU picks it under HVF). Interrupts are only used as
//! WFI wake-ups: PSTATE.I stays masked, so no handler ever runs; the loop
//! that slept acknowledges what woke it and then polls its devices.
//! SGI 0 is the inter-core "wake up" doorbell for the render cores.

use core::arch::asm;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, Ordering};

const GICD: usize = 0x0800_0000;
const GICC: usize = 0x0801_0000; // v2 CPU interface (banked per core)
const GICR: usize = 0x080a_0000; // v3 redistributors, 128 KiB per core

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Version {
    V2,
    V3,
}

static IS_V3: AtomicBool = AtomicBool::new(false);

fn rd(a: usize) -> u32 {
    unsafe { read_volatile(a as *const u32) }
}
fn wr(a: usize, v: u32) {
    unsafe { write_volatile(a as *mut u32, v) }
}
fn wr8(a: usize, v: u8) {
    unsafe { write_volatile(a as *mut u8, v) }
}
fn sgi_base(core: usize) -> usize {
    GICR + core * 0x2_0000 + 0x1_0000
}

pub fn version() -> Version {
    if IS_V3.load(Ordering::Relaxed) { Version::V3 } else { Version::V2 }
}

/// Detect the GIC, enable the distributor and core 0's CPU interface.
/// v2 if GICD_PIDR2 at 0xfe8 (v2 layout) says ArchRev 2; otherwise v3
/// (GICD_PIDR2 at 0xffe8). Probing 0xffe8 on QEMU's 4 KiB v2 distributor
/// is a synchronous external abort, and ID_AA64PFR0_EL1.GIC reads 0 under
/// HVF even with a GICv3, so the order matters.
pub fn init() -> Version {
    let v3 = (rd(GICD + 0xfe8) >> 4) & 0xf != 2 && (rd(GICD + 0xffe8) >> 4) & 0xf >= 3;
    IS_V3.store(v3, Ordering::Relaxed);
    if v3 {
        wr(GICD, 1 << 4); // ARE (security disabled: DS=1)
        while rd(GICD) & (1 << 31) != 0 {}
        wr(GICD, (1 << 4) | 0b11); // ARE | EnableGrp1 | EnableGrp0
    } else {
        wr(GICD, 1); // enable (group 0 -> IRQ)
    }
    init_cpu(0);
    version()
}

/// Per-core CPU interface (and v3 redistributor) setup; SGI 0 enabled.
pub fn init_cpu(core: usize) {
    if version() == Version::V3 {
        let rd_base = GICR + core * 0x2_0000;
        wr(rd_base + 0x14, rd(rd_base + 0x14) & !(1 << 1)); // WAKER.ProcessorSleep = 0
        while rd(rd_base + 0x14) & (1 << 2) != 0 {}
        wr(sgi_base(core) + 0x80, 0xffff_ffff); // SGIs/PPIs in group 1
        unsafe {
            asm!("mrs {t}, S3_0_C12_C12_5", "orr {t}, {t}, #1", "msr S3_0_C12_C12_5, {t}",
                 "isb", t = out(reg) _); // ICC_SRE_EL1.SRE
            asm!("msr S3_0_C4_C6_0, {}", in(reg) 0xffu64); // ICC_PMR_EL1
            asm!("msr S3_0_C12_C12_7, {}", "isb", in(reg) 1u64); // ICC_IGRPEN1_EL1
        }
    } else {
        wr(GICC + 0x4, 0xff); // PMR: let everything through
        wr(GICC, 1);
    }
    enable_local(core, 0);
}

/// Enable a per-core interrupt (SGI 0..16, PPI 16..32) on `core`.
pub fn enable_local(core: usize, id: u32) {
    let base = if version() == Version::V3 { sgi_base(core) } else { GICD }; // v2: banked
    wr8(base + 0x400 + id as usize, 0xa0);
    wr(base + 0x100, 1 << id);
}

/// Enable a shared peripheral interrupt (INTID >= 32), routed to core 0.
pub fn enable_spi(id: u32, edge: bool) {
    let (n, bit) = ((id / 32) as usize, 1u32 << (id % 32));
    wr8(GICD + 0x400 + id as usize, 0xa0);
    let cfg = GICD + 0xc00 + (id as usize / 16) * 4;
    let shift = (id % 16) * 2 + 1;
    wr(cfg, (rd(cfg) & !(1 << shift)) | ((edge as u32) << shift));
    if version() == Version::V3 {
        wr(GICD + 0x80 + n * 4, rd(GICD + 0x80 + n * 4) | bit); // group 1
        unsafe { write_volatile((GICD + 0x6000 + 8 * id as usize) as *mut u64, 0) }; // -> 0.0.0.0
    } else {
        wr8(GICD + 0x800 + id as usize, 1); // target core 0
    }
    wr(GICD + 0x100 + n * 4, bit);
}

/// Raise SGI `id` on every core except this one.
pub fn sgi_others(id: u32) {
    if version() == Version::V3 {
        let v = (1u64 << 40) | ((id as u64) << 24); // IRM: all but self
        unsafe { asm!("msr S3_0_C12_C11_5, {}", "isb", in(reg) v) }; // ICC_SGI1R_EL1
    } else {
        wr(GICD + 0xf00, (1 << 24) | id); // GICD_SGIR, filter: all but self
    }
}

/// Acknowledge one pending interrupt on this core and signal end-of-interrupt.
/// Returns the INTID, or None if nothing is pending.
pub fn ack() -> Option<u32> {
    let v3 = version() == Version::V3;
    let iar = if v3 {
        let v: u64;
        unsafe { asm!("mrs {}, S3_0_C12_C12_0", out(reg) v) }; // ICC_IAR1_EL1
        v as u32
    } else {
        rd(GICC + 0xc)
    };
    if iar & 0x3ff >= 1020 {
        return None;
    }
    if v3 {
        unsafe { asm!("msr S3_0_C12_C12_1, {}", in(reg) iar as u64) }; // ICC_EOIR1_EL1
    } else {
        wr(GICC + 0x10, iar); // includes the v2 source-CPU bits for SGIs
    }
    Some(iar & 0x3ff)
}

pub const WAKE_SGI: u32 = 0;
pub const TIMER_PPI: u32 = 27; // virtual timer
/// virtio-mmio slot n on QEMU virt raises SPI 16+n.
pub fn virtio_intid(slot: usize) -> u32 {
    32 + 16 + slot as u32
}
