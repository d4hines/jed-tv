// Translated from FFmpeg 9.0.1 libavutil/crc.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// copyright (c) 2006 Michael Niedermayer <michaelni@gmx.at>
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
    fn abort() -> !;
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
pub type intptr_t = ::core::ffi::c_int;
pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type size_t = ::core::ffi::c_uint;
pub type pthread_once_t = ::core::ffi::c_int;
pub type AVCRC = uint32_t;
pub type AVCRCId = ::core::ffi::c_uint;
pub const AV_CRC_MAX: AVCRCId = 8;
pub const AV_CRC_8_EBU: AVCRCId = 7;
pub const AV_CRC_24_IEEE: AVCRCId = 6;
pub const AV_CRC_16_ANSI_LE: AVCRCId = 5;
pub const AV_CRC_32_IEEE_LE: AVCRCId = 4;
pub const AV_CRC_32_IEEE: AVCRCId = 3;
pub const AV_CRC_16_CCITT: AVCRCId = 2;
pub const AV_CRC_16_ANSI: AVCRCId = 1;
pub const AV_CRC_8_ATM: AVCRCId = 0;
#[inline(always)]
unsafe extern "C" fn av_bswap32(mut x: uint32_t) -> uint32_t {
    return (x << 8 as ::core::ffi::c_int & 0xff00 as uint32_t
        | x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) << 16 as ::core::ffi::c_int
        | ((x >> 16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            & 0xff00 as uint32_t
            | x >> 16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as uint32_t);
}
static mut av_crc_table: [[AVCRC; 1024]; 8] = [[0; 1024]; 8];
static mut AV_CRC_8_ATM_once_control: pthread_once_t = 0 as pthread_once_t;
unsafe extern "C" fn AV_CRC_8_ATM_init_table_once() {
    if !(av_crc_init(
        &raw mut *(&raw mut av_crc_table as *mut [AVCRC; 1024])
            .offset(AV_CRC_8_ATM as ::core::ffi::c_int as isize) as *mut AVCRC,
        0 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        0x7 as uint32_t,
        ::core::mem::size_of::<[AVCRC; 1024]>() as ::core::ffi::c_int,
    ) >= 0 as ::core::ffi::c_int)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"av_crc_init(av_crc_table[AV_CRC_8_ATM], 0, 8, 0x07, sizeof(av_crc_table[AV_CRC_8_ATM])) >= 0\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/crc.c\0" as *const u8 as *const ::core::ffi::c_char,
            336 as ::core::ffi::c_int,
        );
        abort();
    }
}
static mut AV_CRC_8_EBU_once_control: pthread_once_t = 0 as pthread_once_t;
unsafe extern "C" fn AV_CRC_8_EBU_init_table_once() {
    if !(av_crc_init(
        &raw mut *(&raw mut av_crc_table as *mut [AVCRC; 1024])
            .offset(AV_CRC_8_EBU as ::core::ffi::c_int as isize) as *mut AVCRC,
        0 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        0x1d as uint32_t,
        ::core::mem::size_of::<[AVCRC; 1024]>() as ::core::ffi::c_int,
    ) >= 0 as ::core::ffi::c_int)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"av_crc_init(av_crc_table[AV_CRC_8_EBU], 0, 8, 0x1D, sizeof(av_crc_table[AV_CRC_8_EBU])) >= 0\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/crc.c\0" as *const u8 as *const ::core::ffi::c_char,
            337 as ::core::ffi::c_int,
        );
        abort();
    }
}
static mut AV_CRC_16_ANSI_once_control: pthread_once_t = 0 as pthread_once_t;
unsafe extern "C" fn AV_CRC_16_ANSI_init_table_once() {
    if !(av_crc_init(
        &raw mut *(&raw mut av_crc_table as *mut [AVCRC; 1024])
            .offset(AV_CRC_16_ANSI as ::core::ffi::c_int as isize) as *mut AVCRC,
        0 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
        0x8005 as uint32_t,
        ::core::mem::size_of::<[AVCRC; 1024]>() as ::core::ffi::c_int,
    ) >= 0 as ::core::ffi::c_int)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"av_crc_init(av_crc_table[AV_CRC_16_ANSI], 0, 16, 0x8005, sizeof(av_crc_table[AV_CRC_16_ANSI])) >= 0\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/crc.c\0" as *const u8 as *const ::core::ffi::c_char,
            338 as ::core::ffi::c_int,
        );
        abort();
    }
}
static mut AV_CRC_16_CCITT_once_control: pthread_once_t = 0 as pthread_once_t;
unsafe extern "C" fn AV_CRC_16_CCITT_init_table_once() {
    if !(av_crc_init(
        &raw mut *(&raw mut av_crc_table as *mut [AVCRC; 1024])
            .offset(AV_CRC_16_CCITT as ::core::ffi::c_int as isize) as *mut AVCRC,
        0 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
        0x1021 as uint32_t,
        ::core::mem::size_of::<[AVCRC; 1024]>() as ::core::ffi::c_int,
    ) >= 0 as ::core::ffi::c_int)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"av_crc_init(av_crc_table[AV_CRC_16_CCITT], 0, 16, 0x1021, sizeof(av_crc_table[AV_CRC_16_CCITT])) >= 0\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/crc.c\0" as *const u8 as *const ::core::ffi::c_char,
            339 as ::core::ffi::c_int,
        );
        abort();
    }
}
static mut AV_CRC_24_IEEE_once_control: pthread_once_t = 0 as pthread_once_t;
unsafe extern "C" fn AV_CRC_24_IEEE_init_table_once() {
    if !(av_crc_init(
        &raw mut *(&raw mut av_crc_table as *mut [AVCRC; 1024])
            .offset(AV_CRC_24_IEEE as ::core::ffi::c_int as isize) as *mut AVCRC,
        0 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        0x864cfb as ::core::ffi::c_int as uint32_t,
        ::core::mem::size_of::<[AVCRC; 1024]>() as ::core::ffi::c_int,
    ) >= 0 as ::core::ffi::c_int)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"av_crc_init(av_crc_table[AV_CRC_24_IEEE], 0, 24, 0x864CFB, sizeof(av_crc_table[AV_CRC_24_IEEE])) >= 0\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/crc.c\0" as *const u8 as *const ::core::ffi::c_char,
            340 as ::core::ffi::c_int,
        );
        abort();
    }
}
static mut AV_CRC_32_IEEE_once_control: pthread_once_t = 0 as pthread_once_t;
unsafe extern "C" fn AV_CRC_32_IEEE_init_table_once() {
    if !(av_crc_init(
        &raw mut *(&raw mut av_crc_table as *mut [AVCRC; 1024])
            .offset(AV_CRC_32_IEEE as ::core::ffi::c_int as isize) as *mut AVCRC,
        0 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
        0x4c11db7 as ::core::ffi::c_int as uint32_t,
        ::core::mem::size_of::<[AVCRC; 1024]>() as ::core::ffi::c_int,
    ) >= 0 as ::core::ffi::c_int)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"av_crc_init(av_crc_table[AV_CRC_32_IEEE], 0, 32, 0x04C11DB7, sizeof(av_crc_table[AV_CRC_32_IEEE])) >= 0\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/crc.c\0" as *const u8 as *const ::core::ffi::c_char,
            341 as ::core::ffi::c_int,
        );
        abort();
    }
}
static mut AV_CRC_32_IEEE_LE_once_control: pthread_once_t = 0 as pthread_once_t;
unsafe extern "C" fn AV_CRC_32_IEEE_LE_init_table_once() {
    if !(av_crc_init(
        &raw mut *(&raw mut av_crc_table as *mut [AVCRC; 1024])
            .offset(AV_CRC_32_IEEE_LE as ::core::ffi::c_int as isize) as *mut AVCRC,
        1 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
        0xedb88320 as uint32_t,
        ::core::mem::size_of::<[AVCRC; 1024]>() as ::core::ffi::c_int,
    ) >= 0 as ::core::ffi::c_int)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"av_crc_init(av_crc_table[AV_CRC_32_IEEE_LE], 1, 32, 0xEDB88320, sizeof(av_crc_table[AV_CRC_32_IEEE_LE])) >= 0\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/crc.c\0" as *const u8 as *const ::core::ffi::c_char,
            342 as ::core::ffi::c_int,
        );
        abort();
    }
}
static mut AV_CRC_16_ANSI_LE_once_control: pthread_once_t = 0 as pthread_once_t;
unsafe extern "C" fn AV_CRC_16_ANSI_LE_init_table_once() {
    if !(av_crc_init(
        &raw mut *(&raw mut av_crc_table as *mut [AVCRC; 1024])
            .offset(AV_CRC_16_ANSI_LE as ::core::ffi::c_int as isize) as *mut AVCRC,
        1 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
        0xa001 as uint32_t,
        ::core::mem::size_of::<[AVCRC; 1024]>() as ::core::ffi::c_int,
    ) >= 0 as ::core::ffi::c_int)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"av_crc_init(av_crc_table[AV_CRC_16_ANSI_LE], 1, 16, 0xA001, sizeof(av_crc_table[AV_CRC_16_ANSI_LE])) >= 0\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/crc.c\0" as *const u8 as *const ::core::ffi::c_char,
            343 as ::core::ffi::c_int,
        );
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_crc_init(
    mut ctx: *mut AVCRC,
    mut le: ::core::ffi::c_int,
    mut bits: ::core::ffi::c_int,
    mut poly: uint32_t,
    mut ctx_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    let mut c: uint32_t = 0;
    if bits < 8 as ::core::ffi::c_int || bits > 32 as ::core::ffi::c_int
        || poly as ::core::ffi::c_longlong >= (1 as ::core::ffi::c_longlong) << bits
    {
        return -(22 as ::core::ffi::c_int);
    }
    if ctx_size as usize
        != (::core::mem::size_of::<AVCRC>() as usize).wrapping_mul(257 as usize)
        && ctx_size as usize
            != (::core::mem::size_of::<AVCRC>() as usize).wrapping_mul(1024 as usize)
    {
        return -(22 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_uint;
    while i < 256 as ::core::ffi::c_uint {
        if le != 0 {
            c = i as uint32_t;
            j = 0 as ::core::ffi::c_uint;
            while j < 8 as ::core::ffi::c_uint {
                c = c >> 1 as ::core::ffi::c_int
                    ^ poly & (c & 1 as uint32_t).wrapping_neg();
                j = j.wrapping_add(1);
            }
            *ctx.offset(i as isize) = c as AVCRC;
        } else {
            c = (i << 24 as ::core::ffi::c_int) as uint32_t;
            j = 0 as ::core::ffi::c_uint;
            while j < 8 as ::core::ffi::c_uint {
                c = c << 1 as ::core::ffi::c_int
                    ^ poly << 32 as ::core::ffi::c_int - bits
                        & (c as int32_t >> 31 as ::core::ffi::c_int) as uint32_t;
                j = j.wrapping_add(1);
            }
            *ctx.offset(i as isize) = av_bswap32(c) as AVCRC;
        }
        i = i.wrapping_add(1);
    }
    *ctx.offset(256 as ::core::ffi::c_int as isize) = 1 as AVCRC;
    if ctx_size as usize
        >= (::core::mem::size_of::<AVCRC>() as usize).wrapping_mul(1024 as usize)
    {
        i = 0 as ::core::ffi::c_uint;
        while i < 256 as ::core::ffi::c_uint {
            j = 0 as ::core::ffi::c_uint;
            while j < 3 as ::core::ffi::c_uint {
                *ctx
                    .offset(
                        (256 as ::core::ffi::c_uint)
                            .wrapping_mul(j.wrapping_add(1 as ::core::ffi::c_uint))
                            .wrapping_add(i) as isize,
                    ) = *ctx
                    .offset(
                        (256 as ::core::ffi::c_uint).wrapping_mul(j).wrapping_add(i)
                            as isize,
                    ) >> 8 as ::core::ffi::c_int
                    ^ *ctx
                        .offset(
                            (*ctx
                                .offset(
                                    (256 as ::core::ffi::c_uint).wrapping_mul(j).wrapping_add(i)
                                        as isize,
                                ) & 0xff as AVCRC) as isize,
                        );
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_crc_get_table(mut crc_id: AVCRCId) -> *const AVCRC {
    if crc_id as ::core::ffi::c_uint
        >= AV_CRC_MAX as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<AVCRC>();
    }
    match crc_id as ::core::ffi::c_uint {
        0 => {
            pthread_once(
                &raw mut AV_CRC_8_ATM_once_control,
                Some(AV_CRC_8_ATM_init_table_once as unsafe extern "C" fn() -> ()),
            );
        }
        7 => {
            pthread_once(
                &raw mut AV_CRC_8_EBU_once_control,
                Some(AV_CRC_8_EBU_init_table_once as unsafe extern "C" fn() -> ()),
            );
        }
        1 => {
            pthread_once(
                &raw mut AV_CRC_16_ANSI_once_control,
                Some(AV_CRC_16_ANSI_init_table_once as unsafe extern "C" fn() -> ()),
            );
        }
        2 => {
            pthread_once(
                &raw mut AV_CRC_16_CCITT_once_control,
                Some(AV_CRC_16_CCITT_init_table_once as unsafe extern "C" fn() -> ()),
            );
        }
        6 => {
            pthread_once(
                &raw mut AV_CRC_24_IEEE_once_control,
                Some(AV_CRC_24_IEEE_init_table_once as unsafe extern "C" fn() -> ()),
            );
        }
        3 => {
            pthread_once(
                &raw mut AV_CRC_32_IEEE_once_control,
                Some(AV_CRC_32_IEEE_init_table_once as unsafe extern "C" fn() -> ()),
            );
        }
        4 => {
            pthread_once(
                &raw mut AV_CRC_32_IEEE_LE_once_control,
                Some(AV_CRC_32_IEEE_LE_init_table_once as unsafe extern "C" fn() -> ()),
            );
        }
        5 => {
            pthread_once(
                &raw mut AV_CRC_16_ANSI_LE_once_control,
                Some(AV_CRC_16_ANSI_LE_init_table_once as unsafe extern "C" fn() -> ()),
            );
        }
        _ => {
            unreachable!();
        }
    }
    return &raw mut *(&raw mut av_crc_table as *mut [AVCRC; 1024])
        .offset(crc_id as isize) as *mut AVCRC;
}
#[no_mangle]
pub unsafe extern "C" fn av_crc(
    mut ctx: *const AVCRC,
    mut crc: uint32_t,
    mut buffer: *const uint8_t,
    mut length: size_t,
) -> uint32_t {
    *ctx.offset(0 as ::core::ffi::c_int as isize) != 0;
    let mut end: *const uint8_t = buffer.offset(length as isize);
    if *ctx.offset(256 as ::core::ffi::c_int as isize) == 0 {
        while buffer as ::core::ffi::c_int & 3 as ::core::ffi::c_int != 0 && buffer < end
        {
            let fresh0 = buffer;
            buffer = buffer.offset(1);
            crc = *ctx
                .offset(
                    (crc as uint8_t as ::core::ffi::c_int
                        ^ *fresh0 as ::core::ffi::c_int) as isize,
                ) as uint32_t ^ crc >> 8 as ::core::ffi::c_int;
        }
        while buffer < end.offset(-(3 as ::core::ffi::c_int as isize)) {
            crc ^= *(buffer as *const uint32_t);
            buffer = buffer.offset(4 as ::core::ffi::c_int as isize);
            crc = (*ctx
                .offset(
                    ((3 as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as uint32_t)
                        .wrapping_add(crc & 0xff as uint32_t) as isize,
                )
                ^ *ctx
                    .offset(
                        ((2 as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                            as uint32_t)
                            .wrapping_add(
                                crc >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                            ) as isize,
                    )
                ^ *ctx
                    .offset(
                        ((1 as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                            as uint32_t)
                            .wrapping_add(
                                crc >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                            ) as isize,
                    )
                ^ *ctx
                    .offset(
                        ((0 as ::core::ffi::c_int * 256 as ::core::ffi::c_int)
                            as uint32_t)
                            .wrapping_add(crc >> 24 as ::core::ffi::c_int) as isize,
                    )) as uint32_t;
        }
    }
    while buffer < end {
        let fresh1 = buffer;
        buffer = buffer.offset(1);
        crc = *ctx
            .offset(
                (crc as uint8_t as ::core::ffi::c_int ^ *fresh1 as ::core::ffi::c_int)
                    as isize,
            ) as uint32_t ^ crc >> 8 as ::core::ffi::c_int;
    }
    return crc;
}
