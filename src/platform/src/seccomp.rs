//! seccomp-bpf sandbox: an app allows exactly the syscalls (and ioctl request
//! codes) of the measured platform API, see research/syscall-census-20261031.md.
//! Anything else raises SIGSYS; the handler prints what was blocked and exits
//! with status 159, so a gap in the list is obvious rather than silent.
//!
//! Call `install` early, before spawning threads: the filter applies to the
//! calling thread and everything it creates afterwards.

use std::io;

/// One BPF instruction (struct sock_filter).
#[repr(C)]
#[derive(Clone, Copy)]
struct Insn {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct Prog {
    len: u16,
    filter: *const Insn,
}

const LD_W_ABS: u16 = 0x20; // BPF_LD | BPF_W | BPF_ABS
const JEQ_K: u16 = 0x15; // BPF_JMP | BPF_JEQ | BPF_K
const RET_K: u16 = 0x06; // BPF_RET | BPF_K
const RET_ALLOW: u32 = 0x7fff_0000;
const RET_TRAP: u32 = 0x0003_0000;
const RET_KILL_PROCESS: u32 = 0x8000_0000;
// struct seccomp_data: nr (0), arch (4), instruction_pointer (8), args[6] (16..).
const OFF_NR: u32 = 0;
const OFF_ARCH: u32 = 4;
const OFF_ARG1_LO: u32 = 16 + 8; // little-endian low word of args[1] (the ioctl request)

#[cfg(target_arch = "aarch64")]
const AUDIT_ARCH: u32 = 0xC000_00B7;
#[cfg(target_arch = "arm")]
const AUDIT_ARCH: u32 = 0x4000_0028;
#[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
const AUDIT_ARCH: u32 = 0;

/// ioctl request codes of the platform API (fbdev, ALSA, evdev) plus the
/// terminal-size query Rust's standard library makes on its console.
pub fn platform_ioctls() -> Vec<u32> {
    use crate::{alsa, fb, input};
    vec![
        fb::FBIOGET_VSCREENINFO, fb::FBIOGET_FSCREENINFO, fb::FBIOPAN_DISPLAY, fb::FBIO_WAITFORVSYNC,
        alsa::HW_PARAMS, alsa::PREPARE, alsa::DRAIN, alsa::DROP, alsa::DELAY, alsa::WRITEI_FRAMES,
        input::EVIOCGNAME_128,
        0x5413, // TIOCGWINSZ
    ]
}

// 32-bit ARM time64 syscalls (arch/arm/tools/syscall.tbl; absent from the libc
// crate). musl tries them first; the TV's 4.19 kernel lacks them (ENOSYS), and
// musl then falls back to the 32-bit versions, which are allowed too.
#[cfg(target_arch = "arm")]
const ARM_CLOCK_GETTIME64: libc::c_long = 403;
#[cfg(target_arch = "arm")]
const ARM_CLOCK_NANOSLEEP_TIME64: libc::c_long = 407;
#[cfg(target_arch = "arm")]
const ARM_PPOLL_TIME64: libc::c_long = 414;
#[cfg(target_arch = "arm")]
const ARM_FUTEX_TIME64: libc::c_long = 422;

/// Process plumbing every Rust/musl app here needs: threads, memory, files,
/// time, randomness, exit. `extra` covers paths not hit in the census runs
/// but reachable (e.g. realloc -> mremap, abort -> tgkill, PID 1 power-off).
pub fn base_syscalls() -> Vec<i64> {
    use libc::*;
    let mut v = vec![
        SYS_read, SYS_write, SYS_writev, SYS_openat, SYS_close, SYS_getdents64,
        SYS_munmap, SYS_mprotect, SYS_madvise, SYS_mremap, SYS_brk,
        SYS_futex, SYS_nanosleep, SYS_clock_nanosleep, SYS_clock_gettime,
        SYS_rt_sigprocmask, SYS_rt_sigreturn, SYS_sigaltstack,
        SYS_clone, SYS_exit, SYS_exit_group, SYS_prctl, SYS_sched_getaffinity, SYS_sched_yield,
        SYS_getrandom, SYS_gettid, SYS_getpid, SYS_tgkill, SYS_sync, SYS_reboot, SYS_statx,
        SYS_ioctl, SYS_ftruncate,
    ];
    #[cfg(target_arch = "aarch64")]
    v.extend([SYS_mmap, SYS_fstat, SYS_newfstatat, SYS_fcntl, SYS_lseek]);
    #[cfg(target_arch = "arm")]
    v.extend([SYS_open, SYS_mmap2, SYS_fcntl64, SYS_fstat64, SYS_fstatat64, SYS__llseek,
              ARM_CLOCK_GETTIME64, ARM_CLOCK_NANOSLEEP_TIME64, ARM_FUTEX_TIME64, SYS_ftruncate64]);
    v.into_iter().map(|n| n as i64).collect()
}

/// Sockets for HTTP(S) and musl's DNS resolver.
#[allow(unused_mut)]
pub fn network_syscalls() -> Vec<i64> {
    use libc::*;
    let mut v = vec![SYS_socket, SYS_connect, SYS_bind, SYS_getsockname, SYS_setsockopt,
                     SYS_getsockopt, SYS_sendto, SYS_recvfrom, SYS_ppoll];
    #[cfg(target_arch = "arm")]
    v.extend([SYS_poll, ARM_PPOLL_TIME64]);
    v.into_iter().map(|n| n as i64).collect()
}

/// For an app whose connections come from PID 1 (`broker`): no socket,
/// connect or bind; send/receive on the request socket and on the sockets
/// PID 1 passes it (recvmsg carries them), plus socket options.
pub fn brokered_syscalls() -> Vec<i64> {
    use libc::*;
    let mut v = vec![SYS_setsockopt, SYS_getsockopt, SYS_sendto, SYS_recvfrom, SYS_recvmsg, SYS_ppoll];
    #[cfg(target_arch = "arm")]
    v.extend([SYS_poll, ARM_PPOLL_TIME64]);
    v.into_iter().map(|n| n as i64).collect()
}

fn build(syscalls: &[i64], ioctls: &[u32]) -> Vec<Insn> {
    let ld = |k| Insn { code: LD_W_ABS, jt: 0, jf: 0, k };
    let ret = |k| Insn { code: RET_K, jt: 0, jf: 0, k };
    let mut sc: Vec<u32> = syscalls.iter().map(|&n| n as u32).filter(|&n| n != libc::SYS_ioctl as u32).collect();
    sc.sort_unstable();
    sc.dedup();
    let mut io: Vec<u32> = ioctls.to_vec();
    io.sort_unstable();
    io.dedup();
    // Layout: arch check | syscall compares | ioctl? | TRAP | ld arg1 | ioctl compares | TRAP | ALLOW
    let mut p = vec![
        ld(OFF_ARCH),
        Insn { code: JEQ_K, jt: 1, jf: 0, k: AUDIT_ARCH },
        ret(RET_KILL_PROCESS),
        ld(OFF_NR),
    ];
    let first_sc = p.len();
    let ioctl_block = first_sc + sc.len() + 2; // after the ioctl test and its TRAP
    let allow = ioctl_block + 1 + io.len() + 1;
    for (i, &n) in sc.iter().enumerate() {
        let at = first_sc + i;
        p.push(Insn { code: JEQ_K, jt: (allow - at - 1) as u8, jf: 0, k: n });
    }
    let at = p.len();
    p.push(Insn { code: JEQ_K, jt: (ioctl_block - at - 1) as u8, jf: 0, k: libc::SYS_ioctl as u32 });
    p.push(ret(RET_TRAP));
    p.push(ld(OFF_ARG1_LO));
    for &c in &io {
        let at = p.len();
        p.push(Insn { code: JEQ_K, jt: (allow - at - 1) as u8, jf: 0, k: c });
    }
    p.push(ret(RET_TRAP));
    debug_assert_eq!(p.len(), allow);
    p.push(ret(RET_ALLOW));
    p
}

fn put(buf: &mut [u8], pos: &mut usize, s: &[u8]) {
    for &b in s {
        if *pos < buf.len() {
            buf[*pos] = b;
            *pos += 1;
        }
    }
}

fn put_num(buf: &mut [u8], pos: &mut usize, mut v: u64, base: u64) {
    let mut tmp = [0u8; 20];
    let mut n = 0;
    loop {
        tmp[n] = b"0123456789abcdef"[(v % base) as usize];
        n += 1;
        v /= base;
        if v == 0 {
            break;
        }
    }
    for i in (0..n).rev() {
        put(buf, pos, &[tmp[i]]);
    }
}

#[allow(unused_variables)]
fn ioctl_arg(ctx: *mut libc::c_void) -> u64 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        return (*(ctx as *const libc::ucontext_t)).uc_mcontext.regs[1];
    }
    #[cfg(target_arch = "arm")]
    unsafe {
        return (*(ctx as *const libc::ucontext_t)).uc_mcontext.arm_r1 as u64;
    }
    #[allow(unreachable_code)]
    0
}

extern "C" fn on_sigsys(_sig: libc::c_int, info: *mut libc::siginfo_t, ctx: *mut libc::c_void) {
    // siginfo for SIGSYS: si_signo, si_errno, si_code, then {void *call_addr; int syscall; ...}.
    let off = if cfg!(target_pointer_width = "64") { 24 } else { 16 };
    let nr = unsafe { *((info as *const u8).add(off) as *const i32) };
    let mut buf = [0u8; 96];
    let mut pos = 0;
    put(&mut buf, &mut pos, b"seccomp: blocked syscall ");
    put_num(&mut buf, &mut pos, nr as u32 as u64, 10);
    if nr as i64 == libc::SYS_ioctl as i64 {
        put(&mut buf, &mut pos, b" (ioctl request 0x");
        put_num(&mut buf, &mut pos, ioctl_arg(ctx) & 0xffff_ffff, 16);
        put(&mut buf, &mut pos, b")");
    }
    put(&mut buf, &mut pos, b"\n");
    unsafe {
        libc::write(2, buf.as_ptr() as *const libc::c_void, pos);
        libc::_exit(159);
    }
}

/// Allow exactly `syscalls` and, for ioctl, exactly `ioctls`; trap everything else.
pub fn install(syscalls: &[i64], ioctls: &[u32]) -> io::Result<()> {
    if AUDIT_ARCH == 0 {
        return Err(io::Error::other("seccomp: unsupported architecture"));
    }
    let prog = build(syscalls, ioctls);
    if prog.len() > 255 {
        return Err(io::Error::other("seccomp: allowlist too long for 8-bit jumps"));
    }
    unsafe {
        let mut sa: libc::sigaction = std::mem::zeroed();
        sa.sa_sigaction = on_sigsys as usize;
        sa.sa_flags = libc::SA_SIGINFO;
        if libc::sigaction(libc::SIGSYS, &sa, std::ptr::null_mut()) != 0 {
            return Err(io::Error::last_os_error());
        }
        if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
            return Err(io::Error::last_os_error());
        }
        let fprog = Prog { len: prog.len() as u16, filter: prog.as_ptr() };
        if libc::prctl(libc::PR_SET_SECCOMP, 2 /* SECCOMP_MODE_FILTER */, &fprog as *const Prog, 0, 0) != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}
