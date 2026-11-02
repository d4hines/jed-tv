//! The architecture layer: raw syscalls, the thread pointer, and the few
//! routines that must be assembly (program entry, clone's child side, signal
//! return trampolines, ARM's `__aeabi_read_tp`).

#[cfg(target_arch = "aarch64")]
mod imp {
    use core::arch::{asm, global_asm};

    /// Size of the ABI thread control block at the thread pointer (TLS variant I).
    pub const TCB_SIZE: usize = 16;

    #[inline(always)]
    pub unsafe fn syscall6(n: usize, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> isize {
        let r: isize;
        asm!("svc 0", in("x8") n, inlateout("x0") a as isize => r, in("x1") b, in("x2") c,
             in("x3") d, in("x4") e, in("x5") f, options(nostack));
        r
    }

    #[inline(always)]
    pub fn tp() -> usize {
        let v: usize;
        unsafe { asm!("mrs {}, tpidr_el0", out(reg) v, options(nomem, nostack, preserves_flags)) };
        v
    }

    /// Set the calling thread's thread pointer.
    pub unsafe fn set_tp(v: usize) -> isize {
        asm!("msr tpidr_el0, {}", in(reg) v, options(nostack, preserves_flags));
        0
    }

    global_asm!(
        r#"
        .text
        .globl _start
        .type _start,%function
    _start:
        mov x29, #0
        mov x30, #0
        mov x0, sp
        bl __nolibc_start_c
        brk #1
        .size _start, .-_start

        // long __nolibc_clone(flags, stack, ptid, tls, ctid, fn, arg)
        // The child starts on `stack` with every register but x0 copied from
        // the parent, so fn/arg (x5/x6) are still there.
        .globl __nolibc_clone
        .type __nolibc_clone,%function
    __nolibc_clone:
        and x1, x1, #-16
        mov x8, #220
        svc #0
        cbz x0, 1f
        ret
    1:  mov x29, #0
        mov x30, #0
        mov x0, x6
        blr x5
        mov x8, #93
        svc #0
        brk #1
        .size __nolibc_clone, .-__nolibc_clone

        .globl __nolibc_restore_rt
        .type __nolibc_restore_rt,%function
    __nolibc_restore_rt:
        mov x8, #139
        svc #0
        .size __nolibc_restore_rt, .-__nolibc_restore_rt
        "#
    );
}

#[cfg(target_arch = "arm")]
mod imp {
    use core::arch::{asm, global_asm};

    /// Size of the ABI thread control block at the thread pointer (TLS variant I).
    pub const TCB_SIZE: usize = 8;

    #[inline(always)]
    pub unsafe fn syscall6(n: usize, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> isize {
        let r: isize;
        asm!("svc 0", in("r7") n, inlateout("r0") a as isize => r, in("r1") b, in("r2") c,
             in("r3") d, in("r4") e, in("r5") f, options(nostack));
        r
    }

    #[inline(always)]
    pub fn tp() -> usize {
        let v: usize;
        unsafe { asm!("mrc p15, 0, {}, c13, c0, 3", out(reg) v, options(nomem, nostack, preserves_flags)) };
        v
    }

    /// Set the calling thread's thread pointer (ARM private syscall set_tls).
    pub unsafe fn set_tp(v: usize) -> isize {
        syscall6(0x0f0005, v, 0, 0, 0, 0, 0)
    }

    global_asm!(
        r#"
        .syntax unified
        .arm
        .text
        .globl _start
        .type _start,%function
    _start:
        mov fp, #0
        mov lr, #0
        mov r0, sp
        bic sp, sp, #15
        bl __nolibc_start_c
        udf #0
        .size _start, .-_start

        @ long __nolibc_clone(flags, stack, ptid, tls, ctid, fn, arg)
        @ r0-r3 = flags, stack, ptid, tls; [sp] = ctid, [sp+4] = fn, [sp+8] = arg.
        @ The child starts on `stack` with every register but r0 copied from
        @ the parent, so fn/arg (r5/r6) are still there.
        .globl __nolibc_clone
        .type __nolibc_clone,%function
    __nolibc_clone:
        push {{r4, r5, r6, r7}}
        ldr r4, [sp, #16]
        ldr r5, [sp, #20]
        ldr r6, [sp, #24]
        bic r1, r1, #15
        mov r7, #120
        svc 0
        cmp r0, #0
        beq 1f
        pop {{r4, r5, r6, r7}}
        bx lr
    1:  mov fp, #0
        mov lr, #0
        mov r0, r6
        blx r5
        mov r7, #1
        svc 0
        udf #0
        .size __nolibc_clone, .-__nolibc_clone

        @ Signal return: non-SA_SIGINFO handlers get a sigreturn frame,
        @ SA_SIGINFO handlers an rt_sigreturn frame.
        .globl __nolibc_restore
        .type __nolibc_restore,%function
    __nolibc_restore:
        mov r7, #119
        svc 0
        .size __nolibc_restore, .-__nolibc_restore

        .globl __nolibc_restore_rt
        .type __nolibc_restore_rt,%function
    __nolibc_restore_rt:
        mov r7, #173
        svc 0
        .size __nolibc_restore_rt, .-__nolibc_restore_rt

        @ EABI helper used by compiled code for thread-local access. Must
        @ preserve every register except r0.
        .globl __aeabi_read_tp
        .type __aeabi_read_tp,%function
    __aeabi_read_tp:
        mrc p15, 0, r0, c13, c0, 3
        bx lr
        .size __aeabi_read_tp, .-__aeabi_read_tp
        "#
    );
}

pub use imp::*;

extern "C" {
    /// Assembly above: clone a thread that runs `f(arg)` on `stack`, then exit(2)s.
    pub fn __nolibc_clone(flags: usize, stack: usize, ptid: *mut i32, tls: usize, ctid: *mut i32,
                          f: extern "C" fn(usize) -> usize, arg: usize) -> isize;
    pub fn __nolibc_restore_rt();
    #[cfg(target_arch = "arm")]
    pub fn __nolibc_restore();
}
