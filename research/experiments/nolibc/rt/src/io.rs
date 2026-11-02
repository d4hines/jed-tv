//! Files, directories, memory mappings and other thin syscall wrappers.

use crate::sys::{ret, ret_int, set_errno};
use libc::{c_char, c_int, c_long, c_uint, c_ulong, c_void, mode_t, off_t, size_t, ssize_t};

// The kernel fills these structs directly, so their layouts must match:
// musl's (time32) `struct stat` on ARM is the kernel's `struct stat64`, and
// musl's `struct dirent` is `linux_dirent64`.
#[cfg(target_arch = "arm")]
const _: () = assert!(core::mem::size_of::<libc::stat>() == 104);
#[cfg(target_arch = "aarch64")]
const _: () = assert!(core::mem::size_of::<libc::stat>() == 128);
const _: () = assert!(core::mem::offset_of!(libc::dirent, d_name) == 19);

// ---------- memory ----------

/// mmap without errno (usable before TLS exists): negative errno on failure.
pub unsafe fn mmap_raw(addr: usize, len: usize, prot: c_int, flags: c_int, fd: c_int, off: off_t) -> isize {
    #[cfg(target_arch = "aarch64")]
    {
        sc!(libc::SYS_mmap, addr, len, prot, flags, fd, off)
    }
    #[cfg(target_arch = "arm")]
    {
        if off & 4095 != 0 {
            return -(libc::EINVAL as isize);
        }
        sc!(libc::SYS_mmap2, addr, len, prot, flags, fd, (off as u64 >> 12) as usize)
    }
}

#[no_mangle]
pub unsafe extern "C" fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int,
                              off: off_t) -> *mut c_void {
    let r = mmap_raw(addr as usize, len, prot, flags, fd, off);
    if (-4095..0).contains(&r) {
        set_errno(-r as c_int);
        return libc::MAP_FAILED;
    }
    r as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn munmap(addr: *mut c_void, len: size_t) -> c_int {
    ret_int(sc!(libc::SYS_munmap, addr, len))
}

#[no_mangle]
pub unsafe extern "C" fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int {
    ret_int(sc!(libc::SYS_mprotect, addr, len, prot))
}

#[no_mangle]
pub unsafe extern "C" fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int {
    ret_int(sc!(libc::SYS_madvise, addr, len, advice))
}

// ---------- file descriptors ----------

#[no_mangle]
pub unsafe extern "C" fn read(fd: c_int, buf: *mut c_void, n: size_t) -> ssize_t {
    ret(sc!(libc::SYS_read, fd, buf, n)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn write(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t {
    ret(sc!(libc::SYS_write, fd, buf, n)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn writev(fd: c_int, iov: *const libc::iovec, n: c_int) -> ssize_t {
    ret(sc!(libc::SYS_writev, fd, iov, n)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn readv(fd: c_int, iov: *const libc::iovec, n: c_int) -> ssize_t {
    ret(sc!(libc::SYS_readv, fd, iov, n)) as ssize_t
}

/// open(path, flags, mode): the variadic mode is read as a fixed third
/// argument (same registers on AAPCS32/AAPCS64).
#[no_mangle]
pub unsafe extern "C" fn open(path: *const c_char, flags: c_int, mode: c_uint) -> c_int {
    #[cfg(target_arch = "aarch64")]
    let r = sc!(libc::SYS_openat, libc::AT_FDCWD, path, flags, mode);
    // 32-bit: always ask for large-file semantics, as musl does.
    #[cfg(target_arch = "arm")]
    let r = sc!(libc::SYS_open, path, flags | libc::O_LARGEFILE, mode);
    ret_int(r)
}

#[no_mangle]
pub unsafe extern "C" fn openat(dirfd: c_int, path: *const c_char, flags: c_int, mode: c_uint) -> c_int {
    #[cfg(target_arch = "arm")]
    let flags = flags | libc::O_LARGEFILE;
    ret_int(sc!(libc::SYS_openat, dirfd, path, flags, mode))
}

#[no_mangle]
pub unsafe extern "C" fn close(fd: c_int) -> c_int {
    let r = sc!(libc::SYS_close, fd);
    // Linux closes the descriptor even when interrupted.
    if r == -(libc::EINTR as isize) {
        return 0;
    }
    ret_int(r)
}

#[no_mangle]
pub unsafe extern "C" fn dup(fd: c_int) -> c_int {
    ret_int(sc!(libc::SYS_dup, fd))
}

#[no_mangle]
pub unsafe extern "C" fn fcntl(fd: c_int, cmd: c_int, arg: c_ulong) -> c_int {
    #[cfg(target_arch = "aarch64")]
    let r = sc!(libc::SYS_fcntl, fd, cmd, arg);
    #[cfg(target_arch = "arm")]
    let r = sc!(libc::SYS_fcntl64, fd, cmd, arg);
    ret_int(r)
}

#[no_mangle]
pub unsafe extern "C" fn ioctl(fd: c_int, req: c_int, arg: c_ulong) -> c_int {
    ret_int(sc!(libc::SYS_ioctl, fd, req as c_uint, arg))
}

#[no_mangle]
pub unsafe extern "C" fn lseek(fd: c_int, off: off_t, whence: c_int) -> off_t {
    #[cfg(target_arch = "aarch64")]
    {
        ret(sc!(libc::SYS_lseek, fd, off, whence)) as off_t
    }
    #[cfg(target_arch = "arm")]
    {
        let mut result: i64 = 0;
        let r = sc!(libc::SYS__llseek, fd, (off as u64 >> 32) as usize, off as u64 as u32 as usize,
                    &mut result as *mut i64, whence);
        if ret(r) < 0 { -1 } else { result }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fstat(fd: c_int, st: *mut libc::stat) -> c_int {
    // musl's (time32) struct stat on ARM is the kernel's struct stat64.
    #[cfg(target_arch = "aarch64")]
    let r = sc!(libc::SYS_fstat, fd, st);
    #[cfg(target_arch = "arm")]
    let r = sc!(libc::SYS_fstat64, fd, st);
    ret_int(r)
}

#[no_mangle]
pub unsafe extern "C" fn fstatat(dirfd: c_int, path: *const c_char, st: *mut libc::stat, flags: c_int) -> c_int {
    #[cfg(target_arch = "aarch64")]
    let r = sc!(libc::SYS_newfstatat, dirfd, path, st, flags);
    #[cfg(target_arch = "arm")]
    let r = sc!(libc::SYS_fstatat64, dirfd, path, st, flags);
    ret_int(r)
}

#[no_mangle]
pub unsafe extern "C" fn stat(path: *const c_char, st: *mut libc::stat) -> c_int {
    fstatat(libc::AT_FDCWD, path, st, 0)
}

#[no_mangle]
pub unsafe extern "C" fn lstat(path: *const c_char, st: *mut libc::stat) -> c_int {
    fstatat(libc::AT_FDCWD, path, st, libc::AT_SYMLINK_NOFOLLOW)
}

#[no_mangle]
pub unsafe extern "C" fn poll(fds: *mut libc::pollfd, n: libc::nfds_t, timeout_ms: c_int) -> c_int {
    #[cfg(target_arch = "arm")]
    {
        ret_int(sc!(libc::SYS_poll, fds, n, timeout_ms))
    }
    #[cfg(target_arch = "aarch64")]
    {
        let ts = libc::timespec { tv_sec: (timeout_ms / 1000) as _, tv_nsec: ((timeout_ms % 1000) * 1_000_000) as _ };
        let tsp = if timeout_ms >= 0 { &ts as *const libc::timespec } else { core::ptr::null() };
        ret_int(sc!(libc::SYS_ppoll, fds, n, tsp, 0usize, 8usize))
    }
}

#[no_mangle]
pub unsafe extern "C" fn readlink(path: *const c_char, buf: *mut c_char, n: size_t) -> ssize_t {
    ret(sc!(libc::SYS_readlinkat, libc::AT_FDCWD, path, buf, n)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn getcwd(buf: *mut c_char, n: size_t) -> *mut c_char {
    let r = sc!(libc::SYS_getcwd, buf, n);
    if ret(r) < 0 {
        return core::ptr::null_mut();
    }
    buf
}

/// realpath via the kernel: open with O_PATH, then read /proc/self/fd/N.
#[no_mangle]
pub unsafe extern "C" fn realpath(path: *const c_char, resolved: *mut c_char) -> *mut c_char {
    let fd = openat(libc::AT_FDCWD, path, libc::O_PATH | libc::O_CLOEXEC, 0);
    if fd < 0 {
        return core::ptr::null_mut();
    }
    let mut link = *b"/proc/self/fd/\0\0\0\0\0\0\0\0\0\0\0\0";
    let mut digits = [0u8; 12];
    let (mut n, mut v) = (0, fd as u32);
    loop {
        digits[n] = b'0' + (v % 10) as u8;
        n += 1;
        v /= 10;
        if v == 0 {
            break;
        }
    }
    for i in 0..n {
        link[14 + i] = digits[n - 1 - i];
    }
    let mut buf = [0u8; libc::PATH_MAX as usize];
    let r = readlink(link.as_ptr() as *const c_char, buf.as_mut_ptr() as *mut c_char, buf.len() - 1);
    close(fd);
    if r < 0 {
        return core::ptr::null_mut();
    }
    let len = r as usize;
    let out = if resolved.is_null() { crate::malloc::malloc(len + 1) as *mut c_char } else { resolved };
    if out.is_null() {
        set_errno(libc::ENOMEM);
        return out;
    }
    for i in 0..len {
        *out.add(i) = buf[i] as c_char;
    }
    *out.add(len) = 0;
    out
}

#[no_mangle]
pub unsafe extern "C" fn mkdir(path: *const c_char, mode: mode_t) -> c_int {
    ret_int(sc!(libc::SYS_mkdirat, libc::AT_FDCWD, path, mode))
}

#[no_mangle]
pub unsafe extern "C" fn unlink(path: *const c_char) -> c_int {
    ret_int(sc!(libc::SYS_unlinkat, libc::AT_FDCWD, path, 0))
}

#[no_mangle]
pub unsafe extern "C" fn mount(src: *const c_char, target: *const c_char, fstype: *const c_char,
                               flags: c_ulong, data: *const c_void) -> c_int {
    ret_int(sc!(libc::SYS_mount, src, target, fstype, flags, data))
}

#[no_mangle]
pub unsafe extern "C" fn sync() {
    sc!(libc::SYS_sync);
}

#[no_mangle]
pub unsafe extern "C" fn reboot(cmd: c_int) -> c_int {
    ret_int(sc!(libc::SYS_reboot, 0xfee1deadu32, 672274793, cmd, 0))
}

#[no_mangle]
pub unsafe extern "C" fn uname(buf: *mut libc::utsname) -> c_int {
    ret_int(sc!(libc::SYS_uname, buf))
}

#[no_mangle]
pub unsafe extern "C" fn getrandom(buf: *mut c_void, n: size_t, flags: c_uint) -> ssize_t {
    ret(sc!(libc::SYS_getrandom, buf, n, flags)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn prctl(op: c_int, a: c_ulong, b: c_ulong, c: c_ulong, d: c_ulong) -> c_int {
    ret_int(sc!(libc::SYS_prctl, op, a, b, c, d))
}

/// syscall(n, ...): up to six integer arguments, read as fixed arguments.
#[no_mangle]
pub unsafe extern "C" fn syscall(n: c_long, a: c_long, b: c_long, c: c_long, d: c_long, e: c_long,
                                 f: c_long) -> c_long {
    ret(crate::sys::raw(n, a as usize, b as usize, c as usize, d as usize, e as usize, f as usize))
}

#[no_mangle]
pub unsafe extern "C" fn sched_getaffinity(pid: libc::pid_t, size: size_t, set: *mut libc::cpu_set_t) -> c_int {
    let r = sc!(libc::SYS_sched_getaffinity, pid, size, set);
    if ret(r) < 0 {
        return -1;
    }
    // The kernel fills only its own mask size; clear the rest as musl does.
    let p = set as *mut u8;
    for i in r as usize..size {
        *p.add(i) = 0;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sched_yield() -> c_int {
    ret_int(sc!(libc::SYS_sched_yield))
}

// ---------- directories ----------

const DIR_BUF: usize = 2048;

#[repr(C)]
pub struct Dir {
    fd: c_int,
    pos: usize,
    end: usize,
    buf: [u8; DIR_BUF],
}

#[no_mangle]
pub unsafe extern "C" fn opendir(path: *const c_char) -> *mut Dir {
    let fd = open(path, libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC, 0);
    if fd < 0 {
        return core::ptr::null_mut();
    }
    let d = crate::malloc::calloc(1, core::mem::size_of::<Dir>()) as *mut Dir;
    if d.is_null() {
        close(fd);
        set_errno(libc::ENOMEM);
        return d;
    }
    (*d).fd = fd;
    d
}

#[no_mangle]
pub unsafe extern "C" fn fdopendir(fd: c_int) -> *mut Dir {
    let d = crate::malloc::calloc(1, core::mem::size_of::<Dir>()) as *mut Dir;
    if d.is_null() {
        set_errno(libc::ENOMEM);
        return d;
    }
    (*d).fd = fd;
    d
}

/// musl's `struct dirent` is the kernel's linux_dirent64, so entries are
/// returned straight from the getdents64 buffer.
#[no_mangle]
pub unsafe extern "C" fn readdir(d: *mut Dir) -> *mut libc::dirent {
    if (*d).pos >= (*d).end {
        let r = sc!(libc::SYS_getdents64, (*d).fd, (*d).buf.as_mut_ptr(), DIR_BUF);
        if r <= 0 {
            if r < 0 && r != -(libc::ENOENT as isize) {
                set_errno(-r as c_int);
            }
            return core::ptr::null_mut();
        }
        (*d).pos = 0;
        (*d).end = r as usize;
    }
    let ent = (*d).buf.as_mut_ptr().add((*d).pos) as *mut libc::dirent;
    (*d).pos += (*ent).d_reclen as usize;
    ent
}

#[no_mangle]
pub unsafe extern "C" fn closedir(d: *mut Dir) -> c_int {
    let r = close((*d).fd);
    crate::malloc::free(d as *mut c_void);
    r
}

#[no_mangle]
pub unsafe extern "C" fn dirfd(d: *mut Dir) -> c_int {
    (*d).fd
}
