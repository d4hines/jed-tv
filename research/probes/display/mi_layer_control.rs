//! Explicit single-window layer0 visibility. No allocation, mode or layer1 access.
use std::{ffi::c_void,fs::File,io,os::fd::AsRawFd};
extern "C"{fn ioctl(fd:i32,request:u32,arg:*mut c_void)->i32;}
#[repr(C)]struct Wire{bytes:u16,pad:[u8;6],pointer:u64}
#[repr(C)]struct Capability{handle:u32,kind:u32,result:u64}
fn call<T>(file:&File,request:u32,arg:&mut T)->io::Result<()>{let mut wire=Wire{bytes:std::mem::size_of::<T>() as u16,pad:[0;6],pointer:arg as *mut T as usize as u64};let r=unsafe{ioctl(file.as_raw_fd(),request,(&mut wire as *mut Wire).cast())};if r==0{Ok(())}else{Err(io::Error::other(format!("MI {request:#x} returned{r}")))}}
fn main()->io::Result<()>{
 let mode=std::env::args().nth(1).unwrap_or_default();
 if mode!="--show-layer0"&&mode!="--hide-layer0"{return Err(io::Error::other("explicit --show-layer0 or --hide-layer0 required"))}
 if !std::fs::read_to_string("/proc/cmdline")?.contains("androidboot.serialno=4C0490290FA05A0C0"){return Err(io::Error::other("not G06"))}
 let file=File::open("/dev/mik/osd").or_else(|_| File::open("/dev/mik!osd"))?;let mut layer=[0u32,0];call(&file,0xc0081456,&mut layer)?;
 if layer[0]!=0x20000{return Err(io::Error::other("unexpected layer0 handle"))}
 let mut capacity=0u8;let mut cap=Capability{handle:layer[0],kind:0,result:&mut capacity as *mut u8 as usize as u64};call(&file,0xc0101407,&mut cap)?;
 let mut window=[layer[0],0];call(&file,0xc0081457,&mut window)?;
 if capacity!=1||window[1]!=0x40000{return Err(io::Error::other("layer0 is not the expected single-window compositor"))}
 let mut show=layer[0];call(&file,if mode=="--show-layer0"{0xc0041405}else{0xc0041406},&mut show)?;
 let mut enabled=[layer[0],0];call(&file,0xc008145d,&mut enabled)?;
 println!("LAYER0 visibility handle={:#x} window={:#x} enabled={}",layer[0],window[1],enabled[1]);Ok(())
}
