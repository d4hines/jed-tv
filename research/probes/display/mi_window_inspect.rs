//! Read-only layer capacity/window inventory; no Utopia or framebuffer open.
use std::{ffi::c_void,fs::File,io,os::fd::AsRawFd};
extern "C"{fn ioctl(fd:i32,request:u32,arg:*mut c_void)->i32;}
#[repr(C)]struct Wire{bytes:u16,pad:[u8;6],pointer:u64}
#[repr(C)]struct Capability{handle:u32,kind:u32,result:u64}
fn call<T>(file:&File,request:u32,arg:&mut T)->i32{let mut wire=Wire{bytes:std::mem::size_of::<T>() as u16,pad:[0;6],pointer:arg as *mut T as usize as u64};unsafe{ioctl(file.as_raw_fd(),request,(&mut wire as *mut Wire).cast())}}
fn main()->io::Result<()>{
 if !std::fs::read_to_string("/proc/cmdline")?.contains("androidboot.serialno=4C0490290FA05A0C0"){return Err(io::Error::other("not G06"))}
 let file=File::open("/dev/mik/osd")?;
 for id in 0..5{
  let mut layer=[0u32,id];let r=call(&file,0xc0081456,&mut layer);println!("LAYER id={id} status={r} handle={:#x}",layer[0]);if r!=0{continue}
  let mut max_windows=0u8;let mut cap=Capability{handle:layer[0],kind:0,result:&mut max_windows as *mut u8 as usize as u64};let r=call(&file,0xc0101407,&mut cap);println!("WINDOW_CAPACITY id={id} status={r} capacity={max_windows}");
  let mut window=[layer[0],0];let r=call(&file,0xc0081457,&mut window);println!("FIRST_WINDOW id={id} status={r} handle={:#x}",window[1]);
  let mut enabled=[layer[0],0];let r=call(&file,0xc008145d,&mut enabled);println!("LAYER_ENABLED id={id} status={r} enabled={}",enabled[1]);
 }
 for mut kind in [0u32,1]{let r=call(&file,0xc004144f,&mut kind);println!("DUMP kind={kind} status={r}; existing debug level controls dmesg output");}
 println!("No debug level or display state changed");Ok(())
}
