//! Processes and signals.

use crate::sys::{ret_int, set_errno};
use libc::{c_char, c_int, pid_t};

#[no_mangle]
pub extern "C" fn getpid() -> pid_t {
    unsafe { sc!(libc::SYS_getpid) as pid_t }
}

#[no_mangle]
pub extern "C" fn gettid() -> pid_t {
    unsafe { sc!(libc::SYS_gettid) as pid_t }
}

#[no_mangle]
pub extern "C" fn getuid() -> libc::uid_t {
    #[cfg(target_arch = "arm")]
    let r = unsafe { sc!(libc::SYS_getuid32) };
    #[cfg(target_arch = "aarch64")]
    let r = unsafe { sc!(libc::SYS_getuid) };
    r as libc::uid_t
}

#[no_mangle]
pub unsafe extern "C" fn fork() -> pid_t {
    // Hold the allocator lock across the clone, so the child never inherits
    // it mid-update from another thread, then release it on both sides.
    let lock = crate::malloc::lock();
    lock.lock();
    let r = sc!(libc::SYS_clone, libc::SIGCHLD, 0, 0, 0, 0);
    if r == 0 {
        crate::thread::after_fork_child();
    }
    lock.unlock();
    ret_int(r)
}

#[no_mangle]
pub unsafe extern "C" fn execve(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char)
    -> c_int {
    ret_int(sc!(libc::SYS_execve, path, argv, envp))
}

#[no_mangle]
pub unsafe extern "C" fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t {
    ret_int(sc!(libc::SYS_wait4, pid, status, options, 0))
}

#[no_mangle]
pub unsafe extern "C" fn kill(pid: pid_t, sig: c_int) -> c_int {
    ret_int(sc!(libc::SYS_kill, pid, sig))
}

#[no_mangle]
pub extern "C" fn _exit(code: c_int) -> ! {
    loop {
        unsafe { sc!(libc::SYS_exit_group, code) };
    }
}

/// No atexit handlers and no stdio buffers exist, so exit is _exit.
#[no_mangle]
pub extern "C" fn exit(code: c_int) -> ! {
    _exit(code)
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    unsafe {
        // Unblock and raise SIGABRT; if a handler returns, reset it and retry.
        let mut set = [0u32; 2];
        set[0] = 1 << (libc::SIGABRT - 1);
        sc!(libc::SYS_rt_sigprocmask, libc::SIG_UNBLOCK, set.as_ptr(), 0, 8);
        sc!(libc::SYS_tgkill, getpid(), gettid(), libc::SIGABRT);
        let mut ka = KSigaction { handler: libc::SIG_DFL, flags: 0, restorer: 0, mask: 0 };
        sc!(libc::SYS_rt_sigaction, libc::SIGABRT, &mut ka as *mut KSigaction, 0, 8);
        sc!(libc::SYS_tgkill, getpid(), gettid(), libc::SIGABRT);
    }
    _exit(127)
}

#[no_mangle]
pub unsafe extern "C" fn pause() -> c_int {
    #[cfg(target_arch = "arm")]
    let r = sc!(libc::SYS_pause);
    #[cfg(target_arch = "aarch64")]
    let r = sc!(libc::SYS_ppoll, 0, 0, 0, 0, 8);
    ret_int(r)
}

// ---------- signals ----------

/// The kernel's struct sigaction for rt_sigaction (64-signal mask).
#[repr(C)]
struct KSigaction {
    handler: usize,
    flags: libc::c_ulong,
    restorer: usize,
    mask: u64,
}

const SA_RESTORER: libc::c_ulong = 0x0400_0000;

#[no_mangle]
pub unsafe extern "C" fn sigaction(sig: c_int, act: *const libc::sigaction, old: *mut libc::sigaction) -> c_int {
    let mut k = KSigaction { handler: 0, flags: 0, restorer: 0, mask: 0 };
    let kp = if act.is_null() {
        core::ptr::null()
    } else {
        let a = &*act;
        k.handler = a.sa_sigaction;
        k.flags = a.sa_flags as libc::c_ulong | SA_RESTORER;
        #[cfg(target_arch = "arm")]
        {
            k.restorer = if a.sa_flags & libc::SA_SIGINFO != 0 {
                crate::arch::__nolibc_restore_rt as usize
            } else {
                crate::arch::__nolibc_restore as usize
            };
        }
        #[cfg(target_arch = "aarch64")]
        {
            k.restorer = crate::arch::__nolibc_restore_rt as usize;
        }
        k.mask = *(&a.sa_mask as *const libc::sigset_t as *const u64);
        &k as *const KSigaction
    };
    let mut ko = KSigaction { handler: 0, flags: 0, restorer: 0, mask: 0 };
    let r = sc!(libc::SYS_rt_sigaction, sig, kp, &mut ko as *mut KSigaction, 8);
    if r < 0 {
        set_errno(-r as c_int);
        return -1;
    }
    if !old.is_null() {
        core::ptr::write_bytes(old as *mut u8, 0, core::mem::size_of::<libc::sigaction>());
        let o = &mut *old;
        o.sa_sigaction = ko.handler;
        o.sa_flags = (ko.flags & !SA_RESTORER) as c_int;
        *(&mut o.sa_mask as *mut libc::sigset_t as *mut u64) = ko.mask;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn signal(sig: c_int, handler: libc::sighandler_t) -> libc::sighandler_t {
    let mut sa: libc::sigaction = core::mem::zeroed();
    sa.sa_sigaction = handler;
    sa.sa_flags = libc::SA_RESTART;
    let mut old: libc::sigaction = core::mem::zeroed();
    if sigaction(sig, &sa, &mut old) < 0 {
        return libc::SIG_ERR;
    }
    old.sa_sigaction
}

#[no_mangle]
pub unsafe extern "C" fn sigaltstack(ss: *const libc::stack_t, old: *mut libc::stack_t) -> c_int {
    ret_int(sc!(libc::SYS_sigaltstack, ss, old))
}

#[no_mangle]
pub unsafe extern "C" fn sigprocmask(how: c_int, set: *const libc::sigset_t, old: *mut libc::sigset_t) -> c_int {
    ret_int(sc!(libc::SYS_rt_sigprocmask, how, set, old, 8))
}

#[no_mangle]
pub unsafe extern "C" fn pthread_sigmask(how: c_int, set: *const libc::sigset_t, old: *mut libc::sigset_t)
    -> c_int {
    crate::sys::ret_pthread(sc!(libc::SYS_rt_sigprocmask, how, set, old, 8))
}
