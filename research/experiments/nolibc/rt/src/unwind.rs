//! The unwinder interface std links against (normally LLVM's libunwind, C++).
//!
//! Our programs are built with panic=abort, so nothing ever unwinds. What
//! remains referenced is std's (precompiled, panic=unwind) personality
//! routine, the ARM EHABI personality routines named by unwind tables, and
//! `_Unwind_Backtrace` for printing backtraces. These stubs make a backtrace
//! empty and any unwinding attempt fail (then abort), which is the cost:
//! no RUST_BACKTRACE output.

use core::ffi::c_void;

pub type Ctx = *mut c_void;
const URC_END_OF_STACK: i32 = 5;
#[cfg(target_arch = "arm")]
const URC_FAILURE: i32 = 9;

#[no_mangle]
pub extern "C" fn _Unwind_Backtrace(_trace: *mut c_void, _arg: *mut c_void) -> i32 {
    URC_END_OF_STACK
}

#[no_mangle]
pub extern "C" fn _Unwind_Resume(_e: *mut c_void) -> ! {
    crate::process::abort()
}

#[no_mangle]
pub extern "C" fn _Unwind_GetLanguageSpecificData(_c: Ctx) -> usize {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_GetRegionStart(_c: Ctx) -> usize {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_GetDataRelBase(_c: Ctx) -> usize {
    0
}

#[no_mangle]
pub extern "C" fn _Unwind_GetTextRelBase(_c: Ctx) -> usize {
    0
}

#[cfg(not(target_arch = "arm"))]
mod itanium {
    use super::Ctx;

    #[no_mangle]
    pub extern "C" fn _Unwind_GetIP(_c: Ctx) -> usize {
        0
    }

    #[no_mangle]
    pub extern "C" fn _Unwind_GetIPInfo(_c: Ctx, before: *mut i32) -> usize {
        if !before.is_null() {
            unsafe { *before = 0 };
        }
        0
    }

    #[no_mangle]
    pub extern "C" fn _Unwind_SetGR(_c: Ctx, _i: i32, _v: usize) {}

    #[no_mangle]
    pub extern "C" fn _Unwind_SetIP(_c: Ctx, _v: usize) {}
}

#[cfg(target_arch = "arm")]
mod ehabi {
    use super::{Ctx, URC_FAILURE};
    use core::ffi::c_void;

    /// _UVRSR_NOT_IMPLEMENTED
    #[no_mangle]
    pub extern "C" fn _Unwind_VRS_Get(_c: Ctx, _class: i32, _reg: u32, _repr: i32, _v: *mut c_void) -> i32 {
        1
    }

    #[no_mangle]
    pub extern "C" fn _Unwind_VRS_Set(_c: Ctx, _class: i32, _reg: u32, _repr: i32, _v: *mut c_void) -> i32 {
        1
    }

    #[no_mangle]
    pub extern "C" fn __gnu_unwind_frame(_e: *mut c_void, _c: Ctx) -> i32 {
        URC_FAILURE
    }

    #[no_mangle]
    pub extern "C" fn __aeabi_unwind_cpp_pr0(_s: i32, _e: *mut c_void, _c: Ctx) -> i32 {
        URC_FAILURE
    }

    #[no_mangle]
    pub extern "C" fn __aeabi_unwind_cpp_pr1(_s: i32, _e: *mut c_void, _c: Ctx) -> i32 {
        URC_FAILURE
    }

    #[no_mangle]
    pub extern "C" fn __aeabi_unwind_cpp_pr2(_s: i32, _e: *mut c_void, _c: Ctx) -> i32 {
        URC_FAILURE
    }
}
