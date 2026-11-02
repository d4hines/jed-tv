//! Compile the policy and menu, using our single-executor runtime on ARM32.
use std::{env, fs, path::{Path, PathBuf}, process::Command};
#[path = "../lean/zr/build_lock.rs"]
mod lean_build_lock;

fn run(cmd: &mut Command) {
    assert!(cmd.status().expect("Lean build tool").success(), "Lean code generation failed");
}
fn object_files(directory: &Path) -> Vec<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(directory).expect("runtime object directory")
        .map(|entry| entry.unwrap().path()).filter(|path| path.extension().is_some_and(|ext| ext == "o")).collect();
    files.sort(); files
}
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../lean").canonicalize().unwrap();
    let repo = root.join("../../..").canonicalize().unwrap();
    println!("cargo:rerun-if-changed={}", repo.join("tools/lean/zr/build_lock.rs").display());
    let _lean_build_lock = lean_build_lock::acquire(&repo);
    let target = env::var("TARGET").unwrap();
    let native = target == "armv7-unknown-linux-musleabihf";
    let mut modules = vec!["Menu", "Supervisor", "Apps", "Sandbox", "Settings", "SettingsFile", "Storage", "StorageText", "Wifi", "Core", "MenuView/Types", "SettingsView", "MenuView", "MenuView/Native", "Hardware/TeeBoundary", "Hardware/ColdPlan", "Hardware/Ownership"];
    if !native { modules.push("CoreProofs"); }
    for module in &modules { println!("cargo:rerun-if-changed={}", root.join(format!("TV/{module}.lean")).display()); }
    for file in ["TV/Proofs.lean", "TV/CoreProofs.lean", "native/linux.c", "native/core_effects.h", "TV.lean", "lean-toolchain", "lakefile.lean"] {
        println!("cargo:rerun-if-changed={}", root.join(file).display());
    }
    run(Command::new("lake").args(["build", "TV.Core", "TV.Proofs", "TV.Menu", "TV.Supervisor", "TV.Sandbox", "TV.SettingsFile", "TV.Storage", "TV.StorageText", "TV.Wifi", "TV.MenuView.Native", "TV.Hardware.TeeBoundary", "TV.Hardware.ColdPlan", "TV.Hardware.Ownership"])
        .current_dir(&root));
    let files: Vec<_> = modules.iter().map(|module| repo.join(format!("assets/build/lake/tv/ir/TV/{module}.c"))).collect();
    // Compile the native mechanism with system headers, then add its object
    // to the same archive. The generated Lean files use different headers.
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let native_object = out.join("native-linux.o");
    let compiler = cc::Build::new().get_compiler();
    run(compiler.to_command().args(["-O2", "-DNDEBUG", "-ffunction-sections", "-fdata-sections", "-c"])
        .arg(root.join("native/linux.c")).arg("-o").arg(&native_object));
    if native {
        println!("cargo:rerun-if-changed={}", repo.join("src/lean/zr").display());
        println!("cargo:rerun-if-changed={}", repo.join("tools/lean/zr").display());
        let rt = out.join("zr");
        run(Command::new("lake").args(["exe", "zr-build", "rt"]).arg(&rt).current_dir(repo.join("tools/lean/zr")));
        let gen = rt.join("gen");
        cc::Build::new().files(files).object(&native_object).include(gen.join("include")).include(gen.join("rt"))
            .define("NDEBUG", None).flag("-ffreestanding").flag("-fno-builtin").flag("-nostdinc")
            .flag("-ffunction-sections").flag("-fdata-sections").flag("-include").flag("lean/zr_fastpath.h")
            .warnings(false).compile("tv_control_lean");
        let mut archive = Command::new("zig");
        archive.args(["ar", "rcs"]).arg(out.join("libzr_native.a")).args(object_files(&rt.join("obj")));
        run(&mut archive);
        println!("cargo:rustc-link-search=native={}", out.display());
        println!("cargo:rustc-link-lib=static=zr_native");
        println!("cargo:zr_include={}", gen.join("include").display());
        println!("cargo:zr_headers={}", gen.join("rt").display());
    } else {
        let output = Command::new("lean").arg("--print-prefix").current_dir(&root).output().expect("Lean prefix");
        assert!(output.status.success());
        let prefix = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
        cc::Build::new().files(files).object(&native_object).include(prefix.join("include")).define("NDEBUG", None)
            .flag("-ffunction-sections").flag("-fdata-sections").warnings(false).compile("tv_control_lean");
        let lib = prefix.join("lib/lean");
        println!("cargo:rustc-link-search=native={}", lib.display());
        println!("cargo:rustc-link-lib=dylib=Init_shared");
        println!("cargo:rustc-link-lib=dylib=leanshared");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib.display());
    }
}
