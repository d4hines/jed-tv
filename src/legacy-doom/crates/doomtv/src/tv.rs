use platform::sys;
use std::ffi::CString;
use std::io::{self, Cursor, Read};
use std::path::Path;
use std::time::Duration;

pub fn halt(message: &str) -> ! {
    eprintln!("doom-init: {message}; PID 1 remains alive");
    loop { std::thread::sleep(Duration::from_secs(60)); }
}

pub fn prepare() -> io::Result<()> {
    if Path::new("/doom-data.xz").exists() {
        let packed = std::fs::read("/doom-data.xz")?;
        let mut raw = Vec::new();
        lzma_rs::xz_decompress(&mut Cursor::new(packed), &mut raw)
            .map_err(|e| io::Error::other(format!("data decompression: {e}")))?;
        let mut data = Cursor::new(raw);
        let mut magic = [0; 8]; data.read_exact(&mut magic)?;
        if &magic != b"DOOMDATA" { return Err(io::Error::other("bad data bundle")); }
        let mut size = [0; 4]; data.read_exact(&mut size)?;
        let count = u32::from_le_bytes(size);
        for _ in 0..count {
            data.read_exact(&mut size)?; let namesize = u32::from_le_bytes(size) as usize;
            data.read_exact(&mut size)?; let filesize = u32::from_le_bytes(size) as usize;
            if namesize > 128 || filesize > 32 * 1024 * 1024 { return Err(io::Error::other("data entry too large")); }
            let mut name = vec![0; namesize]; data.read_exact(&mut name)?;
            let name = String::from_utf8(name).map_err(io::Error::other)?;
            if name.starts_with('/') || name.split('/').any(|p| p == ".." || p.is_empty()) {
                return Err(io::Error::other("invalid data path"));
            }
            let path = Path::new("/").join(name);
            if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
            let mut bytes = vec![0; filesize]; data.read_exact(&mut bytes)?;
            std::fs::write(path, bytes)?;
        }
        if data.position() != data.get_ref().len() as u64 { return Err(io::Error::other("trailing bundle bytes")); }
        eprintln!("doom-init: unpacked {count} RAM-only files");
    }
    if sys::cmdline_arg("doom.modules").as_deref() != Some("0") && Path::new("/modules/order").exists() {
        for module in std::fs::read_to_string("/modules/order")?.lines() {
            if !module.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.') {
                return Err(io::Error::other("bad module name"));
            }
            let bytes = std::fs::read(format!("/modules/{module}"))?;
            let params = CString::new("").unwrap();
            eprintln!("doom-init: loading {module}");
            let result = unsafe { libc::syscall(libc::SYS_init_module, bytes.as_ptr(), bytes.len(), params.as_ptr()) };
            if result != 0 { return Err(io::Error::last_os_error()); }
        }
    }
    Ok(())
}
