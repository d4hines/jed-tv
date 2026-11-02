//! TVF2 direct display: bounded --run or persistent --serve; owner belongs in PID1.
#[path = "../../../src/pid1/display/direct_mi.rs"]
mod direct_mi;
#[path = "../../../src/pid1/display/osd_scanout.rs"]
mod osd_scanout;
#[path = "../../../src/pid1/display/osd_abi.rs"]
mod osd_abi;
#[path = "../../../src/pid1/display/mma.rs"]
mod mma;
#[path = "../../../src/pid1/display/frames.rs"]
mod frames;
#[path = "../../../src/pid1/display/overlay.rs"]
mod overlay;
#[path = "../../../src/pid1/display/pixels.rs"]
mod pixels;
#[path = "../../../src/pid1/display/service.rs"]
mod service;
use std::{
    io,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
static RUN: AtomicBool = AtomicBool::new(true);
static PAUSED: AtomicBool = AtomicBool::new(false);
extern "C" fn pause(_: i32) {
    PAUSED.store(true, Ordering::Relaxed);
}
extern "C" fn resume(_: i32) {
    PAUSED.store(false, Ordering::Relaxed);
}
extern "C" fn stop(_: i32) {
    RUN.store(false, Ordering::Relaxed);
}
extern "C" {
    fn signal(number: i32, handler: extern "C" fn(i32)) -> usize;
}
fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_default();
    if mode != "--run" && mode != "--serve" {
        return Err(io::Error::other(
            "usage: --run FRAMEFILE SECONDS (1..120) or --serve FRAMEFILE",
        ));
    }
    let path = args
        .next()
        .ok_or_else(|| io::Error::other("missing TVF2 path"))?;
    let seconds = if mode == "--run" {
        Some(
            args.next()
                .and_then(|v| v.parse::<u64>().ok())
                .filter(|&v| v > 0 && v <= 120)
                .ok_or_else(|| io::Error::other("seconds must be 1..120"))?,
        )
    } else {
        None
    };
    if args.next().is_some() {
        return Err(io::Error::other("unexpected extra arguments"));
    }
    if !std::fs::read_to_string("/proc/cmdline")?.contains("androidboot.serialno=4C0490290FA05A0C0")
    {
        return Err(io::Error::other("not the authorized G06"));
    }
    unsafe {
        signal(15, stop);
        signal(2, stop);
        signal(10, pause);
        signal(12, resume);
    }
    let mut display = service::Service::open(&path)?;
    let until = seconds.map(|s| Instant::now() + Duration::from_secs(s));
    let mut count = 0;
    let mut was_paused = false;
    println!("DIRECT control SIGUSR1=hide-retain SIGUSR2=resume SIGTERM=exit-retain-if-window-live");
    while RUN.load(Ordering::Relaxed) && until.is_none_or(|end| Instant::now() < end) {
        let paused = PAUSED.load(Ordering::Relaxed);
        if let Some(sequence) = display.tick(paused)? {
            count += 1;
            if count == 1 || count % 350 == 0 {
                println!("DIRECT stream frames={count} sequence={sequence}");
            }
        } else {
            std::thread::sleep(Duration::from_millis(if paused { 20 } else { 3 }));
        }
        if paused && !was_paused { println!("DIRECT paused; resources retained"); }
        was_paused = paused;
    }
    drop(display);
    println!("DIRECT stream stopped frames={count}");
    Ok(())
}
