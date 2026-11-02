use std::ffi::CString;
use std::io::Write;

fn cpio(data: &[u8]) -> std::collections::BTreeMap<String, (Vec<u32>, Vec<u8>)> {
    let mut result = std::collections::BTreeMap::new();
    let mut offset = 0;
    loop {
        assert_eq!(&data[offset..offset + 6], b"070701");
        let fields = (0..13).map(|i| u32::from_str_radix(std::str::from_utf8(&data[offset + 6 + i * 8..offset + 14 + i * 8]).unwrap(), 16).unwrap()).collect::<Vec<_>>();
        let name = std::str::from_utf8(&data[offset + 110..offset + 110 + fields[11] as usize - 1]).unwrap().to_owned();
        offset = (offset + 110 + fields[11] as usize + 3) & !3;
        let content = data[offset..offset + fields[6] as usize].to_vec();
        offset = (offset + fields[6] as usize + 3) & !3;
        if name == "TRAILER!!!" { return result; }
        result.insert(name, ([1, 2, 3, 9, 10].iter().map(|index| fields[*index]).collect(), content));
    }
}

fn main() {
    assert_eq!(std::process::id(), 1);
    let role = if std::env::args().next().unwrap().ends_with("launcher-hello") {
        "hello"
    } else if std::path::Path::new("/launcher-stock-init").exists() {
        "android"
    } else { "stock-fallback" };
    assert_eq!(std::env::current_dir().unwrap().to_str(), Some("/"));
    unsafe { libc::mount(c"proc".as_ptr(), c"/proc".as_ptr(), c"proc".as_ptr(), 0, std::ptr::null()); }
    let mounts = std::fs::read_to_string("/proc/mounts").unwrap();
    for line in mounts.lines() {
        let point = line.split_whitespace().nth(1).unwrap();
        assert!(!["/launcher-cache", "/sys", "/dev"].contains(&point), "launcher mount leaked: {line}");
    }
    if role != "hello" {
        assert_eq!(std::fs::read("/init").unwrap(), std::fs::read("/proc/self/exe").unwrap());
    }
    println!("PROBE role={role} pid=1 cwd=/ no_launcher_mounts=true init_restored=true");
    let cmdline = std::fs::read_to_string("/proc/cmdline").unwrap();
    if cmdline.split_whitespace().any(|s| s == "launcher.test=restore-image") {
        let status = std::process::Command::new("/check-magiskboot").args(["cpio", "/check-candidate.cpio", "test"]).status().unwrap();
        assert_eq!(status.code(), Some(1));
        assert!(std::process::Command::new("/check-magiskboot").args(["cpio", "/check-candidate.cpio", "restore"]).status().unwrap().success());
        assert_eq!(cpio(&std::fs::read("/check-candidate.cpio").unwrap()), cpio(&std::fs::read("/check-stock.cpio").unwrap()));
        println!("PROBE upstream_magisk_restore_matches_stock=true");
    }
    if cmdline.split_whitespace().any(|s| s == "launcher.test=control-arm" || s == "launcher.test=control-cancel") {
        assert_eq!(role, "android");
        unsafe {
            assert_eq!(libc::mount(c"devtmpfs".as_ptr(), c"/dev".as_ptr(), c"devtmpfs".as_ptr(), 0, std::ptr::null()), 0);
            assert_eq!(libc::mount(c"sysfs".as_ptr(), c"/sys".as_ptr(), c"sysfs".as_ptr(), 0, std::ptr::null()), 0);
            assert_eq!(libc::mount(c"/dev/vda".as_ptr(), c"/cache".as_ptr(), c"ext4".as_ptr(), 0, std::ptr::null()), 0);
        }
        for action in ["--inspect", "--arm", "--inspect"] {
            assert!(std::process::Command::new("/launcher-self").arg(action).status().unwrap().success());
        }
        assert!(!std::process::Command::new("/launcher-self").arg("--arm").status().unwrap().success());
        if cmdline.split_whitespace().any(|s| s == "launcher.test=control-cancel") {
            assert!(std::process::Command::new("/launcher-self").arg("--cancel").status().unwrap().success());
            assert!(!std::path::Path::new("/cache/tv-launcher/request").exists());
        }
        assert_eq!(unsafe { libc::umount2(c"/cache".as_ptr(), 0) }, 0);
        println!("PROBE native_control_passed=true duplicate_arm_refused=true");
    }
    if role == "hello" {
        assert_eq!(unsafe { libc::mount(c"devtmpfs".as_ptr(), c"/dev".as_ptr(), c"devtmpfs".as_ptr(), 0, std::ptr::null()) }, 0);
        let source = CString::new("/dev/vda").unwrap();
        assert_eq!(unsafe { libc::mount(source.as_ptr(), c"/launcher-cache".as_ptr(), c"ext4".as_ptr(), libc::MS_RDONLY, std::ptr::null()) }, 0);
        assert!(!std::path::Path::new("/launcher-cache/tv-launcher/request").exists());
        assert_eq!(std::fs::read("/launcher-cache/sentinel").unwrap(), b"KEEP-UNRELATED-CACHE-FILE\n");
        println!("PROBE consumed_absent=true sentinel_preserved=true");
        unsafe { libc::umount2(c"/launcher-cache".as_ptr(), 0); }
    }
    std::io::stdout().flush().unwrap();
    unsafe { libc::sync(); libc::reboot(libc::RB_AUTOBOOT); }
    loop { std::thread::park(); }
}
