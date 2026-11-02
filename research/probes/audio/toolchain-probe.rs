use std::{ffi::{c_char,c_void,CStr,CString},io::{Read,Write}};
extern "C" {
    fn dlopen(name:*const c_char,flags:i32)->*mut c_void;
    fn dlsym(handle:*mut c_void,name:*const c_char)->*mut c_void;
    fn dlerror()->*const c_char;
}
fn main() {
    println!("RUST_BIONIC pid={} args={:?}",std::process::id(),std::env::args().collect::<Vec<_>>());
    let handle=unsafe{dlopen(c"libaaudio.so".as_ptr(),2)};
    if handle.is_null() { let e=unsafe{dlerror()};panic!("dlopen libaaudio: {}",if e.is_null(){"unknown".into()}else{unsafe{CStr::from_ptr(e)}.to_string_lossy()}); }
    for name in ["AAudio_createStreamBuilder","AAudioStreamBuilder_openStream","AAudioStream_write","AAudioStream_getFramesWritten","AAudioStream_getFramesRead"] {
        let name=CString::new(name).unwrap();let symbol=unsafe{dlsym(handle,name.as_ptr())};assert!(!symbol.is_null(),"missing {name:?}");
        println!("AAUDIO_SYMBOL {} OK",name.to_str().unwrap());
    }
    let child=std::thread::spawn(|| std::fs::read_to_string("/proc/sys/kernel/random/boot_id").unwrap());
    println!("THREAD_FILE_OK {}",child.join().unwrap().trim());
    let pair=std::os::unix::net::UnixStream::pair().unwrap();
    let (mut tx,mut rx)=pair;tx.write_all(b"ok").unwrap();let mut b=[0;2];rx.read_exact(&mut b).unwrap();assert_eq!(&b,b"ok");
    println!("RUST_BIONIC_PASS std thread file Unix socket dlopen");
}
