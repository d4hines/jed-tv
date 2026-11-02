//! Read-only V109 MI status probe; candidate types for a PID-1-owned bootstrap.
//! No init, ready publication, module loading, or global state mutation.
use std::{
    fs::OpenOptions,
    io, mem,
    os::{
        fd::AsRawFd,
        raw::{c_int, c_ulong},
    },
};

#[repr(C)]
struct Envelope {
    bytes: u16,
    reserved: [u8; 6],
    argument: u64,
}
#[repr(C)]
struct Attribute {
    kind: u64,
    argument: u64,
}
#[repr(C)]
struct Status {
    selector: u32,
    ready: u8,
    reserved: [u8; 3],
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

fn main() -> io::Result<()> {
    assert_eq!(mem::size_of::<Envelope>(), 16);
    assert_eq!(mem::size_of::<Attribute>(), 16);
    assert_eq!(mem::size_of::<Status>(), 8);
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 || args[1] == "--layout" {
        println!("envelope=16 attribute=16 status=8; no device opened");
        return Ok(());
    }
    if args.len() != 2 || args[1] != "--status" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: mi-status [--layout|--status]",
        ));
    }
    if !std::fs::read_to_string("/proc/cmdline")?.contains("androidboot.serialno=4C0490290FA05A0C0")
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "expected original G06 serial in kernel command line",
        ));
    }
    let file = OpenOptions::new()
        .read(true)
        .open("/dev/mik/sys")
        .or_else(|_| OpenOptions::new().read(true).open("/dev/mik!sys"))?;
    let mut failed = false;
    for (selector, name) in [
        (40, "mik"),
        (1, "userspace"),
        (3, "flash"),
        (6, "disp"),
        (9, "osd"),
        (10, "tuner"),
        (11, "dmx"),
        (17, "audio"),
        (23, "dsc"),
    ] {
        let mut status = Status {
            selector,
            ready: 0,
            reserved: [0; 3],
        };
        let mut attr = Attribute {
            kind: 5,
            argument: (&mut status as *mut Status) as u64,
        };
        let mut envelope = Envelope {
            bytes: 16,
            reserved: [0; 6],
            argument: (&mut attr as *mut Attribute) as u64,
        };
        // The kernel copies the nested buffers synchronously before ioctl returns.
        let result = unsafe { ioctl(file.as_raw_fd(), 0xc0101805, &mut envelope as *mut Envelope) };
        if result < 0 {
            failed = true;
            eprintln!(
                "selector={selector} name={name} syscall={result} error={}",
                io::Error::last_os_error()
            );
        } else {
            println!(
                "selector={selector} name={name} vendor_result={result} ready={}",
                status.ready
            );
            failed |= result != 0;
        }
    }
    if failed {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "one or more queries failed",
        ))
    } else {
        Ok(())
    }
}
