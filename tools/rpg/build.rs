//! Compile the original pure Lean adventure with the owned ARM32 runtime.
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
#[path = "../lean/zr/build_lock.rs"]
mod lean_build_lock;
fn run(c: &mut Command) {
    assert!(c.status().unwrap().success(), "RPG Lean build failed");
}
fn files(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            files(&p, out)
        } else if p.extension().is_some_and(|s| s == "c") {
            out.push(p);
        }
    }
}
fn main() {
    let pkg = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let root = pkg.join("../..").canonicalize().unwrap();
    println!("cargo:rerun-if-changed={}", root.join("tools/lean/zr/build_lock.rs").display());
    let _lean_build_lock = lean_build_lock::acquire(&root);
    println!("cargo:rerun-if-changed=lean");
    println!("cargo:rerun-if-changed=../lean/zr");
    run(Command::new("lake")
        .args(["-d", "src/rpgtv/lean", "build"])
        .current_dir(&root));
    let mut inputs = vec![];
    files(&root.join("assets/build/lake/rpgtv/ir"), &mut inputs);
    inputs.sort();
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut cc = cc::Build::new();
    cc.files(inputs)
        .define("NDEBUG", None)
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .warnings(false);
    if env::var("TARGET").unwrap() == "armv7-unknown-linux-musleabihf" {
        println!("cargo:rerun-if-env-changed=RPG_ZR_RUNTIME_DIR");
        let rt = if let Ok(p) = env::var("RPG_ZR_RUNTIME_DIR") {
            PathBuf::from(p)
        } else {
            let rt = out.join("runtime");
            run(Command::new("lake")
                .args(["-d", "tools/lean/zr", "exe", "zr-build", "rt"])
                .arg(rt.join("zr"))
                .current_dir(&root));
            let mut objects: Vec<_> = fs::read_dir(rt.join("zr/obj"))
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|p| p.extension().is_some_and(|s| s == "o"))
                .collect();
            objects.sort();
            run(Command::new("zig")
                .args(["ar", "rcs"])
                .arg(rt.join("libzr_native.a"))
                .args(objects));
            rt
        };
        cc.include(rt.join("zr/gen/include"))
            .include(rt.join("zr/gen/rt"))
            .flag("-ffreestanding")
            .flag("-fno-builtin")
            .flag("-nostdinc")
            .flag("-include")
            .flag("lean/zr_fastpath.h");
        cc.compile("rpg_engine");
        println!("cargo:rustc-link-search=native={}", rt.display());
        println!("cargo:rustc-link-lib=static=zr_native");
    } else {
        let o = Command::new("lean").arg("--print-prefix").output().unwrap();
        let prefix = PathBuf::from(String::from_utf8(o.stdout).unwrap().trim());
        cc.include(prefix.join("include")).compile("rpg_engine");
        let lib = prefix.join("lib/lean");
        println!("cargo:rustc-link-search=native={}", lib.display());
        println!("cargo:rustc-link-lib=dylib=Init_shared");
        println!("cargo:rustc-link-lib=dylib=leanshared");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib.display());
    }
}
