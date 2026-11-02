//! strerror_r: musl's messages for the errnos our programs can plausibly see.

use libc::{c_char, c_int, size_t};

fn message(e: c_int) -> &'static [u8] {
    match e {
        0 => b"No error information",
        libc::EPERM => b"Operation not permitted",
        libc::ENOENT => b"No such file or directory",
        libc::ESRCH => b"No such process",
        libc::EINTR => b"Interrupted system call",
        libc::EIO => b"I/O error",
        libc::ENXIO => b"No such device or address",
        libc::E2BIG => b"Argument list too long",
        libc::ENOEXEC => b"Exec format error",
        libc::EBADF => b"Bad file descriptor",
        libc::ECHILD => b"No child process",
        libc::EAGAIN => b"Resource temporarily unavailable",
        libc::ENOMEM => b"Out of memory",
        libc::EACCES => b"Permission denied",
        libc::EFAULT => b"Bad address",
        libc::EBUSY => b"Resource busy",
        libc::EEXIST => b"File exists",
        libc::ENODEV => b"No such device",
        libc::ENOTDIR => b"Not a directory",
        libc::EISDIR => b"Is a directory",
        libc::EINVAL => b"Invalid argument",
        libc::EMFILE => b"No file descriptors available",
        libc::ENOTTY => b"Not a tty",
        libc::EFBIG => b"File too large",
        libc::ENOSPC => b"No space left on device",
        libc::ESPIPE => b"Invalid seek",
        libc::EROFS => b"Read-only file system",
        libc::EPIPE => b"Broken pipe",
        libc::ERANGE => b"Result not representable",
        libc::EDEADLK => b"Resource deadlock would occur",
        libc::ENAMETOOLONG => b"Filename too long",
        libc::ENOSYS => b"Function not implemented",
        libc::ENOTEMPTY => b"Directory not empty",
        libc::ENOTSOCK => b"Not a socket",
        libc::EADDRINUSE => b"Address in use",
        libc::EADDRNOTAVAIL => b"Address not available",
        libc::ENETDOWN => b"Network is down",
        libc::ENETUNREACH => b"Network unreachable",
        libc::ECONNABORTED => b"Connection aborted",
        libc::ECONNRESET => b"Connection reset by peer",
        libc::ENOTCONN => b"Socket not connected",
        libc::ETIMEDOUT => b"Operation timed out",
        libc::ECONNREFUSED => b"Connection refused",
        libc::EHOSTUNREACH => b"Host is unreachable",
        libc::EALREADY => b"Operation already in progress",
        libc::EINPROGRESS => b"Operation in progress",
        libc::EOPNOTSUPP => b"Not supported",
        libc::EAFNOSUPPORT => b"Address family not supported by protocol",
        libc::EPROTONOSUPPORT => b"Protocol not supported",
        libc::ENOBUFS => b"No buffer space available",
        libc::EMSGSIZE => b"Message too large",
        _ => b"Unknown error",
    }
}

/// POSIX strerror_r (musl exports it under this name too).
#[no_mangle]
pub unsafe extern "C" fn __xpg_strerror_r(e: c_int, buf: *mut c_char, n: size_t) -> c_int {
    let m = message(e);
    if n == 0 {
        return libc::ERANGE;
    }
    let len = m.len().min(n - 1);
    for (i, &b) in m[..len].iter().enumerate() {
        *buf.add(i) = b as c_char;
    }
    *buf.add(len) = 0;
    if len < m.len() { libc::ERANGE } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn strerror_r(e: c_int, buf: *mut c_char, n: size_t) -> c_int {
    __xpg_strerror_r(e, buf, n)
}
