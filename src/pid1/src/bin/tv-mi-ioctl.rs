use std::{fs::OpenOptions,io,os::fd::AsRawFd};
#[repr(C)] struct Wire { size:u16,pad:[u8;6],ptr:u64 }
fn number(s:&str)->u32 { u32::from_str_radix(s.trim_start_matches("0x"),16).expect("hex number") }
fn main()->io::Result<()> {
 let args:Vec<_>=std::env::args().collect();
 assert!(args.len()>=3,"usage: tv-mi-ioctl /dev/mik/name hex-request [hex-words]");
 assert!(std::fs::read_to_string("/proc/cmdline")?.contains("androidboot.serialno=4C0490290FA05A0C0"));
 let file=OpenOptions::new().read(true).write(true).open(&args[1])?;
 let request=number(&args[2]); let size=((request>>16)&0x3fff) as usize;
 let mut words:Vec<u32>=args[3..].iter().map(|s|number(s)).collect();
 assert_eq!(words.len()*4,size,"word count must match ioctl size");
 let mut wire=Wire{size:size as u16,pad:[0;6],ptr:words.as_mut_ptr() as u64};
 let r=unsafe{libc::ioctl(file.as_raw_fd(),request as _,&mut wire)};
 println!("result={r} words={words:x?}");
 if r<0{return Err(io::Error::last_os_error())} Ok(())
}
