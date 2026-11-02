//! Bounded direct speaker trial. No hardware initialization or gain changes.
#[path = "../../../src/pid1/audio/mi-pcm.rs"] mod mi_pcm;
#[path = "../../../src/pid1/audio/aout-route.rs"] mod aout_route;
use std::{io, time::{Duration, Instant}};

fn main() -> io::Result<()> {
    if std::env::args().nth(1).as_deref() != Some("--play-one-second") {
        return Err(io::Error::other("requires --play-one-second; plays one second at fixed 30 percent PCM amplitude"));
    }
    if !std::fs::read_to_string("/proc/cmdline")?.split_whitespace()
        .any(|s| s == "androidboot.serialno=4C0490290FA05A0C0") {
        return Err(io::Error::other("requires original G06"));
    }
    let mut pcm = mi_pcm::Pcm::open()?;
    let mut output = aout_route::Aout::existing_speaker()?;
    println!("DIRECT before={:?} PCM={:#x}", output.snapshot()?, pcm.handle());
    let route = output.route(aout_route::InputHandle::from_raw(pcm.handle())?, aout_route::RoutePolicy::RequireEmpty)?;
    pcm.start()?;
    let deadline = Instant::now() + Duration::from_secs(4);
    let notes = [523.25f32, 659.25, 783.99, 1046.50];
    for block in 0..100 {
        let mut bytes = Vec::with_capacity(1920);
        for n in block * 480..(block + 1) * 480 {
            let local = (n % 12000) as f32 / 48000.0;
            let edge = (local / 0.01).min((0.25 - local) / 0.025).clamp(0.0, 1.0);
            let sample = ((local * notes[n / 12000] * std::f32::consts::TAU).sin() * edge * 9830.0) as i16;
            bytes.extend_from_slice(&sample.to_le_bytes());
            bytes.extend_from_slice(&sample.to_le_bytes());
        }
        pcm.write_all_until(&bytes, deadline)?;
    }
    while pcm.buffer_info()?.queued_bytes != 0 {
        if Instant::now() >= deadline { return Err(io::Error::other("drain deadline")); }
        std::thread::sleep(Duration::from_millis(5));
    }
    route.restore()?;
    pcm.finish()?;
    println!("DIRECT_JINGLE_COMPLETE frames=48000 amplitude=9830 route_restored=true");
    Ok(())
}
