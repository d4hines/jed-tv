//! Syscall census: run a workload in a forked child traced with ptrace
//! (following every thread), counting each syscall number and each ioctl
//! request code. This measures "the exact API we need" from below.
//! Map numbers to names on the host with tools/syscall_census.py.

use std::collections::{BTreeMap, HashMap};

const PTRACE_GETREGS: libc::c_int = 12; // 32-bit ARM
const PTRACE_GETREGSET: libc::c_int = 0x4204; // aarch64
const NT_PRSTATUS: libc::c_int = 1;

#[derive(Default)]
pub struct Census {
    pub syscalls: BTreeMap<u64, u64>,
    pub ioctls: BTreeMap<u32, u64>,
    pub threads: u64,
    pub exit_status: i32,
}

/// (syscall number, first argument, second argument) at a syscall-entry stop.
fn regs(pid: libc::pid_t) -> Option<(u64, u64, u64)> {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let mut r = [0u64; 34]; // user_pt_regs: x0..x30, sp, pc, pstate
        let mut iov = libc::iovec { iov_base: r.as_mut_ptr() as *mut _, iov_len: std::mem::size_of_val(&r) };
        if libc::ptrace(PTRACE_GETREGSET as _, pid, NT_PRSTATUS as usize as *mut libc::c_void,
                        &mut iov as *mut _ as *mut libc::c_void) < 0 {
            return None;
        }
        Some((r[8], r[0], r[1]))
    }
    #[cfg(target_arch = "arm")]
    unsafe {
        let mut r = [0u32; 18]; // pt_regs: r0..r15, cpsr, ORIG_r0; EABI syscall number in r7
        if libc::ptrace(PTRACE_GETREGS as _, pid, std::ptr::null_mut::<libc::c_void>(),
                        r.as_mut_ptr() as *mut libc::c_void) < 0 {
            return None;
        }
        Some((r[7] as u64, r[0] as u64, r[1] as u64))
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
    {
        let _ = pid;
        None
    }
}

/// Run `work` in a traced child process and return what it asked of the kernel.
/// Call while still single-threaded (fork).
pub fn run_traced(work: impl FnOnce()) -> Census {
    let _ = (PTRACE_GETREGS, PTRACE_GETREGSET, NT_PRSTATUS);
    let child = unsafe { libc::fork() };
    if child == 0 {
        unsafe {
            libc::ptrace(libc::PTRACE_TRACEME, 0, std::ptr::null_mut::<libc::c_void>(),
                         std::ptr::null_mut::<libc::c_void>());
            libc::raise(libc::SIGSTOP);
        }
        work();
        use std::io::Write;
        let _ = std::io::stdout().flush();
        unsafe { libc::_exit(0) };
    }
    let mut c = Census::default();
    if child < 0 {
        c.exit_status = -1;
        return c;
    }
    let mut status = 0;
    unsafe { libc::waitpid(child, &mut status, 0) }; // the initial SIGSTOP
    let opts = libc::PTRACE_O_TRACESYSGOOD | libc::PTRACE_O_TRACECLONE | libc::PTRACE_O_EXITKILL;
    unsafe {
        libc::ptrace(libc::PTRACE_SETOPTIONS, child, std::ptr::null_mut::<libc::c_void>(),
                     opts as usize as *mut libc::c_void);
        libc::ptrace(libc::PTRACE_SYSCALL, child, std::ptr::null_mut::<libc::c_void>(),
                     std::ptr::null_mut::<libc::c_void>());
    }
    let mut in_syscall: HashMap<libc::pid_t, bool> = HashMap::new();
    c.threads = 1;
    loop {
        let pid = unsafe { libc::waitpid(-1, &mut status, libc::__WALL) };
        if pid < 0 {
            break;
        }
        if libc::WIFEXITED(status) || libc::WIFSIGNALED(status) {
            in_syscall.remove(&pid);
            if pid == child {
                c.exit_status = if libc::WIFEXITED(status) { libc::WEXITSTATUS(status) } else { 128 + libc::WTERMSIG(status) };
                break;
            }
            continue;
        }
        if !libc::WIFSTOPPED(status) {
            continue;
        }
        let sig = libc::WSTOPSIG(status);
        let event = (status >> 16) & 0xff;
        let mut deliver = 0;
        if sig == (libc::SIGTRAP | 0x80) {
            let entering = !in_syscall.get(&pid).copied().unwrap_or(false);
            in_syscall.insert(pid, entering);
            if entering {
                if let Some((nr, _a0, a1)) = regs(pid) {
                    *c.syscalls.entry(nr).or_default() += 1;
                    if nr == libc::SYS_ioctl as u64 {
                        *c.ioctls.entry(a1 as u32).or_default() += 1;
                    }
                }
            }
        } else if sig == libc::SIGTRAP && event == libc::PTRACE_EVENT_CLONE {
            c.threads += 1;
        } else if sig == libc::SIGSTOP && !in_syscall.contains_key(&pid) {
            in_syscall.insert(pid, false); // a new thread's initial stop
        } else if sig != libc::SIGTRAP {
            deliver = sig;
        }
        unsafe {
            libc::ptrace(libc::PTRACE_SYSCALL, pid, std::ptr::null_mut::<libc::c_void>(),
                         deliver as usize as *mut libc::c_void);
        }
    }
    c
}

impl Census {
    pub fn print(&self) {
        println!("census: arch {} threads {} exit {}", std::env::consts::ARCH, self.threads, self.exit_status);
        for (nr, n) in &self.syscalls {
            println!("census: syscall {nr} {n}");
        }
        for (cmd, n) in &self.ioctls {
            println!("census: ioctl 0x{cmd:08x} {n}");
        }
        println!("census: end");
    }
}
