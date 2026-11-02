//! ARM generic timer: the virtual counter (CNTVCT_EL0) is the clock, and the
//! virtual timer (CNTV_*, PPI 27) wakes the core from WFI at frame deadlines.
//! The virtual timer works under both TCG and HVF (HVF passes it through).

use aarch64_cpu::asm::barrier;
use aarch64_cpu::registers::*;

pub fn freq() -> u64 {
    CNTFRQ_EL0.get()
}

pub fn ticks() -> u64 {
    barrier::isb(barrier::SY);
    CNTVCT_EL0.get()
}

pub fn micros() -> u64 {
    let f = freq();
    if f == 0 {
        return 0;
    }
    (ticks() as u128 * 1_000_000 / f as u128) as u64
}

pub fn ticks_to_us(t: u64) -> u64 {
    (t as u128 * 1_000_000 / freq() as u128) as u64
}

pub fn us_to_ticks(us: u64) -> u64 {
    (us as u128 * freq() as u128 / 1_000_000) as u64
}

/// Arm the virtual timer to fire (level IRQ, PPI 27) at counter value `at`.
pub fn arm(at: u64) {
    CNTV_CVAL_EL0.set(at);
    CNTV_CTL_EL0.write(CNTV_CTL_EL0::ENABLE::SET + CNTV_CTL_EL0::IMASK::CLEAR);
    barrier::isb(barrier::SY);
}

/// Silence the virtual timer (it stays asserted until CVAL moves or it's masked).
pub fn disarm() {
    CNTV_CTL_EL0.write(CNTV_CTL_EL0::ENABLE::CLEAR + CNTV_CTL_EL0::IMASK::SET);
    barrier::isb(barrier::SY);
}
