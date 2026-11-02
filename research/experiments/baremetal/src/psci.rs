//! PSCI over HVC (QEMU virt's DTB: method = "hvc"; QEMU implements it for
//! both TCG and HVF when there is no EL2/EL3 firmware).

use core::arch::asm;

const SYSTEM_OFF: u64 = 0x8400_0008;
const CPU_ON_64: u64 = 0xC400_0003;

unsafe fn hvc(f: u64, a: u64, b: u64, c: u64) -> i64 {
    let r: i64;
    unsafe {
        asm!("hvc #0", inlateout("x0") f as i64 => r, in("x1") a, in("x2") b, in("x3") c,
             options(nostack));
    }
    r
}

pub fn system_off() -> ! {
    unsafe { hvc(SYSTEM_OFF, 0, 0, 0) };
    loop {
        aarch64_cpu::asm::wfe();
    }
}

/// Start core `mpidr` at physical address `entry` with x0 = `context`.
pub fn cpu_on(mpidr: u64, entry: u64, context: u64) -> i64 {
    unsafe { hvc(CPU_ON_64, mpidr, entry, context) }
}
