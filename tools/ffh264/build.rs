//! Assemble FFmpeg's (unmodified) NEON/ARMv6 assembly plus our varargs
//! trampolines into libffh264asm.a, with the target's C compiler (zig cc, from
//! src/.cargo/config.toml) and the same flags FFmpeg's own build uses.

fn main() {
    println!("cargo:rerun-if-changed=asm");
    let target = std::env::var("TARGET").unwrap();
    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "arm" => "arm",
        "aarch64" => "aarch64",
        _ => return,
    };
    if !target.contains("linux") {
        return;
    }
    let dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("asm");
    let inc = dir.join(arch);
    let mut b = cc::Build::new();
    b.no_default_flags(true).include(&inc).flag("-Qunused-arguments");
    for d in ["_ISOC11_SOURCE", "_LARGEFILE_SOURCE"] {
        b.define(d, None);
    }
    b.define("_FILE_OFFSET_BITS", "64").define("_POSIX_C_SOURCE", "200112").define("_XOPEN_SOURCE", "600");
    for s in std::fs::read_to_string(inc.join("SOURCES")).unwrap().lines() {
        b.file(inc.join(s));
    }
    b.file(dir.join(format!("varargs-{arch}.S")));
    b.compile("ffh264asm");
}
