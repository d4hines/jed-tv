//! Run with Rust 1.92, matching the pinned Cargo build toolchain.
#[path = "../../../tools/lean/zr/build_lock.rs"]
mod build_lock;

use std::{fs::{self, OpenOptions}, io::Write, path::Path, process::Command, time::Duration};

fn record(root: &Path, event: &str) {
    writeln!(OpenOptions::new().append(true).create(true).open(root.join("events")).unwrap(),
        "{event} {}", std::process::id()).unwrap();
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() == 3 && args[1] == "worker" {
        let root = Path::new(&args[2]);
        let _lock = build_lock::acquire(root);
        record(root, "enter");
        std::thread::sleep(Duration::from_millis(25));
        record(root, "exit");
        return;
    }
    let root = std::env::temp_dir().join(format!("dande-lean-build-lock-{}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let executable = std::env::current_exe().unwrap();
    let mut children: Vec<_> = (0..8).map(|_| Command::new(&executable)
        .arg("worker").arg(&root).spawn().unwrap()).collect();
    for child in &mut children { assert!(child.wait().unwrap().success()); }
    let events = fs::read_to_string(root.join("events")).unwrap();
    let lines: Vec<_> = events.lines().collect();
    assert_eq!(lines.len(), 16);
    for pair in lines.chunks_exact(2) {
        let owner = pair[0].strip_prefix("enter ").unwrap();
        assert_eq!(pair[1], format!("exit {owner}"), "overlapping generators: {events}");
    }
    fs::remove_dir_all(root).unwrap();
    println!("PASS: eight concurrent build processes serialize and release the shared lock");
}
