// Translated from FFmpeg 9.0.1 libavcodec/threadprogress.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2022 Andreas Rheinhardt <andreas.rheinhardt@outlook.com>
//
// This file is part of FFmpeg.
//
// FFmpeg is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// FFmpeg is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with FFmpeg; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
//

extern "C" {
    fn ff_pthread_init(
        obj: *mut ::core::ffi::c_void,
        offsets: *const ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn ff_pthread_free(
        obj: *mut ::core::ffi::c_void,
        offsets: *const ::core::ffi::c_uint,
    );
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_broadcast(_: *mut pthread_cond_t) -> ::core::ffi::c_int;
}
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_int = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __i: [::core::ffi::c_int; 6],
    pub __vi: [::core::ffi::c_int; 6],
    pub __p: [*mut ::core::ffi::c_void; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_cond_t {
    pub __u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __i: [::core::ffi::c_int; 12],
    pub __vi: [::core::ffi::c_int; 12],
    pub __p: [*mut ::core::ffi::c_void; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadProgress {
    pub progress: atomic_int,
    pub init: ::core::ffi::c_uint,
    pub progress_mutex: pthread_mutex_t,
    pub progress_cond: pthread_cond_t,
}
static mut thread_progress_offsets: [::core::ffi::c_uint; 5] = [
    4 as ::core::ffi::c_uint,
    8 as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    32 as ::core::ffi::c_uint,
    0 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_thread_progress_init(
    mut pro: *mut ThreadProgress,
    mut init_mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *&raw mut (*pro).progress = if init_mode != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        2147483647 as ::core::ffi::c_int
    };
    if init_mode != 0 {
        return ff_pthread_init(
            pro as *mut ::core::ffi::c_void,
            &raw const thread_progress_offsets as *const ::core::ffi::c_uint,
        );
    }
    (*pro).init = init_mode as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_thread_progress_destroy(mut pro: *mut ThreadProgress) {
    ff_pthread_free(
        pro as *mut ::core::ffi::c_void,
        &raw const thread_progress_offsets as *const ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_thread_progress_report(
    mut pro: *mut ThreadProgress,
    mut n: ::core::ffi::c_int,
) {
    if crate::rt::atomic_load_relaxed(&raw mut (*pro).progress) >= n {
        return;
    }
    pthread_mutex_lock(&raw mut (*pro).progress_mutex);
    crate::rt::atomic_store_release(&raw mut (*pro).progress, n);
    pthread_cond_broadcast(&raw mut (*pro).progress_cond);
    pthread_mutex_unlock(&raw mut (*pro).progress_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn ff_thread_progress_await(
    mut pro_c: *const ThreadProgress,
    mut n: ::core::ffi::c_int,
) {
    let mut pro: *mut ThreadProgress = pro_c as *mut ThreadProgress;
    if crate::rt::atomic_load_acquire(&raw mut (*pro).progress) >= n {
        return;
    }
    pthread_mutex_lock(&raw mut (*pro).progress_mutex);
    while crate::rt::atomic_load_relaxed(&raw mut (*pro).progress) < n {
        pthread_cond_wait(&raw mut (*pro).progress_cond, &raw mut (*pro).progress_mutex);
    }
    pthread_mutex_unlock(&raw mut (*pro).progress_mutex);
}
