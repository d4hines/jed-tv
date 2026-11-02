//! Link the Lean plant (demo/flight.nix -> libcityjsb.a with the Lean runtime) for Linux targets.
//! CITYJSB_DIR names the directory holding lib/libcityjsb.a, lib/libInit.a, lib/libleanrt.a and
//! the C++ runtime archives zig built for the target.
fn main() {
    println!("cargo:rerun-if-env-changed=CITYJSB_DIR");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("linux")
        || std::env::var_os("CARGO_FEATURE_JSB").is_none()
    {
        return; // the plant is only cross-built for the TV's Linux (musl) target
    }
    let dir = std::env::var("CITYJSB_DIR").expect("CITYJSB_DIR: output of demo/flight.nix");
    println!("cargo:rustc-link-search=native={dir}/lib");
    for lib in ["cityjsb", "Init", "leanrt", "c++", "c++abi", "unwind"] {
        println!("cargo:rustc-link-lib=static={lib}");
    }
}
