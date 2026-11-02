// Translated from FFmpeg 9.0.1 libavcodec/simple_idct.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Simple IDCT
//
// Copyright (c) 2001 Michael Niedermayer <michaelni@gmx.at>
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

pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union av_alias64 {
    pub u64_0: uint64_t,
    pub u32_0: [uint32_t; 2],
    pub u16_0: [uint16_t; 4],
    pub u8_0: [uint8_t; 8],
    pub f64_0: ::core::ffi::c_double,
    pub f32_0: [::core::ffi::c_float; 2],
}
pub type ptrdiff_t = ::core::ffi::c_long;
#[inline]
unsafe extern "C" fn idct4col_put(
    mut dest: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut col: *const int16_t,
) {
    let mut c0: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut c3: ::core::ffi::c_int = 0;
    let mut a0: ::core::ffi::c_int = 0;
    let mut a1: ::core::ffi::c_int = 0;
    let mut a2: ::core::ffi::c_int = 0;
    let mut a3: ::core::ffi::c_int = 0;
    a0 = *col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    a1 = *col.offset((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    a2 = *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    a3 = *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    c0 = (a0 + a2)
        * ((1 as ::core::ffi::c_int)
            << 12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        + ((1 as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
    c2 = (a0 - a2)
        * ((1 as ::core::ffi::c_int)
            << 12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        + ((1 as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
    c1 = a1
        * (0.6532814824f64
            * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        + a3
            * (0.2705980501f64
                * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int;
    c3 = a1
        * (0.2705980501f64
            * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        - a3
            * (0.6532814824f64
                * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int;
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        c0 + c1
            >> 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        c2 + c3
            >> 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        c2 - c3
            >> 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        c0 - c1
            >> 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct248_put(
    mut dest: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block: *mut int16_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ptr: *mut int16_t = ::core::ptr::null_mut::<int16_t>();
    ptr = block;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut a0: ::core::ffi::c_int = 0;
        let mut a1: ::core::ffi::c_int = 0;
        a0 = *ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        a1 = *ptr.offset((8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        *ptr.offset(0 as ::core::ffi::c_int as isize) = (a0 + a1) as int16_t;
        *ptr.offset((8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) = (a0
            - a1) as int16_t;
        let mut a0_0: ::core::ffi::c_int = 0;
        let mut a1_0: ::core::ffi::c_int = 0;
        a0_0 = *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        a1_0 = *ptr.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        *ptr.offset(1 as ::core::ffi::c_int as isize) = (a0_0 + a1_0) as int16_t;
        *ptr.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) = (a0_0
            - a1_0) as int16_t;
        let mut a0_1: ::core::ffi::c_int = 0;
        let mut a1_1: ::core::ffi::c_int = 0;
        a0_1 = *ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        a1_1 = *ptr.offset((8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        *ptr.offset(2 as ::core::ffi::c_int as isize) = (a0_1 + a1_1) as int16_t;
        *ptr.offset((8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) = (a0_1
            - a1_1) as int16_t;
        let mut a0_2: ::core::ffi::c_int = 0;
        let mut a1_2: ::core::ffi::c_int = 0;
        a0_2 = *ptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        a1_2 = *ptr.offset((8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        *ptr.offset(3 as ::core::ffi::c_int as isize) = (a0_2 + a1_2) as int16_t;
        *ptr.offset((8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) = (a0_2
            - a1_2) as int16_t;
        let mut a0_3: ::core::ffi::c_int = 0;
        let mut a1_3: ::core::ffi::c_int = 0;
        a0_3 = *ptr.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        a1_3 = *ptr.offset((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        *ptr.offset(4 as ::core::ffi::c_int as isize) = (a0_3 + a1_3) as int16_t;
        *ptr.offset((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize) = (a0_3
            - a1_3) as int16_t;
        let mut a0_4: ::core::ffi::c_int = 0;
        let mut a1_4: ::core::ffi::c_int = 0;
        a0_4 = *ptr.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        a1_4 = *ptr.offset((8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        *ptr.offset(5 as ::core::ffi::c_int as isize) = (a0_4 + a1_4) as int16_t;
        *ptr.offset((8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize) = (a0_4
            - a1_4) as int16_t;
        let mut a0_5: ::core::ffi::c_int = 0;
        let mut a1_5: ::core::ffi::c_int = 0;
        a0_5 = *ptr.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        a1_5 = *ptr.offset((8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        *ptr.offset(6 as ::core::ffi::c_int as isize) = (a0_5 + a1_5) as int16_t;
        *ptr.offset((8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize) = (a0_5
            - a1_5) as int16_t;
        let mut a0_6: ::core::ffi::c_int = 0;
        let mut a1_6: ::core::ffi::c_int = 0;
        a0_6 = *ptr.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        a1_6 = *ptr.offset((8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        *ptr.offset(7 as ::core::ffi::c_int as isize) = (a0_6 + a1_6) as int16_t;
        *ptr.offset((8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize) = (a0_6
            - a1_6) as int16_t;
        ptr = ptr.offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_8bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idct4col_put(
            dest.offset(i as isize),
            2 as ptrdiff_t * line_size,
            block.offset(i as isize),
        );
        idct4col_put(
            dest.offset(line_size as isize).offset(i as isize),
            2 as ptrdiff_t * line_size,
            block.offset(8 as ::core::ffi::c_int as isize).offset(i as isize),
        );
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn idctRowCondDC_int16_10bit(
    mut row: *mut int16_t,
    mut extra_shift: ::core::ffi::c_int,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    if (*(row as *const av_alias64)).u64_0 as ::core::ffi::c_ulonglong
        & !((0xffff as ::core::ffi::c_ulonglong)
            << 48 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
        | (*(row.offset(4 as ::core::ffi::c_int as isize) as *const av_alias64)).u64_0
            as ::core::ffi::c_ulonglong == 0 as ::core::ffi::c_ulonglong
    {
        let mut temp: uint64_t = 0;
        if 2 as ::core::ffi::c_int - extra_shift >= 0 as ::core::ffi::c_int {
            temp = (*row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int - extra_shift)
                & 0xffff as ::core::ffi::c_int) as uint64_t;
        } else {
            temp = (*row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << extra_shift - 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                >> extra_shift - 2 as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int)
                as uint64_t;
        }
        temp = temp
            .wrapping_add(
                temp
                    .wrapping_mul(
                        ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                            as uint64_t,
                    ),
            );
        temp = temp
            .wrapping_add(
                temp
                    .wrapping_mul(
                        (1 as ::core::ffi::c_int as uint64_t) << 32 as ::core::ffi::c_int,
                    ),
            );
        (*(row as *mut av_alias64)).u64_0 = temp;
        (*(row.offset(4 as ::core::ffi::c_int as isize) as *mut av_alias64)).u64_0 = temp;
        return;
    }
    a0 = (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        )
        .wrapping_add(
            ((1 as ::core::ffi::c_int)
                << 12 as ::core::ffi::c_int + extra_shift - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_sub(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_sub(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b1 = (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b2 = (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b3 = (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    if (*(row.offset(4 as ::core::ffi::c_int as isize) as *const av_alias64)).u64_0 != 0
    {
        a0 = a0
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_sub(
                        (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_sub(
                        (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
    }
    *row.offset(0 as ::core::ffi::c_int as isize) = (a0.wrapping_add(b0)
        as ::core::ffi::c_int >> 12 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(7 as ::core::ffi::c_int as isize) = (a0.wrapping_sub(b0)
        as ::core::ffi::c_int >> 12 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(1 as ::core::ffi::c_int as isize) = (a1.wrapping_add(b1)
        as ::core::ffi::c_int >> 12 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(6 as ::core::ffi::c_int as isize) = (a1.wrapping_sub(b1)
        as ::core::ffi::c_int >> 12 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(2 as ::core::ffi::c_int as isize) = (a2.wrapping_add(b2)
        as ::core::ffi::c_int >> 12 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(5 as ::core::ffi::c_int as isize) = (a2.wrapping_sub(b2)
        as ::core::ffi::c_int >> 12 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(3 as ::core::ffi::c_int as isize) = (a3.wrapping_add(b3)
        as ::core::ffi::c_int >> 12 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(4 as ::core::ffi::c_int as isize) = (a3.wrapping_sub(b3)
        as ::core::ffi::c_int >> 12 as ::core::ffi::c_int + extra_shift) as int16_t;
}
#[inline]
unsafe extern "C" fn idctRowCondDC_int16_12bit(
    mut row: *mut int16_t,
    mut extra_shift: ::core::ffi::c_int,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    if (*(row as *const av_alias64)).u64_0 as ::core::ffi::c_ulonglong
        & !((0xffff as ::core::ffi::c_ulonglong)
            << 48 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
        | (*(row.offset(4 as ::core::ffi::c_int as isize) as *const av_alias64)).u64_0
            as ::core::ffi::c_ulonglong == 0 as ::core::ffi::c_ulonglong
    {
        let mut temp: uint64_t = 0;
        if -(1 as ::core::ffi::c_int) - extra_shift >= 0 as ::core::ffi::c_int {
            temp = (*row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * ((1 as ::core::ffi::c_int) << -(1 as ::core::ffi::c_int) - extra_shift)
                & 0xffff as ::core::ffi::c_int) as uint64_t;
        } else {
            temp = (*row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << extra_shift - -(1 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)
                >> extra_shift - -(1 as ::core::ffi::c_int)
                & 0xffff as ::core::ffi::c_int) as uint64_t;
        }
        temp = temp
            .wrapping_add(
                temp
                    .wrapping_mul(
                        ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                            as uint64_t,
                    ),
            );
        temp = temp
            .wrapping_add(
                temp
                    .wrapping_mul(
                        (1 as ::core::ffi::c_int as uint64_t) << 32 as ::core::ffi::c_int,
                    ),
            );
        (*(row as *mut av_alias64)).u64_0 = temp;
        (*(row.offset(4 as ::core::ffi::c_int as isize) as *mut av_alias64)).u64_0 = temp;
        return;
    }
    a0 = (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        )
        .wrapping_add(
            ((1 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int + extra_shift - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_sub(
            (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_sub(
            (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b0 = (45451 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b1 = (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = b1
        .wrapping_add(
            (-(9041 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b2 = (25746 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = b2
        .wrapping_add(
            (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b3 = (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = b3
        .wrapping_add(
            (-(25746 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    if (*(row.offset(4 as ::core::ffi::c_int as isize) as *const av_alias64)).u64_0 != 0
    {
        a0 = a0
            .wrapping_add(
                (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(32767 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_sub(
                        (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(32767 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a3 = a3
            .wrapping_add(
                (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_sub(
                        (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        b0 = b0
            .wrapping_add(
                (25746 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b0 = b0
            .wrapping_add(
                (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(25746 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
    }
    *row.offset(0 as ::core::ffi::c_int as isize) = (a0.wrapping_add(b0)
        as ::core::ffi::c_int >> 16 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(7 as ::core::ffi::c_int as isize) = (a0.wrapping_sub(b0)
        as ::core::ffi::c_int >> 16 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(1 as ::core::ffi::c_int as isize) = (a1.wrapping_add(b1)
        as ::core::ffi::c_int >> 16 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(6 as ::core::ffi::c_int as isize) = (a1.wrapping_sub(b1)
        as ::core::ffi::c_int >> 16 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(2 as ::core::ffi::c_int as isize) = (a2.wrapping_add(b2)
        as ::core::ffi::c_int >> 16 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(5 as ::core::ffi::c_int as isize) = (a2.wrapping_sub(b2)
        as ::core::ffi::c_int >> 16 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(3 as ::core::ffi::c_int as isize) = (a3.wrapping_add(b3)
        as ::core::ffi::c_int >> 16 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(4 as ::core::ffi::c_int as isize) = (a3.wrapping_sub(b3)
        as ::core::ffi::c_int >> 16 as ::core::ffi::c_int + extra_shift) as int16_t;
}
#[inline]
unsafe extern "C" fn idctRowCondDC_int16_8bit(
    mut row: *mut int16_t,
    mut extra_shift: ::core::ffi::c_int,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    if (*(row as *const av_alias64)).u64_0 as ::core::ffi::c_ulonglong
        & !((0xffff as ::core::ffi::c_ulonglong)
            << 48 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
        | (*(row.offset(4 as ::core::ffi::c_int as isize) as *const av_alias64)).u64_0
            as ::core::ffi::c_ulonglong == 0 as ::core::ffi::c_ulonglong
    {
        let mut temp: uint64_t = 0;
        if 3 as ::core::ffi::c_int - extra_shift >= 0 as ::core::ffi::c_int {
            temp = (*row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int - extra_shift)
                & 0xffff as ::core::ffi::c_int) as uint64_t;
        } else {
            temp = (*row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << extra_shift - 3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                >> extra_shift - 3 as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int)
                as uint64_t;
        }
        temp = temp
            .wrapping_add(
                temp
                    .wrapping_mul(
                        ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                            as uint64_t,
                    ),
            );
        temp = temp
            .wrapping_add(
                temp
                    .wrapping_mul(
                        (1 as ::core::ffi::c_int as uint64_t) << 32 as ::core::ffi::c_int,
                    ),
            );
        (*(row as *mut av_alias64)).u64_0 = temp;
        (*(row.offset(4 as ::core::ffi::c_int as isize) as *mut av_alias64)).u64_0 = temp;
        return;
    }
    a0 = (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        )
        .wrapping_add(
            ((1 as ::core::ffi::c_int)
                << 11 as ::core::ffi::c_int + extra_shift - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_sub(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_sub(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int
        * *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19266 as ::core::ffi::c_int
                * *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    b1 = (19266 as ::core::ffi::c_int
        * *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int)
                * *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    b2 = (12873 as ::core::ffi::c_int
        * *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int)
                * *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    b3 = (4520 as ::core::ffi::c_int
        * *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int)
                * *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    if (*(row.offset(4 as ::core::ffi::c_int as isize) as *const av_alias64)).u64_0 != 0
    {
        a0 = a0
            .wrapping_add(
                (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16383 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_sub(
                        (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16383 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_sub(
                        (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int
                    * *row.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int
                    * *row.offset(7 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int)
                    * *row.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int)
                    * *row.offset(7 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int
                    * *row.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b2 = b2
            .wrapping_add(
                (19266 as ::core::ffi::c_int
                    * *row.offset(7 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b3 = b3
            .wrapping_add(
                (19266 as ::core::ffi::c_int
                    * *row.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int)
                    * *row.offset(7 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
    }
    *row.offset(0 as ::core::ffi::c_int as isize) = (a0.wrapping_add(b0)
        as ::core::ffi::c_int >> 11 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(7 as ::core::ffi::c_int as isize) = (a0.wrapping_sub(b0)
        as ::core::ffi::c_int >> 11 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(1 as ::core::ffi::c_int as isize) = (a1.wrapping_add(b1)
        as ::core::ffi::c_int >> 11 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(6 as ::core::ffi::c_int as isize) = (a1.wrapping_sub(b1)
        as ::core::ffi::c_int >> 11 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(2 as ::core::ffi::c_int as isize) = (a2.wrapping_add(b2)
        as ::core::ffi::c_int >> 11 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(5 as ::core::ffi::c_int as isize) = (a2.wrapping_sub(b2)
        as ::core::ffi::c_int >> 11 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(3 as ::core::ffi::c_int as isize) = (a3.wrapping_add(b3)
        as ::core::ffi::c_int >> 11 as ::core::ffi::c_int + extra_shift) as int16_t;
    *row.offset(4 as ::core::ffi::c_int as isize) = (a3.wrapping_sub(b3)
        as ::core::ffi::c_int >> 11 as ::core::ffi::c_int + extra_shift) as int16_t;
}
#[inline]
unsafe extern "C" fn idctRowCondDC_int32_10bit(
    mut row: *mut int32_t,
    mut extra_shift: ::core::ffi::c_int,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        )
        .wrapping_add(
            ((1 as ::core::ffi::c_int)
                << 13 as ::core::ffi::c_int + extra_shift - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_sub(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_sub(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b1 = (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b2 = (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    b3 = (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ),
        );
    if (*(row.offset(4 as ::core::ffi::c_int as isize) as *const av_alias64)).u64_0
        | (*(row.offset(6 as ::core::ffi::c_int as isize) as *const av_alias64)).u64_0
        != 0
    {
        a0 = a0
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_sub(
                        (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    )
                    .wrapping_sub(
                        (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            .wrapping_mul(
                                *row.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint,
                            ),
                    ),
            );
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *row.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint,
                    ),
            );
    }
    *row.offset(0 as ::core::ffi::c_int as isize) = (a0.wrapping_add(b0)
        as ::core::ffi::c_int >> 13 as ::core::ffi::c_int + extra_shift) as int32_t;
    *row.offset(7 as ::core::ffi::c_int as isize) = (a0.wrapping_sub(b0)
        as ::core::ffi::c_int >> 13 as ::core::ffi::c_int + extra_shift) as int32_t;
    *row.offset(1 as ::core::ffi::c_int as isize) = (a1.wrapping_add(b1)
        as ::core::ffi::c_int >> 13 as ::core::ffi::c_int + extra_shift) as int32_t;
    *row.offset(6 as ::core::ffi::c_int as isize) = (a1.wrapping_sub(b1)
        as ::core::ffi::c_int >> 13 as ::core::ffi::c_int + extra_shift) as int32_t;
    *row.offset(2 as ::core::ffi::c_int as isize) = (a2.wrapping_add(b2)
        as ::core::ffi::c_int >> 13 as ::core::ffi::c_int + extra_shift) as int32_t;
    *row.offset(5 as ::core::ffi::c_int as isize) = (a2.wrapping_sub(b2)
        as ::core::ffi::c_int >> 13 as ::core::ffi::c_int + extra_shift) as int32_t;
    *row.offset(3 as ::core::ffi::c_int as isize) = (a3.wrapping_add(b3)
        as ::core::ffi::c_int >> 13 as ::core::ffi::c_int + extra_shift) as int32_t;
    *row.offset(4 as ::core::ffi::c_int as isize) = (a3.wrapping_sub(b3)
        as ::core::ffi::c_int >> 13 as ::core::ffi::c_int + extra_shift) as int32_t;
}
#[inline]
unsafe extern "C" fn idct4col_add(
    mut dest: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut col: *const int16_t,
) {
    let mut c0: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut c3: ::core::ffi::c_int = 0;
    let mut a0: ::core::ffi::c_int = 0;
    let mut a1: ::core::ffi::c_int = 0;
    let mut a2: ::core::ffi::c_int = 0;
    let mut a3: ::core::ffi::c_int = 0;
    a0 = *col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    a1 = *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    a2 = *col.offset((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    a3 = *col.offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    c0 = (a0 + a2)
        * (0.5f64 * 1.41421356237309504880f64
            * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
    c2 = (a0 - a2)
        * (0.5f64 * 1.41421356237309504880f64
            * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
    c1 = a1
        * (0.6532814824f64 * 1.41421356237309504880f64
            * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        + a3
            * (0.2705980501f64 * 1.41421356237309504880f64
                * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int;
    c3 = a1
        * (0.2705980501f64 * 1.41421356237309504880f64
            * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        - a3
            * (0.6532814824f64 * 1.41421356237309504880f64
                * ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int;
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (c0 + c1
                >> 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (c2 + c3
                >> 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (c2 - c3
                >> 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (c0 - c1
                >> 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int),
    );
}
#[inline]
unsafe extern "C" fn idct4row(mut row: *mut int16_t) {
    let mut c0: ::core::ffi::c_uint = 0;
    let mut c1: ::core::ffi::c_uint = 0;
    let mut c2: ::core::ffi::c_uint = 0;
    let mut c3: ::core::ffi::c_uint = 0;
    let mut a0: ::core::ffi::c_int = 0;
    let mut a1: ::core::ffi::c_int = 0;
    let mut a2: ::core::ffi::c_int = 0;
    let mut a3: ::core::ffi::c_int = 0;
    a0 = *row.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    a1 = *row.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    a2 = *row.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    a3 = *row.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    c0 = ((a0 + a2)
        * (0.5f64 * 1.41421356237309504880f64
            * ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int)
            << 11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        as ::core::ffi::c_uint;
    c2 = ((a0 - a2)
        * (0.5f64 * 1.41421356237309504880f64
            * ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int)
            << 11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        as ::core::ffi::c_uint;
    c1 = (a1
        * (0.6532814824f64 * 1.41421356237309504880f64
            * ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        + a3
            * (0.2705980501f64 * 1.41421356237309504880f64
                * ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                    as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    c3 = (a1
        * (0.2705980501f64 * 1.41421356237309504880f64
            * ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int
        - a3
            * (0.6532814824f64 * 1.41421356237309504880f64
                * ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                    as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    *row.offset(0 as ::core::ffi::c_int as isize) = (c0.wrapping_add(c1)
        >> 11 as ::core::ffi::c_int) as int16_t;
    *row.offset(1 as ::core::ffi::c_int as isize) = (c2.wrapping_add(c3)
        >> 11 as ::core::ffi::c_int) as int16_t;
    *row.offset(2 as ::core::ffi::c_int as isize) = (c2.wrapping_sub(c3)
        >> 11 as ::core::ffi::c_int) as int16_t;
    *row.offset(3 as ::core::ffi::c_int as isize) = (c0.wrapping_sub(c1)
        >> 11 as ::core::ffi::c_int) as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct84_add(
    mut dest: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block: *mut int16_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        idctRowCondDC_int16_8bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idct4col_add(dest.offset(i as isize), line_size, block.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct48_add(
    mut dest: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block: *mut int16_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idct4row(block.offset((i * 8 as ::core::ffi::c_int) as isize));
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        idctSparseColAdd_int16_8bit(
            dest.offset(i as isize),
            line_size,
            block.offset(i as isize),
        );
        i += 1;
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
pub unsafe extern "C" fn ff_simple_idct44_add(
    mut dest: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block: *mut int16_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        idct4row(block.offset((i * 8 as ::core::ffi::c_int) as isize));
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        idct4col_add(dest.offset(i as isize), line_size, block.offset(i as isize));
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn idctSparseCol_int16_12bit(mut col: *mut int16_t) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 32767 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(17734 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(42813 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (45451 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = (25746 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b1 = b1
        .wrapping_add(
            (-(9041 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b2 = b2
        .wrapping_add(
            (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b3 = b3
        .wrapping_add(
            (-(25746 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(32767 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(32767 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (25746 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(42813 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(17734 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(25746 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    *col.offset(0 as ::core::ffi::c_int as isize) = (a0.wrapping_add(b0)
        as ::core::ffi::c_int >> 17 as ::core::ffi::c_int) as int16_t;
    *col.offset(8 as ::core::ffi::c_int as isize) = (a1.wrapping_add(b1)
        as ::core::ffi::c_int >> 17 as ::core::ffi::c_int) as int16_t;
    *col.offset(16 as ::core::ffi::c_int as isize) = (a2.wrapping_add(b2)
        as ::core::ffi::c_int >> 17 as ::core::ffi::c_int) as int16_t;
    *col.offset(24 as ::core::ffi::c_int as isize) = (a3.wrapping_add(b3)
        as ::core::ffi::c_int >> 17 as ::core::ffi::c_int) as int16_t;
    *col.offset(32 as ::core::ffi::c_int as isize) = (a3.wrapping_sub(b3)
        as ::core::ffi::c_int >> 17 as ::core::ffi::c_int) as int16_t;
    *col.offset(40 as ::core::ffi::c_int as isize) = (a2.wrapping_sub(b2)
        as ::core::ffi::c_int >> 17 as ::core::ffi::c_int) as int16_t;
    *col.offset(48 as ::core::ffi::c_int as isize) = (a1.wrapping_sub(b1)
        as ::core::ffi::c_int >> 17 as ::core::ffi::c_int) as int16_t;
    *col.offset(56 as ::core::ffi::c_int as isize) = (a0.wrapping_sub(b0)
        as ::core::ffi::c_int >> 17 as ::core::ffi::c_int) as int16_t;
}
#[inline]
unsafe extern "C" fn idctSparseCol_int16_10bit(mut col: *mut int16_t) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 16384 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    *col.offset(0 as ::core::ffi::c_int as isize) = (a0.wrapping_add(b0)
        as ::core::ffi::c_int >> 19 as ::core::ffi::c_int) as int16_t;
    *col.offset(8 as ::core::ffi::c_int as isize) = (a1.wrapping_add(b1)
        as ::core::ffi::c_int >> 19 as ::core::ffi::c_int) as int16_t;
    *col.offset(16 as ::core::ffi::c_int as isize) = (a2.wrapping_add(b2)
        as ::core::ffi::c_int >> 19 as ::core::ffi::c_int) as int16_t;
    *col.offset(24 as ::core::ffi::c_int as isize) = (a3.wrapping_add(b3)
        as ::core::ffi::c_int >> 19 as ::core::ffi::c_int) as int16_t;
    *col.offset(32 as ::core::ffi::c_int as isize) = (a3.wrapping_sub(b3)
        as ::core::ffi::c_int >> 19 as ::core::ffi::c_int) as int16_t;
    *col.offset(40 as ::core::ffi::c_int as isize) = (a2.wrapping_sub(b2)
        as ::core::ffi::c_int >> 19 as ::core::ffi::c_int) as int16_t;
    *col.offset(48 as ::core::ffi::c_int as isize) = (a1.wrapping_sub(b1)
        as ::core::ffi::c_int >> 19 as ::core::ffi::c_int) as int16_t;
    *col.offset(56 as ::core::ffi::c_int as isize) = (a0.wrapping_sub(b0)
        as ::core::ffi::c_int >> 19 as ::core::ffi::c_int) as int16_t;
}
#[inline]
unsafe extern "C" fn idctSparseCol_int16_8bit(mut col: *mut int16_t) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 16383 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b1 = (19266 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b2 = (12873 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b3 = (4520 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19266 as ::core::ffi::c_int
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int)
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int)
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int)
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16383 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16383 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int)
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b3 = b3
            .wrapping_add(
                (19266 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int)
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b2 = b2
            .wrapping_add(
                (19266 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int)
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
    }
    *col.offset(0 as ::core::ffi::c_int as isize) = (a0.wrapping_add(b0)
        as ::core::ffi::c_int >> 20 as ::core::ffi::c_int) as int16_t;
    *col.offset(8 as ::core::ffi::c_int as isize) = (a1.wrapping_add(b1)
        as ::core::ffi::c_int >> 20 as ::core::ffi::c_int) as int16_t;
    *col.offset(16 as ::core::ffi::c_int as isize) = (a2.wrapping_add(b2)
        as ::core::ffi::c_int >> 20 as ::core::ffi::c_int) as int16_t;
    *col.offset(24 as ::core::ffi::c_int as isize) = (a3.wrapping_add(b3)
        as ::core::ffi::c_int >> 20 as ::core::ffi::c_int) as int16_t;
    *col.offset(32 as ::core::ffi::c_int as isize) = (a3.wrapping_sub(b3)
        as ::core::ffi::c_int >> 20 as ::core::ffi::c_int) as int16_t;
    *col.offset(40 as ::core::ffi::c_int as isize) = (a2.wrapping_sub(b2)
        as ::core::ffi::c_int >> 20 as ::core::ffi::c_int) as int16_t;
    *col.offset(48 as ::core::ffi::c_int as isize) = (a1.wrapping_sub(b1)
        as ::core::ffi::c_int >> 20 as ::core::ffi::c_int) as int16_t;
    *col.offset(56 as ::core::ffi::c_int as isize) = (a0.wrapping_sub(b0)
        as ::core::ffi::c_int >> 20 as ::core::ffi::c_int) as int16_t;
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
#[inline]
unsafe extern "C" fn idctSparseColPut_int16_12bit(
    mut dest: *mut uint16_t,
    mut line_size: ptrdiff_t,
    mut col: *mut int16_t,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 32767 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(17734 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(42813 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (45451 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = (25746 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b1 = b1
        .wrapping_add(
            (-(9041 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b2 = b2
        .wrapping_add(
            (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b3 = b3
        .wrapping_add(
            (-(25746 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(32767 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(32767 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (25746 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(42813 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(17734 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(25746 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a0.wrapping_add(b0) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a1.wrapping_add(b1) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a2.wrapping_add(b2) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a3.wrapping_add(b3) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a3.wrapping_sub(b3) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a2.wrapping_sub(b2) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a1.wrapping_sub(b1) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a0.wrapping_sub(b0) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ) as uint16_t;
}
#[inline]
unsafe extern "C" fn idctSparseColPut_int32_10bit(
    mut dest: *mut uint16_t,
    mut line_size: ptrdiff_t,
    mut col: *mut int32_t,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 21 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 16384 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a0.wrapping_add(b0) as ::core::ffi::c_int >> 21 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a1.wrapping_add(b1) as ::core::ffi::c_int >> 21 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a2.wrapping_add(b2) as ::core::ffi::c_int >> 21 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a3.wrapping_add(b3) as ::core::ffi::c_int >> 21 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a3.wrapping_sub(b3) as ::core::ffi::c_int >> 21 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a2.wrapping_sub(b2) as ::core::ffi::c_int >> 21 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a1.wrapping_sub(b1) as ::core::ffi::c_int >> 21 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a0.wrapping_sub(b0) as ::core::ffi::c_int >> 21 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
}
#[inline]
unsafe extern "C" fn idctSparseColPut_int16_8bit(
    mut dest: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut col: *mut int16_t,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 16383 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b1 = (19266 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b2 = (12873 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b3 = (4520 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19266 as ::core::ffi::c_int
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int)
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int)
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int)
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16383 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16383 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int)
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b3 = b3
            .wrapping_add(
                (19266 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int)
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b2 = b2
            .wrapping_add(
                (19266 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int)
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
    }
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        a0.wrapping_add(b0) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        a1.wrapping_add(b1) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        a2.wrapping_add(b2) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        a3.wrapping_add(b3) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        a3.wrapping_sub(b3) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        a2.wrapping_sub(b2) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        a1.wrapping_sub(b1) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int,
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        a0.wrapping_sub(b0) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn idctSparseColPut_int16_10bit(
    mut dest: *mut uint16_t,
    mut line_size: ptrdiff_t,
    mut col: *mut int16_t,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 16384 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a0.wrapping_add(b0) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a1.wrapping_add(b1) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a2.wrapping_add(b2) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a3.wrapping_add(b3) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a3.wrapping_sub(b3) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a2.wrapping_sub(b2) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a1.wrapping_sub(b1) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        a0.wrapping_sub(b0) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
    ) as uint16_t;
}
#[inline]
unsafe extern "C" fn idctSparseColAdd_int16_10bit(
    mut dest: *mut uint16_t,
    mut line_size: ptrdiff_t,
    mut col: *mut int16_t,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 16384 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16384 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16384 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (19265 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a0.wrapping_add(b0) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int),
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a1.wrapping_add(b1) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int),
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a2.wrapping_add(b2) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int),
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a3.wrapping_add(b3) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int),
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a3.wrapping_sub(b3) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int),
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a2.wrapping_sub(b2) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int),
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a1.wrapping_sub(b1) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int),
        10 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a0.wrapping_sub(b0) as ::core::ffi::c_int >> 19 as ::core::ffi::c_int),
        10 as ::core::ffi::c_int,
    ) as uint16_t;
}
#[inline]
unsafe extern "C" fn idctSparseColAdd_int16_12bit(
    mut dest: *mut uint16_t,
    mut line_size: ptrdiff_t,
    mut col: *mut int16_t,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 32767 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(17734 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(42813 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (45451 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b1 = (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b2 = (25746 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b3 = (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b1 = b1
        .wrapping_add(
            (-(9041 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b2 = b2
        .wrapping_add(
            (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b3 = b3
        .wrapping_add(
            (-(25746 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(32767 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(32767 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (32767 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (25746 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (17734 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(42813 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (42813 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(17734 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (9041 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b1 = b1
            .wrapping_add(
                (-(25746 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b2 = b2
            .wrapping_add(
                (38531 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        b3 = b3
            .wrapping_add(
                (-(45451 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a0.wrapping_add(b0) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int),
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a1.wrapping_add(b1) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int),
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a2.wrapping_add(b2) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int),
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a3.wrapping_add(b3) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int),
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a3.wrapping_sub(b3) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int),
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a2.wrapping_sub(b2) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int),
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a1.wrapping_sub(b1) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int),
        12 as ::core::ffi::c_int,
    ) as uint16_t;
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uintp2_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a0.wrapping_sub(b0) as ::core::ffi::c_int >> 17 as ::core::ffi::c_int),
        12 as ::core::ffi::c_int,
    ) as uint16_t;
}
#[inline]
unsafe extern "C" fn idctSparseColAdd_int16_8bit(
    mut dest: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut col: *mut int16_t,
) {
    let mut a0: ::core::ffi::c_uint = 0;
    let mut a1: ::core::ffi::c_uint = 0;
    let mut a2: ::core::ffi::c_uint = 0;
    let mut a3: ::core::ffi::c_uint = 0;
    let mut b0: ::core::ffi::c_uint = 0;
    let mut b1: ::core::ffi::c_uint = 0;
    let mut b2: ::core::ffi::c_uint = 0;
    let mut b3: ::core::ffi::c_uint = 0;
    a0 = (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            (*col.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / 16383 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    a1 = a0;
    a2 = a0;
    a3 = a0;
    a0 = a0
        .wrapping_add(
            (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a1 = a1
        .wrapping_add(
            (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a2 = a2
        .wrapping_add(
            (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    a3 = a3
        .wrapping_add(
            (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_mul(
                    *col
                        .offset(
                            (8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_uint,
                ),
        );
    b0 = (22725 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b1 = (19266 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b2 = (12873 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b3 = (4520 as ::core::ffi::c_int
        * *col.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int) as ::core::ffi::c_uint;
    b0 = b0
        .wrapping_add(
            (19266 as ::core::ffi::c_int
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    b1 = b1
        .wrapping_add(
            (-(4520 as ::core::ffi::c_int)
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    b2 = b2
        .wrapping_add(
            (-(22725 as ::core::ffi::c_int)
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    b3 = b3
        .wrapping_add(
            (-(12873 as ::core::ffi::c_int)
                * *col
                    .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    if *col.offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(16383 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (-(16383 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (16383 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (12873 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b1 = b1
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int)
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b2 = b2
            .wrapping_add(
                (4520 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b3 = b3
            .wrapping_add(
                (19266 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize) != 0 {
        a0 = a0
            .wrapping_add(
                (8867 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a1 = a1
            .wrapping_add(
                (-(21407 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a2 = a2
            .wrapping_add(
                (21407 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
        a3 = a3
            .wrapping_add(
                (-(8867 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_mul(
                        *col
                            .offset(
                                (8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_uint,
                    ),
            );
    }
    if *col.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize) != 0 {
        b0 = b0
            .wrapping_add(
                (4520 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b1 = b1
            .wrapping_add(
                (-(12873 as ::core::ffi::c_int)
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b2 = b2
            .wrapping_add(
                (19266 as ::core::ffi::c_int
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        b3 = b3
            .wrapping_add(
                (-(22725 as ::core::ffi::c_int)
                    * *col
                        .offset(
                            (8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
    }
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a0.wrapping_add(b0) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a1.wrapping_add(b1) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a2.wrapping_add(b2) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a3.wrapping_add(b3) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a3.wrapping_sub(b3) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a2.wrapping_sub(b2) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a1.wrapping_sub(b1) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int),
    );
    dest = dest.offset(line_size as isize);
    *dest.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
        *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + (a0.wrapping_sub(b0) as ::core::ffi::c_int >> 20 as ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_put_int16_12bit(
    mut dest_: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block_: *mut int16_t,
) {
    let mut block: *mut int16_t = block_;
    let mut dest: *mut uint16_t = dest_ as *mut uint16_t;
    let mut i: ::core::ffi::c_int = 0;
    line_size = (line_size as ::core::ffi::c_ulong)
        .wrapping_div(
            ::core::mem::size_of::<uint16_t>() as usize as ::core::ffi::c_ulong,
        ) as ptrdiff_t as ptrdiff_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_12bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseColPut_int16_12bit(
            dest.offset(i as isize),
            line_size,
            block.offset(i as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_put_int32_10bit(
    mut dest_: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block_: *mut int16_t,
) {
    let mut block: *mut int32_t = block_ as *mut int32_t;
    let mut dest: *mut uint16_t = dest_ as *mut uint16_t;
    let mut i: ::core::ffi::c_int = 0;
    line_size = (line_size as ::core::ffi::c_ulong)
        .wrapping_div(
            ::core::mem::size_of::<uint16_t>() as usize as ::core::ffi::c_ulong,
        ) as ptrdiff_t as ptrdiff_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int32_10bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseColPut_int32_10bit(
            dest.offset(i as isize),
            line_size,
            block.offset(i as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_put_int16_8bit(
    mut dest_: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block_: *mut int16_t,
) {
    let mut block: *mut int16_t = block_;
    let mut dest: *mut uint8_t = dest_;
    let mut i: ::core::ffi::c_int = 0;
    line_size = (line_size as ::core::ffi::c_ulong)
        .wrapping_div(::core::mem::size_of::<uint8_t>() as usize as ::core::ffi::c_ulong)
        as ptrdiff_t as ptrdiff_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_8bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseColPut_int16_8bit(
            dest.offset(i as isize),
            line_size,
            block.offset(i as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_put_int16_10bit(
    mut dest_: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block_: *mut int16_t,
) {
    let mut block: *mut int16_t = block_;
    let mut dest: *mut uint16_t = dest_ as *mut uint16_t;
    let mut i: ::core::ffi::c_int = 0;
    line_size = (line_size as ::core::ffi::c_ulong)
        .wrapping_div(
            ::core::mem::size_of::<uint16_t>() as usize as ::core::ffi::c_ulong,
        ) as ptrdiff_t as ptrdiff_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_10bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseColPut_int16_10bit(
            dest.offset(i as isize),
            line_size,
            block.offset(i as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_add_int16_12bit(
    mut dest_: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block: *mut int16_t,
) {
    let mut dest: *mut uint16_t = dest_ as *mut uint16_t;
    let mut i: ::core::ffi::c_int = 0;
    line_size = (line_size as ::core::ffi::c_ulong)
        .wrapping_div(
            ::core::mem::size_of::<uint16_t>() as usize as ::core::ffi::c_ulong,
        ) as ptrdiff_t as ptrdiff_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_12bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseColAdd_int16_12bit(
            dest.offset(i as isize),
            line_size,
            block.offset(i as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_add_int16_8bit(
    mut dest_: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block: *mut int16_t,
) {
    let mut dest: *mut uint8_t = dest_;
    let mut i: ::core::ffi::c_int = 0;
    line_size = (line_size as ::core::ffi::c_ulong)
        .wrapping_div(::core::mem::size_of::<uint8_t>() as usize as ::core::ffi::c_ulong)
        as ptrdiff_t as ptrdiff_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_8bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseColAdd_int16_8bit(
            dest.offset(i as isize),
            line_size,
            block.offset(i as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_add_int16_10bit(
    mut dest_: *mut uint8_t,
    mut line_size: ptrdiff_t,
    mut block: *mut int16_t,
) {
    let mut dest: *mut uint16_t = dest_ as *mut uint16_t;
    let mut i: ::core::ffi::c_int = 0;
    line_size = (line_size as ::core::ffi::c_ulong)
        .wrapping_div(
            ::core::mem::size_of::<uint16_t>() as usize as ::core::ffi::c_ulong,
        ) as ptrdiff_t as ptrdiff_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_10bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseColAdd_int16_10bit(
            dest.offset(i as isize),
            line_size,
            block.offset(i as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_int16_10bit(mut block: *mut int16_t) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_10bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseCol_int16_10bit(block.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_int16_12bit(mut block: *mut int16_t) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_12bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseCol_int16_12bit(block.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_simple_idct_int16_8bit(mut block: *mut int16_t) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctRowCondDC_int16_8bit(
            block.offset((i * 8 as ::core::ffi::c_int) as isize),
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        idctSparseCol_int16_8bit(block.offset(i as isize));
        i += 1;
    }
}
