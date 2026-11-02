// Translated from FFmpeg 9.0.1 libavutil/avstring.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2000, 2001, 2002 Fabrice Bellard
// Copyright (c) 2007 Mans Rullgard
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
    fn memcmp(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn abort() -> !;
    fn strcspn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn strspn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn vsnprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        _: crate::rt::VaList,
    ) -> ::core::ffi::c_int;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_bprint_init(
        buf: *mut AVBPrint,
        size_init: ::core::ffi::c_uint,
        size_max: ::core::ffi::c_uint,
    );
    fn av_bprint_append_data(
        buf: *mut AVBPrint,
        data: *const ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
    );
    fn av_realloc(
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_bprint_finalize(
        buf: *mut AVBPrint,
        ret_str: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_bprint_escape(
        dstbuf: *mut AVBPrint,
        src: *const ::core::ffi::c_char,
        special_chars: *const ::core::ffi::c_char,
        mode: AVEscapeMode,
        flags: ::core::ffi::c_int,
    );
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_strdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __ap: *mut ::core::ffi::c_void,
}
pub type va_list = __builtin_va_list;
pub type intptr_t = ::core::ffi::c_int;
pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
pub type size_t = ::core::ffi::c_uint;
pub type __isoc_va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBPrint {
    pub str_0: *mut ::core::ffi::c_char,
    pub len: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub size_max: ::core::ffi::c_uint,
    pub reserved_internal_buffer: [::core::ffi::c_char; 1],
    pub reserved_padding: [::core::ffi::c_char; 1004],
}
pub type AVEscapeMode = ::core::ffi::c_uint;
pub const AV_ESCAPE_MODE_XML: AVEscapeMode = 3;
pub const AV_ESCAPE_MODE_QUOTE: AVEscapeMode = 2;
pub const AV_ESCAPE_MODE_BACKSLASH: AVEscapeMode = 1;
pub const AV_ESCAPE_MODE_AUTO: AVEscapeMode = 0;
#[no_mangle]
pub unsafe extern "C" fn av_strstart(
    mut str: *const ::core::ffi::c_char,
    mut pfx: *const ::core::ffi::c_char,
    mut ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *pfx as ::core::ffi::c_int != 0
        && *pfx as ::core::ffi::c_int == *str as ::core::ffi::c_int
    {
        pfx = pfx.offset(1);
        str = str.offset(1);
    }
    if *pfx == 0 && !ptr.is_null() {
        *ptr = str;
    }
    return (*pfx == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_stristart(
    mut str: *const ::core::ffi::c_char,
    mut pfx: *const ::core::ffi::c_char,
    mut ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *pfx as ::core::ffi::c_int != 0
        && av_toupper(*pfx as ::core::ffi::c_uint as ::core::ffi::c_int)
            == av_toupper(*str as ::core::ffi::c_uint as ::core::ffi::c_int)
    {
        pfx = pfx.offset(1);
        str = str.offset(1);
    }
    if *pfx == 0 && !ptr.is_null() {
        *ptr = str;
    }
    return (*pfx == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_stristr(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if *s2 == 0 {
        return s1 as intptr_t as *mut ::core::ffi::c_char;
    }
    loop {
        if av_stristart(s1, s2, ::core::ptr::null_mut::<*const ::core::ffi::c_char>())
            != 0
        {
            return s1 as intptr_t as *mut ::core::ffi::c_char;
        }
        let fresh0 = s1;
        s1 = s1.offset(1);
        if !(*fresh0 != 0) {
            break;
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn av_strnstr(
    mut haystack: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
    mut hay_length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut needle_len: size_t = strlen(needle) as size_t;
    if needle_len == 0 {
        return haystack as *mut ::core::ffi::c_char;
    }
    while hay_length >= needle_len {
        hay_length = hay_length.wrapping_sub(1);
        if memcmp(
            haystack as *const ::core::ffi::c_void,
            needle as *const ::core::ffi::c_void,
            needle_len as size_t,
        ) == 0
        {
            return haystack as *mut ::core::ffi::c_char;
        }
        haystack = haystack.offset(1);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn av_strlcpy(
    mut dst: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut size: size_t,
) -> size_t {
    let mut len: size_t = 0 as size_t;
    loop {
        len = len.wrapping_add(1);
        if !(len < size && *src as ::core::ffi::c_int != 0) {
            break;
        }
        let fresh1 = src;
        src = src.offset(1);
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = *fresh1;
    }
    if len <= size {
        *dst = 0 as ::core::ffi::c_char;
    }
    return len.wrapping_add(strlen(src) as size_t).wrapping_sub(1 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn av_strlcat(
    mut dst: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut size: size_t,
) -> size_t {
    let mut len: size_t = strlen(dst) as size_t;
    if size <= len.wrapping_add(1 as size_t) {
        return len.wrapping_add(strlen(src) as size_t);
    }
    return len
        .wrapping_add(av_strlcpy(dst.offset(len as isize), src, size.wrapping_sub(len)));
}
extern "C" {
    // asm/varargs-arm.S: builds the va_list, calls av_strlcatf__va
    pub fn av_strlcatf(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> size_t;
}
#[no_mangle]
pub unsafe extern "C" fn av_strlcatf__va(
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut fmt: *const ::core::ffi::c_char,
    mut args: crate::rt::VaList,
) -> size_t {
    let mut len: size_t = strlen(dst) as size_t;
    let mut vl: crate::rt::VaList;
    vl = args.clone();
    len = len
        .wrapping_add(
            vsnprintf(
                dst.offset(len as isize),
                if size > len {
                    (size as size_t).wrapping_sub(len as size_t)
                } else {
                    0 as size_t
                },
                fmt,
                vl.as_va_list(),
            ) as size_t,
        );
    return len;
}
extern "C" {
    // asm/varargs-arm.S: builds the va_list, calls av_asprintf__va
    pub fn av_asprintf(
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn av_asprintf__va(
    mut fmt: *const ::core::ffi::c_char,
    mut args: crate::rt::VaList,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut va: crate::rt::VaList;
    let mut len: ::core::ffi::c_int = 0;
    va = args.clone();
    len = vsnprintf(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
        fmt,
        va.as_va_list(),
    );
    if !(len < 0 as ::core::ffi::c_int) {
        p = av_malloc((len + 1 as ::core::ffi::c_int) as size_t)
            as *mut ::core::ffi::c_char;
        if !p.is_null() {
            va = args.clone();
            len = vsnprintf(
                p,
                (len + 1 as ::core::ffi::c_int) as size_t,
                fmt,
                va.as_va_list(),
            );
            if len < 0 as ::core::ffi::c_int {
                av_freep(&raw mut p as *mut ::core::ffi::c_void);
            }
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_token(
    mut buf: *mut *const ::core::ffi::c_char,
    mut term: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut out: *mut ::core::ffi::c_char = av_realloc(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        (strlen(*buf) as size_t).wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    let mut ret: *mut ::core::ffi::c_char = out;
    let mut end: *mut ::core::ffi::c_char = out;
    let mut p: *const ::core::ffi::c_char = *buf;
    if out.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = p
        .offset(
            strspn(p, b" \n\t\r\0" as *const u8 as *const ::core::ffi::c_char) as isize,
        );
    while *p as ::core::ffi::c_int != 0 && strspn(p, term) == 0 {
        let fresh3 = p;
        p = p.offset(1);
        let mut c: ::core::ffi::c_char = *fresh3;
        if c as ::core::ffi::c_int == '\\' as i32 && *p as ::core::ffi::c_int != 0 {
            let fresh4 = p;
            p = p.offset(1);
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 = *fresh4;
            end = out;
        } else if c as ::core::ffi::c_int == '\'' as i32 {
            while *p as ::core::ffi::c_int != 0
                && *p as ::core::ffi::c_int != '\'' as i32
            {
                let fresh6 = p;
                p = p.offset(1);
                let fresh7 = out;
                out = out.offset(1);
                *fresh7 = *fresh6;
            }
            if *p != 0 {
                p = p.offset(1);
                end = out;
            }
        } else {
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = c;
        }
    }
    loop {
        let fresh9 = out;
        out = out.offset(-1);
        *fresh9 = 0 as ::core::ffi::c_char;
        if !(out >= end
            && strspn(out, b" \n\t\r\0" as *const u8 as *const ::core::ffi::c_char) != 0)
        {
            break;
        }
    }
    *buf = p;
    let mut small_ret: *mut ::core::ffi::c_char = av_realloc(
        ret as *mut ::core::ffi::c_void,
        (out.offset_from(ret) as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as size_t,
    ) as *mut ::core::ffi::c_char;
    return if !small_ret.is_null() { small_ret } else { ret };
}
#[no_mangle]
pub unsafe extern "C" fn av_strtok(
    mut s: *mut ::core::ffi::c_char,
    mut delim: *const ::core::ffi::c_char,
    mut saveptr: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut tok: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if s.is_null()
        && {
            s = *saveptr;
            s.is_null()
        }
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    s = s.offset(strspn(s, delim) as isize);
    if *s == 0 {
        *saveptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let fresh10 = s;
    s = s.offset(1);
    tok = fresh10;
    s = s.offset(strcspn(s, delim) as isize);
    if *s != 0 {
        *s = 0 as ::core::ffi::c_char;
        *saveptr = s.offset(1 as ::core::ffi::c_int as isize);
    } else {
        *saveptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn av_strcasecmp(
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c1: uint8_t = 0;
    let mut c2: uint8_t = 0;
    loop {
        let fresh11 = a;
        a = a.offset(1);
        c1 = av_tolower(*fresh11 as ::core::ffi::c_int) as uint8_t;
        let fresh12 = b;
        b = b.offset(1);
        c2 = av_tolower(*fresh12 as ::core::ffi::c_int) as uint8_t;
        if !(c1 as ::core::ffi::c_int != 0
            && c1 as ::core::ffi::c_int == c2 as ::core::ffi::c_int)
        {
            break;
        }
    }
    return c1 as ::core::ffi::c_int - c2 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_strncasecmp(
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
    mut n: size_t,
) -> ::core::ffi::c_int {
    let mut c1: uint8_t = 0;
    let mut c2: uint8_t = 0;
    if n <= 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let fresh13 = a;
        a = a.offset(1);
        c1 = av_tolower(*fresh13 as ::core::ffi::c_int) as uint8_t;
        let fresh14 = b;
        b = b.offset(1);
        c2 = av_tolower(*fresh14 as ::core::ffi::c_int) as uint8_t;
        n = n.wrapping_sub(1);
        if !(n != 0 && c1 as ::core::ffi::c_int != 0
            && c1 as ::core::ffi::c_int == c2 as ::core::ffi::c_int)
        {
            break;
        }
    }
    return c1 as ::core::ffi::c_int - c2 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_bprint_is_complete(
    mut buf: *const AVBPrint,
) -> ::core::ffi::c_int {
    return ((*buf).len < (*buf).size) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_toupper(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c >= 'a' as i32 && c <= 'z' as i32 {
        c ^= 0x20 as ::core::ffi::c_int;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn av_strireplace(
    mut str: *const ::core::ffi::c_char,
    mut from: *const ::core::ffi::c_char,
    mut to: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut ret: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut pstr2: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut pstr: *const ::core::ffi::c_char = str;
    let mut tolen: size_t = strlen(to) as size_t;
    let mut fromlen: size_t = strlen(from) as size_t;
    let mut pbuf: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1004],
    };
    av_bprint_init(
        &raw mut pbuf,
        1 as ::core::ffi::c_uint,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    loop {
        pstr2 = av_stristr(pstr, from);
        if pstr2.is_null() {
            break;
        }
        av_bprint_append_data(
            &raw mut pbuf,
            pstr,
            pstr2.offset_from(pstr) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        pstr = pstr2.offset(fromlen as isize);
        av_bprint_append_data(&raw mut pbuf, to, tolen as ::core::ffi::c_uint);
    }
    av_bprint_append_data(&raw mut pbuf, pstr, strlen(pstr) as ::core::ffi::c_uint);
    if av_bprint_is_complete(&raw mut pbuf) == 0 {
        av_bprint_finalize(
            &raw mut pbuf,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    } else {
        av_bprint_finalize(&raw mut pbuf, &raw mut ret);
    }
    return ret;
}
#[inline]
unsafe extern "C" fn av_tolower(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        c ^= 0x20 as ::core::ffi::c_int;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn av_basename(
    mut path: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if path.is_null() || *path as ::core::ffi::c_int == '\0' as i32 {
        return b".\0" as *const u8 as *const ::core::ffi::c_char;
    }
    p = strrchr(path, '/' as i32);
    if p.is_null() {
        return path;
    }
    return p.offset(1 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn av_dirname(
    mut path: *mut ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = if !path.is_null() {
        strrchr(path, '/' as i32)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if p.is_null() {
        return b".\0" as *const u8 as *const ::core::ffi::c_char;
    }
    *p = '\0' as i32 as ::core::ffi::c_char;
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn av_append_path_component(
    mut path: *const ::core::ffi::c_char,
    mut component: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p_len: size_t = 0;
    let mut c_len: size_t = 0;
    let mut fullpath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if path.is_null() {
        return av_strdup(component);
    }
    if component.is_null() {
        return av_strdup(path);
    }
    p_len = strlen(path) as size_t;
    c_len = strlen(component) as size_t;
    if p_len > (0xffffffff as size_t).wrapping_sub(c_len)
        || p_len.wrapping_add(c_len)
            > (0xffffffff as ::core::ffi::c_uint).wrapping_sub(2 as ::core::ffi::c_uint)
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    fullpath = av_malloc(p_len.wrapping_add(c_len).wrapping_add(2 as size_t))
        as *mut ::core::ffi::c_char;
    if !fullpath.is_null() {
        if p_len != 0 {
            av_strlcpy(fullpath, path, p_len.wrapping_add(1 as size_t));
            if c_len != 0 {
                if *fullpath.offset(p_len.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_int != '/' as i32
                    && *component.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int != '/' as i32
                {
                    let fresh15 = p_len;
                    p_len = p_len.wrapping_add(1);
                    *fullpath.offset(fresh15 as isize) = '/' as i32
                        as ::core::ffi::c_char;
                } else if *fullpath.offset(p_len.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_int == '/' as i32
                    && *component.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '/' as i32
                {
                    p_len = p_len.wrapping_sub(1);
                }
            }
        }
        av_strlcpy(
            fullpath.offset(p_len as isize) as *mut ::core::ffi::c_char,
            component,
            c_len.wrapping_add(1 as size_t),
        );
        *fullpath.offset(p_len.wrapping_add(c_len) as isize) = 0 as ::core::ffi::c_char;
    }
    return fullpath;
}
#[no_mangle]
pub unsafe extern "C" fn av_escape(
    mut dst: *mut *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut special_chars: *const ::core::ffi::c_char,
    mut mode: AVEscapeMode,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dstbuf: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1004],
    };
    let mut ret: ::core::ffi::c_int = 0;
    av_bprint_init(
        &raw mut dstbuf,
        1 as ::core::ffi::c_uint,
        2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    av_bprint_escape(&raw mut dstbuf, src, special_chars, mode, flags);
    if av_bprint_is_complete(&raw mut dstbuf) == 0 {
        av_bprint_finalize(
            &raw mut dstbuf,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        return -(12 as ::core::ffi::c_int);
    }
    ret = av_bprint_finalize(&raw mut dstbuf, dst);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    return dstbuf.len as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_match_name(
    mut name: *const ::core::ffi::c_char,
    mut names: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    let mut namelen: size_t = 0;
    if name.is_null() || names.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    namelen = strlen(name) as size_t;
    while *names != 0 {
        let mut negate: ::core::ffi::c_int = ('-' as i32 == *names as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        p = strchr(names, ',' as i32);
        if p.is_null() {
            p = names.offset(strlen(names) as isize);
        }
        names = names.offset(negate as isize);
        len = if p.offset_from(names) as ::core::ffi::c_int as size_t > namelen {
            p.offset_from(names) as ::core::ffi::c_int as size_t
        } else {
            namelen
        };
        if av_strncasecmp(name, names, len) == 0
            || strncmp(
                b"ALL\0" as *const u8 as *const ::core::ffi::c_char,
                names,
                (if 3 as ::core::ffi::c_int > p.offset_from(names) as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    p.offset_from(names) as ::core::ffi::c_int
                }) as size_t,
            ) == 0
        {
            return (negate == 0) as ::core::ffi::c_int;
        }
        names = p
            .offset(
                (*p as ::core::ffi::c_int == ',' as i32) as ::core::ffi::c_int as isize,
            );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_utf8_decode(
    mut codep: *mut int32_t,
    mut bufp: *mut *const uint8_t,
    mut buf_end: *const uint8_t,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut p: *const uint8_t = *bufp;
    let mut top: uint32_t = 0;
    let mut code: uint64_t = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tail_len: ::core::ffi::c_int = 0;
    let mut overlong_encoding_mins: [uint32_t; 6] = [
        0 as ::core::ffi::c_int as uint32_t,
        0x80 as ::core::ffi::c_int as uint32_t,
        0x800 as ::core::ffi::c_int as uint32_t,
        0x10000 as ::core::ffi::c_int as uint32_t,
        0x200000 as ::core::ffi::c_int as uint32_t,
        0x4000000 as ::core::ffi::c_int as uint32_t,
    ];
    if p >= buf_end {
        return 0 as ::core::ffi::c_int;
    }
    let fresh16 = p;
    p = p.offset(1);
    code = *fresh16 as uint64_t;
    if code & 0xc0 as uint64_t == 0x80 as uint64_t || code >= 0xfe as uint64_t {
        ret = -(84 as ::core::ffi::c_int);
    } else {
        top = ((code & 128 as uint64_t) >> 1 as ::core::ffi::c_int) as uint32_t;
        tail_len = 0 as ::core::ffi::c_int;
        while code & top as uint64_t != 0 {
            let mut tmp: ::core::ffi::c_int = 0;
            tail_len += 1;
            if p >= buf_end {
                *bufp = (*bufp).offset(1);
                return -(84 as ::core::ffi::c_int);
            }
            let fresh17 = p;
            p = p.offset(1);
            tmp = *fresh17 as ::core::ffi::c_int - 128 as ::core::ffi::c_int;
            if tmp >> 6 as ::core::ffi::c_int != 0 {
                *bufp = (*bufp).offset(1);
                return -(84 as ::core::ffi::c_int);
            }
            code = (code << 6 as ::core::ffi::c_int).wrapping_add(tmp as uint64_t);
            top <<= 5 as ::core::ffi::c_int;
        }
        code &= (top << 1 as ::core::ffi::c_int).wrapping_sub(1 as uint32_t) as uint64_t;
        if !(tail_len <= 5 as ::core::ffi::c_int) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"tail_len <= 5\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavutil/avstring.c\0" as *const u8 as *const ::core::ffi::c_char,
                414 as ::core::ffi::c_int,
            );
            abort();
        }
        if code < overlong_encoding_mins[tail_len as usize] as uint64_t {
            ret = -(84 as ::core::ffi::c_int);
        } else if code
            >= ((1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int) as uint64_t
        {
            ret = -(84 as ::core::ffi::c_int);
        } else {
            *codep = code as int32_t;
            if code > 0x10ffff as uint64_t && flags & 1 as ::core::ffi::c_uint == 0 {
                ret = -(84 as ::core::ffi::c_int);
            }
            if code < 0x20 as uint64_t && code != 0x9 as uint64_t
                && code != 0xa as uint64_t && code != 0xd as uint64_t
                && flags & 8 as ::core::ffi::c_uint != 0
            {
                ret = -(84 as ::core::ffi::c_int);
            }
            if code >= 0xd800 as uint64_t && code <= 0xdfff as uint64_t
                && flags & 4 as ::core::ffi::c_uint == 0
            {
                ret = -(84 as ::core::ffi::c_int);
            }
            if (code == 0xfffe as uint64_t || code == 0xffff as uint64_t)
                && flags & 2 as ::core::ffi::c_uint == 0
            {
                ret = -(84 as ::core::ffi::c_int);
            }
        }
    }
    *bufp = p;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_match_list(
    mut name: *const ::core::ffi::c_char,
    mut list: *const ::core::ffi::c_char,
    mut separator: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = name;
    while !p.is_null() && *p as ::core::ffi::c_int != 0 {
        q = list;
        while !q.is_null() && *q as ::core::ffi::c_int != 0 {
            let mut k: ::core::ffi::c_int = 0;
            k = 0 as ::core::ffi::c_int;
            while *p.offset(k as isize) as ::core::ffi::c_int
                == *q.offset(k as isize) as ::core::ffi::c_int
                || *p.offset(k as isize) as ::core::ffi::c_int
                    * *q.offset(k as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && *p.offset(k as isize) as ::core::ffi::c_int
                        + *q.offset(k as isize) as ::core::ffi::c_int
                        == separator as ::core::ffi::c_int
            {
                if k != 0
                    && (*p.offset(k as isize) == 0
                        || *p.offset(k as isize) as ::core::ffi::c_int
                            == separator as ::core::ffi::c_int)
                {
                    return 1 as ::core::ffi::c_int;
                }
                k += 1;
            }
            q = strchr(q, separator as ::core::ffi::c_int);
            if !q.is_null() {
                q = q.offset(1);
            }
        }
        p = strchr(p, separator as ::core::ffi::c_int);
        if !p.is_null() {
            p = p.offset(1);
        }
    }
    return 0 as ::core::ffi::c_int;
}
