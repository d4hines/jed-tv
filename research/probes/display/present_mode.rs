//! Set the presentation mode of an already-owned framebuffer; never allocate it.
use std::{fs::{self,OpenOptions},io,os::fd::AsRawFd};
extern "C" { fn ioctl(fd:i32,request:u32,arg:*mut u32)->i32; }
fn main()->io::Result<()> {
 let mode=std::env::args().nth(1).and_then(|s|s.parse::<u32>().ok()).filter(|v|*v<=1).ok_or_else(||io::Error::other("usage: present-mode 0|1"))?;
 if !fs::read_to_string("/proc/cmdline")?.split_whitespace().any(|s|s=="androidboot.serialno=4C0490290FA05A0C0"){return Err(io::Error::other("requires G06"))}
 let mut owned=false;
 for p in fs::read_dir("/proc")?.flatten(){
  let Ok(fds)=fs::read_dir(p.path().join("fd")) else {continue};
  for fd in fds.flatten(){if let Ok(path)=fs::read_link(fd.path()){if path.to_str().is_some_and(|s|s=="/dev/fb1"||s=="/dev/graphics/fb1"){owned=true}}}
 }
 if !owned{return Err(io::Error::other("framebuffer must already have a persistent owner"))}
 let path=if std::path::Path::new("/dev/fb1").exists(){"/dev/fb1"}else{"/dev/graphics/fb1"};
 let f=OpenOptions::new().read(true).write(true).open(path)?;
 let call=|request,value:&mut u32|->io::Result<()>{let r=unsafe{ioctl(f.as_raw_fd(),request,value)};if r==0{Ok(())}else{Err(io::Error::other(format!("ioctl {request:#x} returned {r}")))}};
 let mut old=0;call(0x80044689,&mut old)?;if old>1{return Err(io::Error::other("unexpected current mode"))}
 let mut value=mode;call(0x40044690,&mut value)?;call(0x80044689,&mut value)?;
 if value!=mode{return Err(io::Error::other("mode readback mismatch"))}
 println!("PRESENT_MODE previous={old} current={value}");Ok(())
}
