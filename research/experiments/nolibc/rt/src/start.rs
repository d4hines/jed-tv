//! Program startup and the process-wide facts the kernel hands us: argv,
//! environ, the auxiliary vector, our own program headers.

use crate::sync::Racy;
use core::ffi::CStr;
use libc::{c_char, c_int, c_long, c_ulong, c_void, size_t};

#[cfg(target_pointer_width = "64")]
pub type Phdr = libc::Elf64_Phdr;
#[cfg(target_pointer_width = "32")]
pub type Phdr = libc::Elf32_Phdr;

#[derive(Clone, Copy)]
pub struct Aux {
    pub auxv: *const usize,
    pub phdr: usize,
    pub phnum: usize,
    pub pagesz: usize,
    pub execfn: usize,
    pub initial_sp: usize,
}

pub static AUX: Racy<Aux> = Racy::new(Aux {
    auxv: core::ptr::null(), phdr: 0, phnum: 0, pagesz: 4096, execfn: 0, initial_sp: 0,
});

#[no_mangle]
#[used]
pub static mut environ: *mut *mut c_char = core::ptr::null_mut();
#[no_mangle]
#[used]
pub static mut __environ: *mut *mut c_char = core::ptr::null_mut();

extern "C" {
    fn main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;
    static __preinit_array_start: [Option<extern "C" fn()>; 0];
    static __preinit_array_end: [Option<extern "C" fn()>; 0];
    static __init_array_start: [Option<extern "C" fn()>; 0];
    static __init_array_end: [Option<extern "C" fn()>; 0];
}

unsafe fn run_array(start: *const Option<extern "C" fn()>, end: *const Option<extern "C" fn()>) {
    let mut p = start;
    while p < end {
        if let Some(f) = *p {
            f();
        }
        p = p.add(1);
    }
}

/// Called by `_start` (arch.rs) with the initial stack pointer:
/// argc, argv[argc], NULL, envp..., NULL, auxv pairs..., AT_NULL.
#[no_mangle]
pub unsafe extern "C" fn __nolibc_start_c(sp: *mut usize) -> ! {
    let argc = *sp as c_int;
    let argv = sp.add(1) as *mut *mut c_char;
    let envp = argv.add(argc as usize + 1);
    let mut e = envp;
    while !(*e).is_null() {
        e = e.add(1);
    }
    let auxv = e.add(1) as *const usize;
    let mut aux = Aux { auxv, phdr: 0, phnum: 0, pagesz: 4096, execfn: 0, initial_sp: sp as usize };
    let mut a = auxv;
    while *a != 0 {
        let v = *a.add(1);
        match *a as c_ulong {
            libc::AT_PHDR => aux.phdr = v,
            libc::AT_PHNUM => aux.phnum = v,
            libc::AT_PAGESZ => aux.pagesz = v,
            libc::AT_EXECFN => aux.execfn = v,
            _ => {}
        }
        a = a.add(2);
    }
    AUX.set(aux);
    environ = envp;
    __environ = envp;
    // Thread pointer, TLS block and errno for the main thread. Nothing before
    // this point may touch errno or thread-locals.
    crate::thread::init_main_thread();
    run_array(__preinit_array_start.as_ptr(), __preinit_array_end.as_ptr());
    run_array(__init_array_start.as_ptr(), __init_array_end.as_ptr());
    crate::process::exit(main(argc, argv, envp))
}

pub fn phdrs() -> &'static [Phdr] {
    let a = AUX.get();
    if a.phdr == 0 {
        return &[];
    }
    unsafe { core::slice::from_raw_parts(a.phdr as *const Phdr, a.phnum) }
}

/// Load bias: 0 for our non-PIE executables, computed anyway.
pub fn load_bias() -> usize {
    for p in phdrs() {
        if p.p_type == libc::PT_PHDR {
            return AUX.get().phdr.wrapping_sub(p.p_vaddr as usize);
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn getauxval(t: c_ulong) -> c_ulong {
    let mut a = AUX.get().auxv;
    if !a.is_null() {
        while *a != 0 {
            if *a as c_ulong == t {
                return *a.add(1) as c_ulong;
            }
            a = a.add(2);
        }
    }
    crate::sys::set_errno(libc::ENOENT);
    0
}

#[no_mangle]
pub unsafe extern "C" fn getenv(name: *const c_char) -> *mut c_char {
    let name = CStr::from_ptr(name).to_bytes();
    if name.is_empty() || name.contains(&b'=') {
        return core::ptr::null_mut();
    }
    let mut e = environ;
    if e.is_null() {
        return core::ptr::null_mut();
    }
    while !(*e).is_null() {
        let entry = CStr::from_ptr(*e).to_bytes();
        if entry.len() > name.len() && entry.starts_with(name) && entry[name.len()] == b'=' {
            return (*e).add(name.len() + 1);
        }
        e = e.add(1);
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn sysconf(name: c_int) -> c_long {
    match name {
        libc::_SC_PAGESIZE => AUX.get().pagesz as c_long,
        libc::_SC_NPROCESSORS_ONLN | libc::_SC_NPROCESSORS_CONF => {
            let mut set = [0u8; 128];
            let r = sc!(libc::SYS_sched_getaffinity, 0, set.len(), set.as_mut_ptr());
            if r <= 0 {
                return 1;
            }
            set[..r as usize].iter().map(|b| b.count_ones() as c_long).sum::<c_long>().max(1)
        }
        libc::_SC_CLK_TCK => 100,
        libc::_SC_OPEN_MAX => 1024,
        libc::_SC_GETPW_R_SIZE_MAX | libc::_SC_GETGR_R_SIZE_MAX => -1,
        _ => {
            crate::sys::set_errno(libc::EINVAL);
            -1
        }
    }
}

/// One object (the executable); std's backtrace symbolizer walks this.
#[no_mangle]
pub unsafe extern "C" fn dl_iterate_phdr(
    cb: Option<unsafe extern "C" fn(*mut libc::dl_phdr_info, size_t, *mut c_void) -> c_int>,
    data: *mut c_void,
) -> c_int {
    let Some(cb) = cb else { return 0 };
    let a = AUX.get();
    let mut info: libc::dl_phdr_info = core::mem::zeroed();
    info.dlpi_addr = load_bias() as _;
    info.dlpi_name = c"".as_ptr();
    info.dlpi_phdr = a.phdr as *const _;
    info.dlpi_phnum = a.phnum as _;
    info.dlpi_adds = 1;
    info.dlpi_tls_modid = phdrs().iter().any(|p| p.p_type == libc::PT_TLS) as size_t;
    cb(&mut info, core::mem::size_of::<libc::dl_phdr_info>(), data)
}
