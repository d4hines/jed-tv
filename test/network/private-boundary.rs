//! QEMU-only hostile-UID check of the production network installer.
use std::{
    fs,
    os::unix::{
        fs::{symlink, PermissionsExt},
        net::UnixDatagram,
    },
    process::Command,
};
unsafe extern "C" {
    fn mount(
        source: *const u8,
        target: *const u8,
        kind: *const u8,
        flags: usize,
        data: *const u8,
    ) -> i32;
    fn fork() -> i32;
    fn waitpid(pid: i32, status: *mut i32, flags: i32) -> i32;
    fn setgroups(count: usize, groups: *const u32) -> i32;
    fn setresgid(real: u32, effective: u32, saved: u32) -> i32;
    fn setresuid(real: u32, effective: u32, saved: u32) -> i32;
    fn getpid() -> i32;
    fn kill(pid: i32, signal: i32) -> i32;
}
fn mode(path: &str, value: u32) {
    fs::set_permissions(path, fs::Permissions::from_mode(value)).unwrap();
}
fn install() -> bool {
    Command::new("/bin/sh")
        .arg("/tmp/network-install.sh")
        .status()
        .unwrap()
        .success()
}
fn stub(name: &str) {
    let p = format!("/tmp/{name}");
    fs::write(&p, b"#!/bin/sh\nexit 0\n").unwrap();
    mode(&p, 0o700);
}
fn denied(uid: u32, parent: i32) {
    unsafe {
        assert_eq!(setgroups(0, std::ptr::null()), 0);
        assert_eq!(setresgid(4000, 4000, 4000), 0);
        assert_eq!(setresuid(uid, uid, uid), 0);
    }
    for path in [
        "wifi.conf",
        "preset.conf",
        "udhcpc.pid",
        "wpa.pid",
        "lease.sh",
        "wpa_cli",
        "wpa_supplicant",
    ] {
        let path = format!("/run/tv-network/{path}");
        assert!(fs::read(&path).is_err(), "UID {uid} read {path}");
        assert!(
            fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&path)
                .is_err(),
            "UID {uid} wrote {path}"
        );
        assert!(fs::remove_file(&path).is_err(), "UID {uid} removed {path}");
    }
    assert!(symlink("/tmp/hostile", "/run/tv-network/lease.sh.new").is_err());
    assert!(fs::write("/run/tv-network/udhcpc.pid", b"1\n").is_err());
    let client = UnixDatagram::unbound().unwrap();
    assert!(
        client.connect("/run/tv-network/wpa/wlan0").is_err(),
        "UID {uid} connected to WPA control"
    );
    assert!(
        Command::new("/run/tv-network/wpa_cli").status().is_err(),
        "UID {uid} executed WPA control helper"
    );
    assert_ne!(
        unsafe { kill(parent, 0) },
        0,
        "UID {uid} can signal root helper"
    );
}
fn main() {
    unsafe {
        assert_eq!(
            mount(
                b"proc\0".as_ptr(),
                b"/proc\0".as_ptr(),
                b"proc\0".as_ptr(),
                0,
                std::ptr::null()
            ),
            0
        );
    }
    fs::create_dir_all("/bin").unwrap();
    mode("/bin/busybox", 0o700);
    assert!(Command::new("/bin/busybox")
        .args(["--install", "-s", "/bin"])
        .status()
        .unwrap()
        .success());
    fs::create_dir_all("/run").unwrap();
    mode("/run", 0o755);
    mode("/tmp", 0o1777);
    for name in [
        "lean-wpa-supplicant",
        "lean-wpa-cli",
        "wifi-prepare.sh",
        "lean-lease.sh",
        "lean-join.sh",
    ] {
        stub(name);
    }
    fs::write("/tmp/lean-router.conf", b"synthetic credential\n").unwrap();
    mode("/tmp/lean-router.conf", 0o600);
    fs::create_dir_all("/tmp/victim").unwrap();
    fs::write("/tmp/victim/marker", b"unchanged").unwrap();
    symlink("/tmp/victim", "/run/tv-network").unwrap();
    assert!(!install(), "installer accepted a symlink root");
    fs::remove_file("/run/tv-network").unwrap();
    fs::create_dir("/run/tv-network").unwrap();
    mode("/run/tv-network", 0o777);
    assert!(!install(), "installer accepted writable precreated root");
    fs::remove_dir("/run/tv-network").unwrap();
    fs::remove_file("/tmp/lean-wpa-cli").unwrap();
    symlink("/tmp/victim/marker", "/tmp/lean-wpa-cli").unwrap();
    assert!(!install(), "installer accepted a symlink source");
    fs::remove_file("/tmp/lean-wpa-cli").unwrap();
    stub("lean-wpa-cli");
    assert!(install());
    assert!(install(), "private restaging failed");
    assert_eq!(fs::read("/tmp/victim/marker").unwrap(), b"unchanged");
    for name in ["wifi.conf", "udhcpc.pid", "wpa.pid"] {
        let p = format!("/run/tv-network/{name}");
        fs::write(&p, b"synthetic\n").unwrap();
        mode(&p, 0o600);
    }
    fs::create_dir("/run/tv-network/wpa").unwrap();
    mode("/run/tv-network/wpa", 0o700);
    let server = UnixDatagram::bind("/run/tv-network/wpa/wlan0").unwrap();
    let client = UnixDatagram::unbound().unwrap();
    client
        .send_to(b"PING", "/run/tv-network/wpa/wlan0")
        .unwrap();
    let mut bytes = [0; 8];
    assert_eq!(server.recv(&mut bytes).unwrap(), 4);
    let parent = unsafe { getpid() };
    for uid in [10001, 10006, 10008] {
        let pid = unsafe { fork() };
        assert!(pid >= 0);
        if pid == 0 {
            denied(uid, parent);
            std::process::exit(0);
        }
        let mut status = 0;
        assert_eq!(unsafe { waitpid(pid, &mut status, 0) }, pid);
        assert_eq!(status, 0, "UID {uid} boundary failed");
    }
    println!("PASS: production network installer rejects hostile roots/sources; three app UIDs cannot read/write/replace configs, pidfiles, hooks or connect WPA; root control works");
}
