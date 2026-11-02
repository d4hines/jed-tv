//! c-scape as a staticlib: all of its #[no_mangle] libc symbols (plus origin's
//! program start) in one archive, to be used as `libc.a`.
#![no_std]
extern crate c_scape;
