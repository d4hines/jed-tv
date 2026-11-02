//! PID1-owned volume overlay input.
use std::{io::Read,time::{Duration,Instant}};
#[derive(Default)]pub struct Overlay{revision:u64,pub pixels:Vec<u32>,pub until:Option<Instant>,pub was_visible:bool}
impl Overlay{
 pub fn refresh(&mut self)->bool{
  let directory=std::env::var("TV_RUNTIME_DIR").unwrap_or_else(|_|"/tmp".into());
  let Ok(mut file)=std::fs::File::open(std::path::Path::new(&directory).join("app-volume-osd.rgba"))else{return false};let mut header=[0u8;16];if file.read_exact(&mut header).is_err(){return false}
  if u32::from_le_bytes(header[..4].try_into().unwrap())!=0x3144534f{return false}let revision=u64::from_le_bytes(header[4..12].try_into().unwrap());if revision==self.revision{return false}
  let mut bytes=vec![0;480*88*4];if file.read_exact(&mut bytes).is_err(){return false}self.pixels=bytes.chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect();self.revision=revision;self.until=Some(Instant::now()+Duration::from_millis(2200));true
 }
}
