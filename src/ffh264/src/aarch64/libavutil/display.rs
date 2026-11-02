// Translated from FFmpeg 9.0.1 libavutil/display.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2014 Vittorio Giovara <vittorio.giovara@gmail.com>
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn atan2(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn hypot(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type int32_t = ::core::ffi::c_int;
pub type size_t = ::core::ffi::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn av_display_rotation_get(
    mut matrix: *const int32_t,
) -> ::core::ffi::c_double {
    let mut rotation: ::core::ffi::c_double = 0.;
    let mut scale: [::core::ffi::c_double; 2] = [0.; 2];
    scale[0 as ::core::ffi::c_int as usize] = hypot(
        *matrix.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            / ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
        *matrix.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            / ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
    );
    scale[1 as ::core::ffi::c_int as usize] = hypot(
        *matrix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            / ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
        *matrix.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            / ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
    );
    if scale[0 as ::core::ffi::c_int as usize] == 0.0f64
        || scale[1 as ::core::ffi::c_int as usize] == 0.0f64
    {
        return ::core::f32::NAN as ::core::ffi::c_double;
    }
    rotation = atan2(
        *matrix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            / ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_double / scale[1 as ::core::ffi::c_int as usize],
        *matrix.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_double
            / ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_double / scale[0 as ::core::ffi::c_int as usize],
    ) * 180 as ::core::ffi::c_int as ::core::ffi::c_double / 3.14159265358979323846f64;
    return -rotation;
}
#[no_mangle]
pub unsafe extern "C" fn av_display_rotation_set(
    mut matrix: *mut int32_t,
    mut angle: ::core::ffi::c_double,
) {
    let mut radians: ::core::ffi::c_double = -angle * 3.14159265358979323846f64
        / 180.0f64;
    let mut c: ::core::ffi::c_double = cos(radians);
    let mut s: ::core::ffi::c_double = sin(radians);
    memset(
        matrix as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (9 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
    *matrix.offset(0 as ::core::ffi::c_int as isize) = (c
        * ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as int32_t;
    *matrix.offset(1 as ::core::ffi::c_int as isize) = (-s
        * ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as int32_t;
    *matrix.offset(3 as ::core::ffi::c_int as isize) = (s
        * ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as int32_t;
    *matrix.offset(4 as ::core::ffi::c_int as isize) = (c
        * ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as ::core::ffi::c_double) as int32_t;
    *matrix.offset(8 as ::core::ffi::c_int as isize) = ((1 as ::core::ffi::c_int)
        << 30 as ::core::ffi::c_int) as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn av_display_matrix_flip(
    mut matrix: *mut int32_t,
    mut hflip: ::core::ffi::c_int,
    mut vflip: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let flip: [::core::ffi::c_int; 3] = [
        1 as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int * (hflip != 0) as ::core::ffi::c_int,
        1 as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int * (vflip != 0) as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    if hflip != 0 || vflip != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            let ref mut fresh0 = *matrix.offset(i as isize);
            *fresh0 *= flip[(i % 3 as ::core::ffi::c_int) as usize];
            i += 1;
        }
    }
}
