use std::ffi::CString;
use std::fs;
use std::io;
use std::os::fd::AsRawFd;

#[repr(C)]
struct BinderDevice { name: [u8; 256], major: u32, minor: u32 }

fn main() -> io::Result<()> {
    fs::create_dir_all("/dev/binderfs")?;
    let source = CString::new("binder").unwrap();
    let target = CString::new("/dev/binderfs").unwrap();
    if unsafe { libc::mount(source.as_ptr(), target.as_ptr(), source.as_ptr(), 0, std::ptr::null()) } != 0
        && io::Error::last_os_error().raw_os_error() != Some(libc::EBUSY) {
        return Err(io::Error::last_os_error());
    }
    let control = fs::OpenOptions::new().read(true).write(true).open("/dev/binderfs/binder-control")?;
    for name in ["binder", "hwbinder", "vndbinder"] {
        let mut device = BinderDevice { name: [0; 256], major: 0, minor: 0 };
        device.name[..name.len()].copy_from_slice(name.as_bytes());
        if !std::path::Path::new(&format!("/dev/binderfs/{name}")).exists()
            && unsafe { libc::ioctl(control.as_raw_fd(), 0xc1086201u32 as _, &mut device) } != 0 {
            return Err(io::Error::last_os_error());
        }
        let alias = format!("/dev/{name}");
        if !std::path::Path::new(&alias).exists() {
            std::os::unix::fs::symlink(format!("binderfs/{name}"), &alias)?;
        }
        println!("{alias} ready");
    }
    Ok(())
}
