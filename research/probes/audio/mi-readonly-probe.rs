//! AOUT-only inspection. PCM GetHandle was removed: it ALLOCATES on lookup misses.
//! No init, open-stream, write, route, volume, mute, or close-stream ioctl exists here.
use std::{fs::OpenOptions, io, os::fd::AsRawFd};
#[repr(C)]
struct Envelope {
    size: u16,
    pad: [u8; 6],
    pointer: u64,
}
#[repr(C)]
struct GetConnected {
    handle: u32,
    is_input: u32,
    module: u32,
    count: u32,
    array: u64,
}
extern "C" {
    fn ioctl(fd: i32, request: usize, ...) -> i32;
}
fn query<T>(file: &std::fs::File, request: u32, wire: &mut T) -> io::Result<i32> {
    let mut envelope = Envelope {
        size: std::mem::size_of::<T>() as u16,
        pad: [0; 6],
        pointer: wire as *mut T as usize as u64,
    };
    let result = unsafe { ioctl(file.as_raw_fd(), request as usize, &mut envelope) };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}
fn device(name: &str) -> io::Result<std::fs::File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(format!("/dev/mik/{name}"))
        .or_else(|_| {
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("/dev/mik!{name}"))
        })
}
fn main() -> io::Result<()> {
    assert_eq!(std::mem::size_of::<Envelope>(), 16);
    let cmdline = std::fs::read_to_string("/proc/cmdline")?;
    if !cmdline.contains("androidboot.serialno=4C0490290FA05A0C0") {
        return Err(io::Error::other("requires the original G06"));
    }
    let aout = device("aout")?;
    for path in [4u32, 1, 2, 3, 5, 6, 7, 8, 0] {
        let mut wire = [path, 0u32];
        match query(&aout, 0xc0081116, &mut wire) {
            Ok(status) => {
                println!("AOUT path={path} status={status} handle=0x{:08x}", wire[1]);
                if status == 0 && wire[1] != 0 {
                    let mut count = [wire[1], 1u32, 0u32, 0u32];
                    let status = query(&aout, 0xc010111e, &mut count);
                    println!("AOUT input count status={status:?} count={}", count[3]);
                    if matches!(status, Ok(0)) && count[3] > 0 && count[3] <= 32 {
                        let mut handles = vec![0u32; count[3] as usize];
                        let mut connections = GetConnected {
                            handle: wire[1],
                            is_input: 1,
                            module: 0,
                            count: count[3],
                            array: handles.as_mut_ptr() as usize as u64,
                        };
                        let status = query(&aout, 0xc018111f, &mut connections);
                        println!("AOUT input handles status={status:?} handles={handles:x?}");
                    }
                }
            }
            Err(error) => println!("AOUT path={path} errno={error}"),
        }
    }
    println!("PCM GetHandle omitted: lookup misses allocate unopened ALSA metadata");
    println!("MI_READONLY_COMPLETE");
    Ok(())
}
