//! FFmpeg 9.0.1's H.264 decoder (plus the libavcodec/libavutil code it pulls
//! in, and the ytdec shim from src/ffh264/ytdec.c), translated to unsafe Rust
//! by c2rust. FFmpeg's NEON/ARMv6 assembly is kept as assembly (asm/) and
//! called through the same C-ABI function tables as in C.
//!
//! One translation per target (`src/arm`, `src/aarch64`), because FFmpeg is
//! preprocessed per target (config.h, pointer size). How it is produced:
//! tools/ffh264/regenerate.sh (including mandatory shrink.py specialization).
//! The supported format is 8-bit YUV 4:2:0; other SPS formats are rejected
//! before DSP initialization. High-depth generated templates are removed.
//! License: LGPL-2.1-or-later, like FFmpeg; every
//! translated file carries its original notice.
//!
//! The decoder API is ytplay's existing C API: `ytdec_open`, `ytdec_send`,
//! `ytdec_receive`, `ytdec_close` (exported unmangled, see `tr::ytdec`).

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_parens,
    unused_must_use,
    unused_unsafe,
    unused_imports,
    unused_labels,
    unreachable_code,
    unreachable_patterns,
    path_statements,
    improper_ctypes,
    improper_ctypes_definitions,
    clashing_extern_declarations,
    static_mut_refs,
    invalid_value,
    clippy::all
)]

#[cfg(all(target_os = "linux", any(target_arch = "arm", target_arch = "aarch64")))]
pub mod rt;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
#[path = "arm/mod.rs"]
pub mod tr;

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
#[path = "aarch64/mod.rs"]
pub mod tr;

#[cfg(all(target_os = "linux", any(target_arch = "arm", target_arch = "aarch64")))]
pub use tr::ytdec::{ytdec_close, ytdec_open, ytdec_receive, ytdec_send};
