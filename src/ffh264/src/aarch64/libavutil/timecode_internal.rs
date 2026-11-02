// Translated from FFmpeg 9.0.1 libavutil/timecode_internal.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2006 Smartjog S.A.S, Baptiste Coudurier <baptiste.coudurier@gmail.com>
// Copyright (c) 2011-2012 Smartjog S.A.S, Clément Bœsch <clement.boesch@smartjog.com>
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

pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
unsafe extern "C" fn bcd2uint(mut bcd: uint8_t) -> ::core::ffi::c_uint {
    let mut low: ::core::ffi::c_uint = (bcd as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int) as ::core::ffi::c_uint;
    let mut high: ::core::ffi::c_uint = (bcd as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    if low > 9 as ::core::ffi::c_uint || high > 9 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_uint;
    }
    return low.wrapping_add((10 as ::core::ffi::c_uint).wrapping_mul(high));
}
#[no_mangle]
pub unsafe extern "C" fn ff_timecode_set_smpte(
    mut drop_0: *mut ::core::ffi::c_uint,
    mut hh: *mut ::core::ffi::c_uint,
    mut mm: *mut ::core::ffi::c_uint,
    mut ss: *mut ::core::ffi::c_uint,
    mut ff: *mut ::core::ffi::c_uint,
    mut rate: AVRational,
    mut tcsmpte: uint32_t,
    mut prevent_df: ::core::ffi::c_int,
    mut skip_field: ::core::ffi::c_int,
) {
    *hh = bcd2uint((tcsmpte & 0x3f as uint32_t) as uint8_t);
    *mm = bcd2uint((tcsmpte >> 8 as ::core::ffi::c_int & 0x7f as uint32_t) as uint8_t);
    *ss = bcd2uint((tcsmpte >> 16 as ::core::ffi::c_int & 0x7f as uint32_t) as uint8_t);
    *ff = bcd2uint((tcsmpte >> 24 as ::core::ffi::c_int & 0x3f as uint32_t) as uint8_t);
    *drop_0 = (tcsmpte
        & ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int) as uint32_t != 0
        && prevent_df == 0) as ::core::ffi::c_int as ::core::ffi::c_uint;
    if av_cmp_q(
        rate,
        AVRational {
            num: 30 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        },
    ) == 1 as ::core::ffi::c_int
    {
        *ff <<= 1 as ::core::ffi::c_int;
        if skip_field == 0 {
            if av_cmp_q(
                rate,
                AVRational {
                    num: 50 as ::core::ffi::c_int,
                    den: 1 as ::core::ffi::c_int,
                },
            ) == 0 as ::core::ffi::c_int
            {
                *ff = (*ff)
                    .wrapping_add(
                        (tcsmpte
                            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                                as uint32_t != 0) as ::core::ffi::c_int
                            as ::core::ffi::c_uint,
                    );
            } else {
                *ff = (*ff)
                    .wrapping_add(
                        (tcsmpte
                            & ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int)
                                as uint32_t != 0) as ::core::ffi::c_int
                            as ::core::ffi::c_uint,
                    );
            }
        }
    }
}
#[inline]
unsafe extern "C" fn av_cmp_q(
    mut a: AVRational,
    mut b: AVRational,
) -> ::core::ffi::c_int {
    let tmp: int64_t = a.num as int64_t * b.den as int64_t
        - b.num as int64_t * a.den as int64_t;
    if tmp != 0 {
        return ((tmp ^ a.den as int64_t ^ b.den as int64_t) >> 63 as ::core::ffi::c_int)
            as ::core::ffi::c_int | 1 as ::core::ffi::c_int
    } else if b.den != 0 && a.den != 0 {
        return 0 as ::core::ffi::c_int
    } else if a.num != 0 && b.num != 0 {
        return (a.num >> 31 as ::core::ffi::c_int) - (b.num >> 31 as ::core::ffi::c_int)
    } else {
        return -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    };
}
