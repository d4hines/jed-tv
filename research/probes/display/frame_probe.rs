//! Read-only TVF2 transport probe; never opens display hardware.
#[path = "../../../src/pid1/display/frames.rs"]
mod frames;
use std::{io, time::Instant};

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args.next().ok_or_else(|| io::Error::other("usage: frame-probe FRAMEFILE"))?;
    if args.next().is_some() {
        return Err(io::Error::other("unexpected argument"));
    }
    let frame = frames::Frames::open(&path)?;
    let start = Instant::now();
    let mut pixels = Vec::new();
    let sequence = frame.copy_if_new(0, &mut pixels)?;
    let checksum = pixels.iter().fold(0xcbf29ce484222325u64, |hash, pixel| {
        pixel.to_le_bytes().iter().fold(hash, |h, byte| (h ^ u64::from(*byte)).wrapping_mul(0x100000001b3))
    });
    println!("FRAME_READONLY width={} height={} sequence={sequence:?} pixels={} checksum={checksum:016x} elapsed_us={}",
        frame.width, frame.height, pixels.len(), start.elapsed().as_micros());
    if sequence.is_none() {
        return Err(io::Error::other("no frame published"));
    }
    Ok(())
}
