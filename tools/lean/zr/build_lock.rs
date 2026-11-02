//! Cargo build scripts share Lake's host tools and generated runtime modules.
//! Hold this lock before invoking Lake, including its first clean bootstrap.
use std::{fs::{self, File, OpenOptions}, path::Path};

pub fn acquire(repo: &Path) -> File {
    let directory = repo.join("tools/lean/zr/.lake");
    fs::create_dir_all(&directory).expect("create Lean host build directory");
    let lock = OpenOptions::new().create(true).read(true).write(true).truncate(false)
        .open(directory.join("cargo-build.lock")).expect("open Lean host build lock");
    lock.lock().expect("lock shared Lean host build outputs");
    lock
}
