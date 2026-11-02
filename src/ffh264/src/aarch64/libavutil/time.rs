// Translated from FFmpeg 9.0.1 libavutil/time.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2000-2003 Fabrice Bellard
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
    fn gettimeofday(_: *mut timeval, _: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn nanosleep(_: *const timespec, _: *mut timespec) -> ::core::ffi::c_int;
    fn clock_gettime(_: clockid_t, _: *mut timespec) -> ::core::ffi::c_int;
}
pub type int64_t = ::core::ffi::c_long;
pub type time_t = ::core::ffi::c_long;
pub type clockid_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
impl timespec {
}
pub type suseconds_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}
#[no_mangle]
pub unsafe extern "C" fn av_gettime() -> int64_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&raw mut tv, ::core::ptr::null_mut::<::core::ffi::c_void>());
    return tv.tv_sec * 1000000 as int64_t + tv.tv_usec as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn av_gettime_relative() -> int64_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as clockid_t, &raw mut ts);
    return ts.tv_sec * 1000000 as int64_t + ts.tv_nsec as int64_t / 1000 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn av_gettime_relative_is_monotonic() -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_usleep(mut usec: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut ts: timespec = timespec {
        tv_sec: usec.wrapping_div(1000000 as ::core::ffi::c_int as ::core::ffi::c_uint)
            as time_t,
        tv_nsec: usec
            .wrapping_rem(1000000 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(1000 as ::core::ffi::c_uint) as ::core::ffi::c_long,
    };
    while nanosleep(&raw mut ts, &raw mut ts) < 0 as ::core::ffi::c_int
        && *__errno_location() == 4 as ::core::ffi::c_int
    {}
    return 0 as ::core::ffi::c_int;
}
