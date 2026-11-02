//! Compile the scalar Lean compositor without linking a Lean runtime.
use std::{
    env,
    path::PathBuf,
    process::{Command, Output},
};
fn run(command: &mut Command) -> Output {
    let output = command.output().expect("run display build tool");
    assert!(
        output.status.success(),
        "display build failed: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../../..")
        .canonicalize()
        .unwrap();
    for path in [
        "src/pid1/lean/TV/Display.lean",
        "src/pid1/lean/TV/Display/Scanout.lean",
        "src/lean/zr/ZrAbi/DisplayMemory.lean",
        "tools/lean/zr/ZrDisplayHeader.lean",
        "tools/display/build.rs",
    ] {
        println!("cargo:rerun-if-changed={}", root.join(path).display());
    }
    let output = PathBuf::from(env::var("OUT_DIR").unwrap());
    run(Command::new("lake")
        .args(["build", "TV.Display", "TV.Display.Scanout"])
        .current_dir(root.join("src/pid1/lean")));
    run(Command::new("lake")
        .args(["exe", "display-header"])
        .arg(output.join("display-memory.h"))
        .current_dir(root.join("tools/lean/zr")));
    let prefix = run(Command::new("lean")
        .arg("--print-prefix")
        .current_dir(root.join("src/pid1/lean")));
    let prefix = PathBuf::from(String::from_utf8(prefix.stdout).unwrap().trim());
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let compiler = env::var(format!("CC_{}", target.replace('-', "_")))
        .ok()
        .or_else(|| (target == host).then(|| env::var("CC").ok()).flatten());
    let mut objects = Vec::new();
    for (module, signatures) in [
        ("Display", &["LEAN_EXPORT uint32_t tv_display_compose(size_t, uint32_t, uint32_t, uint32_t, uint32_t, size_t, uint32_t, uint32_t, uint32_t, size_t, uint32_t);"][..]),
        ("Display/Scanout", &[
            "LEAN_EXPORT uint32_t tv_scanout_present(size_t, uint32_t, uint32_t, uint32_t, uint32_t);",
            "LEAN_EXPORT uint32_t tv_scanout_hide(size_t, uint32_t, uint32_t);",
            "LEAN_EXPORT uint32_t tv_scanout_configure_alpha(size_t, uint32_t, uint32_t);"][..]),
    ] {
        let generated = root.join(format!("assets/build/lake/tv/ir/TV/{module}.c"));
        let generated_text = std::fs::read_to_string(&generated).expect("generated display C");
        for signature in signatures {
            assert!(generated_text.contains(signature), "Lean changed the scalar display ABI; review before linking it to Rust");
        }
        let mut cc = if let Some(compiler) = &compiler {
            Command::new(compiler)
        } else if target == host {
            Command::new("cc")
        } else {
            let mut cc = Command::new("zig");
            cc.arg("cc").arg("-target").arg(match target.as_str() {
                "armv7-unknown-linux-musleabihf" => "arm-linux-musleabihf",
                // This leaf archive has only integer/pointer ABI and memcpy;
                // use soft-float EABI objects for the Android standalone host.
                "armv7-linux-androideabi" => "arm-linux-musleabi",
                "aarch64-unknown-linux-musl" => "aarch64-linux-musl",
                _ => panic!("no display cross compiler configured for {target}"),
            });
            cc
        };
        let object = output.join(format!("{}.o", module.replace('/', "-")));
        cc.args([
        "-O2",
        "-DNDEBUG",
        "-ffunction-sections",
        "-fdata-sections",
        "-fno-unwind-tables",
        "-fno-asynchronous-unwind-tables",
    ])
    .arg("-I")
    .arg(prefix.join("include"))
    .arg("-include")
    .arg(output.join("display-memory.h"))
    .arg("-c")
    .arg(generated)
    .arg("-o")
    .arg(&object);
        run(&mut cc);
        objects.push(object);
    }
    let archive = output.join("libtv_display_lean.a");
    if archive.exists() {
        std::fs::remove_file(&archive).expect("replace display archive");
    }
    run(Command::new("zig")
        .args(["ar", "rcs"])
        .arg(archive)
        .args(objects));
    println!("cargo:rustc-link-search=native={}", output.display());
    println!("cargo:rustc-link-lib=static=tv_display_lean");
}
