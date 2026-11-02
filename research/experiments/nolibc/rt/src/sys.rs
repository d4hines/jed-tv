//! Syscall helpers and errno.

use crate::arch;
use libc::{c_int, c_long};

/// Raw syscall: returns the kernel's value (negative errno on failure).
#[inline(always)]
pub unsafe fn raw(n: c_long, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> isize {
    arch::syscall6(n as usize, a, b, c, d, e, f)
}

/// `sc!(SYS_x, args...)`: raw syscall with up to 6 arguments, each cast to usize.
#[macro_export]
macro_rules! sc {
    ($n:expr) => { $crate::sys::raw($n as libc::c_long, 0, 0, 0, 0, 0, 0) };
    ($n:expr, $a:expr) => { $crate::sys::raw($n as libc::c_long, $a as usize, 0, 0, 0, 0, 0) };
    ($n:expr, $a:expr, $b:expr) => { $crate::sys::raw($n as libc::c_long, $a as usize, $b as usize, 0, 0, 0, 0) };
    ($n:expr, $a:expr, $b:expr, $c:expr) => {
        $crate::sys::raw($n as libc::c_long, $a as usize, $b as usize, $c as usize, 0, 0, 0)
    };
    ($n:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::sys::raw($n as libc::c_long, $a as usize, $b as usize, $c as usize, $d as usize, 0, 0)
    };
    ($n:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        $crate::sys::raw($n as libc::c_long, $a as usize, $b as usize, $c as usize, $d as usize, $e as usize, 0)
    };
    ($n:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        $crate::sys::raw($n as libc::c_long, $a as usize, $b as usize, $c as usize, $d as usize, $e as usize,
                         $f as usize)
    };
}

/// Kernel error returns are -4095..-1. Don't test `r < 0`: on 32-bit ARM,
/// mmap addresses above 2 GiB are negative as isize.
#[inline(always)]
pub fn is_err(r: isize) -> bool {
    (-4095..0).contains(&r)
}

pub fn set_errno(e: c_int) {
    unsafe { *crate::thread::__errno_location() = e };
}

/// The C convention: -1 and errno on failure, else the value.
#[inline]
pub fn ret(r: isize) -> c_long {
    if (-4095..0).contains(&r) {
        set_errno(-r as c_int);
        -1
    } else {
        r as c_long
    }
}

#[inline]
pub fn ret_int(r: isize) -> c_int {
    ret(r) as c_int
}

/// For pthread-style functions: 0 or a positive errno, errno untouched.
#[inline]
pub fn ret_pthread(r: isize) -> c_int {
    if r < 0 { -r as c_int } else { 0 }
}

/// Write everything to `fd`, ignoring errors (for diagnostics).
pub fn write_all(fd: c_int, mut buf: &[u8]) {
    while !buf.is_empty() {
        let r = unsafe { sc!(libc::SYS_write, fd, buf.as_ptr(), buf.len()) };
        if r <= 0 {
            if r == -(libc::EINTR as isize) {
                continue;
            }
            return;
        }
        buf = &buf[r as usize..];
    }
}
