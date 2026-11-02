//! Test-only PID 1 trampoline. Exercise stock ARM32 syscall fallbacks on the
//! newer QEMU kernel; this is an absence model, not a 4.19 kernel emulator.
use std::os::unix::process::CommandExt;
use std::process::Command;

#[repr(C)]
struct Insn { code: u16, jt: u8, jf: u8, k: u32 }
#[repr(C)]
struct Prog { len: u16, filter: *const Insn }
extern "C" {
    fn prctl(option: i32, ...) -> i32;
    fn syscall(number: i32, ...) -> i32;
    fn __errno_location() -> *mut i32;
}
fn main() {
    assert_eq!(std::process::id(), 1, "QEMU test trampoline must be PID 1");
    let ld = |k| Insn { code: 0x20, jt: 0, jf: 0, k };
    let ret = |k| Insn { code: 0x06, jt: 0, jf: 0, k };
    // ARM EABI calls 403+ were added after 4.19. Preserve the ARM-private
    // 0x0f0000+ range (notably set_tls/cacheflush), needed by static musl.
    let filter = [
        ld(4), Insn { code: 0x15, jt: 1, jf: 0, k: 0x4000_0028 }, ret(0x8000_0000),
        ld(0), Insn { code: 0x35, jt: 0, jf: 2, k: 403 },
        Insn { code: 0x35, jt: 1, jf: 0, k: 0x0f0000 }, ret(0x0005_0000 | 38), ret(0x7fff_0000),
    ];
    let prog = Prog { len: filter.len() as u16, filter: filter.as_ptr() };
    unsafe {
        assert_eq!(prctl(38, 1u32, 0u32, 0u32, 0u32), 0, "no_new_privs failed");
        assert_eq!(prctl(22, 2u32, &prog as *const Prog), 0, "seccomp absence model failed");
        // Verify representative time64 and newer calls before executing the app.
        // A null clock pointer on a newer unfiltered kernel gives EFAULT, not ENOSYS.
        for n in [403, 407, 414, 422, 435, 437] {
            *__errno_location() = 0;
            assert_eq!(syscall(n, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32), -1);
            assert_eq!(*__errno_location(), 38, "new syscall {n} must return ENOSYS");
        }
    }
    // std/musl must successfully fall back to legacy clock/futex calls.
    let start = std::time::Instant::now();
    std::thread::spawn(|| std::thread::sleep(std::time::Duration::from_millis(2))).join().unwrap();
    assert!(start.elapsed() > std::time::Duration::ZERO);
    println!("g06-compat: ARM32 post-4.19 syscalls return ENOSYS; legacy clock/thread fallback passed; PID 1 retained");
    let error = Command::new("/sbin/g06-payload").args(std::env::args_os().skip(1)).exec();
    panic!("exec target init failed: {error}");
}
