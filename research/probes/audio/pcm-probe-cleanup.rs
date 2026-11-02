//! One-off cleanup of the three ALSA metadata slots created by our old probe.
//! Kernel MI_PCM_Close sees bit0x100 and releases metadata without MApi close.
#[path = "../../../src/pid1/audio/mi-pcm.rs"]
mod mi_pcm;
use mi_pcm::{DeviceTransport, Transport};
use std::{fs::OpenOptions, io};
fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args != ["--release-three-probe-created-alsa-slots"] {
        return Err(io::Error::other("requires --release-three-probe-created-alsa-slots; confirm owner0 metadata slots first"));
    }
    if !std::fs::read_to_string("/proc/cmdline")?.contains("androidboot.serialno=4C0490290FA05A0C0")
    {
        return Err(io::Error::other("requires original G06"));
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/mik/pcm")
        .or_else(|_| {
            OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/mik!pcm")
        })?;
    let transport = DeviceTransport::from_file(file);
    for handle in [0x81000100u32, 0x81000101, 0x81000102] {
        assert_eq!(handle & 0xffffff00, 0x81000100);
        let mut wire = handle.to_ne_bytes();
        transport.transact(0xc0043403, &mut wire)?;
        println!("RELEASED_PROBE_ALSA_METADATA handle={handle:#010x}");
    }
    println!(
        "PROBE_METADATA_CLEANUP_COMPLETE; no init, open, start, write, or hardware-close calls"
    );
    Ok(())
}
