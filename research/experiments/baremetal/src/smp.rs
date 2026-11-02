//! SMP: PSCI CPU_ON brings up cores 1..n, which share the identity map and
//! render horizontal bands of each frame. Coordination is a generation
//! counter, per-core done flags and a GIC SGI doorbell; idle cores sleep in
//! WFI. No scheduler, no locks.

use crate::gfx::Frame;
use crate::{gic, println, psci, timer};
use core::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering::*};

pub const MAX_CORES: usize = 4;

struct Job {
    generation: AtomicU64,
    cores: AtomicUsize,
    ptr: AtomicUsize,
    width: AtomicUsize,
    height: AtomicUsize,
    sx: AtomicUsize,
    sy: AtomicUsize,
    color: AtomicU32,
    done: [AtomicU64; MAX_CORES],
}

static JOB: Job = Job {
    generation: AtomicU64::new(0),
    cores: AtomicUsize::new(1),
    ptr: AtomicUsize::new(0),
    width: AtomicUsize::new(0),
    height: AtomicUsize::new(0),
    sx: AtomicUsize::new(0),
    sy: AtomicUsize::new(0),
    color: AtomicU32::new(0),
    done: [const { AtomicU64::new(0) }; MAX_CORES],
};
static ONLINE: AtomicUsize = AtomicUsize::new(1);

unsafe extern "C" {
    fn _secondary_entry();
}

/// Start cores 1..n; returns how many cores (including 0) are online.
pub fn start(n: usize) -> usize {
    for core in 1..n.min(MAX_CORES) {
        let t0 = timer::ticks();
        let r = psci::cpu_on(core as u64, _secondary_entry as usize as u64, core as u64);
        while ONLINE.load(Acquire) <= core && timer::ticks() - t0 < timer::us_to_ticks(500_000) {}
        let us = timer::ticks_to_us(timer::ticks() - t0);
        println!("smp: PSCI CPU_ON core {} -> {} ({} us to Rust)", core, r, us);
    }
    let online = ONLINE.load(Acquire);
    JOB.cores.store(online, Release);
    online
}

fn band(core: usize) {
    let cores = JOB.cores.load(Relaxed);
    let h = JOB.height.load(Relaxed);
    let f = Frame {
        ptr: JOB.ptr.load(Relaxed) as *mut u32,
        width: JOB.width.load(Relaxed),
        height: h,
        clip: (0, 0),
    };
    f.band(core * h / cores, (core + 1) * h / cores)
        .test_pattern(JOB.sx.load(Relaxed), JOB.sy.load(Relaxed), JOB.color.load(Relaxed));
}

#[unsafe(no_mangle)]
extern "C" fn rust_secondary(core: usize) -> ! {
    gic::init_cpu(core);
    ONLINE.fetch_add(1, AcqRel);
    let mut seen = 0;
    loop {
        let g = JOB.generation.load(Acquire);
        if g == seen {
            // Sleep until core 0's SGI. A doorbell rung after the load above
            // stays pending, so WFI returns at once and nothing is lost.
            // (WFE/SEV was tried first: HVF traps WFE and returns at once,
            // so three idle cores burnt 290% host CPU.)
            aarch64_cpu::asm::wfi();
            while gic::ack().is_some() {}
            continue;
        }
        seen = g;
        band(core);
        JOB.done[core].store(g, Release);
    }
}

/// Draw the test pattern with all online cores; returns when every band is done.
pub fn render(f: &Frame, sx: usize, sy: usize, color: u32) {
    let cores = JOB.cores.load(Relaxed);
    if cores == 1 {
        let mut f = f.band(0, f.height);
        return f.test_pattern(sx, sy, color);
    }
    JOB.ptr.store(f.ptr as usize, Relaxed);
    JOB.width.store(f.width, Relaxed);
    JOB.height.store(f.height, Relaxed);
    JOB.sx.store(sx, Relaxed);
    JOB.sy.store(sy, Relaxed);
    JOB.color.store(color, Relaxed);
    let g = JOB.generation.fetch_add(1, AcqRel) + 1;
    gic::sgi_others(gic::WAKE_SGI);
    band(0);
    for c in 1..cores {
        while JOB.done[c].load(Acquire) != g {
            core::hint::spin_loop();
        }
    }
}
