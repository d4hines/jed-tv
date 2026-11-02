//! Pure format oracles plus the real CoreStore IO adapter on Linux.
use std::{ffi::c_void, io};
#[path = "../../../../src/pid1/tvinit/src/lean_runtime.rs"]
mod lean_runtime;
#[cfg(target_os = "linux")]
#[path = "../../../../src/pid1/tvinit/src/settings_store.rs"]
mod settings_store;
#[path = "../../../../src/pid1/tvinit/src/storage_records.rs"]
mod storage_records;
#[path = "../../../../src/pid1/tvinit/src/wifi_codec.rs"]
mod wifi_codec;
#[cfg(target_os = "linux")]
#[path = "../../../../src/pid1/tvinit/src/settings_wifi.rs"]
mod settings_wifi;
mod wifi;
#[path = "../records.rs"]
mod records;
extern "C" {
    fn initialize_tv_TV_SettingsFile(builtin: u8) -> *mut c_void;
    fn tv_settings_file_parse(text: *mut c_void) -> u32;
    fn tv_settings_file_default(unused: u32) -> u32;
    fn tv_settings_file_format(pin: u32, mask: u32) -> *mut c_void;
    fn lean_mk_string_from_bytes(data: *const u8, len: usize) -> *mut c_void;
    fn initialize_Strings(builtin: u8) -> *mut c_void;
    fn settings_test_chars(value: *mut c_void) -> *mut c_void;
    fn settings_test_string(value: *mut c_void) -> *mut c_void;
    fn tv_storage_directory_accepted(mode: u32, uid: u32, owner: u32) -> u32;
    fn tv_storage_file_accepted(mode: u32, uid: u32, owner: u32, links: u64, bytes: u64) -> u32;
    fn tv_storage_block_accepted(mode: u32, actual: u64, expected: u64) -> u32;
    fn tv_storage_cache_geometry_accepted(start: u64, sectors: u64) -> u32;
    fn tv_storage_filesystem_accepted(magic: u32, blocks: u32, shift: u32) -> u32;
    fn tv_storage_mount_decision(actual: u64, expected: u64, owner: u32) -> u32;
    fn tv_storage_commit_start(unused: u32) -> u32;
    fn tv_storage_commit_next(stage: u32, ok: u32) -> u32;
}

// This runs before any Lean module/runtime initialization. The worker-facing
// storage ABI must remain entirely scalar, even on 32-bit ARM.
fn storage_scalar_tests() {
    for mode in 0..=0xffff {
        for (uid, owner) in [(0, 0), (1, 0), (1, 1), (u32::MAX, u32::MAX)] {
            let directory = mode & 0xf000 == 0x4000 && uid == owner && mode & 0x1ff == 0x1c0;
            assert_eq!(unsafe { tv_storage_directory_accepted(mode, uid, owner) }, directory as u32);
            for (links, bytes) in [(0, 0), (1, 0), (1, 64), (1, 65), (2, 1), (u64::MAX, u64::MAX)] {
                let file = mode & 0xf000 == 0x8000 && uid == owner && mode & 0x1ff == 0x180 && links == 1 && bytes <= 64;
                assert_eq!(unsafe { tv_storage_file_accepted(mode, uid, owner, links, bytes) }, file as u32);
            }
        }
        for (actual, expected) in [(0, 0), (1, 0), (u64::MAX, u64::MAX)] {
            let block = mode & 0xf000 == 0x6000 && actual == expected;
            assert_eq!(unsafe { tv_storage_block_accepted(mode, actual, expected) }, block as u32);
        }
    }
    for (actual, expected, owner) in [(29, 29, 1000), (29, 29, 0), (1, 29, 0), (1, 29, 1000), (u64::MAX, 0, 0)] {
        let result = if actual == expected { 0 } else if owner == 0 { 1 } else { 2 };
        assert_eq!(unsafe { tv_storage_mount_decision(actual, expected, owner) }, result);
    }
    for start in [0, 9019391, 9019392, 9019393, u64::MAX] {
        for sectors in [0, 204799, 204800, 204801, u64::MAX] {
            assert_eq!(unsafe { tv_storage_cache_geometry_accepted(start, sectors) }, u32::from(start == 9019392 && sectors == 204800));
        }
    }
    for magic in [0, 0xef53, 0xffff] {
        for shift in [0, 1, 2, 6, 7, 32, 64, u32::MAX] {
            for blocks in [0, 1600, 25600, 51200, 102400, u32::MAX] {
                let accepted = magic == 0xef53 && shift <= 6 && (u64::from(blocks) << (10 + shift)) == 104857600;
                assert_eq!(unsafe { tv_storage_filesystem_accepted(magic, blocks, shift) }, accepted as u32);
            }
        }
    }
    for failures in 0..32 {
        let mut stage = unsafe { tv_storage_commit_start(0) };
        let mut trace = Vec::new();
        while stage < 6 {
            trace.push(stage);
            let ok = stage < 5 && failures & (1 << stage) == 0;
            stage = unsafe { tv_storage_commit_next(stage, u32::from(ok)) };
        }
        let first = (0..5).find(|i| failures & (1 << i) != 0);
        let expected: Vec<_> = match first { Some(i) => (0..=i).chain([5]).collect(), None => (0..5).collect() };
        assert_eq!(trace, expected);
        assert_eq!(stage, if failures == 0 { 6 } else { 7 });
    }
    assert_eq!(unsafe { tv_storage_commit_next(u32::MAX, 1) }, 7);
    println!("PASS: scalar storage acceptance matches prior predicates; all commit failures are fail-closed before runtime initialization");
}

#[cfg(target_os = "linux")]
fn storage_commit_tests() -> io::Result<()> {
    use settings_store::{commit, CommitAction};
    // Exercise the production Rust effect interpreter, failing each real
    // operation boundary. A failed cleanup must preserve the original errno.
    for fail_at in 0..=5 {
        for cleanup_fails in [false, true] {
            let mut trace = Vec::new();
            let result = commit(|action| {
                let code = match action {
                    CommitAction::Create(file) => { *file = Some(std::fs::File::open("/dev/null")?); 0 }
                    CommitAction::Write(_) => 1,
                    CommitAction::SyncFile(_) => 2,
                    CommitAction::Rename => 3,
                    CommitAction::SyncDirectory => 4,
                    CommitAction::Cleanup => 5,
                };
                trace.push(code);
                if code == fail_at { Err(io::Error::from_raw_os_error(28)) }
                else if code == 5 && cleanup_fails { Err(io::Error::from_raw_os_error(13)) }
                else { Ok(()) }
            });
            if fail_at == 5 {
                assert_eq!(trace, [0, 1, 2, 3, 4]);
                result?;
            } else {
                assert_eq!(trace, (0..=fail_at).chain([5]).collect::<Vec<_>>());
                assert_eq!(result.unwrap_err().raw_os_error(), Some(28));
            }
        }
    }
    // atomic_write is also used by native worker threads. No worker may touch
    // boxed Lean state; the compiled policy should be safe concurrently.
    let workers: Vec<_> = (0..4).map(|_| std::thread::spawn(|| {
        for _ in 0..1000 {
            assert_eq!(unsafe { tv_storage_directory_accepted(0x41c0, 0, 0) }, 1);
            let mut stage = unsafe { tv_storage_commit_start(0) };
            for _ in 0..5 { stage = unsafe { tv_storage_commit_next(stage, 1) }; }
            assert_eq!(stage, 6);
        }
    })).collect();
    for worker in workers { worker.join().unwrap(); }
    println!("PASS: production commit effect sequence, each injected failure, preserved errno, concurrent scalar workers");
    Ok(())
}
#[repr(C)]
struct LeanString {
    header: [u8; 8],
    size: usize,
    capacity: usize,
    length: usize,
}
fn decode(text: &str) -> u32 {
    unsafe { tv_settings_file_parse(lean_mk_string_from_bytes(text.as_ptr(), text.len())) }
}
unsafe fn text_value(value: *mut c_void) -> String {
    let header = &*value.cast::<LeanString>();
    assert_eq!(header.header[7], 249);
    let bytes = std::slice::from_raw_parts(
        value.cast::<u8>().add(std::mem::size_of::<LeanString>()),
        header.size - 1,
    );
    let text = std::str::from_utf8(bytes).unwrap().to_owned();
    lean_runtime::release_object(value);
    text
}
fn encode(pin: u32, mask: u32) -> String {
    unsafe {
        text_value(tv_settings_file_format(pin, mask))
    }
}
unsafe fn retain(value: *mut c_void) {
    if value as usize & 1 == 0 {
        let rc = value.cast::<i32>();
        assert!(*rc > 0);
        *rc += 1;
    }
}
fn same_string(text: &str) {
    unsafe {
        let original = lean_mk_string_from_bytes(text.as_ptr(), text.len());
        retain(original);
        let first = settings_test_chars(original);
        let second = settings_test_chars(original);
        retain(first);
        assert_eq!(text_value(settings_test_string(first)), text);
        assert_eq!(text_value(settings_test_string(first)), text);
        assert_eq!(text_value(settings_test_string(second)), text);
    }
}
fn string_tests() {
    for text in ["", "a\0b\n", "\u{7f}\u{80}\u{7ff}\u{800}\u{ffff}\u{10000}\u{10ffff}"] { same_string(text); }
    let mut chunk = String::new();
    let mut count = 0;
    for code in 0..=0x10ffff {
        if let Some(c) = char::from_u32(code) {
            chunk.push(c);
            count += 1;
            if count % 4096 == 0 { same_string(&chunk); chunk.clear(); }
        }
    }
    same_string(&chunk);
    assert_eq!(count, 1_112_064);
    println!("PASS: String/List round trips cover all 1,112,064 Unicode scalars, embedded NUL and shared ownership");
}
fn reference(text: &str) -> Option<u32> {
    let parts: Vec<_> = text.split_whitespace().collect();
    if parts.len() != 3
        || parts[0] != "DANDE1"
        || parts[1].len() != 4
        || !parts[1].bytes().all(|c| c.is_ascii_digit())
    {
        return None;
    }
    let pin = parts[1].parse::<u32>().ok()?;
    let mask = parts[2].parse::<u32>().ok()?;
    if pin >= 10000 || mask > 255 {
        None
    } else {
        Some(pin | (mask << 16))
    }
}
fn same(text: &str) {
    assert_eq!(
        decode(text),
        reference(text).unwrap_or(0x80000000),
        "input {text:?}"
    );
}
fn format_tests() {
    assert_eq!(unsafe { tv_settings_file_default(0) }, 247 << 16);
    for pin in 0..10000 {
        let text = encode(pin, 31);
        assert_eq!(text, format!("DANDE1 {pin:04} 31\n"));
        assert_eq!(decode(&text), pin | (31 << 16));
    }
    for mask in 0..256 {
        for pin in [0, 1, 42, 9999] {
            let text = encode(pin, mask);
            assert_eq!(text, format!("DANDE1 {pin:04} {mask}\n"));
            assert_eq!(decode(&text), pin | (mask << 16));
        }
    }
    for (pin, mask) in [(10000, 0), (u32::MAX, 0), (1, 256), (1, u32::MAX)] {
        assert_eq!(encode(pin, mask), "");
    }
    for text in [
        "",
        "DANDE1",
        "DANDE2 0000 31",
        "DANDE1 000 31",
        "DANDE1 00000 31",
        "DANDE1 +123 31",
        "DANDE1 -123 31",
        "DANDE1 ０１２３ 31",
        "DANDE1 0000 256",
        "DANDE1 0000 -0",
        "DANDE1 0000 -1",
        "DANDE1 0000 4294967295",
        "DANDE1 0000 4294967296",
        "DANDE1 0000 +",
        "DANDE1 0000 ١",
        "DANDE1 0000 0xff",
        "DANDE1 0000 31 extra",
        "DANDE1\0 0000 31",
        "DANDE1 0042 +000255",
        " \r\nDANDE1\t0042\u{b}\u{c}00031 \r\n",
    ] {
        same(text);
    }
    for n in [
        9, 10, 11, 12, 13, 32, 133, 160, 5760, 8192, 8193, 8194, 8195, 8196, 8197, 8198, 8199,
        8200, 8201, 8202, 8232, 8233, 8239, 8287, 12288, 6158, 8203, 65279,
    ] {
        let space = char::from_u32(n).unwrap();
        same(&format!("{space}DANDE1{space}0042{space}+000255{space}"));
    }
    let alphabet = b"DANDE1 +-0123456789\t\nxyz";
    let mut seed = 0x12345678u32;
    for _ in 0..10000 {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let len = (seed % 64) as usize;
        let mut text = String::new();
        for _ in 0..len {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            text.push(alphabet[seed as usize % alphabet.len()] as char);
        }
        same(&text);
    }
    println!("PASS: Lean settings PIN/mask formatting, exact masks, Unicode whitespace and malformed-input oracle");
}
#[cfg(target_os = "linux")]
fn storage_tests() -> io::Result<()> {
    use settings_store::{atomic_write, CoreStore};
    use std::{
        fs,
        os::unix::fs::{symlink, PermissionsExt},
    };
    let root = std::env::temp_dir().join(format!("lean-settings-{}", std::process::id()));
    fs::create_dir_all(&root)?;
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
    let (store, policy) = CoreStore::read_from(root.clone())?;
    assert_eq!(policy, 247u64 << 16);
    store.save(42, 31)?;
    assert_eq!(fs::read(root.join("parental"))?, b"DANDE1 0042 31\n");
    let (store, saved) = CoreStore::read_from(root.clone())?;
    assert_eq!(saved, 42 | (31u64 << 16));
    store.save(9999, 0)?;
    assert_eq!(saved, 42 | (31u64 << 16));
    assert_eq!(CoreStore::read_from(root.clone())?.1, 9999);
    let before = fs::read(root.join("parental"))?;
    assert!(store.save(10000, 0).is_err());
    assert!(store.save(1, 256).is_err());
    assert_eq!(fs::read(root.join("parental"))?, before);
    for text in [b"DANDE1 0042 999\n".as_slice(), b"\xff\n", &[b' '; 65]] {
        atomic_write(&root.join("parental"), text)?;
        assert!(CoreStore::read_from(root.clone()).is_err());
    }
    atomic_write(&root.join("parental"), b"DANDE1 0042 31\n")?;
    fs::set_permissions(root.join("parental"), fs::Permissions::from_mode(0o644))?;
    assert!(CoreStore::read_from(root.clone()).is_err());
    fs::remove_file(root.join("parental"))?;
    atomic_write(&root.join("target"), b"DANDE1 0000 31\n")?;
    symlink(root.join("target"), root.join("parental"))?;
    assert!(CoreStore::read_from(root.clone()).is_err());
    store.save(1234, 255)?;
    assert_eq!(fs::read(root.join("target"))?, b"DANDE1 0000 31\n");
    assert_eq!(CoreStore::read_from(root.clone())?.1, 1234 | (255u64 << 16));
    fs::remove_file(root.join("parental"))?;
    fs::hard_link(root.join("target"), root.join("parental"))?;
    assert!(CoreStore::read_from(root.clone()).is_err());
    fs::remove_dir_all(&root)?;
    assert!(store.save(1111, 0).is_err());
    assert_eq!(
        settings_store::app_id("/bin/lean/doomtv"),
        settings_store::app_id("/bin/doomtv")
    );
    assert_eq!(
        settings_store::app_id("/bin/lean/ytplay"),
        settings_store::app_id("/bin/ytplay")
    );
    println!("PASS: real CoreStore fail-closed loading, native save errors, private files and atomic replacement");
    Ok(())
}
fn main() -> io::Result<()> {
    records::check();
    storage_scalar_tests();
    unsafe {
        lean_runtime::consume_io_unit(initialize_tv_TV_SettingsFile(1))?;
        lean_runtime::consume_io_unit(initialize_Strings(1))?;
    }
    string_tests();
    format_tests();
    wifi::check();
    #[cfg(target_os = "linux")]
    {
        storage_tests()?;
        storage_commit_tests()?;
    }
    Ok(())
}
