use std::{env, path::PathBuf, process::Command};
fn main() {
    let mut cc = cc::Build::new();
    cc.file("driver.c");
    if env::var("TARGET").unwrap() == "armv7-unknown-linux-musleabihf" {
        let rt = PathBuf::from(env::var("ZR_RUNTIME_DIR").expect("built Zr runtime directory"));
        cc.define("USE_ZR", None)
            .include(rt.join("gen/include"))
            .include(rt.join("gen/rt"));
        println!("cargo:rustc-link-search=native={}", rt.display());
        println!("cargo:rustc-link-lib=static=zr_native");
        println!(
            "cargo:rerun-if-changed={}",
            rt.join("libzr_native.a").display()
        );
    } else {
        let prefix = Command::new("lean").arg("--print-prefix").output().unwrap();
        let prefix = PathBuf::from(String::from_utf8(prefix.stdout).unwrap().trim());
        cc.include(prefix.join("include"));
        let lib = prefix.join("lib/lean");
        println!("cargo:rustc-link-search=native={}", lib.display());
        println!("cargo:rustc-link-lib=dylib=Init_shared");
        println!("cargo:rustc-link-lib=dylib=leanshared");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib.display());
    }
    cc.warnings(false).compile("extra_test");
    println!("cargo:rerun-if-changed=driver.c");
    println!("cargo:rerun-if-env-changed=ZR_RUNTIME_DIR");
}
