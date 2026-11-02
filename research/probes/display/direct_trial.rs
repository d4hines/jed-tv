//! Explicit, bounded direct MI layer trial. This program changes display state.
#[path = "../../../src/pid1/display/direct_mi.rs"]
mod direct_mi;
use std::{io,time::Duration,sync::atomic::{AtomicBool,Ordering}};
static RUN:AtomicBool=AtomicBool::new(true);
extern "C" fn stop(_:i32){RUN.store(false,Ordering::Relaxed);}
extern "C"{fn signal(number:i32,handler:extern "C" fn(i32))->usize;}
fn main()->io::Result<()>{
 if std::env::args().nth(1).as_deref()!=Some("--show-color-bars-5s"){return Err(io::Error::other("explicit --show-color-bars-5s required; changes display hardware"))}
 if !std::fs::read_to_string("/proc/cmdline")?.contains("androidboot.serialno=4C0490290FA05A0C0"){return Err(io::Error::other("not the authorized G06"))}
 unsafe{signal(15,stop);signal(2,stop);}
 let mut scanout=direct_mi::Scanout::create()?;scanout.color_bars()?;println!("DIRECT color bars posted; hiding layer in 5 seconds");for _ in 0..50{if !RUN.load(Ordering::Relaxed){break}std::thread::sleep(Duration::from_millis(100));}drop(scanout);println!("DIRECT trial complete; fb1 released");Ok(())
}
