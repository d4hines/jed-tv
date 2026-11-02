//! nolibc-rt: a C library for our static Linux programs, written in Rust
//! (plus a page of assembly per architecture: program entry, thread clone,
//! signal return and the ARM TLS helper).
//!
//! Scope: exactly the C symbols that Rust's `std` (musl flavour, as shipped by
//! rustup for `*-linux-musl`) and our crates import, as measured by
//! `nolibc/tools/probe_undefined.py`. Built as a staticlib and installed as
//! `libc.a`, so std's own `#[link(name = "c")]` resolves here instead of musl.
//! Struct layouts and constants come from the `libc` crate compiled for the
//! same musl target, i.e. the ABI std itself was compiled against (32-bit ARM
//! keeps musl's legacy 32-bit `time_t` symbols, as std calls them).
//!
//! Everything is a direct Linux syscall; no file is read at startup. The
//! syscalls used per function are chosen to stay inside the
//! `platform::seccomp` allowlist (research/syscall-census-20261031.md).
//!
//! Stable Rust only: no `#![feature]`, no `-Zbuild-std`. C variadic functions
//! (`syscall`, `ioctl`, `open`, `fcntl`, `prctl`) are defined with their
//! maximum fixed argument list, which is call-compatible on AAPCS32/AAPCS64
//! for integer and pointer arguments.

#![no_std]
// The memory functions below must not be turned back into calls to themselves.
#![no_builtins]
#![allow(non_camel_case_types, clippy::missing_safety_doc)]

#[macro_use]
pub mod sys;
pub mod arch;

mod errstr;
mod io;
mod malloc;
mod math;
mod mem;
mod net;
mod process;
mod start;
mod sync;
mod thread;
mod time;
mod unwind;
mod vdso;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    // Only reachable through a bug in this crate; keep it small.
    sys::write_all(2, b"nolibc-rt: internal panic\n");
    process::abort()
}
