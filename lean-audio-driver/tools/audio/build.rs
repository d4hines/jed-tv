//! With the `lean` feature: compile the Lean audio driver (src/lean/TV/Audio)
//! and its byte-array marshalling into one archive that every binary using the
//! driver links: PID1 on the G06 and the standalone direct-audio process.
//! tvinit's build script supplies the matching Lean runtime.
use std::{env, path::PathBuf, process::Command};

const MODULES: [&str; 6] = ["Fault", "Gain", "Resample", "Protocol", "Session", "Native"];

fn run(command: &mut Command) {
    assert!(command.status().expect("Lean 4.35.0 is required").success(), "Lean audio build failed");
}

fn main() {
    let lean = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../lean");
    for module in MODULES {
        println!("cargo:rerun-if-changed={}", lean.join(format!("TV/Audio/{module}.lean")).display());
    }
    println!("cargo:rerun-if-changed={}", lean.join("native/audio.c").display());
    println!("cargo:rerun-if-env-changed=LEAN_ARM");
    if env::var_os("CARGO_FEATURE_LEAN").is_none() {
        return;
    }
    run(Command::new("lake").args(["build", "TV.Audio.Native"]).current_dir(&lean));
    let target = env::var("TARGET").unwrap();
    let cross = target.ends_with("linux-musleabihf") || target == "aarch64-unknown-linux-musl";
    let include = if cross {
        let rt = env::var_os("LEAN_ARM").map(PathBuf::from)
            .unwrap_or_else(|| lean.join("../../assets/build/lean-arm").join(&target));
        assert!(rt.join("libleanrt.a").exists(), "build tools/lean-arm/build.sh {target} first, or set LEAN_ARM");
        rt.join("include")
    } else {
        let output = Command::new("lean").arg("--print-prefix").current_dir(&lean).output().expect("Lean prefix");
        assert!(output.status.success());
        PathBuf::from(String::from_utf8(output.stdout).unwrap().trim()).join("include")
    };
    let generated = lean.join("../../assets/build/lake/tv/ir/TV/Audio");
    cc::Build::new()
        .files(MODULES.iter().map(|module| generated.join(format!("{module}.c"))))
        .file(lean.join("native/audio.c"))
        .include(include)
        .define("NDEBUG", None)
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .warnings(false)
        .compile("mi_audio_lean");
}
