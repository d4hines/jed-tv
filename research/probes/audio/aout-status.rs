//! Existing speaker state only: no writes, PCM lookups, init, or routing.
#[path = "../../../src/pid1/audio/aout-route.rs"]
mod aout_route;
#[path = "../../../src/pid1/audio/mi-pcm.rs"]
mod mi_pcm;
use std::{io, os::fd::AsRawFd};
fn main() -> io::Result<()> {
    if !std::fs::read_to_string("/proc/cmdline")?
        .split_whitespace()
        .any(|s| s == "androidboot.serialno=4C0490290FA05A0C0")
    {
        return Err(io::Error::other("requires original G06"));
    }
    let aout = aout_route::Aout::existing_speaker()?;
    println!("AOUT existing_speaker={:#x}", aout.output().raw());
    println!("AOUT muted={:?}", aout.muted());
    println!("AOUT amplifier_muted={:?}", aout.amplifier_muted());
    println!("AOUT output_enabled={:?}", aout.output_enabled());
    println!("AOUT volume_index={:?}", aout.volume_index());
    println!("AOUT volume_db={:?}", aout.volume_db());
    let graph=aout.snapshot()?;
    println!("AOUT graph={graph:?}");
    #[repr(C)] struct GetAttr { handle:u32, attr:u32, input:u64, output:u64 }
    #[repr(C)] struct Envelope { size:u16, pad:[u8;6], pointer:u64 }
    for selector in 0..3u32 {
        let mut query=[0u8;12];
        query[..4].copy_from_slice(&aout.output().raw().to_le_bytes());
        query[4..8].copy_from_slice(&selector.to_le_bytes());
        query[8]=255;
        let file=std::fs::OpenOptions::new().read(true).write(true).open("/dev/mik/aout")?;
        let mut envelope=Envelope{size:12,pad:[0;6],pointer:query.as_mut_ptr() as usize as u64};
        let status=unsafe{libc_ioctl(file.as_raw_fd(),0xc00c110eu32 as usize,(&mut envelope as *mut Envelope).cast())};
        println!("AOUT mute query selector={selector} status={status} value={}",query[8]);
    }
    let pcm=std::fs::OpenOptions::new().read(true).write(true).open("/dev/mik/pcm")
        .or_else(|_|std::fs::OpenOptions::new().read(true).write(true).open("/dev/mik!pcm"))?;
    for input in graph.inputs {
        for _ in 0..3 {
            let mut info=mi_pcm::BufferInfo::default();
            let mut attr=GetAttr{handle:input.raw(),attr:0,input:0,output:(&mut info as *mut mi_pcm::BufferInfo) as usize as u64};
            let mut envelope=Envelope{size:24,pad:[0;6],pointer:(&mut attr as *mut GetAttr) as usize as u64};
            let result=unsafe{libc_ioctl(pcm.as_raw_fd(),0xc018340bu32 as usize,(&mut envelope as *mut Envelope).cast())};
            println!("PCM existing={:#x} status={result} buffer={info:?}",input.raw());
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    }
    Ok(())
}

unsafe extern "C" { #[link_name="ioctl"] fn libc_ioctl(fd:i32, request:usize, arg:*mut std::ffi::c_void)->i32; }
