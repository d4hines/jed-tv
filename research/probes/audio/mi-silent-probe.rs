//! Coordinator-only bounded silent PCM ownership test; never routes or sets gain.
#[path = "../../../src/pid1/audio/mi-pcm.rs"]
mod mi_pcm;
use std::{
    io, thread,
    time::{Duration, Instant},
};

fn run() -> io::Result<()> {
    if std::env::args().nth(1).as_deref() != Some("--exclusive-silent-trial") {
        return Err(io::Error::other(
            "requires --exclusive-silent-trial after quiescing other audio owners",
        ));
    }
    let cmdline = std::fs::read_to_string("/proc/cmdline")?;
    if !cmdline.contains("androidboot.serialno=4C0490290FA05A0C0") {
        return Err(io::Error::other("requires original G06"));
    }
    let mut pcm = mi_pcm::Pcm::open()?;
    println!(
        "MI_OPEN handle={:#x} buffer={:?}",
        pcm.handle(),
        pcm.buffer_info()?
    );
    pcm.start()?;
    println!("MI_START buffer={:?}", pcm.buffer_info()?);
    let silence = [0u8; 1920]; // 10 ms at 48kHz stereo; exactly 100 ms total.
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut total = 0usize;
    for _ in 0..10 {
        let mut done = 0;
        while done < silence.len() {
            if Instant::now() >= deadline {
                return Err(io::Error::other("bounded write deadline"));
            }
            let n = pcm.write(&silence[done..])?;
            done += n;
            total += n;
            println!(
                "MI_WRITE accepted={n} total={total} buffer={:?}",
                pcm.buffer_info()?
            );
            if n == 0 {
                thread::sleep(Duration::from_millis(5))
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    loop {
        let info = pcm.buffer_info()?;
        println!("MI_DRAIN buffer={info:?}");
        if info.queued_bytes == 0 {
            break;
        }
        if Instant::now() >= deadline {
            return Err(io::Error::other("bounded drain deadline"));
        }
        thread::sleep(Duration::from_millis(10));
    }
    pcm.finish()?;
    println!("MI_SILENT_COMPLETE bytes={total}; no AOUT route, init, mute, or volume calls");
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("MI_SILENT_FAILED: {error}");
        std::process::exit(1)
    }
}
