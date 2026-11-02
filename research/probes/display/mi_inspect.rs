//! Read-only MI/display inventory. Does not open fbdev: its open/release path
//! can create/destroy OSD resources even without an explicit mutating ioctl.
use std::{ffi::c_void,fs::{self,File},io,os::fd::AsRawFd};
extern "C" {fn ioctl(fd:i32,request:u32,argument:*mut c_void)->i32;}
#[repr(C)]struct Wire{size:u16,pad:[u8;6],pointer:u64}
#[repr(C)]struct Attr{kind:u64,pointer:u64}
#[repr(C)]struct Status{selector:u32,ready:u8,pad:[u8;3]}
fn words(file:&File,request:u32,values:&mut[u32])->i32{
 let mut wire=Wire{size:(values.len()*4) as u16,pad:[0;6],pointer:values.as_mut_ptr() as usize as u64};
 unsafe{ioctl(file.as_raw_fd(),request,(&mut wire as *mut Wire).cast())}
}
fn main()->io::Result<()> {
 let identity=fs::read_to_string("/proc/cmdline")?;
 if !identity.contains("androidboot.serialno=4C0490290FA05A0C0"){return Err(io::Error::other("not the authorized G06"))}
 println!("MI_INSPECT readonly=1 framebuffer_open=0");
 for path in ["/proc/fb","/sys/class/graphics/fb0/name","/sys/class/graphics/fb1/name","/sys/class/graphics/fb1/virtual_size","/sys/class/graphics/fb1/stride","/sys/class/graphics/fb1/bits_per_pixel","/sys/class/graphics/fb1/blank","/tmp/.hello-mi-fb"] {
  if let Ok(value)=fs::read_to_string(path){println!("FILE {path} = {:?}",value.trim());}
 }
 if let Ok(sys)=File::open("/dev/mik/sys"){
  for selector in [1,2,3,6,9,16,40] {
   let mut status=Status{selector,ready:0,pad:[0;3]};let mut attr=Attr{kind:5,pointer:(&mut status as *mut Status) as usize as u64};
   let mut wire=Wire{size:16,pad:[0;6],pointer:(&mut attr as *mut Attr) as usize as u64};
   let result=unsafe{ioctl(sys.as_raw_fd(),0xc0101805,(&mut wire as *mut Wire).cast())};
   println!("MODULE selector={selector} result={result} ready={}",status.ready);
  }
 }
 if let Ok(disp)=File::open("/dev/mik/disp") {
  let mut controller=[0,0];let r=words(&disp,0xc0081206,&mut controller);println!("CONTROLLER result={r} words={controller:x?}");
  if r==0 {let mut mute=[controller[0],u32::MAX,0];let r=words(&disp,0xc00c1228,&mut mute);println!("MUTE result={r} words={mute:x?}");}
 }
 if let Ok(osd)=File::open("/dev/mik/osd") {
  for id in 0..8 {
   let mut layer=[0,id];let r=words(&osd,0xc0081456,&mut layer);println!("LAYER id={id} result={r} words={layer:x?}");
   if r==0 && layer[0]!=0 {let mut enabled=[layer[0],0];let r=words(&osd,0xc008145d,&mut enabled);println!("ENABLED id={id} result={r} words={enabled:x?}");}
  }
 }
 for process in fs::read_dir("/proc")?.flatten(){
  let name=process.file_name();let Some(pid)=name.to_str().and_then(|s|s.parse::<u32>().ok())else{continue};
  let Ok(fds)=fs::read_dir(process.path().join("fd"))else{continue};
  for fd in fds.flatten(){let Ok(target)=fs::read_link(fd.path())else{continue};let path=target.to_string_lossy();
   if path.starts_with("/dev/fb")||path.starts_with("/dev/graphics/fb")||path=="/dev/mma"||path=="/dev/mik/osd" {
    let comm=fs::read_to_string(process.path().join("comm")).unwrap_or_default();println!("OWNER pid={pid} comm={:?} fd={} target={path}",comm.trim(),fd.file_name().to_string_lossy());
   }
  }
 }
 println!("MI_INSPECT done");Ok(())
}
