//! tvbare: the platform API (display, keys, clock, network, audio) on QEMU
//! virt with no operating system. One core runs a single event loop; the
//! GIC only wakes it from WFI (timer deadline or device interrupt), after
//! which it polls every device. Optional extra cores help render.
//!
//! Boot arguments (QEMU -append, read from the DTB): seconds=10 fps=60
//! smp=1 idle=wfi|spin net=1 sound=1 range=0- path=/videoplayback?itag=140

#![no_std]
#![no_main]

extern crate alloc;

mod boot;
mod fdt;
mod gfx;
mod gic;
mod hal;
mod input;
mod net;
mod psci;
mod smp;
mod sound;
mod timer;
mod uart;

/// `src/platform/src/remote.rs`, unchanged (copied by build.rs).
#[allow(dead_code)]
mod remote {
    use alloc::vec::Vec;
    include!(concat!(env!("OUT_DIR"), "/remote.rs"));
}

use aarch64_cpu::registers::*;
use alloc::format;
use alloc::vec::Vec;
use core::ptr::NonNull;
use hal::HalImpl;
use virtio_drivers::device::gpu::VirtIOGpu;
use virtio_drivers::device::rng::VirtIORng;
use virtio_drivers::transport::mmio::{MmioTransport, VirtIOHeader};
use virtio_drivers::transport::{DeviceType, Transport};

const VIRTIO_BASE: usize = 0x0a00_0000;
const VIRTIO_SLOT: usize = 0x200;
const VIRTIO_SLOTS: usize = 32;
const DTB: usize = 0x4000_0000;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {}", info);
    psci::system_off();
}

fn ms(t: u64) -> f64 {
    timer::ticks_to_us(t) as f64 / 1000.0
}

#[derive(Default)]
struct Stat {
    n: u64,
    sum: u64,
    max: u64,
}
impl Stat {
    fn add(&mut self, t: u64) {
        self.n += 1;
        self.sum += t;
        self.max = self.max.max(t);
    }
    fn avg_ms(&self) -> f64 {
        if self.n == 0 { 0.0 } else { ms(self.sum) / self.n as f64 }
    }
}

#[unsafe(no_mangle)]
extern "C" fn rust_main() -> ! {
    let t_entry = timer::ticks();
    let info = fdt::parse(DTB);
    let args = info.as_ref().map_or("", |i| i.bootargs);
    let arg = |k: &str, d: &'static str| fdt::arg(args, k).unwrap_or(d);
    // quiet=1: no console output until the first frame (each UART byte is a
    // trapped MMIO write under HVF), to measure boot-to-pixel without logging.
    uart::QUIET.store(arg("quiet", "0") == "1", core::sync::atomic::Ordering::Relaxed);
    uart::init_logger(log::LevelFilter::Info);
    let sctlr = SCTLR_EL1.get();
    println!("tvbare: Rust entered {:.3} ms after reset; EL{} MIDR {:#x} CNTFRQ {} Hz",
             ms(t_entry), CurrentEL.read(CurrentEL::EL), MIDR_EL1.get(), timer::freq());
    println!("tvbare: SCTLR_EL1 {:#x}: MMU {} D-cache {} I-cache {}; TCR {:#x} MAIR {:#x}",
             sctlr, sctlr & 1, (sctlr >> 2) & 1, (sctlr >> 12) & 1, TCR_EL1.get(), MAIR_EL1.get());

    let seconds: f64 = arg("seconds", "10").parse().unwrap_or(10.0);
    let fps: u64 = arg("fps", "60").parse().unwrap_or(60);
    let want_cores: usize = arg("smp", "1").parse().unwrap_or(1);
    let spin = arg("idle", "wfi") == "spin";
    println!("tvbare: DTB {}; RAM {:x?}; bootargs \"{}\"",
             if info.is_some() { "found" } else { "missing" }, info.as_ref().and_then(|i| i.mem), args);
    let ram_end = info.as_ref().and_then(|i| i.mem).map_or(0x6000_0000, |(a, s)| (a + s) as usize);
    let (heap, heap_size) = hal::init_heap(ram_end);
    println!("tvbare: image {:#x}, heap {:#x} + {} MiB", hal::image_start(), heap, heap_size >> 20);

    // virtio-mmio: QEMU virt has 32 slots; empty ones report device ID 0.
    let (mut gpu_t, mut kbd_t, mut net_t, mut snd_t, mut rng_t) = (None, Vec::new(), None, None, None);
    for slot in 0..VIRTIO_SLOTS {
        let hdr = NonNull::new((VIRTIO_BASE + slot * VIRTIO_SLOT) as *mut VirtIOHeader).unwrap();
        let Ok(t) = (unsafe { MmioTransport::new(hdr, VIRTIO_SLOT) }) else { continue };
        println!("virtio: slot {:2} {:?} (mmio {:?})", slot, t.device_type(), t.version());
        match t.device_type() {
            DeviceType::GPU => gpu_t = Some((slot, t)),
            DeviceType::Input => kbd_t.push((slot, t)),
            DeviceType::Network => net_t = Some((slot, t)),
            DeviceType::Sound => snd_t = Some((slot, t)),
            DeviceType::EntropySource => rng_t = Some(t),
            _ => {}
        }
    }

    // Randomness (the platform's getrandom).
    if let Some(t) = rng_t {
        let mut rng = VirtIORng::<HalImpl, _>::new(t).expect("rng init");
        let mut seed = [0u8; 16];
        let n = rng.request_entropy(&mut seed).unwrap_or(0);
        println!("rng: {} bytes from virtio-rng: {:02x?}", n, &seed[..n]);
    }

    // Display.
    let (_, gpu_t) = gpu_t.expect("no virtio-gpu");
    let mut gpu = VirtIOGpu::<HalImpl, _>::new(gpu_t).expect("gpu init");
    let (w, h) = gpu.resolution().expect("gpu resolution");
    let fb = gpu.setup_framebuffer().expect("gpu framebuffer");
    let frame = gfx::Frame { ptr: fb.as_mut_ptr() as *mut u32, width: w as usize, height: h as usize,
                             clip: (0, h as usize) };
    smp::render(&frame, 0, 40, gfx::WHITE);
    gpu.flush().expect("gpu flush");
    let t_pixel = timer::ticks();
    uart::QUIET.store(false, core::sync::atomic::Ordering::Relaxed);
    println!("display: virtio-gpu {}x{} B8G8R8A8; first frame flushed {:.3} ms after reset \
              ({:.3} ms after Rust entry at {:.3} ms)", w, h, ms(t_pixel), ms(t_pixel - t_entry),
             ms(t_entry));

    // Interrupt controller, keys, sound, network.
    let gic_version = gic::init();
    gic::enable_local(0, gic::TIMER_PPI);
    println!("gic: {:?}; idle = {}", gic_version, if spin { "busy-wait" } else { "WFI" });
    let mut keys: Vec<input::Device> = Vec::new();
    for (slot, t) in kbd_t {
        let d = input::Device::new(format!("virtio-mmio@{:#x}", VIRTIO_BASE + slot * VIRTIO_SLOT), t)
            .expect("input init");
        println!("input: {} \"{}\" (INTID {})", d.path, d.name, gic::virtio_intid(slot));
        gic::enable_spi(gic::virtio_intid(slot), true);
        keys.push(d);
    }
    let mut tone = match (arg("sound", "1"), snd_t) {
        ("1", Some((slot, t))) => {
            gic::enable_spi(gic::virtio_intid(slot), true);
            sound::Tone::new(t, 3.0)
        }
        _ => None,
    };
    let mut net = match (arg("net", "1"), net_t) {
        ("1", Some((slot, t))) => {
            gic::enable_spi(gic::virtio_intid(slot), true);
            Some(net::Net::new(t, arg("path", "/videoplayback?itag=140"), arg("range", "0-")))
        }
        _ => None,
    };
    let cores = smp::start(want_cores);
    if want_cores > 1 {
        println!("smp: {} cores rendering", cores);
    }

    // Event loop: render at each frame deadline; poll devices on every wake.
    let period = timer::freq() / fps;
    let t0 = timer::ticks();
    let end = t0 + (seconds * timer::freq() as f64) as u64;
    let (mut next, mut frames, mut late) = (t0, 0u64, 0u64);
    let (mut sec_start, mut sec_frames) = (t0, 0u64);
    let (mut draw, mut flush, mut interval) = (Stat::default(), Stat::default(), Stat::default());
    let mut min_interval = u64::MAX;
    let (mut last_frame, mut wakes, mut irq_timer, mut irq_dev) = (0u64, 0u64, 0u64, 0u64);
    let (mut sy, mut color) = (40usize, gfx::WHITE);
    let mut key_events = 0;
    loop {
        let now = timer::ticks();
        if now >= end {
            break;
        }
        if now >= next {
            let t = (now - t0) as f64 / timer::freq() as f64;
            let sx = ((t * 400.0) as usize) % (w as usize - 80);
            let a = timer::ticks();
            smp::render(&frame, sx, sy, color);
            let b = timer::ticks();
            gpu.flush().expect("gpu flush");
            let c = timer::ticks();
            draw.add(b - a);
            flush.add(c - b);
            if last_frame != 0 {
                interval.add(a - last_frame);
                min_interval = min_interval.min(a - last_frame);
            }
            last_frame = a;
            frames += 1;
            sec_frames += 1;
            next += period;
            if next <= c {
                late += 1;
                next = c + period - (c - t0) % period; // skip missed deadlines
            }
            if c - sec_start >= timer::freq() {
                println!("frame: {} fps; draw {:.2} ms, flush {:.2} ms (avg); wakes {} (timer {}, device {})",
                         sec_frames * timer::freq() / (c - sec_start), draw.avg_ms(), flush.avg_ms(),
                         wakes, irq_timer, irq_dev);
                if let Some(n) = net.as_mut().filter(|n| !n.done) {
                    println!("net: {}", n.status());
                }
                sec_start = c;
                sec_frames = 0;
            }
        }
        for d in keys.iter_mut() {
            for k in d.poll_keys().unwrap_or_default() {
                key_events += 1;
                let action = remote::action(k.code);
                println!("input: key code {} value {} -> {:?}", k.code, k.value, action);
                if k.value == 0 {
                    continue;
                }
                match action {
                    Some(remote::Action::Up) => sy = sy.saturating_sub(20),
                    Some(remote::Action::Down) => sy = (sy + 20).min(h as usize / 3 - 80),
                    Some(remote::Action::Ok) => {
                        color = if color == gfx::WHITE { gfx::BARS[5] } else { gfx::WHITE }
                    }
                    _ => {}
                }
            }
        }
        if let Some(n) = net.as_mut() {
            n.ack_interrupt();
            n.poll();
        }
        if let Some(s) = tone.as_mut() {
            s.ack_interrupt();
            s.pump();
        }
        // Sleep until the next frame deadline or a device interrupt.
        if spin {
            core::hint::spin_loop(); // busy polling: loop straight back
        } else {
            timer::arm(next);
            aarch64_cpu::asm::wfi();
            wakes += 1;
            while let Some(id) = gic::ack() {
                if id == gic::TIMER_PPI {
                    irq_timer += 1;
                    timer::disarm(); // level-triggered: drop the line before the next WFI
                } else {
                    irq_dev += 1;
                }
            }
            timer::disarm();
        }
    }
    let el = timer::ticks() - t0;
    println!("frame: {} frames in {:.2} s = {:.1} fps (target {}); draw avg {:.2} max {:.2} ms; \
              flush avg {:.2} max {:.2} ms; interval min {:.2} avg {:.2} max {:.2} ms; late {}",
             frames, ms(el) / 1000.0, frames as f64 * 1000.0 / ms(el), fps, draw.avg_ms(), ms(draw.max),
             flush.avg_ms(), ms(flush.max), ms(if min_interval == u64::MAX { 0 } else { min_interval }),
             interval.avg_ms(), ms(interval.max), late);
    println!("idle: {} WFI wakes ({} timer, {} device IRQs); {} key events; heap used {} KiB",
             wakes, irq_timer, irq_dev, key_events, hal::heap_used() / 1024);
    if let Some(n) = &net {
        if !n.done {
            println!("net: transfer not finished");
        }
    }
    if let Some(s) = &tone {
        if !s.done {
            println!("sound: not finished ({} periods played)", s.played);
        }
    }
    println!("tvbare: done, powering off");
    psci::system_off();
}
