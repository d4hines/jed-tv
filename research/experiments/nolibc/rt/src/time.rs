//! Clocks and sleeping.
//!
//! On 32-bit ARM std calls the legacy symbols with a 32-bit `time_t` (musl's
//! time32 compatibility ABI). musl 1.2 implements those by trying the
//! *_time64 syscalls first, which the TV's 4.19 kernel lacks (ENOSYS, then a
//! second syscall). Here they map 1:1 onto the 32-bit syscalls instead: one
//! syscall per clock read on the TV. (Wall-clock dates past 2038 would need
//! the time64 path; the TV has no RTC time anyway.)

use crate::sys::ret_int;
use libc::{c_int, clockid_t, timespec};

#[no_mangle]
pub unsafe extern "C" fn clock_gettime(clk: clockid_t, ts: *mut timespec) -> c_int {
    if let Some(f) = crate::vdso::clock_gettime() {
        if f(clk, ts) == 0 {
            return 0;
        }
    }
    ret_int(sc!(libc::SYS_clock_gettime, clk, ts))
}

#[no_mangle]
pub unsafe extern "C" fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int {
    ret_int(sc!(libc::SYS_nanosleep, req, rem))
}

#[no_mangle]
pub unsafe extern "C" fn clock_nanosleep(clk: clockid_t, flags: c_int, req: *const timespec, rem: *mut timespec)
    -> c_int {
    // Returns the error number itself, not -1/errno.
    crate::sys::ret_pthread(sc!(libc::SYS_clock_nanosleep, clk, flags, req, rem))
}
