//! Bounded allocation/map/readback probe. No OSD, framebuffer or MI init calls.
#[path = "../../../src/pid1/display/osd_abi.rs"]
mod osd_abi;
#[path = "../../../src/pid1/display/mma.rs"]
mod mma;
fn main() -> std::io::Result<()> {
    if std::env::args().nth(1).as_deref() != Some("--allocate-unpublished") {
        return Err(std::io::Error::other("usage: mma-probe --allocate-unpublished"));
    }
    if !std::fs::read_to_string("/proc/cmdline")?.split_whitespace()
        .any(|item| item == "androidboot.serialno=4C0490290FA05A0C0") {
        return Err(std::io::Error::other("expected original G06"));
    }
    let mut buffer = mma::UnpublishedBuffer::allocate()?;
    buffer.check_pages()?;
    println!("PASS: private DMA-BUF fd={} IOVA={:#x}, two pages mapped/read back; no OSD consumer", buffer.descriptor(), buffer.address());
    drop(buffer);
    println!("Unmapped and closed allocation descriptor");
    Ok(())
}
