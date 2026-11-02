use std::{env, path::PathBuf, process::Command};
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../..")
        .canonicalize()
        .unwrap();
    assert!(Command::new("lake")
        .args(["-d", "src/pid1/lean", "build", "TV.Hardware.ColdPlan"])
        .current_dir(&root)
        .status()
        .unwrap()
        .success());
    let mut cc = cc::Build::new();
    for module in ["Apps", "SettingsFile", "Hardware/ColdPlan"] {
        cc.file(root.join(format!("assets/build/lake/tv/ir/TV/{module}.c")));
        println!(
            "cargo:rerun-if-changed={}",
            root.join(format!("src/pid1/lean/TV/{module}.lean"))
                .display()
        );
    }
    cc.flag("-ffunction-sections")
        .flag("-fdata-sections")
        .define("NDEBUG", None)
        .warnings(false);
    if env::var("TARGET").unwrap() == "armv7-unknown-linux-musleabihf" {
        let runtime = PathBuf::from(
            env::var("COLD_ZR_RUNTIME_DIR").expect("ARM oracle requires built owned runtime"),
        );
        cc.include(runtime.join("gen/include"))
            .include(runtime.join("gen/rt"))
            .flag("-ffreestanding")
            .flag("-fno-builtin")
            .flag("-nostdinc")
            .flag("-include")
            .flag("lean/zr_fastpath.h")
            .compile("cold_plan");
        println!("cargo:rustc-link-search=native={}", runtime.display());
        println!("cargo:rustc-link-lib=static=zr_native");
    } else {
        let output = Command::new("lean").arg("--print-prefix").output().unwrap();
        let prefix = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
        cc.include(prefix.join("include")).compile("cold_plan");
        println!(
            "cargo:rustc-link-search=native={}",
            prefix.join("lib/lean").display()
        );
        println!("cargo:rustc-link-lib=dylib=Init_shared");
        println!("cargo:rustc-link-lib=dylib=leanshared");
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            prefix.join("lib/lean").display()
        );
    }
    println!("cargo:rerun-if-env-changed=COLD_ZR_RUNTIME_DIR");
    println!(
        "cargo:rerun-if-changed={}",
        root.join("src/pid1/src/cold_plan.rs").display()
    );
}
