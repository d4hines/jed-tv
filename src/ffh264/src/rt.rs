//! Hand-written runtime support for the c2rust translation, replacing the
//! nightly-only items c2rust emits with stable equivalents:
//!   * `core::intrinsics::atomic_*`  -> `atomic_*` below (core::sync::atomic)
//!   * `core::ffi::VaList{,Impl}`    -> [`VaList`], an ABI-exact `va_list`
//!   * the `f128` crate              -> [`F128`], C `long double`
//!   * C `_Alignas` on locals/statics -> [`A4`]/[`A8`]/[`A16`] wrappers
//! C variadic *definitions* (av_log, ...) can't be written in stable Rust; they
//! are tiny assembly trampolines (asm/varargs-*.S) that build a `va_list` and
//! call the translated `<name>__va(fixed args..., VaList)` bodies.

use core::mem::{size_of, transmute_copy};
use core::sync::atomic::Ordering::{self, *};
use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8};

// ---------------------------------------------------------------- atomics
// Every C11 atomic in FFmpeg is a 1/2/4/8-byte integer or pointer; dispatch
// on size (resolved at compile time) to the matching core atomic.

macro_rules! by_size {
    ($t:ty, $p:expr, |$a:ident, $u:ident| $body:expr) => {
        match size_of::<$t>() {
            1 => { type $u = u8; let $a = AtomicU8::from_ptr($p as *mut u8); $body }
            2 => { type $u = u16; let $a = AtomicU16::from_ptr($p as *mut u16); $body }
            4 => { type $u = u32; let $a = AtomicU32::from_ptr($p as *mut u32); $body }
            8 => { type $u = u64; let $a = AtomicU64::from_ptr($p as *mut u64); $body }
            _ => unreachable!("atomic of unsupported size"),
        }
    };
}

#[inline(always)]
unsafe fn load<T: Copy>(p: *const T, o: Ordering) -> T {
    by_size!(T, p, |a, U| transmute_copy::<U, T>(&a.load(o)))
}
#[inline(always)]
unsafe fn store<T: Copy>(p: *mut T, v: T, o: Ordering) {
    by_size!(T, p, |a, U| a.store(transmute_copy::<T, U>(&v), o))
}
#[inline(always)]
unsafe fn xchg<T: Copy>(p: *mut T, v: T, o: Ordering) -> T {
    by_size!(T, p, |a, U| transmute_copy::<U, T>(&a.swap(transmute_copy::<T, U>(&v), o)))
}
#[inline(always)]
unsafe fn xadd<T: Copy>(p: *mut T, v: T, o: Ordering) -> T {
    by_size!(T, p, |a, U| transmute_copy::<U, T>(&a.fetch_add(transmute_copy::<T, U>(&v), o)))
}
#[inline(always)]
unsafe fn xsub<T: Copy>(p: *mut T, v: T, o: Ordering) -> T {
    by_size!(T, p, |a, U| transmute_copy::<U, T>(&a.fetch_sub(transmute_copy::<T, U>(&v), o)))
}
#[inline(always)]
unsafe fn or<T: Copy>(p: *mut T, v: T, o: Ordering) -> T {
    by_size!(T, p, |a, U| transmute_copy::<U, T>(&a.fetch_or(transmute_copy::<T, U>(&v), o)))
}

macro_rules! atomic_load {
    ($($name:ident = $o:expr;)*) => {$(
        #[inline(always)]
        pub unsafe fn $name<T: Copy>(p: *const T) -> T { load(p, $o) }
    )*};
}
macro_rules! atomic_rmw {
    ($($name:ident = $f:ident -> $r:ty, $o:expr;)*) => {$(
        #[inline(always)]
        pub unsafe fn $name<T: Copy>(p: *mut T, v: T) -> $r { $f(p, v, $o) }
    )*};
}
atomic_load! {
    atomic_load_relaxed = Relaxed;
    atomic_load_acquire = Acquire;
    atomic_load_seqcst = SeqCst;
}
atomic_rmw! {
    atomic_store_relaxed = store -> (), Relaxed;
    atomic_store_release = store -> (), Release;
    atomic_store_seqcst = store -> (), SeqCst;
    atomic_xchg_relaxed = xchg -> T, Relaxed;
    atomic_xchg_seqcst = xchg -> T, SeqCst;
    atomic_xadd_relaxed = xadd -> T, Relaxed;
    atomic_xadd_acqrel = xadd -> T, AcqRel;
    atomic_xadd_seqcst = xadd -> T, SeqCst;
    atomic_xsub_relaxed = xsub -> T, Relaxed;
    atomic_xsub_acqrel = xsub -> T, AcqRel;
    atomic_xsub_seqcst = xsub -> T, SeqCst;
    atomic_or_relaxed = or -> T, Relaxed;
    atomic_or_seqcst = or -> T, SeqCst;
}

// ---------------------------------------------------------------- va_list
// Same layout and calling convention as the platform's C `va_list`, so it can
// be passed straight to musl's vsnprintf & co. Copying it is `va_copy`.

/// AAPCS32: `struct __va_list { void *__ap; }`, passed in a core register.
#[cfg(target_arch = "arm")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VaList {
    ap: *mut u8,
}

/// AAPCS64: 32-byte struct, passed by reference to a caller-made copy.
#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VaList {
    stack: *mut u8,
    gr_top: *mut u8,
    vr_top: *mut u8,
    gr_offs: i32,
    vr_offs: i32,
}

/// Types `va_arg` is used with. `FP` selects the FP/SIMD register save area.
pub trait VaArg: Copy {
    const FP: bool;
}
macro_rules! va_arg_types {
    ($fp:expr; $($t:ty),*) => {$( impl VaArg for $t { const FP: bool = $fp; } )*};
}
va_arg_types!(false; i32, u32, i64, u64, isize, usize);
va_arg_types!(true; f64);
impl<T> VaArg for *mut T { const FP: bool = false; }
impl<T> VaArg for *const T { const FP: bool = false; }

impl VaList {
    /// `va_copy`.
    #[inline(always)]
    pub fn clone(&self) -> VaList {
        *self
    }
    #[inline(always)]
    pub fn as_va_list(&mut self) -> VaList {
        *self
    }

    /// `va_arg(ap, T)`.
    #[cfg(target_arch = "arm")]
    pub unsafe fn arg<T: VaArg>(&mut self) -> T {
        // Arguments occupy 4-byte slots; 8-byte types are 8-byte aligned.
        let size = size_of::<T>().max(4);
        let addr = (self.ap as usize + size - 1) & !(size - 1);
        self.ap = (addr + size) as *mut u8;
        core::ptr::read(addr as *const T)
    }

    /// `va_arg(ap, T)`, following the AAPCS64 algorithm.
    #[cfg(target_arch = "aarch64")]
    pub unsafe fn arg<T: VaArg>(&mut self) -> T {
        debug_assert!(size_of::<T>() <= 8);
        let (offs, slot) = if T::FP { (&mut self.vr_offs, 16) } else { (&mut self.gr_offs, 8) };
        if *offs < 0 {
            let o = *offs;
            *offs = o + slot;
            if *offs <= 0 {
                let top = if T::FP { self.vr_top } else { self.gr_top };
                return core::ptr::read(top.offset(o as isize) as *const T);
            }
        }
        let addr = self.stack;
        self.stack = addr.add(8);
        core::ptr::read(addr as *const T)
    }
}

// ---------------------------------------------------------- long double
// c2rust maps C `long double` to the `f128` crate. On AArch64 it is IEEE
// binary128 (passed in a Q register; arithmetic via compiler-rt's soft-float
// routines, exactly as C does); on 32-bit ARM it is just `double`.

#[cfg(target_arch = "aarch64")]
pub use self::ld::F128;

#[cfg(target_arch = "aarch64")]
mod ld {
    use core::arch::aarch64::float64x2_t;
    /// C `long double` on AArch64 (binary128).
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct F128(float64x2_t);
    #[allow(improper_ctypes)]
    extern "C" {
        fn __extenddftf2(a: f64) -> F128;
        fn __floatsitf(a: i32) -> F128;
        fn __floatunsitf(a: u32) -> F128;
        fn __trunctfdf2(a: F128) -> f64;
        fn __addtf3(a: F128, b: F128) -> F128;
        fn __multf3(a: F128, b: F128) -> F128;
    }
    pub trait IntoF128 {
        fn into_f128(self) -> F128;
    }
    impl IntoF128 for f64 {
        fn into_f128(self) -> F128 { unsafe { __extenddftf2(self) } }
    }
    impl IntoF128 for i32 {
        fn into_f128(self) -> F128 { unsafe { __floatsitf(self) } }
    }
    impl IntoF128 for u32 {
        fn into_f128(self) -> F128 { unsafe { __floatunsitf(self) } }
    }
    impl F128 {
        pub fn new<T: IntoF128>(x: T) -> F128 { x.into_f128() }
        pub fn to_f64(&self) -> Option<f64> { Some(unsafe { __trunctfdf2(*self) }) }
    }
    impl core::ops::Add for F128 {
        type Output = F128;
        fn add(self, b: F128) -> F128 { unsafe { __addtf3(self, b) } }
    }
    impl core::ops::Mul for F128 {
        type Output = F128;
        fn mul(self, b: F128) -> F128 { unsafe { __multf3(self, b) } }
    }
}

/// C `long double` on 32-bit ARM, which is IEEE double.
#[cfg(target_arch = "arm")]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct F128(f64);

#[cfg(target_arch = "arm")]
impl F128 {
    pub fn new<T: Into<f64>>(x: T) -> F128 { F128(x.into()) }
    pub fn to_f64(&self) -> Option<f64> { Some(self.0) }
}
#[cfg(target_arch = "arm")]
impl core::ops::Add for F128 {
    type Output = F128;
    fn add(self, b: F128) -> F128 { F128(self.0 + b.0) }
}
#[cfg(target_arch = "arm")]
impl core::ops::Mul for F128 {
    type Output = F128;
    fn mul(self, b: F128) -> F128 { F128(self.0 * b.0) }
}

// ------------------------------------------------------- aligned storage
// c2rust drops `_Alignas` on variables; the post-pass wraps those variables'
// types in these. Deref lets indexing and method calls see the inner value,
// and the value sits at offset 0, so `&raw mut v as *mut T` is unchanged.

macro_rules! aligned {
    ($($name:ident = $n:literal),*) => {$(
        #[repr(C, align($n))]
        #[derive(Copy, Clone)]
        pub struct $name<T>(pub T);
        impl<T> core::ops::Deref for $name<T> {
            type Target = T;
            #[inline(always)]
            fn deref(&self) -> &T { &self.0 }
        }
        impl<T> core::ops::DerefMut for $name<T> {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut T { &mut self.0 }
        }
    )*};
}
aligned!(A4 = 4, A8 = 8, A16 = 16);

// ------------------------------------------------------------- bitfields
// Accessors for C bitfields, which c2rust stores as little-endian byte groups
// (bit i of the group is bit i%8 of byte i/8, as the C ABI lays them out).
// Replaces the c2rust-bitfields derive, whose exact-version pins on
// proc-macro2/syn/quote would downgrade the whole workspace.

#[inline(always)]
pub fn bf_get(bytes: &[u8], lo: u32, hi: u32) -> u64 {
    let mut v = 0u64;
    for i in lo..=hi {
        v |= (((bytes[(i / 8) as usize] >> (i % 8)) & 1) as u64) << (i - lo);
    }
    v
}

#[inline(always)]
pub fn bf_get_signed(bytes: &[u8], lo: u32, hi: u32) -> i64 {
    let w = hi - lo + 1;
    ((bf_get(bytes, lo, hi) << (64 - w)) as i64) >> (64 - w)
}

#[inline(always)]
pub fn bf_set(bytes: &mut [u8], lo: u32, hi: u32, v: u64) {
    for i in lo..=hi {
        let (byte, bit) = ((i / 8) as usize, i % 8);
        bytes[byte] = (bytes[byte] & !(1 << bit)) | ((((v >> (i - lo)) & 1) as u8) << bit);
    }
}
