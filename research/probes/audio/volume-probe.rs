//! Brief, scoped speaker attenuation test on the original G06.
#[path = "../../../src/pid1/audio/aout-route.rs"]
mod aout_route;
#[path = "../../../src/pid1/audio/mi-pcm.rs"]
mod mi_pcm;
use std::{io, time::Duration};

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let db: i32 = args.next().and_then(|s| s.parse().ok())
        .filter(|v| (-15..=0).contains(v))
        .ok_or_else(|| io::Error::other("expected attenuation -15..0 dB"))?;
    let seconds: u64 = args.next().and_then(|s| s.parse().ok())
        .filter(|v| (1..=120).contains(v))
        .ok_or_else(|| io::Error::other("expected duration 1..120 seconds"))?;
    if args.next().is_some() || !std::fs::read_to_string("/proc/cmdline")?
        .split_whitespace().any(|s| s == "androidboot.serialno=4C0490290FA05A0C0") {
        return Err(io::Error::other("requires original G06 and two arguments"));
    }
    let aout = aout_route::Aout::existing_speaker()?;
    if aout.snapshot()?.inputs.len() != 1 {
        return Err(io::Error::other("expected one active speaker stream"));
    }
    let volume = aout.volume_owned(aout_route::VolumeDb { integer: db, thousandths: 0 })?;
    println!("speaker {:?} -> {:?}, restoring in {seconds}s", volume.original(), volume.current()?);
    std::thread::sleep(Duration::from_secs(seconds));
    volume.restore()
}
