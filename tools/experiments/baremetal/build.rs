//! Linker script, plus the platform crate's remote-key mapping shared verbatim:
//! `src/platform/src/remote.rs` is copied into OUT_DIR with its inner
//! doc comments demoted, so `include!` can wrap it in a no_std module.
use std::{env, fs, path::PathBuf};

fn main() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-arg=-T{}", dir.join("link.ld").display());
    println!("cargo:rerun-if-changed=link.ld");

    let remote = dir.join("../../../src/platform/src/remote.rs");
    println!("cargo:rerun-if-changed={}", remote.display());
    let src = fs::read_to_string(&remote).expect("read platform remote.rs");
    let out: String = src.lines()
        .map(|l| l.strip_prefix("//!").map_or(l.to_string(), |r| format!("//{r}")))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(PathBuf::from(env::var("OUT_DIR").unwrap()).join("remote.rs"), out).unwrap();
}
