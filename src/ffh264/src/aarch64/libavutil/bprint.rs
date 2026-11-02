// Translated from FFmpeg 9.0.1 libavutil/bprint.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2012 Nicolas George
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strftime(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        _: *const tm,
    ) -> size_t;
    fn vsnprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        _: crate::rt::VaList,
    ) -> ::core::ffi::c_int;
    fn av_realloc(
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_memdup(
        p: *const ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut ::core::ffi::c_void,
    pub __gr_top: *mut ::core::ffi::c_void,
    pub __vr_top: *mut ::core::ffi::c_void,
    pub __gr_offs: ::core::ffi::c_int,
    pub __vr_offs: ::core::ffi::c_int,
}
pub type va_list = __builtin_va_list;
pub type size_t = ::core::ffi::c_ulong;
pub type __isoc_va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub __tm_gmtoff: ::core::ffi::c_long,
    pub __tm_zone: *const ::core::ffi::c_char,
}
pub type AVEscapeMode = ::core::ffi::c_uint;
pub const AV_ESCAPE_MODE_XML: AVEscapeMode = 3;
pub const AV_ESCAPE_MODE_QUOTE: AVEscapeMode = 2;
pub const AV_ESCAPE_MODE_BACKSLASH: AVEscapeMode = 1;
pub const AV_ESCAPE_MODE_AUTO: AVEscapeMode = 0;
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
unsafe extern "C" fn av_bprint_alloc(
    mut buf: *mut AVBPrint,
    mut room: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut old_str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut new_str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut min_size: ::core::ffi::c_uint = 0;
    let mut new_size: ::core::ffi::c_uint = 0;
    if (*buf).size == (*buf).size_max {
        return -(5 as ::core::ffi::c_int);
    }
    if av_bprint_is_complete(buf) == 0 {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    min_size = (*buf)
        .len
        .wrapping_add(1 as ::core::ffi::c_uint)
        .wrapping_add(
            (if (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_sub((*buf).len)
                .wrapping_sub(1 as ::core::ffi::c_uint) > room
            {
                room
            } else {
                (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_sub((*buf).len)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
            }),
        );
    new_size = if (*buf).size > (*buf).size_max.wrapping_div(2 as ::core::ffi::c_uint) {
        (*buf).size_max
    } else {
        (*buf).size.wrapping_mul(2 as ::core::ffi::c_uint)
    };
    if new_size < min_size {
        new_size = if (*buf).size_max > min_size { min_size } else { (*buf).size_max };
    }
    old_str = if (*buf).str_0
        != &raw mut (*buf).reserved_internal_buffer as *mut ::core::ffi::c_char
    {
        (*buf).str_0
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    new_str = av_realloc(old_str as *mut ::core::ffi::c_void, new_size as size_t)
        as *mut ::core::ffi::c_char;
    if new_str.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    if old_str.is_null() {
        memcpy(
            new_str as *mut ::core::ffi::c_void,
            (*buf).str_0 as *const ::core::ffi::c_void,
            (*buf).len.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
        );
    }
    (*buf).str_0 = new_str;
    (*buf).size = new_size;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn av_bprint_grow(
    mut buf: *mut AVBPrint,
    mut extra_len: ::core::ffi::c_uint,
) {
    extra_len = if extra_len
        > (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint)
            .wrapping_sub(5 as ::core::ffi::c_uint)
            .wrapping_sub((*buf).len)
    {
        (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint)
            .wrapping_sub(5 as ::core::ffi::c_uint)
            .wrapping_sub((*buf).len)
    } else {
        extra_len
    };
    (*buf).len = (*buf).len.wrapping_add(extra_len);
    if (*buf).size != 0 {
        *(*buf)
            .str_0
            .offset(
                (if (*buf).len > (*buf).size.wrapping_sub(1 as ::core::ffi::c_uint) {
                    (*buf).size.wrapping_sub(1 as ::core::ffi::c_uint)
                } else {
                    (*buf).len
                }) as isize,
            ) = 0 as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_bprint_init(
    mut buf: *mut AVBPrint,
    mut size_init: ::core::ffi::c_uint,
    mut size_max: ::core::ffi::c_uint,
) {
    let mut size_auto: ::core::ffi::c_uint = (buf as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<AVBPrint>() as usize as isize)
        .offset_from(
            &raw mut (*buf).reserved_internal_buffer as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_long as ::core::ffi::c_uint;
    if size_max == 1 as ::core::ffi::c_uint {
        size_max = size_auto;
    }
    (*buf).str_0 = &raw mut (*buf).reserved_internal_buffer as *mut ::core::ffi::c_char;
    (*buf).len = 0 as ::core::ffi::c_uint;
    (*buf).size = if size_auto > size_max { size_max } else { size_auto };
    (*buf).size_max = size_max;
    *(*buf).str_0 = 0 as ::core::ffi::c_char;
    if size_init > (*buf).size {
        av_bprint_alloc(buf, size_init.wrapping_sub(1 as ::core::ffi::c_uint));
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_bprint_init_for_buffer(
    mut buf: *mut AVBPrint,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_uint,
) {
    if size == 0 as ::core::ffi::c_uint {
        av_bprint_init(buf, 0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint);
        return;
    }
    (*buf).str_0 = buffer;
    (*buf).len = 0 as ::core::ffi::c_uint;
    (*buf).size = size;
    (*buf).size_max = size;
    *(*buf).str_0 = 0 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn av_vbprintf(
    mut buf: *mut AVBPrint,
    mut fmt: *const ::core::ffi::c_char,
    mut vl_arg: crate::rt::VaList,
) {
    let mut room: ::core::ffi::c_uint = 0;
    let mut dst: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut extra_len: ::core::ffi::c_int = 0;
    let mut vl: crate::rt::VaList;
    loop {
        room = (*buf)
            .size
            .wrapping_sub(
                (if (*buf).len > (*buf).size { (*buf).size } else { (*buf).len }),
            );
        dst = if room != 0 {
            (*buf).str_0.offset((*buf).len as isize)
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        vl = vl_arg.clone();
        extra_len = vsnprintf(dst, room as size_t, fmt, vl.as_va_list());
        if extra_len <= 0 as ::core::ffi::c_int {
            return;
        }
        if (extra_len as ::core::ffi::c_uint) < room {
            break;
        }
        if av_bprint_alloc(buf, extra_len as ::core::ffi::c_uint) != 0 {
            break;
        }
    }
    av_bprint_grow(buf, extra_len as ::core::ffi::c_uint);
}
extern "C" {
    // asm/varargs-aarch64.S: builds the va_list, calls av_bprintf__va
    pub fn av_bprintf(
        buf: *mut AVBPrint,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_bprintf__va(
    mut buf: *mut AVBPrint,
    mut fmt: *const ::core::ffi::c_char,
    mut args: crate::rt::VaList,
) {
    let mut vl: crate::rt::VaList;
    vl = args.clone();
    av_vbprintf(buf, fmt, vl.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn av_bprint_chars(
    mut buf: *mut AVBPrint,
    mut c: ::core::ffi::c_char,
    mut n: ::core::ffi::c_uint,
) {
    let mut room: ::core::ffi::c_uint = 0;
    let mut real_n: ::core::ffi::c_uint = 0;
    loop {
        room = (*buf)
            .size
            .wrapping_sub(
                (if (*buf).len > (*buf).size { (*buf).size } else { (*buf).len }),
            );
        if n < room {
            break;
        }
        if av_bprint_alloc(buf, n) != 0 {
            break;
        }
    }
    if room != 0 {
        real_n = if n > room.wrapping_sub(1 as ::core::ffi::c_uint) {
            room.wrapping_sub(1 as ::core::ffi::c_uint)
        } else {
            n
        };
        memset(
            (*buf).str_0.offset((*buf).len as isize) as *mut ::core::ffi::c_void,
            c as ::core::ffi::c_int,
            real_n as size_t,
        );
    }
    av_bprint_grow(buf, n);
}
#[no_mangle]
pub unsafe extern "C" fn av_bprint_append_data(
    mut buf: *mut AVBPrint,
    mut data: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_uint,
) {
    let mut room: ::core::ffi::c_uint = 0;
    let mut real_n: ::core::ffi::c_uint = 0;
    loop {
        room = (*buf)
            .size
            .wrapping_sub(
                (if (*buf).len > (*buf).size { (*buf).size } else { (*buf).len }),
            );
        if size < room {
            break;
        }
        if av_bprint_alloc(buf, size) != 0 {
            break;
        }
    }
    if room != 0 {
        real_n = if size > room.wrapping_sub(1 as ::core::ffi::c_uint) {
            room.wrapping_sub(1 as ::core::ffi::c_uint)
        } else {
            size
        };
        memcpy(
            (*buf).str_0.offset((*buf).len as isize) as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            real_n as size_t,
        );
    }
    av_bprint_grow(buf, size);
}
#[no_mangle]
pub unsafe extern "C" fn av_bprint_strftime(
    mut buf: *mut AVBPrint,
    mut fmt: *const ::core::ffi::c_char,
    mut tm: *const tm,
) {
    let mut room: ::core::ffi::c_uint = 0;
    let mut l: size_t = 0;
    let mut fmt_len: size_t = strlen(fmt) as size_t;
    if *fmt == 0 {
        return;
    }
    loop {
        room = (*buf)
            .size
            .wrapping_sub(
                (if (*buf).len > (*buf).size { (*buf).size } else { (*buf).len }),
            );
        if room != 0
            && {
                l = strftime(
                    (*buf).str_0.offset((*buf).len as isize),
                    room as size_t,
                    fmt,
                    tm,
                ) as size_t;
                l != 0
            }
        {
            break;
        }
        if (room >> 8 as ::core::ffi::c_int) as size_t > fmt_len {
            break;
        }
        room = (if room == 0 {
            fmt_len.wrapping_add(1 as size_t)
        } else {
            (if room
                <= (2147483647 as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
                room.wrapping_mul(2 as ::core::ffi::c_uint)
            } else {
                2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint
            }) as size_t
        }) as ::core::ffi::c_uint;
        if av_bprint_alloc(buf, room) != 0 {
            room = (*buf)
                .size
                .wrapping_sub(
                    (if (*buf).len > (*buf).size { (*buf).size } else { (*buf).len }),
                );
            if room < 1024 as ::core::ffi::c_uint {
                let mut buf2: [::core::ffi::c_char; 1024] = [0; 1024];
                l = strftime(
                    &raw mut buf2 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                    fmt,
                    tm,
                ) as size_t;
                if l != 0 {
                    av_bprintf(
                        buf,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut buf2 as *mut ::core::ffi::c_char,
                    );
                    return;
                }
            }
            if room != 0 {
                static mut txt: [::core::ffi::c_char; 28] = unsafe {
                    ::core::mem::transmute::<
                        [u8; 28],
                        [::core::ffi::c_char; 28],
                    >(*b"[truncated strftime output]\0")
                };
                memset(
                    (*buf).str_0.offset((*buf).len as isize) as *mut ::core::ffi::c_void,
                    '!' as i32,
                    room as size_t,
                );
                memcpy(
                    (*buf).str_0.offset((*buf).len as isize) as *mut ::core::ffi::c_void,
                    &raw const txt as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    if (::core::mem::size_of::<[::core::ffi::c_char; 28]>() as usize)
                        .wrapping_sub(1 as usize) > room as usize
                    {
                        room as size_t
                    } else {
                        (::core::mem::size_of::<[::core::ffi::c_char; 28]>() as size_t)
                            .wrapping_sub(1 as size_t)
                    },
                );
                av_bprint_grow(buf, room);
            }
            return;
        }
    }
    av_bprint_grow(buf, l as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn av_bprint_get_buffer(
    mut buf: *mut AVBPrint,
    mut size: ::core::ffi::c_uint,
    mut mem: *mut *mut ::core::ffi::c_uchar,
    mut actual_size: *mut ::core::ffi::c_uint,
) {
    if size
        > (*buf)
            .size
            .wrapping_sub(
                (if (*buf).len > (*buf).size { (*buf).size } else { (*buf).len }),
            )
    {
        av_bprint_alloc(buf, size);
    }
    *actual_size = (*buf)
        .size
        .wrapping_sub((if (*buf).len > (*buf).size { (*buf).size } else { (*buf).len }));
    *mem = (if *actual_size != 0 {
        (*buf).str_0.offset((*buf).len as isize)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }) as *mut ::core::ffi::c_uchar;
}
#[inline]
unsafe extern "C" fn av_bprint_is_complete(
    mut buf: *const AVBPrint,
) -> ::core::ffi::c_int {
    return ((*buf).len < (*buf).size) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_bprint_clear(mut buf: *mut AVBPrint) {
    if (*buf).len != 0 {
        *(*buf).str_0 = 0 as ::core::ffi::c_char;
        (*buf).len = 0 as ::core::ffi::c_uint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_bprint_finalize(
    mut buf: *mut AVBPrint,
    mut ret_str: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut real_size: ::core::ffi::c_uint = if (*buf)
        .len
        .wrapping_add(1 as ::core::ffi::c_uint) > (*buf).size
    {
        (*buf).size
    } else {
        (*buf).len.wrapping_add(1 as ::core::ffi::c_uint)
    };
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !ret_str.is_null() {
        if (*buf).str_0
            != &raw mut (*buf).reserved_internal_buffer as *mut ::core::ffi::c_char
        {
            str = av_realloc(
                (*buf).str_0 as *mut ::core::ffi::c_void,
                real_size as size_t,
            ) as *mut ::core::ffi::c_char;
            if str.is_null() {
                str = (*buf).str_0;
            }
            (*buf).str_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            str = av_memdup(
                (*buf).str_0 as *const ::core::ffi::c_void,
                real_size as size_t,
            ) as *mut ::core::ffi::c_char;
            if str.is_null() {
                ret = -(12 as ::core::ffi::c_int);
            }
        }
        *ret_str = str;
    } else if (*buf).str_0
        != &raw mut (*buf).reserved_internal_buffer as *mut ::core::ffi::c_char
    {
        av_freep(&raw mut (*buf).str_0 as *mut ::core::ffi::c_void);
    }
    (*buf).size = real_size;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_bprint_escape(
    mut dstbuf: *mut AVBPrint,
    mut src: *const ::core::ffi::c_char,
    mut special_chars: *const ::core::ffi::c_char,
    mut mode: AVEscapeMode,
    mut flags: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut src0: *const ::core::ffi::c_char = src;
    if mode as ::core::ffi::c_uint
        == AV_ESCAPE_MODE_AUTO as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        mode = AV_ESCAPE_MODE_BACKSLASH;
    }
    match mode as ::core::ffi::c_uint {
        2 => {
            av_bprint_chars(
                dstbuf,
                '\'' as i32 as ::core::ffi::c_char,
                1 as ::core::ffi::c_uint,
            );
            while *src != 0 {
                if *src as ::core::ffi::c_int == '\'' as i32 {
                    av_bprintf(
                        dstbuf,
                        b"'\\''\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    av_bprint_chars(dstbuf, *src, 1 as ::core::ffi::c_uint);
                }
                src = src.offset(1);
            }
            av_bprint_chars(
                dstbuf,
                '\'' as i32 as ::core::ffi::c_char,
                1 as ::core::ffi::c_uint,
            );
        }
        3 => {
            while *src != 0 {
                match *src as ::core::ffi::c_int {
                    38 => {
                        av_bprintf(
                            dstbuf,
                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"&amp;\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        current_block = 11812396948646013369;
                    }
                    60 => {
                        av_bprintf(
                            dstbuf,
                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"&lt;\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        current_block = 11812396948646013369;
                    }
                    62 => {
                        av_bprintf(
                            dstbuf,
                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"&gt;\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        current_block = 11812396948646013369;
                    }
                    39 => {
                        if flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                            == 0
                        {
                            current_block = 5023088878038355716;
                        } else {
                            av_bprintf(
                                dstbuf,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"&apos;\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            current_block = 11812396948646013369;
                        }
                    }
                    34 => {
                        if flags & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            == 0
                        {
                            current_block = 5023088878038355716;
                        } else {
                            av_bprintf(
                                dstbuf,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"&quot;\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            current_block = 11812396948646013369;
                        }
                    }
                    _ => {
                        current_block = 5023088878038355716;
                    }
                }
                match current_block {
                    5023088878038355716 => {
                        av_bprint_chars(dstbuf, *src, 1 as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                src = src.offset(1);
            }
        }
        _ => {
            while *src != 0 {
                let mut is_first_last: ::core::ffi::c_int = (src == src0
                    || *src.offset(1 as ::core::ffi::c_int as isize) == 0)
                    as ::core::ffi::c_int;
                let mut is_ws: ::core::ffi::c_int = !strchr(
                        b" \n\t\r\0" as *const u8 as *const ::core::ffi::c_char,
                        *src as ::core::ffi::c_int,
                    )
                    .is_null() as ::core::ffi::c_int;
                let mut is_strictly_special: ::core::ffi::c_int = (!special_chars
                    .is_null()
                    && !strchr(special_chars, *src as ::core::ffi::c_int).is_null())
                    as ::core::ffi::c_int;
                let mut is_special: ::core::ffi::c_int = (is_strictly_special != 0
                    || !strchr(
                            b"'\\\0" as *const u8 as *const ::core::ffi::c_char,
                            *src as ::core::ffi::c_int,
                        )
                        .is_null()
                    || is_ws != 0
                        && flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                            != 0) as ::core::ffi::c_int;
                if is_strictly_special != 0
                    || flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
                        && (is_special != 0 || is_ws != 0 && is_first_last != 0)
                {
                    av_bprint_chars(
                        dstbuf,
                        '\\' as i32 as ::core::ffi::c_char,
                        1 as ::core::ffi::c_uint,
                    );
                }
                av_bprint_chars(dstbuf, *src, 1 as ::core::ffi::c_uint);
                src = src.offset(1);
            }
        }
    };
}
