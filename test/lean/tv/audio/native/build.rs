use std::{env, path::PathBuf, process::Command};
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../../../../..")
        .canonicalize()
        .unwrap();
    assert!(Command::new("lake")
        .args(["-d", "src/pid1/lean", "build", "TV.Audio.Cooperative"])
        .current_dir(&root)
        .status()
        .unwrap()
        .success());
    let output = Command::new("lean").arg("--print-prefix").output().unwrap();
    let prefix = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
    let target = env::var("TARGET").unwrap();
    let cross = target == "armv7-unknown-linux-musleabihf";
    println!("cargo:rerun-if-env-changed=ZR_RUNTIME_DIR");
    let rt = if cross {
        Some(PathBuf::from(env::var("ZR_RUNTIME_DIR").expect(
            "ZR_RUNTIME_DIR must contain libzr_native.a and zr/gen",
        )))
    } else {
        None
    };
    let mut build = cc::Build::new();
    if let Some(rt) = &rt {
        println!(
            "cargo:rerun-if-changed={}",
            rt.join("libzr_native.a").display()
        );
        println!(
            "cargo:rerun-if-changed={}",
            rt.join("zr/gen/include").display()
        );
        println!("cargo:rerun-if-changed={}", rt.join("zr/gen/rt").display());
        build
            .include(rt.join("zr/gen/include"))
            .include(rt.join("zr/gen/rt"))
            .flag("-ffreestanding")
            .flag("-fno-builtin")
            .flag("-nostdinc")
            .flag("-include")
            .flag("lean/zr_fastpath.h");
    } else {
        build.include(prefix.join("include"));
    }
    for module in ["Protocol", "Converter", "Device", "Cooperative"] {
        build.file(root.join(format!("assets/build/lake/tv/ir/TV/Audio/{module}.c")));
        println!(
            "cargo:rerun-if-changed={}",
            root.join(format!("src/pid1/lean/TV/Audio/{module}.lean"))
                .display()
        );
    }
    build
        .define("NDEBUG", None)
        .warnings(false)
        .compile("audiolean");
    if let Some(rt) = rt {
        println!("cargo:rustc-link-search=native={}", rt.display());
        println!("cargo:rustc-link-lib=static=zr_native");
        return;
    }
    let lib = prefix.join("lib/lean");
    println!("cargo:rustc-link-search=native={}", lib.display());
    println!("cargo:rustc-link-lib=dylib=Init_shared");
    println!("cargo:rustc-link-lib=dylib=leanshared");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib.display());
}
