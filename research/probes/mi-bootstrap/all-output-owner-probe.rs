//! Coordinator-only diagnostic for the stock HAL's first (ALL, path 0) output.
//! No INIT, PCM, routing, gain, amplifier or explicit Close request is issued.
//! Open can change global DSP speaker configuration: quiet preconditions apply.
use std::{fs::{self, File, OpenOptions}, io::{self, Write}, os::fd::AsRawFd, time::Duration};

#[repr(C)]
struct Envelope { size: u16, pad: [u8; 6], address: u64 }
extern "C" { fn ioctl(fd:i32,request:usize,...)->i32; fn geteuid()->u32; fn signal(sig:i32,handler:usize)->usize; }
extern "C" fn retain(_:i32) {}
fn word(bytes:&[u8],n:usize)->u32 {u32::from_le_bytes(bytes[n..n+4].try_into().unwrap())}
fn call(file:&File,request:u32,bytes:&mut[u8])->io::Result<i32> {
    let mut envelope=Envelope{size:bytes.len().try_into().map_err(io::Error::other)?,pad:[0;6],address:bytes.as_mut_ptr() as usize as u64};
    let result=unsafe{ioctl(file.as_raw_fd(),request as usize,&mut envelope as *mut Envelope)};
    if result<0 {Err(io::Error::last_os_error())} else {Ok(result)}
}
fn lookup(file:&File,path:u32)->io::Result<(i32,u32)> {
    let mut bytes=[0;8]; bytes[..4].copy_from_slice(&path.to_le_bytes());
    let status=call(file,0xc0081116,&mut bytes)?; Ok((status,word(&bytes,4)))
}
fn count(file:&File,handle:u32,direction:u32)->io::Result<u32> {
    let mut bytes=[0;16]; bytes[..4].copy_from_slice(&handle.to_le_bytes());bytes[4..8].copy_from_slice(&direction.to_le_bytes());
    let status=call(file,0xc010111e,&mut bytes)?;
    if status!=0 {return Err(io::Error::other(format!("count status {status}")));}
    Ok(word(&bytes,12))
}
fn zero_gain()->io::Result<()> {
    let gain: f32=fs::read_to_string("/tmp/app-volume")?.trim().parse().map_err(io::Error::other)?;
    if gain!=0.0 {return Err(io::Error::other("global PCM gain must be exactly zero"));}Ok(())
}
fn run()->io::Result<()> {
    if unsafe{geteuid()}!=0 || !fs::read_to_string("/proc/cmdline")?.split_whitespace().any(|s|s==mi_bootstrap::SERIAL){
        return Err(io::Error::new(io::ErrorKind::PermissionDenied,"requires root on original G06"));
    }
    zero_gain()?;
    let file=OpenOptions::new().read(true).write(true).open("/dev/mik/aout").or_else(|_|OpenOptions::new().read(true).write(true).open("/dev/mik!aout"))?;
    let (status,speaker)=lookup(&file,4)?;
    if status!=0 || speaker>>24!=0x17 {return Err(io::Error::other("expected existing speaker path4"));}
    if count(&file,speaker,1)?!=0 {return Err(io::Error::other("speaker still has an active input; stop the app first"));}
    let (status,all)=lookup(&file,0)?;
    if status!=7 || all!=0 {return Err(io::Error::other(format!("ALL path must be absent before test; status={status},handle={all:#x}")));}
    zero_gain()?;
    // Retain the descriptor after Open, even if validation fails. A path0 Close
    // resets _bIsSettingAllPathType; this probe must not undo global setup.
    unsafe {
        if signal(15,retain as *const () as usize)==usize::MAX || signal(2,retain as *const () as usize)==usize::MAX {
            return Err(io::Error::last_os_error());
        }
    }
    let mut open=[0u8;88]; // handle0, empty name, path0, default table, null pointer
    let result=call(&file,0xc0581103,&mut open);
    println!("ALL_OUTPUT_OPEN status={result:?} handle={:#010x}; retaining descriptor",word(&open,0));
    println!("ALL_OUTPUT_READBACK {:?}",lookup(&file,0));
    println!("SPEAKER_READBACK {:?}; inputs={:?}",lookup(&file,4),count(&file,speaker,1));
    let _=io::stdout().flush();
    loop { std::thread::sleep(Duration::from_secs(1)); std::hint::black_box(&file); }
}
fn main() {if let Err(error)=run(){eprintln!("all-output-owner-probe: {error}");std::process::exit(1)}}
