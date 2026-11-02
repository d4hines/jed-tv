//! Build the actual Lean IO supervisor; tvinit links the shared runtime.
use std::{env, path::PathBuf, process::Command};
fn run(command: &mut Command) {
    assert!(command.status().expect("run Lean supervisor tool").success(), "Lean supervisor build failed");
}
fn main() {
    for name in ["Pid1.lean", "Volume.lean", "Volume/Native.lean", "Audio/Protocol.lean", "Audio/Converter.lean", "Audio/Device.lean", "Audio/Cooperative.lean"] {
        println!("cargo:rerun-if-changed=lean/TV/{name}");
    }
    if env::var_os("CARGO_FEATURE_REPL").is_none() { return; }
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..").canonicalize().unwrap();
    let target = env::var("TARGET").unwrap();
    assert_eq!(target, "armv7-unknown-linux-musleabihf", "native Lean PID1 currently targets the G06 ARM32 firmware");
    let include = PathBuf::from(env::var("DEP_DANDE_LEAN_RUNTIME_ZR_INCLUDE").expect("tvinit Zr runtime headers"));
    let headers = PathBuf::from(env::var("DEP_DANDE_LEAN_RUNTIME_ZR_HEADERS").expect("tvinit Zr inline headers"));
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    run(Command::new("lake").args(["build", "TV.Volume.Native", "TV.Audio.Cooperative"]).current_dir(root.join("src/pid1/lean")));
    let generated = out.join("Pid1.c");
    run(Command::new("lake").args(["env", "lean", "-c"]).arg(&generated).arg("TV/Pid1.lean").current_dir(root.join("src/pid1/lean")));
    run(Command::new("zig").args(["cc", "-target", "arm-linux-musleabihf", "-mcpu=cortex_a55", "-O2", "-DNDEBUG", "-ffunction-sections", "-fdata-sections", "-I"]).arg(&include).arg("-I").arg(&headers).args(["-ffreestanding", "-fno-builtin", "-nostdinc", "-include", "lean/zr_fastpath.h"]).arg("-c").arg(&generated).arg("-o").arg(out.join("Pid1.o")));
    let mut archive = Command::new("zig");
    archive.args(["ar", "rcs"]).arg(out.join("libpid1lean.a")).arg(out.join("Pid1.o"));
    for (name, source) in [("Volume", "Volume.c"), ("VolumeNative", "Volume/Native.c"), ("AudioProtocol", "Audio/Protocol.c"), ("AudioConverter", "Audio/Converter.c"), ("AudioDevice", "Audio/Device.c"), ("AudioCooperative", "Audio/Cooperative.c")] {
        let object = out.join(format!("{name}.o"));
        run(Command::new("zig").args(["cc", "-target", "arm-linux-musleabihf", "-mcpu=cortex_a55", "-O2", "-DNDEBUG", "-ffunction-sections", "-fdata-sections", "-I"])
            .arg(&include).arg("-I").arg(&headers).args(["-ffreestanding", "-fno-builtin", "-nostdinc", "-include", "lean/zr_fastpath.h"]).arg("-c").arg(root.join("assets/build/lake/tv/ir/TV").join(source)).arg("-o").arg(&object));
        archive.arg(object);
    }
    run(&mut archive);
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=pid1lean");
}
