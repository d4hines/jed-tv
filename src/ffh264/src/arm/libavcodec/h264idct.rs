// Translated from FFmpeg 9.0.1 libavcodec/h264idct.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.264 IDCT
// Copyright (c) 2004 Michael Niedermayer <michaelni@gmx.at>
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
}
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type size_t = ::core::ffi::c_uint;



#[no_mangle]
pub unsafe extern "C" fn ff_h264_idct_add_8_c(
    mut _dst: *mut uint8_t,
    mut _block: *mut int16_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut block: *mut int16_t = _block;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    let ref mut fresh1 = *block.offset(0 as ::core::ffi::c_int as isize);
    *fresh1 = (*fresh1 as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)) as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let z0: ::core::ffi::c_uint = (*block
            .offset((i + 4 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (i + 4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let z1: ::core::ffi::c_uint = (*block
            .offset((i + 4 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_sub(
                *block
                    .offset(
                        (i + 4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let z2: ::core::ffi::c_uint = ((*block
            .offset((i + 4 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_sub(
                *block
                    .offset(
                        (i + 4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let z3: ::core::ffi::c_uint = (*block
            .offset((i + 4 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                (*block
                    .offset(
                        (i + 4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
        *block
            .offset((i + 4 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize) = z0
            .wrapping_add(z3) as int16_t;
        *block
            .offset((i + 4 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize) = z1
            .wrapping_add(z2) as int16_t;
        *block
            .offset((i + 4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize) = z1
            .wrapping_sub(z2) as int16_t;
        *block
            .offset((i + 4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize) = z0
            .wrapping_sub(z3) as int16_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let z0_0: ::core::ffi::c_uint = (*block
            .offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * i) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * i) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let z1_0: ::core::ffi::c_uint = (*block
            .offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * i) as isize)
            as ::core::ffi::c_uint)
            .wrapping_sub(
                *block
                    .offset(
                        (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * i) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let z2_0: ::core::ffi::c_uint = ((*block
            .offset((1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * i) as isize)
            as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_sub(
                *block
                    .offset(
                        (3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * i) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let z3_0: ::core::ffi::c_uint = (*block
            .offset((1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * i) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                (*block
                    .offset(
                        (3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * i) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
        *dst.offset((i + 0 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 0 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (z0_0.wrapping_add(z3_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 1 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 1 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (z1_0.wrapping_add(z2_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 2 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 2 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (z1_0.wrapping_sub(z2_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 3 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 3 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (z0_0.wrapping_sub(z3_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        i += 1;
    }
    memset(
        block as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (16 as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    );
}

static mut scan8: [uint8_t; 51] = [
    (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 10 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn ff_h264_idct8_add_8_c(
    mut _dst: *mut uint8_t,
    mut _block: *mut int16_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut block: *mut int16_t = _block;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    let ref mut fresh0 = *block.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = (*fresh0 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as int16_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let a0: ::core::ffi::c_uint = (*block
            .offset((i + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (i + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let a2: ::core::ffi::c_uint = (*block
            .offset((i + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_sub(
                *block
                    .offset(
                        (i + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let a4: ::core::ffi::c_uint = ((*block
            .offset((i + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_sub(
                *block
                    .offset(
                        (i + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let a6: ::core::ffi::c_uint = ((*block
            .offset((i + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (i + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let b0: ::core::ffi::c_uint = a0.wrapping_add(a6);
        let b2: ::core::ffi::c_uint = a2.wrapping_add(a4);
        let b4: ::core::ffi::c_uint = a2.wrapping_sub(a4);
        let b6: ::core::ffi::c_uint = a0.wrapping_sub(a6);
        let a1: ::core::ffi::c_int = (-(*block
            .offset((i + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (i + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_sub(
                *block
                    .offset(
                        (i + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_sub(
                (*block
                    .offset(
                        (i + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        let a3: ::core::ffi::c_int = (*block
            .offset((i + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (i + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_sub(
                *block
                    .offset(
                        (i + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_sub(
                (*block
                    .offset(
                        (i + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        let a5: ::core::ffi::c_int = (-(*block
            .offset((i + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (i + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *block
                    .offset(
                        (i + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (*block
                    .offset(
                        (i + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        let a7: ::core::ffi::c_int = (*block
            .offset((i + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (i + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *block
                    .offset(
                        (i + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (*block
                    .offset(
                        (i + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        let b1: ::core::ffi::c_int = ((a7 >> 2 as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_add(a1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        let b3: ::core::ffi::c_int = (a3 as ::core::ffi::c_uint)
            .wrapping_add((a5 >> 2 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        let b5: ::core::ffi::c_int = ((a3 >> 2 as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_sub(a5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        let b7: ::core::ffi::c_int = (a7 as ::core::ffi::c_uint)
            .wrapping_sub((a1 >> 2 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        *block
            .offset((i + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize) = b0
            .wrapping_add(b7 as ::core::ffi::c_uint) as int16_t;
        *block
            .offset((i + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize) = b0
            .wrapping_sub(b7 as ::core::ffi::c_uint) as int16_t;
        *block
            .offset((i + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize) = b2
            .wrapping_add(b5 as ::core::ffi::c_uint) as int16_t;
        *block
            .offset((i + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize) = b2
            .wrapping_sub(b5 as ::core::ffi::c_uint) as int16_t;
        *block
            .offset((i + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize) = b4
            .wrapping_add(b3 as ::core::ffi::c_uint) as int16_t;
        *block
            .offset((i + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize) = b4
            .wrapping_sub(b3 as ::core::ffi::c_uint) as int16_t;
        *block
            .offset((i + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize) = b6
            .wrapping_add(b1 as ::core::ffi::c_uint) as int16_t;
        *block
            .offset((i + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize) = b6
            .wrapping_sub(b1 as ::core::ffi::c_uint) as int16_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let a0_0: ::core::ffi::c_uint = (*block
            .offset((0 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (4 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let a2_0: ::core::ffi::c_uint = (*block
            .offset((0 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_sub(
                *block
                    .offset(
                        (4 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let a4_0: ::core::ffi::c_uint = ((*block
            .offset((2 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_sub(
                *block
                    .offset(
                        (6 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let a6_0: ::core::ffi::c_uint = ((*block
            .offset((6 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (2 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            );
        let b0_0: ::core::ffi::c_uint = a0_0.wrapping_add(a6_0);
        let b2_0: ::core::ffi::c_uint = a2_0.wrapping_add(a4_0);
        let b4_0: ::core::ffi::c_uint = a2_0.wrapping_sub(a4_0);
        let b6_0: ::core::ffi::c_uint = a0_0.wrapping_sub(a6_0);
        let a1_0: ::core::ffi::c_int = (*block
            .offset((3 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_neg()
            .wrapping_add(
                *block
                    .offset(
                        (5 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_sub(
                *block
                    .offset(
                        (7 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_sub(
                (*block
                    .offset(
                        (7 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        let a3_0: ::core::ffi::c_int = (*block
            .offset((1 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (7 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_sub(
                *block
                    .offset(
                        (3 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_sub(
                (*block
                    .offset(
                        (3 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        let a5_0: ::core::ffi::c_int = (*block
            .offset((1 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_neg()
            .wrapping_add(
                *block
                    .offset(
                        (7 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *block
                    .offset(
                        (5 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (*block
                    .offset(
                        (5 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        let a7_0: ::core::ffi::c_int = (*block
            .offset((3 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                *block
                    .offset(
                        (5 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_add(
                *block
                    .offset(
                        (1 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (*block
                    .offset(
                        (1 as ::core::ffi::c_int + i * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        let b1_0: ::core::ffi::c_uint = ((a7_0 >> 2 as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_add(a1_0 as ::core::ffi::c_uint);
        let b3_0: ::core::ffi::c_uint = (a3_0 as ::core::ffi::c_uint)
            .wrapping_add((a5_0 >> 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        let b5_0: ::core::ffi::c_uint = ((a3_0 >> 2 as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_sub(a5_0 as ::core::ffi::c_uint);
        let b7_0: ::core::ffi::c_uint = (a7_0 as ::core::ffi::c_uint)
            .wrapping_sub((a1_0 >> 2 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        *dst.offset((i + 0 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 0 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (b0_0.wrapping_add(b7_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 1 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 1 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (b2_0.wrapping_add(b5_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 2 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 2 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (b4_0.wrapping_add(b3_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 3 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 3 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (b6_0.wrapping_add(b1_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 4 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 4 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (b6_0.wrapping_sub(b1_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 5 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 5 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (b4_0.wrapping_sub(b3_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 6 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 6 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (b2_0.wrapping_sub(b5_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        *dst.offset((i + 7 as ::core::ffi::c_int * stride) as isize) = av_clip_uint8_c(
            *dst.offset((i + 7 as ::core::ffi::c_int * stride) as isize)
                as ::core::ffi::c_int
                + (b0_0.wrapping_sub(b7_0) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int),
        );
        i += 1;
    }
    memset(
        block as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (64 as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    );
}






#[no_mangle]
pub unsafe extern "C" fn ff_h264_idct_dc_add_8_c(
    mut _dst: *mut uint8_t,
    mut _block: *mut int16_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut block: *mut int16_t = _block;
    let mut dc: ::core::ffi::c_int = *block.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int;
    stride = (stride as ::core::ffi::c_uint)
        .wrapping_div(::core::mem::size_of::<uint8_t>() as usize as ::core::ffi::c_uint)
        as ::core::ffi::c_int as ::core::ffi::c_int;
    *block.offset(0 as ::core::ffi::c_int as isize) = 0 as int16_t;
    j = 0 as ::core::ffi::c_int;
    while j < 4 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            *dst.offset(i as isize) = av_clip_uint8_c(
                *dst.offset(i as isize) as ::core::ffi::c_int + dc,
            );
            i += 1;
        }
        dst = dst.offset(stride as isize);
        j += 1;
    }
}


#[no_mangle]
pub unsafe extern "C" fn ff_h264_idct8_dc_add_8_c(
    mut _dst: *mut uint8_t,
    mut _block: *mut int16_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut block: *mut int16_t = _block;
    let mut dc: ::core::ffi::c_int = *block.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int;
    *block.offset(0 as ::core::ffi::c_int as isize) = 0 as int16_t;
    stride = (stride as ::core::ffi::c_uint)
        .wrapping_div(::core::mem::size_of::<uint8_t>() as usize as ::core::ffi::c_uint)
        as ::core::ffi::c_int as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < 8 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            *dst.offset(i as isize) = av_clip_uint8_c(
                *dst.offset(i as isize) as ::core::ffi::c_int + dc,
            );
            i += 1;
        }
        dst = dst.offset(stride as isize);
        j += 1;
    }
}






#[no_mangle]
pub unsafe extern "C" fn ff_h264_idct_add16_8_c(
    mut dst: *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ::core::ffi::c_int,
    mut nnzc: *const uint8_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        let mut nnz: ::core::ffi::c_int = *nnzc.offset(scan8[i as usize] as isize)
            as ::core::ffi::c_int;
        if nnz != 0 {
            if nnz == 1 as ::core::ffi::c_int
                && *block.offset((i * 16 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int != 0
            {
                ff_h264_idct_dc_add_8_c(
                    dst.offset(*block_offset.offset(i as isize) as isize),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            } else {
                ff_h264_idct_add_8_c(
                    dst.offset(*block_offset.offset(i as isize) as isize),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            }
        }
        i += 1;
    }
}





#[no_mangle]
pub unsafe extern "C" fn ff_h264_idct_add16intra_8_c(
    mut dst: *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ::core::ffi::c_int,
    mut nnzc: *const uint8_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        if *nnzc.offset(scan8[i as usize] as isize) != 0 {
            ff_h264_idct_add_8_c(
                dst.offset(*block_offset.offset(i as isize) as isize),
                block
                    .offset(
                        ((i * 16 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                            as isize,
                    ),
                stride,
            );
        } else if *block.offset((i * 16 as ::core::ffi::c_int) as isize) != 0 {
            ff_h264_idct_dc_add_8_c(
                dst.offset(*block_offset.offset(i as isize) as isize),
                block
                    .offset(
                        ((i * 16 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                            as isize,
                    ),
                stride,
            );
        }
        i += 1;
    }
}




#[no_mangle]
pub unsafe extern "C" fn ff_h264_idct8_add4_8_c(
    mut dst: *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ::core::ffi::c_int,
    mut nnzc: *const uint8_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        let mut nnz: ::core::ffi::c_int = *nnzc.offset(scan8[i as usize] as isize)
            as ::core::ffi::c_int;
        if nnz != 0 {
            if nnz == 1 as ::core::ffi::c_int
                && *block.offset((i * 16 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int != 0
            {
                ff_h264_idct8_dc_add_8_c(
                    dst.offset(*block_offset.offset(i as isize) as isize),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            } else {
                ff_h264_idct8_add_8_c(
                    dst.offset(*block_offset.offset(i as isize) as isize),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            }
        }
        i += 4 as ::core::ffi::c_int;
    }
}

#[inline(always)]
unsafe extern "C" fn av_clip_uint8_c(mut a: ::core::ffi::c_int) -> uint8_t {
    if a & !(0xff as ::core::ffi::c_int) != 0 {
        return (!a >> 31 as ::core::ffi::c_int) as uint8_t
    } else {
        return a as uint8_t
    };
}


#[no_mangle]
pub unsafe extern "C" fn ff_h264_idct_add8_8_c(
    mut dest: *mut *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ::core::ffi::c_int,
    mut nnzc: *const uint8_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 1 as ::core::ffi::c_int;
    while j < 3 as ::core::ffi::c_int {
        i = j * 16 as ::core::ffi::c_int;
        while i < j * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int {
            if *nnzc.offset(scan8[i as usize] as isize) != 0 {
                ff_h264_idct_add_8_c(
                    (*dest.offset((j - 1 as ::core::ffi::c_int) as isize))
                        .offset(*block_offset.offset(i as isize) as isize),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            } else if *block.offset((i * 16 as ::core::ffi::c_int) as isize) != 0 {
                ff_h264_idct_dc_add_8_c(
                    (*dest.offset((j - 1 as ::core::ffi::c_int) as isize))
                        .offset(*block_offset.offset(i as isize) as isize),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            }
            i += 1;
        }
        j += 1;
    }
}



#[no_mangle]
pub unsafe extern "C" fn ff_h264_idct_add8_422_8_c(
    mut dest: *mut *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ::core::ffi::c_int,
    mut nnzc: *const uint8_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 1 as ::core::ffi::c_int;
    while j < 3 as ::core::ffi::c_int {
        i = j * 16 as ::core::ffi::c_int;
        while i < j * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int {
            if *nnzc.offset(scan8[i as usize] as isize) != 0 {
                ff_h264_idct_add_8_c(
                    (*dest.offset((j - 1 as ::core::ffi::c_int) as isize))
                        .offset(*block_offset.offset(i as isize) as isize),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            } else if *block.offset((i * 16 as ::core::ffi::c_int) as isize) != 0 {
                ff_h264_idct_dc_add_8_c(
                    (*dest.offset((j - 1 as ::core::ffi::c_int) as isize))
                        .offset(*block_offset.offset(i as isize) as isize),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            }
            i += 1;
        }
        j += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j < 3 as ::core::ffi::c_int {
        i = j * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int;
        while i < j * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int {
            if *nnzc.offset(scan8[(i + 4 as ::core::ffi::c_int) as usize] as isize) != 0
            {
                ff_h264_idct_add_8_c(
                    (*dest.offset((j - 1 as ::core::ffi::c_int) as isize))
                        .offset(
                            *block_offset.offset((i + 4 as ::core::ffi::c_int) as isize)
                                as isize,
                        ),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            } else if *block.offset((i * 16 as ::core::ffi::c_int) as isize) != 0 {
                ff_h264_idct_dc_add_8_c(
                    (*dest.offset((j - 1 as ::core::ffi::c_int) as isize))
                        .offset(
                            *block_offset.offset((i + 4 as ::core::ffi::c_int) as isize)
                                as isize,
                        ),
                    block
                        .offset(
                            ((i * 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as isize,
                        ),
                    stride,
                );
            }
            i += 1;
        }
        j += 1;
    }
}



#[no_mangle]
pub unsafe extern "C" fn ff_h264_luma_dc_dequant_idct_8_c(
    mut _output: *mut int16_t,
    mut _input: *mut int16_t,
    mut qmul: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut temp: [::core::ffi::c_int; 16] = [0; 16];
    static mut x_offset: [uint8_t; 4] = [
        0 as ::core::ffi::c_int as uint8_t,
        (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
        (8 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
        (10 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
    ];
    let mut input: *mut int16_t = _input;
    let mut output: *mut int16_t = _output;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let z0: ::core::ffi::c_int = *input
            .offset((4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            + *input
                .offset((4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int;
        let z1: ::core::ffi::c_int = *input
            .offset((4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            - *input
                .offset((4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int;
        let z2: ::core::ffi::c_int = *input
            .offset((4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            - *input
                .offset((4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int;
        let z3: ::core::ffi::c_int = *input
            .offset((4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            + *input
                .offset((4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int;
        temp[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize] = z0 + z3;
        temp[(4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize] = z0 - z3;
        temp[(4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as usize] = z1 - z2;
        temp[(4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as usize] = z1 + z2;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let offset: ::core::ffi::c_int = x_offset[i as usize] as ::core::ffi::c_int;
        let z0_0: ::core::ffi::c_uint = (temp[(4 as ::core::ffi::c_int
            * 0 as ::core::ffi::c_int + i) as usize]
            + temp[(4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + i) as usize])
            as ::core::ffi::c_uint;
        let z1_0: ::core::ffi::c_uint = (temp[(4 as ::core::ffi::c_int
            * 0 as ::core::ffi::c_int + i) as usize]
            - temp[(4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + i) as usize])
            as ::core::ffi::c_uint;
        let z2_0: ::core::ffi::c_uint = (temp[(4 as ::core::ffi::c_int
            * 1 as ::core::ffi::c_int + i) as usize]
            - temp[(4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int + i) as usize])
            as ::core::ffi::c_uint;
        let z3_0: ::core::ffi::c_uint = (temp[(4 as ::core::ffi::c_int
            * 1 as ::core::ffi::c_int + i) as usize]
            + temp[(4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int + i) as usize])
            as ::core::ffi::c_uint;
        *output
            .offset(
                (16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int + offset) as isize,
            ) = (z0_0
            .wrapping_add(z3_0)
            .wrapping_mul(qmul as ::core::ffi::c_uint)
            .wrapping_add(128 as ::core::ffi::c_uint) as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as int16_t;
        *output
            .offset(
                (16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int + offset) as isize,
            ) = (z1_0
            .wrapping_add(z2_0)
            .wrapping_mul(qmul as ::core::ffi::c_uint)
            .wrapping_add(128 as ::core::ffi::c_uint) as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as int16_t;
        *output
            .offset(
                (16 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + offset) as isize,
            ) = (z1_0
            .wrapping_sub(z2_0)
            .wrapping_mul(qmul as ::core::ffi::c_uint)
            .wrapping_add(128 as ::core::ffi::c_uint) as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as int16_t;
        *output
            .offset(
                (16 as ::core::ffi::c_int * 5 as ::core::ffi::c_int + offset) as isize,
            ) = (z0_0
            .wrapping_sub(z3_0)
            .wrapping_mul(qmul as ::core::ffi::c_uint)
            .wrapping_add(128 as ::core::ffi::c_uint) as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as int16_t;
        i += 1;
    }
}

#[inline(always)]
unsafe extern "C" fn av_clip_uintp2_c(
    mut a: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    if a as ::core::ffi::c_uint
        & !((1 as ::core::ffi::c_uint) << p).wrapping_sub(1 as ::core::ffi::c_uint) != 0
    {
        return (!a >> 31 as ::core::ffi::c_int) as ::core::ffi::c_uint
            & ((1 as ::core::ffi::c_uint) << p).wrapping_sub(1 as ::core::ffi::c_uint)
    } else {
        return a as ::core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_chroma422_dc_dequant_idct_8_c(
    mut _block: *mut int16_t,
    mut qmul: ::core::ffi::c_int,
) {
    let stride: ::core::ffi::c_int = 16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    let xStride: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut temp: [::core::ffi::c_uint; 8] = [0; 8];
    static mut x_offset: [uint8_t; 2] = [
        0 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
    ];
    let mut block: *mut int16_t = _block;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        temp[(2 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize] = (*block
            .offset((stride * i + xStride * 0 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_add(
                *block.offset((stride * i + xStride * 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_uint,
            );
        temp[(2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize] = (*block
            .offset((stride * i + xStride * 0 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_uint)
            .wrapping_sub(
                *block.offset((stride * i + xStride * 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_uint,
            );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        let offset: ::core::ffi::c_int = x_offset[i as usize] as ::core::ffi::c_int;
        let z0: ::core::ffi::c_uint = temp[(2 as ::core::ffi::c_int
                * 0 as ::core::ffi::c_int + i) as usize]
            .wrapping_add(
                temp[(2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + i) as usize],
            );
        let z1: ::core::ffi::c_uint = temp[(2 as ::core::ffi::c_int
                * 0 as ::core::ffi::c_int + i) as usize]
            .wrapping_sub(
                temp[(2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + i) as usize],
            );
        let z2: ::core::ffi::c_uint = temp[(2 as ::core::ffi::c_int
                * 1 as ::core::ffi::c_int + i) as usize]
            .wrapping_sub(
                temp[(2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int + i) as usize],
            );
        let z3: ::core::ffi::c_uint = temp[(2 as ::core::ffi::c_int
                * 1 as ::core::ffi::c_int + i) as usize]
            .wrapping_add(
                temp[(2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int + i) as usize],
            );
        *block.offset((stride * 0 as ::core::ffi::c_int + offset) as isize) = (z0
            .wrapping_add(z3)
            .wrapping_mul(qmul as ::core::ffi::c_uint)
            .wrapping_add(128 as ::core::ffi::c_uint) as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as int16_t;
        *block.offset((stride * 1 as ::core::ffi::c_int + offset) as isize) = (z1
            .wrapping_add(z2)
            .wrapping_mul(qmul as ::core::ffi::c_uint)
            .wrapping_add(128 as ::core::ffi::c_uint) as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as int16_t;
        *block.offset((stride * 2 as ::core::ffi::c_int + offset) as isize) = (z1
            .wrapping_sub(z2)
            .wrapping_mul(qmul as ::core::ffi::c_uint)
            .wrapping_add(128 as ::core::ffi::c_uint) as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as int16_t;
        *block.offset((stride * 3 as ::core::ffi::c_int + offset) as isize) = (z0
            .wrapping_sub(z3)
            .wrapping_mul(qmul as ::core::ffi::c_uint)
            .wrapping_add(128 as ::core::ffi::c_uint) as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int) as int16_t;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ff_h264_chroma_dc_dequant_idct_8_c(
    mut _block: *mut int16_t,
    mut qmul: ::core::ffi::c_int,
) {
    let stride: ::core::ffi::c_int = 16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    let xStride: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    let mut a: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut c: ::core::ffi::c_uint = 0;
    let mut d: ::core::ffi::c_uint = 0;
    let mut e: ::core::ffi::c_uint = 0;
    let mut block: *mut int16_t = _block;
    a = *block
        .offset(
            (stride * 0 as ::core::ffi::c_int + xStride * 0 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_uint;
    b = *block
        .offset(
            (stride * 0 as ::core::ffi::c_int + xStride * 1 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_uint;
    c = *block
        .offset(
            (stride * 1 as ::core::ffi::c_int + xStride * 0 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_uint;
    d = *block
        .offset(
            (stride * 1 as ::core::ffi::c_int + xStride * 1 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_uint;
    e = a.wrapping_sub(b);
    a = a.wrapping_add(b);
    b = c.wrapping_sub(d);
    c = c.wrapping_add(d);
    *block
        .offset(
            (stride * 0 as ::core::ffi::c_int + xStride * 0 as ::core::ffi::c_int)
                as isize,
        ) = (a.wrapping_add(c).wrapping_mul(qmul as ::core::ffi::c_uint)
        as ::core::ffi::c_int >> 7 as ::core::ffi::c_int) as int16_t;
    *block
        .offset(
            (stride * 0 as ::core::ffi::c_int + xStride * 1 as ::core::ffi::c_int)
                as isize,
        ) = (e.wrapping_add(b).wrapping_mul(qmul as ::core::ffi::c_uint)
        as ::core::ffi::c_int >> 7 as ::core::ffi::c_int) as int16_t;
    *block
        .offset(
            (stride * 1 as ::core::ffi::c_int + xStride * 0 as ::core::ffi::c_int)
                as isize,
        ) = (a.wrapping_sub(c).wrapping_mul(qmul as ::core::ffi::c_uint)
        as ::core::ffi::c_int >> 7 as ::core::ffi::c_int) as int16_t;
    *block
        .offset(
            (stride * 1 as ::core::ffi::c_int + xStride * 1 as ::core::ffi::c_int)
                as isize,
        ) = (e.wrapping_sub(b).wrapping_mul(qmul as ::core::ffi::c_uint)
        as ::core::ffi::c_int >> 7 as ::core::ffi::c_int) as int16_t;
}
