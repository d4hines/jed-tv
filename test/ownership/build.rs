use std::{env, path::PathBuf, process::Command};
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../..")
        .canonicalize()
        .unwrap();
    assert!(Command::new("lake")
        .args(["-d", "src/pid1/lean", "build", "TV.Hardware.Ownership"])
        .current_dir(&root)
        .status()
        .unwrap()
        .success());
    let prefix = Command::new("lean").arg("--print-prefix").output().unwrap();
    let prefix = PathBuf::from(String::from_utf8(prefix.stdout).unwrap().trim());
    cc::Build::new()
        .file(root.join("assets/build/lake/tv/ir/TV/Hardware/Ownership.c"))
        .include(prefix.join("include"))
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .define("NDEBUG", None)
        .warnings(false)
        .compile("ownership");
    println!(
        "cargo:rerun-if-changed={}",
        root.join("src/pid1/lean/TV/Hardware/Ownership.lean")
            .display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        root.join("src/pid1/src/ownership.rs").display()
    );
    // No Lean runtime is linked: only the scalar entry points are reachable.
}
