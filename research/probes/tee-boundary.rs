//! Run through the normal app launcher to test its actual TEE permissions.
use std::{fs, io, ffi::c_void};
unsafe extern "C" {
    fn geteuid() -> u32;
    fn ioctl(fd: i32, request: usize, arg: *mut c_void) -> i32;
}
fn check() -> io::Result<String> {
    let uid = unsafe { geteuid() };
    if uid < 10000 { return Err(io::Error::other("probe is not a confined app")); }
    let mut report = format!("uid={uid}\n");
    for path in ["/dev/tee0", "/dev/teepriv0", "/tmp/tee-supplicant.log"] {
        let error = fs::OpenOptions::new().read(true).write(true).open(path)
            .err().ok_or_else(|| io::Error::other(format!("app opened {path}")))?;
        if error.kind() != io::ErrorKind::PermissionDenied { return Err(error); }
        report.push_str(&format!("DENIED {path}\n"));
    }
    for path in ["/data/tee", "/data/vendor", "/mnt/vendor/tmp"] {
        let error = fs::read_dir(path).err()
            .ok_or_else(|| io::Error::other(format!("app read {path}")))?;
        if error.kind() != io::ErrorKind::PermissionDenied { return Err(error); }
        report.push_str(&format!("DENIED {path}\n"));
    }
    for fd in 0..1024 {
        let mut version = [0u32; 3];
        if unsafe { ioctl(fd, 0x800ca400, version.as_mut_ptr().cast()) } == 0 {
            return Err(io::Error::other(format!("inherited TEE descriptor {fd}")));
        }
    }
    report.push_str("PASS: no inherited TEE descriptor; kernel denied app access\n");
    Ok(report)
}
fn main() {
    let result = check();
    let report = match &result { Ok(report) => report.clone(), Err(e) => format!("FAIL: {e}\n") };
    print!("{report}");
    let _ = fs::write("tee-boundary-result.txt", report);
    if result.is_err() { std::process::exit(1); }
}
