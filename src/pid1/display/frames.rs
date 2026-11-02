//! Bounded TVF2 reads. App-writable files are never mapped into PID1: an app
//! can truncate or corrupt its buffer, but cannot fault the display owner.
use std::{cell::RefCell, fs::File, io, os::{fd::AsRawFd, unix::fs::{FileExt, OpenOptionsExt}}, time::{Duration, Instant}};

extern "C" { fn fcntl(fd: i32, command: i32, ...) -> i32; }
#[cfg(not(target_os = "macos"))]
#[repr(C)]
struct Flock { kind: i16, whence: i16, start: i64, len: i64, pid: i32 }
#[cfg(target_os = "macos")]
#[repr(C)]
struct Flock { start: i64, len: i64, pid: i32, kind: i16, whence: i16 }
#[cfg(target_os = "macos")]
const SETLK: i32 = 8;
#[cfg(target_os = "macos")]
const READ_LOCK: i16 = 1;
#[cfg(not(target_os = "macos"))]
const READ_LOCK: i16 = 0;
#[cfg(all(not(target_os = "macos"), target_pointer_width = "32"))]
const SETLK: i32 = 13;
#[cfg(all(not(target_os = "macos"), target_pointer_width = "64"))]
const SETLK: i32 = 6;
struct Lock { fd: i32, start: i64, len: i64 }
impl Lock {
    fn read(fd: i32, start: i64, len: i64) -> io::Result<Self> {
        let mut lock = Flock { kind: READ_LOCK, whence: 0, start, len, pid: 0 };
        let until = Instant::now() + Duration::from_millis(2);
        loop {
            if unsafe { fcntl(fd, SETLK, &mut lock) } == 0 { return Ok(Self { fd, start, len }); }
            let error = io::Error::last_os_error();
            if !matches!(error.kind(), io::ErrorKind::Interrupted | io::ErrorKind::WouldBlock | io::ErrorKind::PermissionDenied) {
                return Err(error);
            }
            if Instant::now() >= until { return Err(io::Error::new(io::ErrorKind::TimedOut, "frame lease timeout")); }
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}
impl Drop for Lock {
    fn drop(&mut self) {
        let mut lock = Flock { kind: 2, whence: 0, start: self.start, len: self.len, pid: 0 };
        unsafe { fcntl(self.fd, SETLK, &mut lock); }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Header { width: usize, height: usize, stride: usize, page: u32, sequence: u32 }
fn invalid(message: &'static str) -> io::Error { io::Error::new(io::ErrorKind::InvalidData, message) }
impl Header {
    fn read(file: &File) -> io::Result<Self> {
        let mut bytes = [0u8; 24];
        file.read_exact_at(&mut bytes, 0)?;
        let word = |i: usize| u32::from_le_bytes(bytes[i*4..i*4+4].try_into().unwrap());
        if word(0) != 0x32465654 { return Err(invalid("expected TVF2")); }
        let header = Self { width: word(1) as usize, height: word(2) as usize,
            stride: word(3) as usize, page: word(4), sequence: word(5) };
        if header.width == 0 || header.width > 1920 || header.height == 0 || header.height > 1080
            || header.stride != header.width * 4 || header.page >= 2 {
            return Err(invalid("invalid frame layout"));
        }
        if file.metadata()?.len() != header.file_length() { return Err(invalid("invalid frame length")); }
        Ok(header)
    }
    fn file_length(self) -> u64 { 4096 + (self.stride * self.height * 2) as u64 }
    fn same_layout(self, other: Self) -> bool {
        self.width == other.width && self.height == other.height && self.stride == other.stride
    }
}

pub struct Frames {
    file: File,
    pub width: usize,
    pub height: usize,
    stride: usize,
    scratch: RefCell<Vec<u8>>,
}
impl Frames {
    pub fn open(path: &str) -> io::Result<Self> {
        #[cfg(target_os = "macos")]
        const FLAGS: i32 = 0x100 | 0x4; // O_NOFOLLOW | O_NONBLOCK
        #[cfg(not(target_os = "macos"))]
        const FLAGS: i32 = 0o400000 | 0o4000; // O_NOFOLLOW | O_NONBLOCK
        let file = std::fs::OpenOptions::new().read(true).custom_flags(FLAGS).open(path)?;
        if !file.metadata()?.is_file() { return Err(invalid("expected regular frame file")); }
        let header = { let _header = Lock::read(file.as_raw_fd(), 0, 4096)?; Header::read(&file)? };
        Ok(Self { file, width: header.width, height: header.height, stride: header.stride,
            scratch: RefCell::new(Vec::new()) })
    }
    fn header(&self) -> io::Result<Header> {
        let header = Header::read(&self.file)?;
        if header.width != self.width || header.height != self.height || header.stride != self.stride {
            return Err(invalid("frame layout changed"));
        }
        Ok(header)
    }
    pub fn copy_if_new(&self, previous: u32, out: &mut Vec<u32>) -> io::Result<Option<u32>> {
        let fd = self.file.as_raw_fd();
        for _ in 0..32 {
            let header = { let _header = Lock::read(fd, 0, 4096)?; self.header()? };
            if header.sequence == 0 || header.sequence == previous { return Ok(None); }
            let bytes = self.stride * self.height;
            let offset = 4096 + header.page as usize * bytes;
            let _pixels = Lock::read(fd, offset as i64, bytes as i64)?;
            {
                let _header = Lock::read(fd, 0, 4096)?;
                if self.header()? != header { continue; }
            }
            let mut scratch = self.scratch.borrow_mut();
            scratch.resize(bytes, 0);
            // A malicious writer may ignore its advisory lease or truncate the
            // file. pread returns an error/short read instead of causing SIGBUS.
            self.file.read_exact_at(&mut scratch, offset as u64)?;
            {
                let _header = Lock::read(fd, 0, 4096)?;
                let after = self.header()?;
                if !header.same_layout(after) || after != header { continue; }
            }
            out.clear();
            out.extend(scratch.chunks_exact(4).map(|p| u32::from_le_bytes(p.try_into().unwrap())));
            return Ok(Some(header.sequence));
        }
        Err(io::Error::new(io::ErrorKind::WouldBlock, "frame changed during acquisition"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::{self, OpenOptions}, path::PathBuf, sync::atomic::{AtomicU64, Ordering}};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct TestFile { path: PathBuf, file: File }
    impl TestFile {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!("tvf2-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            let file = OpenOptions::new().read(true).write(true).create_new(true).open(&path).unwrap();
            file.set_len(4096 + 2 * 2 * 1 * 4).unwrap();
            let test = Self { path, file };
            for (i, word) in [0x32465654, 2, 1, 8, 0, 1].into_iter().enumerate() { test.word(i, word); }
            test.file.write_all_at(&0xff123456u32.to_le_bytes(), 4096).unwrap();
            test.file.write_all_at(&0xffabcdefu32.to_le_bytes(), 4100).unwrap();
            test
        }
        fn word(&self, i: usize, word: u32) { self.file.write_all_at(&word.to_le_bytes(), (i*4) as u64).unwrap(); }
        fn reader(&self) -> Frames { Frames::open(self.path.to_str().unwrap()).unwrap() }
    }
    impl Drop for TestFile { fn drop(&mut self) { let _ = fs::remove_file(&self.path); } }
    fn rejected(result: io::Result<Option<u32>>) {
        assert!(matches!(result.unwrap_err().kind(), io::ErrorKind::InvalidData | io::ErrorKind::UnexpectedEof));
    }
    #[test]
    fn reads_both_pages_and_skips_unchanged_sequence() {
        let test = TestFile::new(); let reader = test.reader(); let mut out = vec![];
        assert_eq!(reader.copy_if_new(0, &mut out).unwrap(), Some(1));
        assert_eq!(out, [0xff123456, 0xffabcdef]);
        assert_eq!(reader.copy_if_new(1, &mut out).unwrap(), None);
        test.word(4, 1); test.word(5, 2);
        test.file.write_all_at(&0xff654321u32.to_le_bytes(), 4104).unwrap();
        test.file.write_all_at(&0xfffedcbau32.to_le_bytes(), 4108).unwrap();
        assert_eq!(reader.copy_if_new(1, &mut out).unwrap(), Some(2));
        assert_eq!(out, [0xff654321, 0xfffedcba]);
    }
    #[test]
    fn truncation_after_open_is_an_error_and_does_not_replace_output() {
        let test = TestFile::new(); let reader = test.reader(); let mut out = vec![7, 8];
        test.file.set_len(0).unwrap();
        rejected(reader.copy_if_new(0, &mut out));
        assert_eq!(out, [7, 8]);
    }
    #[test]
    fn malformed_current_headers_are_rejected_even_without_a_new_sequence() {
        for (index, value) in [(0, 0), (1, 0), (1, u32::MAX), (2, 0), (3, u32::MAX), (4, 2)] {
            let test = TestFile::new(); let reader = test.reader(); let mut out = vec![7];
            test.word(index, value);
            rejected(reader.copy_if_new(1, &mut out));
            assert_eq!(out, [7]);
        }
    }
    #[test]
    fn valid_but_changed_dimensions_are_rejected() {
        let test = TestFile::new(); let reader = test.reader(); let mut out = vec![7];
        test.word(1, 1); test.word(3, 4); test.file.set_len(4096 + 2 * 4).unwrap();
        rejected(reader.copy_if_new(0, &mut out));
        assert_eq!(out, [7]);
    }
    #[test]
    fn corrupted_file_can_be_repaired_without_restarting_consumer() {
        let test = TestFile::new(); let reader = test.reader(); let mut out = vec![];
        test.word(4, 2); rejected(reader.copy_if_new(0, &mut out));
        test.word(4, 0);
        assert_eq!(reader.copy_if_new(0, &mut out).unwrap(), Some(1));
        assert_eq!(out, [0xff123456, 0xffabcdef]);
    }
}
