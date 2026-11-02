//! Find a function in the kernel's vDSO (AT_SYSINFO_EHDR), so clock reads
//! need no syscall where the kernel offers one. The TV's 32-bit 4.19 kernel
//! has none (research/syscall-census-20261031.md); QEMU's kernels do.

use core::ffi::CStr;
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(target_pointer_width = "64")]
use libc::{Elf64_Ehdr as Ehdr, Elf64_Phdr as Phdr, Elf64_Sym as Sym};
#[cfg(target_pointer_width = "32")]
use libc::{Elf32_Ehdr as Ehdr, Elf32_Phdr as Phdr, Elf32_Sym as Sym};

const DT_NULL: usize = 0;
const DT_HASH: usize = 4;
const DT_STRTAB: usize = 5;
const DT_SYMTAB: usize = 6;

/// clock_gettime in the vDSO taking the same `struct timespec` std passes:
/// 64-bit on AArch64; the 32-bit time_t variant on ARM.
#[cfg(target_arch = "aarch64")]
const CLOCK_GETTIME: &CStr = c"__kernel_clock_gettime";
#[cfg(target_arch = "arm")]
const CLOCK_GETTIME: &CStr = c"__vdso_clock_gettime";

unsafe fn lookup(name: &CStr) -> usize {
    let base = crate::start::getauxval(libc::AT_SYSINFO_EHDR) as usize;
    if base == 0 {
        return 0;
    }
    let eh = &*(base as *const Ehdr);
    if &eh.e_ident[..4] != b"\x7fELF" {
        return 0;
    }
    let phdrs = core::slice::from_raw_parts((base + eh.e_phoff as usize) as *const Phdr, eh.e_phnum as usize);
    let (mut bias, mut dynamic) = (None, 0usize);
    for p in phdrs {
        if p.p_type == libc::PT_LOAD && bias.is_none() {
            bias = Some(base + p.p_offset as usize - p.p_vaddr as usize);
        } else if p.p_type == libc::PT_DYNAMIC {
            dynamic = base + p.p_offset as usize;
        }
    }
    let Some(bias) = bias else { return 0 };
    if dynamic == 0 {
        return 0;
    }
    let (mut hash, mut strtab, mut symtab) = (0usize, 0usize, 0usize);
    let mut d = dynamic as *const usize;
    while *d != DT_NULL {
        let v = *d.add(1);
        match *d {
            DT_HASH => hash = bias + v,
            DT_STRTAB => strtab = bias + v,
            DT_SYMTAB => symtab = bias + v,
            _ => {}
        }
        d = d.add(2);
    }
    if hash == 0 || strtab == 0 || symtab == 0 {
        return 0;
    }
    let nsyms = *((hash + 4) as *const u32) as usize; // nchain = number of symbols
    let syms = core::slice::from_raw_parts(symtab as *const Sym, nsyms);
    for s in syms {
        let bind = s.st_info >> 4;
        if s.st_shndx == 0 || s.st_value == 0 || !(bind == 1 || bind == 2) {
            continue;
        }
        if CStr::from_ptr((strtab + s.st_name as usize) as *const libc::c_char) == name {
            return bias + s.st_value as usize;
        }
    }
    0
}

/// 0 = not looked up yet, 1 = no vDSO function, else its address.
static CGT: AtomicUsize = AtomicUsize::new(0);

pub type ClockGettime = unsafe extern "C" fn(libc::clockid_t, *mut libc::timespec) -> libc::c_int;

pub fn clock_gettime() -> Option<ClockGettime> {
    let mut f = CGT.load(Ordering::Relaxed);
    if f == 0 {
        f = unsafe { lookup(CLOCK_GETTIME) }.max(1);
        CGT.store(f, Ordering::Relaxed);
    }
    (f > 1).then(|| unsafe { core::mem::transmute::<usize, ClockGettime>(f) })
}
