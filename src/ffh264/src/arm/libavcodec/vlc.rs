// Translated from FFmpeg 9.0.1 libavcodec/vlc.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// API for creating VLC trees
// Copyright (c) 2000, 2001 Fabrice Bellard
// Copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
// Copyright (c) 2010 Loren Merritt
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
    static ff_reverse: [uint8_t; 256];
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn abort() -> !;
    fn avpriv_request_sample(
        avc: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_malloc_array(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_realloc_f(
        ptr: *mut ::core::ffi::c_void,
        nelem: size_t,
        elsize: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
}
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
pub type size_t = ::core::ffi::c_uint;
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
pub type VLCBaseType = int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLCElem {
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub level: int16_t,
    pub len8: int8_t,
    pub run: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub sym: VLCBaseType,
    pub len: VLCBaseType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLC {
    pub bits: ::core::ffi::c_int,
    pub table: *mut VLCElem,
    pub table_size: ::core::ffi::c_int,
    pub table_allocated: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLC_MULTI_ELEM {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub len: int8_t,
    pub num: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub val8: [uint8_t; 6],
    pub val16: [uint16_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLC_MULTI {
    pub table: *mut VLC_MULTI_ELEM,
    pub table_size: ::core::ffi::c_int,
    pub table_allocated: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLCcode {
    pub bits: uint8_t,
    pub symbol: VLCBaseType,
    pub code: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLCInitState {
    pub table: *mut VLCElem,
    pub size: ::core::ffi::c_uint,
}
unsafe extern "C" fn alloc_table(
    mut vlc: *mut VLC,
    mut size: ::core::ffi::c_int,
    mut use_static: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = (*vlc).table_size;
    (*vlc).table_size += size;
    if (*vlc).table_size > (*vlc).table_allocated {
        if use_static != 0 {
            abort();
        }
        (*vlc).table_allocated += (1 as ::core::ffi::c_int) << (*vlc).bits;
        (*vlc).table = av_realloc_f(
            (*vlc).table as *mut ::core::ffi::c_void,
            (*vlc).table_allocated as size_t,
            ::core::mem::size_of::<VLCElem>() as size_t,
        ) as *mut VLCElem;
        if (*vlc).table.is_null() {
            (*vlc).table_allocated = 0 as ::core::ffi::c_int;
            (*vlc).table_size = 0 as ::core::ffi::c_int;
            return -(12 as ::core::ffi::c_int);
        }
        memset(
            (*vlc)
                .table
                .offset((*vlc).table_allocated as isize)
                .offset(-(((1 as ::core::ffi::c_int) << (*vlc).bits) as isize))
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<VLCElem>() as size_t) << (*vlc).bits,
        );
    }
    return index;
}
#[inline(always)]
unsafe extern "C" fn bitswap_32(mut x: uint32_t) -> uint32_t {
    return (ff_reverse[(x & 0xff as uint32_t) as usize] as uint32_t)
        << 24 as ::core::ffi::c_int
        | (ff_reverse[(x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            as uint32_t) << 16 as ::core::ffi::c_int
        | (ff_reverse[(x >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            as uint32_t) << 8 as ::core::ffi::c_int
        | ff_reverse[(x >> 24 as ::core::ffi::c_int) as usize] as uint32_t;
}
unsafe extern "C" fn vlc_common_init(
    mut vlc: *mut VLC,
    mut nb_bits: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut buf: *mut *mut VLCcode,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*vlc).bits = nb_bits;
    (*vlc).table_size = 0 as ::core::ffi::c_int;
    if flags & 1 as ::core::ffi::c_int != 0 {
        if !(nb_codes <= 1500 as ::core::ffi::c_int) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"nb_codes <= 1500\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/vlc.c\0" as *const u8 as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int,
            );
            abort();
        }
    } else {
        (*vlc).table = ::core::ptr::null_mut::<VLCElem>();
        (*vlc).table_allocated = 0 as ::core::ffi::c_int;
    }
    if nb_codes > 1500 as ::core::ffi::c_int {
        *buf = av_malloc_array(
            nb_codes as size_t,
            ::core::mem::size_of::<VLCcode>() as size_t,
        ) as *mut VLCcode;
        if (*buf).is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn compare_vlcspec(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sa: *const VLCcode = a as *const VLCcode;
    let mut sb: *const VLCcode = b as *const VLCcode;
    return ((*sa).code >> 1 as ::core::ffi::c_int)
        .wrapping_sub((*sb).code >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn build_table(
    mut vlc: *mut VLC,
    mut table_nb_bits: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut codes: *mut VLCcode,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut table_size: ::core::ffi::c_int = 0;
    let mut table_index: ::core::ffi::c_int = 0;
    let mut table: *mut VLCElem = ::core::ptr::null_mut::<VLCElem>();
    if table_nb_bits > 30 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    table_size = (1 as ::core::ffi::c_int) << table_nb_bits;
    table_index = alloc_table(vlc, table_size, flags & 1 as ::core::ffi::c_int);
    if table_index < 0 as ::core::ffi::c_int {
        return table_index;
    }
    table = (*vlc).table.offset(table_index as isize) as *mut VLCElem;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < nb_codes {
        let mut n: ::core::ffi::c_int = (*codes.offset(i as isize)).bits
            as ::core::ffi::c_int;
        let mut code: uint32_t = (*codes.offset(i as isize)).code;
        let mut symbol: ::core::ffi::c_int = (*codes.offset(i as isize)).symbol
            as ::core::ffi::c_int;
        if n <= table_nb_bits {
            let mut j: ::core::ffi::c_int = (code
                >> 32 as ::core::ffi::c_int - table_nb_bits) as ::core::ffi::c_int;
            let mut nb: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
                << table_nb_bits - n;
            let mut inc: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            if flags & 8 as ::core::ffi::c_int != 0 {
                j = bitswap_32(code) as ::core::ffi::c_int;
                inc = (1 as ::core::ffi::c_int) << n;
            }
            let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while k < nb {
                let mut bits: ::core::ffi::c_int = (*table.offset(j as isize))
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .len as ::core::ffi::c_int;
                let mut oldsym: ::core::ffi::c_int = (*table.offset(j as isize))
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .sym as ::core::ffi::c_int;
                if (bits != 0 || oldsym != 0) && (bits != n || oldsym != symbol) {
                    av_log(
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        16 as ::core::ffi::c_int,
                        b"incorrect codes\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
                (*table.offset(j as isize)).c2rust_unnamed.c2rust_unnamed.len = n
                    as VLCBaseType;
                (*table.offset(j as isize)).c2rust_unnamed.c2rust_unnamed.sym = symbol
                    as VLCBaseType;
                j += inc;
                k += 1;
            }
        } else {
            let mut code_prefix: uint32_t = 0;
            let mut index: ::core::ffi::c_int = 0;
            let mut subtable_bits: ::core::ffi::c_int = 0;
            let mut j_0: ::core::ffi::c_int = 0;
            let mut k_0: ::core::ffi::c_int = 0;
            n -= table_nb_bits;
            code_prefix = code >> 32 as ::core::ffi::c_int - table_nb_bits;
            subtable_bits = n;
            (*codes.offset(i as isize)).bits = n as uint8_t;
            (*codes.offset(i as isize)).code = code << table_nb_bits;
            k_0 = i + 1 as ::core::ffi::c_int;
            while k_0 < nb_codes {
                n = (*codes.offset(k_0 as isize)).bits as ::core::ffi::c_int
                    - table_nb_bits;
                if n <= 0 as ::core::ffi::c_int {
                    break;
                }
                code = (*codes.offset(k_0 as isize)).code;
                if code >> 32 as ::core::ffi::c_int - table_nb_bits != code_prefix {
                    break;
                }
                (*codes.offset(k_0 as isize)).bits = n as uint8_t;
                (*codes.offset(k_0 as isize)).code = code << table_nb_bits;
                subtable_bits = if subtable_bits > n { subtable_bits } else { n };
                k_0 += 1;
            }
            subtable_bits = if subtable_bits > table_nb_bits {
                table_nb_bits
            } else {
                subtable_bits
            };
            j_0 = (if flags & 8 as ::core::ffi::c_int != 0 {
                bitswap_32(code_prefix) >> 32 as ::core::ffi::c_int - table_nb_bits
            } else {
                code_prefix
            }) as ::core::ffi::c_int;
            (*table.offset(j_0 as isize)).c2rust_unnamed.c2rust_unnamed.len = -subtable_bits
                as VLCBaseType;
            index = build_table(
                vlc,
                subtable_bits,
                k_0 - i,
                codes.offset(i as isize),
                flags,
            );
            if index < 0 as ::core::ffi::c_int {
                return index;
            }
            table = (*vlc).table.offset(table_index as isize) as *mut VLCElem;
            (*table.offset(j_0 as isize)).c2rust_unnamed.c2rust_unnamed.sym = index
                as VLCBaseType;
            if (*table.offset(j_0 as isize)).c2rust_unnamed.c2rust_unnamed.sym
                as ::core::ffi::c_int != index
            {
                avpriv_request_sample(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    b"strange codes\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
                    | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            i = k_0 - 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < table_size {
        if (*table.offset(i_0 as isize)).c2rust_unnamed.c2rust_unnamed.len
            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*table.offset(i_0 as isize)).c2rust_unnamed.c2rust_unnamed.sym = -(1
                as ::core::ffi::c_int) as VLCBaseType;
        }
        i_0 += 1;
    }
    return table_index;
}
unsafe extern "C" fn vlc_common_end(
    mut vlc: *mut VLC,
    mut nb_bits: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut codes: *mut VLCcode,
    mut flags: ::core::ffi::c_int,
    mut localbuf: *mut VLCcode,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = build_table(vlc, nb_bits, nb_codes, codes, flags);
    if flags & 1 as ::core::ffi::c_int != 0 {
        if (*vlc).table_size != (*vlc).table_allocated
            && flags
                & ((2 as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
                    & !(1 as ::core::ffi::c_int)) == 0
        {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                16 as ::core::ffi::c_int,
                b"needed %d had %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*vlc).table_size,
                (*vlc).table_allocated,
            );
        }
        if !(ret >= 0 as ::core::ffi::c_int) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"ret >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/vlc.c\0" as *const u8 as *const ::core::ffi::c_char,
                238 as ::core::ffi::c_int,
            );
            abort();
        }
    } else {
        if codes != localbuf {
            av_free(codes as *mut ::core::ffi::c_void);
        }
        if ret < 0 as ::core::ffi::c_int {
            av_freep(&raw mut (*vlc).table as *mut ::core::ffi::c_void);
            return ret;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_vlc_init_sparse(
    mut vlc: *mut VLC,
    mut nb_bits: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut bits: *const ::core::ffi::c_void,
    mut bits_wrap: ::core::ffi::c_int,
    mut bits_size: ::core::ffi::c_int,
    mut codes: *const ::core::ffi::c_void,
    mut codes_wrap: ::core::ffi::c_int,
    mut codes_size: ::core::ffi::c_int,
    mut symbols: *const ::core::ffi::c_void,
    mut symbols_wrap: ::core::ffi::c_int,
    mut symbols_size: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut localbuf: [VLCcode; 1500] = [VLCcode {
        bits: 0,
        symbol: 0,
        code: 0,
    }; 1500];
    let mut buf: *mut VLCcode = &raw mut localbuf as *mut VLCcode;
    let mut j: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    ret = vlc_common_init(vlc, nb_bits, nb_codes, &raw mut buf, flags);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if !(symbols_size as ::core::ffi::c_uint <= 2 as ::core::ffi::c_uint) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"symbols_size <= 2U\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/vlc.c\0" as *const u8 as *const ::core::ffi::c_char,
            263 as ::core::ffi::c_int,
        );
        abort();
    }
    j = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < nb_codes {
        let mut len: ::core::ffi::c_uint = 0;
        let mut ptr: *const uint8_t = (bits as *const uint8_t)
            .offset((i * bits_wrap) as isize);
        match bits_size {
            1 => {
                len = *ptr as ::core::ffi::c_uint;
            }
            2 => {
                len = *(ptr as *const uint16_t) as ::core::ffi::c_uint;
            }
            4 => {
                len = *(ptr as *const uint32_t) as ::core::ffi::c_uint;
            }
            _ => {
                unreachable!();
            }
        }
        if len > nb_bits as ::core::ffi::c_uint {
            if len > (3 as ::core::ffi::c_int * nb_bits) as ::core::ffi::c_uint
                || len > 32 as ::core::ffi::c_uint
            {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    16 as ::core::ffi::c_int,
                    b"Too long VLC (%u) in vlc_init\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    len,
                );
                if buf != &raw mut localbuf as *mut VLCcode {
                    av_free(buf as *mut ::core::ffi::c_void);
                }
                return -(22 as ::core::ffi::c_int);
            }
            (*buf.offset(j as isize)).bits = len as uint8_t;
            let mut ptr_0: *const uint8_t = (codes as *const uint8_t)
                .offset((i * codes_wrap) as isize);
            match codes_size {
                1 => {
                    (*buf.offset(j as isize)).code = *ptr_0 as uint32_t;
                }
                2 => {
                    (*buf.offset(j as isize)).code = *(ptr_0 as *const uint16_t)
                        as uint32_t;
                }
                4 => {
                    (*buf.offset(j as isize)).code = *(ptr_0 as *const uint32_t);
                }
                _ => {
                    unreachable!();
                }
            }
            if (*buf.offset(j as isize)).code as ::core::ffi::c_longlong
                >= (1 as ::core::ffi::c_longlong)
                    << (*buf.offset(j as isize)).bits as ::core::ffi::c_int
            {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    16 as ::core::ffi::c_int,
                    b"Invalid code %x for %d in vlc_init\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*buf.offset(j as isize)).code,
                    i,
                );
                if buf != &raw mut localbuf as *mut VLCcode {
                    av_free(buf as *mut ::core::ffi::c_void);
                }
                return -(22 as ::core::ffi::c_int);
            }
            if flags & 4 as ::core::ffi::c_int != 0 {
                (*buf.offset(j as isize)).code = bitswap_32(
                    (*buf.offset(j as isize)).code,
                );
            } else {
                (*buf.offset(j as isize)).code
                    <<= 32 as ::core::ffi::c_int
                        - (*buf.offset(j as isize)).bits as ::core::ffi::c_int;
            }
            if !symbols.is_null() {
                let mut ptr_1: *const uint8_t = (symbols as *const uint8_t)
                    .offset((i * symbols_wrap) as isize);
                match symbols_size {
                    1 => {
                        (*buf.offset(j as isize)).symbol = *ptr_1 as VLCBaseType;
                    }
                    2 => {
                        (*buf.offset(j as isize)).symbol = *(ptr_1 as *const uint16_t)
                            as VLCBaseType;
                    }
                    4 => {
                        (*buf.offset(j as isize)).symbol = *(ptr_1 as *const uint32_t)
                            as VLCBaseType;
                    }
                    _ => {
                        unreachable!();
                    }
                }
            } else {
                (*buf.offset(j as isize)).symbol = i as VLCBaseType;
            }
            j += 1;
        }
        i += 1;
    }
    let mut stack: [[*mut ::core::ffi::c_void; 2]; 64] = [[::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >(); 2]; 64];
    let mut sp: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    stack[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] = buf
        as *mut ::core::ffi::c_void;
    stack[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize] = buf
        .offset(j as isize)
        .offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void;
    while sp != 0 {
        sp -= 1;
        let mut start: *mut VLCcode = stack[sp
            as usize][0 as ::core::ffi::c_int as usize] as *mut VLCcode;
        let mut end: *mut VLCcode = stack[sp as usize][1 as ::core::ffi::c_int as usize]
            as *mut VLCcode;
        while start < end {
            if start < end.offset(-(1 as ::core::ffi::c_int as isize)) {
                let mut checksort: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut right: *mut VLCcode = end
                    .offset(-(2 as ::core::ffi::c_int as isize));
                let mut left: *mut VLCcode = start
                    .offset(1 as ::core::ffi::c_int as isize);
                let mut mid: *mut VLCcode = start
                    .offset(
                        (end.offset_from(start) as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int) as isize,
                    );
                if compare_vlcspec(
                    start as *const ::core::ffi::c_void,
                    end as *const ::core::ffi::c_void,
                ) > 0 as ::core::ffi::c_int
                {
                    if compare_vlcspec(
                        end as *const ::core::ffi::c_void,
                        mid as *const ::core::ffi::c_void,
                    ) > 0 as ::core::ffi::c_int
                    {
                        let mut SWAP_tmp: VLCcode = *mid;
                        *mid = *start;
                        *start = SWAP_tmp;
                    } else {
                        let mut SWAP_tmp_0: VLCcode = *end;
                        *end = *start;
                        *start = SWAP_tmp_0;
                    }
                } else if compare_vlcspec(
                    start as *const ::core::ffi::c_void,
                    mid as *const ::core::ffi::c_void,
                ) > 0 as ::core::ffi::c_int
                {
                    let mut SWAP_tmp_1: VLCcode = *mid;
                    *mid = *start;
                    *start = SWAP_tmp_1;
                } else {
                    checksort = 1 as ::core::ffi::c_int;
                }
                if compare_vlcspec(
                    mid as *const ::core::ffi::c_void,
                    end as *const ::core::ffi::c_void,
                ) > 0 as ::core::ffi::c_int
                {
                    let mut SWAP_tmp_2: VLCcode = *end;
                    *end = *mid;
                    *mid = SWAP_tmp_2;
                    checksort = 0 as ::core::ffi::c_int;
                }
                if start == end.offset(-(2 as ::core::ffi::c_int as isize)) {
                    break;
                }
                let mut SWAP_tmp_3: VLCcode = *mid;
                *mid = *end.offset(-(1 as ::core::ffi::c_int) as isize);
                *end.offset(-(1 as ::core::ffi::c_int) as isize) = SWAP_tmp_3;
                while left <= right {
                    while left <= right
                        && compare_vlcspec(
                            left as *const ::core::ffi::c_void,
                            end.offset(-(1 as ::core::ffi::c_int as isize))
                                as *const ::core::ffi::c_void,
                        ) < 0 as ::core::ffi::c_int
                    {
                        left = left.offset(1);
                    }
                    while left <= right
                        && compare_vlcspec(
                            right as *const ::core::ffi::c_void,
                            end.offset(-(1 as ::core::ffi::c_int as isize))
                                as *const ::core::ffi::c_void,
                        ) > 0 as ::core::ffi::c_int
                    {
                        right = right.offset(-1);
                    }
                    if left <= right {
                        let mut SWAP_tmp_4: VLCcode = *right;
                        *right = *left;
                        *left = SWAP_tmp_4;
                        left = left.offset(1);
                        right = right.offset(-1);
                    }
                }
                let mut SWAP_tmp_5: VLCcode = *left;
                *left = *end.offset(-(1 as ::core::ffi::c_int) as isize);
                *end.offset(-(1 as ::core::ffi::c_int) as isize) = SWAP_tmp_5;
                if checksort != 0
                    && (mid == left.offset(-(1 as ::core::ffi::c_int as isize))
                        || mid == left)
                {
                    mid = start;
                    while mid < end
                        && compare_vlcspec(
                            mid as *const ::core::ffi::c_void,
                            mid.offset(1 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                        ) <= 0 as ::core::ffi::c_int
                    {
                        mid = mid.offset(1);
                    }
                    if mid == end {
                        break;
                    }
                }
                if (end.offset_from(left) as ::core::ffi::c_int)
                    < left.offset_from(start) as ::core::ffi::c_int
                {
                    stack[sp as usize][0 as ::core::ffi::c_int as usize] = start
                        as *mut ::core::ffi::c_void;
                    let fresh0 = sp;
                    sp = sp + 1;
                    stack[fresh0 as usize][1 as ::core::ffi::c_int as usize] = right
                        as *mut ::core::ffi::c_void;
                    start = left.offset(1 as ::core::ffi::c_int as isize);
                } else {
                    stack[sp as usize][0 as ::core::ffi::c_int as usize] = left
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void;
                    let fresh1 = sp;
                    sp = sp + 1;
                    stack[fresh1 as usize][1 as ::core::ffi::c_int as usize] = end
                        as *mut ::core::ffi::c_void;
                    end = right;
                }
            } else {
                if compare_vlcspec(
                    start as *const ::core::ffi::c_void,
                    end as *const ::core::ffi::c_void,
                ) > 0 as ::core::ffi::c_int
                {
                    let mut SWAP_tmp_6: VLCcode = *end;
                    *end = *start;
                    *start = SWAP_tmp_6;
                }
                break;
            }
        }
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < nb_codes {
        let mut len_0: ::core::ffi::c_uint = 0;
        let mut ptr_2: *const uint8_t = (bits as *const uint8_t)
            .offset((i_0 * bits_wrap) as isize);
        match bits_size {
            1 => {
                len_0 = *ptr_2 as ::core::ffi::c_uint;
            }
            2 => {
                len_0 = *(ptr_2 as *const uint16_t) as ::core::ffi::c_uint;
            }
            4 => {
                len_0 = *(ptr_2 as *const uint32_t) as ::core::ffi::c_uint;
            }
            _ => {
                unreachable!();
            }
        }
        if len_0 != 0 && len_0 <= nb_bits as ::core::ffi::c_uint {
            if len_0 > (3 as ::core::ffi::c_int * nb_bits) as ::core::ffi::c_uint
                || len_0 > 32 as ::core::ffi::c_uint
            {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    16 as ::core::ffi::c_int,
                    b"Too long VLC (%u) in vlc_init\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    len_0,
                );
                if buf != &raw mut localbuf as *mut VLCcode {
                    av_free(buf as *mut ::core::ffi::c_void);
                }
                return -(22 as ::core::ffi::c_int);
            }
            (*buf.offset(j as isize)).bits = len_0 as uint8_t;
            let mut ptr_3: *const uint8_t = (codes as *const uint8_t)
                .offset((i_0 * codes_wrap) as isize);
            match codes_size {
                1 => {
                    (*buf.offset(j as isize)).code = *ptr_3 as uint32_t;
                }
                2 => {
                    (*buf.offset(j as isize)).code = *(ptr_3 as *const uint16_t)
                        as uint32_t;
                }
                4 => {
                    (*buf.offset(j as isize)).code = *(ptr_3 as *const uint32_t);
                }
                _ => {
                    unreachable!();
                }
            }
            if (*buf.offset(j as isize)).code as ::core::ffi::c_longlong
                >= (1 as ::core::ffi::c_longlong)
                    << (*buf.offset(j as isize)).bits as ::core::ffi::c_int
            {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    16 as ::core::ffi::c_int,
                    b"Invalid code %x for %d in vlc_init\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*buf.offset(j as isize)).code,
                    i_0,
                );
                if buf != &raw mut localbuf as *mut VLCcode {
                    av_free(buf as *mut ::core::ffi::c_void);
                }
                return -(22 as ::core::ffi::c_int);
            }
            if flags & 4 as ::core::ffi::c_int != 0 {
                (*buf.offset(j as isize)).code = bitswap_32(
                    (*buf.offset(j as isize)).code,
                );
            } else {
                (*buf.offset(j as isize)).code
                    <<= 32 as ::core::ffi::c_int
                        - (*buf.offset(j as isize)).bits as ::core::ffi::c_int;
            }
            if !symbols.is_null() {
                let mut ptr_4: *const uint8_t = (symbols as *const uint8_t)
                    .offset((i_0 * symbols_wrap) as isize);
                match symbols_size {
                    1 => {
                        (*buf.offset(j as isize)).symbol = *ptr_4 as VLCBaseType;
                    }
                    2 => {
                        (*buf.offset(j as isize)).symbol = *(ptr_4 as *const uint16_t)
                            as VLCBaseType;
                    }
                    4 => {
                        (*buf.offset(j as isize)).symbol = *(ptr_4 as *const uint32_t)
                            as VLCBaseType;
                    }
                    _ => {
                        unreachable!();
                    }
                }
            } else {
                (*buf.offset(j as isize)).symbol = i_0 as VLCBaseType;
            }
            j += 1;
        }
        i_0 += 1;
    }
    nb_codes = j;
    return vlc_common_end(
        vlc,
        nb_bits,
        nb_codes,
        buf,
        flags,
        &raw mut localbuf as *mut VLCcode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_vlc_init_from_lengths(
    mut vlc: *mut VLC,
    mut nb_bits: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut lens: *const int8_t,
    mut lens_wrap: ::core::ffi::c_int,
    mut symbols: *const ::core::ffi::c_void,
    mut symbols_wrap: ::core::ffi::c_int,
    mut symbols_size: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut localbuf: [VLCcode; 1500] = [VLCcode {
        bits: 0,
        symbol: 0,
        code: 0,
    }; 1500];
    let mut buf: *mut VLCcode = &raw mut localbuf as *mut VLCcode;
    let mut code: uint64_t = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut len_max: ::core::ffi::c_int = if 32 as ::core::ffi::c_int
        > 3 as ::core::ffi::c_int * nb_bits
    {
        3 as ::core::ffi::c_int * nb_bits
    } else {
        32 as ::core::ffi::c_int
    };
    ret = vlc_common_init(vlc, nb_bits, nb_codes, &raw mut buf, flags);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    code = 0 as uint64_t;
    j = code as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        if !(i < nb_codes) {
            current_block = 11057878835866523405;
            break;
        }
        let mut len: ::core::ffi::c_int = *lens as ::core::ffi::c_int;
        if len > 0 as ::core::ffi::c_int {
            let mut sym: ::core::ffi::c_uint = 0;
            (*buf.offset(j as isize)).bits = len as uint8_t;
            if !symbols.is_null() {
                let mut ptr: *const uint8_t = (symbols as *const uint8_t)
                    .offset((i * symbols_wrap) as isize);
                match symbols_size {
                    1 => {
                        sym = *ptr as ::core::ffi::c_uint;
                    }
                    2 => {
                        sym = *(ptr as *const uint16_t) as ::core::ffi::c_uint;
                    }
                    4 => {
                        sym = *(ptr as *const uint32_t) as ::core::ffi::c_uint;
                    }
                    _ => {
                        unreachable!();
                    }
                }
            } else {
                sym = i as ::core::ffi::c_uint;
            }
            (*buf.offset(j as isize)).symbol = sym
                .wrapping_add(offset as ::core::ffi::c_uint) as VLCBaseType;
            let fresh2 = j;
            j = j + 1;
            (*buf.offset(fresh2 as isize)).code = code as uint32_t;
            current_block = 4495394744059808450;
        } else if len < 0 as ::core::ffi::c_int {
            len = -len;
            current_block = 4495394744059808450;
        } else {
            current_block = 715039052867723359;
        }
        match current_block {
            4495394744059808450 => {
                if len > len_max
                    || code
                        & ((1 as ::core::ffi::c_uint) << 32 as ::core::ffi::c_int - len)
                            .wrapping_sub(1 as ::core::ffi::c_uint) as uint64_t != 0
                {
                    av_log(
                        logctx,
                        16 as ::core::ffi::c_int,
                        b"Invalid VLC (length %u)\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        len,
                    );
                    current_block = 5140199679691561216;
                    break;
                } else {
                    code = code
                        .wrapping_add(
                            ((1 as ::core::ffi::c_uint)
                                << 32 as ::core::ffi::c_int - len) as uint64_t,
                        );
                    if code
                        > (0xffffffff as ::core::ffi::c_ulonglong)
                            .wrapping_add(1 as ::core::ffi::c_ulonglong)
                    {
                        av_log(
                            logctx,
                            16 as ::core::ffi::c_int,
                            b"Overdetermined VLC tree\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 5140199679691561216;
                        break;
                    }
                }
            }
            _ => {}
        }
        i += 1;
        lens = lens.offset(lens_wrap as isize);
    }
    match current_block {
        11057878835866523405 => {
            return vlc_common_end(
                vlc,
                nb_bits,
                j,
                buf,
                flags,
                &raw mut localbuf as *mut VLCcode,
            );
        }
        _ => {
            if buf != &raw mut localbuf as *mut VLCcode {
                av_free(buf as *mut ::core::ffi::c_void);
            }
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_vlc_init_table_from_lengths(
    mut table: *mut VLCElem,
    mut table_size: ::core::ffi::c_int,
    mut nb_bits: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut lens: *const int8_t,
    mut lens_wrap: ::core::ffi::c_int,
    mut symbols: *const ::core::ffi::c_void,
    mut symbols_wrap: ::core::ffi::c_int,
    mut symbols_size: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) {
    let mut vlc: VLC = VLC {
        bits: 0,
        table: table as *mut VLCElem,
        table_size: 0,
        table_allocated: table_size,
    };
    ff_vlc_init_from_lengths(
        &raw mut vlc,
        nb_bits,
        nb_codes,
        lens,
        lens_wrap,
        symbols,
        symbols_wrap,
        symbols_size,
        offset,
        flags | 1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_vlc_init_tables_from_lengths(
    mut state: *mut VLCInitState,
    mut nb_bits: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut lens: *const int8_t,
    mut lens_wrap: ::core::ffi::c_int,
    mut symbols: *const ::core::ffi::c_void,
    mut symbols_wrap: ::core::ffi::c_int,
    mut symbols_size: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> *const VLCElem {
    let mut vlc: VLC = VLC {
        bits: 0,
        table: (*state).table,
        table_size: 0,
        table_allocated: (*state).size as ::core::ffi::c_int,
    };
    ff_vlc_init_from_lengths(
        &raw mut vlc,
        nb_bits,
        nb_codes,
        lens,
        lens_wrap,
        symbols,
        symbols_wrap,
        symbols_size,
        offset,
        flags | (2 as ::core::ffi::c_int | 1 as ::core::ffi::c_int),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    (*state).table = (*state).table.offset(vlc.table_size as isize);
    (*state).size = (*state).size.wrapping_sub(vlc.table_size as ::core::ffi::c_uint);
    return vlc.table;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_vlc_init_table_sparse(
    mut table: *mut VLCElem,
    mut table_size: ::core::ffi::c_int,
    mut nb_bits: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut bits: *const ::core::ffi::c_void,
    mut bits_wrap: ::core::ffi::c_int,
    mut bits_size: ::core::ffi::c_int,
    mut codes: *const ::core::ffi::c_void,
    mut codes_wrap: ::core::ffi::c_int,
    mut codes_size: ::core::ffi::c_int,
    mut symbols: *const ::core::ffi::c_void,
    mut symbols_wrap: ::core::ffi::c_int,
    mut symbols_size: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) {
    let mut vlc: VLC = VLC {
        bits: 0,
        table: table as *mut VLCElem,
        table_size: 0,
        table_allocated: table_size,
    };
    ff_vlc_init_sparse(
        &raw mut vlc,
        nb_bits,
        nb_codes,
        bits,
        bits_wrap,
        bits_size,
        codes,
        codes_wrap,
        codes_size,
        symbols,
        symbols_wrap,
        symbols_size,
        flags | 1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_vlc_init_tables_sparse(
    mut state: *mut VLCInitState,
    mut nb_bits: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut bits: *const ::core::ffi::c_void,
    mut bits_wrap: ::core::ffi::c_int,
    mut bits_size: ::core::ffi::c_int,
    mut codes: *const ::core::ffi::c_void,
    mut codes_wrap: ::core::ffi::c_int,
    mut codes_size: ::core::ffi::c_int,
    mut symbols: *const ::core::ffi::c_void,
    mut symbols_wrap: ::core::ffi::c_int,
    mut symbols_size: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> *const VLCElem {
    let mut vlc: VLC = VLC {
        bits: 0,
        table: (*state).table,
        table_size: 0,
        table_allocated: (*state).size as ::core::ffi::c_int,
    };
    ff_vlc_init_sparse(
        &raw mut vlc,
        nb_bits,
        nb_codes,
        bits,
        bits_wrap,
        bits_size,
        codes,
        codes_wrap,
        codes_size,
        symbols,
        symbols_wrap,
        symbols_size,
        flags | (2 as ::core::ffi::c_int | 1 as ::core::ffi::c_int),
    );
    (*state).table = (*state).table.offset(vlc.table_size as isize);
    (*state).size = (*state).size.wrapping_sub(vlc.table_size as ::core::ffi::c_uint);
    return vlc.table;
}
unsafe extern "C" fn add_level(
    mut table: *mut VLC_MULTI_ELEM,
    is16bit: ::core::ffi::c_int,
    num: ::core::ffi::c_int,
    numbits: ::core::ffi::c_int,
    mut buf: *const VLCcode,
    mut curcode: uint32_t,
    mut curlen: ::core::ffi::c_int,
    mut curlimit: ::core::ffi::c_int,
    mut curlevel: ::core::ffi::c_int,
    minlen: ::core::ffi::c_int,
    max: ::core::ffi::c_int,
    mut levelcnt: *mut ::core::ffi::c_uint,
    mut info: VLC_MULTI_ELEM,
) {
    let mut max_symbols: ::core::ffi::c_int = 6 as ::core::ffi::c_int >> is16bit;
    let mut i: ::core::ffi::c_int = num - 1 as ::core::ffi::c_int;
    while i >= max {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < 2 as ::core::ffi::c_int {
            let mut newlimit: ::core::ffi::c_int = 0;
            let mut sym: ::core::ffi::c_int = 0;
            let mut t: ::core::ffi::c_int = if j != 0 {
                i - 1 as ::core::ffi::c_int
            } else {
                i
            };
            let mut l: ::core::ffi::c_int = (*buf.offset(t as isize)).bits
                as ::core::ffi::c_int;
            let mut code: uint32_t = 0;
            sym = (*buf.offset(t as isize)).symbol as ::core::ffi::c_int;
            if l >= curlimit {
                return;
            }
            code = curcode.wrapping_add((*buf.offset(t as isize)).code >> curlen);
            newlimit = curlimit - l;
            l += curlen;
            if is16bit != 0 {
                info.c2rust_unnamed.val16[curlevel as usize] = sym as uint16_t;
            } else {
                info.c2rust_unnamed.val8[curlevel as usize] = (sym
                    & 0xff as ::core::ffi::c_int) as uint8_t;
            }
            if curlevel != 0 {
                let mut val: uint32_t = code >> 32 as ::core::ffi::c_int - numbits;
                let mut nb: uint32_t = val.wrapping_add((1 as uint32_t) << numbits - l);
                info.len = l as int8_t;
                info.num = (curlevel + 1 as ::core::ffi::c_int) as uint8_t;
                while val < nb {
                    (*(table.offset(val as isize) as *mut av_alias64)).u64_0 = (*(&raw mut info
                        as *const av_alias64))
                        .u64_0;
                    val = val.wrapping_add(1);
                }
                let ref mut fresh4 = *levelcnt
                    .offset((curlevel - 1 as ::core::ffi::c_int) as isize);
                *fresh4 = (*fresh4).wrapping_add(1);
            }
            if (curlevel + 1 as ::core::ffi::c_int) < max_symbols && newlimit >= minlen {
                add_level(
                    table,
                    is16bit,
                    num,
                    numbits,
                    buf,
                    code,
                    l,
                    newlimit,
                    curlevel + 1 as ::core::ffi::c_int,
                    minlen,
                    max,
                    levelcnt,
                    info,
                );
            }
            j += 1;
        }
        i -= 1;
    }
}
unsafe extern "C" fn vlc_multi_gen(
    mut table: *mut VLC_MULTI_ELEM,
    mut single: *const VLC,
    is16bit: ::core::ffi::c_int,
    nb_codes: ::core::ffi::c_int,
    numbits: ::core::ffi::c_int,
    mut buf: *mut VLCcode,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut minbits: ::core::ffi::c_int = 0;
    let mut maxbits: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut count: [::core::ffi::c_uint; 5] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0,
        0,
        0,
        0,
    ];
    let mut info: VLC_MULTI_ELEM = VLC_MULTI_ELEM {
        c2rust_unnamed: C2RustUnnamed_2 {
            val8: [0 as ::core::ffi::c_int as uint8_t, 0, 0, 0, 0, 0],
        },
        len: 0,
        num: 0,
    };
    let mut count0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j < (1 as ::core::ffi::c_int) << numbits {
        if (*(*single).table.offset(j as isize)).c2rust_unnamed.c2rust_unnamed.len
            as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            count0 += 1;
            j
                += ((1 as ::core::ffi::c_int)
                    << numbits
                        - (*(*single).table.offset(j as isize))
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .len as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
        }
        j += 1;
    }
    minbits = 32 as ::core::ffi::c_int;
    maxbits = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = nb_codes - count0;
    while n < nb_codes {
        minbits = if minbits > (*buf.offset(n as isize)).bits as ::core::ffi::c_int {
            (*buf.offset(n as isize)).bits as ::core::ffi::c_int
        } else {
            minbits
        };
        maxbits = if maxbits > (*buf.offset(n as isize)).bits as ::core::ffi::c_int {
            maxbits
        } else {
            (*buf.offset(n as isize)).bits as ::core::ffi::c_int
        };
        n += 1;
    }
    if !(maxbits <= numbits) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"maxbits <= numbits\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/vlc.c\0" as *const u8 as *const ::core::ffi::c_char,
            488 as ::core::ffi::c_int,
        );
        abort();
    }
    max = nb_codes;
    while max > nb_codes - count0 {
        if (*buf.offset((max - 1 as ::core::ffi::c_int) as isize)).bits
            as ::core::ffi::c_int + minbits > numbits
        {
            break;
        }
        max -= 1;
    }
    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j_0 < (1 as ::core::ffi::c_int) << numbits {
        (*table.offset(j_0 as isize)).len = (*(*single).table.offset(j_0 as isize))
            .c2rust_unnamed
            .c2rust_unnamed
            .len as int8_t;
        (*table.offset(j_0 as isize)).num = (if (*(*single).table.offset(j_0 as isize))
            .c2rust_unnamed
            .c2rust_unnamed
            .len as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if is16bit != 0 {
            (*table.offset(j_0 as isize))
                .c2rust_unnamed
                .val16[0 as ::core::ffi::c_int as usize] = (*(*single)
                .table
                .offset(j_0 as isize))
                .c2rust_unnamed
                .c2rust_unnamed
                .sym as uint16_t;
        } else {
            (*table.offset(j_0 as isize))
                .c2rust_unnamed
                .val8[0 as ::core::ffi::c_int as usize] = (*(*single)
                .table
                .offset(j_0 as isize))
                .c2rust_unnamed
                .c2rust_unnamed
                .sym as uint8_t;
        }
        j_0 += 1;
    }
    add_level(
        table,
        is16bit,
        nb_codes,
        numbits,
        buf,
        0 as uint32_t,
        0 as ::core::ffi::c_int,
        if maxbits > numbits { numbits } else { maxbits },
        0 as ::core::ffi::c_int,
        minbits,
        max,
        &raw mut count as *mut ::core::ffi::c_uint,
        info,
    );
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"Joint: %d/%d/%d/%d/%d codes min=%ubits max=%u\n\0" as *const u8
            as *const ::core::ffi::c_char,
        count[0 as ::core::ffi::c_int as usize],
        count[1 as ::core::ffi::c_int as usize],
        count[2 as ::core::ffi::c_int as usize],
        count[3 as ::core::ffi::c_int as usize],
        count[4 as ::core::ffi::c_int as usize],
        minbits,
        max,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_vlc_init_multi_from_lengths(
    mut vlc: *mut VLC,
    mut multi: *mut VLC_MULTI,
    mut nb_bits: ::core::ffi::c_int,
    mut nb_elems: ::core::ffi::c_int,
    mut nb_codes: ::core::ffi::c_int,
    mut lens: *const int8_t,
    mut lens_wrap: ::core::ffi::c_int,
    mut symbols: *const ::core::ffi::c_void,
    mut symbols_wrap: ::core::ffi::c_int,
    mut symbols_size: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut localbuf: [VLCcode; 1500] = [VLCcode {
        bits: 0,
        symbol: 0,
        code: 0,
    }; 1500];
    let mut buf: *mut VLCcode = &raw mut localbuf as *mut VLCcode;
    let mut code: uint64_t = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut len_max: ::core::ffi::c_int = if 32 as ::core::ffi::c_int
        > 3 as ::core::ffi::c_int * nb_bits
    {
        3 as ::core::ffi::c_int * nb_bits
    } else {
        32 as ::core::ffi::c_int
    };
    ret = vlc_common_init(vlc, nb_bits, nb_codes, &raw mut buf, flags);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*multi).table = av_mallocz(
        (::core::mem::size_of::<VLC_MULTI_ELEM>() as size_t) << nb_bits,
    ) as *mut VLC_MULTI_ELEM;
    if !(*multi).table.is_null() {
        code = 0 as uint64_t;
        j = code as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            if !(i < nb_codes) {
                current_block = 3437258052017859086;
                break;
            }
            let mut len: ::core::ffi::c_int = *lens as ::core::ffi::c_int;
            if len > 0 as ::core::ffi::c_int {
                let mut sym: ::core::ffi::c_uint = 0;
                (*buf.offset(j as isize)).bits = len as uint8_t;
                if !symbols.is_null() {
                    let mut ptr: *const uint8_t = (symbols as *const uint8_t)
                        .offset((i * symbols_wrap) as isize);
                    match symbols_size {
                        1 => {
                            sym = *ptr as ::core::ffi::c_uint;
                        }
                        2 => {
                            sym = *(ptr as *const uint16_t) as ::core::ffi::c_uint;
                        }
                        4 => {
                            sym = *(ptr as *const uint32_t) as ::core::ffi::c_uint;
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                } else {
                    sym = i as ::core::ffi::c_uint;
                }
                (*buf.offset(j as isize)).symbol = sym
                    .wrapping_add(offset as ::core::ffi::c_uint) as VLCBaseType;
                let fresh3 = j;
                j = j + 1;
                (*buf.offset(fresh3 as isize)).code = code as uint32_t;
                current_block = 14401909646449704462;
            } else if len < 0 as ::core::ffi::c_int {
                len = -len;
                current_block = 14401909646449704462;
            } else {
                current_block = 11875828834189669668;
            }
            match current_block {
                14401909646449704462 => {
                    if len > len_max
                        || code
                            & ((1 as ::core::ffi::c_uint)
                                << 32 as ::core::ffi::c_int - len)
                                .wrapping_sub(1 as ::core::ffi::c_uint) as uint64_t != 0
                    {
                        av_log(
                            logctx,
                            16 as ::core::ffi::c_int,
                            b"Invalid VLC (length %u)\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            len,
                        );
                        current_block = 12464540871950957853;
                        break;
                    } else {
                        code = code
                            .wrapping_add(
                                ((1 as ::core::ffi::c_uint)
                                    << 32 as ::core::ffi::c_int - len) as uint64_t,
                            );
                        if code
                            > (0xffffffff as ::core::ffi::c_ulonglong)
                                .wrapping_add(1 as ::core::ffi::c_ulonglong)
                        {
                            av_log(
                                logctx,
                                16 as ::core::ffi::c_int,
                                b"Overdetermined VLC tree\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 12464540871950957853;
                            break;
                        }
                    }
                }
                _ => {}
            }
            i += 1;
            lens = lens.offset(lens_wrap as isize);
        }
        match current_block {
            12464540871950957853 => {}
            _ => {
                ret = vlc_common_end(vlc, nb_bits, j, buf, flags, buf as *mut VLCcode);
                if !(ret < 0 as ::core::ffi::c_int) {
                    ret = vlc_multi_gen(
                        (*multi).table,
                        vlc,
                        (nb_elems > 256 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        j,
                        nb_bits,
                        buf,
                        logctx,
                    );
                    if buf != &raw mut localbuf as *mut VLCcode {
                        av_free(buf as *mut ::core::ffi::c_void);
                    }
                    return ret;
                }
            }
        }
    }
    if buf != &raw mut localbuf as *mut VLCcode {
        av_free(buf as *mut ::core::ffi::c_void);
    }
    ff_vlc_free_multi(multi);
    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
        | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
        | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
        as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ff_vlc_free_multi(mut vlc: *mut VLC_MULTI) {
    av_freep(&raw mut (*vlc).table as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ff_vlc_free(mut vlc: *mut VLC) {
    av_freep(&raw mut (*vlc).table as *mut ::core::ffi::c_void);
}
