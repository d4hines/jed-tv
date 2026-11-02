use std::{fs::OpenOptions, io, os::fd::AsRawFd};
#[repr(C)]
struct Wire { size: u16, pad: [u8; 6], ptr: u64 }
fn call(file: &std::fs::File, request: u32, words: &mut [u32]) -> io::Result<()> {
    let mut wire = Wire { size: ((request >> 16) & 0x3fff) as u16, pad: [0;6], ptr: words.as_mut_ptr() as u64 };
    let result = unsafe { libc::ioctl(file.as_raw_fd(), request as _, &mut wire) };
    println!("ioctl {request:#x}: result={result}, words={words:x?}");
    if result < 0 { return Err(io::Error::last_os_error()); }
    if result != 0 { return Err(io::Error::other(format!("vendor error {result}"))); }
    Ok(())
}
fn main() -> io::Result<()> {
    let cmdline = std::fs::read_to_string("/proc/cmdline")?;
    assert!(cmdline.contains("androidboot.serialno=4C0490290FA05A0C0"));
    let disp = OpenOptions::new().read(true).write(true).open("/dev/mik/disp")?;
    let mut controller = [0, 0];
    call(&disp, 0xc0081206, &mut controller)?;
    let osd = OpenOptions::new().read(true).write(true).open("/dev/mik/osd")?;
    let result = unsafe { libc::ioctl(osd.as_raw_fd(), 0x144eu32 as _, std::ptr::null_mut::<u8>()) };
    println!("OSD DisableBootLogo: {result}");
    if result != 0 { return Err(io::Error::other(format!("OSD error {result}"))); }
    call(&disp, 0xc0041203, &mut controller[..1])?;
    Ok(())
}
