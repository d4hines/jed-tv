//! Rust TVF2 display service. --probe never creates or shows a layer.
mod native;mod frames;mod render;mod pixels;mod overlay;
use std::{io::{self},time::{Duration,Instant},sync::atomic::{AtomicBool,Ordering}};
static RUN:AtomicBool=AtomicBool::new(true);
extern "C" fn stop(_:i32){RUN.store(false,Ordering::Relaxed);}
extern "C"{fn signal(number:i32,handler:extern "C" fn(i32))->usize;fn _exit(status:i32)->!;}
fn main()->io::Result<()> {
 let api=native::Api::load()?;
 let immediate_exit=api.requires_immediate_exit();
 // Release our surfaces first, then let process death terminate Binder's
 // worker pool. libc exit runs libgui's global destructors while those
 // workers are still alive (a destroyed-mutex abort on Android 14).
 let result=run(api);
 if immediate_exit {
  if let Err(error)=&result{eprintln!("native-display: {error}");}
  let _=std::io::Write::flush(&mut std::io::stdout());
  let _=std::io::Write::flush(&mut std::io::stderr());
  unsafe{_exit(if result.is_ok(){0}else{1})}
 }
 result
}
fn run(api:native::Api)->io::Result<()> {
 let mut args=std::env::args().skip(1);let mode=args.next().unwrap_or_else(||"--probe".into());
 println!("NATIVE_DISPLAY symbols OK");
 let client=api.client()?;println!("NATIVE_DISPLAY composer initCheck=0");
 if mode=="--probe" {return Ok(())}
 if mode!="--hidden-probe"&&mode!="--run" {return Err(io::Error::other("usage: native-display [--probe|--hidden-probe|--run FRAMEFILE]"))}
 let (size,height)=if mode=="--hidden-probe"{(64,64)}else{
  let geometry=std::env::var("TV_DISPLAY_SIZE").unwrap_or_else(|_|"1280x720".into());
  let (w,h)=geometry.split_once('x').ok_or_else(||io::Error::other("TV_DISPLAY_SIZE must be WIDTHxHEIGHT"))?;
  let w=w.parse::<u32>().ok().filter(|n|(1..=8192).contains(n));
  let h=h.parse::<u32>().ok().filter(|n|(1..=8192).contains(n));
  (w.ok_or_else(||io::Error::other("invalid display width"))?,h.ok_or_else(||io::Error::other("invalid display height"))?)
 };
 let surface=api.surface(&client,"dande-rust-display",size,height)?;
 println!("NATIVE_DISPLAY hidden SurfaceControl created");let window=api.window(&surface)?;println!("NATIVE_DISPLAY native Surface created");
 if mode=="--hidden-probe" {
  api.draw(&window,|buffer|{println!("NATIVE_DISPLAY buffer {}x{} stride={} format={}",buffer.width,buffer.height,buffer.stride,buffer.format);if buffer.width!=64||buffer.height!=64||buffer.stride<64||buffer.bits.is_null(){return Err(io::Error::other("unexpected buffer layout"))}unsafe{std::slice::from_raw_parts_mut(buffer.bits.cast::<u32>(),buffer.stride as usize*buffer.height as usize)}.fill(0xff102030);Ok(())})?;
  println!("NATIVE_DISPLAY hidden buffer lock/post PASS");return Ok(())
 }
 let path=args.next().unwrap_or_else(||"/tmp/app.frames".into());let frames=frames::Frames::open(&path)?;
 // Some TV composers retain the boot logo until several frames have been
 // presented. A static menu otherwise advances that counter only once a minute.
 // Keep ordinary app sessions unchanged; the native boot script opts in.
 let startup_frames=std::env::var("TV_DISPLAY_STARTUP_FRAMES").ok()
  .map(|s|s.parse::<u64>().ok().filter(|n|(1..=64).contains(n))
   .ok_or_else(||io::Error::other("TV_DISPLAY_STARTUP_FRAMES must be 1..64")))
  .transpose()?.unwrap_or(1);
 unsafe{signal(15,stop);signal(2,stop);}
 let mut pixels=Vec::new();let mut sequence=0;let mut overlay=overlay::Overlay::default();let mut shown=false;
 let mut count=0u64;let mut copy_ns=0u128;let mut post_ns=0u128;
 let mut last_post=Instant::now();
 while RUN.load(Ordering::Relaxed) {
  let t=Instant::now();let next=frames.copy_if_new(sequence,&mut pixels)?;copy_ns+=t.elapsed().as_nanos();
  let changed=overlay.refresh();let visible=overlay.until.is_some_and(|v|v>Instant::now());let visibility_changed=visible!=overlay.was_visible;
  let startup_refresh=sequence!=0&&count<startup_frames&&last_post.elapsed()>=Duration::from_millis(100);
  if next.is_none()&&!changed&&!visibility_changed&&!startup_refresh{std::thread::sleep(Duration::from_millis(3));continue}
  if let Some(n)=next{sequence=n;}
  if sequence==0{continue}
  let t=Instant::now();api.draw(&window,|buffer|render::compose(buffer,&pixels,frames.width,frames.height,visible.then_some(overlay.pixels.as_slice())))?;
  if !shown {api.visible(&surface,true)?;shown=true;}
  post_ns+=t.elapsed().as_nanos();count+=1;overlay.was_visible=visible;
  last_post=Instant::now();
  if count==startup_frames {println!("NATIVE_DISPLAY startup complete frames={count}");}
  if count==1||count%350==0 {println!("NATIVE_DISPLAY frames={count} sequence={sequence} avgCopyUs={} avgPostUs={}",copy_ns/count as u128/1000,post_ns/count as u128/1000);}
 }
 api.visible(&surface,false)?;println!("NATIVE_DISPLAY stopped");Ok(())
}
