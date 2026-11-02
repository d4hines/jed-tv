// Translated from FFmpeg 9.0.1 libavcodec/tiff_common.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// TIFF Common Routines
// Copyright (c) 2013 Thilo Borgmann <thilo.borgmann _at_ mail.de>
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
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_bprint_init(
        buf: *mut AVBPrint,
        size_init: ::core::ffi::c_uint,
        size_max: ::core::ffi::c_uint,
    );
    fn av_bprintf(buf: *mut AVBPrint, fmt: *const ::core::ffi::c_char, ...);
    fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_bprint_finalize(
        buf: *mut AVBPrint,
        ret_str: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = ::core::ffi::c_ulong;
pub type int16_t = ::core::ffi::c_short;
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBPrint {
    pub str_0: *mut ::core::ffi::c_char,
    pub len: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub size_max: ::core::ffi::c_uint,
    pub reserved_internal_buffer: [::core::ffi::c_char; 1],
    pub reserved_padding: [::core::ffi::c_char; 1000],
}
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_64 {
    pub l: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_16 {
    pub l: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetByteContext {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub buffer_start: *const uint8_t,
}
pub type AVTiffDataType = ::core::ffi::c_uint;
pub const AV_TIFF_IFD: AVTiffDataType = 13;
pub const AV_TIFF_DOUBLE: AVTiffDataType = 12;
pub const AV_TIFF_FLOAT: AVTiffDataType = 11;
pub const AV_TIFF_SRATIONAL: AVTiffDataType = 10;
pub const AV_TIFF_SLONG: AVTiffDataType = 9;
pub const AV_TIFF_SSHORT: AVTiffDataType = 8;
pub const AV_TIFF_UNDEFINED: AVTiffDataType = 7;
pub const AV_TIFF_SBYTE: AVTiffDataType = 6;
pub const AV_TIFF_RATIONAL: AVTiffDataType = 5;
pub const AV_TIFF_LONG: AVTiffDataType = 4;
pub const AV_TIFF_SHORT: AVTiffDataType = 3;
pub const AV_TIFF_STRING: AVTiffDataType = 2;
pub const AV_TIFF_BYTE: AVTiffDataType = 1;
#[no_mangle]
pub unsafe extern "C" fn ff_tis_ifd(mut tag: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[uint16_t; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<uint16_t>() as usize)
    {
        if ifd_tags[i as usize] as ::core::ffi::c_uint == tag {
            return i + 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
static mut type_sizes: [uint8_t; 14] = [
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    100 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
];
static mut ifd_tags: [uint16_t; 3] = [
    0x8769 as ::core::ffi::c_int as uint16_t,
    0x8825 as ::core::ffi::c_int as uint16_t,
    0xa005 as ::core::ffi::c_int as uint16_t,
];
#[no_mangle]
pub unsafe extern "C" fn ff_tget_short(
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    return if le != 0 { bytestream2_get_le16(gb) } else { bytestream2_get_be16(gb) };
}
#[no_mangle]
pub unsafe extern "C" fn ff_tget_long(
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    return if le != 0 { bytestream2_get_le32(gb) } else { bytestream2_get_be32(gb) };
}
#[inline(always)]
unsafe extern "C" fn av_bswap16(mut x: uint16_t) -> uint16_t {
    x = (x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint16_t;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn ff_tget_double(
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut i: av_alias64 = av_alias64 {
        u64_0: if le != 0 { bytestream2_get_le64(gb) } else { bytestream2_get_be64(gb) },
    };
    return i.f64_0;
}
#[inline(always)]
unsafe extern "C" fn av_bswap32(mut x: uint32_t) -> uint32_t {
    return (x << 8 as ::core::ffi::c_int & 0xff00 as uint32_t
        | x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) << 16 as ::core::ffi::c_int
        | ((x >> 16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            & 0xff00 as uint32_t
            | x >> 16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn ff_tget(
    mut gb: *mut GetByteContext,
    mut type_0: ::core::ffi::c_int,
    mut le: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    match type_0 {
        1 => return bytestream2_get_byte(gb),
        3 => return ff_tget_short(gb, le),
        4 => return ff_tget_long(gb, le),
        _ => {
            return (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint);
        }
    };
}
#[inline]
unsafe extern "C" fn av_bswap64(mut x: uint64_t) -> uint64_t {
    return (av_bswap32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int
        | av_bswap32((x >> 32 as ::core::ffi::c_int) as uint32_t) as uint64_t;
}
unsafe extern "C" fn auto_sep(
    mut count: ::core::ffi::c_int,
    mut sep: *const ::core::ffi::c_char,
    mut i: ::core::ffi::c_int,
    mut columns: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if !sep.is_null() {
        return if i != 0 {
            sep
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        };
    }
    if i != 0 && i % columns != 0 {
        return b", \0" as *const u8 as *const ::core::ffi::c_char
    } else {
        return if columns < count {
            b"\n\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        }
    };
}
unsafe extern "C" fn bprint_to_avdict(
    mut bp: *mut AVBPrint,
    mut name: *const ::core::ffi::c_char,
    mut metadata: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    let mut ap: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    if av_bprint_is_complete(bp) == 0 {
        av_bprint_finalize(bp, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>());
        return -(12 as ::core::ffi::c_int);
    }
    ret = av_bprint_finalize(bp, &raw mut ap);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    return av_dict_set(metadata, name, ap, 8 as ::core::ffi::c_int);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_le64(mut b: *mut *const uint8_t) -> uint64_t {
    *b = (*b).offset(8 as ::core::ffi::c_int as isize);
    return (*((*b).offset(-(8 as ::core::ffi::c_int as isize)) as *const unaligned_64))
        .l;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_le64u(mut g: *mut GetByteContext) -> uint64_t {
    return bytestream_get_le64(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_le64(mut g: *mut GetByteContext) -> uint64_t {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 8 as ::core::ffi::c_long
    {
        (*g).buffer = (*g).buffer_end;
        return 0 as uint64_t;
    }
    return bytestream2_get_le64u(g);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_le32(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(4 as ::core::ffi::c_int as isize);
    return (*((*b).offset(-(4 as ::core::ffi::c_int as isize)) as *const unaligned_32)).l
        as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_le32u(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_le32(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_le32(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 4 as ::core::ffi::c_long
    {
        (*g).buffer = (*g).buffer_end;
        return 0 as ::core::ffi::c_uint;
    }
    return bytestream2_get_le32u(g);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_le16(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(2 as ::core::ffi::c_int as isize);
    return (*((*b).offset(-(2 as ::core::ffi::c_int as isize)) as *const unaligned_16)).l
        as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_le16u(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_le16(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_le16(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 2 as ::core::ffi::c_long
    {
        (*g).buffer = (*g).buffer_end;
        return 0 as ::core::ffi::c_uint;
    }
    return bytestream2_get_le16u(g);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_be64(mut b: *mut *const uint8_t) -> uint64_t {
    *b = (*b).offset(8 as ::core::ffi::c_int as isize);
    return av_bswap64(
        (*((*b).offset(-(8 as ::core::ffi::c_int as isize)) as *const unaligned_64)).l,
    );
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be64u(mut g: *mut GetByteContext) -> uint64_t {
    return bytestream_get_be64(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be64(mut g: *mut GetByteContext) -> uint64_t {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 8 as ::core::ffi::c_long
    {
        (*g).buffer = (*g).buffer_end;
        return 0 as uint64_t;
    }
    return bytestream2_get_be64u(g);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_be32(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(4 as ::core::ffi::c_int as isize);
    return av_bswap32(
        (*((*b).offset(-(4 as ::core::ffi::c_int as isize)) as *const unaligned_32)).l,
    ) as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be32u(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_be32(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be32(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 4 as ::core::ffi::c_long
    {
        (*g).buffer = (*g).buffer_end;
        return 0 as ::core::ffi::c_uint;
    }
    return bytestream2_get_be32u(g);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_be16(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(2 as ::core::ffi::c_int as isize);
    return av_bswap16(
        (*((*b).offset(-(2 as ::core::ffi::c_int as isize)) as *const unaligned_16)).l,
    ) as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be16u(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_be16(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be16(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 2 as ::core::ffi::c_long
    {
        (*g).buffer = (*g).buffer_end;
        return 0 as ::core::ffi::c_uint;
    }
    return bytestream2_get_be16u(g);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_byte(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(1 as ::core::ffi::c_int as isize);
    return *(*b)
        .offset(-(1 as ::core::ffi::c_int as isize))
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_byteu(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_byte(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_byte(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 1 as ::core::ffi::c_long
    {
        (*g).buffer = (*g).buffer_end;
        return 0 as ::core::ffi::c_uint;
    }
    return bytestream2_get_byteu(g);
}
#[no_mangle]
pub unsafe extern "C" fn ff_tadd_doubles_metadata(
    mut count: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut sep: *const ::core::ffi::c_char,
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
    mut metadata: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    let mut bp: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1000],
    };
    let mut i: ::core::ffi::c_int = 0;
    if count as usize
        >= (2147483647 as ::core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<int64_t>() as usize)
        || count <= 0 as ::core::ffi::c_int
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (bytestream2_get_bytes_left(gb) as usize)
        < (count as usize).wrapping_mul(::core::mem::size_of::<int64_t>() as usize)
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    av_bprint_init(
        &raw mut bp,
        (10 as ::core::ffi::c_int * count) as ::core::ffi::c_uint,
        (100 as ::core::ffi::c_int * count) as ::core::ffi::c_uint,
    );
    i = 0 as ::core::ffi::c_int;
    while i < count {
        av_bprintf(
            &raw mut bp,
            b"%s%.15g\0" as *const u8 as *const ::core::ffi::c_char,
            auto_sep(count, sep, i, 4 as ::core::ffi::c_int),
            ff_tget_double(gb, le),
        );
        i += 1;
    }
    return bprint_to_avdict(&raw mut bp, name, metadata);
}
#[no_mangle]
pub unsafe extern "C" fn ff_tadd_shorts_metadata(
    mut count: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut sep: *const ::core::ffi::c_char,
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
    mut is_signed: ::core::ffi::c_int,
    mut metadata: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    let mut bp: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1000],
    };
    let mut i: ::core::ffi::c_int = 0;
    if count as usize
        >= (2147483647 as ::core::ffi::c_int as usize)
            .wrapping_div(::core::mem::size_of::<int16_t>() as usize)
        || count <= 0 as ::core::ffi::c_int
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (bytestream2_get_bytes_left(gb) as usize)
        < (count as usize).wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    av_bprint_init(
        &raw mut bp,
        (10 as ::core::ffi::c_int * count) as ::core::ffi::c_uint,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    i = 0 as ::core::ffi::c_int;
    while i < count {
        let mut v: ::core::ffi::c_int = (if is_signed != 0 {
            ff_tget_short(gb, le) as int16_t as ::core::ffi::c_uint
        } else {
            ff_tget_short(gb, le)
        }) as ::core::ffi::c_int;
        av_bprintf(
            &raw mut bp,
            b"%s%5i\0" as *const u8 as *const ::core::ffi::c_char,
            auto_sep(count, sep, i, 8 as ::core::ffi::c_int),
            v,
        );
        i += 1;
    }
    return bprint_to_avdict(&raw mut bp, name, metadata);
}
#[no_mangle]
pub unsafe extern "C" fn ff_tadd_string_metadata(
    mut count: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
    mut metadata: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if bytestream2_get_bytes_left(gb) < count || count < 0 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    value = av_malloc((count + 1 as ::core::ffi::c_int) as size_t)
        as *mut ::core::ffi::c_char;
    if value.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    bytestream2_get_bufferu(gb, value as *mut uint8_t, count as ::core::ffi::c_uint);
    *value.offset(count as isize) = 0 as ::core::ffi::c_char;
    av_dict_set(metadata, name, value, 8 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_bytes_left(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_int {
    return (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_tdecode_header(
    mut gb: *mut GetByteContext,
    mut le: *mut ::core::ffi::c_int,
    mut ifd_offset: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if bytestream2_get_bytes_left(gb) < 8 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    *le = bytestream2_get_le16u(gb) as ::core::ffi::c_int;
    if *le
        == av_bswap16(
            (*(b"II\0" as *const u8 as *const ::core::ffi::c_char
                as *const unaligned_16))
                .l,
        ) as ::core::ffi::c_int
    {
        *le = 1 as ::core::ffi::c_int;
    } else if *le
        == av_bswap16(
            (*(b"MM\0" as *const u8 as *const ::core::ffi::c_char
                as *const unaligned_16))
                .l,
        ) as ::core::ffi::c_int
    {
        *le = 0 as ::core::ffi::c_int;
    } else {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int)
    }
    if ff_tget_short(gb, *le) != 42 as ::core::ffi::c_uint {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    *ifd_offset = ff_tget_long(gb, *le) as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
#[no_mangle]
pub unsafe extern "C" fn ff_tread_tag(
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
    mut tag: *mut ::core::ffi::c_uint,
    mut type_0: *mut ::core::ffi::c_uint,
    mut count: *mut ::core::ffi::c_uint,
    mut next: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ifd_tag: ::core::ffi::c_int = 0;
    let mut valid_type: ::core::ffi::c_int = 0;
    *tag = ff_tget_short(gb, le);
    *type_0 = ff_tget_short(gb, le);
    *count = ff_tget_long(gb, le);
    ifd_tag = ff_tis_ifd(*tag);
    valid_type = (*type_0 != 0 as ::core::ffi::c_uint
        && (*type_0 as usize)
            < (::core::mem::size_of::<[uint8_t; 14]>() as usize)
                .wrapping_div(::core::mem::size_of::<uint8_t>() as usize))
        as ::core::ffi::c_int;
    *next = bytestream2_tell(gb) + 4 as ::core::ffi::c_int;
    if valid_type == 0 {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if ifd_tag != 0
        || (*count > 4 as ::core::ffi::c_uint
            || !((type_sizes[*type_0 as usize] as ::core::ffi::c_uint)
                .wrapping_mul(*count) <= 4 as ::core::ffi::c_uint
                || *type_0
                    == AV_TIFF_STRING as ::core::ffi::c_int as ::core::ffi::c_uint))
    {
        bytestream2_seek(
            gb,
            ff_tget_long(gb, le) as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_tell(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_int {
    return (*g).buffer.offset_from((*g).buffer_start) as ::core::ffi::c_long
        as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_seek(
    mut g: *mut GetByteContext,
    mut offset: ::core::ffi::c_int,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match whence {
        1 => {
            offset = av_clip_c(
                offset,
                -((*g).buffer.offset_from((*g).buffer_start) as ::core::ffi::c_long)
                    as ::core::ffi::c_int,
                (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            (*g).buffer = (*g).buffer.offset(offset as isize);
        }
        2 => {
            offset = av_clip_c(
                offset,
                -((*g).buffer_end.offset_from((*g).buffer_start) as ::core::ffi::c_long)
                    as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            (*g).buffer = (*g).buffer_end.offset(offset as isize);
        }
        0 => {
            offset = av_clip_c(
                offset,
                0 as ::core::ffi::c_int,
                (*g).buffer_end.offset_from((*g).buffer_start) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            (*g).buffer = (*g).buffer_start.offset(offset as isize);
        }
        _ => return -(22 as ::core::ffi::c_int),
    }
    return bytestream2_tell(g);
}
#[inline]
unsafe extern "C" fn av_bprint_is_complete(
    mut buf: *const AVBPrint,
) -> ::core::ffi::c_int {
    return ((*buf).len < (*buf).size) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_bufferu(
    mut g: *mut GetByteContext,
    mut dst: *mut uint8_t,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        (*g).buffer as *const ::core::ffi::c_void,
        size as size_t,
    );
    (*g).buffer = (*g).buffer.offset(size as isize);
    return size;
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
