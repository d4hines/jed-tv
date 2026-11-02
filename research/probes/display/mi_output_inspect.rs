//! Read-only display output timing and individual valid mute reasons.
use std::{ffi::c_void,fs::{self,File},io,os::fd::AsRawFd};
extern "C"{fn ioctl(fd:i32,request:u32,arg:*mut c_void)->i32;}
#[repr(C)]struct Wire{bytes:u16,pad:[u8;6],pointer:u64}
fn call<const N:usize>(file:&File,request:u32,arg:&mut[u32;N])->i32{let mut wire=Wire{bytes:(N*4) as u16,pad:[0;6],pointer:arg.as_mut_ptr() as usize as u64};unsafe{ioctl(file.as_raw_fd(),request,(&mut wire as *mut Wire).cast())}}
fn main()->io::Result<()>{
 if !fs::read_to_string("/proc/cmdline")?.contains("androidboot.serialno=4C0490290FA05A0C0"){return Err(io::Error::other("not G06"))}
 let file=File::open("/dev/mik/disp")?;let mut controller=[0u32,0];let r=call(&file,0xc0081206,&mut controller);println!("CONTROLLER status={r} words={controller:x?}");if r!=0||controller[0]!=0x36004354{return Err(io::Error::other("unexpected controller"))}
 let mut timing=[controller[0],u32::MAX];let r=call(&file,0xc008122d,&mut timing);println!("OUTPUT_TIMING status={r} value={} hex={:#x}",timing[1],timing[1]);
 for index in 0u32..16 {
  let handle=0x36000000|index;
  let mut probe=[handle,1,0xcccccccc];let r=call(&file,0xc00c1228,&mut probe);
  println!("VIDEO_WINDOW index={index} handle={handle:#x} status={r}");if r!=0{continue}
  for(reason,mask)in [("USER",1u32),("VIDEO",2),("SET_WINDOW",4),("BAD_VIDEO",8),("RATING",16),("BLOCK",32),("CA_SCRAMBLED",64),("PVR",128),("UNSUPPORT",256),("STILL_IMAGE",512),("SET_TIMING",1024),("AVSYNC",2048),("FORCE",0x80000000)]{
   let mut mute=[handle,mask,0xcccccccc];let r=call(&file,0xc00c1228,&mut mute);println!("MUTE window={handle:#x} reason={reason} mask={mask:#x} status={r} enabled={} raw={:#x}",mute[2]&255,mute[2]);
  }
 }
 for path in ["/sys/class/backlight/backlight/brightness","/sys/class/backlight/backlight/actual_brightness","/sys/class/backlight/backlight/bl_power"]{if let Ok(value)=fs::read_to_string(path){println!("BACKLIGHT {path}={}",value.trim());}}
 Ok(())
}
