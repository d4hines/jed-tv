//! Read-only evdev capture. Write each event immediately, even without a TTY.
use std::{fs::{File,OpenOptions},io::{Read,Write},sync::{Arc,Mutex},thread};

fn main() -> std::io::Result<()> {
    let log=Arc::new(Mutex::new(OpenOptions::new().create(true).truncate(true).write(true)
        .open("/tmp/volume-input-watch.log")?));
    let mut workers=Vec::new();
    for entry in std::fs::read_dir("/dev/input")? {
        let path=entry?.path();
        if !path.file_name().unwrap_or_default().to_string_lossy().starts_with("event") { continue; }
        let Ok(mut device)=File::open(&path) else { continue };
        writeln!(log.lock().unwrap(),"watching {}",path.display())?;
        let log=log.clone();
        workers.push(thread::spawn(move || {
            let offset=std::mem::size_of::<usize>()*2;
            let mut bytes=vec![0u8;offset+8];
            while device.read_exact(&mut bytes).is_ok() {
                let kind=u16::from_ne_bytes(bytes[offset..offset+2].try_into().unwrap());
                let code=u16::from_ne_bytes(bytes[offset+2..offset+4].try_into().unwrap());
                let value=i32::from_ne_bytes(bytes[offset+4..offset+8].try_into().unwrap());
                if kind==1 && matches!(code,113..=115) {
                    let sec=u32::from_ne_bytes(bytes[..4].try_into().unwrap());
                    let usec=u32::from_ne_bytes(bytes[4..8].try_into().unwrap());
                    let mut output=log.lock().unwrap();
                    let _=writeln!(output,"{sec}.{usec:06} {} code={code} value={value}",path.display());
                    let _=output.flush();
                }
            }
        }));
    }
    for worker in workers { let _=worker.join(); }
    Ok(())
}
