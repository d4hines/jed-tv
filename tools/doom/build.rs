//! Link the Doom engine (tools/doom/build.sh -> libdoomgeneric.a) for Linux targets.
use std::path::PathBuf;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    println!("cargo:rerun-if-env-changed=DOOMGENERIC_DIR");
    if !target.contains("linux") {
        return; // the engine is only cross-built for the Linux (musl) targets
    }
    // Runtime CARGO_MANIFEST_DIR (not env!): the build-script binary is cached in
    // the shared target dir and must follow the checkout if it moves.
    let dir = std::env::var("DOOMGENERIC_DIR").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("../../assets/build/doom").join(&target)
    });
    let lib = dir.join("libdoomgeneric.a");
    if !lib.exists() {
        panic!("Doom engine not built for {target}: run tools/doom/build.sh {target} (looked for {})",
               lib.display());
    }
    let feature = dir.join("NETWORK_FEATURE");
    if std::fs::read_to_string(&feature).ok().as_deref() != Some("chocolate-2.2.1-native-udp-v1\n") {
        panic!("Doom native library is missing private-LAN networking: rebuild with tools/doom/build.sh {target}");
    }
    println!("cargo:rerun-if-changed={}", feature.display());
    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=static=doomgeneric");
    println!("cargo:rerun-if-changed={}", lib.display());
}
