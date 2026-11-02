//! Link the Doom engine (tools/legacy-doom/engine/build.sh -> libdoomgeneric.a) for Linux targets.
use std::path::PathBuf;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    println!("cargo:rerun-if-env-changed=DOOMGENERIC_DIR");
    if !target.contains("linux") {
        return; // the engine is only cross-built for the Linux (musl) targets
    }
    let dir = std::env::var("DOOMGENERIC_DIR").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../assets/build/doom-minimal/engine").join(&target)
    });
    let lib = dir.join("libdoomgeneric.a");
    if !lib.exists() {
        panic!("Doom engine not built for {target}: run tools/legacy-doom/engine/build.sh {target} (looked for {})",
               lib.display());
    }
    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=static=doomgeneric");
    println!("cargo:rerun-if-changed={}", lib.display());
}
